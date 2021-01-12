fn main() {
    //
    let arithmetic_operators = vec![
        "+".to_string(),
        "-".to_string(),
        "*".to_string(),
        "/".to_string(),
        "**".to_string()
    ];
    
    let current_slice = "5 *5 ";
    //
    
    let mut last_character = ' ';
    let mut current_operator = String::new();
    let mut current_number = 0;
    let mut number_position = 1;
    let mut last_result = 0;
    let mut allowed_one_more_asterisk = true;
    for (index, character) in current_slice.chars().enumerate() {
        if index == 0 {
            if character.is_numeric() {
                current_number += character.to_string().parse::<i32>().unwrap();
                number_position *= 10;
                last_result += character.to_string().parse::<i32>().unwrap();
            } else if arithmetic_operators.contains(&character.to_string()) {
                final_tokens.push(("ERROR_MESSAGE".to_string(), ValueEnum::String("NUMBER VERINE CAN'T START WITH OPERATOR!".to_string())));
                return final_tokens;
            } else {
                final_tokens.push(("ERROR_MESSAGE".to_string(), ValueEnum::String("INVALID CHARACTER IN NUMBER VERINE!".to_string())));
                return final_tokens;
            }
        } else {
            if character.is_numeric() {
                if last_character.is_numeric() {
                    current_number += character.to_string().parse::<i32>().unwrap() * number_position;
                    number_position *= 10;
                } else {
                    current_number += character.to_string().parse::<i32>().unwrap();
                    number_position *= 10;
                }
            } else if arithmetic_operators.contains(&character.to_string()) {
                if index == current_slice.len() - 1 {
                    final_tokens.push(("ERROR_MESSAGE".to_string(), ValueEnum::String("NUMBER VERINE CAN'T END WITH AN OPERATOR!".to_string())));
                    return final_tokens;
                }
                if arithmetic_operators.contains(&last_character.to_string()) {
                    if last_character != '*' && !allowed_one_more_asterisk {
                        final_tokens.push(("ERROR_MESSAGE".to_string(), ValueEnum::String("INVALID CHARACTER IN NUMBER VERINE!".to_string()))); // actually invalid OPERATOR but there are also other possibilities
                        return final_tokens;    
                    }
                    else if last_character == '*' && character == '*' {
                        allowed_one_more_asterisk = false;
                    }
                    else if last_character != '*' && character == '*' {
                        allowed_one_more_asterisk = true;
                    }
                } else {
                    current_operator = String::new();
                }
                current_operator += &character.to_string();
                
                match current_operator.as_str() {
                    "+" => last_result += current_number.to_string().parse::<i32>().unwrap(),
                    "-" => last_result -= current_number.to_string().parse::<i32>().unwrap(),
                    "*" => last_result *= current_number.to_string().parse::<i32>().unwrap(),
                    "/" => last_result /= current_number.to_string().parse::<i32>().unwrap(),
                    "**" => last_result = last_result.pow(character.to_string().parse::<u32>().unwrap()),
                    _ => ()
                }
                current_number = 1;
                number_position = 1;
                current_operator = String::new();
            } 
            else if character != ' ' {
                final_tokens.push(("ERROR_MESSAGE".to_string(), ValueEnum::String("INVALID CHARACTER IN NUMBER VERINE!".to_string())));
                return final_tokens;
            }
        }
        last_character = character;
    }
    println!("{}", last_result);
}
