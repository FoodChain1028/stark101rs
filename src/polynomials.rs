use crate::field::FieldElement;
use crate::utils::{remove_trailing_elements, zip_with};
use std::fmt;
use std::ops::{Add, Div, Mul, Neg, Sub};

/// This represents a polynomial over FieldElements.
/// coeffs: the coefficients of the polynomial, represented in least-significant-term
/// var: the variable of the polynomial
/// i.e. `1 + x + 2*x^4` has coeffs = [1, 1, 0, 0, 2] and var = 'x'
#[derive(Debug, Clone)]
pub struct Polynomial {
    coeffs: Vec<FieldElement>,
    var: String,
}

impl Polynomial {
    pub fn new(coeffs: Vec<FieldElement>) -> Self {
        let coeffs = remove_trailing_elements(coeffs, FieldElement::zero());
        let var = "X".to_string();
        Self { coeffs, var }
    }

    pub fn neg(&self) -> Self {
        Self::new(vec![]) - self.clone()
    }

    pub fn qdiv(&self, other: &Self) -> (Self, Self) {
        todo!()
    }

    /// Composes this polynomial with `other`.
    /// f =  X^2 + X
    /// g = X + 1
    /// f.compose(g) = f(g(x)) = (x + 1)^2 + (x + 1) = 2 + 3*x + x^2
    pub fn compose(&self, other: &Self) -> Self {
        let empty_input = vec![];
        let mut result = Polynomial::new(empty_input);
        for coeff in self.coeffs.iter() {
            result = (result * other.clone()) + Polynomial::new(vec![*coeff]);
        }
        result
    }

    pub fn degree(&self) -> usize {
        remove_trailing_elements(self.coeffs.clone(), FieldElement::zero()).len() - 1
    }
}

impl fmt::Display for Polynomial {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.coeffs.is_empty() {
            return write!(f, "0");
        }

        let mut first = true;
        for (power, coeff) in self.coeffs.iter().rev().enumerate() {
            if *coeff == FieldElement::zero() {
                continue;
            }

            if !first && *coeff > FieldElement::zero() {
                write!(f, " + ")?;
            }
            first = false;

            match power {
                // constant
                0 => write!(f, "{}", coeff)?,
                // a_i*X
                1 => {
                    if *coeff == FieldElement::one() {
                        write!(f, "{}", self.var)?;
                    } else {
                        write!(f, "{}*{}", coeff, self.var)?;
                    }
                }
                // a_i*X^i
                _ => {
                    if *coeff == FieldElement::one() {
                        write!(f, "{}^{}", self.var, power)?
                    } else {
                        write!(f, "{}*{}^{}", coeff, self.var, power)?
                    }
                }
            }
        }
        Ok(())
    }
}

impl PartialEq for Polynomial {
    fn eq(&self, other: &Self) -> bool {
        self.coeffs == other.coeffs
    }
}

// impl Eq for Polynomial {}

/// Basic operations for polynomials
impl Add for Polynomial {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        // adding 2 polynomials means adding their coefficients
        // i.e. [1,2,3] + [4, 2] = [5, 4, 3]
        Self::new(zip_with(self.coeffs, other.coeffs, |a, b| a + b))
    }
}

/// Supporting Adding an integer with a Polynomial
impl Add<u32> for Polynomial {
    type Output = Self;

    fn add(self, other: u32) -> Self {
        // converting the usize to FieldElement
        let other = FieldElement::new(other);
        self + Polynomial::new(vec![other])
    }
}

impl Sub for Polynomial {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Self::new(zip_with(self.coeffs, other.coeffs, |a, b| a - b))
    }
}

/// Supporting Substracting an integer with a Polynomial
impl Sub<u32> for Polynomial {
    type Output = Self;

    fn sub(self, other: u32) -> Self {
        // converting the integer to FieldElement
        let other = FieldElement::new(other);
        self - Polynomial::new(vec![other])
    }
}

impl Mul for Polynomial {
    type Output = Self;

    fn mul(self, other: Self) -> Self {
        let mut result = vec![FieldElement::zero(); self.degree() + other.degree() + 1];

        for i in 0..self.coeffs.len() {
            for j in 0..other.coeffs.len() {
                result[i + j] += self.coeffs[i] * other.coeffs[j];
            }
        }
        let result = remove_trailing_elements(result, FieldElement::zero());
        Self::new(result)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_polynomial_display() {
        let p1 = Polynomial::new(vec![
            FieldElement::new(1),
            FieldElement::new(2),
            FieldElement::new(3),
        ]);
        assert_eq!(p1.to_string(), "1 + 2*X + 3*X^2");

        let p2 = Polynomial::new(vec![
            FieldElement::new(1),
            FieldElement::new(0),
            FieldElement::new(0),
            FieldElement::new(2),
        ]);
        assert_eq!(p2.to_string(), "1 + 2*X^3");

        let p3 = Polynomial::new(vec![FieldElement::zero()]);
        assert_eq!(p3.to_string(), "0");
    }
}
