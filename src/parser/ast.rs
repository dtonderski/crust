pub enum Constant {
    Int(i64),
}

pub enum Expression {
    Constant(Constant),
}

pub enum Statement {
    Return(Expression),
}

pub struct FunctionDeclaration {
    pub name: String,
    pub statement: Statement,
}

pub struct Program {
    pub function: FunctionDeclaration,
}
