mod rust;
mod search_highlight;

use super::{annotated_string::Annotation, buffer::Line, RenderContext};
use crate::editor::filetype::FileType;
use search_highlight::SearchHighlighter;

trait Highlighter {
    fn highlight_line(&mut self, line: &Line);
    fn get_annotations(&self, line_idx: usize) -> Vec<Annotation>;
}

fn create_syntax_highlighter(file_type: FileType) -> Option<Box<dyn Highlighter>> {
    match file_type {
        FileType::Rust => Some(Box::new(rust::RustSyntaxHighlighter::new())),
        FileType::Text => None,
    }
}

pub struct HighlighterBundler<'a> {
    // line index to annotations of line.
    syntax_highlighter: Option<Box<dyn Highlighter>>,
    search_highlighter: SearchHighlighter<'a>,
}

impl<'a> HighlighterBundler<'a> {
    pub fn new(context: &'a RenderContext) -> Self {
        Self {
            syntax_highlighter: create_syntax_highlighter(context.file_type),
            search_highlighter: SearchHighlighter::new(context),
        }
    }

    pub fn highlight_line(&mut self, line: &Line) {
        if self.syntax_highlighter.is_some() {
            self.syntax_highlighter
                .as_mut()
                .unwrap()
                .highlight_line(line);
        }
        self.search_highlighter.highlight_line(line);
    }

    pub fn get_annotations(&self, line_idx: usize) -> Vec<Annotation> {
        let mut annotations = self.search_highlighter.get_annotations(line_idx);
        if self.syntax_highlighter.is_some() {
            annotations.append(
                &mut self
                    .syntax_highlighter
                    .as_ref()
                    .unwrap()
                    .get_annotations(line_idx),
            );
        }
        annotations
    }
}

pub struct LineHighlighter<'a> {
    highlighter: &'a HighlighterBundler<'a>,
    line_idx: usize,
}

impl<'a> LineHighlighter<'a> {
    pub fn new(highlighter: &'a HighlighterBundler, line_idx: usize) -> Self {
        Self {
            highlighter,
            line_idx,
        }
    }
    pub fn get_annotations(&self) -> Vec<Annotation> {
        self.highlighter.get_annotations(self.line_idx)
    }
}