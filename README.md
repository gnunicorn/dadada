# dadada - [by trio](https://www.youtube.com/watch?v=lNYcviXK4rg&app=desktop)

Artisanal Rust inlined code documentation renderer (forked off from [`dada` by Rui Vieira](https://gitlab.com/ruivieira/dada); inspired by `docco`)

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

 - [ ] [Improve Readme](https://github.com/gnunicorn/dadada/issues/2)
 - [ ] [Make html mobile-friendly / responsive](https://github.com/gnunicorn/dadada/issues/1)
 - [x] [Remove remote includes from html](https://github.com/gnunicorn/dadada/issues/8)
 - [x] [Add `--title`-CLI parameter](https://github.com/gnunicorn/dadada/issues/3) to add title
 - [ ] additional [`--meta`, `--header`  and `--footer` parameters](https://github.com/gnunicorn/dadada/issues/4) to allow for easy customisation
 - [ ] [`travis.yml` example](https://github.com/gnunicorn/dadada/issues/6) to build on push
 - [ ] [`build.rs` script-example](https://github.com/gnunicorn/dadada/issues/5) to automagically build all examples of a crate
 - [ ] [Integration and regression test suite](https://github.com/gnunicorn/dadada/issues/7)