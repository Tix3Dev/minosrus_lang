/*
 *   This file is part of an interpreter for a programming language called minosrus_lang
 *   Copyright (C) 2020-2021  Yves Vollmeier
 *
 *    This program is free software: you can redistribute it and/or modify
 *    it under the terms of the GNU General Public License as published by
 *    the Free Software Foundation, either version 3 of the License, or
 *    (at your option) any later version.
 *
 *    This program is distributed in the hope that it will be useful,
 *    but WITHOUT ANY WARRANTY; without even the implied warranty of
 *    MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 *   GNU General Public License for more details.
 *
 *   You should have received a copy of the GNU General Public License
 *   along with this program.  If not, see <https://www.gnu.org/licenses/>.
 *
 */


use crate::ExecData;
use crate::tokenizer;

use std::collections::HashMap;
use std::process;
use crate::verine_expression::{VerineTokenizer, VerineValue, Token};
use crate::tokenizer::ValueEnum;
use crate::tokenizer::ArrayTypesEnum;

#[derive(Clone)]
enum OrderEnum {
    SingleOption(Vec<&'static str>),
    MultipleOptions(Vec<Vec<&'static str>>)
}

fn add_indentation(indentation: &mut String) {
    indentation.push_str("    ");
}

fn subtract_indentation(indentation: &mut String) {
    *indentation = indentation[4..].to_string();
}

fn is_key_and_value_order_right(line: Vec<(String, tokenizer::ValueEnum)>) -> Result<(), String> {
    let clean = match &line[0].1 {
        tokenizer::ValueEnum::String(clean) => clean,
        _ => unreachable!("SOMEHOW THIS SHOULDN'T BE PRINTED!")
    };

    let predefined_name_order = include!("predefined_name_order.in");
    let value = predefined_name_order.get(&clean.as_str()).ok_or(format!("EXECUTION ERROR: '{}' IS NEVER AT THE BEGINNING!", clean))?;

    // check if the key of the first token has multiple options
    match value {
        OrderEnum::SingleOption(v) => {
            // length check - otherwise the indexing would panic
            if line.len() < v.len() {
                return Err(format!("EXECUTION ERROR: THERE ARE LESS TOKENS THAN '{}' NEEDS!", clean));
            }
            if line.len() > v.len() {
                return Err(format!("EXECUTION ERROR: THERE ARE MORE TOKENS THAN '{}' NEEDS!", clean));
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
                if let tokenizer::ValueEnum::String(tc) = &line[element_nr].1 {
                    if tc != v[element_nr].split(':').nth(1).unwrap() && v[element_nr].split(':').nth(1).unwrap() != "?" {
                        is_value_order_right = false;
                        break;
                    }
                }
            }
            if !(is_key_order_right) {
                return Err(format!("EXECUTION ERROR: KEY ORDER FOR '{}' ISN'T RIGHT!", clean)); 
            }
            if !(is_value_order_right) {
                return Err(format!("EXECUTION ERROR: VALUE ORDER FOR '{}' ISN'T RIGHT!", clean)); 
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
                return Err(format!("EXECUTION ERROR: THERE ARE LESS TOKENS THAN '{}' NEEDS!", clean));
            }
            if too_many_tokens {
                return Err(format!("EXECUTION ERROR: THERE ARE MORE TOKENS THAN '{}' NEEDS!", clean));
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
                    if let tokenizer::ValueEnum::String(tc) = &line[element_nr].1 {
                        if tc != v[possibility_nr][element_nr].split(':').nth(1).unwrap() && v[possibility_nr][element_nr].split(':').nth(1).unwrap() != "?"{
                            is_current_value_order_right = false;
                        }
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
                return Err(format!("EXECUTION ERROR: KEY ORDER FOR '{}' ISN'T RIGHT!", clean));
            }
            if !(is_one_value_order_right) {
                return Err(format!("EXECUTION ERROR: VALUE ORDER FOR '{}' ISN'T RIGHT!", clean));
            }
        }
    }

    Ok(())
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
) -> Result<Vec<Vec<(String, tokenizer::ValueEnum)>>, String> {
    let mut block_code_clone = block_code.clone();

    if block_code[0][1].0 == "VARIABLE/FUNCTION_NAME" {
        let variable_name = match &block_code[0][1].1 {
            tokenizer::ValueEnum::String(variable_name) => variable_name,
            _ => unreachable!("SOMEHOW THIS SHOULDN'T BE PRINTED!")
        };

        let value_of_variable = global_variables.get(variable_name).ok_or(format!("EXECUTION ERROR: THERE IS NO VARIABLE CALLED {}!", variable_name))?;

        match value_of_variable {
            tokenizer::ValueEnum::String(v) => {
                block_code_clone[0][1].0 = "STRING".to_string();
                block_code_clone[0][1].1 = tokenizer::ValueEnum::String(v.to_string());
            },
            tokenizer::ValueEnum::Integer(v) => {
                block_code_clone[0][1].0 = "INTEGER".to_string();
                block_code_clone[0][1].1 = tokenizer::ValueEnum::Integer(*v);
            },
            _ => *error = "EXECUTION ERROR: FIRST VARIABLE HAS TO BE A STRING OR INTEGER!".to_string()
        }
    }
    if block_code[0][3].0 == "VARIABLE/FUNCTION_NAME" {
        let variable_name = match &block_code[0][3].1 {
            tokenizer::ValueEnum::String(variable_name) => variable_name,
            _ => unreachable!("SOMEHOW THIS SHOULDN'T BE PRINTED!")
        };

        let value_of_variable = global_variables.get(variable_name).ok_or(format!("EXECUTION ERROR: THERE IS NO VARIABLE CALLED {}!", variable_name))?;

        match value_of_variable {
            tokenizer::ValueEnum::String(v) => {
                block_code_clone[0][3].0 = "STRING".to_string();
                block_code_clone[0][3].1 = tokenizer::ValueEnum::String(v.to_string());
            },
            tokenizer::ValueEnum::Integer(v) => {
                block_code_clone[0][3].0 = "INTEGER".to_string();
                block_code_clone[0][3].1 = tokenizer::ValueEnum::Integer(*v);
            },
            _ => *error = "EXECUTION ERROR: SECOND VARIABLE HAS TO BE A STRING OR INTEGER!".to_string()
        }
    }

    Ok(block_code_clone)
}

impl ExecData {
    fn execute_block_code(&mut self, block_code: Vec<Vec<(String, tokenizer::ValueEnum)>>, called_from_block: bool) -> Result<(), String> {
        self.block_code.clear();

        for line in block_code {
            let return_of_execution = self.exec(line, called_from_block);
            if let Err(e) = return_of_execution {
                return Err(e);
            }
        }

        Ok(())
    }

    pub fn exec(&mut self, token_collection: Vec<(String, tokenizer::ValueEnum)>, called_from_while: bool) -> Result<(), String> {
        let mut token_collection = token_collection.clone();
        // debugging purpose
        println!("token_collection: {:?}", token_collection);

        // check for syntax errors
        if let Some((_, value)) = token_collection.iter().find(|(key, _)| key == &"ERROR_MESSAGE") {
            match value {
                tokenizer::ValueEnum::String(v) => {
                    return Err(format!("SYNTAX ERROR: {}", v));
                },
                _ => unreachable!("SOMEHOW THIS SHOULDN'T BE PRINTED!")
            }
        }
        // check for comments -> just make a newline
        if let Some((_, value)) = token_collection.iter().find(|(key, _)| key == &"COMMENT") {
            match value {
                tokenizer::ValueEnum::String(_v) => {
                    return Ok(());
                },
                _ => unreachable!("SOMEHOW THIS SHOULDN'T BE PRINTED!")
            }
        }
        // check for reset
        match &token_collection[0].1 {
            tokenizer::ValueEnum::String(v) => {
                if v == "RESET" && token_collection.len() == 1 {
                    self.global_variables.clear();
                    return Ok(());
                }
            },
            _ => unreachable!("SOMEHOW THIS SHOULDN'T BE PRINTED!")
        }

        let mut saved_verines = HashMap::<usize, Vec<Token>>::new();

        if called_from_while || self.indentation.is_empty() {
            for (i, (key, token)) in &mut token_collection.iter_mut().enumerate() {
                if let ValueEnum::Verine(verine) = token {
                    saved_verines.insert(i, verine.clone());

                    let verine = VerineTokenizer::evaluate(verine.clone(), &self.global_variables);

                    let push_error = |message: &str| {
                        return Err(format!("EXECUTION ERROR: {}", message));
                    };

                    match verine {
                        Ok(verine) => {
                            match verine {
                                VerineValue::String(s) => {
                                    *key = "STRING".to_string();
                                    *token = ValueEnum::String(s);
                                },
                                VerineValue::Integer(int) => {
                                    *key = "INTEGER".to_string();
                                    *token = ValueEnum::Integer(int);
                                },
                                VerineValue::Float(float) => {
                                    *key = "FLOAT".to_string();
                                    *token = ValueEnum::Float(float);
                                },
                            }
                        }
                        Err(error) => {
                            use crate::verine_expression::VerineTokenizerError::*;
                            return match error {
                                StdInError => push_error("PROBLEMS READING USER INPUT!"),
                                VariableNotFound(var) => push_error(&format!("THERE IS NO VARIABLE CALLED {}!", var)),
                                NumberNotAnInteger(var) => push_error(&format!("'{}' IS NOT A INTEGER!", var)),
                                InvalidOperands => push_error("INVALID OPERANDS!"),
                                InvalidIndex(i) => push_error(&format!("'{}' IS NOT A VALID INDEX!", i)),
                                IndexOutOfBounds => push_error("INDEX IS OUT OF BOUNDS!"),
                                TypeNotIndexable => push_error("TYPE IS NOT INDEXABLE!"),
                                TypeHasNoLength => push_error("TYPE HAS NO LENGTH!"),
                                DivisionByZero => push_error("CAN'T DIVIDE BY ZERO!"),
                                UnsupportedReturnType => push_error("VERINE RETURN TYPE IS NOT SUPPORTED!"),
                                InvalidExpression => push_error("INVALID VERINE EXPRESSION!"),
                                NumberNotAFloat(var) => push_error(&format!("'{}' IS NOT A FLOAT!", var)),
                                _ => unreachable!("SOMEHOW THIS SHOULDN'T BE PRINTED!")
                            }
                        }
                    }
                }
            }
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

        let predefined_name_order = include!("predefined_name_order.in");

        // check for code block stuff
        if self.indentation.to_string() != "".to_string() {
            let v = match &token_collection[0].1 {
                tokenizer::ValueEnum::String(v) => v,
                _ => unreachable!("SOMEHOW THIS SHOULDN'T BE PRINTED!")
            };

            if v == "IF" || v == "WHILE" {
                add_indentation(&mut self.indentation);
            }
            else if v == "FN" {
                return Err("EXECUTION ERROR: FUNCTIONS CAN'T BE INSIDE OF OTHER CODE BLOCKS!".to_string());
            }
            else if v == "END" && token_collection.len() == 1 {
                subtract_indentation(&mut self.indentation);
                if self.indentation.to_string() == "".to_string() {
                    if self.current_block_type.0 == "normal" {
                        let first_predefined_name = match &self.block_code[0][0].1 {
                            tokenizer::ValueEnum::String(first_predefined_name) => first_predefined_name,
                            _ => unreachable!("SOMEHOW THIS SHOULDN'T BE PRINTED!")
                        };

                        if first_predefined_name == "IF" {
                            let operator = match &self.block_code[0][2].1 {
                                tokenizer::ValueEnum::String(operator) => operator,
                                _ => unreachable!("SOMEHOW THIS SHOULDN'T BE PRINTED!")
                            };

                            let mut else_part: Vec<Vec<(String, tokenizer::ValueEnum)>> = vec![];

                            let mut is_there_elif_block = false;
                            let mut is_elif_block_true = false;
                            let mut where_are_elif_blocks = vec![];
                            let mut where_is_right_elif_block = 0;

                            let mut is_there_else_block = false;
                            let mut where_is_else_block = 0;

                            for (line_position, line) in self.block_code.iter().enumerate() {
                                let first_token = match &line[0].1 {
                                    tokenizer::ValueEnum::String(first_token) => first_token,
                                    _ => unreachable!("SOMEHOW THIS SHOULDN'T BE PRINTED!")
                                };

                                if first_token == "ELIF" {
                                    is_there_elif_block = true;
                                    where_are_elif_blocks.push(line_position);

                                    // check key and value order
                                    let return_of_check = is_key_and_value_order_right(line.to_vec());

                                    match return_of_check {
                                        Ok(_) => (),
                                        Err(e) => return Err(format!("{}", e))
                                    }

                                    // evaluate left and right side of elif
                                    let mut line_clone = line.clone();

                                    if line[1].0 == "VARIABLE/FUNCTION_NAME" {
                                        let variable_name = match &line[1].1 {
                                            tokenizer::ValueEnum::String(variable_name) => variable_name,
                                            _ => unreachable!("SOMEHOW THIS SHOULDN'T BE PRINTED!")
                                        };

                                        let value_of_variable = self.global_variables.get(variable_name).ok_or(format!("EXECUTION ERROR: THERE IS NO VARIABLE CALLED {}!", variable_name))?;

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
                                                return Err("EXECUTION ERROR: FIRST VARIABLE HAS TO BE A STRING OR INTEGER!".to_string());
                                            }
                                        }
                                    }
                                    if line[3].0 == "VARIABLE/FUNCTION_NAME" {
                                        let variable_name = match &line[3].1 {
                                            tokenizer::ValueEnum::String(variable_name) => variable_name,
                                            _ => unreachable!("SOMEHOW THIS SHOULDN'T BE PRINTED!")
                                        };

                                        let value_of_variable = self.global_variables.get(variable_name).ok_or(format!("EXECUTION ERROR: THERE IS NO VARIABLE CALLED {}!", variable_name))?;

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
                                                return Err("EXECUTION ERROR: SECOND VARIABLE HAS TO BE A STRING OR INTEGER!".to_string());
                                            }
                                        }
                                    }

                                    // check if elif condition is true
                                    match &line_clone[2].1 {
                                        tokenizer::ValueEnum::String(elif_operator) => {
                                            if check_block_code_condition(elif_operator.to_string(), vec![line_clone]) {
                                                is_elif_block_true = true;
                                                where_is_right_elif_block = where_are_elif_blocks.len() - 1;
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
                                        return Err("EXECUTION ERROR: THERE ARE MORE TOKENS THAN ELSE NEEDS!".to_string());
                                    }
                                }
                            }
                            let if_part = if is_there_elif_block && is_there_else_block {
                                else_part = self.block_code[where_is_else_block+1..].to_vec();
                                self.block_code[1..where_are_elif_blocks[0]].to_vec()
                            }
                            else if is_there_elif_block {
                                self.block_code[1..where_are_elif_blocks[0]].to_vec()
                            }
                            else if is_there_else_block {
                                else_part = self.block_code[where_is_else_block+1..].to_vec();
                                self.block_code[1..where_is_else_block].to_vec()
                            } else {
                                self.block_code[1..].to_vec()
                            };

                            if check_block_code_condition(operator.to_string(), self.block_code.to_vec()) {
                                let return_of_block_code_execution = self.execute_block_code(if_part.to_vec(), false);
                                
                                match return_of_block_code_execution {
                                    Ok(_) => (),
                                    Err(e) => return Err(format!("{}", e))
                                }
                            }
                            else if is_there_elif_block && is_elif_block_true {
                                let final_right_elif_position = where_are_elif_blocks[where_is_right_elif_block]+1;
                                let elif_part = self.block_code[final_right_elif_position..final_right_elif_position+1].to_vec();

                                let return_of_block_code_execution = self.execute_block_code(elif_part.to_vec(), false);
                                
                                match return_of_block_code_execution {
                                    Ok(_) => (),
                                    Err(e) => return Err(format!("{}", e))
                                }
                            }
                            else if is_there_else_block {
                                let return_of_block_code_execution = self.execute_block_code(else_part.to_vec(), false);

                                match return_of_block_code_execution {
                                    Ok(_) => (),
                                    Err(e) => return Err(format!("{}", e))
                                }
                            }
                        }
                        else if first_predefined_name == "WHILE" {
                            let mut error = "".to_string();

                            let original_operator = &self.block_code[0][2].1.clone();
                            let operator = match original_operator {
                                tokenizer::ValueEnum::String(operator) => operator,
                                _ => unreachable!("SOMEHOW THIS SHOULDN'T BE PRINTED!")
                            };

                            loop {
                                let block_code_original = self.block_code.clone();
                                let new_block_code = update_while_condition_values(&self.block_code, &self.global_variables, &mut error)?;

                                if check_block_code_condition(operator.to_string(), new_block_code) {
                                    let return_of_block_code_execution = self.execute_block_code(self.block_code[1..].to_vec(), true);
                                    
                                    match return_of_block_code_execution {
                                        Ok(_) => (),
                                        Err(e) => return Err(format!("{}", e))
                                    }
                                } else {
                                    break;
                                }

                                self.block_code = block_code_original;
                            }
                        }

                        self.current_block_type.0 = "".to_string();
                    }
                    else if self.current_block_type.0 == "function" {
                        self.current_block_type.0 = "".to_string();
                        self.current_block_type.1 = "".to_string();
                    }
                }
            }

            // saving code block stuff
            if self.current_block_type.0 == "normal" {
                for (i, tokens) in saved_verines {
                    token_collection[i].0 = "VERINE".to_string();
                    token_collection[i].1 = ValueEnum::Verine(tokens);
                }

                self.block_code.push(token_collection.clone());
            }
            else if self.current_block_type.0 == "function" {
                for (i, tokens) in saved_verines {
                    token_collection[i].0 = "VERINE".to_string();
                    token_collection[i].1 = ValueEnum::Verine(tokens);
                }

                self.functions.get_mut(&self.current_block_type.1).unwrap().push(token_collection.clone());
            }

            return Ok(()); 
        }


        // *check order of keys and values + evaluation* //

        let first_key_element = &token_collection[0].0;

        if first_key_element != "PREDEFINED_NAME" {
            return Err("EXECUTION ERROR: EVERY LINE HAS TO START WITH A PREDEFINED NAME (EXCEPT FOR COMMENT-LINES) !".to_string());
        }

        // check if key and value order is right
        let return_of_check = is_key_and_value_order_right(token_collection.to_vec());

        match return_of_check {
            Ok(_) => (),
            Err(e) => return Err(format!("{}", e))
        }

        // evaluate value for LET, PRINT, IF, PUSH, INSERT, SET
        if token_collection.len() > 0 {
            let mut token_collection_clone = token_collection.clone();

            if let tokenizer::ValueEnum::String(clean) = &token_collection[0].1 {
                let value = predefined_name_order.get(&clean.as_str()).ok_or(format!("EXECUTION ERROR: '{}' IS NEVER AT THE BEGINNING!", clean))?;
                
                if let OrderEnum::MultipleOptions(_v) = value {
                    if let tokenizer::ValueEnum::String(fv) = &token_collection[0].1 {
                        if fv == "LET" {
                            if token_collection[3].0 == "VARIABLE/FUNCTION_NAME" {
                                let variable_name = match &token_collection[3].1 {
                                    tokenizer::ValueEnum::String(variable_name) => variable_name,
                                    _ => unreachable!("SOMEHOW THIS SHOULDN'T BE PRINTED!")
                                };

                                let value_of_variable = self.global_variables.get(variable_name).ok_or(format!("EXECUTION ERROR: THERE IS NO VARIABLE CALLED {}!", variable_name))?;

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
                                    tokenizer::ValueEnum::Verine(_) => unreachable!("SOMEHOW THIS SHOULDN'T BE PRINTED!")
                                }
                            }
                        }
                        if fv == "PRINT" {
                            if token_collection[1].0 == "VARIABLE/FUNCTION_NAME" {
                                let variable_name = match &token_collection[1].1 {
                                    tokenizer::ValueEnum::String(variable_name) => variable_name,
                                    _ => unreachable!("SOMEHOW THIS SHOULDN'T BE PRINTED!")
                                };

                                let value_of_variable = self.global_variables.get(variable_name).ok_or(format!("EXECUTION ERROR: THERE IS NO VARIABLE CALLED {}!", variable_name))?;

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
                                    _ => return Err("EXECUTION ERROR: CAN'T PRINT THIS VARIABLE!".to_string())
                                }
                            }
                        }
                        else if fv == "IF" {
                            if token_collection[1].0 == "VARIABLE/FUNCTION_NAME" {
                                let variable_name = match &token_collection[1].1 {
                                    tokenizer::ValueEnum::String(variable_name) => variable_name,
                                    _ => unreachable!("SOMEHOW THIS SHOULDN'T BE PRINTED!")
                                };

                                let value_of_variable = self.global_variables.get(variable_name).ok_or(format!("EXECUTION ERROR: THERE IS NO VARIABLE CALLED {}!", variable_name))?;

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
                                    _ => return Err("EXECUTION ERROR: FIRST VARIABLE HAS TO BE A STRING, INTEGER OR FLOAT!".to_string())
                                }
                            }
                            if token_collection[3].0 == "VARIABLE/FUNCTION_NAME" {
                                let variable_name = match &token_collection[3].1 {
                                    tokenizer::ValueEnum::String(variable_name) => variable_name,
                                    _ => unreachable!("SOMEHOW THIS SHOULDN'T BE PRINTED!")
                                };

                                let value_of_variable = self.global_variables.get(variable_name).ok_or(format!("EXECUTION ERROR: THERE IS NO VARIABLE CALLED {}!", variable_name))?;

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
                                    _ => return Err("EXECUTION ERROR: SECOND VARIABLE HAS TO BE A STRING, INTEGER OR FLOAT!".to_string())
                                }
                            }
                        }
                        else if fv == "PUSH" {
                            if token_collection[1].0 == "VARIABLE/FUNCTION_NAME" {
                                let variable_name = match &token_collection[1].1 {
                                    tokenizer::ValueEnum::String(variable_name) => variable_name,
                                    _ => unreachable!("SOMEHOW THIS SHOULDN'T BE PRINTED!")
                                };

                                let value_of_variable = self.global_variables.get(variable_name).ok_or(format!("EXECUTION ERROR: THERE IS NO VARIABLE CALLED {}!", variable_name))?;

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
                                    _ => return Err("EXECUTION ERROR: FIRST VARIABLE HAS TO BE A STRING, INTEGER OR FLOAT!".to_string())
                                }
                            }
                        }
                        else if fv == "INSERT" {
                            if token_collection[1].0 == "VARIABLE/FUNCTION_NAME" {
                                let variable_name = match &token_collection[1].1 {
                                    tokenizer::ValueEnum::String(variable_name) => variable_name,
                                    _ => unreachable!("SOMEHOW THIS SHOULDN'T BE PRINTED!")
                                };

                                let value_of_variable = self.global_variables.get(variable_name).ok_or(format!("EXECUTION ERROR: THERE IS NO VARIABLE CALLED {}!", variable_name))?;

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
                                    _ => return Err("EXECUTION ERROR: FIRST VARIABLE HAS TO BE A STRING, INTEGER OR FLOAT!".to_string())
                                }
                            }
                            if token_collection[5].0 == "VARIABLE/FUNCTION_NAME" {
                                let variable_name = match &token_collection[5].1 {
                                    tokenizer::ValueEnum::String(variable_name) => variable_name,
                                    _ => unreachable!("SOMEHOW THIS SHOULDN'T BE PRINTED!")
                                };

                                let value_of_variable = self.global_variables.get(variable_name).ok_or(format!("EXECUTION ERROR: THERE IS NO VARIABLE CALLED {}!", variable_name))?;

                                match value_of_variable {
                                    tokenizer::ValueEnum::Integer(v) => {
                                        token_collection_clone[5].0 = "INTEGER".to_string();
                                        token_collection_clone[5].1 = tokenizer::ValueEnum::Integer(*v);
                                    },
                                    _ => return Err("EXECUTION ERROR: THIRD VARIABLE HAS TO BE AN INTEGER!".to_string())
                                }
                            }

                        }
                        else if fv == "SET" {
                            if token_collection[3].0 == "VARIABLE/FUNCTION_NAME" {
                                let variable_name = match &token_collection[3].1 {
                                    tokenizer::ValueEnum::String(variable_name) => variable_name,
                                    _ => unreachable!("SOMEHOW THIS SHOULDN'T BE PRINTED!")
                                };

                                let value_of_variable = self.global_variables.get(variable_name).ok_or(format!("EXECUTION ERROR: THERE IS NO VARIABLE CALLED {}!", variable_name))?;

                                match value_of_variable {
                                    tokenizer::ValueEnum::Integer(v) => {
                                        token_collection_clone[3].0 = "INTEGER".to_string();
                                        token_collection_clone[3].1 = tokenizer::ValueEnum::Integer(*v);
                                    },
                                    _ => return Err("EXECUTION ERROR: SECOND VARIABLE HAS TO BE AN INTEGER!".to_string())
                                }
                            }
                            if token_collection[5].0 == "VARIABLE/FUNCTION_NAME" {
                                let variable_name = match &token_collection[5].1 {
                                    tokenizer::ValueEnum::String(variable_name) => variable_name,
                                    _ => unreachable!("SOMEHOW THIS SHOULDN'T BE PRINTED!")
                                };

                                let value_of_variable = self.global_variables.get(variable_name).ok_or(format!("EXECUTION ERROR: THERE IS NO VARIABLE CALLED {}!", variable_name))?;

                                match value_of_variable {
                                    tokenizer::ValueEnum::String(v) => {
                                        token_collection_clone[5].0 = "STRING".to_string();
                                        token_collection_clone[5].1 = tokenizer::ValueEnum::String(v.to_string());
                                    },
                                    tokenizer::ValueEnum::Integer(v) => {
                                        token_collection_clone[5].0 = "INTEGER".to_string();
                                        token_collection_clone[5].1 = tokenizer::ValueEnum::Integer(*v);
                                    },
                                    tokenizer::ValueEnum::Float(v) => {
                                        token_collection_clone[5].0 = "FLOAT".to_string();
                                        token_collection_clone[5].1 = tokenizer::ValueEnum::Float(*v);
                                    },
                                    _ => return Err("EXECUTION ERROR: THIRD VARIABLE HAS TO BE A STRING, INTEGER OR FLOAT!".to_string())
                                }
                            }
                        }
                    }
                }
            }

            token_collection = token_collection_clone;
        }


        // * real execution part * //

        match &token_collection[0].1 {
            tokenizer::ValueEnum::String(v) => {
                if v == &"LET".to_string() {
                    let variable_name: String = {
                        match &token_collection[1].1 {
                            tokenizer::ValueEnum::String(current_v) => current_v.to_string(),
                            _ => unreachable!("SOMEHOW THIS SHOULDN'T BE PRINTED!")
                        }
                    };
                    self.global_variables.insert(variable_name, token_collection[3].1.clone());
                }
                if v == &"PRINT".to_string() {
                    let stuff_to_print: String = {
                        match &token_collection[1].1 {
                            tokenizer::ValueEnum::String(stuff) => stuff.to_string(), 
                            tokenizer::ValueEnum::Integer(stuff) => stuff.to_string(),
                            tokenizer::ValueEnum::Float(stuff) => format!("{:?}", stuff), 
                            _ => unreachable!("SOMEHOW THIS SHOULDN'T BE PRINTED!")
                        }
                    };

                    println!("{}", stuff_to_print);
                }
                else if v == &"FN".to_string() {
                    let fn_name = match &token_collection[1].1 {
                        tokenizer::ValueEnum::String(fn_name) => fn_name,
                        _ => unreachable!("SOMEHOW THIS SHOULDN'T BE PRINTED!")
                    };

                    self.functions.insert(fn_name.to_string(), vec![token_collection.clone()]);
                    self.current_block_type.0 = "function".to_string();
                    self.current_block_type.1 = fn_name.to_string();
                    
                    add_indentation(&mut self.indentation);
                }
                else if v == &"DO".to_string() {
                    let function_name = match &token_collection[1].1 {
                        tokenizer::ValueEnum::String(function_name) => function_name,
                        _ => unreachable!("SOMEHOW THIS SHOULDN'T BE PRINTED!")
                    };

                    let function_code_block = self.functions.get(function_name).ok_or(format!("EXECUTION ERROR: THERE IS NO FUNCTION CALLED {}!", function_name))?;
                    
                    let block_code = function_code_block[1..].to_vec();
                    let return_of_block_code_execution = self.execute_block_code(block_code, false);
                                                    
                    match return_of_block_code_execution {
                        Ok(_) => (),
                        Err(e) => return Err(format!("{}", e))
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
                    let stuff = match &token_collection[3].1 {
                        tokenizer::ValueEnum::String(stuff) => stuff,
                        _ => unreachable!("SOMEHOW THIS SHOULDN'T BE PRINTED!")
                    };

                    let value = self.global_variables.get(stuff).ok_or(format!("EXECUTION ERROR: THERE IS NO VARIABLE CALLED {}!", stuff))?;

                    let vec = match &value {
                        tokenizer::ValueEnum::Array(vec) => vec,
                        _ => unreachable!("SOMEHOW THIS SHOULDN'T BE PRINTED!")
                    };

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
                        _ => return Err("EXECUTION ERROR: YOU HAVE TO PUSH A STRING, INTEGER OR FLOAT ONTO AN ARRAY!".to_string())
                    }
                }
                else if v == &"POP".to_string() {
                    let stuff = match &token_collection[2].1 {
                        tokenizer::ValueEnum::String(stuff) => stuff,
                        _ => unreachable!("SOMEHOW THIS SHOULDN'T BE PRINTED!")
                    };

                    let value = self.global_variables.get(stuff).ok_or(format!("EXECUTION ERROR: THERE IS NO VARIABLE CALLED {}!", stuff))?;

                    let vec = match &value {
                        tokenizer::ValueEnum::Array(vec) => vec,
                        _ => return Err("EXECUTION  ERROR: YOU CAN ONLY POP FROM ARRAYS!".to_string())
                    };

                    let mut vec_clone = vec.clone();
                    vec_clone.pop();
                    *self.global_variables.get_mut(stuff).unwrap() = tokenizer::ValueEnum::Array(vec_clone);
                }
                else if v == &"INSERT".to_string() {
                    let stuff = match &token_collection[3].1 {
                        tokenizer::ValueEnum::String(stuff) => stuff,
                        _ => unreachable!("SOMEHOW THIS SHOULDN'T BE PRINTED!")
                    };

                    let value = self.global_variables.get(stuff).ok_or(format!("EXECUTION ERROR: THERE IS NO VARIABLE CALLED {}!", stuff))?;

                    let vec = match &value {
                        tokenizer::ValueEnum::Array(vec) => vec,
                        _ => return Err("EXECUTION ERROR: YOU CAN ONLY INSERT INTO ARRAYS!".to_string())
                    };

                    let index = {
                        match &token_collection[5].1 {
                            tokenizer::ValueEnum::Integer(index_where_to_insert) => *index_where_to_insert as usize,
                            _ => unreachable!("SOMEHOW THIS SHOULDN'T BE PRINTED!")
                        }
                    };

                    if index >= vec.len() && index != 0 {
                        return Err("EXECUTION ERROR: INDEX IS OUT OF BOUNDS!".to_string());
                    }

                    match &token_collection[1].1 {
                        tokenizer::ValueEnum::String(stuff_to_push) => {
                            let mut vec_clone = vec.clone(); 
                            vec_clone.insert(index, tokenizer::ArrayTypesEnum::String(stuff_to_push.to_string()));
                            *self.global_variables.get_mut(stuff).unwrap() = tokenizer::ValueEnum::Array(vec_clone);
                        },
                        tokenizer::ValueEnum::Integer(stuff_to_push) => {
                            let mut vec_clone = vec.clone();
                            vec_clone.insert(index, tokenizer::ArrayTypesEnum::Integer(*stuff_to_push));
                            *self.global_variables.get_mut(stuff).unwrap() = tokenizer::ValueEnum::Array(vec_clone);
                        },
                        tokenizer::ValueEnum::Float(stuff_to_push) => {
                            let mut vec_clone = vec.clone();
                            vec_clone.insert(index, tokenizer::ArrayTypesEnum::Float(*stuff_to_push));
                            *self.global_variables.get_mut(stuff).unwrap() = tokenizer::ValueEnum::Array(vec_clone);
                        },
                        _ => return Err("EXECUTION ERROR: YOU HAVE TO INSERT A STRING, INTEGER OR FLOAT INTO AN ARRAY!".to_string())
                    }
                }
                else if v == &"REMOVE".to_string() {
                    let stuff = match &token_collection[2].1 {
                        tokenizer::ValueEnum::String(stuff) => stuff,
                        _ => unreachable!("SOMEHOW THIS SHOULDN'T BE PRINTED!")
                    };

                    let value = self.global_variables.get(stuff).ok_or(format!("EXECUTION ERROR: THERE IS NO VARIABLE CALLED {}!", stuff))?;

                    let vec = match &value {
                        tokenizer::ValueEnum::Array(vec) => vec,
                        _ => unreachable!("SOMEHOW THIS SHOULDN'T BE PRINTED!")
                    };

                    let index = {
                        match &token_collection[4].1 {
                            tokenizer::ValueEnum::Integer(index_where_to_remove) => *index_where_to_remove as usize,
                            _ => unreachable!("SOMEHOW THIS SHOULDN'T BE PRINTED!")
                        }
                    };
                    
                    if index >= vec.len() {
                        return Err("EXECUTION ERROR: INDEX IS OUT OF BOUNDS!".to_string());
                    }

                    let mut vec_clone = vec.clone();
                    vec_clone.remove(index);
                    *self.global_variables.get_mut(stuff).unwrap() = tokenizer::ValueEnum::Array(vec_clone);
                }
                else if v == &"SET".to_string() {
                    let stuff = match &token_collection[1].1 {
                        tokenizer::ValueEnum::String(stuff) => stuff,
                        _ => unreachable!("SOMEHOW THIS SHOULDN'T BE PRINTED!")
                    };

                    let value = self.global_variables.get(stuff).ok_or(format!("EXECUTION ERROR: THERE IS NO VARIABLE CALLED {}!", stuff))?;

                    let vec = match &value {
                        tokenizer::ValueEnum::Array(vec) => vec,
                        _ => unreachable!("SOMEHOW THIS SHOULDN'T BE PRINTED!")
                    };

                    let index = {
                        match &token_collection[3].1 {
                            tokenizer::ValueEnum::Integer(index_where_to_remove) => *index_where_to_remove as usize,
                            _ => unreachable!("SOMEHOW THIS SHOULDN'T BE PRINTED!")
                        }
                    };

                    if index >= vec.len() {
                        return Err("EXECUTION ERROR: INDEX IS OUT OF BOUNDS!".to_string());
                    }

                    match &token_collection[5].1 {
                        tokenizer::ValueEnum::String(end_value) => {
                            let mut vec_clone = vec.clone();
                            vec_clone[index] = ArrayTypesEnum::String(end_value.to_string());
                            *self.global_variables.get_mut(stuff).unwrap() = tokenizer::ValueEnum::Array(vec_clone);
                        },
                        tokenizer::ValueEnum::Integer(end_value) => {
                            let mut vec_clone = vec.clone();
                            vec_clone[index] = ArrayTypesEnum::Integer(*end_value);
                            *self.global_variables.get_mut(stuff).unwrap() = tokenizer::ValueEnum::Array(vec_clone);
                        },
                        tokenizer::ValueEnum::Float(end_value) => {
                            let mut vec_clone = vec.clone();
                            vec_clone[index] = ArrayTypesEnum::Float(*end_value);
                            *self.global_variables.get_mut(stuff).unwrap() = tokenizer::ValueEnum::Array(vec_clone);
                        }
                        _ => unreachable!("SOMEHOW THIS SHOULDN'T BE PRINTED!")
                    }
                }

                else if v == &"HELP_FOR".to_string() {
                    let keyword = match &token_collection[1].1 {
                        tokenizer::ValueEnum::String(keyword) => keyword,
                        _ => unreachable!("SOME THIS SHOULDN'T BE PRINTED!")
                    };

                    let order_collection= predefined_name_order.get(&keyword.as_str()).ok_or(format!("EXECUTION ERROR: CAN'T PRINT HELP FOR {} BECAUSE IT'S NOT A PREDEFINED NAME WHICH IS AT THE BEGINNING OF A LINE!", keyword))?;

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
                }
            },
            _ => unreachable!("SOMEHOW THIS SHOULDN'T BE PRINTED!")
        }

        // debugging purpose
        println!("self.global_variables: {:?}", self.global_variables);

        Ok(()) 
    }
}
