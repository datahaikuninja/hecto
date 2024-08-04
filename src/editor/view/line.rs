use unicode_segmentation::UnicodeSegmentation;
use unicode_width::UnicodeWidthStr;

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

struct Grapheme {
    string: String,
    width: GraphemeWidth,
}

pub struct Line {
    graphemes: Vec<Grapheme>,
}

impl Line {
    pub fn from_str(s: &str) -> Self {
        let graphemes = s
            .graphemes(true)
            .map(|s| Grapheme {
                string: String::from(s),
                width: GraphemeWidth::from_usize(s.width_cjk()),
            })
            .collect::<Vec<_>>();
        Self { graphemes }
    }
    pub fn get_visible_graphemes(&self, left: usize, right: usize) -> String {
        let mut result = String::new();
        let mut current_pos = 0;
        for grapheme in &self.graphemes {
            let next_pos = current_pos + grapheme.width.to_usize();
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
        self.graphemes
            .iter()
            .take(graphme_index)
            .map(|g| g.width.to_usize())
            .sum()
    }
    pub fn len(&self) -> usize {
        self.graphemes.len()
    }
}
