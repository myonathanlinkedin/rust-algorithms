use std::collections::HashMap;
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Clone, Debug)]
pub struct Vector {
    pub data: Vec<f32>,
}

impl Vector {
    pub fn dot(&self, other: &Vector) -> f32 {
        self.data.iter().zip(&other.data).map(|(a, b)| a * b).sum()
    }
    pub fn norm(&self) -> f32 {
        self.data.iter().map(|x| x * x).sum::<f32>().sqrt()
    }
    pub fn cosine_similarity(&self, other: &Vector) -> f32 {
        let denom = self.norm() * other.norm();
        if denom == 0.0 {
            0.0
        } else {
            self.dot(other) / denom
        }
    }
}

#[derive(Clone, Debug)]
pub enum Value {
    Int(i64),
    Float(f64),
    Str(String),
    Vec(Vector),
}

#[derive(Clone, Debug)]
pub struct Column {
    pub name: String,
    pub col_type: ColumnType,
}

#[derive(Clone, Debug)]
pub enum ColumnType {
    Int,
    Float,
    Str,
    Vec,
}

#[derive(Clone, Debug)]
pub struct Row {
    pub id: u64,
    pub columns: Vec<Value>,
}

#[derive(Clone, Debug)]
pub struct Table {
    pub name: String,
    pub schema: Vec<Column>,
    pub rows: Vec<Row>,
}

impl Table {
    pub fn new(name: &str, schema: Vec<Column>) -> Self {
        Table {
            name: name.to_string(),
            schema,
            rows: Vec::new(),
        }
    }

    pub fn insert(&mut self, values: Vec<Value>) -> Result<u64, &'static str> {
        if values.len() != self.schema.len() {
            return Err("Column count mismatch");
        }
        let id = self.rows.len() as u64 + 1;
        self.rows.push(Row { id, columns: values });
        Ok(id)
    }

    pub fn vector_search(&self, col_name: &str, query: &Vector, k: usize) -> Vec<(u64, f32)> {
        let col_index = self
            .schema
            .iter()
            .position(|c| c.name == col_name && matches!(c.col_type, ColumnType::Vec));
        if col_index.is_none() {
            return Vec::new();
        }
        let idx = col_index.unwrap();
        let mut sims: Vec<(u64, f32)> = self
            .rows
            .iter()
            .filter_map(|row| {
                if let Value::Vec(ref vec) = row.columns[idx] {
                    Some((row.id, vec.cosine_similarity(query)))
                } else {
                    None
                }
            })
            .collect();
        sims.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Equal));
        sims.truncate(k);
        sims
    }
}

#[derive(Clone, Debug)]
pub struct Commit {
    pub id: u64,
    pub timestamp: u64,
    pub message: String,
    pub table_snapshots: HashMap<String, Vec<Row>>,
}

impl Commit {
    fn new(id: u64, message: String, tables: &HashMap<String, Table>) -> Self {
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();
        let mut snapshots = HashMap::new();
        for (name, table) in tables {
            snapshots.insert(name.clone(), table.rows.clone());
        }
        Commit {
            id,
            timestamp,
            message,
            table_snapshots: snapshots,
        }
    }
}

#[derive(Clone, Debug)]
pub struct Database {
    pub tables: HashMap<String, Table>,
    pub commits: Vec<Commit>,
    pub next_commit_id: u64,
}

impl Database {
    pub fn new() -> Self {
        Database {
            tables: HashMap::new(),
            commits: Vec::new(),
            next_commit_id: 1,
        }
    }

    pub fn create_table(&mut self, name: &str, schema: Vec<Column>) -> Result<(), &'static str> {
        if self.tables.contains_key(name) {
            return Err("Table already exists");
        }
        self.tables.insert(name.to_string(), Table::new(name, schema));
        Ok(())
    }

    pub fn insert(&mut self, table_name: &str, values: Vec<Value>) -> Result<u64, &'static str> {
        match self.tables.get_mut(table_name) {
            Some(table) => table.insert(values),
            None => Err("Table not found"),
        }
    }

    pub fn commit(&mut self, message: &str) {
        let commit = Commit::new(self.next_commit_id, message.to_string(), &self.tables);
        self.commits.push(commit);
        self.next_commit_id += 1;
    }

    pub fn get_table(&self, name: &str) -> Option<&Table> {
        self.tables.get(name)
    }

    pub fn vector_search(
        &self,
        table_name: &str,
        col_name: &str,
        query: &Vector,
        k: usize,
    ) -> Vec<(u64, f32)> {
        match self.tables.get(table_name) {
            Some(table) => table.vector_search(col_name, query, k),
            None => Vec::new(),
        }
    }
}
