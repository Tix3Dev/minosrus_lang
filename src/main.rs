#![feature(or_patterns)]
#![feature(bindings_after_at)]

mod execution;
mod verine_expression;
mod tokenizer;

use std::io::{self, Write};
use std::collections::HashMap;

// here are all variables for exec saved
pub struct ExecData {
    // here are all the global variables stored; not changed after one loop iteration
    pub global_variables: HashMap<String, tokenizer::ValueEnum>,

    // save state of indentation
    pub indentation: String,

    // here is all block code saved (except for function code)
    pub block_code: Vec<Vec<(String, tokenizer::ValueEnum)>>,

    // here are all functions saved
    pub functions: HashMap<String, Vec<Vec<(String, tokenizer::ValueEnum)>>>,

    // keep track of the current block code type (normal or functions)
    pub current_block_type: (String, String)
}

impl ExecData {
    pub fn new() -> Self {
        Self {
            global_variables: HashMap::new(),
            indentation: "".to_string(),
            block_code: Vec::new(),
            functions: HashMap::new(),
            current_block_type: ("".to_string(), "".to_string())
        }
    }
}

fn main() {
    // some starting text
    println!("EVERYTHING HAS TO BE UPPERCASE!");

    // make ExecData instance
    let mut exec_data_variable = ExecData::new();

    // one loop iteration = one prompt (" >")
    loop {
        let mut valid_input = true;
        let mut input = String::new();

        print!("{}> ", exec_data_variable.indentation);
        let _ = io::stdout().flush(); // needed because of the print! macro

        match io::stdin().read_line(&mut input) { // reading input
            Ok(_) => {
                if !(input.trim().to_string().is_empty()) {
                    for character in input.chars() {
                        if character.is_lowercase() {
                            println!("SYNTAX ERROR: INPUT INCLUDES LOWERCASE CHARACTER!");
                            valid_input = false;
                            break;
                        }
                    }
                    if valid_input {
                        let tokens = tokenizer::make_tokens(input, &mut exec_data_variable.global_variables);
                        exec_data_variable.exec(tokens);
                    }
                }
            },

            Err(e) => println!("Something with you input went wrong: {}", e)
        }
    }
}
