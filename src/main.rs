use std::io::{self, Write};

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
                print!("{}", buf);
            }
        }
    }
}
