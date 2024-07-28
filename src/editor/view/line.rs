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
    pub fn get_range(&self, left: usize, right: usize) -> String {
        let end = std::cmp::min(right, self.len());
        let display_line = self
            .graphemes
            .iter()
            .skip(left)
            .take(end.saturating_sub(left))
            .map(|g| g.string.clone())
            .collect::<String>();
        display_line
    }
    pub fn len(&self) -> usize {
        self.graphemes.len()
    }
}
