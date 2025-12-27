use std::iter;

use crate::lexing::token::Token;
#[derive(Debug)]
pub enum Expr {
    //remember to change Box to Rc (shared ownership) or Arc (multi thread)
    Binary{
        left: Box<Expr>,
        op: BinaryOp,
        right: Box<Expr>
    },
    Unary{
        exp: Box<Expr>,
        op: UnaryOp
    },
    Grouping{exp: Box<Expr>},
    Literal(Value), //expect this to be a string, int, or boolean tokens
}

#[derive(Debug)]
pub enum BinaryOp {
    Plus,
    Minus,
    Star, 
    Slash,
    Equal,
    GreaterEqual,
    GreaterThan,
    EqualEqual,
    BangEqual,
    LessEqual,
    LessThan,
    And,
    Or,
}

impl std::fmt::Display for BinaryOp {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            BinaryOp::Plus => "+",
            BinaryOp::Minus => "-",
            BinaryOp::Star => "*",
            BinaryOp::Slash => "/",
            BinaryOp::And => "&&",
            BinaryOp::Or => "||",
            BinaryOp::Equal => "=",
            BinaryOp::GreaterEqual => ">=",
            BinaryOp::GreaterThan => ">",
            BinaryOp::EqualEqual => "==",
            BinaryOp::BangEqual => "!=",
            BinaryOp::LessEqual => "<=",
            BinaryOp::LessThan => "<",
        };
        write!(f, "{}", s)
    }
}

#[derive(Debug)]
pub enum UnaryOp {
    Bang,
}

impl std::fmt::Display for UnaryOp {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            UnaryOp::Bang => "!",
        };
        write!(f, "{}", s)
    }
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

    fn parenthesize(name: String, exprs: Vec<Expr>) -> String {
        result = "(";
        for exp in exprs.iter() {
            result.push(" ");
            result.push_str(exp.get_string())
        }
        result.push(")");
        return result;
    }

    fn get_string(exp: Expr) -> String {
        match exp {
            Expr::Binary(l, op, r) => return parenthesize(op)
        }
    }

    fn parse_binary_op(token: &Token) -> BinaryOp {
        match token.kind {
            
        }
    }
}
