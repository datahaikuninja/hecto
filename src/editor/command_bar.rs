use super::buffer::Line;
use super::terminal::Position;
use super::Terminal;

pub struct CommandBar {
    prompt: char,
    cmdline: Line,
    needs_redraw: bool,
    pos_y: usize,
}

impl CommandBar {
    pub fn new(pos_y: usize) -> Self {
        Self {
            prompt: ':',
            cmdline: Line::from_str(""),
            needs_redraw: true,
            pos_y,
        }
    }
    pub fn render(&mut self) -> Result<(), std::io::Error> {
        let message = format!("{}{}", self.prompt, self.cmdline);
        Terminal::move_cursor_to(Position {
            row: self.pos_y,
            col: 0,
        })?;
        Terminal::clear_line()?;
        Terminal::print(&message)?;
        Ok(())
    }

    pub fn insert_char(&mut self, c: char) {
        self.cmdline.insert_char(c, self.cmdline.len());
    }

    pub fn handle_backspace(&mut self) {
        if self.cmdline.len() > 0 {
            self.cmdline.delete_grapheme(self.cmdline.len() - 1);
        }
    }

    pub fn clear_cmdline(&mut self) {
        self.cmdline = Line::from_str("");
    }
}
