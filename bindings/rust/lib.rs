//! This crate provides Bats language support for the [tree-sitter] parsing library.
//!
//! Typically, you will use the [`LANGUAGE`] constant to add this language to a
//! tree-sitter [`Parser`], and then use the parser to parse some code:
//!
//! ```
//! let code = r#"
//! "#;
//! let mut parser = tree_sitter::Parser::new();
//! let language = tree_sitter_bats::LANGUAGE;
//! parser
//!     .set_language(&language.into())
//!     .expect("Error loading Bats parser");
//! let tree = parser.parse(code, None).unwrap();
//! assert!(!tree.root_node().has_error());
//! ```
//!
//! [`Parser`]: https://docs.rs/tree-sitter/0.27.0/tree_sitter/struct.Parser.html
//! [tree-sitter]: https://tree-sitter.github.io/

use tree_sitter_language::LanguageFn;

unsafe extern "C" {
    fn tree_sitter_bats() -> *const ();
}

/// The tree-sitter [`LanguageFn`] for this grammar.
pub const LANGUAGE: LanguageFn = unsafe { LanguageFn::from_raw(tree_sitter_bats) };

/// The content of the [`node-types.json`] file for this grammar.
///
/// [`node-types.json`]: https://tree-sitter.github.io/tree-sitter/using-parsers/6-static-node-types
pub const NODE_TYPES: &str = include_str!("../../src/node-types.json");

#[cfg(with_highlights_query)]
/// The syntax highlighting query for this grammar.
pub const HIGHLIGHTS_QUERY: &str = include_str!("../../queries/highlights.scm");

#[cfg(with_injections_query)]
/// The language injection query for this grammar.
pub const INJECTIONS_QUERY: &str = include_str!("../../queries/injections.scm");

#[cfg(with_locals_query)]
/// The local variable query for this grammar.
pub const LOCALS_QUERY: &str = include_str!("../../queries/locals.scm");

#[cfg(with_tags_query)]
/// The symbol tagging query for this grammar.
pub const TAGS_QUERY: &str = include_str!("../../queries/tags.scm");

#[cfg(test)]
mod tests {
    #[test]
    fn test_can_load_grammar() {
        let mut parser = tree_sitter::Parser::new();
        parser
            .set_language(&super::LANGUAGE.into())
            .expect("Error loading Bats parser");
    }

    // Highlighters disagree on which of two patterns matching one node wins
    // (tree-sitter-highlight takes the first, Zed the last), so every node
    // gets at most one highlight capture. Checks the highlight tests, and the
    // example suites when script/fetch-examples has fetched them.
    #[cfg(with_highlights_query)]
    #[test]
    fn test_highlights_capture_each_node_once() {
        use std::collections::HashMap;
        use std::path::{Path, PathBuf};
        use tree_sitter::StreamingIterator;

        fn bats_files(dir: &Path, files: &mut Vec<PathBuf>) {
            let Ok(entries) = std::fs::read_dir(dir) else {
                return;
            };
            for path in entries.map(|entry| entry.unwrap().path()) {
                if path.is_dir() {
                    bats_files(&path, files);
                } else if path.extension().is_some_and(|ext| ext == "bats") {
                    files.push(path);
                }
            }
        }

        let root = Path::new(env!("CARGO_MANIFEST_DIR"));
        let mut files = Vec::new();
        bats_files(&root.join("test/highlight"), &mut files);
        bats_files(&root.join("examples"), &mut files);
        assert!(!files.is_empty(), "no highlight tests found");

        let language = super::LANGUAGE.into();
        let query = tree_sitter::Query::new(&language, super::HIGHLIGHTS_QUERY)
            .expect("Error compiling highlights query");
        let mut parser = tree_sitter::Parser::new();
        parser.set_language(&language).unwrap();
        let mut overlaps = Vec::new();
        for path in &files {
            let source = std::fs::read(path).unwrap();
            let tree = parser.parse(&source, None).unwrap();
            let mut cursor = tree_sitter::QueryCursor::new();
            let mut captures: HashMap<usize, Vec<&str>> = HashMap::new();
            let mut matches = cursor.matches(&query, tree.root_node(), source.as_slice());
            while let Some(found) = matches.next() {
                for capture in found.captures() {
                    let name = query.capture_names()[capture.index as usize];
                    let names = captures.entry(capture.node.id()).or_default();
                    names.push(name);
                    if names.len() == 2 {
                        let point = capture.node.start_position();
                        overlaps.push(format!(
                            "{}:{}:{} {} ({})",
                            path.strip_prefix(root).unwrap().display(),
                            point.row + 1,
                            point.column + 1,
                            capture.node.kind(),
                            names.join(", "),
                        ));
                    }
                }
            }
        }
        assert!(
            overlaps.is_empty(),
            "nodes with several captures:\n{}",
            overlaps.join("\n")
        );
    }
}
