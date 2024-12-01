use super::Highlighter;
use crate::editor::annotated_string::{Annotation, Style};
use crate::editor::buffer::Line;

const KEYWORDS: [&str; 12] = [
    "fn", "mod", "use", "pub", "if", "else", "for", "in", "struct", "impl", "let", "match",
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

impl Highlighter for RustSyntaxHighlighter {
    fn highlight_line(&mut self, line: &Line) {
        let mut annotations = vec![];
        for (idx, word) in line.split_word_bound_indices() {
            let annotation = if is_number(word) {
                Some(Annotation::new(Style::Digit, idx, idx + word.len()))
            } else if is_keyword(word) {
                Some(Annotation::new(Style::Keywords, idx, idx + word.len()))
            } else {
                None
            };
            annotation.map_or((), |annot| annotations.push(annot))
        }
        self.highlights.push(annotations);
    }
    fn get_annotations(&self, line_idx: usize) -> Vec<Annotation> {
        self.highlights[line_idx].clone()
    }
}
