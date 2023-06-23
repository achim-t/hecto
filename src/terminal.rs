use crossterm::{ cursor, event::{ read, Event }, terminal, ExecutableCommand, QueueableCommand };
use std::io::{ stdout, Write };

use crate::Position;

pub struct Size {
    pub width: u16,
    pub height: u16,
}

pub struct Terminal {
    size: Size,
}

impl Terminal {
    pub fn default() -> Result<Self, std::io::Error> {
        let size = terminal::size()?;
        terminal::enable_raw_mode().ok();
        Ok(Self {
            size: Size {
                width: size.0,
                height: size.1.saturating_sub(2),
            },
        })
    }

    pub fn size(&self) -> &Size {
        &self.size
    }

    pub fn clear_screen() {
        stdout().execute(terminal::Clear(terminal::ClearType::All)).ok();
    }

    pub fn cursor_position(position: &Position) {
        let Position { mut x, mut y } = position;
        let x = x.saturating_add(1) as u16;
        let y = y.saturating_add(1) as u16;
        stdout()
            .queue(cursor::MoveTo(x - 1, y - 1))
            .ok();
    }

    pub fn flush() -> Result<(), std::io::Error> {
        stdout().flush()
    }

    pub fn read_key() -> Result<Event, std::io::Error> {
        loop {
            let event = read();
            return event;
        }
    }

    pub fn cursor_hide() {
        stdout().execute(cursor::DisableBlinking).ok();
    }

    pub fn cursor_show() {
        stdout().execute(cursor::EnableBlinking).ok();
    }

    pub fn clear_current_line() {
        stdout().execute(terminal::Clear(terminal::ClearType::CurrentLine)).ok();
    }
}
