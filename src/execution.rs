pub mod tokenizer;

use std::collections::HashMap;

enum OrderEnum {
    SingleOption(Vec<&'static str>),
    MultipleOptions(Vec<Vec<&'static str>>)
}

pub fn exec(input: String) {
    // tokenize the input
    let token_collection = tokenizer::make_tokens(input);

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
                                for element_nr in 0..v.len() {
                                    println!("{:?}", v[element_nr]); 
                                }
                            },
                            OrderEnum::MultipleOptions(v) => {
                                for possibility_nr in 0..v.len() {
                                    for element_nr in 0..v[possibility_nr].len() {
                                        println!("{:?}", v[possibility_nr][element_nr]);
                                    }
                                }
                            }
                        }
                    },
                    None => {
                        println!("EXECUTION ERROR: FIRST PREDEFINED NAME IS ALWAYS NOT AT THE BEGINNING!");
                        return;
                    }
                }
            },
            tokenizer::ValueEnum::IntegerVector(_clean) => (),
            tokenizer::ValueEnum::StringVector(_clean) => (),
            tokenizer::ValueEnum::TokenVector(_clean) => ()
        }
    }

    /*
    match predefined_name_order.get("LET") {
        Some(value) => {
            match value {
                OrderEnum::SingleOption(v) => println!("It's single: {:?}", v),
                OrderEnum::MultipleOptions(v) => println!("It's not single: {:?}", v)
            }
        },
        None => ()
    }
    println!("{:?}", token_collection);
    */
}    

pub fn reset() {
    //
}

pub fn stop() {
    //
}
