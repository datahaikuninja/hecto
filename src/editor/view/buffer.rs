use super::line::Line;
use super::Location;

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
    pub fn insert_char(&mut self, c: char, loc: Location) {
        if loc.y >= self.lines.len() {
            // TODO: insert new line at the end of buffer
            return;
        }
        self.lines[loc.y].insert_char(c, loc.x);
    }
}
