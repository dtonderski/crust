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
        r#"    .globl {function_name}
{function_name}:
{function_body}"#
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
            write!(
                &mut buf,
                r#"{expression}
    ret
"#
            )?;
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
        },
        Expression::UnaryOperation {
            operator,
            expression,
        } => {
            let expression_string = generate_expression(&expression)?;
            let operation = generate_operation(&operator);
            write!(
                buf,
                r#"{expression_string}
{operation}"#)?;
        },
    };
    return Ok(buf);
}

fn generate_operation(operator: &UnaryOperator) -> String {
    return match operator {
        UnaryOperator::Negation => "    neg     %eax".to_string(),
        UnaryOperator::BinaryComplement => "    not     %eax".to_string(),
        UnaryOperator::LogicalNegation => r#"    cmpl    $0, %eax
    movl    $0, %eax
    sete    %al"#.to_string(),
    };
}

fn generate_constant(constant: &Constant) -> Result<String, GenError> {
    let mut buf = String::new();
    match constant {
        Constant::Int(int) => {
            write!(buf, r#"    movl    ${int}, %eax"#)?;
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
