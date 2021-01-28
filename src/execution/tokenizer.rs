#[derive(Debug, Clone)]
pub enum ValueEnum {
    String(String),
    Integer(i32),
    IntegerArray(Vec<i32>),
    StringArray(Vec<String>)
}

pub fn make_tokens(mut input: String) -> Vec<(String, ValueEnum)> {
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
        "STRING_ARRAY".to_string(),
        "INTEGER_ARRAY".to_string(),
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
        '?'
    ];
   
    // check for one verine and if one exists replace input
    if input.contains('|') {
        let input_as_str = input.as_str();
        
        // save | positions
        let mut verine_positions: Vec<usize> = vec![];
        for (index, character) in input_as_str.chars().enumerate() {
            if character == '|' {
                verine_positions.push(index);
            }
        }
        // needed error checks for further tokenizing
        if input_as_str.contains("\"") && verine_positions.len() != 2 {
            final_tokens.push(("ERROR_MESSAGE".to_string(), ValueEnum::String("TWO | EXPECTED (FOR STRING VERINE)!".to_string())));
            return final_tokens;
        }
        if verine_positions.len() % 2 != 0 {
            final_tokens.push(("ERROR_MESSAGE".to_string(), ValueEnum::String("EVERY | NEEDS A | !".to_string())));
            return final_tokens;
        }
        if input_as_str[verine_positions[0]+1..verine_positions[1]].trim().is_empty() {
            final_tokens.push(("ERROR_MESSAGE".to_string(), ValueEnum::String("VERINE IS EMPTY!".to_string())));
            return final_tokens;
        }

        // predefined name verine
        if input[verine_positions[0]+1..verine_positions[verine_positions.len()-1]].split_whitespace().any(|c| predefined_names.contains(&c.to_string())) {
            println!("not implemented yet: predefined_verine!");
        }
        // string verine
        else if input_as_str.contains("\"") {
            // make tokens
            let verine = input_as_str[verine_positions[0]+1..verine_positions[1]-1].trim().to_string() + " ";
            let mut split_of_string_verine: Vec<String> = vec![];
            let mut current_token = String::new();
            let mut last_character = 'a';
            let mut string_started = false;
            
            for character in verine.chars() {
                if character == ' ' {
                    if current_token.starts_with('"') && !current_token.ends_with('"') {
                        // space belongs to the string
                        current_token.push(character);
                    }
                    else {
                        // end of token
                        if last_character != ' ' {
                            split_of_string_verine.push(current_token);
                            current_token = String::new();
                        }
                    }
                } else {
                    if character == '"' {
                        if current_token.starts_with('"') {
                            string_started = false;
                        } 
                    }
        
                    if !(allowed_string_inner_part_characters.contains(&character)) && string_started {
                        final_tokens.push(("ERROR_MESSAGE".to_string(), ValueEnum::String("INVALID CHARACTER INSIDE OF THE STRING (IN VERINE)!".to_string())));
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
            
            // check if tokens are in right order
            let mut i = 0;
            while i < split_of_string_verine.len() {
                let part = &split_of_string_verine[i];
                
                if i % 2 == 0 {
                    if !(part.chars().nth(0).unwrap() == '\"' && part.chars().rev().nth(0).unwrap() == '\"') {
                        final_tokens.push(("ERROR_MESSAGE".to_string(), ValueEnum::String("ORDER IN STRING VERINE IS WRONG!".to_string())));
                        return final_tokens;
                    }   
                } else {
                    if part != "+" {
                        final_tokens.push(("ERROR_MESSAGE".to_string(), ValueEnum::String("ORDER IN STRING VERINE IS WRONG!".to_string())));
                        return final_tokens;
                    }
                }
                
                i += 1;
            }
            
            let mut final_result = String::new();
            for element in split_of_string_verine.iter().step_by(2) {
                final_result.push_str(&element[1..element.len()-1]);   
            }
            
            // all went right -> change input
            input = input_as_str[0..verine_positions[0]].to_string() + &final_result.to_string() + &input_as_str[verine_positions[verine_positions.len()-1]+1..input_as_str.len()].to_string();
        }
        // number verine
        else if input_as_str.chars().into_iter().any(|c| c.is_numeric()) {      
            // check for invalid characters
            if input_as_str[verine_positions[0]+1..verine_positions[verine_positions.len()-1]].chars().into_iter().any(|character| !(arithmetic_operators.contains(&character.to_string()) || character.is_numeric() || character == '|' || character == ' ')) {
                final_tokens.push(("ERROR_MESSAGE".to_string(), ValueEnum::String("INVALID CHARACTER IN NUMBER VERINE!".to_string())));
                return final_tokens;
            }
            
            // go over each verine (start with the innermost) and caculate it's value; add each value to the final result (last_result)
            let mut last_pos = verine_positions.len() - (verine_positions.len() / 2);
            let mut last_result: i32 = 0;
            for i in (0..verine_positions.len() / 2).rev() {
                let current_slice = if i == verine_positions.len() / 2 - 1 {
                    input_as_str[verine_positions[i]+1..verine_positions[last_pos]].to_string()   
                } else {
                    input_as_str[verine_positions[i]+1..verine_positions[i+1]].to_string() + &last_result.to_string() + &input_as_str[verine_positions[last_pos-1]+1..verine_positions[last_pos]-1].to_string()
                };
                
                let mut last_character = "";
                last_result = 0;
                for (index, character) in current_slice.split_whitespace().collect::<Vec<&str>>().iter().enumerate() {
                    if character.parse::<i32>().is_ok() {
                        if index == 0 {
                            last_character = character;
                            last_result += character.to_string().parse::<i32>().unwrap();
                        } 
                        else if arithmetic_operators.contains(&last_character.to_string()) {
                            match last_character {
                                "+" => last_result += character.parse::<i32>().unwrap(),
                                "-" => last_result -= character.parse::<i32>().unwrap(),
                                "*" => last_result *= character.parse::<i32>().unwrap(),
                                "/" => last_result /= character.parse::<i32>().unwrap(),
                                "**" => last_result = last_result.pow(character.parse::<u32>().unwrap()),
                                _ => ()
                            }
                            last_character = character;
                        } else { 
                            final_tokens.push(("ERROR_MESSAGE".to_string(), ValueEnum::String("ORDER IN NUMBER VERINE IS WRONG!".to_string())));
                            return final_tokens;
                        }
                    }
                    if arithmetic_operators.contains(&character.to_string()) {
                        if index == 0 {
                            final_tokens.push(("ERROR_MESSAGE".to_string(), ValueEnum::String("NUMBER VERINE CAN'T START WITH AN OPERATOR!".to_string())));
                            return final_tokens;
                        } else if index == input_as_str.len() - 1 {
                            final_tokens.push(("ERROR_MESSAGE".to_string(), ValueEnum::String("NUMBER VERINE CAN'T END WITH AN OPERATOR!".to_string())));
                            return final_tokens;
                        } else if last_character.parse::<i32>().is_ok() {
                            last_character = character;
                        } else {
                            final_tokens.push(("ERROR_MESSAGE".to_string(), ValueEnum::String("ORDER IN NUMBER VERINE IS WRONG!".to_string())));
                            return final_tokens;
                        }
                    }
                }
                
                last_pos += 1;
            }
            
            // all went right -> change input
            input = input_as_str[0..verine_positions[0]].to_string() + &last_result.to_string() + &input_as_str[verine_positions[verine_positions.len()-1]+1..input_as_str.len()].to_string();
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
        else if part.parse::<i32>().is_ok() {
            final_tokens.push((token_classification[5].to_string(), ValueEnum::Integer(part.parse::<i32>().unwrap())));
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

                final_tokens.push((token_classification[6].to_string(), ValueEnum::StringArray(split_of_array)));
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
                let mut current_position = 0;
                
                for character in array.chars() {
                    if character == '-' {
                        if !(array.chars().nth(current_position+1).unwrap().is_numeric()) {
                            final_tokens.push(("ERROR_MESSAGE".to_string(), ValueEnum::String("INVALID CHARACTER IN ARRAY!".to_string())));
                        }
                        else if current_position != 0 {
                            if array.chars().nth(current_position-1).unwrap().is_numeric() {
                                final_tokens.push(("ERROR_MESSAGE".to_string(), ValueEnum::String("INVALID CHARACTER IN ARRAY!".to_string())));
                            }
                        } else {
                            current_number.push(character);
                            number_started = true;
                            valid_element = true;
                        }
                    }
                    else if character.is_numeric() {
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

                    current_position += 1;
                }

                final_tokens.push((token_classification[7].to_string(), ValueEnum::IntegerArray(split_of_array)));
            } else {
                final_tokens.push(("ERROR_MESSAGE".to_string(), ValueEnum::String("ELEMENTS OF ARRAY DON'T SEEM TO BE STRINGS OR INTEGERS!".to_string())));
                break;
            }
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
