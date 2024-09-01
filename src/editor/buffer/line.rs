use core::fmt;

use super::grapheme::{str_to_graphemes, Grapheme};

pub struct Line {
    graphemes: Vec<Grapheme>,
}

impl Line {
    pub fn from_str(s: &str) -> Self {
        let graphemes = str_to_graphemes(s);
        Self { graphemes }
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
        let mut result = String::new();
        for (i, grapheme) in self.graphemes.iter().enumerate() {
            if i == idx {
                result.push(c);
            }
            result.push_str(&grapheme.to_string());
        }
        if idx >= self.len() {
            result.push(c);
        }
        self.graphemes = str_to_graphemes(&result);
    }
    pub fn delete_grapheme(&mut self, idx: usize) {
        let mut result = String::new();

        for (i, grapheme) in self.graphemes.iter().enumerate() {
            if i != idx {
                result.push_str(&grapheme.to_string());
            }
        }
        self.graphemes = str_to_graphemes(&result);
    }
    pub fn push_line(&mut self, other: &Self) {
        let mut result = self.to_string();
        result.push_str(&other.to_string());
        self.graphemes = str_to_graphemes(&result);
    }
    pub fn split_off(&mut self, at: usize) -> Self {
        let remainder = self.graphemes.split_off(at);
        Self {
            graphemes: remainder,
        }
    }
}

impl fmt::Display for Line {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        let result: String = self
            .graphemes
            .iter()
            .map(|g| g.to_string().clone())
            .collect();
        write!(formatter, "{result}")
    }
}
