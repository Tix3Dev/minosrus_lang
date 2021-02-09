mod execution;
mod tokenizer;

use std::io::{self, Write};
use std::collections::HashMap;
use std::env;
use std::path::Path;
use std::fs::File;
use std::io::{BufRead, BufReader};

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

fn repl() {
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
                        let return_of_execution = exec_data_variable.exec(tokenizer::make_tokens(input));
                        if return_of_execution != "".to_string() {
                            // print error message
                            println!("{}", return_of_execution); 
                        }
                    }
                }
            },

            Err(e) => println!("Something with you input went wrong: {}", e)
        }
    }
}

fn main() {
    // get command line arguments
    let args: Vec<String> = env::args().collect(); 

    // check if valid
    if args.len() == 3 {
        if args[1] == "--execute".to_string() {
            if Path::new(&args[2]).exists() {
                let file = File::open(&args[2]).unwrap();
                let reader = BufReader::new(file);

                for (line_nr, line) in reader.lines().enumerate() {
                    let print_err = | error_message | {
                        println!("- ERROR OCCURED ON LINE NR. {}", line_nr + 1);
                        println!("  -> {}", error_message);
                    };

                    let line = line.unwrap();

                    let mut valid_input = true;

                    // make ExecData instance
                    let mut exec_data_variable = ExecData::new();

                    if !(line.trim().to_string().is_empty()) {
                        for character in line.chars() {
                            if character.is_lowercase() {
                                print_err("SYNTAX ERROR: INPUT INCLUDES LOWERCASE CHARACTER!");
                                valid_input = false;
                                break;
                            }
                        }
                        if valid_input {
                            let return_of_execution = exec_data_variable.exec(tokenizer::make_tokens(line));
                            if return_of_execution != "".to_string() {
                                print_err(&return_of_execution);
                            }
                        }
                    }
                }
            } else {
                println!("INTERPRETER ERROR: THIRD COMMAND LINE ARGUMENT IS NOT A VALID PATH!");
            }
        } else {
            println!("INTERPRETER ERROR: COMMAND LINE ARGUMENTS AREN'T RIGHT!");
            println!("HELP: ARGUMENTS");
            println!("  --execute <path/filename.morl> | WILL EXECUTE A MORL FILE");
            println!("  --repl                         | WILL START A REPL");
        }
    }
    else if args.len() == 2 {
        if args[1] == "--repl".to_string() {
            repl();
        } else {
            println!("INTERPRETER ERROR: COMMAND LINE ARGUMENTS AREN'T RIGHT!"); 
            println!("HELP: ARGUMENTS");
            println!("  --execute <path/filename.morl> | WILL EXECUTE A MORL FILE");
            println!("  --repl                         | WILL START A REPL");
        }
    } else {
        println!("INTERPRETER ERROR: COMMAND LINE ARGUMENTS AREN'T RIGHT!");
        println!("HELP: ARGUMENTS");
        println!("  --execute <path/filename.morl> | WILL EXECUTE A MORL FILE");
        println!("  --repl                         | WILL START A REPL");
    }
}
