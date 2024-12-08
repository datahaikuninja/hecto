use super::Highlighter;
use crate::editor::annotated_string::{Annotation, Style};
use crate::editor::buffer::Line;
use crate::editor::RenderContext;

pub struct SearchHighlighter<'a> {
    highlights: Vec<Vec<Annotation>>,
    render_context: &'a RenderContext,
}

impl<'a> SearchHighlighter<'a> {
    pub fn new(render_context: &'a RenderContext) -> Self {
        Self {
            highlights: Vec::new(),
            render_context,
        }
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
}

impl<'a> Highlighter for SearchHighlighter<'a> {
    fn highlight_line(&mut self, line: &Line) {
        let mut annotations = vec![];
        self.highlight_search(line, &mut annotations);
        self.highlights.push(annotations);
    }
    fn get_annotations(&self, line_idx: usize) -> Vec<Annotation> {
        self.highlights[line_idx].clone()
    }
}
