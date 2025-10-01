
enum Vocab{
    
}

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

pub struct ParseError;

pub fn scan_token(input: &str) -> Result<Option<Token>, ParseError> {
    let mut words = input
        .split_inclusive(|c: char| c.is_whitespace() || c == '{' || c == '}')
        .map(|c| c.trim());

    let x: Vec<_> = words.collect();
    println!("{:?}", x);
    Err(ParseError)

    // match words.next() {
    //     Some("turn") => {
    //         Ok(Some(Token::Turn))
    //     }
    //     Some("move") => {
    //         Ok(Some(Token::Move))
    //     }
    //     None => Ok(None)
    // }
}