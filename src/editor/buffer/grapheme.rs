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
    pub fn get_width_at_current_pos(&self, current_pos: usize) -> usize {
        if self.is_tab() {
            calc_tab_width(current_pos)
        } else {
            self.width.to_usize()
        }
    }
}

impl std::fmt::Display for Grapheme {
    fn fmt(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(formatter, "{}", self.string)
    }
}

pub fn str_to_graphemes(s: &str) -> (Vec<Grapheme>, Vec<usize>) {
    s.grapheme_indices(true)
        .map(|(grapheme_idx, s)| {
            (
                Grapheme {
                    string: String::from(s),
                    width: GraphemeWidth::from_usize(s.width_cjk()),
                },
                grapheme_idx,
            )
        })
        .unzip()
}
