use crossterm::style::Color;
use unicode_segmentation::UnicodeSegmentation;

#[derive(Clone, Copy, Debug)]
pub enum Style {
    SearchHit,
    Digit,
}

pub struct DrawingOptions {
    pub foreground_color: Color,
    pub background_color: Color,
}

impl Style {
    pub fn get_drawing_options(&self) -> DrawingOptions {
        match self {
            Self::SearchHit => DrawingOptions {
                foreground_color: Color::White,
                background_color: Color::Rgb {
                    r: 104,
                    g: 83,
                    b: 0,
                },
            },
            Self::Digit => DrawingOptions {
                foreground_color: Color::Rgb {
                    r: 234,
                    g: 156,
                    b: 88,
                },
                background_color: Color::Reset,
            },
        }
    }
}

#[derive(Clone, Debug)]
pub struct Annotation {
    style: Style,
    // byte index
    start_idx: usize,
    end_idx: usize,
}

pub struct Segment {
    pub style: Option<Style>,
    pub string: String,
}

impl Annotation {
    pub fn new(style: Style, start_idx: usize, end_idx: usize) -> Self {
        Self {
            style,
            start_idx,
            end_idx,
        }
    }
}

#[derive(Default, Debug)]
pub struct AnnotatedString {
    string: String,
    annots: Vec<Annotation>,
}

impl AnnotatedString {
    pub fn from_str(s: &str) -> Self {
        Self {
            string: String::from(s),
            annots: vec![],
        }
    }
    pub fn add_annotation(&mut self, annot: Annotation) {
        self.annots.push(annot);
    }
    pub fn get_str(&self) -> &str {
        &self.string
    }
    pub fn get_annotations(&self) -> &Vec<Annotation> {
        &self.annots
    }
    pub fn substr(&self, start: usize, end: usize) -> Self {
        let sub = String::from(&self.string[start..end]);
        let mut annots = vec![];
        for annot in &self.annots {
            let Annotation {
                style,
                start_idx,
                end_idx,
            } = annot;
            if *end_idx <= start || end <= *start_idx {
                // annotation is out of bound
                continue;
            }
            if start <= *start_idx && *end_idx <= end {
                annots.push(annot.clone());
            } else if *start_idx <= start && *end_idx <= end {
                annots.push(Annotation {
                    style: *style,
                    start_idx: start,
                    end_idx: *end_idx,
                });
            } else if start <= *start_idx && end <= *end_idx {
                annots.push(Annotation {
                    style: *style,
                    start_idx: *start_idx,
                    end_idx: end,
                });
            }
        }
        Self {
            string: sub,
            annots,
        }
    }
    pub fn push_annot_str(&mut self, rhs: &AnnotatedString) {
        let orig_len = self.string.len();
        self.string.push_str(&rhs.string);
        for annot in rhs.get_annotations() {
            let Annotation {
                style,
                start_idx,
                end_idx,
            } = annot;
            self.add_annotation(Annotation {
                style: *style,
                start_idx: start_idx + orig_len,
                end_idx: end_idx + orig_len,
            })
        }
    }
    pub fn into_segments(&self) -> Vec<Segment> {
        let mut result = vec![];
        for (idx, s) in self.string.grapheme_indices(true) {
            let mut style = None;
            for annot in &self.annots {
                let Annotation {
                    style: s,
                    start_idx,
                    end_idx,
                } = annot;
                if *start_idx <= idx && idx < *end_idx {
                    style = Some(*s);
                }
            }
            result.push(Segment {
                style,
                string: String::from(s),
            })
        }
        result
    }
}
