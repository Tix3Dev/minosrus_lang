use std::iter::{Enumerate, Peekable};
use std::str::Chars;

#[derive(Debug, Clone)]
pub enum Token {
    Id(String),
    Number(String),
    String(String),
    Operator(Operator),
    GET,
    FROM,
    LEN,
    AT,
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
            "GET" => Token::GET,
            "FROM" => Token::FROM,
            "LEN" => Token::LEN,
            "AT" => Token::AT,
            _ => {
                let mut it = symbol.chars();
                let (first, second) = (it.next(), it.next());

                let is_number = match (first, second) {
                    (Some(sign), Some(digit)) if (sign == '+' || sign == '-') && digit.is_ascii_digit() => true,
                    (Some(digit), ..) if digit.is_ascii_digit() => true,
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
    StringLiteral(char), // stores the opening/closing character, either ' or "
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

    pub fn tokenize(mut self) -> Vec<Token> {
        while let Some((i, c)) = self.current_character {

            // String literals
            match self.current_token_type {
                // End
                TokenType::StringLiteral(closing_quote) if c == closing_quote => {
                    self.end_current_token();
                    self.current_token_type = TokenType::Symbol;
                    self.current_character = self.it.next();
                    continue;
                }
                // Middle
                TokenType::StringLiteral(_) => {
                    if c == '\\' {
                        self.it.next(); // Skip the next character
                    }
                    self.current_character = self.it.next();
                    continue;
                }
                // Start
                _ => if matches!(c, '\"' | '\'') {
                    self.end_current_token();
                    self.current_token_type = TokenType::StringLiteral(c);
                    self.current_character = self.it.next();
                    continue;
                }
            }

            match c {
                '+' | '-' if self.it.peek().filter(|v| v.1.is_ascii_digit()).is_some() => {
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

    pub fn end_current_token(&mut self) {
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
            TokenType::StringLiteral(_) => {
                self.tokens.push(Token::String(token_str.to_string()));
            }
        }
        self.starting_new_token = true;
        self.current_token_start_index = end_index + 1;
    }
}

