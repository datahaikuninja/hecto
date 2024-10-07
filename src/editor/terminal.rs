use crossterm::cursor::MoveTo;
use crossterm::execute;
use crossterm::style::{Print, ResetColor, SetBackgroundColor, SetForegroundColor};
use crossterm::terminal::{
    disable_raw_mode, enable_raw_mode, Clear, ClearType, EnterAlternateScreen, LeaveAlternateScreen,
};

use super::annotated_string::{AnnotatedString, DrawingOptions};

pub struct Terminal {}

#[derive(Copy, Clone)]
pub struct Size {
    pub height: usize,
    pub width: usize,
}

#[derive(Copy, Clone, Default, Debug)]
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
    pub fn print_annotated_str(s: &AnnotatedString) -> Result<(), std::io::Error> {
        let segments = s.into_segments();
        for seg in &segments {
            if let Some(style) = &seg.style {
                let DrawingOptions {
                    foreground_color,
                    background_color,
                } = style.get_drawing_options();
                execute!(
                    std::io::stdout(),
                    SetForegroundColor(foreground_color),
                    SetBackgroundColor(background_color)
                )?;
            }
            execute!(std::io::stdout(), Print(&seg.string))?;
            if seg.style.is_some() {
                execute!(std::io::stdout(), ResetColor)?;
            }
        }
        Ok(())
    }
    pub fn print_log(s: &str) -> Result<(), std::io::Error> {
        // Leave alternate screen temporarily, and send log
        // message to normal screen.
        execute!(
            std::io::stdout(),
            // Temporarily leave alternate screen.
            Print("\x1b[?1049l"), // Same as `LeaveAlternateScreen`
            // Write log message.
            Print(s),
            Print("\r\n"),
            // Re-enter to alternate screen without clearing screen buffer.
            // `EnterAlternateScreen` (CSI ?1049h) does not work because it will
            // clear the screen buffer, when entering to alternate screen.
            // see xterm manual: https://invisible-island.net/xterm/ctlseqs/ctlseqs.html#h3-Functions-using-CSI-_-ordered-by-the-final-character_s_
            Print("\x1b[?1048h"), // Save cursor (DECSC)
            Print("\x1b[?1047h"), // Enter alternate screen (without clearing buffer)
        )?;
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
