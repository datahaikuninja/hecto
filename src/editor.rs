use crossterm::event::{read, Event, Event::Key, KeyCode::Char, KeyEvent, KeyModifiers};
use crossterm::execute;
use crossterm::terminal::{disable_raw_mode, enable_raw_mode, Clear, ClearType};

pub struct Editor {
    should_quit: bool,
}

impl Editor {
    pub fn default() -> Self {
        Editor { should_quit: false }
    }
    pub fn run(&mut self) {
        Self::initialize().unwrap();
        let result = self.repl();
        Self::terminate().unwrap();
        result.unwrap();
    }
    fn initialize() -> Result<(), std::io::Error> {
        enable_raw_mode()?;
        Self::clear_screen()
    }
    fn terminate() -> Result<(), std::io::Error> {
        disable_raw_mode()
    }
    fn clear_screen() -> Result<(), std::io::Error> {
        let mut stdout = std::io::stdout();
        execute!(stdout, Clear(ClearType::All))
    }
    fn repl(&mut self) -> Result<(), std::io::Error> {
        loop {
            let event = read()?;
            self.evaluate_evnet(&event);
            self.refresh_screen()?;
            if self.should_quit {
                break;
            }
        }
        Ok(())
    }
    fn evaluate_evnet(&mut self, event: &Event) {
        if let Key(KeyEvent {
            code, modifiers, ..
        }) = event
        {
            println!("Code: {code:?} Modifiers: {modifiers:?} \r");
            if let Char(c) = code {
                if *c == 'q' && *modifiers == KeyModifiers::CONTROL {
                    self.should_quit = true;
                }
            }
        }
    }
    fn refresh_screen(&self) -> Result<(), std::io::Error> {
        if self.should_quit {
            Self::clear_screen()?;
            println!("Goodbye!");
        }
        Ok(())
    }
}
