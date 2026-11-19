mod core;
use core::{Sexp, SexpMatrix};

fn main() {
    let mut mat = SexpMatrix::new(2, 3, Sexp::Atom("0".to_string()));
    mat.set(0, 0, Sexp::Atom("a".to_string()));
    mat.set(1, 2, Sexp::List(vec![Sexp::Atom("b".to_string()), Sexp::Atom("c".to_string())]));
    assert_eq!(mat.get(0, 0), Some(&Sexp::Atom("a".to_string())));
    assert_eq!(mat.get(1, 2), Some(&Sexp::List(vec![Sexp::Atom("b".to_string()), Sexp::Atom("c".to_string())])));
    let trans = mat.transpose();
    assert_eq!(trans.rows(), 3);
    assert_eq!(trans.cols(), 2);
    assert_eq!(trans.get(0, 0), Some(&Sexp::Atom("a".to_string())));
    assert_eq!(trans.get(2, 1), Some(&Sexp::List(vec![Sexp::Atom("b".to_string()), Sexp::Atom("c".to_string())])));
    println!("{}", mat);
    println!("{}", trans);
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_empty_matrix() {
        let mat = SexpMatrix::new(0, 0, Sexp::Atom("x".to_string()));
        assert_eq!(mat.rows(), 0);
        assert_eq!(mat.cols(), 0);
        assert_eq!(mat.get(0, 0), None);
    }

    #[test]
    fn test_single_element() {
        let mut mat = SexpMatrix::new(1, 1, Sexp::Atom("0".to_string()));
        mat.set(0, 0, Sexp::Atom("single".to_string()));
        assert_eq!(mat.get(0, 0), Some(&Sexp::Atom("single".to_string())));
    }

    #[test]
    fn test_nested_lists() {
        let nested = Sexp::List(vec![
            Sexp::Atom("x".to_string()),
            Sexp::List(vec![Sexp::Atom("y".to_string()), Sexp::Atom("z".to_string())]),
        ]);
        let mut mat = SexpMatrix::new(1, 1, Sexp::Atom("0".to_string()));
        mat.set(0, 0, nested.clone());
        assert_eq!(mat.get(0, 0), Some(&nested));
    }

    #[test]
    fn test_transpose() {
        let mut mat = SexpMatrix::new(2, 3, Sexp::Atom("0".to_string()));
        mat.set(0, 0, Sexp::Atom("a".to_string()));
        mat.set(1, 2, Sexp::Atom("b".to_string()));
        let trans = mat.transpose();
        assert_eq!(trans.rows(), 3);
        assert_eq!(trans.cols(), 2);
        assert_eq!(trans.get(0, 0), Some(&Sexp::Atom("a".to_string())));
        assert_eq!(trans.get(2, 1), Some(&Sexp::Atom("b".to_string())));
    }

    #[test]
    fn test_display() {
        let mut mat = SexpMatrix::new(1, 2, Sexp::Atom("0".to_string()));
        mat.set(0, 0, Sexp::Atom("a".to_string()));
        mat.set(0, 1, Sexp::List(vec![Sexp::Atom("b".to_string()), Sexp::Atom("c".to_string())]));
        let s = format!("{}", mat);
        assert!(s.contains("a"));
        assert!(s.contains("(b c)"));
    }
}