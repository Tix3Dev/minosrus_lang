fn main() {
    // first step: check if verine includes only right characters
    
    let a = "LET A = | 1 + | 5 * 5 + | 6 * 6 | + 2  - 333 | |".to_string();
    // let a = "LET A = | 1 + 4 + 2 |".to_string();
    let a = a.as_str();
    
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
        
        let arithmetic_operators = vec![
            "+",
            "-",
            "*",
            "/",
            "**",
            "//"
        ];
        
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
