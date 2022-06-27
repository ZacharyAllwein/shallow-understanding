use std::{ops::{Add, Sub, Div, Mul}, fmt};

//very simple fraction implementation because I'm tired and telling a computer how to do polynomial division is hard >:(
#[derive(Debug, Clone, PartialEq)]
pub struct Fraction{
    n: i32,
    d: i32,
}

impl Fraction{
    pub fn new(n: i32, d: i32) -> Self{
        Fraction{n, d}
    }

    pub fn simplify(self) -> Self{

        let mut n = self.n;
        let mut d = self.d;

        //if both the numerator and denominator are negative fix that
        if self.n < 0 && self.d < 0{
            n *= -1;
            d *= -1;
        }
        let n_factors = get_factors(n.abs() as u32);
        let d_factors = get_factors(d.abs() as u32);

        //filters one of the lists doesn't matter which one based on the other and takes the largest element from it
        let gcf = n_factors
            .into_iter()
            .filter(|f| d_factors.contains(f))
            .next();
        
        //if there is something that matches returns a new fraction that has been divided by the gcf
        match gcf {
            Some(g) => {
                return Fraction::new(n / g as i32, d / g as i32)
            }
            None => return self,
        }
    }
}

impl Add for Fraction{
    type Output = Self;

    fn add(self, other: Self) -> Self{
        Fraction::new(self.n * other.d + other.n * self.d, self.d * other.d).simplify()
    }


}

impl Sub for Fraction{
    type Output = Self;

    fn sub(self, other: Self) -> Self{
        Fraction::new(self.n * other.d - other.n * self.d, self.d * other.d).simplify()
    }
}

impl Mul for Fraction {
    type Output = Self;

    fn mul(self, other: Self) -> Self{
        Fraction::new(self.n * other.n, self.d * other.d).simplify()
    }
}

impl Div for Fraction {
    type Output = Self;

    fn div(self, other: Self) -> Self{
        Fraction::new(self.n * other.d, self.d * other.n).simplify()
    }
}

impl fmt::Display for Fraction{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}/{})", self.n, self.d)
    }
}

//all factors for a positive number
pub fn get_factors(num: u32) -> Vec<u32> {
    
    let mut factors = vec![];

    for i in 1..num{
        
        if factors.contains(&i){
            break;
        }
        if num % i == 0{
            factors.push(i);
            factors.push(num / i);
        }
    }
    factors.sort_by(|a, b| b.cmp(a));
    factors
}