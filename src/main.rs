mod term;

use term::Term;
use std::collections::HashMap;

fn main() {
    let term = Term::new(10f32, HashMap::from([('x', 3), ('y', 4)]));

    println!("{}", term.clone() * term)
}

