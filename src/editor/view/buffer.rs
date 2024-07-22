use super::line::Line;

#[derive(Default)]
pub struct Buffer {
    pub lines: Vec<Line>,
}

impl Buffer {
    pub fn is_empty(&self) -> bool {
        self.lines.is_empty()
    }
    pub fn load(&mut self, contents: &str) {
        let lines: Vec<_> = contents.lines().map(Line::from_str).collect();
        self.lines = lines;
    }
}
