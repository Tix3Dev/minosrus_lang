fn main() {
    let arithmetic_operators = vec![
        "+".to_string(),
        "-".to_string(),
        "*".to_string(),
        "/".to_string(),
        "**".to_string(),
    ];
    
    let current_slice = "5*5";
    let mut last_character = ' ';
    let mut current_op = " ";
    let mut current_num = 0;
    let mut num_pos = 1;
    let mut last_result = 0;
    let mut allowed_one_more_asterisk = true;
    for (index, c) in current_slice.chars().enumerate() {
        if index == 0 {
            if c.is_numeric() {
                current_num += c.to_string().parse::<i32>().unwrap();
                num_pos *= 10;
                last_result += c.to_string().parse::<i32>().unwrap();
            } else if arithmetic_operators.contains(&c.to_string()) {
                println!("Can't start with operator");
            } else {
                println!("Invalid character in verine");
            }
        } else {
            if c.is_numeric() {
                if last_character.is_numeric() {
                    current_num += c.to_string().parse::<i32>().unwrap() * num_pos;
                    num_pos *= 10;
                } else {
                    current_num += c.to_string().parse::<i32>().unwrap();
                    num_pos *= 10;
                }
            } else if arithmetic_operators.contains(&c.to_string()) {
                if index == current_slice.len() - 1 {
                    println!("Can't end with operator");
                }
                if c == '*' {
                    if !allowed_one_more_asterisk {
                        println!("Invalid operator!");
                    }
                    if last_character == '*' {
                        allowed_one_more_asterisk = false;
                        current_op = "**".to_string();
                    } else {
                        allowed_one_more_asterisk = true;
                        current_op = 
                        continue;
                    }
                } else {
                    last_character = last_character.to_string();
                }
                
                match last_character {
                    "+" => last_result += current_num.to_string().parse::<i32>().unwrap(),
                    "-" => last_result -= current_num.to_string().parse::<i32>().unwrap(),
                    "*" => last_result *= current_num.to_string().parse::<i32>().unwrap(),
                    "/" => last_result /= current_num.to_string().parse::<i32>().unwrap(),
                    "**" => last_result = last_result.pow(c.parse::<u32>().unwrap()),
                    _ => ()
                }
                current_num = 1;
                num_pos = 1;
            } else {
                println!("Invalid character in verine");
            }
        }
        last_character = c;
    }   
}
