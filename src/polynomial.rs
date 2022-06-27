use crate::term::Term;
use std::{
    collections::HashMap,
    fmt,
    ops::{Add, Div, Mul, Sub},
};

//ok I think this idea for polynomial represenation might work and is kind of interesting
//It is composed of two things, terms and a degree. Ex. (x + 3)^1, in this example, x + 3 are the terms and 1 is the degree.
//I think this lets me do a couple of interesting things for example representing division as multiplication by terms to a negative degree
//Ex. to represent x^2 + 6x + 9 / x + 3 I could write (x + 3)^2 * (x + 3)^-1, and also helps lead the way into factoring
//lets see how it goes
#[derive(Clone, Debug)]
pub struct Polynomial{
    terms: Vec<Term>,
    degree: i32,
}

impl Polynomial {

    pub fn new(terms: Vec<Term>, degree: i32) -> Self{
        Polynomial { terms, degree }
    }

    //takes a polynomial and expands it using its degree
    pub fn expand(self) -> Self{
        if self.degree < 2{
            return self
        }
        
        let mut cur_terms = self.terms.clone();

        for _ in 0..self.degree-1{
            cur_terms = Polynomial::mul_once(&cur_terms, &self.terms);
        }

        Polynomial::new(cur_terms, 1)

        
    }

    //multiplies two sets of terms
    fn mul_once(terms: &Vec<Term>, other_terms: &Vec<Term>) -> Vec<Term>{
                
        let mut new_terms = vec![];

        for term in terms {
            for other_term in other_terms {
                new_terms.push(term.mul(other_term));
            }
        }

        new_terms
    }
}

impl fmt::Display for Polynomial {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let poly_string = self
            .terms
            .iter()
            .map(|term| format!("+ {} ", term))
            .collect::<String>();
        

        write!(f, "({})^{}", poly_string, self.degree)
    }
}

// impl Add for Polynomial {
//     type Output = Self;

//     fn add(self, other: Self) -> Self {
//         let mut new_poly = self.0.clone();
//         new_poly.append(&mut other.0.clone());

//         Polynomial(new_poly).simplify()
//     }
// }

// impl Mul for Polynomial {
//     type Output = Self;

//     fn mul(self, other: Self) -> Self {
//         let mut new_polynomial = vec![];

//         for self_term in self.0.iter() {
//             for other_term in other.0.iter() {
//                 new_polynomial.push(self_term.clone() * other_term.clone());
//             }
//         }
//         Polynomial(new_polynomial).simplify()
//     }
// }

// impl Sub for Polynomial {
//     type Output = Self;

//     fn sub(self, other: Self) -> Self {
//         let other_neg = other * Polynomial(vec![Term::new(-1f32, HashMap::from([('x', 0)]))]);

//         (self + other_neg).simplify()
//     }
// }

// impl Div for Polynomial {
//     type Output = Self;

//     fn div(self, other: Self) -> Self {
//         let mut other_reciprocals = vec![];

//         for term in other.0.iter() {
//             other_reciprocals.push(Term::num(1f32) / (term.clone()))
//         }

//         println!("{:?}", other_reciprocals);
//         self * Polynomial(other_reciprocals)
//     }
// }
