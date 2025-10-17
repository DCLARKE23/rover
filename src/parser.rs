use std::fs::{File};
use std::io::{Read};
use std::iter::Peekable;
use std::path::Path;
use std::str::Chars;

#[derive(Debug, PartialEq)]
pub enum Token {
    Turn,
    Move,
    Looking,
    Repeat,
    If,
    Else,
    LCurly,
    RCurly,
    Int(i64),
    Char(char),
} 

pub fn read_file (file_path: &Path) -> String {
    let mut file = File::open(file_path).unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    contents
}


pub fn scan_token (chars: &mut Peekable<Chars>) -> Option<Token> {
    while chars.next_if(|c| c.is_whitespace()).is_some() {} 
    match chars.next() {    
        Some(c) if c.is_alphabetic() => {
            let mut buf = String::new();
            buf.push(c);
            while let Some(x) = chars.next_if(|c| c.is_alphabetic()){
                buf.push(x);
            }
            match buf.as_str() {
                "turn" => Some(Token::Turn),
                "move" => Some(Token::Move),
                "if" => Some(Token::If),
                "else" => Some(Token::Else),
                "repeat" => Some(Token::Repeat),
                "looking" => Some(Token::Looking),
                _ => panic!(),
            }
        },
        Some(c) if c.is_ascii_digit() => {
            let mut buf = String::new();
            buf.push(c);
            while let Some(x) = chars.next_if(|c| c.is_ascii_digit()){
                buf.push(x);
            }
            Some(Token::Int(buf.parse().unwrap()))
        },
        Some('{') => Some(Token::LCurly),
        Some('}') => Some(Token::RCurly),
        Some('\'') => {
            let c = match chars.next() {
                Some(c) => c,
                None => panic!(),
            };
            if chars.next_if(|c| *c == '\'').is_some() {
                Some(Token::Char(c))
            } else {
                panic!();
            }
        }
        Some(c) => panic!("{c}"),
        None => None,
    }
}


pub fn tokenize (s: &str) -> Vec<Token> {
    let mut chars = s.chars().peekable();
    let mut tokens = Vec::new();
    while let Some(t) = scan_token(&mut chars){
        tokens.push(t);
    }
    println!("{:?}", tokens);
    tokens

}

// TODO: Make parser
// - Functions for production rules for each element of the grammar
// - Peekable iterator over the tokens


// Tests

#[cfg(test)]
mod tests {
    use super::*;

    fn scan(input: &str) -> Option<Token> {
        let mut chars = input.chars().peekable();
        scan_token(&mut chars)
    }

    #[test]
    fn test_keywords() {
        assert_eq!(scan("turn"), Some(Token::Turn));
        assert_eq!(scan("move"), Some(Token::Move));
        assert_eq!(scan("if"), Some(Token::If));
        assert_eq!(scan("else"), Some(Token::Else));
        assert_eq!(scan("repeat"), Some(Token::Repeat));
        assert_eq!(scan("looking"), Some(Token::Looking));
    }

    #[test]
    fn test_integer() {
        assert_eq!(scan("1234"), Some(Token::Int(1234)));
        assert_eq!(scan("0"), Some(Token::Int(0)));
    }

    #[test]
    fn test_char_literal() {
        assert_eq!(scan("'a'"), Some(Token::Char('a')));
        assert_eq!(scan("'Z'"), Some(Token::Char('Z')));
        assert_eq!(scan("'1'"), Some(Token::Char('1')));
    }

    #[test]
    fn test_braces() {
        assert_eq!(scan("{"), Some(Token::LCurly));
        assert_eq!(scan("}"), Some(Token::RCurly));
    }

    #[test]
    fn test_whitespace_skipping() {
        let input = "   turn";
        assert_eq!(scan(input), Some(Token::Turn));
    }

    #[test]
    fn test_tokenize_full_input() {
        let input = "turn move 123 { 'x' }";
        let tokens = tokenize(input);
        let expected = vec![
            Token::Turn,
            Token::Move,
            Token::Int(123),
            Token::LCurly,
            Token::Char('x'),
            Token::RCurly,
        ];
        assert_eq!(tokens, expected);
    }

    #[test]
    #[should_panic]
    fn test_unrecognized_token_should_panic() {
        scan("@");
    }

    #[test]
    #[should_panic]
    fn test_invalid_char_literal_missing_closing_quote() {
        scan("'a");
    }

    #[test]
    #[should_panic]
    fn test_invalid_char_literal_empty() {
        scan("''");
    }

    #[test]
    fn test_tokenize_with_newlines_and_spaces() {
        let input = "   if\n123\n{\n'a'\n}";
        let tokens = tokenize(input);
        let expected = vec![
            Token::If,
            Token::Int(123),
            Token::LCurly,
            Token::Char('a'),
            Token::RCurly,
        ];
        assert_eq!(tokens, expected);
    }
}
