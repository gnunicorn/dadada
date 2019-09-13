
use std::fs::File;
use std::io::{BufRead, BufReader, Result};
use std::io::prelude::*;
use std::thread::current;
use std::fs;
use std::path::Path;
use std::iter::IntoIterator;
use pulldown_cmark::{Parser, html};

// `Block` stores code sections, consisting of comments and associated code.
// We initialise a new block with empty `Vec` which will later be joined.
pub struct Block {
    comment: Vec<String>,
    code: Vec<String>
}

impl Block {
    pub fn new() -> Block {
        return Block {
            comment: Vec::new(),
            code: Vec::new()
        }
    }

    pub fn title(title: &str) -> Block {
        return Block {
            comment: vec![format!("---\nFile: `{:}`", title)],
            code: Vec::new()
        }

    }
}

// We divide the source code into code/comment blocks.
// A `Vec` of `Block`s is returned for further processing.
pub fn extract(path: String) -> Vec<Block> {
    let file = File::open(path).expect("Unable to open the file");
    let mut process_as_code = false;
    let mut blocks: Vec<Block> = Vec::new();
    let mut current_block = Block::new();

    for line in BufReader::new(file).lines() {

        let line_str = line.unwrap().to_string();
        let stripped = line_str.trim();

        // ignore empty lines. Just one big block.
        if stripped.len() == 0 { 
            if !process_as_code {
                current_block.comment.push("\n".to_owned());
            }
            continue
         }

        if stripped.starts_with("//") {
            if process_as_code {
                blocks.push(current_block);
                current_block = Block::new();
            }
            process_as_code = false;
        } else {
            process_as_code = true;
        }

        if process_as_code {
            current_block.code.push(line_str.to_string());
        } else {
            let line = stripped.split_at({
                if stripped.starts_with("///")  || stripped.starts_with("//!") {
                    3
                } else if stripped.starts_with("// !") {
                    4
                } else {
                    2
                }
            }).1;
            current_block.comment.push(line.trim().to_string());
        }
    }
    blocks.push(current_block);
    return blocks;
}

// Build a full HTML document from a vector of blocks.
// This function also inlines the CSS.
pub fn build_html<I: IntoIterator<Item=Block>>(blocks: I) -> String {
    let css = include_str!("style.css").to_string();
    let mut html_output = String::new();

    for (i, block) in blocks.into_iter().enumerate() {
        html_output.push_str(&format!(include_str!("block_before.html"), index=i));
        html::push_html(&mut html_output, Parser::new(&block.comment.join("\n")));
        html_output.push_str(&format!(include_str!("block_after.html"), code=block.code.join("\n")));
    }

    return format!(include_str!("template.html"),
                       title="dada",
                       css=css,
                       blocks=html_output);
}