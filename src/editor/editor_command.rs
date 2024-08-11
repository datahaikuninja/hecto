use crossterm::event::Event::{self, Key};
use crossterm::event::{KeyCode, KeyEvent, KeyEventKind, KeyModifiers};

pub enum Direction {
    Left,
    Right,
    Up,
    Down,
}

pub enum Command {
    CursorMove(Direction),
    Quit,
    Nop,
}

impl Command {
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
                _ => Self::Nop,
            };
            command
        } else {
            Self::Nop
        }
    }
}
