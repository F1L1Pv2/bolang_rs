#[derive(Debug,Clone, PartialEq)]
pub enum TokenType{
    Ident, // var name
    IntLit, // 0 - 2137 etc
    Dots, // :
    OParen, // (
    ClParen, // )
    OCurly,  // {
    ClCurcly, // }
    Semi,  // ;
    Coma, // ,
    Minus, // -
    Greater, // <
    Lower, // >
}

#[derive(Debug, Clone)]
pub struct Token{
    pub token_type: TokenType,
    pub value: String
}

impl Token{
    pub fn new(token_type: TokenType, value: String) -> Token {
        Token{
            token_type,
            value
        }
    }
}

// -----------------------------------------------------------------

#[derive(Debug, Clone)]
pub struct  ExprIntLit{
    pub value: String
}

#[derive(Debug, Clone)]
pub enum Expr{
    IntLit(ExprIntLit)
}

#[derive(Debug, Clone)]
pub struct TermReturn{
    pub value: Expr
}

#[derive(Debug, Clone)]
pub enum Term{
    Return(TermReturn)
}

#[derive(Debug, Clone)]
pub struct FunctionDecl{
    pub name: String,
    pub terms: Vec<Term>
}

#[derive(Debug, Clone)]
pub struct RootNode{
    pub functions: Vec<FunctionDecl>
}
