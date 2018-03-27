
extern crate compiler_in_rust_lib;

use compiler_in_rust_lib::*;
use compiler_in_rust_lib::value::*;
use compiler_in_rust_lib::ast::*;
use compiler_in_rust_lib::passes::*;

#[test]
fn simple_expression_1() {
   assert_eq!(Value::Int32(2), execute("1 + 1"))
}

#[test]
fn simple_expression_2() {
    assert_eq!(Value::Int32(4), execute("2 * 2"))
}

#[test]
fn simple_expression_3() {
    assert_eq!(Value::Int32(5), execute("2 * 2 + 1"))
}

/*#[test]
fn expression_with_env() {
    let builder = EnvDefBuilder::new();
    let env_def = builder
        .with_item("foo", DataType::Int32)
        .with_item("bar", DataType::Int32)
        .build();

    let env = vec![Value::Int32(100), Value::Int32(200)];

    let initial_ast = parse("foo + bar");
    let modified_ast = resolve_indexes(initial_ast, &env_def);

    assert_eq!(Value::Int32(300), evaluate(&modified_ast, &env));
}*/

#[test]
fn expression_with_env() {
    let builder = EnvDefBuilder::new();
    let env_def = builder
        .with_item("foo", Value::Int32(100))
        .with_item("bar", Value::Int32(200))
        .build();

    let default_env = env_def.create_with_default_values();

    let initial_ast = parse("foo + bar");
    let modified_ast = resolve_indexes(initial_ast, &env_def);

    assert_eq!(Value::Int32(300), evaluate(&modified_ast, &default_env));
}

