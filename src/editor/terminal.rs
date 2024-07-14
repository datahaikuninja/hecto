use crossterm::cursor::MoveTo;
use crossterm::execute;
use crossterm::style::Print;
use crossterm::terminal::{
    disable_raw_mode, enable_raw_mode, Clear, ClearType, EnterAlternateScreen, LeaveAlternateScreen,
};

pub struct Terminal {}

#[derive(Copy, Clone)]
pub struct Size {
    pub height: usize,
    pub width: usize,
}

#[derive(Copy, Clone, Default)]
pub struct Position {
    pub col: usize,
    pub row: usize,
}

impl Terminal {
    pub fn initialize() -> Result<(), std::io::Error> {
        Self::enter_alternate_screen()?;
        enable_raw_mode()?;
        Self::clear_screen()?;
        Ok(())
    }
    pub fn terminate() -> Result<(), std::io::Error> {
        Self::leave_alternate_screen()?;
        disable_raw_mode()?;
        Ok(())
    }
    pub fn clear_screen() -> Result<(), std::io::Error> {
        execute!(std::io::stdout(), Clear(ClearType::All))?;
        Ok(())
    }
    pub fn clear_line() -> Result<(), std::io::Error> {
        execute!(std::io::stdout(), Clear(ClearType::CurrentLine))?;
        Ok(())
    }
    pub fn enter_alternate_screen() -> Result<(), std::io::Error> {
        execute!(std::io::stdout(), EnterAlternateScreen)?;
        Ok(())
    }
    pub fn leave_alternate_screen() -> Result<(), std::io::Error> {
        execute!(std::io::stdout(), LeaveAlternateScreen)?;
        Ok(())
    }
    pub fn move_cursor_to(position: Position) -> Result<(), std::io::Error> {
        execute!(
            std::io::stdout(),
            MoveTo(position.col as u16, position.row as u16)
        )?;
        Ok(())
    }
    pub fn print(s: &str) -> Result<(), std::io::Error> {
        execute!(std::io::stdout(), Print(s))?;
        Ok(())
    }
    pub fn size() -> Result<Size, std::io::Error> {
        let (ncol, nrow) = crossterm::terminal::size()?;
        Ok(Size {
            height: nrow as usize,
            width: ncol as usize,
        })
    }
}
