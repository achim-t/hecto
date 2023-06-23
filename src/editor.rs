use std::io::{self, stdout, Write};
use crossterm::{
    event::{read, Event, KeyCode, KeyModifiers},
    terminal, ExecutableCommand, QueueableCommand, cursor
};
pub struct Editor {
    should_quit: bool,
}

impl Editor {
    pub fn run(&mut self) {
        terminal::enable_raw_mode().unwrap();

        loop {
            if let Err(error) = self.refresh_screen() {
                die(error);
            }
            if self.should_quit {
                break;
            }
            if let Err(error) = self.process_keypress() {
                die(error);
            }
        }
    }

    fn process_keypress(&mut self) -> Result<(), std::io::Error> {
        let event = read_key()?;
        if let Event::Key(pressed_key) = event {
            match (pressed_key.modifiers, pressed_key.code) {
                (KeyModifiers::CONTROL, KeyCode::Char('q')) => self.should_quit = true,
                _ => (),
            }
        }; 
        Ok(())
    }

    fn refresh_screen(&mut self) -> Result<(), std::io::Error> {
        clear_screen();
        if self.should_quit {
            println!("Goodbye\r");
        } else {
            self.draw_rows();
            stdout().queue(cursor::MoveTo(1, 1)).ok();
        }
        io::stdout().flush()
    }

    fn draw_rows(&self) {
        for _ in 0..24 {
            println!("~\r");
        }
    }

    pub fn default() -> Self {
        Self {
            should_quit: false
        }
    }
}

fn read_key() -> Result<Event, std::io::Error> {
    loop {
        let event = read();
        return event
    }
}

fn die(e: std::io::Error) {
    panic!("{}", e);
}

fn clear_screen() {
    stdout().execute(terminal::Clear(terminal::ClearType::All)).ok();
}
