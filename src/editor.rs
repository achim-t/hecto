use crossterm::{
    event::{read, Event, KeyCode, KeyModifiers},
    terminal,
};
pub struct Editor {
    should_quit: bool,
}

impl Editor {
    pub fn run(&mut self) {
        terminal::enable_raw_mode().unwrap();

        loop {
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
