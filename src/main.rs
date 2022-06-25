mod polynomial;
mod term;

use polynomial::Polynomial;
use std::collections::HashMap;
use term::Term;

fn main() {
    let three = Term::new(3f32, HashMap::from([('x', 0)]));
    let term = Term::new(1f32, HashMap::from([('x', 3)]));

    let poly = Polynomial(vec![three, term]);

    println!("{}", (poly.clone() / poly.clone()));
}
