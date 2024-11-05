use modulo::Mod;
use rand::{self, Rng};
use std::fmt::Display;
use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Sub, SubAssign};

#[derive(Debug, Clone)]
pub struct FieldElement {
    val: u32,
    p: u32,
    generator: u32,
}

impl FieldElement {
    /// We are using the Finite Field F_{2^30 - 1}
    /// with generator 5.
    pub fn new(val: u32) -> Self {
        let p = 3 * 2u32.pow(30) + 1;
        let generator = 5;
        Self {
            val: val.modulo(p),
            p,
            generator,
        }
    }

    pub fn zero() -> Self {
        Self::new(0)
    }

    pub fn one() -> Self {
        Self::new(1)
    }

    pub fn get_prime() -> u32 {
        3 * 2u32.pow(30) + 1
    }

    /// use Fermat's little theorem
    /// a^p = a (mod p)
    /// a^{p-2} * a = 1 (mod p)
    pub fn inverse(&self) -> Self {
        if self.val == 0 {
            panic!("Cannot compute inverse of zero");
        }
        let exp = self.p - 2;
        self.pow(exp)
    }

    pub fn pow(&self, exp: u32) -> Self {
        let mut base = self.val;
        let mut result = 1u32;
        let mut exponent = exp;

        while exponent > 0 {
            if exponent & 1 == 1 {
                result = (result as u64 * base as u64 % self.p as u64) as u32;
            }
            base = (base as u64 * base as u64 % self.p as u64) as u32;
            exponent >>= 1;
        }
        Self::new(result)
    }

    // pub fn clone(&self) -> Self {
    //     Self::new(self.val)
    // }

    pub fn get_generator(&self) -> u32 {
        self.generator
    }

    // TODO: make it faster
    pub fn is_order(&self, n: u32) -> bool {
        assert!(n >= 1);
        if self.pow(n) != FieldElement::one() {
            return false;
        }

        for i in 2..n {
            if n % i == 0 {
                if self.pow(i) == FieldElement::one() {
                    return false;
                }
            }
        }
        return true;
    }

    pub fn random_element() -> Self {
        let mut rng = rand::thread_rng();
        Self::new(rng.gen_range(0..Self::get_prime()))
    }
}

impl Copy for FieldElement {}

impl PartialEq for FieldElement {
    fn eq(&self, other: &Self) -> bool {
        self.val == other.val // just check the value here
    }
}

impl Eq for FieldElement {}

impl Add for FieldElement {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self::new((self.val + other.val).modulo(self.p))
    }
}

impl Sub for FieldElement {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        if self.val < other.val {
            Self::new((self.p + self.val - other.val).modulo(self.p))
        } else {
            Self::new((self.val - other.val).modulo(self.p))
        }
    }
}

impl Mul for FieldElement {
    type Output = Self;

    fn mul(self, other: Self) -> Self {
        let result = (self.val as u64 * other.val as u64) % self.p as u64;
        Self::new(result as u32)
    }
}

impl Div for FieldElement {
    type Output = Self;

    fn div(self, other: Self) -> Self {
        let other_inv = other.inverse();
        self * other_inv
    }
}

impl AddAssign for FieldElement {
    fn add_assign(&mut self, other: Self) {
        self.val = (self.val + other.val).modulo(self.p);
    }
}

impl SubAssign for FieldElement {
    fn sub_assign(&mut self, other: Self) {
        if self.val < other.val {
            self.val = (self.p + self.val - other.val).modulo(self.p);
        } else {
            self.val = (self.val - other.val).modulo(self.p);
        }
    }
}

impl MulAssign for FieldElement {
    fn mul_assign(&mut self, other: Self) {
        let result = (self.val as u64 * other.val as u64).modulo(self.p as u64);
        self.val = result as u32;
    }
}

impl DivAssign for FieldElement {
    fn div_assign(&mut self, other: Self) {
        let other_inv = other.inverse();
        *self *= other_inv;
    }
}

impl Display for FieldElement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.val)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_operations() {
        let a = FieldElement::new(10);
        let b = FieldElement::new(5);

        assert_eq!(FieldElement::new(15).val, (a.clone() + b.clone()).val);
        assert_eq!(FieldElement::new(5).val, (a.clone() - b.clone()).val);
        assert_eq!(FieldElement::new(50).val, (a.clone() * b.clone()).val);
        assert_eq!(FieldElement::new(2).val, (a.clone() / b.clone()).val);
    }

    #[test]
    fn test_zero_and_one() {
        let a = FieldElement::new(10);
        let zero = FieldElement::zero();
        let one = FieldElement::one();

        assert_eq!(a.val, (a.clone() + zero.clone()).val);
        assert_eq!(a.val, (a.clone() - zero.clone()).val);
        assert_eq!(zero.val, (a.clone() * zero.clone()).val);
        assert_eq!(a.val, (a.clone() * one.clone()).val);
        assert_eq!(a.val, (a.clone() / one.clone()).val);
    }

    #[test]
    fn test_large_numbers() {
        let a = FieldElement::new(u32::MAX - 2);
        let b = FieldElement::new(u32::MAX - 1);

        // These operations should not panic due to overflow
        let _sum = a.clone() + b.clone();
        let _diff = a.clone() - b.clone();
        let _prod = a.clone() * b.clone();
        let _div = a.clone() / b.clone();
    }

    #[test]
    fn test_subtraction_edge_cases() {
        let small = FieldElement::new(5);
        let large = FieldElement::new(10);

        // Test subtraction where result would be negative
        let diff = small.clone() - large.clone();
        assert!(diff.val < small.p);
        assert!(diff.val > 0);
    }

    #[test]
    fn test_pow() {
        let a = FieldElement::new(5);
        let a_pow = a.pow(3);
        assert_eq!(a_pow.val, 125);

        let a_pow2 = a.pow(FieldElement::get_prime() - 2);
        assert_eq!(a_pow2, a.inverse());
    }

    #[test]
    fn test_inverse() {
        let a = FieldElement::new(6);
        let a_inv = a.inverse();

        // a * a^(-1) should equal 1
        assert_eq!((a * a_inv), FieldElement::one());
    }

    #[test]
    #[should_panic]
    fn test_zero_inverse() {
        // Trying to get inverse of zero should panic
        let zero = FieldElement::zero();
        let _inv = zero.inverse();
    }

    #[test]
    fn test_is_order() {
        let a = FieldElement::new(5);
        assert!(a.is_order(FieldElement::get_prime() - 1));
    }

    // TODO: how to test randomness?
    #[test]
    fn test_random_element() {
        for _ in 0..100 {
            let a = FieldElement::random_element();
            assert!(a.val < FieldElement::get_prime());
        }
    }
}
