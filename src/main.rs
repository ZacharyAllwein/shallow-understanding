mod polynomial;
mod term;
mod fraction;

use polynomial::Polynomial;
use term::Term;
use fraction::Fraction;

fn main() {

    let x = 5;

    let f1 = Fraction::new(3, 5);
    let f2 = Fraction::new(1, 1);
    // let three = Term::new(f1, vec![('x', 1), ('y', 3), ('a', 5)]);
    let term = Term::new(f2, vec![('x', 1)]);
    let poly = Polynomial::new(vec![Term::num(3), term], 1);

    println!("{}", poly.expand());

}
