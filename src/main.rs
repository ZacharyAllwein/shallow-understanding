mod term;
mod polynomial;

use term::Term;
use std::{collections::HashMap, boxed::Box, ops::Add};
use polynomial::{Polynomial, PolyEl, PolyOp};

fn main() {
    let term = Term::new(10f32, HashMap::from([('x', 1), ('y', 4)]));
    let term1 = Term::new(10f32, HashMap::from([('x', 3), ('y', 4)]));
    let term2 = Term::new(10f32, HashMap::from([('x', 1), ('y', 4)]));

    let poly: Polynomial = Polynomial(vec![PolyEl::Term(term), PolyEl::Operator(PolyOp::Add), PolyEl::Term(term1), PolyEl::Operator(PolyOp::Add), PolyEl::Term(term2)]);
}

