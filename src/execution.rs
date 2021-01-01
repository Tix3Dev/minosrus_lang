pub mod tokenizer;

use std::collections::HashMap;

enum OrderEnum {
    SingleOption(Vec<&'static str>),
    MultipleOptions(Vec<Vec<&'static str>>)
}

pub fn exec(input: String) {
    // tokenize the input
    let token_collection = tokenizer::make_tokens(input);
    println!("{:?}", token_collection);

    // check for error
    if let Some((_, value)) = token_collection.iter().find(|(key, _)| key == &"ERROR_MESSAGE") {
        match value {
            tokenizer::ValueEnum::String(v) => {
                println!("SYNTAX ERROR: {}", v);
                return;
            },
            tokenizer::ValueEnum::IntegerVector(_v) => (),
            tokenizer::ValueEnum::StringVector(_v) => (),
            tokenizer::ValueEnum::TokenVector(_v) => ()
        }
    }
    // check for comments -> just make a newline
    if let Some((_, value)) = token_collection.iter().find(|(key, _)| key == &"COMMENT") {
        match value {
            tokenizer::ValueEnum::String(_v) => {
                println!("");
                return;
            },
            tokenizer::ValueEnum::IntegerVector(_v) => (),
            tokenizer::ValueEnum::StringVector(_v) => (),
            tokenizer::ValueEnum::TokenVector(_v) => ()
        }
    }
    
    // order of predefined names for checking
    let predefined_name_order = {
        let mut hashmap = HashMap::new();

        hashmap.insert("LET", OrderEnum::MultipleOptions(
        vec![
            vec![
                "PREDEFINED_NAME", 
                "VARIABLE/FUNCTION_NAME", 
                "OPERATOR", 
                "STRING"
            ], 
            vec![
                "PREDEFINED_NAME", 
                "VARIABLE/FUNCTION_NAME", 
                "OPERATOR", 
                "INTEGER"
            ],
            vec![
                "PREDEFINED_NAME", 
                "VARIABLE/FUNCTION_NAME", 
                "OPERATOR", 
                "VARIABLE/FUNCTION_NAME"
            ],

        ]));

        hashmap.insert("PRINT", OrderEnum::MultipleOptions(
        vec![
            vec![
                "PREDEFINED_NAME", 
                "STRING"
            ], 
            vec![
                "PREDEFINED_NAME", 
                "INTEGER"
            ], 
            vec![
                "PREFEINDED_NAME", 
                "VARIABLE/FUNCTION_NAME"
            ]
        ]));

        hashmap.insert("FN", OrderEnum::SingleOption(
        vec![
            "PREDEFINED_NAME", 
            "VARIABLE/FUNCTION_NAME", 
            "PREDEFINED_NAME"
        ]));

        hashmap.insert("DO", OrderEnum::SingleOption(
        vec![
            "PREDEFINED_NAME", 
            "VARIABLE/FUNCTION_NAME", 
        ]));

        hashmap.insert("IF", OrderEnum::MultipleOptions(
        vec![
            vec![
                "PREDEFINED_NAME",
                "STRING",
                "OPERATOR",
                "STRING",
                "PREDEFINED_NAME"
            ],
            vec![
                "PREDEFINED_NAME",
                "INTEGER",
                "OPERATOR",
                "INTEGER",
                "PREDEFINED_NAME"
            ],
            vec![
                "PREDEFINED_NAME",
                "VARIABLE/FUNCTION_NAME",
                "OPERATOR",
                "VARIABLE/FUNCTION_NAME",
                "PREDEFINED_NAME"
            ]
        ]));

        hashmap.insert("WHILE", OrderEnum::MultipleOptions(
        vec![
            vec![
                "PREDEFINED_NAME",
                "STRING",
                "OPERATOR",
                "STRING",
                "PREDEFINED_NAME"
            ],
            vec![
                "PREDEFINED_NAME",
                "INTEGER",
                "OPERATOR",
                "INTEGER",
                "PREDEFINED_NAME"
            ],
            vec![
                "PREDEFINED_NAME",
                "VARIABLE/FUNCTION_NAME",
                "OPERATOR",
                "VARIABLE/FUNCTION_NAME",
                "PREDEFINED_NAME"
            ]
        ]));

        hashmap.insert("PUSH", OrderEnum::MultipleOptions(
        vec![
            vec![
                "PREDEFINED_NAME",
                "STRING",
                "PREDEFINED_NAME",
                "VARIABLE/FUNCTION_NAME"
            ],
            vec![
                "PREDEFINED_NAME",
                "INTEGER",
                "PREDEFINED_NAME",
                "VARIABLE/FUNCTION_NAME"
            ],
            vec![
                "PREDEFINED_NAME",
                "VARIABLE/FUNCTION_NAME",
                "PREDEFINED_NAME",
                "VARIABLE/FUNCTION_NAME"
            ]
        ]));

        hashmap.insert("POP", OrderEnum::SingleOption (
        vec![
            "PREDEFINED_NAME",
            "PREDEFINED_NAME",
            "VARIABLE/FUNCTION_NAME"
        ]));

        hashmap.insert("INSERT", OrderEnum::MultipleOptions(
        vec![
            vec![
                "PREDEFINED_NAME",
                "STRING",
                "PREDEFINED_NAME",
                "VARIABLE/FUNCTION_NAME",
                "PREDEFINED_NAME",
                "INTEGER"
            ],
            vec![
                "PREDEFINED_NAME",
                "INTEGER",
                "PREDEFINED_NAME",
                "VARIABLE/FUNCTION_NAME",
                "PREDEFINED_NAME",
                "INTEGER"
            ],
            vec![
                "PREDEFINED_NAME",
                "VARIABLE/FUNCTION_NAME",
                "PREDEFINED_NAME",
                "VARIABLE/FUNCTION_NAME",
                "PREDEFINED_NAME",
                "INTEGER"
            ]
        ]));

        hashmap.insert("REMOVE", OrderEnum::MultipleOptions(
        vec![
            vec![
                "PREDEFINED_NAME",
                "PREDEFINED_NAME",
                "VARIABLE/FUNCTION_NAME",
                "PREDEFINED_NAME",
                "INTEGER"
            ],
            vec![
                "PREDEFINED_NAME",
                "PREDEFINED_NAME",
                "VARIABLE/FUNCTION_NAME",
                "PREDEFINED_NAME",
                "INTEGER"
            ],
            vec![
                "PREDEFINED_NAME",
                "PREDEFINED_NAME",
                "VARIABLE/FUNCTION_NAME",
                "PREDEFINED_NAME",
                "INTEGER"
            ]
        ]));

        hashmap.insert("REMOVE", OrderEnum::MultipleOptions(
        vec![
            vec![
                "PREDEFINED_NAME",
                "PREDEFINED_NAME",
                "VARIABLE/FUNCTION_NAME",
                "PREDEFINED_NAME",
                "INTEGER"
            ],
            vec![
                "PREDEFINED_NAME",
                "PREDEFINED_NAME",
                "VARIABLE/FUNCTION_NAME",
                "PREDEFINED_NAME",
                "INTEGER"
            ],
            vec![
                "PREDEFINED_NAME",
                "PREDEFINED_NAME",
                "VARIABLE/FUNCTION_NAME",
                "PREDEFINED_NAME",
                "INTEGER"
            ]
        ]));
        
        hashmap.insert("GET", OrderEnum::MultipleOptions(
        vec![
            vec![
                "PREDEFINED_NAME",
                "PREDEFINED_NAME",
                "VARIABLE/FUNCTION_NAME",
                "PREDEFINED_NAME",
                "INTEGER"
            ],
            vec![ // LEN
                "PREDEFINED_NAME",
                "PREDEFINED_NAME",
                "VARIABLE/FUNCTION_NAME",
                "PREDEFINED_NAME"
            ]
        ]));

        hashmap
    };

    let first_key_element = &token_collection[0].0;
    let first_value_element = &token_collection[0].1; 

    // first execution error check - check if which predefined_name - check if order is right
    if first_key_element != "PREDEFINED_NAME" {
        println!("EXECUTION ERROR: EVERY LINE HAS TO START WITH A PREDEFINED NAME (EXCEPT FOR COMMENT-LINES) !");
        return;
    } else {
        match first_value_element {
            tokenizer::ValueEnum::String(clean) => {
                match predefined_name_order.get(&clean.as_str()) {
                    Some(value) => {
                        match value {
                            OrderEnum::SingleOption(v) => {
                                if token_collection.len() < v.len() {
                                    println!("EXECUTION ERROR: THERE ARE LESS TOKENS THAN {} NEEDS!", clean);
                                    return;
                                }
                                if token_collection.len() > v.len() {
                                    println!("EXECUTION ERROR: THERE ARE MORE TOKENS THAN {} NEEDS!", clean);
                                    return;
                                }

                                let mut option_passed = true;
                                
                                for element_nr in 0..v.len() {
                                    if token_collection[element_nr].0 != v[element_nr] {
                                        option_passed = false;
                                    }
                                }
                                if !(option_passed) {
                                    println!("EXECUTION ERROR: TOKEN ORDER FOR {} ISN'T RIGHT!", clean);
                                    return;
                                }
                            },
                            OrderEnum::MultipleOptions(v) => {
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
                                    println!("EXECUTION ERROR: THERE ARE LESS TOKENS THAN {} NEEDS!", clean);
                                    return;
                                }
                                if too_many_tokens {
                                    println!("EXECUTION ERROR: THERE ARE MORE TOKENS THAN {} NEEDS!", clean);
                                    return;
                                }
                                let mut one_option_passed = false;
                                for possibility_nr in 0..v.len() {
                                    let mut current_option_passed = true;
                                    for element_nr in 0..v[possibility_nr].len() {
                                        if token_collection[element_nr].0 != v[possibility_nr][element_nr] {
                                            current_option_passed = false;
                                        }
                                    }
                                    if current_option_passed {
                                        one_option_passed = true;
                                        break;
                                    }
                                }
                                if !(one_option_passed) {
                                    println!("EXECUTION ERROR: TOKEN ORDER FOR {} ISN'T RIGHT!", clean);
                                }
                            }
                        }
                    },
                    None => {
                        println!("EXECUTION ERROR: {} IS ALWAYS NOT AT THE BEGINNING!", clean);
                        return;
                    }
                }
            },
            tokenizer::ValueEnum::IntegerVector(_clean) => (),
            tokenizer::ValueEnum::StringVector(_clean) => (),
            tokenizer::ValueEnum::TokenVector(_clean) => ()
        }
    }
}    

pub fn reset() {
    //
}

pub fn stop() {
    //
}
