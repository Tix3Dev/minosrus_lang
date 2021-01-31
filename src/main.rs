mod execution;

use std::io::{self, Write};
use std::collections::HashMap;

fn main() {
    // some starting text
    println!("EVERYTHING HAS TO BE UPPERCASE!");
    println!("DON'T USE VERINES! THEY AREN'T WORKING PROPERLY");

    // here are all the global variables stored; not changed after one loop iteration
    let mut global_variables: HashMap<String, execution::tokenizer::ValueEnum> = HashMap::new();

    // save state of indentation
    let mut indentation = "".to_string();

    // here is all block code saved (except for function code)
    let mut block_code: Vec<Vec<(String, execution::tokenizer::ValueEnum)>> = Vec::new();

    // here are all functions saved 
    let mut functions: HashMap<String, Vec<Vec<(String, execution::tokenizer::ValueEnum)>>> = HashMap::new(); 

    // keep track of the current block code type (normal or functions)
    let mut current_block_type = ("".to_string(), "".to_string());

    // one loop iteration = one prompt (" >")
    loop {
        let mut input = String::new();

        print!("{}> ", indentation);
        let _ = io::stdout().flush(); // needed because of the print! macro

        match io::stdin().read_line(&mut input) { // reading input
            Ok(_) => {
                if !(input.trim().to_string().is_empty()) {
                    execution::exec(input, &mut global_variables, &mut indentation, &mut block_code, &mut functions, &mut current_block_type);
                }
            },

            Err(e) => println!("Something with you input went wrong: {}", e)
        }
    }
}
