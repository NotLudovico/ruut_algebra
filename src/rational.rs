use std::ops::{Add, Mul};

use crate::gcd;

pub struct Rat {
    pub num: i32,
    pub den: i32,
}

impl Add for Rat {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        let num = self.num * rhs.den + self.den * rhs.num;
        let den = self.den * rhs.den;
        let gcd = gcd(num.unsigned_abs(), den.unsigned_abs()) as i32;
        Rat {
            num: num / gcd,
            den: den / gcd,
        }
    }
}
impl Mul for Rat {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        let num = self.num * rhs.num;
        let den = self.den * rhs.den;
        let gcd = gcd(num.unsigned_abs(), den.unsigned_abs()) as i32;
        Rat {
            num: num / gcd,
            den: den / gcd,
        }
    }
}
