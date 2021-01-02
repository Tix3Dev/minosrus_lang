#[derive(Debug)]
pub enum ValueEnum {
    String(String),
    IntegerVector(Vec<i32>),
    StringVector(Vec<String>),
    TokenVector(Vec<(String, ValueEnum)>) // used for verines 
}

pub fn make_tokens(input: String) -> Vec<(String, ValueEnum)> {
    // final tokens that are returned stored here
    let mut final_tokens: Vec<(String, ValueEnum)> = Vec::new();

    // used for checking whether the inner part of a string is valid or not
    let allowed_string_inner_part_characters = vec![
        'A',
        'B',
        'C',
        'D',
        'E',
        'F',
        'G',
        'H',
        'I',
        'J',
        'K',
        'L',
        'M',
        'N',
        'O',
        'P',
        'Q',
        'R',
        'S',
        'T',
        'U',
        'V',
        'W',
        'X',
        'Y',
        'Z',
        ',',
        '.',
        ':',
        '!',
        '?',
    ];

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

    // used for the hashmap final_tokens -> classification of token
    let token_classification = vec![
        "PREDEFINED_NAME".to_string(),
        "OPERATOR".to_string(),
        "STRING".to_string(),
        "INTEGER".to_string(),
        "STRING_ARRAY".to_string(),
        "INTEGER_ARRAY".to_string(),
        "VERINE".to_string(),
        "COMMENT".to_string(),
        "VARIABLE/FUNCTION_NAME".to_string()
    ];
    
    // used to check whether a token is ... or not
    let predefined_names = vec![
        "LET".to_string(),
        "PRINT".to_string(),
        "FN".to_string(),
        "START".to_string(),
        "END".to_string(),
        "DO".to_string(),
        "IF".to_string(),
        "ELSE".to_string(),
        "WHILE".to_string(),
        "RESET".to_string(),
        "PUSH".to_string(),
        "POP".to_string(),
        "INSERT".to_string(),
        "REMOVE".to_string(),
        "GET".to_string(),
        "ONTO".to_string(),
        "FROM".to_string(),
        "INTO".to_string(),
        "AT".to_string(),
        "LEN".to_string()
    ];
    let operators = vec![
        "+".to_string(),
        "-".to_string(),
        "*".to_string(),
        "/".to_string(),
        "**".to_string(),
        "//".to_string(),
        "=".to_string(),
        "==".to_string(),
        "!=".to_string()
    ];

    // used for checking whether a name is valid or not
    let allowed_variable_function_characters = vec![
        'A',
        'B',
        'C',
        'D',
        'E',
        'F',
        'G',
        'H',
        'I',
        'J',
        'K',
        'L',
        'M',
        'N',
        'O',
        'P',
        'Q',
        'R',
        'S',
        'T',
        'U',
        'V',
        'W',
        'X',
        'Y',
        'Z',
        '_',
    ];

    let mut i = 0;
    while i < split_of_input.len() {
        let part = &split_of_input[i];
        // predefined name check
        if predefined_names.contains(part) {
            final_tokens.push((token_classification[0].to_string(), ValueEnum::String(part.to_string())));
        }
        // operator check
        else if operators.contains(part) {
            final_tokens.push((token_classification[1].to_string(), ValueEnum::String(part.to_string())));
        }
        // string check
        else if part.chars().nth(0).unwrap() == '\"' && part.chars().rev().nth(0).unwrap() == '\"' {
            final_tokens.push((token_classification[2].to_string(), ValueEnum::String(part.to_string())));
        }
        // integer check
        else if part.parse::<i32>().is_ok() {
            final_tokens.push((token_classification[3].to_string(), ValueEnum::String(part.to_string())));
        }
        // array check
        else if part.chars().nth(0).unwrap() == '[' && part.chars().rev().nth(0).unwrap() == ']' {
            let mut array = part.clone();

            // remove [ and ]
            array.remove(0);
            array.remove(array.len() - 1);

            // check if array is empty
            if array.trim().is_empty() {
                final_tokens.push(("ERROR_MESSAGE".to_string(), ValueEnum::String("ARRAY IS EMPTY!".to_string()))); 
                break;
            }

            // string elements
            if part.contains("\"") {
                // check if there is a comma at the beginning or at the end - otherwise wrong error message would occur
                if array.trim().chars().nth(0).unwrap() == ',' {
                    final_tokens.push(("ERROR_MESSAGE".to_string(), ValueEnum::String("THERE IS A COMMA AT WRONG PLACE IN THE ARRAY!".to_string())));
                    break;
                }
                if array.trim().chars().rev().nth(0).unwrap() == ',' {
                    final_tokens.push(("ERROR_MESSAGE".to_string(), ValueEnum::String("THERE IS A COMMA AT WRONG PLACE IN THE ARRAY!".to_string())));
                    break;
                }

                let mut split_of_array: Vec<String> = vec![];
                let mut current_element = String::new();
                let mut first_quote = true;
                let mut comma_count = 1;

                for character in array.chars() {
                    if character == '"' {
                        if comma_count == 1 {
                            if first_quote {
                                current_element.push(character);
                                first_quote = false;
                            } else {
                                current_element.push(character);
                                split_of_array.push(current_element);
                                current_element = String::new();
                                comma_count = 0;
                                first_quote = true;
                            }
                        } else if comma_count == 0 {
                            final_tokens.push(("ERROR_MESSAGE".to_string(), ValueEnum::String("COMMA BETWEEN ELEMENTS OF ARRAY MISSING!".to_string())));
                            break;
                        } else if comma_count > 1 {
                            final_tokens.push(("ERROR_MESSAG".to_string(), ValueEnum::String("TOO MANY COMMAS BETWEEN ELEMENTS OF ARRAY!".to_string())));
                            break;
                        }
                    } else {
                        if !(first_quote) {
                            if !(allowed_string_inner_part_characters.contains(&character)) {
                                final_tokens.push(("ERROR_MESSAGE".to_string(), ValueEnum::String("INVALID CHARACTER INSIDE OF THE STRING!".to_string())));
                                break;
                        }
                            current_element.push(character);
                        } else if character == ',' {
                            comma_count += 1;
                        } else if character != ' ' {
                            final_tokens.push(("ERROR_MESSAGE".to_string(), ValueEnum::String("INVALID CHARACTER IN ARRAY! MAYBE THERE A ARE QUOTES MISSING?".to_string())));
                            break;
                        }
                    }
                }

                final_tokens.push((token_classification[4].to_string(), ValueEnum::StringVector(split_of_array)));
            }
            // integer elements
            else if part.chars().into_iter().any(|c| c.is_numeric()) {
                if array.trim().chars().nth(0).unwrap() == ',' {
                    final_tokens.push(("ERROR_MESSAGE".to_string(), ValueEnum::String("THERE IS A COMMA AT WRONG PLACE IN THE ARRAY!".to_string())));
                    break;

                }
                else if array.trim().chars().rev().nth(0).unwrap() == ',' {
                    final_tokens.push(("ERROR_MESSAGE".to_string(), ValueEnum::String("THERE IS A COMMA AT WRONG PLACE IN THE ARRAY!".to_string())));
                    break;

                } else {
                    array.push(',');
                }
                
                let mut split_of_array: Vec<i32> = vec![];
                let mut current_number = String::new();
                let mut number_started = false;
                let mut valid_element = false;
                let mut comma_count = 1;
                
                for character in array.chars() {
                    if character.is_numeric() {
                        if comma_count == 1 {
                            if !(number_started) {
                                current_number.push(character);
                                number_started = true;
                                valid_element = true;
                            }
                            else if number_started && valid_element {
                                current_number.push(character);
                            }
                            else if number_started && !(valid_element) {
                                final_tokens.push(("ERROR_MESSAGE".to_string(), ValueEnum::String("INVALID FORMATION OF ELEMENTS OF ARRAY!".to_string())));
                                break;
                            }   
                        } else if comma_count == 0 {
                            final_tokens.push(("ERROR_MESSAGE".to_string(), ValueEnum::String("COMMA BETWEEN ELEMENTS OF ARRAY MISSING!".to_string())));
                            break;
                        } else if comma_count > 1 {
                            final_tokens.push(("ERROR_MESSAGE".to_string(), ValueEnum::String("TOO MANY COMMAS BETWEEN ELEMENTS OF ARRAY!".to_string())));
                            break;
                        }
                    }
                    else if character == ' ' {
                        if number_started {
                            valid_element = false;
                        }
                    }
                    else if character == ',' {
                        match current_number.parse::<i32>() {
                            Ok(number) => {
                                split_of_array.push(number);
                                current_number = String::new();
                                number_started = false;
                                valid_element = false;
                                comma_count = 0;
                            },
                            Err(_) => {
                                final_tokens.push(("ERROR_MESSAGE".to_string(), ValueEnum::String("INTEGER ELEMENTS OF AN ARRAY HAVE TO BE I32!".to_string())));
                                break;
                            }
                        }
                        comma_count += 1;
                    } else {
                        final_tokens.push(("ERROR_MESSAGE".to_string(), ValueEnum::String("INVALID CHARACTER IN ARRAY!".to_string())));
                    }
                }

                final_tokens.push((token_classification[5].to_string(), ValueEnum::IntegerVector(split_of_array)));
            } else {
                final_tokens.push(("ERROR_MESSAGE".to_string(), ValueEnum::String("ELEMENTS OF ARRAY DON'T SEEM TO BE STRINGS OR INTEGERS!".to_string())));
                break;
            }
        }
        // verine check
        else if part.chars().nth(0).unwrap() == '|' && part.chars().rev().nth(0).unwrap() == '|' {
            let mut verine = part.clone();

            // remove | and |
            verine.remove(0);
            verine.remove(verine.len() - 1);

            // check if verine is empty
            if verine.trim().is_empty() {
                final_tokens.push(("ERROR_MESSAGE".to_string(), ValueEnum::String("VERINE IS EMPTY!".to_string()))); 
                break;
            }
            
            let split_of_verine: Vec<(String, ValueEnum)> = make_tokens(verine);

            if let Some((_, value)) = split_of_verine.iter().find(|(key, _)| key == &"ERROR_MESSAGE") {
                match value {
                    ValueEnum::String(v) => {
                        let error_message = "IN VERINE: ".to_string() + v;
                        final_tokens.push(("ERROR_MESSAGE".to_string(), ValueEnum::String(error_message.to_string())));
                        break;
                    },
                    ValueEnum::IntegerVector(_v) => (),
                    ValueEnum::StringVector(_v) => (),
                    ValueEnum::TokenVector(_v) => ()
                }
            }

            final_tokens.push((token_classification[6].to_string(), ValueEnum::TokenVector(split_of_verine)));
        }
        // comment check
        else if part.chars().nth(0).unwrap() == '#' {
            if final_tokens.len() == 0 {
                final_tokens.push((token_classification[7].to_string(), ValueEnum::String(input.as_str()[1..].to_string())));
                return final_tokens;
            } else  {
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
                final_tokens.push((token_classification[8].to_string(), ValueEnum::String(part.to_string())));
            } else {
                final_tokens.push(("ERROR_MESSAGE".to_string(), ValueEnum::String("VARIABLE/FUNCTION NAME INCLUDES INVALID CHARACTERS!".to_string())));
                break;
            }
        }
        
        i += 1;
    }    

    return final_tokens;
}
