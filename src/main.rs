mod execution;

use std::io::{self, Write};

fn main() {
    println!("EVERYTHING HAS TO BE UPPERCASE!");

    loop {
        let mut input = String::new();

        print!("> ");
        let _ = io::stdout().flush();

        match io::stdin().read_line(&mut input) {
            Ok(_) => {
                input = input.trim().to_string();
                if !(input.is_empty()) {
                    execution::exec(input);
                }
            },

            Err(e) => println!("Something with you input went wrong: {}", e)
        }
    }
}
