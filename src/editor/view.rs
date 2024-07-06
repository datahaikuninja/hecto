use super::terminal::{Position, Size, Terminal};

pub struct View;

impl View {
    const NAME: &'static str = env!("CARGO_PKG_NAME");
    const VERSION: &'static str = env!("CARGO_PKG_VERSION");

    pub fn render() -> Result<(), std::io::Error> {
        let Size { height, .. } = Terminal::size()?;
        for i in 0..height {
            let pos = Position { row: i, col: 0 };
            Terminal::move_cursor_to(pos)?;
            Terminal::print("~")?;
        }
        Self::draw_welcom_message()?;
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
