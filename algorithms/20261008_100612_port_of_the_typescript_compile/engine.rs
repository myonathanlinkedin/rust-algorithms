pub mod engine {
    use crate::types::types::{Expr, SimpleType, Stmt, TypeEnv};

    #[derive(Debug, Clone, PartialEq)]
    pub struct TypeChecker {
        pub env: TypeEnv,
        pub return_type_stack: Vec<SimpleType>,

    }

    impl TypeChecker {
        pub fn new() -> Self {
            Self {
                env: TypeEnv::new(),
                return_type_stack: Vec::new(),
            }
        }

        pub fn check_stmt(&mut self, stmt: &Stmt) -> Result<(), String> {
            match stmt {
                Stmt::VarDecl { name, typ, init } => {
                    if let Some(expr) = init {
                        let expr_type = self.check_expr(expr)?;
                        if &expr_type != typ {
                            return Err(format!(
                                "Type mismatch in var {}: expected {:?}, got {:?}",
                                name, typ, expr_type
                            ));
                        }
                    }
                    self.env.insert(name.clone(), typ.clone())
                }
                Stmt::FuncDecl {
                    name,
                    params,
                    ret_type,
                    body,
                } => {
                    // Register function name in outer scope
                    self.env.insert(name.clone(), ret_type.clone())?;
                    // New scope for function body
                    self.env.push_scope();
                    for (pname, ptype) in params {
                        self.env.insert(pname.clone(), ptype.clone())?;
                    }
                    self.return_type_stack.push(ret_type.clone());
                    for s in body {
                        self.check_stmt(s)?;
                    }
                    self.return_type_stack.pop();
                    self.env.pop_scope();
                    Ok(())
                }
                Stmt::Return(expr_opt) => {
                    let expected = self
                        .return_type_stack
                        .last()
                        .ok_or_else(|| "Return outside function".to_string())?
                        .clone();
                    match expr_opt {
                        Some(expr) => {
                            let expr_type = self.check_expr(expr)?;
                            if expr_type != expected {
                                Err(format!(
                                    "Return type mismatch: expected {:?}, got {:?}",
                                    expected, expr_type
                                ))
                            } else {
                                Ok(())
                            }
                        }
                        None => {
                            if expected != SimpleType::Void {
                                Err(format!(
                                    "Missing return expression, expected {:?}",
                                    expected
                                ))
                            } else {
                                Ok(())
                            }
                        }
                    }
                }
            }
        }

        pub fn check_expr(&self, expr: &Expr) -> Result<SimpleType, String> {
            match expr {
                Expr::NumberLit(_) => Ok(SimpleType::Number),
                Expr::StringLit(_) => Ok(SimpleType::String),
                Expr::BoolLit(_) => Ok(SimpleType::Bool),
                Expr::Identifier(name) => self
                    .env
                    .lookup(name)
                    .ok_or_else(|| format!("Undeclared identifier {}", name)),
            }
        }
    }
}