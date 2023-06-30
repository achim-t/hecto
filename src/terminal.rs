use crossterm::{
    cursor,
    terminal,
    ExecutableCommand,
    QueueableCommand,
    style::{ ResetColor, SetColors },
};
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
        let Position { x, y } = *position;
        let x = x as u16;
        let y = y as u16;
        stdout().queue(cursor::MoveTo(x, y)).ok();
    }

    pub fn flush() -> Result<(), std::io::Error> {
        stdout().flush()
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

    pub fn set_colors(colors: crossterm::style::Colors) {
        stdout().execute(SetColors(colors)).ok();
    }

    pub fn reset_colors() {
        stdout().execute(ResetColor).ok();
    }
}
