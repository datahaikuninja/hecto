use crossterm::cursor::MoveTo;
use crossterm::execute;
use crossterm::style::Print;
use crossterm::terminal::{disable_raw_mode, enable_raw_mode, Clear, ClearType};

pub struct Terminal {}

impl Terminal {
    pub fn initialize() -> Result<(), std::io::Error> {
        enable_raw_mode()?;
        Self::clear_screen()?;
        Ok(())
    }
    pub fn terminate() -> Result<(), std::io::Error> {
        disable_raw_mode()?;
        Ok(())
    }
    pub fn clear_screen() -> Result<(), std::io::Error> {
        execute!(std::io::stdout(), Clear(ClearType::All))?;
        Ok(())
    }
    pub fn move_cursor_to(x: u16, y: u16) -> Result<(), std::io::Error> {
        execute!(std::io::stdout(), MoveTo(x, y))?;
        Ok(())
    }
    pub fn print(s: &str) -> Result<(), std::io::Error> {
        execute!(std::io::stdout(), Print(s))?;
        Ok(())
    }
    pub fn size() -> Result<(u16, u16), std::io::Error> {
        crossterm::terminal::size()
    }
}
