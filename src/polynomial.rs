use crate::term::Term;
use std::fmt;
use std::ops::{Add, Sub, Mul, Div, Deref};


#[derive(Clone, Debug)]
pub struct Polynomial(pub Vec<Term>);

impl Polynomial{
    pub fn simplify(self) -> Polynomial{
        
        for (i, term1) in self.0.iter().enumerate(){
            for (j, term2) in self.0.iter().enumerate(){
                if i == j {continue}

                
                match term1.clone() + term2.clone() {
                    Ok(new_term) => {

                        let mut new_terms: Vec<Term> = self
                            .0
                            .iter()
                            .enumerate()
                            .filter(|(k, _)| k != &i && k!= &j)
                            .map(|(_, term)| term.clone())
                            .collect();
                        
                        new_terms.push(new_term);

                        return Polynomial(new_terms).simplify();
                        
                    },
                    _ => continue,
                }
            }
        }
        return self;
    }
}

impl fmt::Display for Polynomial{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    
        let poly_string = self
            .0
            .iter()
            .map(|term| format!("+ {} ", term))
            .collect::<String>();

        write!(f, "{}", poly_string)
    }
}

impl Add for Polynomial{
    type Output = Self;

    fn add(self, other: Self) -> Self{

        let mut new_poly = self.0.clone();
        new_poly.append(&mut other.0.clone());

        Polynomial(new_poly)
    }
}

impl Mul for Polynomial {
    type Output = Self;

    fn mul(self, other: Self) -> Self{

        let mut new_polynomial = vec![];

        for self_term in self.0.iter(){


            for other_term in other.0.iter(){

                new_polynomial.push(self_term.clone() * other_term.clone());
            
            }
        }
        Polynomial(new_polynomial)
    }
}


