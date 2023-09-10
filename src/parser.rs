use crate::common::*;
use std::process::exit;

fn recursive_check(tokens: &Vec<Token>, i: &mut usize, structure: Vec<TokenType>) -> Option<Token> {
    for n in 0..structure.len() {
        let token = tokens.get(*i).unwrap();

        if token.token_type != *structure.get(n).unwrap() {
            println!("Expecred {:?} got {:?}", structure.get(n).unwrap(), token.token_type);
            return None;
        }

        *i += 1;
    }

    Some(tokens.get(*i).unwrap().to_owned())
}

fn parse_expr(tokens: &Vec<Token>, i: &mut usize) -> Option<Expr> {
    let token = tokens.get(*i).unwrap();
    let val = token.value.clone();
    match token.token_type {
        TokenType::IntLit => {
            *i += 1;
            return Some(Expr::IntLit(ExprIntLit { value: val }));
        }
        _ => {
            println!("Not implemented");
            exit(1)
        }
    }
}

fn parse_return(tokens: &Vec<Token>, i: &mut usize) -> Option<TermReturn>{

    recursive_check(tokens, i, [TokenType::Greater, TokenType::Minus].to_vec()).unwrap();

    let expr = parse_expr(tokens, i);

    match expr{
        Some(val) => {
            return Some(TermReturn{value: val});
        }
        None => {
            return None;
        }
    }

}

fn parse_term(tokens: &Vec<Token>, i: &mut usize) -> Option<Term> {
    
    let starting_i = *i;

    let term_return = parse_return(tokens, i);

    let semi = recursive_check(tokens, i, [TokenType::Semi].to_vec());

    match semi{
        Some(_) => {}
        None => {
            println!("Expected ;");
            return None;
        }
    }

    match term_return{
        Some(val) => {
            return Some(Term::Return(val));
        }
        None => {
            *i = starting_i;
        }
    }

    println!("Unreachable");
    exit(1);
}

fn parse_block(tokens: &Vec<Token>, i: &mut usize) -> Option<Vec<Term>> {
    let mut terms: Vec<Term> = Vec::new();

    if tokens.get(*i).unwrap().token_type != TokenType::OCurly{
        return None;
    }

    *i += 1;

    while tokens.get(*i).unwrap().token_type != TokenType::ClCurcly {
        let term = parse_term(tokens, i);
        match term {
            Some(val) => {
                terms.push(val);
            }
            None => {
                return None;
            }
        }
    }

    *i += 1;

    Some(terms)
}

fn parse_function_decl(tokens: &Vec<Token>, i: &mut usize) -> Option<FunctionDecl> {
    let function_name = tokens.get(*i).unwrap().value.clone();
    *i += 1;

    let out_bruv = recursive_check(
        tokens,
        i,
        [TokenType::Dots, TokenType::Dots, TokenType::OParen, TokenType::ClParen].to_vec()
    );

    match out_bruv {
        Some(val) => {
            if val.token_type == TokenType::OCurly {
                let terms = parse_block(tokens, i);
                match terms {
                    Some(val) => {
                        let function = FunctionDecl {
                            name: function_name,
                            terms: val,
                        };

                        return Some(function);
                    }
                    None => {
                        println!("Couldn't parse block");
                    }
                }
            } else {
                println!("Expected block");
                return None;
            }
        }
        None => {
            println!("Couldn't parse function {}", function_name);
            return None;
        }
    }

    None
}

pub fn parse_lexer(tokens: Vec<Token>) -> RootNode {
    let mut root_node = RootNode {
        functions: Vec::new(),
    };

    let mut i: usize = 0;
    let len: usize = tokens.len();
    while i < len {
        let token = tokens.get(i).unwrap();

        if token.token_type == TokenType::Ident {
            let fucntion = parse_function_decl(&tokens, &mut i);
            match fucntion {
                Some(val) => {
                    root_node.functions.push(val);
                }
                None => {
                    println!("Couldn't parse function");
                }
            }
        }

        i += 1;
    }

    root_node
}
