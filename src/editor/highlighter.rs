use super::{
    annotated_string::{Annotation, Style},
    buffer::Line,
    RenderContext,
};

pub struct Highlighter<'a> {
    // line index to annotations of line.
    highlights: Vec<Vec<Annotation>>,
    render_context: &'a RenderContext,
}

impl<'a> Highlighter<'a> {
    pub fn new(context: &'a RenderContext) -> Self {
        Self {
            highlights: Vec::new(),
            render_context: context,
        }
    }

    pub fn highlight_line(&mut self, line: &Line) {
        let mut annotations = vec![];
        Self::highlight_digits(line, &mut annotations);
        self.highlight_search(line, &mut annotations);
        self.highlights.push(annotations);
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
        self.highlights[line_idx].clone()
    }
}

pub struct LineHighlighter<'a> {
    highlighter: &'a Highlighter<'a>,
    line_idx: usize,
}

impl<'a> LineHighlighter<'a> {
    pub fn new(highlighter: &'a Highlighter, line_idx: usize) -> Self {
        Self {
            highlighter,
            line_idx,
        }
    }
    pub fn get_annotations(&self) -> Vec<Annotation> {
        self.highlighter.get_annotations(self.line_idx)
    }
}
