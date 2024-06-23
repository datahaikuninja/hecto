use crossterm::event::{read, Event, Event::Key, KeyCode::Char, KeyEvent, KeyModifiers};
use crossterm::execute;
use crossterm::style::Print;

mod terminal;
use terminal::Terminal;

pub struct Editor {
    should_quit: bool,
}

impl Editor {
    pub const fn default() -> Self {
        Editor { should_quit: false }
    }
    pub fn run(&mut self) {
        Terminal::initialize().unwrap();
        let result = self.repl();
        Terminal::terminate().unwrap();
        result.unwrap();
    }
    fn draw_rows() -> Result<(), std::io::Error> {
        let (_, nrow) = Terminal::size()?;
        for i in 0..nrow {
            Terminal::move_cursor_to(0, i)?;
            execute!(std::io::stdout(), Print("~"))?;
        }
        Ok(())
    }
    fn repl(&mut self) -> Result<(), std::io::Error> {
        loop {
            self.refresh_screen()?;
            if self.should_quit {
                break;
            }
            let event = read()?;
            self.evaluate_evnet(&event);
        }
        Ok(())
    }
    fn evaluate_evnet(&mut self, event: &Event) {
        if let Key(KeyEvent {
            code: Char(c),
            modifiers,
            ..
        }) = event
        {
            if *c == 'q' && *modifiers == KeyModifiers::CONTROL {
                self.should_quit = true;
            }
        }
    }
    fn refresh_screen(&self) -> Result<(), std::io::Error> {
        if self.should_quit {
            Terminal::clear_screen()?;
            print!("Goodbye!\r\n");
        } else {
            Self::draw_rows()?;
            Terminal::move_cursor_to(0, 0)?;
        }
        Ok(())
    }
}
