use crate::term::Term;
use std::fmt;
use std::ops::{Add, Sub, Mul, Div};

#[derive(Clone)]
pub enum PolyEl {
    Term(Term),
    Add,
}

#[derive(Clone)]
pub struct Polynomial(pub Vec<PolyEl>);

impl fmt::Display for Polynomial{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    
        let poly_string = self.0.iter().map(|poly_el| {
            match poly_el {
                PolyEl::Term(term) => format!("{}", term),
                PolyEl::Add => String::from(" + "),
            }
        }).collect::<String>();

        write!(f, "{}", poly_string)
    }
}

impl Add for Polynomial{
    type Output = Self;

    fn add(self, other: Self) -> Self{

        let mut new_poly = self.0.clone();
        new_poly.push(PolyEl::Add);
        new_poly.append(&mut other.0.clone());

        Polynomial(new_poly)
    }
}


