struct Interpreter<'a> {
    expr: Expr,
}

impl<'a> Interpreter<'a> {
    fn literal(&mut self) -> Result<Value, InterpreterError> {
        self.expr
    }
}
