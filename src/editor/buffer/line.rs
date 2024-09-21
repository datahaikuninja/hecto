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
    pub fn get_nth_grapheme(&self, index: usize) -> Option<Grapheme> {
        self.graphemes.get(index).cloned()
    }
    pub fn get_visible_graphemes(&self, left: usize, right: usize) -> String {
        let mut result = String::new();
        let mut current_pos = 0;
        for grapheme in &self.graphemes {
            let next_pos = current_pos + grapheme.get_width_at_current_pos(current_pos);
            if current_pos >= right {
                break;
            }
            if next_pos > left {
                // Replace cut-off text with '>' or '<'.
                if next_pos > right {
                    result.push('>');
                } else if current_pos < left {
                    result.push('<');
                }
                // add fully visible grapheme
                else {
                    result.push_str(&grapheme.to_string());
                }
            }
            current_pos = next_pos;
        }
        result
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
        let str_idx = self.to_str_idx[at];
        let remainder = self.raw_string.split_off(str_idx);
        self.rebuild_fragments();
        Self::from_str(&remainder)
    }
}

impl std::fmt::Display for Line {
    fn fmt(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(formatter, "{}", self.raw_string)
    }
}
