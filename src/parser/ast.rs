#[derive(Debug, PartialEq, Eq)]
pub enum Constant {
    Int(i64),
}

#[derive(Debug, PartialEq, Eq)]
pub enum UnaryOperator {
    Negation,
    BinaryComplement,
    LogicalNegation,
}

#[derive(Debug, PartialEq, Eq)]
pub enum Expression {
    Constant(Constant),
    UnaryOperation {
        operator: UnaryOperator,
        expression: Box<Expression>,
    },
}

#[derive(Debug, PartialEq, Eq)]
pub enum Statement {
    Return(Expression),
}

#[derive(Debug, PartialEq, Eq)]
pub struct FunctionDeclaration {
    pub name: String,
    pub statement: Statement,
}

#[derive(Debug, PartialEq, Eq)]
pub struct Program {
    pub function: FunctionDeclaration,
}
