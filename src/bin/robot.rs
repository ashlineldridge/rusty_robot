extern crate rusty_robot;

use std::io;
use std::io::BufRead;

fn main() {
    let mut game = rusty_robot::new(5, 5);
    let stdin = io::stdin();
    for line in stdin.lock().lines() {
        match line {
            Ok(line) => {
                let res = match line.as_str() {
                    "MOVE" => game.r#move(),
                    "LEFT" => game.rotate_left(),
                    "RIGHT" => game.rotate_right(),
                    "REPORT" => game.report().map(|v| println!("{}", v)),
                    _ => Err(format!("Unrecognised command '{}'", line)),
                };

                let _ = res.is_ok();
                match res {
                    Err(e) => println!("{}", e),
                    _ => (),
                }
            }
            // TODO: Handle errors properly by printing them and exit 1 (if let Err(e) = ...)
            Err(e) => panic!("Error reading input: {}", e),
        }
    }
}
