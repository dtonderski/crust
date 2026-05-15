use super::generate;
use crate::parser::ast::{
    BinaryOperator, Constant, Expression, FunctionDeclaration, Program, Statement, UnaryOperator,
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
    assert_binary_operation_result(BinaryOperator::Division, 14, 3, 4);
}

#[test]
fn generates_modulo() {
    assert_binary_operation_result(BinaryOperator::Modulo, 14, 3, 2);
}

fn assert_binary_operation_result(operator: BinaryOperator, left: i64, right: i64, expected: i32) {
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
