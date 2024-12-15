mod rust;
mod search_highlight;

use super::{annotated_string::Annotation, buffer::Line, RenderContext};
use crate::editor::filetype::FileType;
use search_highlight::SearchHighlighter;

struct HighlightContext {
    in_multiline_comment: bool,
    in_string_literal: bool,
}

trait Highlighter {
    fn highlight_line(&mut self, line: &Line, ctx: &mut HighlightContext);
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
    highlight_context: HighlightContext,
}

impl<'a> HighlighterBundler<'a> {
    pub fn new(context: &'a RenderContext) -> Self {
        Self {
            syntax_highlighter: create_syntax_highlighter(context.file_type),
            search_highlighter: SearchHighlighter::new(context),
            highlight_context: HighlightContext {
                in_multiline_comment: false,
                in_string_literal: false,
            },
        }
    }

    pub fn highlight_line(&mut self, line: &Line) {
        if self.syntax_highlighter.is_some() {
            self.syntax_highlighter
                .as_mut()
                .unwrap()
                .highlight_line(line, &mut self.highlight_context);
        }
        self.search_highlighter
            .highlight_line(line, &mut self.highlight_context);
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
