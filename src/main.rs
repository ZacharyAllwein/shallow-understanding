use std::{
    char,
    collections::HashMap,
    fmt,
    ops::{Add, Mul, Sub},
};

fn main() {
    let term = Term::new(10, HashMap::from([('x', 3), ('y', 4)]));
    let term1 = Term::new(-11, HashMap::from([('x', 3), ('y', 4)]));
    println!("{}", term.sub(term1).unwrap())
}

#[derive(Debug)]
struct Term {
    coefficient: i32,
    variables: HashMap<char, i32>,
}

impl Term {
    fn new(coefficient: i32, variables: HashMap<char, i32>) -> Self {
        Self {
            coefficient,
            variables,
        }
    }
}

impl fmt::Display for Term {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let variables_string = self
            .variables
            .iter()
            .map(|(&sym, &exp)| format!("{}^{}", sym, exp))
            .fold(String::new(), |acc, var| acc + &var);

        write!(f, "{}{}", self.coefficient, variables_string)
    }
}

impl Add for Term {
    type Output = Result<Self, &'static str>;

    fn add(self, other: Self) -> Result<Self, &'static str> {
        for (key, value) in self.variables.iter() {
            match other.variables.get(key) {
                Some(val) if val == value => (),
                _ => return Err("incompatible variables"),
            }
        }

        Ok(Self {
            coefficient: self.coefficient + other.coefficient,
            variables: self.variables,
        })
    }
}

impl Sub for Term {
    type Output = Result<Self, &'static str>;

    fn sub(self, other: Self) -> Result<Self, &'static str> {
        for (key, value) in self.variables.iter() {
            match other.variables.get(key) {
                Some(val) if val == value => (),
                _ => return Err("incompatible variables"),
            }
        }

        Ok(Self {
            coefficient: self.coefficient - other.coefficient,
            variables: self.variables,
        })
    }
}

// impl Mul for Term {
//     fn mul(self, other: Self) -> Self {
//         let coefficient = self.coefficient * other.coefficient;
//     }
// }
