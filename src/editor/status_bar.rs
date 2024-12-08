use super::terminal::Position;
use super::DocumentStatus;
use super::Terminal;

pub struct StatusBar {
    current_status: DocumentStatus,
    needs_redraw: bool,
    width: usize,
    position_y: usize,
}

impl StatusBar {
    pub fn new(position_y: usize) -> Self {
        let size = Terminal::size().expect("Failed to get terminal size");
        StatusBar {
            current_status: DocumentStatus::default(),
            needs_redraw: true,
            width: size.width,
            position_y,
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
        let buffer_name = self
            .current_status
            .file_name
            .clone()
            .unwrap_or(String::from("[No Name]"));
        let modified_status_str = if self.current_status.is_modified {
            "[+]"
        } else {
            ""
        };
        let left_status = format!("{} {}", buffer_name, modified_status_str);
        let right_status = format!(
            "[{}] {}/{}",
            self.current_status.file_type.to_string(),
            self.current_status.current_line_index + 1, // 0-idx to 1-idx
            usize::max(1, self.current_status.total_lines), // If the buffer is empty, it is treated as a single line.
        );
        let padding_len = self
            .width
            .saturating_sub(left_status.len())
            .saturating_sub(right_status.len());
        let mut status_str = left_status + &" ".repeat(padding_len) + &right_status;
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
