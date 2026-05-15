use std::fmt::Write;

use crate::parser::ast::{
    Constant, Expression, FunctionDeclaration, Program, Statement, UnaryOperator,
};

#[derive(Debug)]
pub enum GenError {
    Format(std::fmt::Error),
}

impl From<std::fmt::Error> for GenError {
    fn from(value: std::fmt::Error) -> Self {
        return GenError::Format(value);
    }
}

pub fn generate(program: &Program) -> Result<String, GenError> {
    let function = &program.function;
    let function_body = generate_function_body(function)?;
    let function_name = &function.name;
    let mut buf = String::new();
    write!(
        &mut buf,
        "    .globl {function_name}\n{function_name}:\n{function_body}"
    )?;
    return Ok(buf);
}

fn generate_function_body(fun_dec: &FunctionDeclaration) -> Result<String, GenError> {
    let statement = generate_statement(&fun_dec.statement)?;
    return Ok(statement);
}

fn generate_statement(statement: &Statement) -> Result<String, GenError> {
    let mut buf = String::new();

    match statement {
        Statement::Return(expression) => {
            let expression = generate_expression(&expression)?;
            write!(&mut buf, "{expression}\n    ret\n")?;
        }
    }
    return Ok(buf);
}

fn generate_expression(expression: &Expression) -> Result<String, GenError> {
    let mut buf = String::new();
    match expression {
        Expression::Constant(constant) => {
            let constant = generate_constant(&constant)?;
            write!(buf, "{constant}")?;
        }
        Expression::UnaryOperation {
            operator,
            expression,
        } => {
            let expression_string = generate_expression(&expression)?;
            let operation = generate_operation(&operator);
            write!(buf, "{expression_string}\n{operation}")?;
        }
        Expression::BinaryOperation {
            operator,
            left,
            right,
        } => todo!(),
    };
    return Ok(buf);
}

fn generate_operation(operator: &UnaryOperator) -> &'static str {
    return match operator {
        UnaryOperator::Negation => "    neg     %eax",
        UnaryOperator::BinaryComplement => "    not     %eax",
        UnaryOperator::LogicalNegation => {
            "    cmpl    $0, %eax\n    movl    $0, %eax\n    sete    %al"
        }
    };
}

fn generate_constant(constant: &Constant) -> Result<String, GenError> {
    let mut buf = String::new();
    match constant {
        Constant::Int(int) => {
            write!(buf, "    movl    ${int}, %eax")?;
        }
    };
    return Ok(buf);
}

#[cfg(test)]
mod tests {
    use super::generate;
    use crate::parser::ast::{
        Constant, Expression, FunctionDeclaration, Program, Statement, UnaryOperator,
    };

    #[test]
    fn generates_return_constant_function() {
        let program = Program {
            function: FunctionDeclaration {
                name: "main".to_string(),
                statement: Statement::Return(Expression::UnaryOperation {
                    operator: UnaryOperator::Negation,
                    expression: Box::new(Expression::Constant(Constant::Int(2))),
                }),
            },
        };

        assert_eq!(
            generate(&program).expect("codegen should succeed"),
            r#"    .globl main
main:
    movl    $2, %eax
    neg     %eax
    ret
"#
        );
    }
}
