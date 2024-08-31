use core::fmt;

use unicode_segmentation::UnicodeSegmentation;
use unicode_width::UnicodeWidthStr;

#[derive(Copy, Clone)]
enum GraphemeWidth {
    Half,
    Full,
}

impl GraphemeWidth {
    pub fn from_usize(w: usize) -> Self {
        match w {
            0 | 1 => Self::Half,
            2 => Self::Full,
            _ => panic!("Invalid grapheme width"),
        }
    }
    pub fn to_usize(&self) -> usize {
        match self {
            Self::Half => 1,
            Self::Full => 2,
        }
    }
}

fn calc_tab_width(current_pos: usize) -> usize {
    let tabstop = 8;
    (current_pos / tabstop + 1) * tabstop - current_pos
}

#[derive(Clone)]
pub struct Grapheme {
    string: String,
    width: GraphemeWidth,
}

impl Grapheme {
    pub fn is_tab(&self) -> bool {
        self.string
            .chars()
            .next()
            .expect("contents of grapheme should not be empty")
            == '\t'
    }
    fn get_width_at_current_pos(&self, current_pos: usize) -> usize {
        if self.is_tab() {
            calc_tab_width(current_pos)
        } else {
            self.width.to_usize()
        }
    }
}

fn str_to_graphemes(s: &str) -> Vec<Grapheme> {
    let graphemes = s
        .graphemes(true)
        .map(|s| Grapheme {
            string: String::from(s),
            width: GraphemeWidth::from_usize(s.width_cjk()),
        })
        .collect::<Vec<_>>();
    graphemes
}

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
                    result.push_str(&grapheme.string);
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
            result.push_str(&grapheme.string);
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
                result.push_str(&grapheme.string);
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
        let result: String = self.graphemes.iter().map(|g| g.string.clone()).collect();
        write!(formatter, "{result}")
    }
}
