use std::fmt;

#[derive(Debug, Clone, PartialEq)]
pub enum Sexp {
    Atom(String),
    List(Vec<Sexp>),
}

impl fmt::Display for Sexp {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Sexp::Atom(s) => write!(f, "{}", s),
            Sexp::List(v) => {
                write!(f, "(")?;
                for (i, e) in v.iter().enumerate() {
                    if i > 0 {
                        write!(f, " ")?;
                    }
                    write!(f, "{}", e)?;
                }
                write!(f, ")")
            }
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct SexpMatrix {
    pub rows: usize,
    pub cols: usize,
    pub data: Vec<Vec<Sexp>>,

}

impl SexpMatrix {
    pub fn new(rows: usize, cols: usize, default: Sexp) -> Self {
        let row = vec![default.clone(); cols];
        let data = vec![row; rows];
        Self { rows, cols, data }
    }

    pub fn rows(&self) -> usize { self.rows }
    pub fn cols(&self) -> usize { self.cols }

    pub fn get(&self, row: usize, col: usize) -> Option<&Sexp> {
        if row < self.rows && col < self.cols {
            Some(&self.data[row][col])
        } else {
            None
        }
    }

    pub fn set(&mut self, row: usize, col: usize, value: Sexp) {
        if row < self.rows && col < self.cols {
            self.data[row][col] = value;
        }
    }

    pub fn transpose(&self) -> Self {
        let mut transposed = vec![vec![Sexp::Atom(String::new()); self.rows]; self.cols];
        for r in 0..self.rows {
            for c in 0..self.cols {
                transposed[c][r] = self.data[r][c].clone();
            }
        }
        Self { rows: self.cols, cols: self.rows, data: transposed }
    }
}

impl fmt::Display for SexpMatrix {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for r in 0..self.rows {
            write!(f, "[")?;
            for (c, e) in self.data[r].iter().enumerate() {
                if c > 0 {
                    write!(f, ", ")?;
                }
                write!(f, "{}", e)?;
            }
            writeln!(f, "]")?;
        }
        Ok(())
    }
}