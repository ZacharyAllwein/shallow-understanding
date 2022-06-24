mod term;
mod polynomial;

use term::Term;
use std::{collections::HashMap};
use polynomial::{Polynomial, PolyEl};

fn main() {
    let term = Term::new(6f32, HashMap::from([('x', 3), ('z', 8)]));
    let term1 = Term::new(2f32, HashMap::from([('x', 3), ('z', 8)]));
    // let term2 = Term::new(10f32, HashMap::from([('x', 1), ('y', 4)]));

    // let poly: Polynomial = Polynomial(vec![PolyEl::Term(term), PolyEl::Add, PolyEl::Term(term1), PolyEl::Add, PolyEl::Term(term2)]);
    // let poly2 = poly.clone();
    println!("{}", (term - term1).unwrap());
}

