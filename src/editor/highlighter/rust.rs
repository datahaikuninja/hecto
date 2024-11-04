use super::Highlighter;
use crate::editor::annotated_string::{Annotation, Style};
use crate::editor::buffer::Line;

pub struct RustSyntaxHighlighter {
    highlights: Vec<Vec<Annotation>>,
}

impl RustSyntaxHighlighter {
    pub fn new() -> Self {
        Self {
            highlights: Vec::new(),
        }
    }
    fn highlight_digits(line: &Line, annotations: &mut Vec<Annotation>) {
        // digit annotations
        line.get_raw_str()
            .chars()
            .enumerate()
            .for_each(|(idx, ch)| {
                if ch.is_ascii_digit() {
                    annotations.push(Annotation::new(Style::Digit, idx, idx + 1));
                }
            });
    }
}

impl Highlighter for RustSyntaxHighlighter {
    fn highlight_line(&mut self, line: &Line) {
        let mut annotations = vec![];
        Self::highlight_digits(line, &mut annotations);
        self.highlights.push(annotations);
    }
    fn get_annotations(&self, line_idx: usize) -> Vec<Annotation> {
        self.highlights[line_idx].clone()
    }
}
