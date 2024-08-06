use crate::scanner::token::{Literal, Token, TokenType};

use super::Scanner;

#[test]
fn test_parsing() {
    let source = "// this is a comment
    (( )){} // grouping stuff
    !*+-/=<> <= == // operators
    \"and this is a string\"
    12345
    123.45
    class fun if nil or orchard super";

    #[rustfmt::skip]
    let result = vec![
        Token { token_type: TokenType::LeftParen, lexeme: "(".to_string(), literal: Literal::None, line: 2 },
        Token { token_type: TokenType::LeftParen, lexeme: "(".to_string(), literal: Literal::None, line: 2 },
        Token { token_type: TokenType::RightParen, lexeme: ")".to_string(), literal: Literal::None, line: 2 },
        Token { token_type: TokenType::RightParen, lexeme: ")".to_string(), literal: Literal::None, line: 2 },
        Token { token_type: TokenType::LeftBrace, lexeme: "{".to_string(), literal: Literal::None, line: 2 },
        Token { token_type: TokenType::RightBrace, lexeme: "}".to_string(), literal: Literal::None, line: 2 },
        Token { token_type: TokenType::Bang, lexeme: "!".to_string(), literal: Literal::None, line: 3 },
        Token { token_type: TokenType::Star, lexeme: "*".to_string(), literal: Literal::None, line: 3 },
        Token { token_type: TokenType::Plus, lexeme: "+".to_string(), literal: Literal::None, line: 3 },
        Token { token_type: TokenType::Minus, lexeme: "-".to_string(), literal: Literal::None, line: 3 },
        Token { token_type: TokenType::Slash, lexeme: "/".to_string(), literal: Literal::None, line: 3 },
        Token { token_type: TokenType::Equal, lexeme: "=".to_string(), literal: Literal::None, line: 3 },
        Token { token_type: TokenType::Less, lexeme: "<".to_string(), literal: Literal::None, line: 3 },
        Token { token_type: TokenType::Greater, lexeme: ">".to_string(), literal: Literal::None, line: 3 },
        Token { token_type: TokenType::LessEqual, lexeme: "<=".to_string(), literal: Literal::None, line: 3 },
        Token { token_type: TokenType::EqualEqual, lexeme: "==".to_string(), literal: Literal::None, line: 3 },
        Token { token_type: TokenType::String, lexeme: "\"and this is a string\"".to_string(), literal: Literal::String("and this is a string".to_string()), line: 4 },
        Token { token_type: TokenType::Number, lexeme: "12345".to_string(), literal: Literal::Number(12345.0), line: 5 },
        Token { token_type: TokenType::Number, lexeme: "123.45".to_string(), literal: Literal::Number(123.45), line: 6 },
        Token { token_type: TokenType::Class, lexeme: "class".to_string(), literal: Literal::None, line: 7 },
        Token { token_type: TokenType::Fun, lexeme: "fun".to_string(), literal: Literal::None, line: 7 },
        Token { token_type: TokenType::If, lexeme: "if".to_string(), literal: Literal::None, line: 7 },
        Token { token_type: TokenType::Nil, lexeme: "nil".to_string(), literal: Literal::None, line: 7 },
        Token { token_type: TokenType::Or, lexeme: "or".to_string(), literal: Literal::None, line: 7 },
        Token { token_type: TokenType::Identifier, lexeme: "orchard".to_string(), literal: Literal::None, line: 7 },
        Token { token_type: TokenType::Super, lexeme: "super".to_string(), literal: Literal::None, line: 7 },
        Token { token_type: TokenType::Eof, lexeme: String::new(), literal: Literal::None, line: 7 },
    ];

    let scanner = Scanner::new(source.into());

    let tokens = scanner.scan_tokens().unwrap();

    assert_eq!(result, tokens);
}
