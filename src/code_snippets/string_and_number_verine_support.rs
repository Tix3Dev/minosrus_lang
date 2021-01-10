fn main() {
    // first step: check if verine includes only right characters
    
    // let a = "LET A = | 1 + | 5 * 5 + | 6 * 6 | + 2  - 333 | |".to_string();
    let a = "LET A = | \"MAU\" + \"MOIN\" + \"LOL\" |".to_string();
    let a = a.as_str();
    
    let arithmetic_operators = vec![
        "+",
        "-",
        "*",
        "/",
        "**",
        "//"
    ];
    
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
    
    if a.contains("\"") {
        let mut verine_positions: Vec<usize> = vec![];
        for (index, character) in a.chars().enumerate() {
            // println!("{}, {}", index, character);
            if character == '|' {
                verine_positions.push(index);
            }
        }
        if verine_positions.len() != 2 {
            println!("TWO | EXPECTED (FOR STRING VERINE)");
        }
        
        let verine = a[verine_positions[0]+1..verine_positions[1]-1].trim().to_string() + " ";
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
                    println!("ERROR: INVALID CHARACTER INSIDE OF THE STRING!");
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
        
        let mut i = 0;
        while i < split_of_string_verine.len() {
            let part = &split_of_string_verine[i];
            
            if i % 2 == 0 {
                if !(part.chars().nth(0).unwrap() == '\"' && part.chars().rev().nth(0).unwrap() == '\"') {
                    println!("ERROR: ORDER OF STRING VERINE IS WRONG!");
                }   
            } else {
                if part != "+" {
                    println!("ERROR: ORDER OF STRING VERINE IS WRONG!");
                }
            }
            
            i += 1;
        }
        
        let mut final_result = String::new();
        for element in split_of_string_verine.iter().step_by(2) {
            final_result.push_str(&element[1..element.len()-1]);   
        }
        
        println!("final result: {:?}", final_result);
    }
    else if a.chars().into_iter().any(|c| c.is_numeric()) {
        let mut verine_positions: Vec<usize> = vec![];
        for (index, character) in a.chars().enumerate() {
            // println!("{}, {}", index, character);
            if character == '|' {
                verine_positions.push(index);
            }
        }
        if verine_positions.len() % 2 != 0 {
            println!("ERROR: EVERY | NEEDS A | !");
            return;
        }
        if a[verine_positions[0]+1..verine_positions[verine_positions.len()-1]].chars().into_iter().any(|character| !(arithmetic_operators.contains(&character.to_string().as_str()) || character.is_numeric() || character == '|' || character == ' ')) {
            println!("{}", &a[verine_positions[0]..verine_positions[verine_positions.len()-1]]);
            println!("ERROR: INVALID CHARACTER IN VERINE (NUMBER VERINE EXPECTED!))");
        }
        
        let mut last_pos = verine_positions.len() - (verine_positions.len() / 2);
        let mut last_result: i32 = 0;
        let mut current_slice = String::new();
        for i in (0..verine_positions.len() / 2).rev() {
            println!("{} {}", i, last_pos);
            println!("original: {}", a);
            
            if i == verine_positions.len() / 2 - 1 {
                current_slice = a[verine_positions[i]+1..verine_positions[last_pos]].to_string();   
            } else {
                println!("nr. 1: {}", a[verine_positions[i]+1..verine_positions[i+1]].to_string());
                println!("nr. 2: {}", last_result);
                println!("nr. 3: {}", a[verine_positions[last_pos-1]+1..verine_positions[last_pos]-1].to_string());
                current_slice = a[verine_positions[i]+1..verine_positions[i+1]].to_string() + &last_result.to_string() + &a[verine_positions[last_pos-1]+1..verine_positions[last_pos]-1].to_string();
            }
            println!("current: {}", current_slice);
            if current_slice.trim().is_empty() {
                println!("ERROR: VERINE IS EMPTY");
            }
            
            let mut last_character = "";
            last_result = 0;
            for (index, character) in current_slice.split_whitespace().collect::<Vec<&str>>().iter().enumerate() {
                if character.parse::<i32>().is_ok() {
                    if index == 0 {
                        last_character = character;
                        last_result += character.to_string().parse::<i32>().unwrap();
                    } 
                    else if arithmetic_operators.contains(&last_character) {
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
                        println!("1: ORDER WRONG");
                        break;
                    }
                }
                if arithmetic_operators.contains(&character) {
                    if index == 0 {
                        println!("CAN' START WITH OPERATOR!");
                    } else if index == a.len() - 1 {
                        println!("CAN'T END WITH OPERATOR!");
                    } else if last_character.parse::<i32>().is_ok() {
                        last_character = character;
                    } else {
                        println!("2: ORDER WRONG");
                        break;
                    }
                }
            }
            println!("current result: {}", last_result);
            
            last_pos += 1;
            println!("");
        }
        
        println!("final result: {}", last_result);
        
        let edited_line = a[0..verine_positions[0]].to_string() + &last_result.to_string() + &a[verine_positions[verine_positions.len()-1]+1..a.len()].to_string();
        println!("{:?}", edited_line);
    }
    // else if a.contains_a_predefined_name {
    // }
}
