use clap::{App, Arg};
use log::{error, info};
use std::fs;

use dadada::{build_html, extract, Block, Options};

fn main() {
    pretty_env_logger::formatted_builder()
        .filter(None, log::LevelFilter::Info)
        .init();

    let matches = App::new("cargo-dadada")
        .version(clap::crate_version!())
        .author(clap::crate_authors!())
        .about(clap::crate_description!())
        .arg(
            Arg::with_name("no_css")
                .long("no-css")
                .required(false)
                .help("Do not add CSS to output"),
        )
        .arg(
            Arg::with_name("no_js")
                .long("no-js")
                .required(false)
                .help("Do not add Javascript to output"),
        )
        .arg(
            Arg::with_name("extra_meta")
                .long("meta")
                .value_name("FILE")
                .help("extra meta to include in html head")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("extra_header")
                .long("header")
                .value_name("FILE")
                .help("extra html/markdown to include on top of html body")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("extra_footer")
                .long("footer")
                .value_name("FILE")
                .help("extra html/markdown to include at the end of html body")
                .takes_value(true),
        )
        .arg(
            clap::Arg::with_name("manifest-path")
                .long("manifest-path")
                .value_name("PATH")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("split_package")
                .long("split-package")
                .required(false)
                .help("whether to split output per package, defaults to false"),
        )
        .arg(
            Arg::with_name("split_example")
                .long("split-example")
                .required(false)
                .help("whether to split output per example, defaults to false"),
        )
        .arg(
            Arg::with_name("output_directory")
                .long("out-dir")
                .value_name("PATH")
                .required(true)
                .help("output directory"),
        )
        .get_matches();

    let options = Options {
        title: matches.value_of("title").unwrap_or("").to_string(),
        with_css: !matches.is_present("no_css"),
        with_js: !matches.is_present("no_js"),
        extra_meta: matches.value_of("extra_meta").map(|s| s.to_string()),
        extra_header: matches.value_of("extra_header").map(|s| s.to_string()),
        extra_footer: matches.value_of("extra_footer").map(|s| s.to_string()),
    };

    let mut cmd = cargo_metadata::MetadataCommand::new();

    if let Some(path) = matches.value_of("manifest-path") {
        cmd.manifest_path(path);
    }

    let metadata = cmd.exec().unwrap();

    let members = metadata
        .packages
        .iter()
        .filter(|p| metadata.workspace_members.iter().any(|m| *m == p.id));

    let members = if matches.is_present("split_package") {
        members
            .map(|package| {
                (
                    package.name.to_owned(),
                    package
                        .targets
                        .iter()
                        .filter(|t| t.kind.iter().any(|s| s == "example"))
                        .collect::<Vec<&cargo_metadata::Target>>(),
                )
            })
            .collect()
    } else {
        vec![(
            "index".to_owned(),
            members
                .map(|p| &p.targets)
                .flatten()
                .filter(|t| t.kind.iter().any(|s| s == "example"))
                .collect::<Vec<&cargo_metadata::Target>>(),
        )]
    };

    let output_dir = std::path::Path::new(matches.value_of("output_directory").unwrap());

    if !output_dir.exists() {
        fs::create_dir_all(output_dir).unwrap();
    } else if output_dir.is_file() {
        error!("output path {:?} is a file, aborting...", output_dir);
        std::process::exit(1);
    } else {
        info!(
            "output path {:?} already exists, files may be overwritten",
            output_dir
        );
    }

    members
        .iter()
        .filter(|(_, examples)| !examples.is_empty())
        .for_each(|(member, examples)| {
            let inputs = examples.iter().map(|e| {
                let mut blocks = extract(e.src_path.to_str().unwrap());
                let path = e.src_path.as_path();

                let title = path
                    .file_name()
                    .expect("Must be a file")
                    .to_str()
                    .unwrap_or("");

                let dir = path
                    .parent()
                    .map(|i| i.to_str().unwrap_or(""))
                    .unwrap_or("");

                blocks.insert(0, Block::new_file(title, dir));

                (e.name.to_owned(), blocks)
            });

            if matches.is_present("split_example") {
                let scope_dir = output_dir.join(member);

                if !scope_dir.exists() {
                    fs::create_dir_all(scope_dir.clone()).unwrap();
                } else if scope_dir.is_file() {
                    error!("  output path {:?} is a file, aborting...", scope_dir);
                    std::process::exit(1);
                } else {
                    info!(
                        "  output path {:?} already exists, files may be overwritten",
                        scope_dir
                    );
                }

                inputs.for_each(|(name, blocks)| {
                    let path = scope_dir.join(format!("{}.html", name));
                    let output = build_html(blocks, options.clone());

                    fs::write(path, output).expect("Could not write to output file.")
                });
            } else {
                let path = output_dir.join(format!("{}.html", member));
                let output =
                    build_html(inputs.map(|(_, blocks)| blocks).flatten(), options.clone());
                fs::write(path, output).expect("Could not write to output file.")
            }
        });
}
