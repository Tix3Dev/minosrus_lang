mod execution;
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
    println!("DON'T USE VERINES! THEY AREN'T WORKING PROPERLY");

    // make ExecData instance
    let mut exec_data_variable = ExecData::new();

    // one loop iteration = one prompt (" >")
    loop {
        let mut input = String::new();

        print!("{}> ", exec_data_variable.indentation);
        let _ = io::stdout().flush(); // needed because of the print! macro

        match io::stdin().read_line(&mut input) { // reading input
            Ok(_) => {
                if !(input.trim().to_string().is_empty()) {
                    exec_data_variable.exec(tokenizer::make_tokens(input));
                }
            },

            Err(e) => println!("Something with you input went wrong: {}", e)
        }
    }
}
