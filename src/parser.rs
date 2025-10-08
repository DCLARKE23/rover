use std::fs::{read_to_string, File};
use std::io::{self, Read};
use std::path::Path;
use std::str::Chars;

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
    file.read_to_string(&mut contents);
    contents
}

pub fn scan_token (chars: &mut Chars) -> Option<Token> {
    match chars.next() {
        Some(c) if c.is_alphabetic() => {
            let mut buf = String::new();
            buf.push(c);
            while let Some(x) = chars.next().filter(|c| c.is_alphabetic()){
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
            while let Some(x) = chars.next().filter(|c| c.is_ascii_digit()){
                buf.push(x);
            }
            Some(Token::Int(buf.parse().unwrap()))
        }
        _ => todo!(),
    }
}


pub fn tokenize (s: &str) -> Vec<Token> {
    let mut chars = s.chars();
    let mut tokens = Vec::new();
    while let Some(t) = scan_token(&mut chars){
        tokens.push(t);
    }
    tokens

}
// pub struct ParseError;

// pub fn scan_token(input: &str) -> Result<Option<Token>, ParseError> {
//     let mut words = input
//         .split_inclusive(|c: char| c.is_whitespace() || c == '{' || c == '}')
//         .map(|c| c.trim());

//     let x: Vec<_> = words.collect();
//     println!("{:?}", x);
//     Err(ParseError)

    // match words.next() {
    //     Some("turn") => {
    //         Ok(Some(Token::Turn))
    //     }
    //     Some("move") => {
    //         Ok(Some(Token::Move))
    //     }
    //     None => Ok(None)
    // }
// }

