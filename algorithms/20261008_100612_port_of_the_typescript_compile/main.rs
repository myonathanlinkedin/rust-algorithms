mod types;
mod engine;

use types::types::{Expr, SimpleType, Stmt};
use engine::engine::TypeChecker;

fn main() {
    // Simple function: foo(x: number): number { return x; }
    let foo = Stmt::FuncDecl {
        name: "foo".to_string(),
        params: vec![("x".to_string(), SimpleType::Number)],
        ret_type: SimpleType::Number,
        body: vec![Stmt::Return(Some(Expr::Identifier("x".to_string())))],
    };

    let mut checker = TypeChecker::new();
    assert!(checker.check_stmt(&foo).is_ok());

    // Variable declaration with mismatched initializer
    let bad_var = Stmt::VarDecl {
        name: "y".to_string(),
        typ: SimpleType::String,
        init: Some(Expr::NumberLit(42.0)),
    };
    assert!(checker.check_stmt(&bad_var).is_err());

    // Nested function with correct types
    let outer = Stmt::FuncDecl {
        name: "outer".to_string(),
        params: vec![],
        ret_type: SimpleType::Void,
        body: vec![
            Stmt::VarDecl {
                name: "a".to_string(),
                typ: SimpleType::Number,
                init: Some(Expr::NumberLit(1.0)),
            },
            Stmt::FuncDecl {
                name: "inner".to_string(),
                params: vec![("b".to_string(), SimpleType::String)],
                ret_type: SimpleType::Void,
                body: vec![Stmt::Return(None)],
            },
        ],
    };
    assert!(checker.check_stmt(&outer).is_ok());

    // Return statement outside function should error
    let stray_return = Stmt::Return(Some(Expr::BoolLit(true)));
    assert!(checker.check_stmt(&stray_return).is_err());

    // Function with missing return expression
    let missing_ret = Stmt::FuncDecl {
        name: "no_ret".to_string(),
        params: vec![],
        ret_type: SimpleType::Number,
        body: vec![Stmt::Return(None)],
    };
    assert!(checker.check_stmt(&missing_ret).is_err());

    // Function with correct void return
    let void_func = Stmt::FuncDecl {
        name: "do_nothing".to_string(),
        params: vec![],
        ret_type: SimpleType::Void,
        body: vec![Stmt::Return(None)],
    };
    assert!(checker.check_stmt(&void_func).is_ok());

    // Identifier usage before declaration
    let use_before = Stmt::VarDecl {
        name: "z".to_string(),
        typ: SimpleType::Number,
        init: Some(Expr::Identifier("undef".to_string())),
    };
    assert!(checker.check_stmt(&use_before).is_err());

    // All tests passed
    println!("All type checker assertions succeeded.");
}