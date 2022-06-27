// mod polynomial;
mod term;
mod fraction;

// use polynomial::Polynomial;
use term::Term;
use fraction::Fraction;

fn main() {
    // let three = Term::new(3f32, vec![('x', 1), ('y', 3), ('a', 5)]);
    // let term = Term::new(4f32, vec![('x', -1), ('y', -3), ('m', 6)]);
    let f1 = Fraction::new(3, 5);
    let f2 = Fraction::new(1, 4);

    println!("{}", Fraction::new(-1000, -55).simplify());
}
