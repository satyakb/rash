use std::io::{self, Write};
use std::process::Command;

pub const PROMPT: &'static str = "$ ";

fn main() {
    loop {
        print!("{}", PROMPT);
        io::stdout().flush().unwrap();

        let mut buf = String::new();
        match io::stdin().read_line(&mut buf) {
            Err(e) => {
                panic!("rash: error: {}", e);
            },
            Ok(_) => {
                buf.pop(); // Remove ending newline

                let mut split : Vec<&str> = buf.split(" ").collect();
                let com = split.remove(0);

                let status = Command::new(com)
                                    .args(split.as_slice())
                                    .status();
                match status {
                    Ok(_) => (),
                    Err(_) => {
                        println!("rash: command not found: {}", com);
                    },
                }
            }
        }
    }
}
