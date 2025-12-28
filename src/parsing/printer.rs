

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