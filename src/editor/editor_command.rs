use crossterm::event::Event::{self, Key};
use crossterm::event::{KeyCode, KeyEvent, KeyEventKind, KeyModifiers};

#[derive(Clone, Copy)]
pub enum CmdlineSubmode {
    Cmdline,
    Search,
}

pub enum EditorMode {
    NormalMode,
    InsertMode,
    CmdlineMode(CmdlineSubmode),
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
    EnterCmdlineMode(CmdlineSubmode),
    SearchNext,
    SearchPrev,
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
                KeyCode::Char('h') => Self::CursorMove(Direction::Left),
                KeyCode::Char('j') => Self::CursorMove(Direction::Down),
                KeyCode::Char('k') => Self::CursorMove(Direction::Up),
                KeyCode::Char('l') => Self::CursorMove(Direction::Right),
                KeyCode::Char('i') => Self::EnterInsertMode,
                KeyCode::Char('a') => Self::EnterInsertModeAppend,
                KeyCode::Char(':') => Self::EnterCmdlineMode(CmdlineSubmode::Cmdline),
                KeyCode::Char('/') => Self::EnterCmdlineMode(CmdlineSubmode::Search),
                KeyCode::Char('n') => Self::SearchNext,
                KeyCode::Char('N') => Self::SearchPrev,
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
