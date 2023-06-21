use crossterm::{
    event::{read, Event, KeyCode, KeyModifiers},
    terminal,
};
pub struct Editor {}

impl Editor {
    pub fn run(&self) {
        terminal::enable_raw_mode().unwrap();

        loop {
            let event = read();
            match event {
                Ok(event) => {
                    if let Event::Key(pressed_key) = event {
                        match (pressed_key.modifiers, pressed_key.code) {
                            (KeyModifiers::CONTROL, KeyCode::Char('q')) => break,
                            (_, KeyCode::Char(c)) => {
                                if c.is_control() {
                                    println!("{:?}\r", c as u8);
                                } else {
                                    println!("{:?} ({})\r", c as u8, c)
                                }
                            }
                            _ => println!("{:?}\r", pressed_key),
                        }
                    }
                }
                Err(err) => die(err),
            }
        }
    }

    pub fn default() -> Self {
        Editor{}
    }
}

fn die(e: std::io::Error) {
    panic!("{}", e);
}