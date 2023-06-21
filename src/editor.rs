use crossterm::{
    event::{read, Event, KeyCode, KeyModifiers},
    terminal,
};
pub struct Editor {}

impl Editor {
    pub fn run(&self) {
        terminal::enable_raw_mode().unwrap();

        loop {
            if let Err(error) = self.process_keypress() {
                die(error);
            }
        }
    }

    fn process_keypress(&self) -> Result<(), std::io::Error> {
        let event = read_key()?;
        if let Event::Key(pressed_key) = event {
            match (pressed_key.modifiers, pressed_key.code) {
                (KeyModifiers::CONTROL, KeyCode::Char('q')) => panic!("program end"),
                _ => (),
            }
        }; 
        Ok(())
    }

    pub fn default() -> Self {
        Self {}
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
