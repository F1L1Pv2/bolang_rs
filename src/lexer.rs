use crate::common::*;

pub fn lex_file(source_file: String) -> Vec<Token> {
    let mut tokens: Vec<Token> = Vec::new();

    let mut i: usize = 0;
    let file_len: usize = source_file.len();

    while i < file_len {
        let ch = source_file.chars().nth(i).unwrap();

        if !ch.is_whitespace() {
            // print!("{ch}");
            if ch.is_alphanumeric() {

                let start_i = i;

                let is_digit = ch.is_digit(10);

                while
                    !source_file.chars().nth(i).unwrap().is_whitespace() &&
                    source_file.chars().nth(i).unwrap().is_alphanumeric()
                {
                    i += 1;
                }

                if !is_digit{
                    tokens.push(Token::new(TokenType::Ident, source_file[start_i..i].to_string()));
                }else{
                    tokens.push(Token::new(TokenType::IntLit, source_file[start_i..i].to_string()));
                }

                i -= 1;

            } else {
                match ch {
                    ':' => {
                        tokens.push(Token::new(TokenType::Dots, ch.to_string()));
                    }

                    '(' => {
                        tokens.push(Token::new(TokenType::OParen, ch.to_string()));
                    }

                    ')' => {
                        tokens.push(Token::new(TokenType::ClParen, ch.to_string()));
                    }

                    '{' => {
                        tokens.push(Token::new(TokenType::OCurly, ch.to_string()));
                    }

                    '}' => {
                        tokens.push(Token::new(TokenType::ClCurcly, ch.to_string()));
                    }

                    '-' => {
                        tokens.push(Token::new(TokenType::Minus, ch.to_string()));
                    }

                    '<' => {
                        tokens.push(Token::new(TokenType::Greater, ch.to_string()));
                    }

                    '>' => {
                        tokens.push(Token::new(TokenType::Lower, ch.to_string()));
                    }

                    ';' => {
                        tokens.push(Token::new(TokenType::Semi, ch.to_string()));
                    }

                    ',' => {
                        tokens.push(Token::new(TokenType::Coma, ch.to_string()));
                    }

                    _ => {
                        println!("Couldn't lex this char: {}", ch);
                    }
                }
            }
        }

        i += 1;
    }

    tokens
}
