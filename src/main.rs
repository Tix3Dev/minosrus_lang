mod execution;

use std::io::{self, Write};
use std::collections::HashMap;

// here are all variables for exec saved
pub struct ExecData {
    // here are all the global variables stored; not changed after one loop iteration
    pub global_variables: HashMap<String, execution::tokenizer::ValueEnum>,
    
    // save state of indentation
    pub indentation: String,

    // here is all block code saved (except for function code)
    pub block_code: Vec<Vec<(String, execution::tokenizer::ValueEnum)>>,

    // here are all functions saved 
    pub functions: HashMap<String, Vec<Vec<(String, execution::tokenizer::ValueEnum)>>>,

    // keep track of the current block code type (normal or functions)
    pub current_block_type: (String, String)
}

impl ExecData {
    pub fn new(a, b, c, d, e) -> Self {
        Self {
            global_variables: a,
            indentation: b,
            block_code: c,
            functions: d,
            current_block_type: e
        }
    }
}

fn main() {
    // some starting text
    println!("EVERYTHING HAS TO BE UPPERCASE!");
    println!("DON'T USE VERINES! THEY AREN'T WORKING PROPERLY");

    // make ExecData instance
    let mut exec_data_variable = ExecData::new(HashMap::new(), "".to_string(), Vec::new(), HashMap::new(), ("".to_string(), "".to_string()));

    // one loop iteration = one prompt (" >")
    loop {
        let mut input = String::new();

        print!("{}> ", indentation);
        let _ = io::stdout().flush(); // needed because of the print! macro

        match io::stdin().read_line(&mut input) { // reading input
            Ok(_) => {
                if !(input.trim().to_string().is_empty()) {
                    execution::exec(input, exec_data_variable);
                }
            },

            Err(e) => println!("Something with you input went wrong: {}", e)
        }
    }
}
