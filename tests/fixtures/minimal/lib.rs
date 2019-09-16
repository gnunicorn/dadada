//! ## Minimal example file
//! With an introduction header
//! 

// some imports
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::iter::IntoIterator;
use std::cmp::PartialEq;
use pulldown_cmark::{Parser, html};

// Some struct definition
// `Block` stores code sections, consisting of comments and associated code.
// We initialise a new block with empty `Vec` which will later be joined.
pub struct Block {
    comment: Vec<String>,
    code: Vec<String>,
    /// With a doc comment
    starting_line: usize,
}

/// and a a main function, with doc comment
pub fn main() {
    //
    // As this is only for tests
    for i in ..100 {
        // ! We do not mind, that this isn't actually doing anything
    }
}
