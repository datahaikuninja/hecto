use crossterm::event::{
    read,
    Event::{self, Key},
    KeyCode, KeyEvent, KeyEventKind, KeyModifiers,
};
use std::cmp::min;

mod terminal;
use terminal::Terminal;

#[derive(Copy, Clone, Default)]
struct Location {
    x: usize,
    y: usize,
}

#[derive(Default)]
pub struct Editor {
    should_quit: bool,
    location: Location,
}

impl Editor {
    const NAME: &'static str = env!("CARGO_PKG_NAME");
    const VERSION: &'static str = env!("CARGO_PKG_VERSION");

    pub fn run(&mut self) {
        Terminal::initialize().unwrap();
        let result = self.repl();
        Terminal::terminate().unwrap();
        result.unwrap();
    }
    fn draw_tildes() -> Result<(), std::io::Error> {
        let (_, nrow) = Terminal::size()?;
        for i in 0..nrow {
            Terminal::move_cursor_to(0, i)?;
            Terminal::print("~")?;
        }
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
        Self::draw_tildes()?;
        Terminal::move_cursor_to(col, row)?;
        Terminal::print(&message)?;
        Ok(())
    }
    fn repl(&mut self) -> Result<(), std::io::Error> {
        loop {
            self.refresh_screen()?;
            if self.should_quit {
                break;
            }
            let event = read()?;
            self.evaluate_evnet(&event)?;
        }
        Ok(())
    }
    fn move_point(&mut self, key_code: KeyCode) -> Result<(), std::io::Error> {
        let Location { mut x, mut y } = self.location;
        let (ncol, nrow) = Terminal::size()?;
        match key_code {
            KeyCode::Char('h') => {
                x = x.saturating_sub(1);
            }
            KeyCode::Char('j') => {
                y = min(nrow as usize - 1, y.saturating_add(1));
            }
            KeyCode::Char('k') => {
                y = y.saturating_sub(1);
            }
            KeyCode::Char('l') => {
                x = min(ncol as usize - 1, x.saturating_add(1));
            }
            _ => (),
        }
        self.location = Location { x, y };
        Ok(())
    }
    fn evaluate_evnet(&mut self, event: &Event) -> Result<(), std::io::Error> {
        if let Key(KeyEvent {
            code,
            modifiers,
            kind: KeyEventKind::Press,
            ..
        }) = event
        {
            match code {
                KeyCode::Char('q') if *modifiers == KeyModifiers::CONTROL => {
                    self.should_quit = true;
                }
                KeyCode::Char('h')
                | KeyCode::Char('j')
                | KeyCode::Char('k')
                | KeyCode::Char('l') => {
                    self.move_point(*code)?;
                }
                _ => (),
            }
        }
        Ok(())
    }
    fn refresh_screen(&self) -> Result<(), std::io::Error> {
        if self.should_quit {
            Terminal::clear_screen()?;
            print!("Goodbye!\r\n");
        } else {
            Self::draw_welcom_message()?;
            let Location { x, y } = self.location;
            Terminal::move_cursor_to(x as u16, y as u16)?;
        }
        Ok(())
    }
}
