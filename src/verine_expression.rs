use std::collections::HashMap;

use crate::tokenizer;
use crate::tokenizer::{ArrayTypesEnum, ValueEnum};
use crate::verine_expression::TokenizerError::InvalidExpression;

#[derive(Debug, Clone)]
enum Token {
    Id(String),
    Value(Value),
    Operator(Op),
    Get,
    From,
    Len,
    At,
    StringFrom,
    IntegerFrom,
    ReadLn,
    OpenVerine,
    CloseVerine,
}

#[derive(Debug, Clone)]
pub enum Value {
    Float(f32),
    Integer(i32),
    String(String),
}

impl From<Value> for Token {
    fn from(value: Value) -> Self {
        Self::Value(value)
    }
}

#[derive(Debug, Copy, Clone)]
pub enum Op {
    Plus,
    Minus,
    Asterisk,
    Slash,
    Pow,
}

impl From<Op> for Token {
    fn from(op: Op) -> Self {
        Self::Operator(op)
    }
}

pub enum TokenizerError {
    UnexpectedCharacter(char),
    StdInError,
    InvalidExpression,
    VariableNotFound(String),
    NumberNotAnInteger(String),
    InvalidOperands,
    InvalidIndex(String),
    IndexOutOfBounds,
    TypeNotIndexable,
    TypeHasNoLength,
    DivisionByZero,
}

pub struct Tokenizer<'a> {
    view: &'a [char],
    tokens: Vec<Token>,
}

type Globals = HashMap<String, tokenizer::ValueEnum>;

impl<'a> Tokenizer<'a> {
    pub fn new(view: &'a [char]) -> Self {
        Self {
            view,
            tokens: vec![],
        }
    }

    fn tokenize(&mut self) -> Result<Vec<Token>, TokenizerError> {
        loop {
            let is_the_last_token_an_operator = {
                match self.tokens.last() {
                    None => true,
                    Some(Token::Operator(_)) => true,
                    Some(_) => false,
                }
            };

            match self.view {
                [w, ..] if w.is_whitespace() => self.view = &self.view[1..],
                ['"', ..] => self.process_string_literals()?,
                [digit, ..] if digit.is_ascii_digit() => self.process_numeric_literals()?,
                ['+' | '-', digit, ..] if digit.is_ascii_digit() && is_the_last_token_an_operator => self.process_numeric_literals()?,
                [p, ..] if is_punctuation(*p) => self.process_operators_and_punctuation()?,
                [c, ..] if is_valid_identifier_character(*c) => self.process_keywords_and_identifiers()?,
                [e, ..] => return Err(TokenizerError::UnexpectedCharacter(*e)),
                [] => break,
            }
        }
        Ok(self.tokens.clone())
    }

    fn process_keywords_and_identifiers(&mut self) -> Result<(), TokenizerError> {
        let start = self.view;
        let mut i = 0;

        fn end_token(start: &[char], i: usize) -> Option<Token> {
            match start.is_empty() {
                true => None,
                false => {
                    let token = start[..i].iter().collect::<String>();
                    let token = match token.as_str() {
                        "GET" => Token::Get,
                        "FROM" => Token::From,
                        "LEN" => Token::Len,
                        "AT" => Token::At,
                        "STRING_FROM" => Token::StringFrom,
                        "INTEGER_FROM" => Token::IntegerFrom,
                        "READLN" => Token::ReadLn,
                        _ => Token::Id(start[..i].iter().collect::<String>())
                    };
                    Some(token)
                }
            }
        }

        loop {
            match self.view {
                [c, ..] if !is_valid_identifier_character(*c) => {
                    if let Some(token) = end_token(start, i) {
                        self.tokens.push(token);
                    }
                    break Ok(());
                }
                [] => {
                    if let Some(token) = end_token(start, i) {
                        self.tokens.push(token);
                    }
                    break Ok(());
                }
                [_, ..] => {
                    self.view = &self.view[1..];
                    i += 1;
                }
            }
        }
    }

    fn process_operators_and_punctuation(&mut self) -> Result<(), TokenizerError> {
        let token = match self.view {
            ['|', ..] => {
                let token = match self.tokens.last() {
                    None => Token::OpenVerine,
                    Some(Token::Operator(_)) => Token::OpenVerine,
                    Some(Token::From) => Token::OpenVerine,
                    Some(Token::At) => Token::OpenVerine,
                    Some(Token::StringFrom) => Token::OpenVerine,
                    Some(Token::IntegerFrom) => Token::OpenVerine,
                    Some(_) => Token::CloseVerine,
                };
                Some((1, token))
            },
            ['+', ..] => Some((1, Op::Plus.into())),
            ['-', ..] => Some((1, Op::Minus.into())),
            ['*', '*', ..] => Some((2, Op::Pow.into())),
            ['*', ..] => Some((1, Op::Asterisk.into())),
            ['/', ..] => Some((1, Op::Slash.into())),
            _ => None,
        };
        if let Some((n, token)) = token {
            self.tokens.push(token);
            self.view = &self.view[n..];
        }
        Ok(())
    }

    fn process_string_literals(&mut self) -> Result<(), TokenizerError> {
        self.view = &self.view[1..]; // Eat first quote
        let start = self.view;
        let mut i = 0;
        loop {
            match self.view {
                ['\\', '"', ..] => {
                    self.view = &self.view[2..];
                    i += 2;
                }
                ['"', ..] => {
                    let string = start[..i].iter().collect::<String>();
                    self.tokens.push(Value::String(string).into());

                    self.view = &self.view[1..]; // Eat last quote
                    break Ok(());
                }
                [_, ..] => {
                    self.view = &self.view[1..];
                    i += 1;
                }
                [] => break Err(InvalidExpression),
            }
        }
    }

    fn process_numeric_literals(&mut self) -> Result<(), TokenizerError> {
        let start = self.view;
        let mut i = 0;
        let mut is_float = false;

        let mut is_sign_allowed = true;
        let mut is_point_allowed = true;

        loop {
            match self.view {
                ['+' | '-', ..] if is_sign_allowed => {
                    is_sign_allowed = false;

                    self.view = &self.view[1..];
                    i += 1;
                }
                ['.', ..] if is_point_allowed => {
                    is_point_allowed = false;
                    is_sign_allowed = false;
                    is_float = true;

                    self.view = &self.view[1..];
                    i += 1;
                }
                [d, ..] if d.is_ascii_digit() => {
                    is_sign_allowed = false;

                    self.view = &self.view[1..];
                    i += 1;
                }
                _ => {
                    let number = &start[..i].iter().collect::<String>();
                    self.tokens.push(if is_float {
                        let float = number.parse::<f32>().unwrap();
                        Value::Float(float).into()
                    } else {
                        let integer = number.parse::<i32>().unwrap();
                        Value::Integer(integer).into()
                    });
                    break Ok(())
                }
            }
        }
    }

    fn evaluate(mut tokens: Vec<Token>, global_variables: &Globals) -> Result<Value, TokenizerError> {
        // Nested verine expression evaluation
        {
            // Remember the ranges of top level verine expressions
            let mut verine_expression_ranges = vec![];
            let mut verine_level = 0;
            let mut opening_verine = 0;

            for (i, token) in tokens.iter().enumerate() {
                match token {
                    Token::OpenVerine => {
                        if verine_level == 0 {
                            opening_verine = i;
                        }
                        verine_level += 1;
                    }
                    Token::CloseVerine => {
                        verine_level -= 1;
                        if verine_level == 0 {
                            verine_expression_ranges.push(opening_verine..=i);
                        }
                    }
                    _ => ()
                }
            }

            // Evaluate each verine expression
            let mut resulting_tokens: Vec<Token> = vec![];

            for range in &verine_expression_ranges {
                let without_verines = range.start() + 1..*range.end();
                let result = Tokenizer::evaluate(tokens[without_verines].to_vec(), &global_variables)?;
                resulting_tokens.push(result.into());
            }

            // Replace the tokens of top level verine expression with their resulting token
            // We need to shift the ranges to the left because we remove from the beginning of the vector
            let mut shift = 0;
            for (range, token) in verine_expression_ranges.iter().zip(resulting_tokens) {
                let range = range.start() - shift..=range.end() - shift;
                tokens.splice(range.clone(), std::iter::once(token));
                shift += range.end() - range.start();
            }
        }

        // Start evaluating this verine expression
        use TokenizerError::*;

        let get_global_variable = |var: &str| {
            global_variables.get(var).ok_or(VariableNotFound(var.to_owned()))
        };

        // Do a first pass for READLN
        for token in &mut tokens {
            if matches!(token, Token::ReadLn) {
                let mut input = String::new();
                std::io::stdin().read_line(&mut input).map_err(|_| StdInError)?;
                input.pop(); // Remove \n
                *token = Value::String(input).into()
            }
        }

        // Do a second pass for STRING_FROM and INTEGER_FROM
        let mut tokens = {
            let mut new_tokens = vec![];

            let mut tokens = tokens.as_slice();
            loop {
                match tokens {
                    [Token::StringFrom, argument, ..] => {
                        let argument = Self::evaluate(vec![argument.clone()], global_variables)?;
                        let argument_string = match argument {
                            Value::String(str) => str,
                            Value::Integer(int) => int.to_string(),
                            Value::Float(float) => float.to_string(),
                        };
                        new_tokens.push(Value::String(argument_string).into());
                        tokens = &tokens[2..];
                    }
                    [Token::IntegerFrom, argument, ..] => {
                        let argument = Self::evaluate(vec![argument.clone()], global_variables)?;
                        let argument_int = match argument {
                            Value::Integer(int) => int,
                            Value::Float(float) => float as i32,
                            Value::String(str) => str.parse::<i32>().map_err(|_| NumberNotAnInteger(str))?,
                        };
                        new_tokens.push(Value::Integer(argument_int).into());
                        tokens = &tokens[2..];
                    }
                    [token, ..] => {
                        new_tokens.push(token.clone());
                        tokens = &tokens[1..];
                    }
                    [] => break,
                }
            }

            new_tokens
        };

        use Token::{Get, From, Id, At, Len};

        // Final pass, evaluation
        loop {
            match tokens.as_slice() {
                [left, Token::Operator(op), right, ..] => {
                    let left = Tokenizer::evaluate(vec![left.clone()], global_variables)?;
                    let right = Tokenizer::evaluate(vec![right.clone()], global_variables)?;

                    fn compute_float_operation(l: f32, op: &Op, r: f32) -> Value {
                        match op {
                            Op::Plus => Value::Float(l + r),
                            Op::Minus => Value::Float(l - r),
                            Op::Asterisk => Value::Float(l * r),
                            Op::Slash => Value::Float(l / r),
                            Op::Pow => Value::Float(l.powf(r))
                        }
                    }

                    use Value::*;
                    let result = match (left, op, right) {
                        // String concatenation
                        (String(l), Op::Plus, String(r)) => String(format!("{}{}", l, r)),
                        (String(l), Op::Plus, Integer(r)) => String(format!("{}{}", l, r)),
                        (String(l), Op::Plus, Float(r)) => String(format!("{}{}", l, r)),
                        (Integer(l), Op::Plus, String(r)) => String(format!("{}{}", l, r)),
                        (Float(l), Op::Plus, String(r)) => String(format!("{}{}", l, r)),
                        // int [op] int = int
                        (Integer(l), _, Integer(r)) => {
                            match op {
                                Op::Plus => Integer(l + r),
                                Op::Minus => Integer(l - r),
                                Op::Asterisk => Integer(l * r),
                                Op::Slash => Integer(l.checked_div(r).ok_or(DivisionByZero)?),
                                Op::Pow => Float((l as f32).powi(r))
                            }
                        }
                        // Implicit int to float conversion
                        (Integer(l), _, Float(r)) => compute_float_operation(l as f32, op, r),
                        (Float(l), _, Integer(r)) => compute_float_operation(l, op, r as f32),
                        (Float(l), _, Float(r)) => compute_float_operation(l, op, r),
                        _ => return Err(InvalidOperands)
                    };
                    // Replace the first 3 tokens with the result of them
                    tokens.splice(0..3, std::iter::once(result.into()));
                }
                [Get, From, Id(var), At, index, ..] => {
                    let index = match index {
                        Id(id) => {
                            match get_global_variable(id)? {
                                &ValueEnum::Integer(index) => index as usize,
                                _ => return Err(InvalidIndex(var.to_owned()))
                            }
                        }
                        Token::Value(Value::Integer(index)) => *index as usize,
                        _ => return Err(InvalidIndex(var.to_owned()))
                    };

                    let result = match get_global_variable(var)? {
                        ValueEnum::Array(array) => {
                            match array.get(index).ok_or(IndexOutOfBounds)? {
                                ArrayTypesEnum::String(s) => Value::String(s.to_owned()),
                                ArrayTypesEnum::Integer(i) => Value::Integer(*i),
                                ArrayTypesEnum::Float(f) => Value::Float(*f),
                            }
                        }
                        ValueEnum::String(var) => {
                            let char = var.chars().nth(index).ok_or(IndexOutOfBounds)?;
                            Value::String(char.to_string())
                        }
                        _ => return Err(TypeNotIndexable)
                    };

                    tokens.splice(0..5, std::iter::once(result.into()));
                }
                [Get, From, Id(var), Len, ..] => {
                    let result = match get_global_variable(var)? {
                        ValueEnum::Array(array) => Value::Integer(array.len() as i32),
                        ValueEnum::String(var) => Value::Integer(var.len() as i32),
                        _ => return Err(TypeHasNoLength)
                    };

                    tokens.splice(0..4, std::iter::once(result.into()));
                }
                [single] => {
                    // Make sure the remaining token is valid for the interpreter
                    let single = match single.clone() {
                        Token::Id(var) => {
                            match get_global_variable(&var)? {
                                ValueEnum::String(str) => Value::String(str.to_owned()),
                                ValueEnum::Integer(int) => Value::Integer(*int),
                                ValueEnum::Float(float) => Value::Float(*float),
                                _ => return Err(InvalidExpression)
                            }
                        }
                        Token::Value(value) => value,
                        _ => return Err(InvalidExpression)
                    };
                    break Ok(single)
                }
                _ => break Err(InvalidExpression)
            }
        }
    }

    pub fn tokenize_and_evaluate(input: &str, global_variables: &Globals) -> Result<Value, TokenizerError> {
        let chars = input.chars().collect::<Vec<_>>();
        let mut tokenizer = Tokenizer::new(chars.as_slice());
        let tokens = tokenizer.tokenize()?;
        Tokenizer::evaluate(tokens, &global_variables)
    }
}

fn is_valid_identifier_character(c: char) -> bool {
    c.is_alphanumeric() || c == '_'
}

fn is_punctuation(c: char) -> bool {
    match c {
        '+' | '-' | '*' | '/' | '.' | '|' => true,
        _ => false,
    }
}