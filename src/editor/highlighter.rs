mod rust;

use super::{
    annotated_string::{Annotation, Style},
    buffer::Line,
    RenderContext,
};
use crate::editor::filetype::FileType;

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
    highlights: Vec<Vec<Annotation>>,
    syntax_highlighter: Option<Box<dyn Highlighter>>,
    render_context: &'a RenderContext,
}

impl<'a> HighlighterBundler<'a> {
    pub fn new(context: &'a RenderContext) -> Self {
        Self {
            highlights: Vec::new(),
            syntax_highlighter: create_syntax_highlighter(context.file_type),
            render_context: context,
        }
    }

    pub fn highlight_line(&mut self, line: &Line) {
        let mut annotations = vec![];
        if self.syntax_highlighter.is_some() {
            self.syntax_highlighter
                .as_mut()
                .unwrap()
                .highlight_line(line);
        }
        self.highlight_search(line, &mut annotations);
        self.highlights.push(annotations);
    }

    fn highlight_search(&self, line: &Line, annotations: &mut Vec<Annotation>) {
        // search result annotations
        let search_hits = match self.render_context.search_pattern.as_deref() {
            Some(s) => line.search_all_occurence(s),
            None => vec![],
        };
        for (match_start, match_end) in search_hits {
            annotations.push(Annotation::new(Style::SearchHit, match_start, match_end));
        }
    }

    pub fn get_annotations(&self, line_idx: usize) -> Vec<Annotation> {
        let mut annotations = self.highlights[line_idx].clone();
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
