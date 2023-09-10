use std::path::Path;
use std::process::exit;
use std::env;
use std::fs;

mod common;
mod lexer;
mod parser;
mod code_gen;

use common::*;




fn main() {

    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!("USAGE: {} <input file>", args[0]);
        exit(1);
    }
    
    let path = &args[1];

    if !path.ends_with("bo"){
        println!("BoLang only accepts files with .bo");
        exit(1);
    }

    let path = Path::new(path);



    let source_file = fs::read_to_string(path).unwrap();

    let tokens: Vec<Token> = lexer::lex_file(source_file);

    let parser: RootNode = parser::parse_lexer(tokens);

    code_gen::generate_asm(parser, &args[1]);


}
