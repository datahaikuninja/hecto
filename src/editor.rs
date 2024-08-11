use crossterm::event::{read, Event};

mod editor_command;
use editor_command::Command;

mod terminal;
use terminal::Terminal;

mod view;
use view::View;

pub struct Editor {
    should_quit: bool,
    view: View,
}

impl Editor {
    pub fn new() -> Self {
        let current_hook = std::panic::take_hook();
        std::panic::set_hook(Box::new(move |panic_info| {
            let _ = Terminal::terminate(); // explicitly ignore errors in terminate()
            current_hook(panic_info);
        }));
        let view = View::default();
        Self {
            should_quit: false,
            view,
        }
    }
    pub fn load_file(&mut self, filename: &str) {
        // TODO: consider execute read_to_string inside Buffer::load().
        let contents = std::fs::read_to_string(filename).expect("cannot open file");
        self.view.load(&contents);
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
        let command = Command::from_key_event(event);
        match command {
            Command::Quit => {
                self.should_quit = true;
            }
            Command::CursorMove(direction) => {
                self.view.handle_move(direction)?;
            }
            Command::Nop => (),
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
