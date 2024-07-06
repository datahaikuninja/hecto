use super::terminal::Terminal;

pub struct View;

impl View {
    const NAME: &'static str = env!("CARGO_PKG_NAME");
    const VERSION: &'static str = env!("CARGO_PKG_VERSION");

    pub fn render() -> Result<(), std::io::Error> {
        let (_, nrow) = Terminal::size()?;
        for i in 0..nrow {
            Terminal::move_cursor_to(0, i)?;
            Terminal::print("~")?;
        }
        Self::draw_welcom_message()?;
        Ok(())
    }
    fn draw_welcom_message() -> Result<(), std::io::Error> {
        // make message content
        let message = format!("{} editor -- v{}", Self::NAME, Self::VERSION);
        // calculate draw position
        let (ncol, nrow) = Terminal::size()?;
        let row = nrow / 3;
        let col = (ncol - message.len() as u16) / 2;
        // draw messages and column of tildes
        Terminal::move_cursor_to(col, row)?;
        Terminal::print(&message)?;
        Ok(())
    }
}
