use crossterm::event::{read, Event};

mod editor_command;
use editor_command::{
    CmdlineModeCommand, Direction, EditorMode, InsertModeCommand, NormalModeCommand,
};

mod terminal;
use terminal::{Size, Terminal};

mod status_bar;
use status_bar::StatusBar;

mod window;
use window::Window;

mod buffer;

mod command_bar;
use command_bar::CommandBar;

mod cmdline_commands;
use cmdline_commands::CmdlineCommands;

#[derive(Default, Eq, PartialEq, Debug)]
pub struct DocumentStatus {
    total_lines: usize,
    current_line_index: usize,
    is_modified: bool,
    file_name: Option<String>,
}

pub struct Editor {
    should_quit: bool,
    mode: EditorMode,
    window: Window,
    status_bar: StatusBar,
    command_bar: CommandBar,
}

impl Editor {
    pub fn new() -> Self {
        let current_hook = std::panic::take_hook();
        std::panic::set_hook(Box::new(move |panic_info| {
            let _ = Terminal::terminate(); // explicitly ignore errors in terminate()
            current_hook(panic_info);
        }));
        let status_bar_height = 1;
        let message_bar_height = 1;
        let view = Window::new(status_bar_height + message_bar_height);
        let Size { height, .. } = Terminal::size().expect("Coud not get terminal size!");
        Self {
            should_quit: false,
            mode: EditorMode::NormalMode,
            window: view,
            status_bar: StatusBar::new(message_bar_height),
            command_bar: CommandBar::new(height - 1),
        }
    }
    pub fn load_file(&mut self, filename: &str) {
        self.window.load_file(&filename);
        let status = self.window.get_status();
        self.status_bar.update_status(status);
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
            let status = self.window.get_status();
            self.status_bar.update_status(status);
        }
        Ok(())
    }
    fn evaluate_evnet(&mut self, event: &Event) -> Result<(), std::io::Error> {
        match self.mode {
            EditorMode::NormalMode => self.evaluate_evnet_in_normal_mode(event)?,
            EditorMode::InsertMode => self.evaluate_evnet_in_insert_mode(event)?,
            EditorMode::CmdlineMode => self.evalueate_event_in_cmdline_mode(event)?,
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
            NormalModeCommand::EnterCmdlineMode => {
                self.mode = EditorMode::CmdlineMode;
                self.command_bar.clear_cmdline();
                Terminal::print_log("enter command line mode")?;
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
    fn evalueate_event_in_cmdline_mode(&mut self, event: &Event) -> Result<(), std::io::Error> {
        let command = CmdlineModeCommand::from_key_event(event);
        match command {
            CmdlineModeCommand::LeaveCmdlineMode => {
                self.mode = EditorMode::NormalMode;
                self.command_bar.clear_cmdline();
                Terminal::print_log("leave command line mode")?;
            }
            CmdlineModeCommand::Execute => {
                let raw_cmdline = self.command_bar.get_current_cmdline();
                let cmd = CmdlineCommands::from_str(&raw_cmdline);
                if let Some(cmd) = cmd {
                    self.execute_cmdline_command(cmd)?;
                    self.command_bar.clear_cmdline();
                } else {
                    Terminal::print_log(&format!("No such command: {}", raw_cmdline))?;
                }
                self.mode = EditorMode::NormalMode;
            }
            CmdlineModeCommand::Insert(c) => {
                self.command_bar.insert_char(c);
                Terminal::print_log("inser char in command line mode")?;
            }
            CmdlineModeCommand::Backspace => {
                self.command_bar.handle_backspace();
                Terminal::print_log("Backspace in command line mode")?;
            }
            CmdlineModeCommand::Nop => (),
        }
        Ok(())
    }
    fn execute_cmdline_command(&mut self, cmd: CmdlineCommands) -> Result<(), std::io::Error> {
        match cmd {
            CmdlineCommands::Quit => {
                self.should_quit = true;
            }
            CmdlineCommands::Write => {
                self.window.save_buffer()?;
            }
        }
        Ok(())
    }
    fn refresh_screen(&mut self) -> Result<(), std::io::Error> {
        if self.should_quit {
            Terminal::clear_screen()?;
            print!("Goodbye!\r\n");
        } else {
            self.window.render()?;
            self.status_bar.render()?;
            self.command_bar.render()?;
            let pos = self.window.get_relative_position();
            Terminal::move_cursor_to(pos)?;
        }
        Ok(())
    }
}
