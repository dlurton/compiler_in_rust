
extern crate compiler_in_rust_lib;

use compiler_in_rust_lib::*;
use compiler_in_rust_lib::value::*;
use compiler_in_rust_lib::env::*;

#[test]
fn simple_expression_1() {
   assert_eq!(Ok(Value::Int32(2)), execute("1 + 1"))
}

#[test]
fn simple_expression_2() {
    assert_eq!(Ok(Value::Int32(4)), execute("2 * 2"))
}

#[test]
fn simple_expression_3() {
    assert_eq!(Ok(Value::Int32(5)), execute("2 * 2 + 1"))
}

#[test]
fn expression_with_env() {
    let builder = EnvDefBuilder::new();
    let env_def = builder
        .with_item("foo", Value::Int32(100))
        .with_item("bar", Value::Int32(200))
        .build();

    assert_eq!(Ok(Value::Int32(300)), execute_with_globals("foo + bar", &env_def));
}

#[test]
fn compount_expr() {
    //The resulting value is always the last expression in a compound expression.

    //Expressions "1", "2", "3"
    assert_eq!(Ok(Value::Int32(3)), execute("1 2 3"));
    //Expressions "1", "2", "3 * 4"
    assert_eq!(Ok(Value::Int32(12)), execute("1 2 3 * 4"));
    //Expressions "1", "2 * 4", "3"
    assert_eq!(Ok(Value::Int32(3)), execute("1 2 * 4 3"));
    //Expressions "1 * 4", "2", "3"
    assert_eq!(Ok(Value::Int32(3)), execute("1 * 4 2 3"));
}
