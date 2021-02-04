use crate::ExecData;
use crate::tokenizer;

use std::collections::HashMap;
use std::process;

enum OrderEnum {
    SingleOption(Vec<&'static str>),
    MultipleOptions(Vec<Vec<&'static str>>)
}

fn add_indentation(indentation: &mut String) {
    indentation.push_str("    ");
}

fn subtract_indentation(indentation: &mut String) {
    if indentation.to_string() == "    ".to_string() {
        *indentation = "".to_string();
    } else {
        *indentation = indentation[..4].to_string();
    }
}

fn execute_block_code(block_code: Vec<Vec<(String, tokenizer::ValueEnum)>>) {
    let mut code_block_exec_data_variable = ExecData::new();
    for line in block_code.iter() {
        code_block_exec_data_variable.exec(line.to_vec());
    }
}

fn check_block_code_condition(operator: String, block_code: Vec<Vec<(String, tokenizer::ValueEnum)>>) -> bool {
    if operator == "==" {
        match (&block_code[0][1].1, &block_code[0][3].1) {
            (tokenizer::ValueEnum::String(a), tokenizer::ValueEnum::String(b)) => {
                if a == b {
                    true
                } else {
                    false
                }
            },
            (tokenizer::ValueEnum::Integer(a), tokenizer::ValueEnum::Integer(b)) => {
                if a == b {
                    true
                } else {
                    false
                }
            },
            _ => unreachable!("SOMEHOW THIS SHOULDN'T BE PRINTED!")
        } 
    }
    else if operator == "!=" {
        match (&block_code[0][1].1, &block_code[0][3].1) {
            (tokenizer::ValueEnum::String(a), tokenizer::ValueEnum::String(b)) => {
                if a != b {
                    true
                } else {
                    false
                }
            },
            (tokenizer::ValueEnum::Integer(a), tokenizer::ValueEnum::Integer(b)) => {
                if a != b {
                    true
                } else {
                    false
                }
            },
            _ => unreachable!("SOMEHOW THIS SHOULDN'T BE PRINTED!")
        } 
    }
    else if operator == "<" {
        match (&block_code[0][1].1, &block_code[0][3].1) {
            (tokenizer::ValueEnum::String(a), tokenizer::ValueEnum::String(b)) => {
                if a < b {
                    true
                } else {
                    false
                }
            },
            (tokenizer::ValueEnum::Integer(a), tokenizer::ValueEnum::Integer(b)) => {
                if a < b {
                    true
                } else {
                    false
                }
            },
            _ => unreachable!("SOMEHOW THIS SHOULDN'T BE PRINTED!")
        } 
    }
    else if operator == ">" {
        match (&block_code[0][1].1, &block_code[0][3].1) {
            (tokenizer::ValueEnum::String(a), tokenizer::ValueEnum::String(b)) => {
                if a > b {
                    true
                } else {
                    false
                }
            },
            (tokenizer::ValueEnum::Integer(a), tokenizer::ValueEnum::Integer(b)) => {
                if a > b {
                    true
                } else {
                    false
                }
            },
            _ => unreachable!("SOMEHOW THIS SHOULDN'T BE PRINTED!")
        } 
    }
    else if operator == "<=" {
        match (&block_code[0][1].1, &block_code[0][3].1) {
            (tokenizer::ValueEnum::String(a), tokenizer::ValueEnum::String(b)) => {
                if a <= b {
                    true
                } else {
                    false
                }
            },
            (tokenizer::ValueEnum::Integer(a), tokenizer::ValueEnum::Integer(b)) => {
                if a <= b {
                    true
                } else {
                    false
                }
            },
            _ => unreachable!("SOMEHOW THIS SHOULDN'T BE PRINTED!")
        } 
    }
    else if operator == ">=" {
        match (&block_code[0][1].1, &block_code[0][3].1) {
            (tokenizer::ValueEnum::String(a), tokenizer::ValueEnum::String(b)) => {
                if a >= b {
                    true
                } else {
                    false
                }
            },
            (tokenizer::ValueEnum::Integer(a), tokenizer::ValueEnum::Integer(b)) => {
                if a >= b {
                    true
                } else {
                    false
                }
            },
            _ => unreachable!("SOMEHOW THIS SHOULDN'T BE PRINTED!")
        } 
    } else {
        unreachable!("SOMEHOW THIS SHOULDN'T BE PRINTED!");
    }
}

impl ExecData {
    pub fn exec(&mut self, token_collection: Vec<(String, tokenizer::ValueEnum)>) {
        let global_variables = &mut self.global_variables;
        let indentation = &mut self.indentation;
        let block_code = &mut self.block_code;
        let functions = &mut self.functions;
        let current_block_type = &mut self.current_block_type;
        
        let mut token_collection = token_collection.clone();
        println!("token_collection: {:?}", token_collection);
        
        // check for syntax errors
        if let Some((_, value)) = token_collection.iter().find(|(key, _)| key == &"ERROR_MESSAGE") {
            match value {
                tokenizer::ValueEnum::String(v) => {
                    println!("SYNTAX ERROR: {}", v);
                    return;
                },
                _ => unreachable!("SOMEHOW THIS SHOULDN'T BE PRINTED!")
            }
        }
        // check for comments -> just make a newline
        if let Some((_, value)) = token_collection.iter().find(|(key, _)| key == &"COMMENT") {
            match value {
                tokenizer::ValueEnum::String(_v) => {
                    println!("");
                    return;
                },
                _ => unreachable!("SOMEHOW THIS SHOULDN'T BE PRINTED!") 
            }
        }
        // check for reset
        match &token_collection[0].1 {
            tokenizer::ValueEnum::String(v) => {
                if v == "RESET" && token_collection.len() == 1 {
                    *global_variables = HashMap::new();
                    return;
                }
            },
            _ => unreachable!("SOMEHOW THIS SHOULDN'T BE PRINTED!")
        }
        // check for stop 
        match &token_collection[0].1 {
            tokenizer::ValueEnum::String(v) => {
                if v == "STOP" && token_collection.len() == 1 {
                    process::exit(1);
                }
            },
            _ => unreachable!("SOMEHOW THIS SHOULDN'T BE PRINTED!")
        }

        // check for code block stuff
        if indentation.to_string() != "".to_string() {
            match &token_collection[0].1 {
                tokenizer::ValueEnum::String(v) => {
                    if v == "IF" || v == "WHILE" {
                        add_indentation(indentation);
                    }
                    else if v == "FN" {
                        if current_block_type.0 != "normal" {
                            add_indentation(indentation);
                        } else {
                            println!("FUNCTIONS CAN'T BE INSIDE OF OTHER CODE BLOCKS!");
                            return;
                        }
                    }
                    else if v == "END" && token_collection.len() == 1 {
                        subtract_indentation(indentation);
                        if indentation.to_string() == "".to_string() {
                            if current_block_type.0 == "normal" {
                                match &block_code[0][0].1 {
                                    tokenizer::ValueEnum::String(first_predefined_name) => {
                                        if first_predefined_name == "IF" {
                                            match &block_code[0][2].1 {
                                                tokenizer::ValueEnum::String(operator) => {
                                                    let mut if_part: Vec<Vec<(String, tokenizer::ValueEnum)>> = vec![]; 
                                                    let mut else_part: Vec<Vec<(String, tokenizer::ValueEnum)>> = vec![]; 
                                                    let mut is_there_else_block = false;
                                                    for (line_position, line) in block_code.iter().enumerate() {
                                                        match &line[0].1 {
                                                            tokenizer::ValueEnum::String(first_token) => {
                                                                if first_token == "ELSE" {
                                                                    if line.len() == 1 {
                                                                        if_part = block_code[..line_position].to_vec();
                                                                        else_part = block_code[line_position+1..].to_vec();
                                                                        is_there_else_block = true;
                                                                    } else {
                                                                        println!("EXECUTION ERROR: THERE ARE MORE TOKENS THAN ELSE NEEDS!");
                                                                        return;
                                                                    }
                                                                }
                                                            },
                                                            _ => unreachable!("SOMEHOW THIS SHOULDN'T BE PRINTED")
                                                        }
                                                    }
                                                    if !(is_there_else_block) {
                                                        if_part = block_code.to_vec();
                                                    }
                                                    
                                                    if check_block_code_condition(operator.to_string(), block_code.to_vec()) {
                                                        execute_block_code(if_part[1..].to_vec());
                                                    } else if is_there_else_block {
                                                        execute_block_code(else_part);
                                                    }
                                                },
                                                _ => unreachable!("SOMEHOW THIS SHOULDN'T BE PRINTED!")
                                            }
                                        }
                                        else if first_predefined_name == "WHILE" {
                                            let mut error = "".to_string();

                                            match &block_code[0][2].1 {
                                                tokenizer::ValueEnum::String(operator) => {
                                                    while {
                                                        let new_block_code = update_while_condition_values(&block_code, &global_variables, &mut error);
                                                        if error != "".to_string() {
                                                            println!("{}", error);
                                                            return;
                                                        }
                                                        check_block_code_condition(operator.to_string(), new_block_code)
                                                    } {
                                                        execute_block_code(block_code[1..].to_vec());
                                                    }
                                                },
                                                _ => unreachable!("SOMEHOW THIS SHOULDN'T BE PRINTED!")
                                            }
                                        }
                                    },
                                    _ => unreachable!("SOMEHOW THIS SHOULDN'T BE PRINTED!")
                                }
 
                                current_block_type.0 = "".to_string();
                                *block_code = Vec::new();
                            }
                            else if current_block_type.0 == "function" {
                                current_block_type.0 = "".to_string();
                                current_block_type.1 = "".to_string();
                            }
                        }
                    }
                    
                    // saving code block stuff
                    if current_block_type.0 == "normal" {
                        if v == "FN" {
                            println!("EXECUTION ERROR: FUNCTIONS CAN'T BE INSIDE OF OTHER CODE BLOCKS!");
                            return;
                        }
                        block_code.push(token_collection.clone());
                    }
                    else if current_block_type.0 == "function" {
                        if v == "FN" {
                            println!("EXECUTION ERROR: FUNCTIONS CAN'T BE INSIDE OF OTHER CODE BLOCKS!");
                            return;
                        }
                        functions.get_mut(&current_block_type.1).unwrap().push(token_collection.clone());
                    }
                    
                },
                _ => unreachable!("SOMEHOW THIS SHOULDN'T BE PRINTED!")
            }

            return;
        }

        // order of predefined names for checking and if the value is set the value
        let predefined_name_order = {
            let mut hashmap = HashMap::new();

            hashmap.insert("LET", OrderEnum::MultipleOptions(
            vec![
                vec![
                    "PREDEFINED_NAME:LET", 
                    "VARIABLE/FUNCTION_NAME:?", 
                    "EQUAL_SIGN:=", 
                    "STRING:?"
                ], 
                vec![
                    "PREDEFINED_NAME:LET", 
                    "VARIABLE/FUNCTION_NAME:?", 
                    "EQUAL_SIGN:=", 
                    "INTEGER:?"
                ],
                vec![
                    "PREDEFINED_NAME:LET", 
                    "VARIABLE/FUNCTION_NAME:?", 
                    "EQUAL_SIGN:=", 
                    "STRING_ARRAY:?"
                ],
                vec![
                    "PREDEFINED_NAME:LET", 
                    "VARIABLE/FUNCTION_NAME:?", 
                    "EQUAL_SIGN:=", 
                    "INTEGER_ARRAY:?"
                ]
            ]));

            hashmap.insert("PRINT", OrderEnum::MultipleOptions(
            vec![
                vec![
                    "PREDEFINED_NAME:PRINT", 
                    "STRING:?"
                ], 
                vec![
                    "PREDEFINED_NAME:PRINT", 
                    "INTEGER:?"
                ] 
            ]));

            hashmap.insert("FN", OrderEnum::SingleOption(
            vec![
                "PREDEFINED_NAME:FN", 
                "VARIABLE/FUNCTION_NAME:?", 
                "PREDEFINED_NAME:START"
            ]));

            hashmap.insert("DO", OrderEnum::SingleOption(
            vec![
                "PREDEFINED_NAME:DO", 
                "VARIABLE/FUNCTION_NAME:?", 
            ]));

            hashmap.insert("IF", OrderEnum::MultipleOptions(
            vec![
                vec![
                    "PREDEFINED_NAME:IF",
                    "STRING:?",
                    "COMPARING_OPERATOR:?",
                    "STRING:?",
                    "PREDEFINED_NAME:START"
                ],
                vec![
                    "PREDEFINED_NAME:IF",
                    "INTEGER:?",
                    "COMPARING_OPERATOR:?",
                    "INTEGER:?",
                    "PREDEFINED_NAME:START"
                ]
            ]));

            hashmap.insert("WHILE", OrderEnum::MultipleOptions(
            vec![
                vec![
                    "PREDEFINED_NAME:WHILE",
                    "STRING:?",
                    "COMPARING_OPERATOR:?",
                    "STRING:?",
                    "PREDEFINED_NAME:START"
                ],
                vec![
                    "PREDEFINED_NAME:WHILE",
                    "INTEGER:?",
                    "COMPARING_OPERATOR:?",
                    "INTEGER:?",
                    "PREDEFINED_NAME:START"
                ]
            ]));

            hashmap.insert("PUSH", OrderEnum::MultipleOptions(
            vec![
                vec![
                    "PREDEFINED_NAME:PUSH",
                    "STRING:?",
                    "PREDEFINED_NAME:ONTO",
                    "VARIABLE/FUNCTION_NAME:?"
                ],
                vec![
                    "PREDEFINED_NAME:PUSH",
                    "INTEGER:?",
                    "PREDEFINED_NAME:ONTO",
                    "VARIABLE/FUNCTION_NAME:?"
                ]
            ]));

            hashmap.insert("POP", OrderEnum::SingleOption(
            vec![
                "PREDEFINED_NAME:POP",
                "PREDEFINED_NAME:FROM",
                "VARIABLE/FUNCTION_NAME:?"
            ]));

            hashmap.insert("INSERT", OrderEnum::MultipleOptions(
            vec![
                vec![
                    "PREDEFINED_NAME:INSERT",
                    "STRING:?",
                    "PREDEFINED_NAME:INTO",
                    "VARIABLE/FUNCTION_NAME:?",
                    "PREDEFINED_NAME:AT",
                    "INTEGER:?"
                ],
                vec![
                    "PREDEFINED_NAME:INSERT",
                    "INTEGER:?",
                    "PREDEFINED_NAME:INTO",
                    "VARIABLE/FUNCTION_NAME:?",
                    "PREDEFINED_NAME:AT",
                    "INTEGER:?"
                ]
            ]));

            hashmap.insert("REMOVE", OrderEnum::SingleOption(
            vec![
                "PREDEFINED_NAME:REMOVE",
                "PREDEFINED_NAME:FROM",
                "VARIABLE/FUNCTION_NAME:?",
                "PREDEFINED_NAME:AT",
                "INTEGER:?"
            ]));

            hashmap.insert("GET", OrderEnum::MultipleOptions(
            vec![
                vec![
                    "PREDEFINED_NAME:GET",
                    "PREDEFINED_NAME:FROM",
                    "VARIABLE/FUNCTION_NAME:?",
                    "PREDEFINED_NAME:AT",
                    "INTEGER:?"
                ],
                vec![ // LEN
                    "PREDEFINED_NAME:GET",
                    "PREDEFINED_NAME:FROM",
                    "VARIABLE/FUNCTION_NAME:?",
                    "PREDEFINED_NAME:LEN"
                ]
            ]));

            hashmap
        };
        

        // *check order of keys and values* //
        
        let first_key_element = &token_collection[0].0;

        if first_key_element != "PREDEFINED_NAME" {
            println!("EXECUTION ERROR: EVERY LINE HAS TO START WITH A PREDEFINED NAME (EXCEPT FOR COMMENT-LINES) !");
            return;
        } 

        // evaluate value for LET, PRINT, IF, PUSH, INSERT
        if token_collection.len() > 0 {
            let mut token_collection_clone = token_collection.clone();
            match &token_collection[0].1 {
                tokenizer::ValueEnum::String(clean) => {
                    match predefined_name_order.get(&clean.as_str()) {
                        Some(value) => {
                            match value {
                                OrderEnum::MultipleOptions(_v) => {
                                    match &token_collection[0].1 {
                                        tokenizer::ValueEnum::String(fv) => {
                                            if fv == "LET" {
                                                if token_collection[3].0 == "VARIABLE/FUNCTION_NAME" {
                                                    match &token_collection[3].1 {
                                                        tokenizer::ValueEnum::String(variable_name) => {
                                                            match global_variables.get(variable_name) {
                                                                Some(value_of_variable) => { 
                                                                    match value_of_variable {
                                                                        tokenizer::ValueEnum::String(v) => {
                                                                            token_collection_clone[3].0 = "STRING".to_string();
                                                                            token_collection_clone[3].1 = tokenizer::ValueEnum::String(v.to_string()); 
                                                                        },
                                                                        tokenizer::ValueEnum::Integer(v) => {
                                                                            token_collection_clone[3].0 = "INTEGER".to_string();
                                                                            token_collection_clone[3].1 = tokenizer::ValueEnum::Integer(*v);   
                                                                        },
                                                                        tokenizer::ValueEnum::IntegerArray(v) => {
                                                                            token_collection_clone[3].0 = "INTEGER_ARRAY".to_string();
                                                                            token_collection_clone[3].1 = tokenizer::ValueEnum::IntegerArray(v.to_vec()); 
                                                                        },
                                                                        tokenizer::ValueEnum::StringArray(v) => {
                                                                            token_collection_clone[3].0 = "STRING_ARRAY".to_string();
                                                                            token_collection_clone[3].1 = tokenizer::ValueEnum::StringArray(v.to_vec());
                                                                        }
                                                                    }
                                                                },
                                                                None => {
                                                                    println!("EXECUTION ERROR: THERE IS NO VARIABLE CALLED {}", variable_name);
                                                                    return;
                                                                }
                                                            }
                                                        },
                                                        _ => unreachable!("SOMEHOW THIS SHOULDN'T BE PRINTED!")
                                                    }
                                                }
                                            }
                                            if fv == "PRINT" {
                                                if token_collection[1].0 == "VARIABLE/FUNCTION_NAME" {
                                                    match &token_collection[1].1 {
                                                        tokenizer::ValueEnum::String(variable_name) => {
                                                            match global_variables.get(variable_name) {
                                                                Some(value_of_variable) => { 
                                                                    match value_of_variable {
                                                                        tokenizer::ValueEnum::String(v) => {
                                                                            token_collection_clone[1].0 = "STRING".to_string();
                                                                            token_collection_clone[1].1 = tokenizer::ValueEnum::String(v.to_string()); 
                                                                        },
                                                                        tokenizer::ValueEnum::Integer(v) => {
                                                                            token_collection_clone[1].0 = "INTEGER".to_string();
                                                                            token_collection_clone[1].1 = tokenizer::ValueEnum::Integer(*v);   
                                                                        },
                                                                        _ => {
                                                                            println!("EXECUTION ERROR: CAN'T PRINT THIS VARIABLE!");
                                                                            return;
                                                                        }
                                                                    }
                                                                },
                                                                None => {
                                                                    println!("EXECUTION ERROR: THERE IS NO VARIABLE CALLED {}", variable_name);
                                                                    return;
                                                                }
                                                            }
                                                        },
                                                        _ => unreachable!("SOMEHOW THIS SHOULDN'T BE PRINTED!")
                                                    }
                                                }
                                            }
                                            else if fv == "IF" {
                                                if token_collection[1].0 == "VARIABLE/FUNCTION_NAME" {
                                                    match &token_collection[1].1 {
                                                        tokenizer::ValueEnum::String(variable_name) => {
                                                            match global_variables.get(variable_name) {
                                                                Some(value_of_variable) => { 
                                                                    match value_of_variable {
                                                                        tokenizer::ValueEnum::String(v) => {
                                                                            token_collection_clone[1].0 = "STRING".to_string();
                                                                            token_collection_clone[1].1 = tokenizer::ValueEnum::String(v.to_string()); 
                                                                        },
                                                                        tokenizer::ValueEnum::Integer(v) => {
                                                                            token_collection_clone[1].0 = "INTEGER".to_string();
                                                                            token_collection_clone[1].1 = tokenizer::ValueEnum::Integer(*v);   
                                                                        },
                                                                        _ => {
                                                                            println!("EXECUTION ERROR: FIRST VARIABLE HAS TO BE A STRING OR INTEGER!");
                                                                            return;
                                                                        }
                                                                    }
                                                                },
                                                                None => {
                                                                    println!("EXECUTION ERROR: THERE IS NO VARIABLE CALLED {}", variable_name);
                                                                    return;
                                                                }
                                                            }
                                                        },
                                                        _ => unreachable!("SOMEHOW THIS SHOULDN'T BE PRINTED!")
                                                    }
                                                }
                                                if token_collection[3].0 == "VARIABLE/FUNCTION_NAME" {
                                                    match &token_collection[3].1 {
                                                        tokenizer::ValueEnum::String(variable_name) => {
                                                            match global_variables.get(variable_name) {
                                                                Some(value_of_variable) => { 
                                                                    match value_of_variable {
                                                                        tokenizer::ValueEnum::String(v) => {
                                                                            token_collection_clone[3].0 = "STRING".to_string();
                                                                            token_collection_clone[3].1 = tokenizer::ValueEnum::String(v.to_string()); 
                                                                        },
                                                                        tokenizer::ValueEnum::Integer(v) => {
                                                                            token_collection_clone[3].0 = "INTEGER".to_string();
                                                                            token_collection_clone[3].1 = tokenizer::ValueEnum::Integer(*v);   
                                                                        },
                                                                        _ => {
                                                                            println!("EXECUTION ERROR: SECOND VARIABLE HAS TO BE A STRING OR INTEGER!");
                                                                            return;
                                                                        }
                                                                    }
                                                                },
                                                                None => {
                                                                    println!("EXECUTION ERROR: THERE IS NO VARIABLE CALLED {}", variable_name);
                                                                    return;
                                                                }
                                                            }
                                                        },
                                                        _ => unreachable!("SOMEHOW THIS SHOULDN'T BE PRINTED!")
                                                    }
                                                }

                                            }
                                            else if fv == "PUSH" {
                                                if token_collection[1].0 == "VARIABLE/FUNCTION_NAME" {
                                                    match &token_collection[1].1 {
                                                        tokenizer::ValueEnum::String(variable_name) => {
                                                            match global_variables.get(variable_name) {
                                                                Some(value_of_variable) => { 
                                                                    match value_of_variable {
                                                                        tokenizer::ValueEnum::String(v) => {
                                                                            token_collection_clone[1].0 = "STRING".to_string();
                                                                            token_collection_clone[1].1 = tokenizer::ValueEnum::String(v.to_string()); 
                                                                        },
                                                                        tokenizer::ValueEnum::Integer(v) => {
                                                                            token_collection_clone[1].0 = "INTEGER".to_string();
                                                                            token_collection_clone[1].1 = tokenizer::ValueEnum::Integer(*v);   
                                                                        },
                                                                        _ => {
                                                                            println!("EXECUTION ERROR: FIRST VARIABLE HAS TO BE A STRING OR INTEGER!");
                                                                            return;
                                                                        }
                                                                    }
                                                                },
                                                                None => {
                                                                    println!("EXECUTION ERROR: THERE IS NO VARIABLE CALLED {}", variable_name);
                                                                    return;
                                                                }
                                                            }
                                                        },
                                                        _ => unreachable!("SOMEHOW THIS SHOULDN'T BE PRINTED!")
                                                    }
                                                }
                                            }
                                            else if fv == "INSERT" {
                                                if token_collection[1].0 == "VARIABLE/FUNCTION_NAME" {
                                                    match &token_collection[1].1 {
                                                        tokenizer::ValueEnum::String(variable_name) => {
                                                            match global_variables.get(variable_name) {
                                                                Some(value_of_variable) => { 
                                                                    match value_of_variable {
                                                                        tokenizer::ValueEnum::String(v) => {
                                                                            token_collection_clone[1].0 = "STRING".to_string();
                                                                            token_collection_clone[1].1 = tokenizer::ValueEnum::String(v.to_string()); 
                                                                        },
                                                                        tokenizer::ValueEnum::Integer(v) => {
                                                                            token_collection_clone[1].0 = "INTEGER".to_string();
                                                                            token_collection_clone[1].1 = tokenizer::ValueEnum::Integer(*v);   
                                                                        },
                                                                        _ => {
                                                                            println!("EXECUTION ERROR: SECOND VARIABLE HAS TO BE A STRING OR INTEGER!");
                                                                            return;
                                                                        }
                                                                    }
                                                                },
                                                                None => {
                                                                    println!("EXECUTION ERROR: THERE IS NO VARIABLE CALLED {}", variable_name);
                                                                    return;
                                                                }
                                                            }
                                                        },
                                                        _ => unreachable!("SOMEHOW THIS SHOULDN'T BE PRINTED!")
                                                    }
                                                }    
                                            }
                                        },
                                        _ => ()
                                    }
                                },
                                _ => ()
                            }
                        },
                        None => {
                            println!("EXECUTION ERROR: '{}' IS NEVER AT THE BEGINNING!", clean);
                            return;
                        }
                    }
                },
                _ => ()
            }

            token_collection = token_collection_clone;
        }

        match &token_collection[0].1 {
            tokenizer::ValueEnum::String(clean) if clean == "WHILE" => (),
            tokenizer::ValueEnum::String(clean) => {
                match predefined_name_order.get(&clean.as_str()) {
                    Some(value) => {
                        // check if the key of the first token has multiple options 
                        match value {
                            OrderEnum::SingleOption(v) => {
                                // length check - otherwise the indexing would panic
                                if token_collection.len() < v.len() {
                                    println!("EXECUTION ERROR: THERE ARE LESS TOKENS THAN '{}' NEEDS!", clean);
                                    return;
                                }
                                if token_collection.len() > v.len() {
                                    println!("EXECUTION ERROR: THERE ARE MORE TOKENS THAN '{}' NEEDS!", clean);
                                    return;
                                }

                                // analyse if order of key and value is right
                                let mut is_key_order_right = true;
                                let mut is_value_order_right = true;

                                for element_nr in 0..v.len() {
                                    // check if key is right
                                    if token_collection[element_nr].0 != v[element_nr].split(':').nth(0).unwrap() {
                                        is_key_order_right= false;
                                        break;
                                    }
                                    // check if value is right
                                    match &token_collection[element_nr].1 {
                                        tokenizer::ValueEnum::String(tc) => {
                                            if tc != v[element_nr].split(':').nth(1).unwrap() && v[element_nr].split(':').nth(1).unwrap() != "?" {
                                                is_value_order_right = false;
                                                break; 
                                            }
                                        },
                                        _ => () 
                                    }
                                }
                                if !(is_key_order_right) {
                                    println!("EXECUTION ERROR: KEY ORDER FOR '{}' ISN'T RIGHT!", clean);
                                    print!("HELP: RIGHT ORDER: '");
                                    for e_nr in 0..v.len() {
                                        if e_nr == v.len() - 1 {
                                            println!("{}'", v[e_nr].split(':').nth(0).unwrap());
                                        } else {
                                            print!("{}, ", v[e_nr].split(':').nth(0).unwrap());
                                        }
                                    }
                                    return;
                                }
                                if !(is_value_order_right) {
                                    println!("EXECUTION ERROR: VALUE ORDER FOR '{}' ISN'T RIGHT!", clean);
                                    print!("HELP: RIGHT ORDER: '");
                                    for e_nr in 0..v.len() {
                                        if e_nr == v.len() - 1 {
                                            println!("{}'", v[e_nr].split(':').nth(1).unwrap());
                                        } else {
                                            println!("{}, ", v[e_nr].split(':').nth(1).unwrap());
                                        }
                                    }
                                    return;
                                }
                            },
                            OrderEnum::MultipleOptions(v) => {
                                // length check - otherwise the indexing would panic
                                let mut too_few_tokens = false;
                                let mut too_many_tokens = false;
                                for possibility_nr in 0..v.len() {
                                    if token_collection.len() < v[possibility_nr].len() {
                                        too_few_tokens = true;
                                    }
                                    if token_collection.len() > v[possibility_nr].len() {
                                        too_many_tokens = true;
                                    }
                                }
                                if too_few_tokens {
                                    println!("EXECUTION ERROR: THERE ARE LESS TOKENS THAN '{}' NEEDS!", clean);
                                    return;
                                }
                                if too_many_tokens {
                                    println!("EXECUTION ERROR: THERE ARE MORE TOKENS THAN '{}' NEEDS!", clean);
                                    return;
                                }
     
                                // analyse if order of key and value is right
                                let mut is_one_token_order_right = false;
                                let mut is_one_value_order_right = false;
                                // iterate trough possibilitys
                                for possibility_nr in 0..v.len() {
                                    let mut is_current_token_order_right = true;
                                    let mut is_current_value_order_right = true;

                                    for element_nr in 0..v[possibility_nr].len() {
                                        // check if key is right
                                        if token_collection[element_nr].0 != v[possibility_nr][element_nr].split(':').nth(0).unwrap() {
                                            is_current_token_order_right = false;
                                        }
                                        // check if value is right
                                        match &token_collection[element_nr].1 {
                                            tokenizer::ValueEnum::String(tc) => {
                                                if tc != v[possibility_nr][element_nr].split(':').nth(1).unwrap() && v[possibility_nr][element_nr].split(':').nth(1).unwrap() != "?"{
                                                    is_current_value_order_right = false;
                                                }
                                            },
                                            _ => ()
                                        }
                                    }
                                    if is_current_token_order_right {
                                        is_one_token_order_right = true;
                                    }
                                    if is_current_value_order_right {
                                        is_one_value_order_right = true;
                                    }
                                }
                                // check if just one order is right
                                if !(is_one_token_order_right) {
                                    println!("EXECUTION ERROR: KEY ORDER FOR '{}' ISN'T RIGHT!", clean);
                                    println!("HELP: RIGHT ORDER FOR POSSIBILITY NR. :");
                                    for p_nr in 0..v.len() {
                                        print!("{}: '", p_nr+1);
                                        for e_nr in 0..v[p_nr].len() {
                                            if e_nr == v[p_nr].len() - 1 {
                                                println!("{}'", v[p_nr][e_nr].split(':').nth(0).unwrap());
                                            } else {
                                                print!("{}, ", v[p_nr][e_nr].split(':').nth(0).unwrap());
                                            }
                                        }
                                    }
                                    return;
                                }
                                if !(is_one_value_order_right) {
                                    println!("EXECUTION ERROR: VALUE ORDER FOR '{}' ISN'T RIGHT!", clean);
                                    println!("HELP: RIGHT ORDER FOR POSSIBILITY NR. :");
                                    for p_nr in 0..v.len() {
                                        print!("{}: '", p_nr+1);
                                        for e_nr in 0..v[p_nr].len() {
                                            if e_nr == v[p_nr].len() - 1 {
                                                println!("{}'", v[p_nr][e_nr].split(':').nth(0).unwrap());
                                            } else {
                                                print!("{}, ", v[p_nr][e_nr].split(':').nth(0).unwrap());
                                            }
                                        }
                                    }
                                    return;
                                }
                            }
                        }
                    },
                    None => {
                        println!("EXECUTION ERROR: '{}' IS NEVER AT THE BEGINNING!", clean);
                        return;
                    }
                }
            },
            _ => unreachable!("SOMEHOW THIS SHOULDN'T BE PRINTED!")
        }

        // * real execution part * //
        
        match &token_collection[0].1 {
            tokenizer::ValueEnum::String(v) => {
                if v == &"LET".to_string() { // E.G. LET A = 123
                    let variable_name: String = {
                        match &token_collection[1].1 {
                            tokenizer::ValueEnum::String(current_v) => current_v.to_string(),
                            _ => unreachable!("SOMEHOW THIS SHOULDN'T BE PRINTED!")
                        }
                    };
                    if &token_collection[3].0 == &"STRING".to_string() {
                        global_variables.insert(variable_name, token_collection[3].1.clone());
                    }
                    else if &token_collection[3].0 == &"INTEGER".to_string() {
                        global_variables.insert(variable_name, token_collection[3].1.clone());
                    }
                    else if &token_collection[3].0 == &"VARIABLE/FUNCTION_NAME".to_string() {
                        global_variables.insert(variable_name, token_collection[3].1.clone());
                    }
                    else if &token_collection[3].0 == &"STRING_ARRAY".to_string() {
                        global_variables.insert(variable_name, token_collection[3].1.clone());
                    }
                    else if &token_collection[3].0 == &"INTEGER_ARRAY".to_string() {
                        global_variables.insert(variable_name, token_collection[3].1.clone());
                    }
                }
                else if v == &"PRINT".to_string() {
                    let stuff_to_print: String = {
                        match &token_collection[1].1 { 
                            tokenizer::ValueEnum::String(stuff) => {
                                if &token_collection[1].0 == &"STRING".to_string() {
                                    stuff.to_string()
                                }
                                else {
                                    match global_variables.get(stuff) {
                                        Some(value) => {
                                            match &value {
                                                tokenizer::ValueEnum::String(final_value) => final_value.to_string(),
                                                tokenizer::ValueEnum::Integer(final_value) => final_value.to_string(),
                                                _ => unreachable!("SOMEHOW THIS SHOULDN'T BE PRINTED!") 
                                            }
                                        }
                                        None => {
                                            println!("EXECUTION ERROR: THERE IS NO VARIABLE CALLED {}", stuff);
                                            return;
                                        }
                                    }
                                }
                            },
                            tokenizer::ValueEnum::Integer(stuff) => stuff.to_string(),
                            _ => unreachable!("SOMEHOW THIS SHOULDN'T BE PRINTED!")
                        }
                    };

                    println!("{}", stuff_to_print);
                }
                else if v == &"FN".to_string() {
                    match &token_collection[1].1 {
                        tokenizer::ValueEnum::String(fn_name) => {
                            functions.insert(fn_name.to_string(), vec![token_collection.clone()]);
                            current_block_type.0 = "function".to_string();
                            current_block_type.1 = fn_name.to_string();
                        },
                        _ => unreachable!("SOMEHOW THIS SHOULDN'T BE PRINTED!")
                    }
                    add_indentation(indentation);
                }
                else if v == &"DO".to_string() {
                    match &token_collection[1].1 {
                        tokenizer::ValueEnum::String(function_name) => {
                            match functions.get(function_name) {
                                Some(function_code_block) => {
                                    println!("function {} would get now executed", function_name);
                                    println!("execution starts now");
                                    execute_block_code(function_code_block[1..].to_vec()); 
                                    println!("execution ends now");
                                },
                                None => {
                                    println!("EXECUTION ERROR: THERE IS NO FUNCTION CALLED {}", function_name);
                                    return;
                                }
                            }
                        },
                        _ => unreachable!("SOMEHOW THIS SHOULDN'T BE PRINTED!")
                    }
                }
                else if v == &"IF".to_string() {
                    block_code.push(token_collection);
                    current_block_type.0 = "normal".to_string();
                    add_indentation(indentation);

                }
                else if v == &"WHILE".to_string() {
                    block_code.push(token_collection);
                    current_block_type.0 = "normal".to_string();
                    add_indentation(indentation);
                }
                else if v == &"PUSH".to_string() {
                    match &token_collection[3].1 {
                        tokenizer::ValueEnum::String(stuff) => {
                            match global_variables.get(stuff) {
                                Some(value) => {
                                    match &value {
                                        tokenizer::ValueEnum::IntegerArray(i_vec) => {
                                            match &token_collection[1].1 {
                                                tokenizer::ValueEnum::Integer(stuff_to_push) => {
                                                    let mut i_vec_clone = i_vec.clone();
                                                    i_vec_clone.push(*stuff_to_push);
                                                    *global_variables.get_mut(stuff).unwrap() = tokenizer::ValueEnum::IntegerArray(i_vec_clone);
                                                }
                                                _ => {
                                                    println!("EXECUTION ERROR: YOU HAVE TO PUSH AN INTEGER ONTO A INTEGER ARRAY!");
                                                    return;
                                                }
                                            }
                                        },
                                        tokenizer::ValueEnum::StringArray(s_vec) => {
                                            match &token_collection[1].1 {
                                                tokenizer::ValueEnum::String(stuff_to_push) => {
                                                    let mut s_vec_clone = s_vec.clone();
                                                    s_vec_clone.push(stuff_to_push.to_string());
                                                    *global_variables.get_mut(stuff).unwrap() = tokenizer::ValueEnum::StringArray(s_vec_clone);
                                                }
                                                _ => {
                                                    println!("EXECUTION ERROR: YOU HAVE TO PUSH AN STRING ONTO A STRING ARRAY!");
                                                    return;
                                                }
                                            }
                                        },
                                        _ => {
                                            println!("EXECUTION  ERROR: YOU CAN ONLY PUSH ONTO ARRAYS!");
                                            return;
                                        }

                                    }
                                },
                                None => {
                                    println!("EXECUTION ERROR: THERE IS NO VARIABLE CALLED {}", stuff);
                                    return;
                                }
                            }
                        },
                        _ => unreachable!("SOMEHOW THIS SHOULDN'T BE PRINTED!")
                    }
                }
                else if v == &"POP".to_string() {
                    match &token_collection[2].1 {
                        tokenizer::ValueEnum::String(stuff) => {
                            match global_variables.get(stuff) {
                                Some(value) => {
                                    match &value {
                                        tokenizer::ValueEnum::IntegerArray(i_vec) => {
                                            let mut i_vec_clone = i_vec.clone();
                                            i_vec_clone.pop();
                                            *global_variables.get_mut(stuff).unwrap() = tokenizer::ValueEnum::IntegerArray(i_vec_clone);
                                        },
                                        tokenizer::ValueEnum::StringArray(s_vec) => {
                                            let mut s_vec_clone = s_vec.clone();
                                            s_vec_clone.pop();
                                            *global_variables.get_mut(stuff).unwrap() = tokenizer::ValueEnum::StringArray(s_vec_clone);
                                        },
                                        _ => {
                                            println!("EXECUTION  ERROR: YOU CAN ONLY POP FROM ARRAYS!");
                                            return;
                                        }
                                    }
                                },
                                None => {
                                    println!("EXECUTION ERROR: THERE IS NO VARIABLE CALLED {}", stuff);
                                    return;
                                }
                            }
                        },
                        _ => unreachable!("SOMEHOW THIS SHOULDN'T BE PRINTED!")
                    }
                }
                else if v == &"INSERT".to_string() {
                    match &token_collection[3].1 {
                        tokenizer::ValueEnum::String(stuff) => {
                            match global_variables.get(stuff) {
                                Some(value) => {
                                    match &value {
                                        tokenizer::ValueEnum::IntegerArray(i_vec) => {
                                            match &token_collection[1].1 {
                                                tokenizer::ValueEnum::Integer(stuff_to_push) => {
                                                    match &token_collection[5].1 {
                                                        tokenizer::ValueEnum::Integer(index_where_to_insert) => {
                                                            let mut i_vec_clone = i_vec.clone();
                                                            i_vec_clone.insert(*index_where_to_insert as usize, *stuff_to_push);
                                                            *global_variables.get_mut(stuff).unwrap() = tokenizer::ValueEnum::IntegerArray(i_vec_clone);
                                                        },
                                                        _ => unreachable!("SOMEHOW THIS SHOUDLN'T BE PRINTED!")
                                                    }
                                                },
                                                _ => {
                                                    println!("EXECUTION ERROR: YOU HAVE TO INSERT AN INTEGER INTO A INTEGER ARRAY!");
                                                    return;
                                                }
                                            }
                                        },
                                        tokenizer::ValueEnum::StringArray(s_vec) => {
                                            match &token_collection[1].1 {
                                                tokenizer::ValueEnum::String(stuff_to_push) => {
                                                    match &token_collection[5].1 {
                                                        tokenizer::ValueEnum::Integer(index_where_to_insert) => {
                                                            let mut s_vec_clone = s_vec.clone();
                                                            s_vec_clone.insert(*index_where_to_insert as usize, stuff_to_push.to_string());
                                                            *global_variables.get_mut(stuff).unwrap() = tokenizer::ValueEnum::StringArray(s_vec_clone);
                                                        },
                                                        _ => unreachable!("SOMEHOW THIS SHOUDLN'T BE PRINTED!")
                                                    }
                                                },
                                                _ => {
                                                    println!("EXECUTION ERROR: YOU HAVE TO INSERT AN STRING INTO A STRING ARRAY!");
                                                    return;
                                                }
                                            }
                                        },
                                        _ => {
                                            println!("EXECUTION ERROR: YOU CAN ONLY INSERT INTO ARRAYS!");
                                            return;
                                        }
                                    }
                                },
                                None => {
                                    println!("EXECUTION ERROR: THERE IS NO VARIABLE CALLED {}", stuff);
                                    return;
                                }
                            }
                        },
                        _ => unreachable!("SOMEHOW THIS SHOULDN'T BE PRINTED!")
                    }

                }
                else if v == &"REMOVE".to_string() {
                    match &token_collection[2].1 {
                        tokenizer::ValueEnum::String(stuff) => {
                            match global_variables.get(stuff) {
                                Some(value) => {
                                    match &value {
                                        tokenizer::ValueEnum::IntegerArray(i_vec) => {
                                            match &token_collection[4].1 {
                                                tokenizer::ValueEnum::Integer(index_where_to_remove) => {
                                                    let mut i_vec_clone = i_vec.clone();
                                                    i_vec_clone.remove(*index_where_to_remove as usize);
                                                    *global_variables.get_mut(stuff).unwrap() = tokenizer::ValueEnum::IntegerArray(i_vec_clone);
                                                },
                                                _ => unreachable!("SOMEHOW THIS SHOULDN'T BE PRINTED!")
                                            }
                                        },
                                        tokenizer::ValueEnum::StringArray(s_vec) => {
                                            match &token_collection[4].1 {
                                                tokenizer::ValueEnum::Integer(index_where_to_remove) => {
                                                    let mut s_vec_clone = s_vec.clone();
                                                    s_vec_clone.remove(*index_where_to_remove as usize);
                                                    *global_variables.get_mut(stuff).unwrap() = tokenizer::ValueEnum::StringArray(s_vec_clone);
                                                },
                                                _ => unreachable!("SOMEHOW THIS SHOULDN'T BE PRINTED!") 
                                            }
                                        },
                                        _ => unreachable!("SOMEHOW THIS SHOULDN'T BE PRINTED!")
                                    }
                                },
                                None => {
                                    println!("EXECUTION ERROR: THERE IS NO VARIABLE CALLED {}", stuff);
                                    return;
                                }
                            }
                        },
                        _ => unreachable!("SOMEHOW THIS SHOULDN'T BE PRINTED!")
                    }

                }
                else if v == &"GET".to_string() {
                    //
                }
            }, 
            _ => unreachable!("SOMEHOW THIS SHOULDN'T BE PRINTED!")
        }

        println!("global_variables: {:?}", global_variables);
    }    
}

fn update_while_condition_values(
    block_code: &Vec<Vec<(String, tokenizer::ValueEnum)>>,
    global_variables: &HashMap<String, tokenizer::ValueEnum>,
    error: &mut String,
) -> Vec<Vec<(String, tokenizer::ValueEnum)>> {
    let mut block_code_clone = block_code.clone();

    if block_code[0][1].0 == "VARIABLE/FUNCTION_NAME" {
        match &block_code[0][1].1 {
            tokenizer::ValueEnum::String(variable_name) => {
                match global_variables.get(variable_name) {
                    Some(value_of_variable) => {
                        match value_of_variable {
                            tokenizer::ValueEnum::String(v) => {
                                block_code_clone[0][1].0 = "STRING".to_string();
                                block_code_clone[0][1].1 = tokenizer::ValueEnum::String(v.to_string());
                            },
                            tokenizer::ValueEnum::Integer(v) => {
                                block_code_clone[0][1].0 = "INTEGER".to_string();
                                block_code_clone[0][1].1 = tokenizer::ValueEnum::Integer(*v);
                            },
                            _ => {
                                *error = "EXECUTION ERROR: FIRST VARIABLE HAS TO BE A STRING OR INTEGER!".to_string();
                            }
                        }
                    },
                    None => {
                        *error = "EXECUTION ERROR: THERE IS NO VARIABLE CALLED ".to_string() + variable_name;
                    }
                }
            },
            _ => unreachable!("SOMEHOW THIS SHOULDN'T BE PRINTED!")
        }
    }
    if block_code[0][3].0 == "VARIABLE/FUNCTION_NAME" {
        match &block_code[0][3].1 {
            tokenizer::ValueEnum::String(variable_name) => {
                match global_variables.get(variable_name) {
                    Some(value_of_variable) => {
                        match value_of_variable {
                            tokenizer::ValueEnum::String(v) => {
                                block_code_clone[0][3].0 = "STRING".to_string();
                                block_code_clone[0][3].1 = tokenizer::ValueEnum::String(v.to_string());
                            },
                            tokenizer::ValueEnum::Integer(v) => {
                                block_code_clone[0][3].0 = "INTEGER".to_string();
                                block_code_clone[0][3].1 = tokenizer::ValueEnum::Integer(*v);
                            },
                            _ => {
                                *error = "EXECUTION ERROR: SECOND VARIABLE HAS TO BE A STRING OR INTEGER!".to_string();
                            }
                        }
                    },
                    None => {
                        *error = "EXECUTION ERROR: THERE IS NO VARIABLE CALLED ".to_string() + variable_name;
                    }
                }
            },
            _ => unreachable!("SOMEHOW THIS SHOULDN'T BE PRINTED!")
        }
    }

    block_code_clone
}