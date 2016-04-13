use std::io::{self, Write};
use std::process::Command;
use std::process::Stdio;

fn main() {
    loop {
        print!("> ");
        io::stdout().flush().unwrap();

        let mut buf = String::new();
        match io::stdin().read_line(&mut buf) {
            Err(e) => {
                panic!("error: {}", e);
            },
            Ok(_) => {
                buf.pop(); // Remove last newline

                let mut split : Vec<&str> = buf.split(" ").collect();
                let com = split.remove(0);

                let output = Command::new(com)
                                    .args(split.as_slice())
                                    .stdout(Stdio::inherit())
                                    //.stderr(Stdio::inherit())
                                    .stdin(Stdio::inherit())
                                    .output();
                match output {
                    Ok(o) => {
                        print!("{}", String::from_utf8_lossy(&o.stdout));
                    },
                    Err(e) => {
                        println!("Error: {}", e);
                    },
                }
            }
        }
    }
}
