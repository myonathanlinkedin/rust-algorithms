pub mod types {
    #[derive(Debug, Clone, PartialEq)]
    pub enum SimpleType {
        Number,
        String,
        Bool,
        Void,
        Unknown,
    }

    #[derive(Debug, Clone, PartialEq)]
    pub enum Expr {
        NumberLit(f64),
        StringLit(String),
        BoolLit(bool),
        Identifier(String),
    }

    #[derive(Debug, Clone, PartialEq)]
    pub enum Stmt {
        VarDecl {
            name: String,
            typ: SimpleType,
            init: Option<Expr>,
        },
        FuncDecl {
            name: String,
            params: Vec<(String, SimpleType)>,
            ret_type: SimpleType,
            body: Vec<Stmt>,
        },
        Return(Option<Expr>),
    }

    use std::collections::HashMap;

    #[derive(Debug, Clone, PartialEq)]
    pub struct TypeEnv {
        pub scopes: Vec<HashMap<String, SimpleType>>,

    }

    impl TypeEnv {
        pub fn new() -> Self {
            Self {
                scopes: vec![HashMap::new()],
            }
        }

        pub fn push_scope(&mut self) {
            self.scopes.push(HashMap::new());
        }

        pub fn pop_scope(&mut self) {
            self.scopes.pop();
        }

        pub fn insert(&mut self, name: String, typ: SimpleType) -> Result<(), String> {
            let current = self.scopes.last_mut().unwrap();
            if current.contains_key(&name) {
                Err(format!("Duplicate symbol {}", name))
            } else {
                current.insert(name, typ);
                Ok(())
            }
        }

        pub fn lookup(&self, name: &str) -> Option<SimpleType> {
            for scope in self.scopes.iter().rev() {
                if let Some(t) = scope.get(name) {
                    return Some(t.clone());
                }
            }
            None
        }
    }
}