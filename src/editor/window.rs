use super::terminal::{Position, Size, Terminal};

use super::editor_command::Direction;

use super::buffer::grapheme::Grapheme;
use super::buffer::Buffer;

use super::DocumentStatus;

#[derive(Copy, Clone, Default)]
pub struct TextLocation {
    pub grapheme_idx: usize,
    pub line_idx: usize,
}

struct CursorInfo {
    // grapheme at cursor position
    // maybe `None` if cursor is at empty line
    grapheme: Option<Grapheme>,
    // absolute cursor positions in virtual terminal with infinite size
    row: usize,
    col_start: usize,
    col_end: usize,
}

pub struct Window {
    buffer: Buffer,
    needs_redraw: bool,
    cursor_location: TextLocation,
    scroll_offset: Position,
    size: Size,
}

impl Window {
    const NAME: &'static str = env!("CARGO_PKG_NAME");
    const VERSION: &'static str = env!("CARGO_PKG_VERSION");

    pub fn new(margin: usize) -> Self {
        let size = Terminal::size().expect("Failed to get terminal size.");
        Self {
            buffer: Buffer::default(),
            needs_redraw: true,
            cursor_location: TextLocation::default(),
            scroll_offset: Position::default(),
            size: Size {
                width: size.width,
                height: size.height - margin,
            },
        }
    }
    pub fn load_file(&mut self, filename: &str) {
        self.buffer.load_file(filename);
        self.needs_redraw = true;
    }
    pub fn get_status(&self) -> DocumentStatus {
        DocumentStatus {
            total_lines: self.buffer.get_n_lines(),
            current_line_index: self.cursor_location.line_idx,
            is_modified: self.buffer.modified,
            file_name: self.buffer.get_filename(),
        }
    }
    pub fn render(&mut self) -> Result<(), std::io::Error> {
        // TODO: separate implementation of render()
        // according to whether buffer is empty or not.
        if !self.needs_redraw {
            return Ok(());
        }
        let top = self.scroll_offset.row;
        let Size { height, width } = self.size;
        for i in 0..height {
            if let Some(line) = self.buffer.lines.get(i + top) {
                let left = self.scroll_offset.col;
                let right = left + width;
                let display_line = line.get_visible_graphemes(left, right);
                self.render_line(i, &display_line)?;
            } else {
                self.render_line(i, "~")?;
            }
        }
        if self.buffer.is_empty() {
            self.draw_welcom_message()?;
        }
        self.needs_redraw = false;
        Ok(())
    }
    pub fn save_buffer(&mut self) -> Result<(), std::io::Error> {
        self.buffer.save()?;
        Ok(())
    }
    pub fn save_buffer_with_filename(&mut self, filename: &str) -> Result<(), std::io::Error> {
        self.buffer.save_as_filename(filename)?;
        Ok(())
    }
    pub fn handle_move(
        &mut self,
        direction: Direction,
        allow_past_end: bool,
    ) -> Result<(), std::io::Error> {
        let TextLocation {
            mut grapheme_idx,
            mut line_idx,
        } = self.cursor_location;
        match direction {
            Direction::Left => {
                grapheme_idx = grapheme_idx.saturating_sub(1);
            }
            Direction::Down => {
                line_idx = line_idx.saturating_add(1);
            }
            Direction::Up => {
                line_idx = line_idx.saturating_sub(1);
            }
            Direction::Right => {
                grapheme_idx = grapheme_idx.saturating_add(1);
            }
        }

        self.cursor_location = TextLocation {
            grapheme_idx,
            line_idx,
        };
        self.normalize_cursor_position(allow_past_end)?;
        self.update_scroll_offset()?;
        Ok(())
    }
    pub fn normalize_cursor_position(
        &mut self,
        allow_past_end: bool,
    ) -> Result<(), std::io::Error> {
        // Ensure self.location points to valid text position.
        let TextLocation {
            mut grapheme_idx,
            mut line_idx,
        } = self.cursor_location;
        let n_line = self.buffer.lines.len();
        line_idx = std::cmp::min(line_idx, n_line.saturating_sub(1));

        let line_length = self.buffer.lines.get(line_idx).map_or(0, |s| s.len());
        let idx_lim = if allow_past_end {
            line_length
        } else {
            line_length.saturating_sub(1)
        };
        grapheme_idx = std::cmp::min(grapheme_idx, idx_lim);

        self.cursor_location = TextLocation {
            grapheme_idx,
            line_idx,
        };
        self.update_scroll_offset()?;
        Ok(())
    }
    pub fn insert_char(&mut self, c: char) -> Result<(), std::io::Error> {
        let orig_len = self.buffer.get_line_length(self.cursor_location.line_idx);
        self.buffer.insert_char(c, self.cursor_location);
        let new_len = self.buffer.get_line_length(self.cursor_location.line_idx);
        if new_len > orig_len {
            self.cursor_location.grapheme_idx += 1;
        }
        self.update_scroll_offset()?;
        self.needs_redraw = true;
        Ok(())
    }
    pub fn handle_backspace(&mut self) -> Result<(), std::io::Error> {
        if self.cursor_location.grapheme_idx > 0 {
            self.cursor_location.grapheme_idx -= 1;
            self.buffer.delete_grapheme(self.cursor_location);
            self.update_scroll_offset()?;
            self.needs_redraw = true;
        } else if self.cursor_location.grapheme_idx == 0 {
            if self.cursor_location.line_idx == 0 {
                return Ok(());
            }
            let orig_len = self
                .buffer
                .get_line_length(self.cursor_location.line_idx - 1);
            self.buffer
                .join_adjacent_rows(self.cursor_location.line_idx - 1);
            self.cursor_location = TextLocation {
                grapheme_idx: orig_len,
                line_idx: self.cursor_location.line_idx - 1,
            };
            self.update_scroll_offset()?;
            self.needs_redraw = true;
        }
        Ok(())
    }
    pub fn insert_newline(&mut self) -> Result<(), std::io::Error> {
        self.buffer.insert_newline(self.cursor_location);
        self.cursor_location = TextLocation {
            grapheme_idx: 0,
            line_idx: self.cursor_location.line_idx + 1,
        };
        self.update_scroll_offset()?;
        self.needs_redraw = true;
        Ok(())
    }
    pub fn get_relative_position(&self) -> Position {
        let Position { row, col } = self.get_absolute_position();
        Position {
            col: col - self.scroll_offset.col,
            row: row - self.scroll_offset.row,
        }
    }
    pub fn get_absolute_position(&self) -> Position {
        let cursor_info = self.get_cursor_info();
        if cursor_info
            .grapheme
            .map_or(false, |grapheme| grapheme.is_tab())
        {
            // renders cursor at right end of TAB character
            Position {
                row: cursor_info.row,
                col: cursor_info.col_end - 1,
            }
        } else {
            Position {
                row: cursor_info.row,
                col: cursor_info.col_start,
            }
        }
    }
    fn get_cursor_info(&self) -> CursorInfo {
        let TextLocation {
            grapheme_idx,
            line_idx,
        } = self.cursor_location;
        let line = self.buffer.lines.get(line_idx);
        let col_start = line.map_or(0, |line| line.calc_width_until_grapheme_index(grapheme_idx));
        let col_end = line.map_or(0, |line| {
            line.calc_width_until_grapheme_index(grapheme_idx + 1)
        });
        CursorInfo {
            grapheme: line.map_or(None, |line| line.get_nth_grapheme(grapheme_idx)),
            row: line_idx,
            col_start,
            col_end,
        }
    }
    fn render_line(&self, row: usize, text: &str) -> Result<(), std::io::Error> {
        let pos = Position { row, col: 0 };
        Terminal::move_cursor_to(pos)?;
        Terminal::clear_line()?;
        Terminal::print(text)?;
        Ok(())
    }
    fn update_scroll_offset(&mut self) -> Result<(), std::io::Error> {
        let Size { width, height } = self.size;
        let mut offset_changed = false;
        let CursorInfo {
            row,
            col_start: left,
            col_end: right,
            ..
        } = self.get_cursor_info();

        // Scroll vertically
        if row < self.scroll_offset.row {
            self.scroll_offset.row = row;
            offset_changed = true;
        } else if row >= self.scroll_offset.row + height {
            self.scroll_offset.row = row - height + 1;
            offset_changed = true;
        }

        //Scroll horizontally
        if left < self.scroll_offset.col {
            self.scroll_offset.col = left;
            offset_changed = true;
        } else if right > self.scroll_offset.col + width {
            self.scroll_offset.col = right - width;
            offset_changed = true;
        }
        self.needs_redraw = self.needs_redraw || offset_changed;
        Ok(())
    }
    fn draw_welcom_message(&self) -> Result<(), std::io::Error> {
        // make message content
        let message = format!("{} editor -- v{}", Self::NAME, Self::VERSION);
        // calculate draw position
        let Size { height, width } = self.size;
        let row = height / 3;
        let col = (width - message.len()) / 2;
        // draw messages and column of tildes
        let pos = Position { row, col };
        Terminal::move_cursor_to(pos)?;
        Terminal::print(&message)?;
        Ok(())
    }
}
