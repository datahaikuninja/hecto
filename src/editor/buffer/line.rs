use super::super::annotated_string::{AnnotatedString, Annotation};
use super::super::RenderContext;
use super::grapheme::{str_to_graphemes, Grapheme};

#[derive(Default)]
pub struct Line {
    graphemes: Vec<Grapheme>,
    raw_string: String,
    to_str_idx: Vec<usize>, // grapheme index to corresponding string index
}

impl Line {
    fn rebuild_fragments(&mut self) {
        let (graphemes, to_str_idx) = str_to_graphemes(&self.raw_string);
        self.graphemes = graphemes;
        self.to_str_idx = to_str_idx;
    }
    pub fn from_str(s: &str) -> Self {
        let (graphemes, to_str_idx) = str_to_graphemes(s);
        Self {
            graphemes,
            raw_string: String::from(s),
            to_str_idx,
        }
    }
    pub fn get_raw_str(&self) -> &str {
        &self.raw_string
    }
    pub fn get_nth_grapheme(&self, index: usize) -> Option<Grapheme> {
        self.graphemes.get(index).cloned()
    }
    pub fn calc_width_until_grapheme_index(&self, graphme_index: usize) -> usize {
        let mut current_pos = 0;
        for grapheme in self.graphemes.iter().take(graphme_index) {
            current_pos += grapheme.get_width_at_current_pos(current_pos);
        }
        current_pos
    }
    pub fn len(&self) -> usize {
        self.graphemes.len()
    }
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
    pub fn insert_char(&mut self, c: char, idx: usize) {
        if let Some(str_idx) = self.to_str_idx.get(idx) {
            self.raw_string.insert(*str_idx, c);
        } else {
            self.raw_string.push(c);
        }
        self.rebuild_fragments();
    }
    pub fn delete_grapheme(&mut self, idx: usize) {
        if let Some(grapheme) = self.graphemes.get(idx) {
            let start = self.to_str_idx[idx];
            let end = start + grapheme.to_string().len();
            self.raw_string.drain(start..end);
            self.rebuild_fragments();
        }
    }
    pub fn push_line(&mut self, other: &Self) {
        self.raw_string.push_str(&other.raw_string);
        self.rebuild_fragments();
    }
    pub fn split_off(&mut self, at: usize) -> Self {
        assert!(at <= self.graphemes.len());
        let str_idx = self
            .to_str_idx
            .get(at)
            .cloned()
            .unwrap_or(self.raw_string.len()); // just past the end
        let remainder = self.raw_string.split_off(str_idx);
        self.rebuild_fragments();
        Self::from_str(&remainder)
    }
    fn to_byte_idx(&self, grapheme_idx: usize) -> usize {
        self.to_str_idx
            .get(grapheme_idx)
            .cloned()
            .unwrap_or(self.raw_string.len())
    }
    fn to_grapheme_idx(&self, str_idx: usize) -> usize {
        for (grapheme_idx, cur_str_idx) in self.to_str_idx.iter().enumerate() {
            if *cur_str_idx >= str_idx {
                return grapheme_idx;
            }
        }
        panic!("Error: str index is out of bound");
    }
    pub fn search_all_occurence(&self, pattern: &str) -> Vec<(usize, usize)> {
        let mut result = vec![];
        if pattern.is_empty() {
            return result;
        }
        let mut start_index = 0;
        while let Some(grapheme_idx) = self.search(pattern, start_index) {
            let start = grapheme_idx;
            let end = start + pattern.len();
            result.push((start, end));
            start_index = grapheme_idx + 1;
        }
        result
    }
    pub fn search(&self, pattern: &str, start_idx: usize) -> Option<usize> {
        if self.is_empty() {
            return None;
        }
        let byte_index = self.to_str_idx[start_idx];
        self.raw_string[byte_index..]
            .find(pattern)
            .map(|str_idx| byte_index + self.to_grapheme_idx(str_idx))
    }
}

impl std::fmt::Display for Line {
    fn fmt(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(formatter, "{}", self.raw_string)
    }
}

pub struct LineView<'a> {
    line: &'a Line,
    padding_left: String,
    padding_right: String,
    visible_range: (usize, usize),
}

impl<'a> LineView<'a> {
    pub fn new(line: &'a Line, left: usize, right: usize) -> Self {
        // make view from terminal column range [left, right) for line
        let mut current_pos = 0;
        let mut padding_left = String::new();
        let mut padding_right = String::new();
        let mut left_grapheme_idx = usize::MAX;
        let mut right_grapheme_idx = usize::MAX;

        for (i, grapheme) in line.graphemes.iter().enumerate() {
            let next_pos = current_pos + grapheme.get_width_at_current_pos(current_pos);
            // Current character is out of visible range
            if next_pos <= left || current_pos >= right {
                current_pos = next_pos;
                continue;
            }
            let right_end_visible = left <= next_pos - 1 && next_pos - 1 < right;
            let left_end_visible = left <= current_pos && current_pos < right;
            assert!(right_end_visible || left_end_visible);
            if !left_end_visible {
                padding_left.push('<');
            } else if !right_end_visible {
                padding_right.push('>');
            } else {
                left_grapheme_idx = usize::min(left_grapheme_idx, i);
                right_grapheme_idx = i + 1;
            }
            current_pos = next_pos;
        }

        Self {
            line,
            padding_left,
            padding_right,
            visible_range: (left_grapheme_idx, right_grapheme_idx),
        }
    }
    pub fn build_rendered_str(&self, context: &RenderContext) -> AnnotatedString {
        let search_hits = self.line.search_all_occurence(&context.search_pattern);
        let mut content = AnnotatedString::from_str(self.line.get_raw_str());
        for (match_start, match_end) in search_hits {
            content.add_annotation(Annotation::new(match_start, match_end));
        }
        let start = self.line.to_byte_idx(self.visible_range.0);
        let end = self.line.to_byte_idx(self.visible_range.1);
        let visible_content = content.substr(start, end);
        let mut result = AnnotatedString::default();
        result.push_annot_str(&AnnotatedString::from_str(&self.padding_left));
        result.push_annot_str(&visible_content);
        result.push_annot_str(&AnnotatedString::from_str(&self.padding_right));
        result
    }
}
