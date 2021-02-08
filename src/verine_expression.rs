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
    Operator(Operator),
    Get,
    From,
    Len,
    At,
    StringFrom,
    IntegerFrom,
    ReadLn,
}

#[derive(Debug, Copy, Clone)]
pub enum Operator {
    Plus,
    Minus,
    Asterisk,
    Slash,
}

impl Token {
    pub fn from_single_char(c: char) -> Self {
        match c {
            '+' => Self::Operator(Operator::Plus),
            '-' => Self::Operator(Operator::Minus),
            '*' => Self::Operator(Operator::Asterisk),
            '/' => Self::Operator(Operator::Slash),
            _ => unimplemented!()
        }
    }

    pub fn from_symbol(symbol: &str) -> Self {
        match symbol {
            "GET" => Token::Get,
            "FROM" => Token::From,
            "LEN" => Token::Len,
            "AT" => Token::At,
            "STRING_FROM" => Token::StringFrom,
            "INTEGER_FROM" => Token::IntegerFrom,
            "READLN" => Token::ReadLn,
            _ => {
                let mut it = symbol.chars();
                let (first, second) = (it.next(), it.next());

                let is_number = match (first, second) {
                    (Some(sign), Some(digit)) if (sign == '+' || sign == '-') && digit.is_ascii_digit() => true,
                    (Some(digit), _) if digit.is_ascii_digit() => true,
                    _ => false,
                };
                if is_number {
                    Token::Number(symbol.to_string())
                } else {
                    Token::Id(symbol.to_string())
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
                '+' | '-' | '*' | '/' => {
                    self.end_current_token();
                    self.tokens.push(Token::from_single_char(c));
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

    pub fn tokenize_and_evaluate(
        input: &str,
        global_variables: &HashMap<String, tokenizer::ValueEnum>,
    ) -> Result<Token, TokenizerError> {
        use TokenizerError::*;

        let get_global_variable = |var: &str| {
            global_variables.get(var).ok_or(VariableNotFound(var.to_owned()))
        };

        // Do a first pass for READLN
        let mut tokens = Tokenizer::new(&input).tokenize();
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

        // Final pass, evaluation
        match tokens.first().unwrap() {
            // Number computation or string interpolation
            Token::Number(_) | Token::String(_) => {
                while tokens.len() > 1 {
                    // We only support binary operations for now
                    if let [left, Token::Operator(op), right, ..] = tokens.as_slice() {
                        let token = match (left, right) {
                            (Token::Number(l), Token::Number(r)) => {
                                // Supposing they are integers for now
                                let l = l.parse::<i32>().map_err(|_| NumberParsingError(l.to_owned()))?;
                                let r = r.parse::<i32>().map_err(|_| NumberParsingError(r.to_owned()))?;

                                // Ugly
                                // We should maybe store a number inside Token::Number but it's good enough for now
                                match op {
                                    Operator::Plus => Ok(Token::Number((l + r).to_string())),
                                    Operator::Minus => Ok(Token::Number((l - r).to_string())),
                                    Operator::Asterisk => Ok(Token::Number((l * r).to_string())),
                                    Operator::Slash => Ok(Token::Number((l / r).to_string())),
                                }
                            }
                            (Token::String(l), Token::String(r)) => {
                                match op {
                                    Operator::Plus => Ok(Token::String(format!("{}{}", l, r))),
                                    _ => Err(InvalidOperator),
                                }
                            }
                            _ => Err(InvalidOperands),
                        }?;
                        // Replace the first 3 tokens with the result of them
                        tokens.remove(0);
                        tokens.remove(0);
                        tokens[0] = token;
                    } else {
                        return Err(InvalidExpression)
                    }
                }

                // At the end, everything was calculated into one final token
                assert_eq!(tokens.len(), 1);
                match &tokens[0] {
                    token @ Token::String(_) => Ok(token.clone()),
                    token @ Token::Number(number) => {
                        // Ugly
                        // Force-parse the number to check if it's valid
                        number.parse::<i32>().map_err(|_| NumberParsingError(number.to_owned()))?;
                        Ok(token.clone())
                    },
                    _ => Err(InvalidExpression)
                }
            }
            // GET FROM VAR [AT X | LEN]
            Token::Get => {
                use Token::*;
                match tokens.as_slice() {
                    [Get, From, Id(var), At, index] => {
                        let var_token = get_global_variable(var)?;

                        let index = match index {
                            Token::Id(id) => {
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

                        match var_token {
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
                        }
                    }
                    [Get, From, Id(var), Len] => {
                        match get_global_variable(var)? {
                            ValueEnum::Array(array) => Ok(Token::Number(array.len().to_string())),
                            ValueEnum::String(var) => Ok(Token::Number(var.len().to_string())),
                            _ => Err(TypeHasNoLength)
                        }
                    }
                    _ => Err(InvalidExpression)
                }
            }
            _ => Err(InvalidExpression)
        }
    }
}
