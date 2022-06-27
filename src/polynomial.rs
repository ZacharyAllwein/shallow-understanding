use crate::term::Term;
use std::{
    collections::HashMap,
    fmt,
    ops::{Add, Div, Mul, Sub},
};

#[derive(Clone, Debug)]
pub struct Polynomial(pub Vec<Term>);

impl Polynomial {
    fn simplify(self) -> Polynomial {
        
    }
}

impl fmt::Display for Polynomial {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let poly_string = self
            .0
            .iter()
            .map(|term| format!("+ {} ", term))
            .collect::<String>();

        write!(f, "{}", poly_string)
    }
}

impl Add for Polynomial {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        let mut new_poly = self.0.clone();
        new_poly.append(&mut other.0.clone());

        Polynomial(new_poly).simplify()
    }
}

impl Mul for Polynomial {
    type Output = Self;

    fn mul(self, other: Self) -> Self {
        let mut new_polynomial = vec![];

        for self_term in self.0.iter() {
            for other_term in other.0.iter() {
                new_polynomial.push(self_term.clone() * other_term.clone());
            }
        }
        Polynomial(new_polynomial).simplify()
    }
}

impl Sub for Polynomial {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        let other_neg = other * Polynomial(vec![Term::new(-1f32, HashMap::from([('x', 0)]))]);

        (self + other_neg).simplify()
    }
}

impl Div for Polynomial {
    type Output = Self;

    fn div(self, other: Self) -> Self {
        let mut other_reciprocals = vec![];

        for term in other.0.iter() {
            other_reciprocals.push(Term::num(1f32) / (term.clone()))
        }

        println!("{:?}", other_reciprocals);
        self * Polynomial(other_reciprocals)
    }
}
