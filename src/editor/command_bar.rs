use super::buffer::Line;
use super::editor_command::CmdlineSubmode;
use super::terminal::Position;
use super::Terminal;

pub struct CommandBar {
    prompt: String,
    cmdline: Line,
    needs_redraw: bool,
    pos_y: usize,
}

impl CommandBar {
    pub fn new(pos_y: usize) -> Self {
        Self {
            prompt: String::new(),
            cmdline: Line::from_str(""),
            needs_redraw: true,
            pos_y,
        }
    }
    pub fn render(&mut self) -> Result<(), std::io::Error> {
        if !self.needs_redraw {
            return Ok(());
        }
        let message = format!("{}{}", self.prompt, self.cmdline);
        Terminal::move_cursor_to(Position {
            row: self.pos_y,
            col: 0,
        })?;
        Terminal::clear_line()?;
        Terminal::print(&message)?;
        self.needs_redraw = false;
        Ok(())
    }

    pub fn insert_char(&mut self, c: char) {
        self.needs_redraw = true;
        self.cmdline.insert_char(c, self.cmdline.len());
    }

    pub fn handle_backspace(&mut self) {
        if self.cmdline.len() > 0 {
            self.needs_redraw = true;
            self.cmdline.delete_grapheme(self.cmdline.len() - 1);
        }
    }
    pub fn set_cmdline_prompt(&mut self, submode: CmdlineSubmode) {
        let prompt_str = match submode {
            CmdlineSubmode::Cmdline => ":",
            CmdlineSubmode::Search => "/",
        };
        self.needs_redraw = true;
        self.prompt = String::from(prompt_str);
    }
    pub fn clear_cmdline(&mut self) {
        self.needs_redraw = true;
        self.prompt = String::new();
        self.cmdline = Line::from_str("");
    }
    pub fn get_raw_cmdline(&self) -> String {
        self.cmdline.to_string()
    }
    pub fn get_current_cmdline(&self) -> Vec<String> {
        self.cmdline
            .to_string()
            .split_whitespace()
            .map(String::from)
            .collect()
    }
    pub fn set_error_message(&mut self, msg: &str) {
        self.needs_redraw = true;
        self.prompt = String::new();
        self.cmdline = Line::from_str(msg);
    }
}
