use std::collections::HashMap;

use crate::tokenizer;
use crate::verine_expression::{Tokenizer, Token, TokenizerError};

#[derive(Debug, Clone)]
pub enum ArrayTypesEnum {
    String(String),
    Integer(i32),
    Float(f32)
}

#[derive(Debug, Clone)]
pub enum ValueEnum {
    String(String),
    Integer(i32),
    Float(f32),
    Array(Vec<ArrayTypesEnum>),
}

pub fn make_tokens(mut input: String, global_variables: &HashMap<String, tokenizer::ValueEnum>) -> Vec<(String, ValueEnum)> {
    // final tokens that are returned stored here
    let mut final_tokens: Vec<(String, ValueEnum)> = Vec::new();

    // used for the hashmap final_tokens -> classification of token
    let token_classification = vec![
        "PREDEFINED_NAME".to_string(),
        "ARITHMETIC_OPERATOR".to_string(),
        "COMPARING_OPERATOR".to_string(),
        "EQUAL_SIGN".to_string(),
        "STRING".to_string(),
        "INTEGER".to_string(),
        "FLOAT".to_string(),
        "ARRAY".to_string(),
        "COMMENT".to_string(),
        "VARIABLE/FUNCTION_NAME".to_string()
    ];
    
    // used to check whether a token is a ... or not
    let predefined_names = vec![
        "LET".to_string(),
        "PRINT".to_string(),
        "FN".to_string(),
        "START".to_string(),
        "END".to_string(),
        "DO".to_string(),
        "IF".to_string(),
        "ELSE".to_string(),
        "ELIF".to_string(),
        "WHILE".to_string(),
        "PUSH".to_string(),
        "POP".to_string(),
        "INSERT".to_string(),
        "REMOVE".to_string(),
        "GET".to_string(),
        "ONTO".to_string(),
        "FROM".to_string(),
        "INTO".to_string(),
        "AT".to_string(),
        "LEN".to_string(),
        "READLN".to_string(),
        "STRING_FROM".to_string(),
        "INTEGER_FROM".to_string(),
        "HELP_FOR".to_string(),
        "RESET".to_string(),
        "STOP".to_string()
    ];
    let arithmetic_operators = vec![
        "+".to_string(),
        "-".to_string(),
        "*".to_string(),
        "/".to_string(),
        "**".to_string(),
    ];
    let comparing_operators = vec![
        "==".to_string(),
        "!=".to_string(),
        "<".to_string(),
        ">".to_string(),
        "<=".to_string(),
        ">=".to_string()
    ];
    let equal_sign = "=".to_string();

    // used for checking whether a name is valid or not
    let allowed_variable_function_characters = {
        let mut vec = vec![];
        vec.extend('A'..='Z');
        vec.push('_');
        vec
    };

    // used for checking whether the inner part of a string is valid or not
    let allowed_string_inner_part_characters = {
        let mut vec = vec![];
        vec.extend('A'..='Z');
        vec.extend('0'..='9');
        vec.extend_from_slice(&[',', '.', ':', '!', '?', ' ']);
        vec
    };

    /////////////////

    // check for one verine and if one exists replace input
    if input.contains('|') {
        // save | positions
        let mut verine_positions: Vec<usize> = vec![];
        for (index, character) in input.chars().enumerate() {
            if character == '|' {
                verine_positions.push(index);
            }
        }

        if verine_positions.len() % 2 != 0 {
            final_tokens.push(("ERROR_MESSAGE".to_string(), ValueEnum::String("EVERY | NEEDS A | !".to_string())));
            return final_tokens;
        }

        /////////

        let (from, to) = (*verine_positions.first().unwrap(), *verine_positions.last().unwrap());
        let verine = Tokenizer::tokenize_and_evaluate(&input[from + 1..to], &global_variables);

        let mut evaluate_to = |result: &str| {
            input.replace_range(from..=to, result);
        };

        let mut push_error = |message: &str| {
            final_tokens.push(("ERROR_MESSAGE".to_string(), ValueEnum::String(message.to_string())));
        };

        match verine {
            Ok(token) => {
                match token {
                    Token::String(s) => evaluate_to(&format!("\"{}\"", s)),
                    Token::Integer(int) => evaluate_to(&int.to_string()),
                    Token::Float(float) => evaluate_to(&float.to_string()),
                    _ => panic!("The tokenizer must evaluate to a value"),
                }
            }
            Err(error) => {
                use TokenizerError::*;
                match error {
                    UnexpectedCharacter(char) => push_error(&format!("Unexpected character '{}'", char)),
                    StdInError => push_error("Error reading from stdin"),
                    InvalidExpression => push_error("Invalid expression"),
                    VariableNotFound(var) => push_error(&format!("Variable '{}' not found", var)),
                    NumberNotAnInteger(var) => push_error(&format!("'{}' is not an integer", var)),
                    InvalidOperands => push_error("Invalid operands"),
                    InvalidIndex(i) => push_error(&format!("'{}' is not a valid index", i)),
                    IndexOutOfBounds => push_error("Index out of bounds"),
                    TypeNotIndexable => push_error("Type is not indexable"),
                    TypeHasNoLength => push_error("Type has no length"),
                    DivisionByZero => push_error("Division by zero")
                }
            }
        }
    }

    // split input into parts; strings don't get split; arrays don't get split
    let input = input.trim().to_string() + " ";
    let mut current_token = String::new();
    let mut array_token = String::new();
    let mut verine_token = String::new();
    let mut last_character = 'a'; // just something that isn't a space
    let mut is_there_a_string = false;
    let mut array_started = false;
    let mut string_started = false;
    let mut verine_started = false;
    let mut split_of_input: Vec<String> = vec![];
    
    if input.contains(&"[".to_string()) && !(input.contains(&"]".to_string())) {
        final_tokens.push(("ERROR_MESSAGE".to_string(), ValueEnum::String("A [ ALWAYS NEEDS A ] ! ORDER: [ AND THEN ] !".to_string())));
        return final_tokens;
    }
    
    for character in input.chars() {
        if character == '[' {
            array_token.push(character);
            array_started = true;
        }
        else if character == ']' {
            if array_started {
                array_token.push(character);
                split_of_input.push(array_token);
                array_token = String::new();
                array_started = false;   
            } else {
                final_tokens.push(("ERROR_MESSAGE".to_string(), ValueEnum::String("A ] ALWAYS NEEDS A [ ! ORDER: [ AND THEN ] !".to_string())));
                return final_tokens;
            }
        }
        else if array_started {
            array_token.push(character);
        }
        else if character == '|' {
            if verine_token.starts_with('|') {
                verine_token.push(character);
                split_of_input.push(verine_token);
                verine_token = String::new();
                verine_started = false;
            } else {
                verine_token.push(character);
                verine_started = true;
            }
        }
        else if verine_started {
            verine_token.push(character);
        }
        else if character == ' ' {
            if current_token.starts_with('"') && !current_token.ends_with('"') {
                // space belongs to the string
                current_token.push(character);
            }
            else {
                // end of token
                if last_character != ' ' {
                    split_of_input.push(current_token);
                    current_token = String::new();
                }
            }
        } else {
            // normal character and if used (later on) for string not closed error; not in array because array checking does that; and checking if character is valid
            if character == '"' {
                is_there_a_string = true;
                if current_token.starts_with('"') {
                    string_started = false;
                } 
            }

            if !(allowed_string_inner_part_characters.contains(&character)) && string_started {
                final_tokens.push(("ERROR_MESSAGE".to_string(), ValueEnum::String("INVALID CHARACTER INSIDE OF THE STRING!".to_string())));
                return final_tokens;
            }

            if character == '"' {
                if !(current_token.starts_with('"')) {
                    string_started = true;
                } 
            }

            current_token.push(character);
        }

        last_character = character;
    }

    if is_there_a_string && current_token != "" {
        final_tokens.push(("ERROR_MESSAGE".to_string(), ValueEnum::String("STRING ISN'T CLOSED!".to_string())));
        return final_tokens;
    }
   
    if split_of_input[split_of_input.len() - 1] == "" {
        split_of_input.remove(split_of_input.len() - 1);
    }
    println!("split_of_input: {:?}", split_of_input);

    let mut i = 0;
    while i < split_of_input.len() {
        let part = &split_of_input[i];
        // predefined name check
        if predefined_names.contains(part) {
            final_tokens.push((token_classification[0].to_string(), ValueEnum::String(part.to_string())));
        }
        // arithmetic_operator check
        else if arithmetic_operators.contains(part) {
            final_tokens.push((token_classification[1].to_string(), ValueEnum::String(part.to_string())));
        }
        // comparing_operator check
        else if comparing_operators.contains(part) {
            final_tokens.push((token_classification[2].to_string(), ValueEnum::String(part.to_string())));
        }
        // equal_sign check
        else if part == &equal_sign {
            final_tokens.push((token_classification[3].to_string(), ValueEnum::String(part.to_string())));
        }
        // string check
        else if part.chars().nth(0).unwrap() == '\"' && part.chars().rev().nth(0).unwrap() == '\"' {
            final_tokens.push((token_classification[4].to_string(), ValueEnum::String(part.as_str()[1..part.len()-1].to_string())));
        }
        // integer check
        else if !(part.chars().any(|c| !(c.is_numeric() || c == '-' || c == '.'))) {
            let mut dot_count = 0;
            for (index, character) in part.chars().enumerate() {
                if index != 0 && character == '-' {
                    final_tokens.push(("ERROR_MESSAGE".to_string(), ValueEnum::String("NEGATIVE SIGN HAS TO BE AT THE BEGINNING OF A NUMBER!".to_string())));
                    break;

                }
                else if character == '.' {
                    if index == 0 {
                        final_tokens.push(("ERROR_MESSAGE".to_string(), ValueEnum::String("DOT CAN'T BE AT THE BEGINNING OF THE NUMBER!".to_string())));
                        break;

                    }
                    else if index == part.len() - 1 {
                         final_tokens.push(("ERROR_MESSAGE".to_string(), ValueEnum::String("DOT CAN'T BE AT THE END OF THE NUMBER!".to_string())));
                         break;
                    } else {
                        dot_count += 1;
                    }
                }
            }
            if dot_count > 1 {
                final_tokens.push(("ERROR_MESSAGE".to_string(), ValueEnum::String("THERE ARE TOO MANY DOTS IN THE NUMBER!".to_string())));
                break;
            }

            if part.parse::<i32>().is_ok() {
                final_tokens.push((token_classification[5].to_string(), ValueEnum::Integer(part.parse::<i32>().unwrap())));
            }
            else if part.parse::<f32>().is_ok() {
                final_tokens.push((token_classification[6].to_string(), ValueEnum::Float(part.parse::<f32>().unwrap())));
            } else {
                final_tokens.push(("ERROR_MESSAGE".to_string(), ValueEnum::String("THE NUMBER IS NOT I32 OR F32!".to_string())));
                break;
            }
        }
        // array check
        else if part.chars().nth(0).unwrap() == '[' && part.chars().rev().nth(0).unwrap() == ']' {
            let mut array = part.clone();

            // remove [ and ]
            array.remove(0);
            array.remove(array.len() - 1);

            // check if array is empty
            if array.trim().is_empty() {
                final_tokens.push((token_classification[7].to_string(), ValueEnum::Array(vec![])));
                break;
            }

            // check if there is a comma at the beginning or at the end - otherwise wrong error message would occur
            if array.trim().chars().nth(0).unwrap() == ',' {
                final_tokens.push(("ERROR_MESSAGE".to_string(), ValueEnum::String("THERE IS A COMMA AT THE BEGINNING OF THE ARRAY; NOT ALLOWED!".to_string())));
                break;
            }
            if array.trim().chars().rev().nth(0).unwrap() == ',' {
                final_tokens.push(("ERROR_MESSAGE".to_string(), ValueEnum::String("THERE IS A COMMA AT THE END OF THE ARRAY; NOT ALLOWED!".to_string())));
                break;
            } else {
                array.push(',');
            }

            // acutal spliting
            let mut split_of_array: Vec<ArrayTypesEnum> = vec![];
            let mut current_element = String::new();
            let mut is_string_active = false;
            let mut is_integer_active = false;
            let mut dot_count = 0;
            let mut valid_for_next_element = true;

            for (position, character) in array.chars().enumerate() {
                if is_string_active && character != '"' && position == array.len() - 1 {
                    final_tokens.push(("ERROR_MESSAGE".to_string(), ValueEnum::String("STRING IN THE ARRAY ISN'T CLOSED!".to_string())));
                    break;
                }
                if character == '"' {
                    if !(is_string_active) && !(is_integer_active) {
                        current_element.push(character);
                        is_string_active = true;
                    }
                    else if is_integer_active {
                        final_tokens.push(("ERROR_MESSAGE".to_string(), ValueEnum::String("INVALID CHARACTER IN THE ARRAY!".to_string())));
                        break;
                    }
                    else if is_string_active {
                        current_element.push(character);
                        split_of_array.push(ArrayTypesEnum::String(current_element));
                        current_element = String::new();
                        is_string_active = false;
                        valid_for_next_element = false;
                    }
                }
                else if character == '-' {
                    if !(is_string_active) && !(is_integer_active) && valid_for_next_element {
                        if array.chars().nth(position+1).unwrap().is_numeric() {
                            current_element.push(character);
                            is_integer_active = true;
                        } else {
                            final_tokens.push(("ERROR_MESSAGE".to_string(), ValueEnum::String("INVALID CHARACTER IN THE ARRAY!".to_string())));
                            break;
                        }
                    }
                }
                else if character == ',' {
                    if !(is_string_active) && !(is_integer_active) {
                        if !(valid_for_next_element) {
                            valid_for_next_element = true;
                        } else {
                            final_tokens.push(("ERROR_MESSAGE".to_string(), ValueEnum::String("THERE ARE TOO MANY COMMAS IN THE ARRAY!".to_string())));
                            break;
                        }
                    }
                    else if is_integer_active {
                        if current_element.parse::<i32>().is_ok() {
                            split_of_array.push(ArrayTypesEnum::Integer(current_element.parse::<i32>().unwrap()));
                            current_element = String::new();
                            dot_count = 0;
                            is_integer_active = false;
                        }
                        else if current_element.parse::<f32>().is_ok() {
                            split_of_array.push(ArrayTypesEnum::Float(current_element.parse::<f32>().unwrap()));
                            current_element = String::new();
                            dot_count = 0;
                            is_integer_active = false;
                        } else {
                            final_tokens.push(("ERROR_MESSAGE".to_string(), ValueEnum::String("NUMBER ELEMENTS OF AN ARRAY HAVE TO BE I32 OR F32!".to_string())));
                            break;
                        }
                    }
                } else {
                    if !(valid_for_next_element) {
                        final_tokens.push(("ERROR_MESSAGE".to_string(), ValueEnum::String("A COMMA IS MISSING!".to_string())));
                        break;
                    }
                    else if !(is_string_active) && !(is_integer_active) {
                        if character.is_numeric() {
                            current_element.push(character);
                            is_integer_active = true;
                        } else {
                            if character != ' ' {
                                final_tokens.push(("ERROR_MESSAGE".to_string(), ValueEnum::String("INVALID CHARACTER IN THE ARRAY!".to_string())));
                                break;
                            }
                        }
                    }
                    else if is_string_active {
                        if !(allowed_string_inner_part_characters.contains(&character)) {
                            final_tokens.push(("ERROR_MESSAGE".to_string(), ValueEnum::String("INVALID CHARACTER INSIDE OF THE STRING IN THE ARRAY!".to_string())));
                            break;
                        }
                        current_element.push(character);
                    }
                    else if is_integer_active {
                        if character.is_numeric() {
                            current_element.push(character);   
                        }
                        else if character == ' ' {
                            if part.parse::<i32>().is_ok() {
                                split_of_array.push(ArrayTypesEnum::Integer(part.parse::<i32>().unwrap()));
                                current_element = String::new();
                                dot_count = 0;
                                is_integer_active = false;
                                valid_for_next_element = false;
                            }
                            else if part.parse::<f32>().is_ok() {
                                split_of_array.push(ArrayTypesEnum::Float(part.parse::<f32>().unwrap()));
                                current_element = String::new();
                                dot_count = 0;
                                is_integer_active = false;
                                valid_for_next_element = false;
                            } else {
                                final_tokens.push(("ERROR_MESSAGE".to_string(), ValueEnum::String("NUMBER ELEMENTS OF AN ARRAY HAVE TO BE I32 OR F32!".to_string())));
                                break;
                            }
                        }
                        else if character == '.' {
                            if dot_count > 0 {
                                final_tokens.push(("ERROR_MESSAGE".to_string(), ValueEnum::String("THERE ARE TOO MANY DOTS IN THE NUMBER!".to_string())));
                                break;
                            } else {
                                current_element.push(character);
                                dot_count += 1;
                            }
                        } else {
                            final_tokens.push(("ERROR_MESSAGE".to_string(), ValueEnum::String("INVALID CHARACTER IN THE ARRAY!".to_string())));
                            break;
                        }
                    }
                }
            }
        
            final_tokens.push((token_classification[7].to_string(), ValueEnum::Array(split_of_array)));
        }
        // comment check
        else if part.chars().nth(0).unwrap() == '#' {
            if final_tokens.len() == 0 {
                final_tokens.push((token_classification[8].to_string(), ValueEnum::String(input.as_str()[1..].to_string())));
                return final_tokens;
            } else {
               final_tokens.push(("ERROR_MESSAGE".to_string(), ValueEnum::String("IT'S NOT ALLOWED TO PUT A COMMENT AFTER SOMETHING. ONE COMMENT TAKES ONE LINE!".to_string())));
               break;
            }
        }
        // variable/function name check
        else {
            let mut is_valid_name = true;
            for character in part.chars() {
                if !(allowed_variable_function_characters.contains(&character)) {
                    is_valid_name = false;
                }
            }
            if is_valid_name {
                final_tokens.push((token_classification[9].to_string(), ValueEnum::String(part.to_string())));
            } else {
                final_tokens.push(("ERROR_MESSAGE".to_string(), ValueEnum::String("VARIABLE/FUNCTION NAME INCLUDES INVALID CHARACTERS!".to_string())));
                break;
            }
        }
        
        i += 1;
    }    

    return final_tokens;
}
