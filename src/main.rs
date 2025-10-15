// use crate::parser::scan_token;
use crate::parser::tokenize;
use std::io::{self};
use std::path::Path;

mod parser;

fn main() -> io::Result<()>{
    let _path = Path::new("/home/delilah/Documents/Projects/rover/src/test.txt");
    tokenize("turn 9 { }");
    Ok(())
}
