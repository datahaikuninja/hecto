use super::terminal::{Position, Size, Terminal};

use super::editor_command::Direction;

mod line;
use line::Grapheme;

mod buffer;
use buffer::Buffer;

mod location;
use location::Location;

struct CursorInfo {
    // grapheme at cursor position
    // maybe `None` if cursor is at empty line
    grapheme: Option<Grapheme>,
    // absolute cursor positions in virtual terminal with infinite size
    row: usize,
    col_start: usize,
    col_end: usize,
}

pub struct View {
    buffer: Buffer,
    needs_redraw: bool,
    location: Location,
    scroll_offset: Position,
}

impl View {
    const NAME: &'static str = env!("CARGO_PKG_NAME");
    const VERSION: &'static str = env!("CARGO_PKG_VERSION");

    pub fn load(&mut self, contents: &str) {
        self.buffer.load(contents);
        self.needs_redraw = true;
    }
    pub fn render(&mut self) -> Result<(), std::io::Error> {
        // TODO: separate implementation of render()
        // according to whether buffer is empty or not.
        if !self.needs_redraw {
            return Ok(());
        }
        let top = self.scroll_offset.row;
        let Size { height, width } = Terminal::size()?;
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
            Self::draw_welcom_message()?;
        }
        self.needs_redraw = false;
        Ok(())
    }
    pub fn handle_move(&mut self, direction: Direction) -> Result<(), std::io::Error> {
        let Location { mut x, mut y } = self.location;
        match direction {
            Direction::Left => {
                x = x.saturating_sub(1);
            }
            Direction::Down => {
                y = y.saturating_add(1);
            }
            Direction::Up => {
                y = y.saturating_sub(1);
            }
            Direction::Right => {
                x = x.saturating_add(1);
            }
        }

        // Ensure self.location points to valid text position.
        let n_line = self.buffer.lines.len();
        y = std::cmp::min(y, n_line.saturating_sub(1));
        let line_length = self.buffer.lines.get(y).map_or(0, |s| s.len());
        x = std::cmp::min(x, line_length.saturating_sub(1));

        self.location = Location { x, y };
        self.update_scroll_offset()?;
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
        let Location { x, y } = self.location;
        let line = self.buffer.lines.get(y);
        let col_start = line.map_or(0, |line| line.calc_width_until_grapheme_index(x));
        let col_end = line.map_or(0, |line| line.calc_width_until_grapheme_index(x + 1));
        CursorInfo {
            grapheme: line.map_or(None, |line| line.get_nth_grapheme(x)),
            row: y,
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
        let Size { width, height } = Terminal::size()?;
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
    fn draw_welcom_message() -> Result<(), std::io::Error> {
        // make message content
        let message = format!("{} editor -- v{}", Self::NAME, Self::VERSION);
        // calculate draw position
        let Size { height, width } = Terminal::size()?;
        let row = height / 3;
        let col = (width - message.len()) / 2;
        // draw messages and column of tildes
        let pos = Position { row, col };
        Terminal::move_cursor_to(pos)?;
        Terminal::print(&message)?;
        Ok(())
    }
}

impl Default for View {
    fn default() -> Self {
        Self {
            buffer: Buffer::default(),
            needs_redraw: true,
            location: Location::default(),
            scroll_offset: Position::default(),
        }
    }
}
