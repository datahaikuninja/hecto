use std::io::Write;

use super::window::TextLocation;

use super::highlighter::HighlighterBundler;

mod line;
pub use line::{Line, LineView};

pub mod grapheme;

use crate::editor::filetype::FileType;

#[derive(Default)]
pub struct Buffer {
    pub lines: Vec<Line>,
    filename: Option<String>,
    pub modified: bool,
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
    pub fn save(&mut self) -> Result<(), std::io::Error> {
        if let Some(filename) = &self.filename.clone() {
            self.save_as_filename(filename)?;
        }
        Ok(())
    }
    pub fn save_as_filename(&mut self, filename: &str) -> Result<(), std::io::Error> {
        let mut file = std::fs::File::create(filename)?;
        for line in &self.lines {
            writeln!(file, "{line}")?;
        }
        self.filename = Some(filename.to_string());
        self.modified = false;
        Ok(())
    }
    pub fn get_filetype(&self) -> FileType {
        self.filename
            .as_deref()
            .map_or(FileType::Text, |s| FileType::from_filename(s))
    }
    pub fn get_line_length(&self, line_index: usize) -> usize {
        self.lines.get(line_index).map_or(0, |line| line.len())
    }
    pub fn get_n_lines(&self) -> usize {
        self.lines.len()
    }
    pub fn get_filename(&self) -> Option<String> {
        self.filename.clone()
    }
    fn init_if_empty(&mut self) {
        if self.is_empty() {
            self.lines.push(Line::default());
        }
    }
    pub fn insert_char(&mut self, c: char, loc: TextLocation) {
        self.init_if_empty();
        if loc.line_idx >= self.lines.len() {
            // TODO: insert new line at the end of buffer
            return;
        }
        self.lines[loc.line_idx].insert_char(c, loc.grapheme_idx);
        self.modified = true;
    }
    pub fn delete_grapheme(&mut self, loc: TextLocation) {
        self.lines[loc.line_idx].delete_grapheme(loc.grapheme_idx);
        self.modified = true;
    }
    pub fn join_adjacent_rows(&mut self, idx: usize) {
        let next_line = self.lines.remove(idx + 1);
        let current_line = &mut self.lines[idx];
        current_line.push_line(&next_line);
        self.modified = true;
    }
    pub fn begin_newline_above(&mut self, loc: TextLocation) {
        self.init_if_empty();
        self.lines.insert(loc.line_idx, Line::default());
        self.modified = true;
    }
    pub fn begin_newline_below(&mut self, loc: TextLocation) {
        self.init_if_empty();
        self.lines.insert(loc.line_idx + 1, Line::default());
        self.modified = true;
    }
    pub fn insert_newline(&mut self, loc: TextLocation) {
        self.init_if_empty();
        let remainder = self.lines[loc.line_idx].split_off(loc.grapheme_idx);
        self.lines.insert(loc.line_idx + 1, remainder);
        self.modified = true;
    }
    pub fn search(&self, pattern: &str) -> Vec<TextLocation> {
        // return first match in the entire buffer
        let mut result_list = vec![];
        for (line_idx, line) in self.lines.iter().enumerate() {
            let mut start_index = 0;
            while let Some(grapheme_idx) = line.search(pattern, start_index) {
                result_list.push(TextLocation {
                    grapheme_idx,
                    line_idx,
                });
                start_index = grapheme_idx + 1;
            }
        }
        result_list
    }
    pub fn highlight(&self, highlighter: &mut HighlighterBundler) {
        for line in self.lines.iter() {
            highlighter.highlight_line(line);
        }
    }
}
