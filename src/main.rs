use clap::{App, Arg};
use std::fs;
use std::path::Path;

use dadada::{build_html, extract, Block, Options};

fn main() {
    let matches = App::new("dadada")
        .version("0.9.5-dev")
        .author("Benjamin Kampmann <ben@gnunicorn.org>, Rui Vieira <ruidevieira@googlemail.com>")
        .about("Artisanal Rust inlined code documentation renderer")
        .arg(
            Arg::with_name("title")
                .short("t")
                .long("title")
                .value_name("String")
                .help("The HTML title to render")
                .takes_value(true),
        )
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
            Arg::with_name("output")
                .short("o")
                .long("output")
                .value_name("FILE")
                .help("target file to render to, stdout if not given")
                .takes_value(true),
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
            Arg::with_name("input")
                .value_name("FILE")
                .help("rust source files")
                .required(true)
                .multiple(true)
                .takes_value(true),
        )
        .get_matches();

    let output = build_html(
        matches
            .values_of("input")
            .expect("This is required")
            .map(|i| {
                let mut blocks = extract(i.to_string());
                let path = Path::new(i);
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
                blocks
            })
            .flatten(),
        Options {
            title: matches.value_of("title").unwrap_or("").to_string(),
            with_css: !matches.is_present("no_css"),
            with_js: !matches.is_present("no_js"),
            extra_meta: matches.value_of("extra_meta").map(|s| s.to_string()),
            extra_header: matches.value_of("extra_header").map(|s| s.to_string()),
            extra_footer: matches.value_of("extra_footer").map(|s| s.to_string()),
        },
    );

    match matches.value_of("output") {
        Some(f) => fs::write(f, output).expect("Could not write to output file."),
        None => println!("{}", output),
    }
}
