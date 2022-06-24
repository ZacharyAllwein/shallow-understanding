mod term;
mod polynomial;

use term::Term;
use std::{collections::HashMap};
use polynomial::{Polynomial};

fn main() {
    let term = Term::new(1f32, HashMap::from([('x', 1)]));
    let term2 = Term::new(5f32, HashMap::from([('x', 0)]));
    let term3 = Term::new(-1f32, HashMap::from([('x', 0)]));
    let poly = Polynomial(vec![term, term2]);
    let poly2 = Polynomial(vec![term3]);
    let poly3 = poly.clone();
    println!("{}", (poly * poly3 * poly2).simplify());
}

