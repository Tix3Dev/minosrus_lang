use crate::ExecData;
use crate::tokenizer;

use std::collections::HashMap;
use std::process;

#[derive(Clone)]
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

fn is_key_and_value_order_right(line: Vec<(String, tokenizer::ValueEnum)>, predefined_name_order: HashMap<&str, OrderEnum>) -> String {
    match &line[0].1 {
        tokenizer::ValueEnum::String(clean) => {
            match predefined_name_order.get(&clean.as_str()) {
                Some(value) => {
                    // check if the key of the first token has multiple options
                    match value {
                        OrderEnum::SingleOption(v) => {
                            // length check - otherwise the indexing would panic
                            if line.len() < v.len() {
                                return format!("EXECUTION ERROR: THERE ARE LESS TOKENS THAN '{}' NEEDS!", clean);
                            }
                            if line.len() > v.len() {
                                return format!("EXECUTION ERROR: THERE ARE MORE TOKENS THAN '{}' NEEDS!", clean);
                            }

                            // analyse if order of key and value is right
                            let mut is_key_order_right = true;
                            let mut is_value_order_right = true;

                            for element_nr in 0..v.len() {
                                // check if key is right
                                if line[element_nr].0 != v[element_nr].split(':').nth(0).unwrap() {
                                    is_key_order_right= false;
                                    break;
                                }
                                // check if value is right
                                match &line[element_nr].1 {
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
                                return format!("EXECUTION ERROR: KEY ORDER FOR '{}' ISN'T RIGHT!", clean); 
                            }
                            if !(is_value_order_right) {
                                return format!("EXECUTION ERROR: VALUE ORDER FOR '{}' ISN'T RIGHT!", clean); 
                            }
                        },
                        OrderEnum::MultipleOptions(v) => {
                            // length check - otherwise the indexing would panic
                            let mut too_few_tokens = false;
                            let mut too_many_tokens = false;
                            for possibility_nr in 0..v.len() {
                                if line.len() < v[possibility_nr].len() {
                                    too_few_tokens = true;
                                }
                                if line.len() > v[possibility_nr].len() {
                                    too_many_tokens = true;
                                }
                            }
                            if too_few_tokens {
                                return format!("EXECUTION ERROR: THERE ARE LESS TOKENS THAN '{}' NEEDS!", clean);
                            }
                            if too_many_tokens {
                                return format!("EXECUTION ERROR: THERE ARE MORE TOKENS THAN '{}' NEEDS!", clean);
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
                                    if line[element_nr].0 != v[possibility_nr][element_nr].split(':').nth(0).unwrap() {
                                        is_current_token_order_right = false;
                                    }
                                    // check if value is right
                                    match &line[element_nr].1 {
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
                            // print help - show all possible orders
                            if !(is_one_token_order_right) {
                                return format!("EXECUTION ERROR: KEY ORDER FOR '{}' ISN'T RIGHT!", clean);
                            }
                            if !(is_one_value_order_right) {
                                return format!("EXECUTION ERROR: VALUE ORDER FOR '{}' ISN'T RIGHT!", clean);
                            }
                        }
                    }
                },
                None => {
                    return format!("EXECUTION ERROR: '{}' IS NEVER AT THE BEGINNING!", clean);
                }
            }
        },
        _ => unreachable!("SOMEHOW THIS SHOULDN'T BE PRINTED!")
    }

    return format!("");
}

fn check_block_code_condition(operator: String, block_code: Vec<Vec<(String, tokenizer::ValueEnum)>>) -> bool {
    if operator == "==" {
        match (&block_code[0][1].1, &block_code[0][3].1) {
            (tokenizer::ValueEnum::String(a), tokenizer::ValueEnum::String(b)) => a == b,
            (tokenizer::ValueEnum::Integer(a), tokenizer::ValueEnum::Integer(b)) => a == b,
            (tokenizer::ValueEnum::Float(a), tokenizer::ValueEnum::Float(b)) => a == b,
            _ => unreachable!("SOMEHOW THIS SHOULDN'T BE PRINTED!")
        }
    }
    else if operator == "!=" {
        match (&block_code[0][1].1, &block_code[0][3].1) {
            (tokenizer::ValueEnum::String(a), tokenizer::ValueEnum::String(b)) => a != b,
            (tokenizer::ValueEnum::Integer(a), tokenizer::ValueEnum::Integer(b)) => a != b,
            (tokenizer::ValueEnum::Float(a), tokenizer::ValueEnum::Float(b)) => a != b,
            _ => unreachable!("SOMEHOW THIS SHOULDN'T BE PRINTED!")
        }
    }
    else if operator == "<" {
        match (&block_code[0][1].1, &block_code[0][3].1) {
            (tokenizer::ValueEnum::String(a), tokenizer::ValueEnum::String(b)) => a < b,
            (tokenizer::ValueEnum::Integer(a), tokenizer::ValueEnum::Integer(b)) => a < b,
            (tokenizer::ValueEnum::Float(a), tokenizer::ValueEnum::Float(b)) => a < b,
            _ => unreachable!("SOMEHOW THIS SHOULDN'T BE PRINTED!")
        }
    }
    else if operator == ">" {
        match (&block_code[0][1].1, &block_code[0][3].1) {
            (tokenizer::ValueEnum::String(a), tokenizer::ValueEnum::String(b)) => a > b,
            (tokenizer::ValueEnum::Integer(a), tokenizer::ValueEnum::Integer(b)) => a > b,
            (tokenizer::ValueEnum::Float(a), tokenizer::ValueEnum::Float(b)) => a > b,
            _ => unreachable!("SOMEHOW THIS SHOULDN'T BE PRINTED!")
        }
    }
    else if operator == "<=" {
        match (&block_code[0][1].1, &block_code[0][3].1) {
            (tokenizer::ValueEnum::String(a), tokenizer::ValueEnum::String(b)) => a <= b,
            (tokenizer::ValueEnum::Integer(a), tokenizer::ValueEnum::Integer(b)) => a <= b,
            (tokenizer::ValueEnum::Float(a), tokenizer::ValueEnum::Float(b)) => a <= b,
            _ => unreachable!("SOMEHOW THIS SHOULDN'T BE PRINTED!")
        }
    }
    else if operator == ">=" {
        match (&block_code[0][1].1, &block_code[0][3].1) {
            (tokenizer::ValueEnum::String(a), tokenizer::ValueEnum::String(b)) => a >= b,
            (tokenizer::ValueEnum::Integer(a), tokenizer::ValueEnum::Integer(b)) => a >= b,
            (tokenizer::ValueEnum::Float(a), tokenizer::ValueEnum::Float(b)) => a >= b,
            _ => unreachable!("SOMEHOW THIS SHOULDN'T BE PRINTED!")
        }
    } else {
        unreachable!("SOMEHOW THIS SHOULDN'T BE PRINTED!");
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

impl ExecData {
    fn execute_block_code(&mut self, block_code: Vec<Vec<(String, tokenizer::ValueEnum)>>) {
        for line in block_code {
            self.exec(line);
        }
    }

    pub fn exec(&mut self, token_collection: Vec<(String, tokenizer::ValueEnum)>) -> String {
        let mut token_collection = token_collection.clone();
        println!("token_collection: {:?}", token_collection);

        // check for syntax errors
        if let Some((_, value)) = token_collection.iter().find(|(key, _)| key == &"ERROR_MESSAGE") {
            match value {
                tokenizer::ValueEnum::String(v) => {
                    return format!("SYNTAX ERROR: {}", v);
                },
                _ => unreachable!("SOMEHOW THIS SHOULDN'T BE PRINTED!")
            }
        }
        // check for comments -> just make a newline
        if let Some((_, value)) = token_collection.iter().find(|(key, _)| key == &"COMMENT") {
            match value {
                tokenizer::ValueEnum::String(_v) => {
                    println!("");
                    return format!("");
                },
                _ => unreachable!("SOMEHOW THIS SHOULDN'T BE PRINTED!")
            }
        }
        // check for reset
        match &token_collection[0].1 {
            tokenizer::ValueEnum::String(v) => {
                if v == "RESET" && token_collection.len() == 1 {
                    self.global_variables.clear();
                    return format!("");
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
                        "FLOAT:?"
                    ],
                    vec![
                        "PREDEFINED_NAME:LET",
                        "VARIABLE/FUNCTION_NAME:?",
                        "EQUAL_SIGN:=",
                        "ARRAY:?"
                    ],
                    vec![
                        "PREDEFINED_NAME:LET",
                        "VARIABLE/FUNCTION_NAME:?",
                        "EQUAL_SIGN:=",
                        "VARIABLE/FUNCTION_NAME:?"
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
                    ],
                    vec![
                        "PREDEFINED_NAME:PRINT",
                        "FLOAT:?"
                    ],
                    vec![
                        "PREDEFINED_NAME:PRINT",
                        "VARIABLE/FUNCTION_NAME:?"
                    ],
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
                        "FLOAT:?",
                        "COMPARING_OPERATOR:?",
                        "FLOAT:?",
                        "PREDEFINED_NAME:START"
                    ],
                    vec![
                        "PREDEFINED_NAME:IF",
                        "STRING:?",
                        "COMPARING_OPERATOR:?",
                        "VARIABLE/FUNCTION_NAME:?",
                        "PREDEFINED_NAME:START"
                    ],
                    vec![
                        "PREDEFINED_NAME:IF",
                        "INTEGER:?",
                        "COMPARING_OPERATOR:?",
                        "VARIABLE/FUNCTION_NAME:?",
                        "PREDEFINED_NAME:START"
                    ],
                    vec![
                        "PREDEFINED_NAME:IF",
                        "FLOAT:?",
                        "COMPARING_OPERATOR:?",
                        "VARIABLE/FUNCTION_NAME:?",
                        "PREDEFINED_NAME:START"
                    ],
                    vec![
                        "PREDEFINED_NAME:IF",
                        "VARIABLE/FUNCTION_NAME:?",
                        "COMPARING_OPERATOR:?",
                        "STRING:?",
                        "PREDEFINED_NAME:START"
                    ],
                    vec![
                        "PREDEFINED_NAME:IF",
                        "VARIABLE/FUNCTION_NAME:?",
                        "COMPARING_OPERATOR:?",
                        "INTEGER:?",
                        "PREDEFINED_NAME:START"
                    ],
                    vec![
                        "PREDEFINED_NAME:IF",
                        "VARIABLE/FUNCTION_NAME:?",
                        "COMPARING_OPERATOR:?",
                        "FLOAT:?",
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

            hashmap.insert("ELIF", OrderEnum::MultipleOptions(
                vec![
                    vec![
                        "PREDEFINED_NAME:ELIF",
                        "STRING:?",
                        "COMPARING_OPERATOR:?",
                        "STRING:?",
                        "PREDEFINED_NAME:START"
                    ],
                    vec![
                        "PREDEFINED_NAME:ELIF",
                        "INTEGER:?",
                        "COMPARING_OPERATOR:?",
                        "INTEGER:?",
                        "PREDEFINED_NAME:START"
                    ],
                    vec![
                        "PREDEFINED_NAME:ELIF",
                        "FLOAT:?",
                        "COMPARING_OPERATOR:?",
                        "FLOAT:?",
                        "PREDEFINED_NAME:START"
                    ],
                    vec![
                        "PREDEFINED_NAME:ELIF",
                        "STRING:?",
                        "COMPARING_OPERATOR:?",
                        "VARIABLE/FUNCTION_NAME:?",
                        "PREDEFINED_NAME:START"
                    ],
                    vec![
                        "PREDEFINED_NAME:ELIF",
                        "INTEGER:?",
                        "COMPARING_OPERATOR:?",
                        "VARIABLE/FUNCTION_NAME:?",
                        "PREDEFINED_NAME:START"
                    ],
                    vec![
                        "PREDEFINED_NAME:ELIF",
                        "FLOAT:?",
                        "COMPARING_OPERATOR:?",
                        "VARIABLE/FUNCTION_NAME:?",
                        "PREDEFINED_NAME:START"
                    ],
                    vec![
                        "PREDEFINED_NAME:ELIF",
                        "VARIABLE/FUNCTION_NAME:?",
                        "COMPARING_OPERATOR:?",
                        "STRING:?",
                        "PREDEFINED_NAME:START"
                    ],
                    vec![
                        "PREDEFINED_NAME:ELIF",
                        "VARIABLE/FUNCTION_NAME:?",
                        "COMPARING_OPERATOR:?",
                        "INTEGER:?",
                        "PREDEFINED_NAME:START"
                    ],
                    vec![
                        "PREDEFINED_NAME:ELIF",
                        "VARIABLE/FUNCTION_NAME:?",
                        "COMPARING_OPERATOR:?",
                        "FLOAT:?",
                        "PREDEFINED_NAME:START"
                    ],
                    vec![
                        "PREDEFINED_NAME:ELIF",
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
                        "FLOAT:?",
                        "COMPARING_OPERATOR:?",
                        "FLOAT:?",
                        "PREDEFINED_NAME:START"
                    ],
                    vec![
                        "PREDEFINED_NAME:WHILE",
                        "STRING:?",
                        "COMPARING_OPERATOR:?",
                        "VARIABLE/FUNCTION_NAME:?",
                        "PREDEFINED_NAME:START"
                    ],
                    vec![
                        "PREDEFINED_NAME:WHILE",
                        "INTEGER:?",
                        "COMPARING_OPERATOR:?",
                        "VARIABLE/FUNCTION_NAME:?",
                        "PREDEFINED_NAME:START"
                    ],
                    vec![
                        "PREDEFINED_NAME:WHILE",
                        "FLOAT:?",
                        "COMPARING_OPERATOR:?",
                        "VARIABLE/FUNCTION_NAME:?",
                        "PREDEFINED_NAME:START"
                    ],
                    vec![
                        "PREDEFINED_NAME:WHILE",
                        "VARIABLE/FUNCTION_NAME:?",
                        "COMPARING_OPERATOR:?",
                        "STRING:?",
                        "PREDEFINED_NAME:START"
                    ],
                    vec![
                        "PREDEFINED_NAME:WHILE",
                        "VARIABLE/FUNCTION_NAME:?",
                        "COMPARING_OPERATOR:?",
                        "INTEGER:?",
                        "PREDEFINED_NAME:START"
                    ],
                    vec![
                        "PREDEFINED_NAME:WHILE",
                        "VARIABLE/FUNCTION_NAME:?",
                        "COMPARING_OPERATOR:?",
                        "FLOAT:?",
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
                        "FLOAT:?",
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
                    ],
                    vec![
                        "PREDEFINED_NAME:INSERT",
                        "FLOAT:?",
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

            hashmap.insert("HELP_FOR", OrderEnum::SingleOption(
                vec![
                    "PREDEFINED_NAME:HELP_FOR",
                    "PREDEFINED_NAME:?",
                ]));


            hashmap
        };



        // check for code block stuff
        if self.indentation.to_string() != "".to_string() {
            match &token_collection[0].1 {
                tokenizer::ValueEnum::String(v) => {
                    if v == "IF" || v == "WHILE" {
                        add_indentation(&mut self.indentation);
                    }
                    else if v == "FN" {
                        if self.current_block_type.0 == "" {
                            add_indentation(&mut self.indentation);
                        } else {
                            return format!("EXECUTION ERROR: FUNCTIONS CAN'T BE INSIDE OF OTHER CODE BLOCKS!");
                        }
                    }
                    else if v == "END" && token_collection.len() == 1 {
                        subtract_indentation(&mut self.indentation);
                        if self.indentation.to_string() == "".to_string() {
                            if self.current_block_type.0 == "normal" {
                                match &self.block_code[0][0].1 {
                                    tokenizer::ValueEnum::String(first_predefined_name) => {
                                        if first_predefined_name == "IF" {
                                            match &self.block_code[0][2].1 {
                                                tokenizer::ValueEnum::String(operator) => {
                                                    let mut elif_part: Vec<Vec<(String, tokenizer::ValueEnum)>> = Vec::new();
                                                    let mut else_part: Vec<Vec<(String, tokenizer::ValueEnum)>> = Vec::new();

                                                    let mut is_there_elif_block = false;
                                                    let mut is_elif_block_true = false;
                                                    let mut where_is_elif_block = 0;

                                                    let mut is_there_else_block = false;
                                                    let mut where_is_else_block = 0;

                                                    for (line_position, line) in self.block_code.iter().enumerate() {
                                                        match &line[0].1 {
                                                            tokenizer::ValueEnum::String(first_token) => {
                                                                if first_token == "ELIF" {
                                                                    is_there_elif_block = true;
                                                                    where_is_elif_block = line_position;

                                                                    // check key and value order
                                                                    let return_of_check = is_key_and_value_order_right(line.to_vec(), predefined_name_order.clone());
                                                                    if return_of_check != "".to_string() {
                                                                        return format!("{}", return_of_check);
                                                                    }

                                                                    // evaluate left and right side of elif
                                                                    let mut line_clone = line.clone();

                                                                    if line[1].0 == "VARIABLE/FUNCTION_NAME" {
                                                                        match &line[1].1 {
                                                                            tokenizer::ValueEnum::String(variable_name) => {
                                                                                match self.global_variables.get(variable_name) {
                                                                                    Some(value_of_variable) => {
                                                                                        match value_of_variable {
                                                                                            tokenizer::ValueEnum::String(v) => {
                                                                                                line_clone[1].0 = "STRING".to_string();
                                                                                                line_clone[1].1 = tokenizer::ValueEnum::String(v.to_string());
                                                                                            },
                                                                                            tokenizer::ValueEnum::Integer(v) => {
                                                                                                line_clone[1].0 = "INTEGER".to_string();
                                                                                                line_clone[1].1 = tokenizer::ValueEnum::Integer(*v);
                                                                                            },
                                                                                            _ => {
                                                                                                return format!("EXECUTION ERROR: FIRST VARIABLE HAS TO BE A STRING OR INTEGER!");
                                                                                            }
                                                                                        }
                                                                                    },
                                                                                    None => {
                                                                                        return format!("EXECUTION ERROR: THERE IS NO VARIABLE CALLED {}", variable_name);
                                                                                    }
                                                                                }
                                                                            },
                                                                            _ => unreachable!("SOMEHOW THIS SHOULDN'T BE PRINTED!")
                                                                        }
                                                                    }
                                                                    if line[3].0 == "VARIABLE/FUNCTION_NAME" {
                                                                        match &line[3].1 {
                                                                            tokenizer::ValueEnum::String(variable_name) => {
                                                                                match self.global_variables.get(variable_name) {
                                                                                    Some(value_of_variable) => {
                                                                                        match value_of_variable {
                                                                                            tokenizer::ValueEnum::String(v) => {
                                                                                                line_clone[3].0 = "STRING".to_string();
                                                                                                line_clone[3].1 = tokenizer::ValueEnum::String(v.to_string());
                                                                                            },
                                                                                            tokenizer::ValueEnum::Integer(v) => {
                                                                                                line_clone[3].0 = "INTEGER".to_string();
                                                                                                line_clone[3].1 = tokenizer::ValueEnum::Integer(*v);
                                                                                            },
                                                                                            _ => {
                                                                                                return format!("EXECUTION ERROR: SECOND VARIABLE HAS TO BE A STRING OR INTEGER!");
                                                                                            }
                                                                                        }
                                                                                    },
                                                                                    None => {
                                                                                        return format!("EXECUTION ERROR: THERE IS NO VARIABLE CALLED {}", variable_name);
                                                                                    }
                                                                                }
                                                                            },
                                                                            _ => unreachable!("SOMEHOW THIS SHOULDN'T BE PRINTED!")
                                                                        }
                                                                    }

                                                                    // check if elif condition is true
                                                                    match &line_clone[2].1 {
                                                                        tokenizer::ValueEnum::String(elif_operator) => {
                                                                            if check_block_code_condition(elif_operator.to_string(), vec![line_clone]) {
                                                                                is_elif_block_true = true;
                                                                            }
                                                                        },
                                                                        _ => unreachable!("SOMEHOW THIS SHOULDN'T BE PRINTED!")
                                                                    }
                                                                }
                                                                if first_token == "ELSE" {
                                                                    if line.len() == 1 {
                                                                        is_there_else_block = true;
                                                                        where_is_else_block = line_position;
                                                                    } else {
                                                                        return format!("EXECUTION ERROR: THERE ARE MORE TOKENS THAN ELSE NEEDS!");
                                                                    }
                                                                }
                                                            },
                                                            _ => unreachable!("SOMEHOW THIS SHOULDN'T BE PRINTED")
                                                        }
                                                    }
                                                    let if_part = if is_there_elif_block && is_there_else_block {
                                                        elif_part = self.block_code[where_is_elif_block+1..where_is_else_block].to_vec();
                                                        else_part = self.block_code[where_is_else_block+1..].to_vec();
                                                        self.block_code[..where_is_elif_block].to_vec()
                                                    }
                                                    else if is_there_elif_block {
                                                        elif_part = self.block_code[where_is_elif_block+1..].to_vec();
                                                        self.block_code[..where_is_elif_block].to_vec()
                                                    }
                                                    else if is_there_else_block {
                                                        else_part = self.block_code[where_is_else_block+1..].to_vec();
                                                        self.block_code[..where_is_else_block].to_vec()
                                                    } else {
                                                        self.block_code[1..].to_vec()
                                                    };


                                                    if check_block_code_condition(operator.to_string(), self.block_code.to_vec()) {
                                                        self.execute_block_code(if_part.to_vec());
                                                    }
                                                    else if is_there_elif_block && is_elif_block_true {
                                                        self.execute_block_code(elif_part.to_vec());
                                                    }
                                                    else if is_there_else_block {
                                                        self.execute_block_code(else_part);
                                                    }
                                                },
                                                _ => unreachable!("SOMEHOW THIS SHOULDN'T BE PRINTED!")
                                            }
                                        }
                                        else if first_predefined_name == "WHILE" {
                                            let mut error = "".to_string();

                                            match &self.block_code[0][2].1.clone() {
                                                tokenizer::ValueEnum::String(operator) => {
                                                    loop {
                                                        let new_block_code = update_while_condition_values(&self.block_code, &self.global_variables, &mut error);
                                                        if error != "".to_string() {
                                                            return format!("{}", error);
                                                        }

                                                        if check_block_code_condition(operator.to_string(), new_block_code) {
                                                            self.execute_block_code(self.block_code[1..].to_vec());
                                                        } else {
                                                            break;
                                                        }
                                                    }
                                                },
                                                _ => unreachable!("SOMEHOW THIS SHOULDN'T BE PRINTED!")
                                            }
                                        }
                                    },
                                    _ => unreachable!("SOMEHOW THIS SHOULDN'T BE PRINTED!")
                                }

                                self.current_block_type.0 = "".to_string();
                                self.block_code.clear();
                            }
                            else if self.current_block_type.0 == "function" {
                                self.current_block_type.0 = "".to_string();
                                self.current_block_type.1 = "".to_string();
                            }
                        }
                    }

                    // saving code block stuff
                    if self.current_block_type.0 == "normal" {
                        if v == "FN" {
                            return format!("EXECUTION ERROR: FUNCTIONS CAN'T BE INSIDE OF OTHER CODE BLOCKS!");
                        }
                        self.block_code.push(token_collection.clone());
                    }
                    else if self.current_block_type.0 == "function" {
                        // function in function already checked
                        self.functions.get_mut(&self.current_block_type.1).unwrap().push(token_collection.clone());
                    }

                },
                _ => unreachable!("SOMEHOW THIS SHOULDN'T BE PRINTED!")
            }

            return format!("");
        }


        // *check order of keys and values + evaluation* //

        let first_key_element = &token_collection[0].0;

        if first_key_element != "PREDEFINED_NAME" {
            return format!("EXECUTION ERROR: EVERY LINE HAS TO START WITH A PREDEFINED NAME (EXCEPT FOR COMMENT-LINES) !");
        }

        // check if key and value order is right
        let return_of_check = is_key_and_value_order_right(token_collection.to_vec(), predefined_name_order.clone());
        if return_of_check != "".to_string() {
            return format!("{}", return_of_check);
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
                                                            match self.global_variables.get(variable_name) {
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
                                                                        tokenizer::ValueEnum::Float(v) => {
                                                                            token_collection_clone[3].0 = "FLOAT".to_string();
                                                                            token_collection_clone[3].1 = tokenizer::ValueEnum::Float(*v);
                                                                        },
                                                                        tokenizer::ValueEnum::Array(v) => {
                                                                            token_collection_clone[3].0 = "ARRAY".to_string();
                                                                            token_collection_clone[3].1 = tokenizer::ValueEnum::Array(v.to_vec());
                                                                        }
                                                                    }
                                                                },
                                                                None => {
                                                                    return format!("EXECUTION ERROR: THERE IS NO VARIABLE CALLED {}", variable_name);
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
                                                            match self.global_variables.get(variable_name) {
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
                                                                        tokenizer::ValueEnum::Float(v) => {
                                                                            token_collection_clone[1].0 = "FLOAT".to_string();
                                                                            token_collection_clone[1].1 = tokenizer::ValueEnum::Float(*v);
                                                                        },
                                                                        _ => {
                                                                            return format!("EXECUTION ERROR: CAN'T PRINT THIS VARIABLE!");
                                                                        }
                                                                    }
                                                                },
                                                                None => {
                                                                    return format!("EXECUTION ERROR: THERE IS NO VARIABLE CALLED {}", variable_name);
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
                                                            match self.global_variables.get(variable_name) {
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
                                                                        tokenizer::ValueEnum::Float(v) => {
                                                                            token_collection_clone[1].0 = "FLOAT".to_string();
                                                                            token_collection_clone[1].1 = tokenizer::ValueEnum::Float(*v);
                                                                        },
                                                                        _ => {
                                                                            return format!("EXECUTION ERROR: FIRST VARIABLE HAS TO BE A STRING, INTEGER OR FLOAT!");
                                                                        }
                                                                    }
                                                                },
                                                                None => {
                                                                    return format!("EXECUTION ERROR: THERE IS NO VARIABLE CALLED {}", variable_name);
                                                                }
                                                            }
                                                        },
                                                        _ => unreachable!("SOMEHOW THIS SHOULDN'T BE PRINTED!")
                                                    }
                                                }
                                                if token_collection[3].0 == "VARIABLE/FUNCTION_NAME" {
                                                    match &token_collection[3].1 {
                                                        tokenizer::ValueEnum::String(variable_name) => {
                                                            match self.global_variables.get(variable_name) {
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
                                                                        tokenizer::ValueEnum::Float(v) => {
                                                                            token_collection_clone[3].0 = "FLOAT".to_string();
                                                                            token_collection_clone[3].1 = tokenizer::ValueEnum::Float(*v);
                                                                        },
                                                                        _ => {
                                                                            return format!("EXECUTION ERROR: SECOND VARIABLE HAS TO BE A STRING, INTEGER OR FLOAT!");
                                                                        }
                                                                    }
                                                                },
                                                                None => {
                                                                    return format!("EXECUTION ERROR: THERE IS NO VARIABLE CALLED {}", variable_name);
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
                                                            match self.global_variables.get(variable_name) {
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
                                                                        tokenizer::ValueEnum::Float(v) => {
                                                                            token_collection_clone[1].0 = "FLOAT".to_string();
                                                                            token_collection_clone[1].1 = tokenizer::ValueEnum::Float(*v);
                                                                        },
                                                                        _ => {
                                                                            return format!("EXECUTION ERROR: FIRST VARIABLE HAS TO BE A STRING, INTEGER OR FLOAT!");
                                                                        }
                                                                    }
                                                                },
                                                                None => {
                                                                    return format!("EXECUTION ERROR: THERE IS NO VARIABLE CALLED {}", variable_name);
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
                                                            match self.global_variables.get(variable_name) {
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
                                                                        tokenizer::ValueEnum::Float(v) => {
                                                                            token_collection_clone[1].0 = "FLOAT".to_string();
                                                                            token_collection_clone[1].1 = tokenizer::ValueEnum::Float(*v);
                                                                        },
                                                                        _ => {
                                                                            return format!("EXECUTION ERROR: SECOND VARIABLE HAS TO BE A STRING, INTEGER OR FLOAT!");
                                                                        }
                                                                    }
                                                                },
                                                                None => {
                                                                    return format!("EXECUTION ERROR: THERE IS NO VARIABLE CALLED {}", variable_name);
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
                            return format!("EXECUTION ERROR: '{}' IS NEVER AT THE BEGINNING!", clean);
                        }
                    }
                },
                _ => ()
            }

            token_collection = token_collection_clone;
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
                    self.global_variables.insert(variable_name, token_collection[3].1.clone());
                }
                else if v == &"PRINT".to_string() {
                    let stuff_to_print: String = {
                        match &token_collection[1].1 {
                            tokenizer::ValueEnum::String(stuff) => stuff.to_string(), 
                            tokenizer::ValueEnum::Integer(stuff) => stuff.to_string(),
                            tokenizer::ValueEnum::Float(stuff) => stuff.to_string(),
                            _ => unreachable!("SOMEHOW THIS SHOULDN'T BE PRINTED!")
                        }
                    };

                    println!("{}", stuff_to_print); // if float print decimal places
                }
                else if v == &"FN".to_string() {
                    match &token_collection[1].1 {
                        tokenizer::ValueEnum::String(fn_name) => {
                            self.functions.insert(fn_name.to_string(), vec![token_collection.clone()]);
                            self.current_block_type.0 = "function".to_string();
                            self.current_block_type.1 = fn_name.to_string();
                        },
                        _ => unreachable!("SOMEHOW THIS SHOULDN'T BE PRINTED!")
                    }
                    add_indentation(&mut self.indentation);
                }
                else if v == &"DO".to_string() {
                    match &token_collection[1].1 {
                        tokenizer::ValueEnum::String(function_name) => {
                            match self.functions.get(function_name) {
                                Some(function_code_block) => {
                                    let block_code = function_code_block[1..].to_vec();
                                    self.execute_block_code(block_code);
                                },
                                None => {
                                    return format!("EXECUTION ERROR: THERE IS NO FUNCTION CALLED {}", function_name);
                                }
                            }
                        },
                        _ => unreachable!("SOMEHOW THIS SHOULDN'T BE PRINTED!")
                    }
                }
                else if v == &"IF".to_string() {
                    self.block_code.push(token_collection);
                    self.current_block_type.0 = "normal".to_string();
                    add_indentation(&mut self.indentation);

                }
                else if v == &"WHILE".to_string() {
                    self.block_code.push(token_collection);
                    self.current_block_type.0 = "normal".to_string();
                    add_indentation(&mut self.indentation);
                }
                else if v == &"PUSH".to_string() {
                    match &token_collection[3].1 {
                        tokenizer::ValueEnum::String(stuff) => {
                            match self.global_variables.get(stuff) {
                                Some(value) => {
                                    match &value {
                                        tokenizer::ValueEnum::Array(vec) => {
                                            match &token_collection[1].1 {
                                                tokenizer::ValueEnum::String(stuff_to_push) => {
                                                    let mut vec_clone = vec.clone();
                                                    vec_clone.push(tokenizer::ArrayTypesEnum::String(stuff_to_push.to_string()));
                                                    *self.global_variables.get_mut(stuff).unwrap() = tokenizer::ValueEnum::Array(vec_clone);
                                                },
                                                tokenizer::ValueEnum::Integer(stuff_to_push) => {
                                                    let mut vec_clone = vec.clone();
                                                    vec_clone.push(tokenizer::ArrayTypesEnum::Integer(*stuff_to_push));
                                                    *self.global_variables.get_mut(stuff).unwrap() = tokenizer::ValueEnum::Array(vec_clone);
                                                },
                                                tokenizer::ValueEnum::Float(stuff_to_push) => {
                                                    let mut vec_clone = vec.clone();
                                                    vec_clone.push(tokenizer::ArrayTypesEnum::Float(*stuff_to_push));
                                                    *self.global_variables.get_mut(stuff).unwrap() = tokenizer::ValueEnum::Array(vec_clone);
                                                },
                                                _ => {
                                                    return format!("EXECUTION ERROR: YOU HAVE TO PUSH AN INTEGER OR A STRING ONTO AN ARRAY!");
                                                }
                                            }
                                        },
                                        _ => {
                                            return format!("EXECUTION  ERROR: YOU CAN ONLY PUSH ONTO ARRAYS!");
                                        }

                                    }
                                },
                                None => {
                                    return format!("EXECUTION ERROR: THERE IS NO VARIABLE CALLED {}", stuff);
                                }
                            }
                        },
                        _ => unreachable!("SOMEHOW THIS SHOULDN'T BE PRINTED!")
                    }
                }
                else if v == &"POP".to_string() {
                    match &token_collection[2].1 {
                        tokenizer::ValueEnum::String(stuff) => {
                            match self.global_variables.get(stuff) {
                                Some(value) => {
                                    match &value {
                                        tokenizer::ValueEnum::Array(vec) => {
                                            let mut vec_clone = vec.clone();
                                            vec_clone.pop();
                                            *self.global_variables.get_mut(stuff).unwrap() = tokenizer::ValueEnum::Array(vec_clone);
                                        },
                                        _ => {
                                            return format!("EXECUTION  ERROR: YOU CAN ONLY POP FROM ARRAYS!");
                                        }
                                    }
                                },
                                None => {
                                    return format!("EXECUTION ERROR: THERE IS NO VARIABLE CALLED {}", stuff);
                                }
                            }
                        },
                        _ => unreachable!("SOMEHOW THIS SHOULDN'T BE PRINTED!")
                    }
                }
                else if v == &"INSERT".to_string() {
                    match &token_collection[3].1 {
                        tokenizer::ValueEnum::String(stuff) => {
                            match self.global_variables.get(stuff) {
                                Some(value) => {
                                    match &value {
                                        tokenizer::ValueEnum::Array(vec) => {
                                            match &token_collection[1].1 {
                                                tokenizer::ValueEnum::String(stuff_to_push) => {
                                                    match &token_collection[5].1 {
                                                        tokenizer::ValueEnum::Integer(index_where_to_insert) => {
                                                            let mut vec_clone = vec.clone();
                                                            vec_clone.insert(*index_where_to_insert as usize, tokenizer::ArrayTypesEnum::String(stuff_to_push.to_string()));
                                                            *self.global_variables.get_mut(stuff).unwrap() = tokenizer::ValueEnum::Array(vec_clone);
                                                        },
                                                        _ => unreachable!("SOMEHOW THIS SHOUDLN'T BE PRINTED!")
                                                    }
                                                },
                                                tokenizer::ValueEnum::Integer(stuff_to_push) => {
                                                    match &token_collection[5].1 {
                                                        tokenizer::ValueEnum::Integer(index_where_to_insert) => {
                                                            let mut vec_clone = vec.clone();
                                                            vec_clone.insert(*index_where_to_insert as usize, tokenizer::ArrayTypesEnum::Integer(*stuff_to_push));
                                                            *self.global_variables.get_mut(stuff).unwrap() = tokenizer::ValueEnum::Array(vec_clone);
                                                        },
                                                        _ => unreachable!("SOMEHOW THIS SHOUDLN'T BE PRINTED!")
                                                    }
                                                },
                                                tokenizer::ValueEnum::Float(stuff_to_push) => {
                                                    match &token_collection[5].1 {
                                                        tokenizer::ValueEnum::Integer(index_where_to_insert) => {
                                                            let mut vec_clone = vec.clone();
                                                            vec_clone.insert(*index_where_to_insert as usize, tokenizer::ArrayTypesEnum::Float(*stuff_to_push));
                                                            *self.global_variables.get_mut(stuff).unwrap() = tokenizer::ValueEnum::Array(vec_clone);
                                                        },
                                                        _ => unreachable!("SOMEHOW THIS SHOUDLN'T BE PRINTED!")
                                                    }
                                                },
                                                _ => {
                                                    return format!("EXECUTION ERROR: YOU HAVE TO INSERT AN INTEGER INTO A INTEGER ARRAY!");
                                                }
                                            }
                                        },
                                        _ => {
                                            return format!("EXECUTION ERROR: YOU CAN ONLY INSERT INTO ARRAYS!");
                                        }
                                    }
                                },
                                None => {
                                    return format!("EXECUTION ERROR: THERE IS NO VARIABLE CALLED {}", stuff);
                                }
                            }
                        },
                        _ => unreachable!("SOMEHOW THIS SHOULDN'T BE PRINTED!")
                    }

                }
                else if v == &"REMOVE".to_string() {
                    match &token_collection[2].1 {
                        tokenizer::ValueEnum::String(stuff) => {
                            match self.global_variables.get(stuff) {
                                Some(value) => {
                                    match &value {
                                        tokenizer::ValueEnum::Array(vec) => {
                                            match &token_collection[4].1 {
                                                tokenizer::ValueEnum::Integer(index_where_to_remove) => {
                                                    let mut vec_clone = vec.clone();
                                                    vec_clone.remove(*index_where_to_remove as usize);
                                                    *self.global_variables.get_mut(stuff).unwrap() = tokenizer::ValueEnum::Array(vec_clone);
                                                },
                                                _ => unreachable!("SOMEHOW THIS SHOULDN'T BE PRINTED!")
                                            }
                                        },
                                        _ => unreachable!("SOMEHOW THIS SHOULDN'T BE PRINTED!")
                                    }
                                },
                                None => {
                                    return format!("EXECUTION ERROR: THERE IS NO VARIABLE CALLED {}", stuff);
                                }
                            }
                        },
                        _ => unreachable!("SOMEHOW THIS SHOULDN'T BE PRINTED!")
                    }

                }
                else if v == &"HELP_FOR".to_string() {
                    match &token_collection[1].1 {
                        tokenizer::ValueEnum::String(keyword) => {
                            match predefined_name_order.get(&keyword.as_str()) {
                                Some(order_collection) => {
                                    match order_collection {
                                        OrderEnum::SingleOption(v) => {
                                            println!("-SINGLE OPTION-");
                                            print!("1: '");
                                            for e_nr in 0..v.len() {
                                                if e_nr == v.len() - 1 {
                                                    println!("{}'", v[e_nr]);
                                                } else {
                                                    print!("{}, ", v[e_nr]);
                                                }
                                            }
                                        },
                                        OrderEnum::MultipleOptions(v) => {
                                            println!("-MULTIPLE OPTIONS-");
                                            for p_nr in 0..v.len() {
                                                print!("{}: '", p_nr+1);
                                                for e_nr in 0..v[p_nr].len() {
                                                    if e_nr == v[p_nr].len() - 1 {
                                                        println!("{}'", v[p_nr][e_nr]);
                                                    } else {
                                                        print!("{}, ", v[p_nr][e_nr]);
                                                    }
                                                }
                                            }
                                        }    
                                    }
                                },
                                None => {
                                    return format!("EXECUTION ERROR: CAN'T PRINT HELP FOR {} BECAUSE IT'S NOT A PREDEFINED NAME WHICH IS AT THE BEGINNING OF A LINE!", keyword);
                                }
                            }
                        },
                        _ => unreachable!("SOMEHOW THIS SHOULDN'T BE PRINTED!")
                    }
                }
            },
            _ => unreachable!("SOMEHOW THIS SHOULDN'T BE PRINTED!")
        }

        println!("self.global_variables: {:?}", self.global_variables);

        return format!("");
    }
}
