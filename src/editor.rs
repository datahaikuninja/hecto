use crossterm::event::{
    read,
    Event::{self, Key},
    KeyCode, KeyEvent, KeyEventKind, KeyModifiers,
};
use std::cmp::min;

mod terminal;
use terminal::{Position, Size, Terminal};

mod view;
use view::View;

#[derive(Copy, Clone, Default)]
struct Location {
    x: usize,
    y: usize,
}

#[derive(Default)]
pub struct Editor {
    should_quit: bool,
    location: Location,
    view: View,
}

impl Editor {
    pub fn new(filename: &str) -> Self {
        // TODO: consider execute read_to_string inside Buffer::load().
        let contents = std::fs::read_to_string(filename).expect("cannot open file");
        let mut view = View::default();
        view.load(&contents);
        Self {
            should_quit: false,
            location: Location::default(),
            view,
        }
    }
    pub fn run(&mut self) {
        Terminal::initialize().unwrap();
        let result = self.repl();
        Terminal::terminate().unwrap();
        result.unwrap();
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
        let Size { height, width } = Terminal::size()?;
        match key_code {
            KeyCode::Char('h') => {
                x = x.saturating_sub(1);
            }
            KeyCode::Char('j') => {
                y = min(height - 1, y.saturating_add(1));
            }
            KeyCode::Char('k') => {
                y = y.saturating_sub(1);
            }
            KeyCode::Char('l') => {
                x = min(width - 1, x.saturating_add(1));
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
            self.view.render()?;
            let Location { x, y } = self.location;
            let pos = Position { row: y, col: x };
            Terminal::move_cursor_to(pos)?;
        }
        Ok(())
    }
}
