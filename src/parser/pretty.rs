use std::fmt::Write;

use super::ast::{
    BinaryOperator, Constant, Expression, FunctionDeclaration, Program, Statement, UnaryOperator,
};

impl Program {
    pub fn pretty_print(&self) -> String {
        let mut buf = String::new();
        self.write_pretty(&mut buf, 0);
        return buf;
    }

    fn write_pretty(&self, buf: &mut String, indent: usize) {
        write_line(buf, indent, "Program");
        self.function.write_pretty(buf, indent + 1);
    }
}

impl FunctionDeclaration {
    fn write_pretty(&self, buf: &mut String, indent: usize) {
        write_line(buf, indent, &format!("Function {}", self.name));
        self.statement.write_pretty(buf, indent + 1);
    }
}

impl Statement {
    fn write_pretty(&self, buf: &mut String, indent: usize) {
        match self {
            Statement::Return(expression) => {
                write_line(buf, indent, "Return");
                expression.write_pretty(buf, indent + 1);
            }
        }
    }
}

impl Expression {
    fn write_pretty(&self, buf: &mut String, indent: usize) {
        match self {
            Expression::Constant(constant) => {
                write_line(buf, indent, &format!("Constant {}", constant.pretty_name()));
            }
            Expression::UnaryOperation {
                operator,
                expression,
            } => {
                write_line(
                    buf,
                    indent,
                    &format!("UnaryOperation {}", operator.pretty_name()),
                );
                expression.write_pretty(buf, indent + 1);
            }
            Expression::BinaryOperation {
                operator,
                left,
                right,
            } => {
                write_line(
                    buf,
                    indent,
                    &format!("BinaryOperation {}", operator.pretty_name()),
                );
                left.write_pretty(buf, indent + 1);
                right.write_pretty(buf, indent + 1);
            }
        }
    }
}

impl Constant {
    fn pretty_name(&self) -> String {
        match self {
            Constant::Int(value) => value.to_string(),
        }
    }
}

impl UnaryOperator {
    fn pretty_name(&self) -> &'static str {
        match self {
            UnaryOperator::Negation => "Negation",
            UnaryOperator::BinaryComplement => "BinaryComplement",
            UnaryOperator::LogicalNegation => "LogicalNegation",
        }
    }
}

impl BinaryOperator {
    fn pretty_name(&self) -> &'static str {
        match self {
            BinaryOperator::Multiplication => "Multiplication",
            BinaryOperator::Division => "Division",
            BinaryOperator::Modulo => "Modulo",
            BinaryOperator::Addition => "Addition",
            BinaryOperator::Subtraction => "Subtraction",
            BinaryOperator::LessThan => "LessThan",
            BinaryOperator::LessThanOrEqual => "LessThanOrEqual",
            BinaryOperator::GreaterThan => "GreaterThan",
            BinaryOperator::GreaterThanOrEqual => "GreaterThanOrEqual",
            BinaryOperator::Equal => "Equal",
            BinaryOperator::NotEqual => "NotEqual",
            BinaryOperator::LogicalAnd => "LogicalAnd",
            BinaryOperator::LogicalOr => "LogicalOr",
        }
    }
}

fn write_line(buf: &mut String, indent: usize, text: &str) {
    for _ in 0..indent {
        buf.push_str("  ");
    }
    writeln!(buf, "{text}").expect("writing to a String should not fail");
}

#[cfg(test)]
mod tests {
    use super::{
        BinaryOperator, Constant, Expression, FunctionDeclaration, Program, Statement,
        UnaryOperator,
    };

    #[test]
    fn pretty_prints_program_tree() {
        let program = Program {
            function: FunctionDeclaration {
                name: "main".to_string(),
                statement: Statement::Return(Expression::BinaryOperation {
                    operator: BinaryOperator::Multiplication,
                    left: Box::new(Expression::UnaryOperation {
                        operator: UnaryOperator::Negation,
                        expression: Box::new(Expression::Constant(Constant::Int(2))),
                    }),
                    right: Box::new(Expression::Constant(Constant::Int(3))),
                }),
            },
        };

        assert_eq!(
            program.pretty_print(),
            r#"Program
  Function main
    Return
      BinaryOperation Multiplication
        UnaryOperation Negation
          Constant 2
        Constant 3
"#
        );
    }
}
