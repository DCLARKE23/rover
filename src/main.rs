// use crate::parser::scan_token;
use crate::parser::tokenize;
use std::fs::File;
use std::io::{self, Read};
use std::path::Path;

mod parser;

fn main() -> io::Result<()>{
    let path = Path::new("/home/delilah/Documents/Projects/rover/src/test.txt");
    tokenize("hello world");
    Ok(())
}
