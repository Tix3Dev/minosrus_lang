pub mod tokenizer;

use std::collections::HashMap;

enum OrderEnum {
    SingleOption(Vec<&'static str>),
    MultipleOptions(Vec<Vec<&'static str>>)
}

pub fn exec(input: String, global_variables: &mut HashMap<String, tokenizer::ValueEnum>) {
    // tokenize the input
    let token_collection = tokenizer::make_tokens(input);
    println!("{:?}", token_collection);
    
    // check for syntax errors
    if let Some((_, value)) = token_collection.iter().find(|(key, _)| key == &"ERROR_MESSAGE") {
        match value {
            tokenizer::ValueEnum::String(v) => {
                println!("SYNTAX ERROR: {}", v);
                return;
            },
            _ => ()
        }
    }
    // check for comments -> just make a newline
    if let Some((_, value)) = token_collection.iter().find(|(key, _)| key == &"COMMENT") {
        match value {
            tokenizer::ValueEnum::String(_v) => {
                println!("");
                return;
            },
            _ => ()
        }
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
                "VARIABLE/FUNCTION_NAME:?"
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
            ],
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
            ], 
            vec![
                "PREFEINDED_NAME:PRINT", 
                "VARIABLE/FUNCTION_NAME:?"
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
            ],
            vec![
                "PREDEFINED_NAME:IF",
                "VARIABLE/FUNCTION_NAME:?",
                "COMPARING_OPERATOR:?",
                "VARIABLE/FUNCTION_NAME:?",
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
            ],
            vec![
                "PREDEFINED_NAME:WHILE",
                "VARIABLE/FUNCTION_NAME:?",
                "COMPARING_OPERATOR:?",
                "VARIABLE/FUNCTION_NAME:?",
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
            ],
            vec![
                "PREDEFINED_NAME:PUSH",
                "VARIABLE/FUNCTION_NAME:?",
                "PREDEFINED_NAME:ONTO",
                "VARIABLE/FUNCTION_NAME:?"
            ]
        ]));

        hashmap.insert("POP", OrderEnum::SingleOption (
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
            ],
            vec![
                "PREDEFINED_NAME:INSERT",
                "VARIABLE/FUNCTION_NAME:?",
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
    let first_value_element = &token_collection[0].1; 

    if first_key_element != "PREDEFINED_NAME" {
        println!("EXECUTION ERROR: EVERY LINE HAS TO START WITH A PREDEFINED NAME (EXCEPT FOR COMMENT-LINES) !");
        return;
    }

    match first_value_element {
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
        _ => ()
    }

    // * real execution part * //
    
    match first_value_element {
        tokenizer::ValueEnum::String(v) => {
            if v == &"LET".to_string() { // E.G. LET A = 123
                let variable_name: String = {
                    match &token_collection[1].1 {
                        tokenizer::ValueEnum::String(current_v) => current_v.to_string(),
                        _ => unreachable!("SOMEHOW THIS SHOULDN'T BE PRINTED")
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
                let mut was_there_an_error = false;
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
                                            _ => unreachable!("SOMEHOW THIS SHOULDN'T BE PRINTED") 
                                        }
                                    }
                                    None => {
                                        println!("EXECUTION ERROR: THERE IS NO VARIABLE CALLED {}", stuff);
                                        was_there_an_error = true;
                                        stuff.to_string()
                                    }
                                }
                            }
                        },
                        tokenizer::ValueEnum::Integer(stuff) => stuff.to_string(),
                        _ => unreachable!("SOMEHOW THIS SHOULDN'T BE PRINTED")
                    }
                };
                if was_there_an_error {
                    println!("EXECUTION ERROR: THERE IS NO VARIABLE CALLED {}", stuff_to_print);
                    return;
                }

                println!("{}", stuff_to_print);
            }
            else if v == &"FN".to_string() {
                //
            }
            else if v == &"DO".to_string() {
                //
            }
            else if v == &"IF".to_string() {
                //
            }
            else if v == &"WHILE".to_string() {
                //
            }
            else if v == &"PUSH".to_string() {
                //
            }
            else if v == &"POP".to_string() {
                //
            }
            else if v == &"INSERT".to_string() {
                //
            }
            else if v == &"REMOVE".to_string() {
                //
            }
            else if v == &"GET".to_string() {
                //
            }
        }, 
        _ => ()
    }

    println!("{:?}", global_variables);
}    

pub fn reset() {
    //
}

pub fn stop() {
    //
}
