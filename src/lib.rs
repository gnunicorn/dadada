use pulldown_cmark::{html, Parser};
use std::cmp::PartialEq;
use std::fs::File;
use std::io::{BufRead, BufReader, Read};
use std::iter::IntoIterator;
use std::path::Path;

// `Block` stores code sections, consisting of comments and associated code.
// We initialise a new block with empty `Vec` which will later be joined.
pub struct Block {
    comment: Vec<String>,
    code: Vec<String>,
    starting_line: usize,
}

/// Rendering Options
pub struct Options {
    /// HTML title to include
    pub title: String,
    /// Whether to include the static css
    pub with_css: bool,
    /// Whether to include the static javascript
    pub with_js: bool,
    /// Filepath with extra for meta
    pub extra_meta: Option<String>,
    /// Filepath with extra for header
    pub extra_header: Option<String>,
    /// Filepath with extra for footer
    pub extra_footer: Option<String>,
}

impl Block {
    pub fn new(starting_line: usize) -> Block {
        Block {
            comment: Vec::new(),
            code: Vec::new(),
            starting_line,
        }
    }

    pub fn new_file(title: &str, path: &str) -> Block {
        Block {
            comment: vec![format!("**`{:}`** (in `{:}`)", title, path)],
            code: vec![],
            starting_line: 0,
        }
    }

    pub fn has_code(&self) -> bool {
        if self.code.is_empty() {
            return false;
        }
        self.code.iter().find(|i| i.trim().len() > 0).is_some()
    }
}

#[derive(PartialEq)]
enum CommentType {
    Simple,
    Bang,
    Doc,
    ANY,
}

// We divide the source code into code/comment blocks.
// A `Vec` of `Block`s is returned for further processing.
pub fn extract(path: String) -> Vec<Block> {
    let file = File::open(path).expect("Unable to open input file");
    let mut process_as_code = false;
    let mut current_comment_type: CommentType = CommentType::ANY;
    let mut blocks: Vec<Block> = Vec::new();
    let mut current_block = Block::new(1);

    for (idx, line) in BufReader::new(file).lines().into_iter().enumerate() {
        let line_str = line.unwrap().to_string();
        let stripped = line_str.trim();

        if stripped.starts_with("//") {
            if process_as_code {
                blocks.push(current_block);
                current_block = Block::new(idx + 1);
            }
            process_as_code = false;
        } else {
            process_as_code = true;
            current_comment_type = CommentType::ANY;
        }

        if process_as_code {
            current_block.code.push(line_str.to_string());
        } else {
            let (strip_pos, com_type) = {
                if stripped.starts_with("///") {
                    (3, CommentType::Doc)
                } else if stripped.starts_with("//!") {
                    (3, CommentType::Bang)
                } else if stripped.starts_with("// !") {
                    (4, CommentType::Bang)
                } else {
                    (2, CommentType::Simple)
                }
            };

            let line = stripped.split_at(strip_pos).1;
            if current_comment_type != CommentType::ANY && com_type != current_comment_type {
                // different type of comment, means we assume a new block
                blocks.push(current_block);
                current_block = Block::new(idx + 1);
            }
            current_comment_type = com_type;
            current_block.comment.push(line.trim().to_string());
        }
    }
    blocks.push(current_block);
    blocks
}

// Build a full HTML document from a vector of blocks.
// This function also inlines the CSS.
pub fn build_html<I: IntoIterator<Item = Block>>(blocks: I, options: Options) -> String {
    let mut html_output = String::new();

    let include_static = |file: String, mut target: &mut String| {
        let path = Path::new(&file);
        let is_md = if let Some(ext) = path.extension() {
            match ext.to_str() {
                Some("md") | Some("mdown") | Some("markdown") => true,
                _ => false,
            }
        } else {
            false
        };

        let mut f = File::open(path).expect("File not found");
        if is_md {
            let mut source = String::new();
            f.read_to_string(&mut source).expect("failed  to read file");
            html::push_html(&mut target, Parser::new(&source));
        } else {
            f.read_to_string(&mut target).expect("failed to read file");
        };
    };

    html_output.push_str(&format!(
        include_str!("static/head.html"),
        title = options.title
    ));

    if options.with_css {
        html_output.push_str("<style>");
        html_output.push_str(include_str!("static/style.css"));
        html_output.push_str("</style>");
    };

    if options.with_js {
        html_output.push_str("<script>");
        html_output.push_str(include_str!("static/prism.min.js"));
        html_output.push_str(include_str!("static/prism-rust.min.js"));
        html_output.push_str(include_str!("static/line-numbers.js"));
        html_output.push_str("</script>");
    };

    options
        .extra_meta
        .map(|f| include_static(f, &mut html_output));

    html_output.push_str("</head><body>");

    options
        .extra_header
        .map(|f| include_static(f, &mut html_output));

    html_output.push_str("<div id=\"container\"><div id=\"main\">");

    for (i, block) in blocks.into_iter().enumerate() {
        html_output.push_str(&format!(
            include_str!("static/block_before.html"),
            index = i
        ));

        html::push_html(&mut html_output, Parser::new(&block.comment.join("\n")));

        if block.has_code() {
            html_output.push_str(&format!(
                include_str!("static/block_code.html"),
                code = block.code.join("\n").replace("<", "&lt;"),
                start = block.starting_line
            ));
        }

        html_output.push_str(include_str!("static/block_after.html"));
    }

    html_output.push_str("</div></div>");

    options
        .extra_footer
        .map(|f| include_static(f, &mut html_output));

    html_output.push_str("</body></html>");
    html_output
}
