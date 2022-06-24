use std::{
    char,
    collections::HashMap,
    fmt,
    ops::{Add, Mul, Sub, Div},
};


#[derive(Debug, Clone, PartialEq)]
pub struct Term {
    pub coefficient: f32,
    pub variables: HashMap<char, i32>,
}

impl Term {
    pub fn new(coefficient: f32, variables: HashMap<char, i32>) -> Self {
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

impl Mul for Term {
    type Output = Self;

    fn mul(self, other: Self) -> Self{
        
        let new_coefficient: f32 = self.coefficient * other.coefficient;

        let mut new_variables: HashMap<char, i32> = self.variables.clone();


        for (other_sym, other_exp) in other.variables.iter(){

            match new_variables.get_mut(other_sym){
                Some(self_exp) => *self_exp += *other_exp,
                None => {
                    new_variables.insert(*other_sym, *other_exp);
                }
            }
        }

        return Self {coefficient: new_coefficient, variables: new_variables};

        
    }
}

impl Sub for Term {
    type Output = Result<Self, &'static str>;

    fn sub(self, other: Self) -> Result<Self, &'static str> {

        let other = Self::new(-1f32 * other.coefficient, other.variables);

        self + other
    }
}

impl Div for Term {
    type Output = Self;

    fn div(self, other: Self) -> Self{

        let other_new_variables = other
            .variables
            .iter()
            .map(|(&sym, &exp)| (sym, exp * -2))
            .collect();
        
        let other_new_coefficient = 1f32 / (other.coefficient.powi(2));

        let other_reciprocal = other * Term::new(other_new_coefficient, other_new_variables);

        self * other_reciprocal
    }
}