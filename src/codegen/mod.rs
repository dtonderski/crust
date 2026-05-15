use std::fmt::Write;

use crate::parser::ast::{
    BinaryOperator, Constant, Expression, FunctionDeclaration, Program, Statement, UnaryOperator,
};

#[cfg(test)]
mod tests;

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
        } => {
            let operation_string = generate_binary_operation(operator, left, right)?;
            buf = operation_string;
        }
    };
    return Ok(buf);
}

fn generate_binary_operation(
    operator: &BinaryOperator,
    left: &Expression,
    right: &Expression,
) -> Result<String, GenError> {
    let mut buf = String::new();
    let left_str = generate_expression(left)?;
    let right_str = generate_expression(right)?;
    match operator {
        BinaryOperator::Addition | BinaryOperator::Multiplication => {
            let operator_str = match operator {
                BinaryOperator::Addition => "addl",
                BinaryOperator::Multiplication => "imul",
                _ => unreachable!(),
            };

            write!(
                buf,
                "{left_str}\n    pushq   %rax\n{right_str}\n    popq    %rcx\n    {operator_str}    %ecx, %eax"
            )?;
        }
        BinaryOperator::Subtraction => {
            write!(
                buf,
                "{right_str}\n    pushq   %rax\n{left_str}\n    popq    %rcx\n    subl    %ecx, %eax"
            )?;
        }
        BinaryOperator::Division | BinaryOperator::Modulo => {
            write!(
                buf,
                "{right_str}\n    pushq   %rax\n{left_str}\n    popq    %rcx\n    cdq\n    idivl   %ecx"
            )?;
            if *operator == BinaryOperator::Modulo {
                write!(buf, "\n    movl    %edx, %eax")?;
            }
        }
        BinaryOperator::LessThan
        | BinaryOperator::LessThanOrEqual
        | BinaryOperator::GreaterThan
        | BinaryOperator::GreaterThanOrEqual
        | BinaryOperator::Equal
        | BinaryOperator::NotEqual
        | BinaryOperator::LogicalAnd
        | BinaryOperator::LogicalOr => todo!(),
    }
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
