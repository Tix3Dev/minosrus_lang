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

fn print_interpreter_error(error_message: &str) {
    println!("{}", error_message);
    println!("HELP: ARGUMENTS");
    println!("  --execute <path/filename.morl> | WILL EXECUTE A MORL FILE");
    println!("  --repl                         | WILL START A REPL");
}

fn file_execution(args_2: String) {
    if Path::new(&args_2).exists() {
        let file = File::open(&args_2).unwrap();
        let reader = BufReader::new(file);

        let mut exec_data_variable = ExecData::new();

        let mut token_collection_of_all_lines: Vec<Vec<(String, tokenizer::ValueEnum)>> = Vec::new();
        let mut collection_of_all_lines: Vec<String> = Vec::new();

        let mut tokenizing_error_count = 0;

        // tokenizing - syntax errors - compilation errors - all lines are checked
        for (line_nr, line) in reader.lines().enumerate() {
            let line = line.unwrap();

            let mut print_err = | error_message | {
                println!("- ERROR OCCURED ON LINE NR. {}: '{}'", line_nr + 1, line);
                println!("  -> {}", error_message);
                tokenizing_error_count += 1;
            };

            if !(line.trim().to_string().is_empty()) {
                for character in line.chars() {
                    if character.is_lowercase() {
                        print_err("INPUT ERROR: INPUT INCLUDES LOWERCASE CHARACTER!");
                        println!("INTERPRETER STOPPPED!");
                        return;
                    }
                }

                let token_collection_of_current_line = tokenizer::make_tokens(&line); 

                // check for syntax errors
                if let Some((_, value)) = token_collection_of_current_line.iter().find(|(key, _)| key == &"ERROR_MESSAGE") {
                    match value {
                        tokenizer::ValueEnum::String(v) => {
                            print_err(format!("SYNTAX ERROR: {}", v).as_str());
                        },
                        _ => unreachable!("SOMEHOW THIS SHOULDN'T BE PRINTED!")
                    }
                }

                // save stuff if there were no errors
                token_collection_of_all_lines.push(token_collection_of_current_line);
                collection_of_all_lines.push(line);
            }
        }

        if tokenizing_error_count != 0 {
            if tokenizing_error_count == 1 {
                println!("\nABORTING DUE THE PREVIOUS {} ERROR!", tokenizing_error_count); 
            } else {
                println!("\nABORTING DUE THE PREVIOUS {} ERRORS!", tokenizing_error_count); 
            }
            println!("INTERPRETER STOPPED!");
            return;
        }

        // executing - execution errors - runtime errors - execution stops when an error occurs
        for (tokenized_line_nr, tokenized_line) in token_collection_of_all_lines.iter().enumerate() {
            let print_err = | error_message | {
                println!("- ERROR OCCURED ON LINE NR. {}: '{}'", tokenized_line_nr + 1, collection_of_all_lines[tokenized_line_nr]);
                println!("  -> {}", error_message);
                println!("INTERPRETER STOPPED DUE PREVIOUS RUNTIME ERROR!");
            };

            // check if indentation is right
            let mut indentation_count = 0;
            for character in collection_of_all_lines[tokenized_line_nr].chars() {
                if character == ' ' {
                    indentation_count += 1;
                }
                else if character == '\t' {
                    indentation_count += 4;
                } else {
                    break;
                }
            }
            if exec_data_variable.indentation.len() != indentation_count {
                print_err("INDENTATION ERROR!");
                return;
            }

            let return_of_execution = exec_data_variable.exec(tokenized_line.to_vec());
            if return_of_execution != "".to_string() {
                print_err(&return_of_execution);
                return;
            }
        }

        if exec_data_variable.indentation != "".to_string() {
            let len_of_file = collection_of_all_lines.len();
            println!("- ERROR OCCURED ON LINE NR. {}: '{}'", len_of_file, collection_of_all_lines[len_of_file - 1]);
            println!("  -> EXECUTION ERROR: BLOCK CODE ISN'T CLOSED!");
            println!("INTERPRETER STOPPED DUE PREVIOUS RUNTIME ERROR!");
        }
    } else {
        print_interpreter_error("INTERPRETER ERROR: THIRD COMMAND LINE ARGUMENT IS NOT A VALID PATH!");
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
                        let return_of_execution = exec_data_variable.exec(tokenizer::make_tokens(&input));
                        if return_of_execution != "".to_string() {
                            // print error message
                            println!("{}", return_of_execution); 
                        }
                    }
                }
            },

            Err(_) => println!("INPUT ERROR: SOMETHING WITH YOUR INPUT IS WRONG!")
        }
    }
}

fn main() {
    // get command line arguments
    let args: Vec<String> = env::args().collect(); 

    // check if valid
    if args.len() == 3 {
        if args[1] == "--execute".to_string() {
            file_execution(args[2].to_string());
        } else {
            print_interpreter_error("INTERPRETER ERROR: COMMAND LINE ARGUMENTS AREN'T RIGHT!");
        }
    }
    else if args.len() == 2 {
        if args[1] == "--repl".to_string() {
            repl();
        } else {
            print_interpreter_error("INTERPRETER ERROR: COMMAND LINE ARGUMENTS AREN'T RIGHT!"); 
        }
    } else {
        print_interpreter_error("INTERPRETER ERROR: COMMAND LINE ARGUMENTS AREN'T RIGHT!"); 
    }
}
