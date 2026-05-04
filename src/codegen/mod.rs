use std::fmt::Write;

use crate::parser::ast::{Constant, Expression, FunctionDeclaration, Program, Statement};

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
    let mut buf = String::new();
    let statement = generate_statement(&fun_dec.statement)?;
    write!(
        &mut buf,
        r#"    movl ${statement}, %eax
    ret
"#
    )?;
    return Ok(buf);
}

fn generate_statement(statement: &Statement) -> Result<String, GenError> {
    return match statement {
        Statement::Return(expression) => generate_expression(&expression),
    };
}

fn generate_expression(expression: &Expression) -> Result<String, GenError> {
    return match expression {
        Expression::Constant(constant) => generate_constant(&constant),
    };
}

fn generate_constant(constant: &Constant) -> Result<String, GenError> {
    return match constant {
        Constant::Int(int) => Ok(int.to_string()),
    };
}

#[cfg(test)]
mod tests {
    use super::generate;
    use crate::parser::ast::{Constant, Expression, FunctionDeclaration, Program, Statement};

    #[test]
    fn generates_return_constant_function() {
        let program = Program {
            function: FunctionDeclaration {
                name: "main".to_string(),
                statement: Statement::Return(Expression::Constant(Constant::Int(2))),
            },
        };

        assert_eq!(
            generate(&program).expect("codegen should succeed"),
            r#"    .globl main
main:
    movl $2, %eax
    ret
"#
        );
    }
}
