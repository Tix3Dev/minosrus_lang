mod execution;
mod verine_expression;

use std::io::{self, Write};
use std::collections::HashMap;

fn main() {
    // some starting text
    println!("EVERYTHING HAS TO BE UPPERCASE!");
    println!("DON'T USE VERINES! THEY AREN'T WORKING PROPERLY");

    // here are all the global variables stored; not changed after one loop iteration
    let mut global_variables: HashMap<String, execution::tokenizer::ValueEnum> = HashMap::new();

    // one loop iteration = one prompt (" >")
    loop {
        let mut input = String::new();

        print!("> ");
        let _ = io::stdout().flush(); // needed because of the print! macro

        match io::stdin().read_line(&mut input) { // reading input
            Ok(_) => {
                if !(input.trim().to_string().is_empty()) {
                    execution::exec(input, &mut global_variables);
                }
            },

            Err(e) => println!("Something with you input went wrong: {}", e)
        }
    }
}
