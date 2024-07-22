use unicode_segmentation::UnicodeSegmentation;

pub struct Line {
    content: String,
}

impl Line {
    pub fn from_str(s: &str) -> Self {
        Self {
            content: String::from(s),
        }
    }
    pub fn get_range(&self, left: usize, right: usize) -> String {
        let end = std::cmp::min(right, self.len());
        let display_line = self
            .content
            .graphemes(true)
            .skip(left)
            .take(end.saturating_sub(left))
            .collect::<String>();
        display_line
    }
    pub fn len(&self) -> usize {
        self.content.graphemes(true).count()
    }
}
