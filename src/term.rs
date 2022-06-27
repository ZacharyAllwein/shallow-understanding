use std::{char, fmt, collections::HashMap};

#[derive(Debug, Clone, PartialEq)]
pub struct Term {
    pub coefficient: f32,
    //why not use a hashmap you might ask, first fuck you I tried, second not sortable or hashable however
    //now that I think about it I probably could have gotten away with just turning a hashmap into a vec whenever I needed it
    //hmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmm whatever this is what version control is for
    //I still think this probably works better because the sortable thing is a real problem, but I'm not so sure anymore
    pub variables: Vec<(char, i32)>,
}

impl Term {
    //base constructor
    pub fn new(coefficient: f32, mut variables: Vec<(char, i32)>) -> Self {

        //just calling attention to this because it is everywhere in the code just sorts the variables alphebetically to make things work well
        variables.sort_by(|a, b| a.0.cmp(&b.0));

        Self {
            coefficient,
            variables,
        }
    }

    //allows the creation of numbers defined as a term ex 5x^0 represents 5
    pub fn num(num: f32) -> Self {
        Self {
            coefficient: num,
            variables: vec![('x', 0)],
        }
    }

    //adds two terms, may fail if incompatible variables
    pub fn add(&self, other: &Self) -> Result<Self, &'static str> {

        for (key, value) in self.variables.iter() {
            //sees if each self variable is in other and makes sure there exponents are matching for addition
            match other.var_get(*key) {
                Ok(val) if val == *value => (),
                _ => return Err("incompatible variables"),
            }
        }

        //return a new instance of Term that adds the two terms cloning self variables
        Ok(Self {
            coefficient: self.coefficient + other.coefficient,
            variables: self.variables.clone(),
        })
    }

    pub fn sub(&self, other: &Self) -> Result<Self, &'static str> {
        //I think copying the add function is the most efficient, but I could use a combination of add and mul -1
        
        for (key, value) in self.variables.iter() {

            match other.var_get(*key) {
                Ok(val) if val == *value => (),
                _ => return Err("incompatible variables"),
            }
        }

        Ok(Self {
            coefficient: self.coefficient + other.coefficient,
            variables: self.variables.clone(),
        })
    }

    pub fn mul(&self, other: &Self) -> Self {
        let mut new_term = Term::new(self.coefficient * other.coefficient, self.variables.clone());

        //kind of weird function, instead of maintaining a list I create a new instance of a term that is a copy of self
        //this gives me access to the var_get functionality. Using this I'm able to iterate through the variables in other,
        //changing the variables in new_term where they already exist and adding new ones as needed
        //It allows me to maintain an immutable external api while using mutability internally which I think is kind of cool.
        for &(sym, exp) in other.variables.iter() {
            match new_term.var_get_mut(sym) {
                Ok(exp_ref) => *exp_ref += exp,
                Err(_) => new_term.variables.push((sym, exp)),
            }
        }

        //clean up variables with 0 exponents so we don't end up with terms like 6z^0y^0a^0x^0 when 6x^0 will suffice
        new_term.variables = new_term
            .variables
            .into_iter()
            .filter(|(_sym, exp)| exp != &0)
            .collect();
        
        new_term.variables.sort_by(|a, b| a.0.cmp(&b.0));

        if new_term.variables.len() == 0 {
            new_term.variables.push(('x', 0));
        }

        new_term
    }

    pub fn div(&self, other: &Self) -> Self {
        //same thing with subtraction I think it makes more sense to just copy paste mul logic instead of
        //writing some odd reciprocal multiplication wrapper, check history if you want to see that though
        //see mul comments
        let mut new_term = Term::new(self.coefficient / other.coefficient, self.variables.clone());

        for &(sym, exp) in other.variables.iter() {
            match new_term.var_get_mut(sym) {
                Ok(exp_ref) => *exp_ref -= exp,
                Err(_) => new_term.variables.push((sym, exp)),
            }
        }

        new_term.variables = new_term
            .variables
            .into_iter()
            .filter(|(_sym, exp)| exp != &0)
            .collect();
        
        new_term.variables.sort_by(|a, b| a.0.cmp(&b.0));

        if new_term.variables.len() == 0 {
            new_term.variables.push(('x', 0));
        }

        new_term
    }

    //private helper function that acts as a 'get' for a hashmap
    fn var_get(&self, sym: char) -> Result<i32, &'static str> {
        for &(sym2, exp) in self.variables.iter() {
            if sym2 == sym {
                return Ok(exp);
            }
        }

        return Err("key not found");
    }

    //private helper function that acts as a 'get_mut' for a hashmap
    fn var_get_mut(&mut self, sym: char) -> Result<&mut i32, &'static str> {
        for (sym2, exp) in self.variables.iter_mut() {
            if *sym2 == sym {
                return Ok(exp);
            }
        }

        return Err("key not found");
    }
}

impl fmt::Display for Term {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let variables_string = self
            .variables
            .iter()
            .map(|(sym, exp)| format!("{}^{}", sym, exp))
            .fold(String::new(), |acc, var| acc + &var);

        write!(f, "{}{}", self.coefficient, variables_string)
    }
}
