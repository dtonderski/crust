use std::fmt::Write;

use crate::parser::ast::{
    BinaryOperator, Constant, Expression, FunctionDeclaration, Program, Statement, UnaryOperator,
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

#[cfg(test)]
mod tests {
    use super::generate;
    use crate::parser::ast::{
        BinaryOperator, Constant, Expression, FunctionDeclaration, Program, Statement,
        UnaryOperator,
    };
    use std::{
        fs,
        path::{Path, PathBuf},
        process::Command,
        time::{SystemTime, UNIX_EPOCH},
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

    #[test]
    fn generates_addition() {
        assert_binary_operation_result(BinaryOperator::Addition, 2, 3, 5);
    }

    #[test]
    fn generates_subtraction() {
        assert_binary_operation_result(BinaryOperator::Subtraction, 7, 4, 3);
    }

    #[test]
    fn generates_multiplication() {
        assert_binary_operation_result(BinaryOperator::Multiplication, 3, 4, 12);
    }

    #[test]
    fn generates_division() {
        assert_binary_operation_result(BinaryOperator::Division, 9, 3, 3);
    }

    #[test]
    fn generates_modulo() {
        assert_binary_operation_result(BinaryOperator::Modulo, 10, 4, 2);
    }

    fn assert_binary_operation_result(
        operator: BinaryOperator,
        left: i64,
        right: i64,
        expected: i32,
    ) {
        let program = Program {
            function: FunctionDeclaration {
                name: "main".to_string(),
                statement: Statement::Return(Expression::BinaryOperation {
                    operator,
                    left: Box::new(Expression::Constant(Constant::Int(left))),
                    right: Box::new(Expression::Constant(Constant::Int(right))),
                }),
            },
        };

        let assembly = generate(&program).expect("codegen should succeed");
        let executable = compile_assembly(&assembly);
        let output = Command::new(&executable)
            .output()
            .expect("generated executable should run");

        assert_eq!(output.status.code(), Some(expected));
        cleanup_generated_files(&executable);
    }

    fn compile_assembly(assembly: &str) -> PathBuf {
        let dir = std::env::temp_dir();
        let id = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("system time should be after unix epoch")
            .as_nanos();
        let assembly_path = dir.join(format!("crust_codegen_test_{id}.s"));
        let executable_path = dir.join(format!("crust_codegen_test_{id}"));

        fs::write(&assembly_path, assembly).expect("assembly file should be written");
        let output = Command::new("gcc")
            .arg(&assembly_path)
            .arg("-o")
            .arg(&executable_path)
            .output()
            .expect("gcc should run");

        assert!(
            output.status.success(),
            "gcc failed:\n{}",
            String::from_utf8_lossy(&output.stderr)
        );

        fs::remove_file(&assembly_path).expect("assembly file should be removed");
        return executable_path;
    }

    fn cleanup_generated_files(executable: &Path) {
        fs::remove_file(executable).expect("generated executable should be removed");
    }
}
