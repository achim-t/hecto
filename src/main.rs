use std::io::{self, Read};

use crossterm::{
    terminal
};
fn main() {
    terminal::enable_raw_mode().unwrap();
    for b in io::stdin().bytes() {
        let c = b.unwrap() as char;
        println!("{}", c);
        if c == 'q' {
            break;
        }
    }
}
