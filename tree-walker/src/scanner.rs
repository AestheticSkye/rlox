#[cfg(test)]
mod tests;
mod token;

use std::{collections::HashMap, sync::LazyLock};

use token::{Literal, Token, TokenType};

use crate::interpreter_error::InterpreterError;

static KEYWORDS: LazyLock<HashMap<&str, TokenType>> = LazyLock::new(|| {
    HashMap::from([
        ("and", TokenType::And),
        ("class", TokenType::Class),
        ("else", TokenType::Else),
        ("false", TokenType::False),
        ("for", TokenType::For),
        ("fun", TokenType::Fun),
        ("if", TokenType::If),
        ("nil", TokenType::Nil),
        ("or", TokenType::Or),
        ("print", TokenType::Print),
        ("return", TokenType::Return),
        ("super", TokenType::Super),
        ("this", TokenType::This),
        ("true", TokenType::True),
        ("var", TokenType::Var),
        ("while", TokenType::While),
    ])
});

#[derive(Default)]
pub struct Scanner {
    source: String,
    tokens: Vec<Token>,
    errors: Vec<InterpreterError>,
    start: usize,
    current: usize,
    line: usize,
}

impl Scanner {
    pub fn new(source: String) -> Self {
        Self {
            source,
            tokens: Vec::new(),
            errors: Vec::new(),
            start: 0,
            current: 0,
            line: 1,
        }
    }

    pub fn scan_tokens(mut self) -> Result<Vec<Token>, Vec<InterpreterError>> {
        while !self.is_at_end() {
            self.start = self.current;
            self.scan_token();
        }

        self.tokens.push(Token {
            token_type: TokenType::Eof,
            lexeme: String::new(),
            literal: Literal::None,
            line: self.line,
        });

        if self.errors.is_empty() {
            Ok(self.tokens)
        } else {
            Err(self.errors)
        }
    }

    fn scan_token(&mut self) {
        let char = self.advance();

        match char {
            '(' => self.add_token(TokenType::LeftParen, Literal::None),
            ')' => self.add_token(TokenType::RightParen, Literal::None),
            '{' => self.add_token(TokenType::LeftBrace, Literal::None),
            '}' => self.add_token(TokenType::RightBrace, Literal::None),
            ',' => self.add_token(TokenType::Comma, Literal::None),
            '.' => self.add_token(TokenType::Dot, Literal::None),
            '-' => self.add_token(TokenType::Minus, Literal::None),
            '+' => self.add_token(TokenType::Plus, Literal::None),
            ';' => self.add_token(TokenType::Semicolon, Literal::None),
            '*' => self.add_token(TokenType::Star, Literal::None),
            '!' => {
                let token_type = if self.match_next('=') {
                    TokenType::BangEqual
                } else {
                    TokenType::Bang
                };
                self.add_token(token_type, Literal::None);
            }
            '=' => {
                let token_type = if self.match_next('=') {
                    TokenType::EqualEqual
                } else {
                    TokenType::Equal
                };
                self.add_token(token_type, Literal::None);
            }
            '<' => {
                let token_type = if self.match_next('=') {
                    TokenType::LessEqual
                } else {
                    TokenType::Less
                };
                self.add_token(token_type, Literal::None);
            }
            '>' => {
                let token_type = if self.match_next('=') {
                    TokenType::GreaterEqual
                } else {
                    TokenType::Greater
                };
                self.add_token(token_type, Literal::None);
            }
            '/' => {
                if self.match_next('/') {
                    // A comment goes until the end of the line.”
                    while self.peek() != '\n' && !self.is_at_end() {
                        self.advance();
                    }
                } else {
                    self.add_token(TokenType::Slash, Literal::None);
                }
            }
            // Ignore whitespace.
            ' ' | '\r' | '\t' => {}
            '\n' => {
                self.line += 1;
            }
            '"' => {
                self.string();
            }
            '0'..='9' => {
                self.number();
            }
            'a'..='z' | 'A'..='Z' | '_' => {
                self.identifier();
            }
            _ => self.errors.push(InterpreterError {
                line: self.line,
                location: String::new(),
                message: format!("Unexpected character: {char}"),
            }),
        }
    }

    /// Push a token onto the token stack.
    ///
    /// If the token is a string or number `literal` should be set accordingly, otherwise set to [`Literal::None`].
    fn add_token(&mut self, token_type: TokenType, literal: Literal) {
        let text = self.source[self.start..self.current].to_string();
        self.tokens.push(Token {
            token_type,
            lexeme: text,
            literal,
            line: self.line,
        });
    }

    /// Advance to the next character in the source, consuming and returning it.
    fn advance(&mut self) -> char {
        let result = self
            .source
            .chars()
            .nth(self.current)
            .expect("Index out of bounds");
        self.current += 1;
        result
    }

    fn is_at_end(&self) -> bool {
        self.current >= self.source.len()
    }

    /// Match to see if the next character is equal to whats expected.
    /// Consumes the character if true.
    fn match_next(&mut self, expected: char) -> bool {
        if self.is_at_end() {
            return false;
        }

        if self.source.chars().nth(self.current).unwrap_or_default() == expected {
            self.current += 1;

            true
        } else {
            false
        }
    }

    /// Return the char at self.current.
    fn peek(&self) -> char {
        return self.source.chars().nth(self.current).unwrap_or_default();
    }

    /// Return the char at self.current + 1.
    fn peek_next(&self) -> char {
        return self
            .source
            .chars()
            .nth(self.current + 1)
            .unwrap_or_default();
    }

    /// Scans the following string.
    fn string(&mut self) {
        while self.peek() != '"' && !self.is_at_end() {
            if self.peek() == '\n' {
                self.line += 1;
            }
            self.advance();
        }

        if self.is_at_end() {
            self.errors.push(InterpreterError {
                line: self.line,
                location: String::new(),
                message: "Unterminated string.".into(),
            });

            return;
        }

        self.advance();

        let value = self.source[self.start + 1..self.current - 1].to_string();
        self.add_token(TokenType::String, Literal::String(value));
    }

    /// Scans the following number.
    fn number(&mut self) {
        while self.peek().is_ascii_digit() {
            self.advance();
        }

        // Look for a fractional part.
        if self.peek() == '.' && self.peek_next().is_ascii_digit() {
            // Consume the "."
            self.advance();

            while self.peek().is_ascii_digit() {
                self.advance();
            }
        }

        self.add_token(
            TokenType::Number,
            Literal::Number(self.source[self.start..self.current].parse().unwrap()),
        );
    }

    /// Scans the following keyword or identifier.
    fn identifier(&mut self) {
        while self.peek().is_alphanumeric() {
            self.advance();
        }

        let text = self.source[self.start..self.current].to_owned();
        let token_type = KEYWORDS.get(text.trim()).unwrap_or(&TokenType::Identifier);

        self.add_token(*token_type, Literal::None);
    }
}
