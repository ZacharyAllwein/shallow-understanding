// mod polynomial;
mod term;

// use polynomial::Polynomial;
use std::collections::HashMap;
use term::Term;

fn main() {
    let three = Term::new(3f32, vec![('x', 1), ('y', 3)]);
    let term = Term::new(4f32, vec![('x', -1), ('y', -3)]);

    println!("{}", three.div(&term));
}
