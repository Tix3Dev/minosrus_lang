use std::collections::HashMap;
use std::iter::{Enumerate, Peekable};
use std::str::Chars;

use crate::tokenizer;
use crate::tokenizer::{ArrayTypesEnum, ValueEnum};

#[derive(Debug, Clone)]
pub enum Token {
    Id(String),
    Number(String),
    String(String),
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

#[derive(Debug, Copy, Clone)]
pub enum Op {
    Plus,
    Minus,
    Asterisk,
    Slash,
}

impl Token {
    pub fn from_single_char(c: char, last_token: Option<&Token>) -> Self {
        match c {
            '|' if last_token.map_or(true, |t| matches!(t, Self::Operator(_))) => Self::OpenVerine,
            '|' => Self::CloseVerine,
            '+' => Self::Operator(Op::Plus),
            '-' => Self::Operator(Op::Minus),
            '*' => Self::Operator(Op::Asterisk),
            '/' => Self::Operator(Op::Slash),
            _ => unimplemented!()
        }
    }

    pub fn from_symbol(symbol: &str) -> Self {
        match symbol {
            "GET" => Self::Get,
            "FROM" => Self::From,
            "LEN" => Self::Len,
            "AT" => Self::At,
            "STRING_FROM" => Self::StringFrom,
            "INTEGER_FROM" => Self::IntegerFrom,
            "READLN" => Self::ReadLn,
            _ => {
                let mut it = symbol.chars();
                let (first, second) = (it.next(), it.next());

                let is_number = match (first, second) {
                    (Some(sign), Some(digit)) if (sign == '+' || sign == '-') && digit.is_ascii_digit() => true,
                    (Some(digit), _) if digit.is_ascii_digit() => true,
                    _ => false,
                };
                if is_number {
                    Self::Number(symbol.to_string())
                } else {
                    Self::Id(symbol.to_string())
                }
            }
        }
    }
}

pub enum TokenType {
    Symbol,
    StringLiteral,
}

pub enum TokenizerError {
    StdInError,
    InvalidExpression,
    VariableNotFound(String),
    NumberParsingError(String),
    NumberNotAnI32(String),
    InvalidOperator,
    InvalidOperands,
    InvalidIndex(String),
    IndexOutOfBounds,
    TypeNotIndexable,
    TypeHasNoLength,
}

pub struct Tokenizer<'a> {
    source: &'a str,
    current_token_start_index: usize,
    starting_new_token: bool,
    current_token_type: TokenType,

    tokens: Vec<Token>,

    it: Peekable<Enumerate<Chars<'a>>>,
    current_character: Option<(usize, char)>,
}

type Globals = HashMap<String, tokenizer::ValueEnum>;

impl<'a> Tokenizer<'a> {
    pub fn new(source: &'a str) -> Self {
        let mut this = Self {
            source,
            current_token_start_index: 0,
            starting_new_token: true,
            current_token_type: TokenType::Symbol,

            tokens: vec![],

            it: source.chars().enumerate().peekable(),
            current_character: None,
        };
        this.current_character = this.it.next();
        this
    }

    fn tokenize(mut self) -> Vec<Token> {
        while let Some((i, c)) = self.current_character {

            // String literals
            match self.current_token_type {
                // End
                TokenType::StringLiteral if c == '"' => {
                    self.end_current_token();
                    self.current_token_type = TokenType::Symbol;
                    self.current_character = self.it.next();
                    continue;
                }
                // Middle
                TokenType::StringLiteral => {
                    // Escape quotes with backward slash (example: \")
                    if c == '\\' {
                        self.it.next(); // Skip the next character
                    }
                    self.current_character = self.it.next();
                    continue;
                }
                // Start
                TokenType::Symbol => if matches!(c, '"') {
                    self.end_current_token();
                    self.current_token_type = TokenType::StringLiteral;
                    self.current_character = self.it.next();
                    continue;
                }
            }

            match c {
                '+' | '-' if self.it.peek().filter(|(_, char)| char.is_ascii_digit()).is_some() => {
                    if self.starting_new_token {
                        self.current_token_start_index = i;
                        self.starting_new_token = false;
                    }
                }
                // Special characters and operators that terminate a token
                '|' | '+' | '-' | '*' | '/' => {
                    self.end_current_token();
                    self.tokens.push(Token::from_single_char(c, self.tokens.last()));
                }
                whitespace if whitespace.is_whitespace() => {
                    self.end_current_token();
                }
                // Mark the beginning of the new token
                _ => {
                    if self.starting_new_token {
                        self.current_token_start_index = i;
                        self.starting_new_token = false;
                    }
                }
            }
            self.current_character = self.it.next();
        }
        self.end_current_token();
        self.tokens.clone()
    }

    fn end_current_token(&mut self) {
        let end_index = if let Some((i, _)) = self.current_character {
            i
        } else {
            self.source.len()
        };

        let token_str = &self.source[self.current_token_start_index..end_index];

        match self.current_token_type {
            TokenType::Symbol => {
                if !token_str.is_empty() {
                    self.tokens.push(Token::from_symbol(token_str));
                }
            }
            TokenType::StringLiteral => {
                self.tokens.push(Token::String(token_str.to_string()));
            }
        }
        self.starting_new_token = true;
        self.current_token_start_index = end_index + 1;
    }

    fn evaluate(mut tokens: Vec<Token>, global_variables: &Globals) -> Result<Token, TokenizerError> {
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
            let mut resulting_tokens = vec![];

            for range in &verine_expression_ranges {
                let without_verines = range.start() + 1..*range.end();
                let result = Tokenizer::evaluate(tokens[without_verines].to_vec(), &global_variables)?;
                resulting_tokens.push(result);
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
                *token = Token::String(input)
            }
        }

        // Do a second pass for FROM_STRING and FROM_INTEGER
        let mut tokens = {
            let mut new_tokens = vec![];
            let mut i = 0;
            while i < tokens.len() - 1 {
                match (&tokens[i], &tokens[i + 1]) {
                    (Token::StringFrom, Token::Number(n)) => {
                        new_tokens.push(Token::String(n.to_owned()));
                        i += 2;
                    }
                    (Token::IntegerFrom, Token::String(s)) => {
                        new_tokens.push(Token::Number(s.to_owned()));
                        i += 2;
                    }
                    (Token::StringFrom | Token::IntegerFrom, _) => {
                        return Err(InvalidExpression);
                    },
                    (token, _) => {
                        new_tokens.push(token.clone());
                        i += 1;
                    }
                }
            }
            new_tokens.push(tokens.last().unwrap().clone());
            new_tokens
        };

        fn parse_i32(n: &str) -> Result<i32, TokenizerError> {
            // f64 is a superset of all integers
            // Any valid number can be parsed into an f64
            n.parse::<f64>().map_err(|_| NumberParsingError(n.to_owned()))?;
            n.parse::<i32>().map_err(|_| NumberNotAnI32(n.to_owned()))
        }

        use Token::{Get, From, Id, At, Len};

        // Final pass, evaluation
        while tokens.len() > 1 {
            match tokens.as_slice() {
                [left, Token::Operator(op), right, ..] => {
                    let left = Tokenizer::evaluate(vec![left.clone()], global_variables)?;
                    let right = Tokenizer::evaluate(vec![right.clone()], global_variables)?;

                    let result = match (left, right) {
                        (Token::Number(l), Token::Number(r)) => {
                            let l = parse_i32(&l)?;
                            let r = parse_i32(&r)?;

                            // Ugly
                            // We should maybe store a number inside Token::Number but it's good enough for now
                            match op {
                                Op::Plus => Ok(Token::Number((l + r).to_string())),
                                Op::Minus => Ok(Token::Number((l - r).to_string())),
                                Op::Asterisk => Ok(Token::Number((l * r).to_string())),
                                Op::Slash => Ok(Token::Number((l / r).to_string())),
                            }
                        }
                        (Token::String(l), Token::String(r)) => {
                            match op {
                                Op::Plus => Ok(Token::String(format!("{}{}", l, r))),
                                _ => Err(InvalidOperator),
                            }
                        }
                        _ => Err(InvalidOperands),
                    }?;
                    // Replace the first 3 tokens with the result of them
                    tokens.splice(0..3, std::iter::once(result));
                }
                [Get, From, Id(var), At, index, ..] => {
                    let index = match index {
                        Id(id) => {
                            match get_global_variable(id)? {
                                &ValueEnum::Integer(index) => Ok(index as usize),
                                _ => Err(InvalidIndex(var.to_owned()))
                            }
                        }
                        Token::Number(index) => {
                            index.parse::<usize>().map_err(|_| NumberParsingError(index.to_owned()))
                        }
                        _ => Err(InvalidIndex(var.to_owned()))
                    }?;

                    let result = match get_global_variable(var)? {
                        ValueEnum::Array(array) => {
                            match array.get(index).ok_or(IndexOutOfBounds)? {
                                ArrayTypesEnum::String(s) => Ok(Token::String(s.to_owned())),
                                ArrayTypesEnum::Integer(i) => Ok(Token::Number(i.to_string()))
                            }
                        }
                        ValueEnum::String(var) => {
                            let char = var.chars().nth(index).ok_or(IndexOutOfBounds)?;
                            Ok(Token::String(char.to_string()))
                        }
                        _ => Err(TypeNotIndexable)
                    }?;

                    tokens.splice(0..5, std::iter::once(result));
                }
                [Get, From, Id(var), Len, ..] => {
                    let result = match get_global_variable(var)? {
                        ValueEnum::Array(array) => Ok(Token::Number(array.len().to_string())),
                        ValueEnum::String(var) => Ok(Token::Number(var.len().to_string())),
                        _ => Err(TypeHasNoLength)
                    }?;

                    tokens.splice(0..4, std::iter::once(result));
                }
                _ => return Err(InvalidExpression),
            }
        }
        assert_eq!(tokens.len(), 1);

        // Make sure the remaining token is valid for the interpreter
        match tokens.remove(0) {
            Token::Id(var) => {
                match get_global_variable(&var)? {
                    ValueEnum::String(str) => Ok(Token::String(str.to_owned())),
                    ValueEnum::Integer(int) => Ok(Token::Number(int.to_string())),
                    _ => return Err(InvalidExpression)
                }
            }
            // Force parse the number to make sure it's an integer
            Token::Number(n) => Ok(Token::Number(parse_i32(&n)?.to_string())),
            token @ Token::String(_) => Ok(token),
            _ => Err(InvalidExpression)
        }
    }

    pub fn tokenize_and_evaluate(input: &str, global_variables: &Globals) -> Result<Token, TokenizerError> {
        let tokens = Tokenizer::new(&input).tokenize();
        Tokenizer::evaluate(tokens, &global_variables)
    }
}