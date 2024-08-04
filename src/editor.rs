use crossterm::event::{
    read,
    Event::{self, Key},
    KeyCode, KeyEvent, KeyEventKind, KeyModifiers,
};

mod terminal;
use terminal::Terminal;

mod view;
use view::View;

#[derive(Default)]
pub struct Editor {
    should_quit: bool,
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
                    self.view.handle_move(*code)?;
                }
                _ => (),
            }
        }
        Ok(())
    }
    fn refresh_screen(&mut self) -> Result<(), std::io::Error> {
        if self.should_quit {
            Terminal::clear_screen()?;
            print!("Goodbye!\r\n");
        } else {
            self.view.render()?;
            let pos = self.view.get_relative_position();
            Terminal::move_cursor_to(pos)?;
        }
        Ok(())
    }
}
