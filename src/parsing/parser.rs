use crate::{
    errors::parseError::ParseError,
    lexing::token::Token,
    parsing::ast::{BinaryOp, Literal, UnaryOp},
};

pub fn parse_binary_op(token: &Token) -> Result<BinaryOp, ParseError> {
    match token {
        Token::And => Ok(BinaryOp::And),
        Token::Or => Ok(BinaryOp::Or),
        Token::Plus => Ok(BinaryOp::Plus),
        Token::Minus => Ok(BinaryOp::Minus),
        Token::Star => Ok(BinaryOp::Star),
        Token::Slash => Ok(BinaryOp::Slash),
        Token::GreaterEqual => Ok(BinaryOp::GreaterEqual),
        Token::GreaterThan => Ok(BinaryOp::GreaterThan),
        Token::EqualEqual => Ok(BinaryOp::EqualEqual),
        Token::BangEqual => Ok(BinaryOp::BangEqual),
        Token::LessEqual => Ok(BinaryOp::LessEqual),
        Token::LessThan => Ok(BinaryOp::LessThan),
        Token::Equal => Ok(BinaryOp::Equal),
        _ => Err(ParseError::InvalidConversion(
            "could not convert to binary operator".to_string(),
        )), //figure out formatting later to enter to display tokens
    }
}

pub fn parse_unary_op(token: &Token) -> Result<UnaryOp, ParseError> {
    match token {
        Token::Bang => Ok(UnaryOp::Bang),
        _ => Err(ParseError::InvalidConversion(
            "could not convert to unary operator".to_string(),
        )),
    }
}

pub fn parse_literal(token: &Token) -> Result<Literal, ParseError> {
    match token {
        Token::Number(i) => Ok(Literal::Number(*i)),
        Token::StringLiteral(s) => Ok(Literal::StringLiteral(s.clone())),
        Token::Boolean(b) => Ok(Literal::Boolean(*b)),
        Token::Null => Ok(Literal::Null),
        _ => Err(ParseError::InvalidConversion(
            "could not convert literal".to_string(),
        )),
    }
}
