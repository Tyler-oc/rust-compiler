use crate::lexing::token::Token;
#[derive(Debug)]
pub enum Expr {
    //remember to change Box to Rc (shared ownership) or Arc (multi thread)
    Binary(Box<Expr>, Token, Box<Expr>),
    Unary(Box<Expr>, Token),
    Grouping(Box<Expr>),
    Literal(Token), //expect this to be a string, int, or boolean tokens
}

impl Expr {}
