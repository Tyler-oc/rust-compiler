use crate::lexing::token::Token;
#[derive(Debug)]
pub enum Expr {
    //remember to change Box to Rc (shared ownership) or Arc (multi thread)
    Binary(Box<Expr>, Token, Box<Expr>),
    Unary(Box<Expr>, Token),
    Grouping(Box<Expr>),
    Literal(Token), //expect this to be a string, int, or boolean tokens
}

impl Expr {

    pub fn print_AST(self) -> () {
        match self {
            Expr::Binary(left, op, right) => ,
            Expr::Unary(e, op) => ,
            Expr::Grouping(e) => ,
            Expr::Literal(t) => {
                t match {
                    Token::IntegerLiteral(i) => ,
                    Token::StringLiteral(s) => ,
                    Token::Boolean(b) => ,
                    Token::Null(_) => ,
                }
            },
        }
    }
}
