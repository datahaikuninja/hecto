use crossterm::event::{read, Event};

mod editor_command;
use editor_command::{
    CmdlineModeCommand, CmdlineSubmode, Direction, EditorMode, InsertModeCommand, NormalModeCommand,
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

mod annotated_string;

mod highlighter;

#[derive(Default, Eq, PartialEq, Debug)]
pub struct DocumentStatus {
    total_lines: usize,
    current_line_index: usize,
    is_modified: bool,
    file_name: Option<String>,
}

#[derive(Debug)]
pub enum SearchDirection {
    Forward,
    Backward,
}

pub struct RenderContext {
    pub search_pattern: Option<String>,
}

pub struct Editor {
    should_quit: bool,
    mode: EditorMode,
    window: Window,
    status_bar: StatusBar,
    command_bar: CommandBar,
    last_search_pattern: Option<String>,
}

impl Editor {
    pub fn new() -> Self {
        let current_hook = std::panic::take_hook();
        std::panic::set_hook(Box::new(move |panic_info| {
            let _ = Terminal::terminate(); // explicitly ignore errors in terminate()
            current_hook(panic_info);
        }));
        let Size { height, .. } = Terminal::size().expect("Coud not get terminal size!");
        let status_bar_height = 1;
        let message_bar_height = 1;
        let view = Window::new(status_bar_height + message_bar_height);
        Self {
            should_quit: false,
            mode: EditorMode::NormalMode,
            window: view,
            status_bar: StatusBar::new(height - status_bar_height - message_bar_height),
            command_bar: CommandBar::new(height - message_bar_height),
            last_search_pattern: None,
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
            EditorMode::CmdlineMode(_) => self.evalueate_event_in_cmdline_mode(event)?,
        }
        Ok(())
    }
    fn evaluate_evnet_in_normal_mode(&mut self, event: &Event) -> Result<(), std::io::Error> {
        let command = NormalModeCommand::from_key_event(event);
        match command {
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
            NormalModeCommand::EnterInsertModeBeginNewLineAbove => {
                self.window.begin_newline_above()?;
                self.mode = EditorMode::InsertMode;
            }
            NormalModeCommand::EnterInsertModeBeginNewLineBelow => {
                self.window.begin_newline_below()?;
                self.mode = EditorMode::InsertMode;
            }
            NormalModeCommand::EnterCmdlineMode(submode) => {
                self.mode = EditorMode::CmdlineMode(submode);
                self.command_bar.clear_cmdline();
                self.command_bar.set_cmdline_prompt(submode);
            }
            NormalModeCommand::SearchNext => {
                self.window.search(
                    self.last_search_pattern.as_deref(),
                    SearchDirection::Forward,
                )?;
            }
            NormalModeCommand::SearchPrev => {
                self.window.search(
                    self.last_search_pattern.as_deref(),
                    SearchDirection::Backward,
                )?;
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
            }
            CmdlineModeCommand::Execute => {
                // if let EditorMode::CmdlineMode(submode) = self.mode {}
                match self.mode {
                    EditorMode::CmdlineMode(CmdlineSubmode::Cmdline) => {
                        self.parse_and_execute_cmdline_command()?;
                    }
                    EditorMode::CmdlineMode(CmdlineSubmode::Search) => {
                        self.execute_search(SearchDirection::Forward)?;
                    }
                    _ => {
                        panic!("You should be in cmdline mode here.")
                    }
                }
                self.mode = EditorMode::NormalMode;
            }
            CmdlineModeCommand::Insert(c) => {
                self.command_bar.insert_char(c);
            }
            CmdlineModeCommand::Backspace => {
                self.command_bar.handle_backspace();
            }
            CmdlineModeCommand::Nop => (),
        }
        Ok(())
    }
    fn parse_and_execute_cmdline_command(&mut self) -> Result<(), std::io::Error> {
        let raw_cmdline = self.command_bar.get_current_cmdline();
        if raw_cmdline.len() >= 1 {
            let cmd = CmdlineCommands::parse_cmdline(&raw_cmdline);
            match cmd {
                Ok(cmd) => {
                    self.execute_cmdline_command(cmd)?;
                    self.command_bar.clear_cmdline();
                }
                Err(msg) => {
                    self.command_bar.set_error_message(&msg);
                }
            }
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
            CmdlineCommands::Saveas(filename) => {
                self.window.save_buffer_with_filename(&filename)?;
            }
            CmdlineCommands::StopHighlighting => {
                self.last_search_pattern = None;
                self.window.set_needs_redraw();
            }
        }
        Ok(())
    }
    fn execute_search(&mut self, direction: SearchDirection) -> Result<(), std::io::Error> {
        let pattern = Some(self.command_bar.get_raw_cmdline());
        self.window.search(pattern.as_deref(), direction)?;
        self.last_search_pattern = pattern;
        self.command_bar.clear_cmdline();
        Ok(())
    }
    fn refresh_screen(&mut self) -> Result<(), std::io::Error> {
        if self.should_quit {
            Terminal::clear_screen()?;
            print!("Goodbye!\r\n");
        } else {
            let context = RenderContext {
                search_pattern: self.last_search_pattern.clone(),
            };
            self.window.render(&context)?;
            self.status_bar.render()?;
            self.command_bar.render()?;
            let pos = self.window.get_relative_position();
            Terminal::move_cursor_to(pos)?;
        }
        Ok(())
    }
}
