use crossterm::event::Event::{self, Key};
use crossterm::event::{KeyCode, KeyEvent, KeyEventKind, KeyModifiers};

pub enum EditorMode {
    NormalMode,
    InsertMode,
    CmdlineMode,
}

impl Default for EditorMode {
    fn default() -> Self {
        Self::NormalMode
    }
}

pub enum Direction {
    Left,
    Right,
    Up,
    Down,
}

pub enum NormalModeCommand {
    CursorMove(Direction),
    EnterInsertMode,
    EnterInsertModeAppend,
    EnterCmdlineMode,
    Save,
    Quit,
    Nop,
}

impl NormalModeCommand {
    pub fn from_key_event(event: &Event) -> Self {
        if let Key(KeyEvent {
            code,
            modifiers,
            kind: KeyEventKind::Press,
            ..
        }) = event
        {
            let command = match code {
                KeyCode::Char('q') if *modifiers == KeyModifiers::CONTROL => Self::Quit,
                KeyCode::Char('s') if *modifiers == KeyModifiers::CONTROL => Self::Save,
                KeyCode::Char('h') => Self::CursorMove(Direction::Left),
                KeyCode::Char('j') => Self::CursorMove(Direction::Down),
                KeyCode::Char('k') => Self::CursorMove(Direction::Up),
                KeyCode::Char('l') => Self::CursorMove(Direction::Right),
                KeyCode::Char('i') => Self::EnterInsertMode,
                KeyCode::Char('a') => Self::EnterInsertModeAppend,
                KeyCode::Char(':') => Self::EnterCmdlineMode,
                _ => Self::Nop,
            };
            command
        } else {
            Self::Nop
        }
    }
}

pub enum InsertModeCommand {
    LeaveInsertMode,
    Insert(char),
    Backspace,
    InsertNewLine,
    Nop,
}

impl InsertModeCommand {
    pub fn from_key_event(event: &Event) -> Self {
        if let Key(KeyEvent {
            code,
            modifiers,
            kind: KeyEventKind::Press,
            ..
        }) = event
        {
            let command = match code {
                KeyCode::Char(c) if *modifiers == KeyModifiers::NONE => Self::Insert(*c),
                KeyCode::Tab if *modifiers == KeyModifiers::NONE => Self::Insert('\t'),
                KeyCode::Backspace if *modifiers == KeyModifiers::NONE => Self::Backspace,
                KeyCode::Enter if *modifiers == KeyModifiers::NONE => Self::InsertNewLine,
                KeyCode::Esc => Self::LeaveInsertMode,
                _ => Self::Nop,
            };
            command
        } else {
            Self::Nop
        }
    }
}

pub enum CmdlineModeCommand {
    LeaveCmdlineMode,
    Execute,
    Insert(char),
    Backspace,
    Nop,
}

impl CmdlineModeCommand {
    pub fn from_key_event(event: &Event) -> Self {
        if let Key(KeyEvent {
            code,
            modifiers,
            kind: KeyEventKind::Press,
            ..
        }) = event
        {
            let command = if *modifiers == KeyModifiers::NONE {
                match code {
                    KeyCode::Esc => Self::LeaveCmdlineMode,
                    KeyCode::Enter => Self::Execute,
                    KeyCode::Char(c) => Self::Insert(*c),
                    KeyCode::Backspace => Self::Backspace,
                    _ => Self::Nop,
                }
            } else {
                Self::Nop
            };
            command
        } else {
            Self::Nop
        }
    }
}
