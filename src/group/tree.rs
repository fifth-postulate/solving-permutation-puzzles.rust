//! In order to cut down on exponential growth of words when forming products
//! we are creating the structure of a calculation. When actual calculations
//! need to be done, we can use a morphism to determine the result.

use std::fmt;
use std::fmt::Display;
use super::{GroupElement};

/// Single Line Program (SLP) references various elements to form a expression
/// That can be evaluated to actual group elements.
#[derive(Debug, PartialEq, Clone)]
pub enum SLP {
    /// The identity element of a SLP.
    Identity,
    /// A generator, indexed by an integer.
    Generator(u64),
    /// Product of two SLPs.
    Product(Box<SLP>, Box<SLP>),
    /// Inverse of a SLP.
    Inverse(Box<SLP>),
}

impl GroupElement for SLP {
    fn is_identity(&self) -> bool {
        match *self {
            SLP::Identity => true,
            _ => false,
        }
    }

    fn times(&self, multiplicant: &SLP) -> SLP {
        let left: SLP = self.clone();
        let right: SLP = multiplicant.clone();
        SLP::Product(Box::new(left), Box::new(right))
    }

    fn inverse(&self) -> SLP {
        SLP::Inverse(Box::new(self.clone()))
    }
}

impl Display for SLP {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            SLP::Identity => write!(f, "Id"),
            SLP::Generator(n) => write!(f, "G_{}", n),
            SLP::Product(ref left, ref right) => write!(f, "({}) * ({})", left, right),
            SLP::Inverse(ref term) => write!(f, "({})^-1", term),
        }
    } 
}

#[cfg(test)]
mod tests {
    use super::super::GroupElement;
    use super::*;

    #[test]
    fn slp_should_know_when_it_is_the_identity() {
        let not_identity = SLP::Generator(1);

        assert!(!not_identity.is_identity());

        let identity = SLP::Identity;

        assert!(identity.is_identity());
    }

    #[test]
    fn multiplication_should_be_from_left_to_right() {
        let first = SLP::Generator(1);

        let second = SLP::Generator(2);

        let product = first.times(&second);

        let expected = SLP::Product(Box::new(first), Box::new(second));

        assert_eq!(product, expected);
    }

    #[test]
    fn inverse_should_multiply_to_identity() {
        let first = SLP::Generator(1);

        let inverse = first.inverse();

        let expected = SLP::Inverse(Box::new(first));

        assert_eq!(inverse, expected);
    }

    #[test]
    fn permutation_should_display_correctly() {
        let identity = SLP::Identity;
        let generator = SLP::Generator(1);
        let product = SLP::Product(
            Box::new(SLP::Generator(1)),
            Box::new(SLP::Generator(2)));
        let inverse = SLP::Inverse(Box::new(SLP::Generator(1)));


        assert_eq!("Id", format!("{}", identity));
        assert_eq!("G_1", format!("{}", generator));
        assert_eq!("(G_1) * (G_2)", format!("{}", product));
        assert_eq!("(G_1)^-1", format!("{}",  inverse));
    }
}
