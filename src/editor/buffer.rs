use std::io::Write;

use super::window::Location;

mod line;
use line::Line;

pub mod grapheme;

#[derive(Default)]
pub struct Buffer {
    pub lines: Vec<Line>,
    filename: Option<String>,
}

impl Buffer {
    pub fn is_empty(&self) -> bool {
        self.lines.is_empty()
    }
    pub fn load_file(&mut self, filename: &str) {
        let contents = std::fs::read_to_string(filename).expect("cannot open file");
        let lines: Vec<_> = contents.lines().map(Line::from_str).collect();
        self.lines = lines;
        self.filename = Some(String::from(filename));
    }
    pub fn save(&self) -> Result<(), std::io::Error> {
        if let Some(filename) = &self.filename {
            let mut file = std::fs::File::create(filename)?;
            for line in &self.lines {
                writeln!(file, "{line}")?;
            }
        }
        Ok(())
    }
    pub fn get_line_length(&self, line_index: usize) -> usize {
        self.lines.get(line_index).map_or(0, |line| line.len())
    }
    pub fn insert_char(&mut self, c: char, loc: Location) {
        if loc.y >= self.lines.len() {
            // TODO: insert new line at the end of buffer
            return;
        }
        self.lines[loc.y].insert_char(c, loc.x);
    }
    pub fn delete_grapheme(&mut self, loc: Location) {
        self.lines[loc.y].delete_grapheme(loc.x);
    }
    pub fn join_adjacent_rows(&mut self, idx: usize) {
        let next_line = self.lines.remove(idx + 1);
        let current_line = &mut self.lines[idx];
        current_line.push_line(&next_line);
    }
    pub fn insert_newline(&mut self, loc: Location) {
        let remainder = self.lines[loc.y].split_off(loc.x);
        self.lines.insert(loc.y + 1, remainder);
    }
}
