use super::terminal::Position;
use super::DocumentStatus;
use super::Terminal;

pub struct StatusBar {
    current_status: DocumentStatus,
    needs_redraw: bool,
    margin_bottom: usize,
    width: usize,
    position_y: usize,
}

impl StatusBar {
    pub fn new(margin_bottom: usize) -> Self {
        let size = Terminal::size().expect("Failed to get terminal size");
        StatusBar {
            current_status: DocumentStatus::default(),
            needs_redraw: true,
            margin_bottom,
            width: size.width,
            position_y: size.height - margin_bottom - 1,
        }
    }
    pub fn update_status(&mut self, new_stat: DocumentStatus) {
        if new_stat != self.current_status {
            self.current_status = new_stat;
            self.needs_redraw = true;
        }
    }
    pub fn render(&mut self) -> Result<(), std::io::Error> {
        if !self.needs_redraw {
            return Ok(());
        }
        let mut status_str = format!("{:?}", self.current_status);
        status_str.truncate(self.width);
        Terminal::move_cursor_to(Position {
            col: 0,
            row: self.position_y,
        })?;
        Terminal::print(&status_str)?;
        self.needs_redraw = false;
        Ok(())
    }
}
