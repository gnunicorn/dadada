
use clap::{Arg, App};
use std::fs;

use dadada::{extract, build_html};

fn main() {
    let matches = App::new("dadada")
        .version("0.0.1")
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
        .map(|i| extract(i.to_string())
        ).flatten()
    );

    match matches.value_of("output") {
        Some(f) => fs::write(f, output).expect("Could not write to output file."),
        None => println!("{}",  output),
    }
}

