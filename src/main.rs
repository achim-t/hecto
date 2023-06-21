use std::io::{self, Read};

use crossterm::{
    terminal,
};
fn main() {
    terminal::enable_raw_mode().unwrap();

    fn to_ctrl_byte(c: char) -> u8 {
        let byte = c as u8;
        byte & 0b0001_1111
    }

    for b in io::stdin().bytes() {
        let b = b.unwrap();
        let c = b as char;
        if c.is_control() {
            println!("{:?} \r", b);
        } else {
            println!("{:?} ({})\r", b,c);
        }
        if b == to_ctrl_byte('q') {
            break;
        }
    }
}
