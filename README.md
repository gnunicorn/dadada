# [dadada](https://github.com/gnunicorn/dadada) - [by trio](https://www.youtube.com/watch?v=lNYcviXK4rg&app=desktop)

![Crates.io](https://img.shields.io/crates/v/dadada?style=flat-square) ![License: MIT](https://img.shields.io/crates/l/dadada?style=flat-square) ![GitHub last commit](https://img.shields.io/github/last-commit/gnunicorn/dadada?style=flat-square) ![Travis (.org)](https://img.shields.io/travis/gnunicorn/dadada?style=flat-square)

Artisanal Rust inlined code documentation renderer

**Here Screenshot** / Action video

## Install

You can install it quite easily with `cargo install dadada`.

## Usage

`dadada` will be installed as command line tool for you to run. Just specify the files you want to have rendered and it will do so. If you do not provide a target output file (`--output` / `-o`) the result will be printed to stdout.

The full list of cli arguments is:
```
Benjamin Kampmann <ben@gnunicorn.org>, Rui Vieira <ruidevieira@googlemail.com>
Artisanal Rust inlined code documentation renderer

USAGE:
    dadada [FLAGS] [OPTIONS] <FILE>...

FLAGS:
    -h, --help       Prints help information
        --no-css     Do not add CSS to output
        --no-js      Do not add Javascript to output
    -V, --version    Prints version information

OPTIONS:
    -o, --output <FILE>     target file to render to, stdout if not given
    -t, --title <String>    The HTML title to render

ARGS:
    <FILE>...    rust source files
```

An example to render all the rust files in your crate under `target/dadada-output.html` therefor would be: `dadada --title "All my Code Example" -o target/dadada-output.html src/*.rs`.


## ToDo's [towards 1.0](https://github.com/gnunicorn/dadada/milestone/1)
_[Help wanted](https://github.com/gnunicorn/dadada/labels/help%20wanted)_!

 - [-] [Improve Readme](https://github.com/gnunicorn/dadada/issues/2)
 - [x] [Make html mobile-friendly / responsive](https://github.com/gnunicorn/dadada/issues/1)
 - [x] [Remove remote includes from html](https://github.com/gnunicorn/dadada/issues/8)
 - [x] [Add `--title`-CLI parameter](https://github.com/gnunicorn/dadada/issues/3) to add title
 - [ ] additional [`--meta`, `--header`  and `--footer` parameters](https://github.com/gnunicorn/dadada/issues/4) to allow for easy customisation
 - [ ] [`travis.yml` example](https://github.com/gnunicorn/dadada/issues/6) to build on push
 - [ ] [`build.rs` script-example](https://github.com/gnunicorn/dadada/issues/5) to automagically build all examples of a crate
 - [x] [Integration and regression test suite](https://github.com/gnunicorn/dadada/issues/7)

## Credits

This is inspired [by `docco` by Bardadym Denis](https://github.com/btd/docco) (sadly discontinued) and based off of the Rust tool [`dada` by Rui Vieira](https://gitlab.com/ruivieira/dada)