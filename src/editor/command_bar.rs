use super::buffer::Line;
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
        Terminal::print(&message)?;
        Ok(())
    }

    pub fn insert_char(&mut self, c: char) {
        self.cmdline.insert_char(c, self.cmdline.len());
    }
}
