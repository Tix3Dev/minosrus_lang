mod execution;

use std::io::{self, Write};

fn main() {
    println!("EVERYTHING HAS TO BE UPPERCASE!");
    println!("DON' USE VERINES! THEY AREN'T WORKING PROPERLY");

    loop {
        let mut input = String::new();

        print!("> ");
        let _ = io::stdout().flush();

        match io::stdin().read_line(&mut input) {
            Ok(_) => {
                if !(input.trim().to_string().is_empty()) {
                    execution::exec(input);
                }
            },

            Err(e) => println!("Something with you input went wrong: {}", e)
        }
    }
}
