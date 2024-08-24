use crossterm::event::Event::{self, Key};
use crossterm::event::{KeyCode, KeyEvent, KeyEventKind, KeyModifiers};

pub enum EditorMode {
    NormalMode,
    InsertMode,
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
                KeyCode::Char('h') => Self::CursorMove(Direction::Left),
                KeyCode::Char('j') => Self::CursorMove(Direction::Down),
                KeyCode::Char('k') => Self::CursorMove(Direction::Up),
                KeyCode::Char('l') => Self::CursorMove(Direction::Right),
                KeyCode::Char('i') => Self::EnterInsertMode,
                KeyCode::Char('a') => Self::EnterInsertModeAppend,
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
                KeyCode::Esc => Self::LeaveInsertMode,
                _ => Self::Nop,
            };
            command
        } else {
            Self::Nop
        }
    }
}
