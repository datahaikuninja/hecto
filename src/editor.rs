use crossterm::event::{read, Event};

mod editor_command;
use editor_command::{Direction, EditorMode, InsertModeCommand, NormalModeCommand};

mod terminal;
use terminal::Terminal;

mod window;
use window::Window;

mod buffer;

pub struct Editor {
    should_quit: bool,
    mode: EditorMode,
    window: Window,
}

impl Editor {
    pub fn new() -> Self {
        let current_hook = std::panic::take_hook();
        std::panic::set_hook(Box::new(move |panic_info| {
            let _ = Terminal::terminate(); // explicitly ignore errors in terminate()
            current_hook(panic_info);
        }));
        let view = Window::default();
        Self {
            should_quit: false,
            mode: EditorMode::NormalMode,
            window: view,
        }
    }
    pub fn load_file(&mut self, filename: &str) {
        self.window.load_file(&filename);
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
        match self.mode {
            EditorMode::NormalMode => self.evaluate_evnet_in_normal_mode(event)?,
            EditorMode::InsertMode => self.evaluate_evnet_in_insert_mode(event)?,
        }
        Ok(())
    }
    fn evaluate_evnet_in_normal_mode(&mut self, event: &Event) -> Result<(), std::io::Error> {
        let command = NormalModeCommand::from_key_event(event);
        match command {
            NormalModeCommand::Quit => {
                self.should_quit = true;
            }
            NormalModeCommand::Save => {
                self.window.save_buffer()?;
            }
            NormalModeCommand::CursorMove(direction) => {
                self.window.handle_move(direction, false)?;
            }
            NormalModeCommand::EnterInsertMode => {
                self.mode = EditorMode::InsertMode;
            }
            NormalModeCommand::EnterInsertModeAppend => {
                self.window.handle_move(Direction::Right, true)?;
                self.mode = EditorMode::InsertMode;
            }
            NormalModeCommand::Nop => (),
        }
        Ok(())
    }
    fn evaluate_evnet_in_insert_mode(&mut self, event: &Event) -> Result<(), std::io::Error> {
        let command = InsertModeCommand::from_key_event(event);
        match command {
            InsertModeCommand::LeaveInsertMode => {
                self.mode = EditorMode::NormalMode;
                self.window.normalize_cursor_position(false)?;
            }
            InsertModeCommand::Insert(c) => {
                self.window.insert_char(c)?;
            }
            InsertModeCommand::Backspace => {
                self.window.handle_backspace()?;
            }
            InsertModeCommand::InsertNewLine => {
                self.window.insert_newline()?;
            }
            InsertModeCommand::Nop => (),
        }
        Ok(())
    }
    fn refresh_screen(&mut self) -> Result<(), std::io::Error> {
        if self.should_quit {
            Terminal::clear_screen()?;
            print!("Goodbye!\r\n");
        } else {
            self.window.render()?;
            let pos = self.window.get_relative_position();
            Terminal::move_cursor_to(pos)?;
        }
        Ok(())
    }
}
