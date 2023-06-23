use crate::Terminal;
use crossterm::{
    event::{ Event, KeyCode, KeyModifiers}
};
pub struct Editor {
    should_quit: bool,
    terminal: Terminal,
}

impl Editor {
    pub fn run(&mut self) {
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
        let event = Terminal::read_key()?;
        if let Event::Key(pressed_key) = event {
            match (pressed_key.modifiers, pressed_key.code) {
                (KeyModifiers::CONTROL, KeyCode::Char('q')) => self.should_quit = true,
                _ => (),
            }
        }; 
        Ok(())
    }

    fn refresh_screen(&mut self) -> Result<(), std::io::Error> {
        Terminal::clear_screen();
        if self.should_quit {
            println!("Goodbye\r");
        } else {
            self.draw_rows();
            Terminal::cursor_position(0,0);
        }
        Terminal::flush()
    }

    fn draw_rows(&self) {
        for _ in 0..self.terminal.size().height {
            println!("~\r");
        }
    }

    pub fn default() -> Self {
        Self {
            should_quit: false,
            terminal: Terminal::default().expect("Failed to initialize terminal"),
        }
    }
}



fn die(e: std::io::Error) {
    panic!("{}", e);
}


