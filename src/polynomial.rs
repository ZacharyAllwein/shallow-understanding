use crate::term::Term;
use std::fmt;

pub enum PolyOp {
    Add,
    Sub,
    Mul,
    Div
}

pub enum PolyEl {
    Term(Term),
    Operator(PolyOp),
}

pub struct Polynomial(pub Vec<PolyEl>);

impl fmt::Display for Polynomial{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    
        let poly_string = self.0.iter().map(|poly_el| {
            match poly_el {
                PolyEl::Term(term) => format!("{}", term),
                PolyEl::Operator(op) => {
                    String::new()
                }
            }
        }).collect::<String>();

        write!(f, "{}", poly_string)
    }
}


