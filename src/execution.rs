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
                "PREDEFINED_NAME:LET", 
                "VARIABLE/FUNCTION_NAME:?", 
                "OPERATOR:=", 
                "STRING:?"
            ], 
            vec![
                "PREDEFINED_NAME:LET", 
                "VARIABLE/FUNCTION_NAME:?", 
                "OPERATOR:=", 
                "INTEGER:?"
            ],
            vec![
                "PREDEFINED_NAME:LET", 
                "VARIABLE/FUNCTION_NAME:?", 
                "OPERATOR:=", 
                "VARIABLE/FUNCTION_NAME:?"
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
                "OPERATOR:?",
                "STRING:?",
                "PREDEFINED_NAME:START"
            ],
            vec![
                "PREDEFINED_NAME:IF",
                "INTEGER:?",
                "OPERATOR:?",
                "INTEGER:?",
                "PREDEFINED_NAME:START"
            ],
            vec![
                "PREDEFINED_NAME:IF",
                "VARIABLE/FUNCTION_NAME:?",
                "OPERATOR:?",
                "VARIABLE/FUNCTION_NAME:?",
                "PREDEFINED_NAME:START"
            ]
        ]));

        hashmap.insert("WHILE", OrderEnum::MultipleOptions(
        vec![
            vec![
                "PREDEFINED_NAME:WHILE",
                "STRING:?",
                "OPERATOR:?",
                "STRING:?",
                "PREDEFINED_NAME:START"
            ],
            vec![
                "PREDEFINED_NAME:WHILE",
                "INTEGER:?",
                "OPERATOR:?",
                "INTEGER:?",
                "PREDEFINED_NAME:START"
            ],
            vec![
                "PREDEFINED_NAME:WHILE",
                "VARIABLE/FUNCTION_NAME:?",
                "OPERATOR:?",
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

                                let mut is_classification_order_right = true;
                                let mut is_value_order_right = true;

                                for element_nr in 0..v.len() {
                                    println!("1.1: {} compared to {}", token_collection[element_nr].0, v[element_nr].split(':').nth(0).unwrap());
                                    if token_collection[element_nr].0 != v[element_nr].split(':').nth(0).unwrap() {
                                        println!("compared");
                                        is_classification_order_right = false;
                                        break;
                                    }
                                    match &token_collection[element_nr].1 {
                                        tokenizer::ValueEnum::String(tc) => {
                                            println!("1.2: {} compared to {}", tc, v[element_nr].split(':').nth(1).unwrap());
                                            if tc != v[element_nr].split(':').nth(1).unwrap() && v[element_nr].split(':').nth(1).unwrap() != "?" {
                                                println!("compared");
                                                is_value_order_right = false;
                                                break; 
                                            }
                                        },
                                        tokenizer::ValueEnum::IntegerVector(_tc) => (),
                                        tokenizer::ValueEnum::StringVector(_tc) => (),
                                        tokenizer::ValueEnum::TokenVector(_tc) => ()
                                    }
                                }
                                if !(is_classification_order_right) {
                                    println!("EXECUTION ERROR: TOKEN ORDER FOR {} ISN'T RIGHT!", clean);
                                    return;
                                }
                                if !(is_value_order_right) {
                                    println!("EXECUTION ERROR: VALUE ORDER FOR {} ISN'T RIGHT!", clean);
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
                                let mut is_one_token_order_right = false;
                                let mut is_one_value_order_right = false;
                                for possibility_nr in 0..v.len() {
                                    let mut is_current_token_order_right = true;
                                    let mut is_current_value_order_right = true;
                                    for element_nr in 0..v[possibility_nr].len() {
                                        println!("2.1: {} compared to {}", token_collection[element_nr].0, v[possibility_nr][element_nr].split(':').nth(0).unwrap());
                                        if token_collection[element_nr].0 != v[possibility_nr][element_nr].split(':').nth(0).unwrap() {
                                            println!("compared");
                                            is_current_token_order_right = false;
                                            break;
                                        }
                                        match &token_collection[element_nr].1 {
                                            tokenizer::ValueEnum::String(tc) => {
                                                println!("2.2: {} compared to {}", tc, v[possibility_nr][element_nr].split(':').nth(1).unwrap());
                                                if tc != v[possibility_nr][element_nr].split(':').nth(1).unwrap() && v[possibility_nr][element_nr].split(':').nth(1).unwrap() != "?"{
                                                    println!("compared");
                                                    is_current_value_order_right = false;
                                                    break;
                                                }
                                            },
                                            tokenizer::ValueEnum::IntegerVector(_tc) => (),
                                            tokenizer::ValueEnum::StringVector(_tc) => (),
                                            tokenizer::ValueEnum::TokenVector(_tc) => ()
                                        }
                                    }
                                    if is_current_token_order_right {
                                        is_one_token_order_right = true;
                                        break;
                                    }
                                    if is_current_value_order_right {
                                        is_one_value_order_right = true;
                                        break;
                                    }
                                }
                                if !(is_one_token_order_right) {
                                    println!("EXECUTION ERROR: TOKEN ORDER FOR {} ISN'T RIGHT!", clean);
                                    return;
                                }
                                if !(is_one_value_order_right) {
                                    println!("EXECUTION ERROR: VALUE ORDER FOR {} ISN'T RIGHT!", clean);
                                    return;
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
