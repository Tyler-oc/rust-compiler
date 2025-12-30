#[derive(Debug)]
pub enum Expr {
    //remember to change Box to Rc (shared ownership) or Arc (multi thread)
    Binary {
        left: Box<Expr>,
        op: BinaryOp,
        right: Box<Expr>,
    },
    Unary {
        exp: Box<Expr>,
        op: UnaryOp,
    },
    Grouping {
        exp: Box<Expr>,
    },
    Literal(Literal), //expect this to be a string, int, or boolean tokens
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

#[derive(Debug)]
pub enum UnaryOp {
    Bang,
}

#[derive(Debug)]
pub enum Literal {
    IntegerLiteral(i32),
    StringLiteral(String),
    Null,
    Boolean(bool),
}

//display implementations for enums

impl std::fmt::Display for Expr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Expr::Binary { left, op, right } => {
                write!(f, "{}", left);
                write!(f, "{}", op);
                write!(f, "{}", right)
            }
            Expr::Unary { exp, op } => {
                write!(f, "{}", op);
                write!(f, "{}", exp)
            }
            Expr::Grouping { exp } => {
                write!(f, "({})", exp)
            }
            Expr::Literal(val) => {
                write!(f, "{}", val)
            }
        }
    }
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

impl std::fmt::Display for UnaryOp {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            UnaryOp::Bang => "!",
        };
        write!(f, "{}", s)
    }
}

impl std::fmt::Display for Literal {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            Literal::IntegerLiteral(val) => val.to_string(),
            Literal::StringLiteral(val) => val.to_string(),
            Literal::Null => "NULL".to_string(),
            Literal::Boolean(val) => val.to_string(),
        };
        write!(f, "{}", s)
    }
}
