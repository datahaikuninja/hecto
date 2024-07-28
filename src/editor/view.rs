use super::terminal::{Position, Size, Terminal};

use crossterm::event::KeyCode;

mod line;

mod buffer;
use buffer::Buffer;

mod location;
use location::Location;

pub struct View {
    buffer: Buffer,
    needs_redraw: bool,
    location: Location,
    scroll_offset: Location,
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
        let top = self.scroll_offset.y;
        let Size { height, width } = Terminal::size()?;
        for i in 0..height {
            if let Some(line) = self.buffer.lines.get(i + top) {
                let left = self.scroll_offset.x;
                let right = left + width;
                let display_line = line.get_range(left, right);
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
    pub fn handle_move(&mut self, key_code: KeyCode) -> Result<(), std::io::Error> {
        let Location { mut x, mut y } = self.location;
        match key_code {
            KeyCode::Char('h') => {
                x = x.saturating_sub(1);
            }
            KeyCode::Char('j') => {
                y = y.saturating_add(1);
            }
            KeyCode::Char('k') => {
                y = y.saturating_sub(1);
            }
            KeyCode::Char('l') => {
                x = x.saturating_add(1);
            }
            _ => (),
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
    pub fn get_position(&self) -> Position {
        let Location { x, y: ypos } = self.location;
        let xpos = self
            .buffer
            .lines
            .get(ypos)
            .map_or(0, |line| line.calc_width_until_grapheme_index(x));
        Position {
            row: ypos - self.scroll_offset.y,
            col: xpos - self.scroll_offset.x,
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
        let Location { x, y } = self.location;
        let Size { width, height } = Terminal::size()?;
        let mut offset_changed = false;

        // Scroll vertically
        if y < self.scroll_offset.y {
            self.scroll_offset.y = y;
            offset_changed = true;
        } else if y >= self.scroll_offset.y.saturating_add(height) {
            self.scroll_offset.y = y.saturating_sub(height).saturating_add(1);
            offset_changed = true;
        }

        //Scroll horizontally
        if x < self.scroll_offset.x {
            self.scroll_offset.x = x;
            offset_changed = true;
        } else if x >= self.scroll_offset.x.saturating_add(width) {
            self.scroll_offset.x = x.saturating_sub(width).saturating_add(1);
            offset_changed = true;
        }
        self.needs_redraw = offset_changed;
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
            scroll_offset: Location::default(),
        }
    }
}
