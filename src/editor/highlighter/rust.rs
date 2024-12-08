use super::Highlighter;
use crate::editor::annotated_string::{Annotation, Style};
use crate::editor::buffer::Line;

const KEYWORDS: [&str; 12] = [
    "fn", "mod", "use", "pub", "if", "else", "for", "in", "struct", "impl", "let", "match",
];

const TYPE_NAMES: [&str; 21] = [
    "i8", "i16", "i32", "i64", "i128", "isize", // signed integeres
    "u8", "u16", "u32", "u64", "u128", "usize", // unsigned integeres
    "f32", "f64", // floating point numbers
    "bool", "char", "str", // other builtin types
    "Option", "Result", // enum types in std::prelude
    "String", "Vec", // struct types in std::prelude
];

const VARIANT_NAMES: [&str; 6] = [
    "true", "false", // bool
    "Some", "None", // Option
    "Ok", "Err", // Result
];

pub struct RustSyntaxHighlighter {
    highlights: Vec<Vec<Annotation>>,
}

impl RustSyntaxHighlighter {
    pub fn new() -> Self {
        Self {
            highlights: Vec::new(),
        }
    }
}

fn is_number(word: &str) -> bool {
    word.chars().all(|char| char.is_ascii_digit())
}

fn is_keyword(word: &str) -> bool {
    KEYWORDS.contains(&word)
}

fn is_type_name(word: &str) -> bool {
    TYPE_NAMES.contains(&word)
}

fn is_variant_name(word: &str) -> bool {
    VARIANT_NAMES.contains(&word)
}

fn is_single_line_comment_start(word: &str) -> bool {
    word.starts_with("//")
}

impl Highlighter for RustSyntaxHighlighter {
    fn highlight_line(&mut self, line: &Line) {
        let mut annotations = vec![];
        let mut in_comment = false;
        for (idx, word) in line.split_word_bound_indices() {
            let annotation = if is_single_line_comment_start(&line.get_raw_str()[idx..]) {
                in_comment = true;
                Some(Annotation::new(Style::Comment, idx, idx + line.byte_len()))
            } else if is_number(word) {
                Some(Annotation::new(Style::Digit, idx, idx + word.len()))
            } else if is_keyword(word) {
                Some(Annotation::new(Style::Keywords, idx, idx + word.len()))
            } else if is_type_name(word) {
                Some(Annotation::new(Style::TypeName, idx, idx + word.len()))
            } else if is_variant_name(word) {
                Some(Annotation::new(Style::VarinatName, idx, idx + word.len()))
            } else {
                None
            };
            let _ = annotation.map_or((), |annot| annotations.push(annot));
            if in_comment {
                break;
            }
        }
        self.highlights.push(annotations);
    }
    fn get_annotations(&self, line_idx: usize) -> Vec<Annotation> {
        self.highlights[line_idx].clone()
    }
}
