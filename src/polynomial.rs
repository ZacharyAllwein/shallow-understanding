use crate::term::Term;
use std::fmt;

pub enum PolyEl {
    Term(Term),
    Add,
    Sub,
    Mul,
    Div
}

pub struct Polynomial(pub Vec<PolyEl>);

impl fmt::Display for Polynomial{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    
        let poly_string = self.0.iter().map(|poly_el| {
            match poly_el {
                PolyEl::Term(term) => format!("{}", term),
                PolyEl::Add => String::from(" + "),
                PolyEl::Sub => String::from(" - "),
                PolyEl::Mul => String::from(" * "),
                PolyEl::Div => String::from(" / "),
            }
        }).collect::<String>();

        write!(f, "{}", poly_string)
    }
}


