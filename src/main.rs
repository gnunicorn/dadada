
use clap::{Arg, App};
use std::fs;
use std::path::Path;

use dadada::{Block, extract, build_html};

fn main() {
    let matches = App::new("dadada")
        .version("0.9.1")
        .author("Benjamin Kampmann <ben@gnunicorn.org>, Rui Vieira <ruidevieira@googlemail.com>")
        .about("Artisanal Rust inlined code documentation renderer")
        .arg(Arg::with_name("output")
            .short("o")
            .long("output")
            .value_name("FILE")
            .help("target file to render to, stdout if not given")
            .takes_value(true))

        .arg(Arg::with_name("input")
            .value_name("FILE")
            .help("rust source files")
            .required(true)
            .multiple(true)
            .takes_value(true))
        .get_matches();

    let output = build_html(matches.values_of("input").expect("This is required")
        .map(|i| {
            let mut blocks = extract(i.to_string());
            let path = Path::new(i);
            let title = path.file_name().expect("Must be a file").to_str().unwrap_or("");
            blocks.insert(0, Block::new_file(title, i));
            blocks
        }).flatten()
    );

    match matches.value_of("output") {
        Some(f) => fs::write(f, output).expect("Could not write to output file."),
        None => println!("{}",  output),
    }
}

