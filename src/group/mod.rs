//! The core of working with groups.
//!
//! A *group* is a set _G_ with an associated operation _G_*_G_ -> _G_ such that
//! 1. The operation is associative. I.e. (_a_ * _b_) * _c_ = _a_ * (_b_ * _c_)
//!    for all _a_, _b_, _c_ in _G_.
//! 2. There exist an identity element. I.e. an _e_ in _G_ with _e_ * _g_ = _g_
//!    for all _g_ in _G_.
//! 3. For each element _g_ in _G_ there is an inverse. I.e. an element _h_ in
//!    _G_ such that _g_ * _h_ = _e_, the identity element in _G_.

use std::collections::HashMap;

/// The contract for a group element.
pub trait GroupElement {
    /// Determine if the group element is the identity.
    fn is_identity(&self) -> bool;
    /// The associated operation of the Group.
    fn times(&self, multiplicant: &Self) -> Self;
    /// Returns the inverse of the group element.
    fn inverse(&self) -> Self;
}

/// A permutation is a bijection of a set. For us the set will be 0..n for a
/// suitable choice of n.
#[derive(Debug, PartialEq)]
pub struct Permutation {
    n: usize,
    images: HashMap<u64, u64>,
}

impl Permutation {
    /// Create an permutation with a given image.
    pub fn new(images: HashMap<u64, u64>) -> Permutation {
        let n = images.len();
        Permutation { images: images, n: n }
    }
}

impl GroupElement for Permutation {
    fn is_identity(&self) -> bool {
        for i in 0..self.n {
            let original = i as u64;
            let image = self.images.get(&original).unwrap_or(&original).clone();
            if image != original {
                return false
            } 
        }
        true
    }

    fn times(&self, multiplicant: &Permutation) -> Permutation {
        let max_n = if self.n > multiplicant.n { self.n } else { multiplicant.n };
        let mut images = HashMap::new();
        for i in 0..max_n {
            let original = i as u64;
            let mut image = self.images.get(&original).unwrap_or(&original).clone();
            image = multiplicant.images.get(&image).unwrap_or(&image).clone();
            images.insert(original, image);
        }
        Permutation::new(images)
    }

    fn inverse(&self) -> Permutation {
        let mut images = HashMap::new();
        for i in 0..self.n {
            let original = i as u64;
            let image = self.images.get(&original).unwrap_or(&original).clone();
            images.insert(image, original);
        }
        Permutation::new(images)
    }
}

/// A group can _act_ on a set. (See [Group Action](https://en.wikipedia.org/wiki/Group_action)).
pub trait GroupAction {
    /// The set the group acts on.
    type Domain;

    /// The action that the group has on the domain.
    fn act_on(&self, element: &Self::Domain) -> Self::Domain;
}

impl GroupAction for Permutation {
    type Domain = u64;

    fn act_on(&self, original: &u64) -> u64 {
        self.images.get(&original).unwrap_or(&original).clone()
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;
    use super::*;

    #[test]
    fn permutaion_should_know_when_it_is_the_identity() {
        let mut not_identity_images = HashMap::new();
        not_identity_images.insert(0u64, 1u64);
        not_identity_images.insert(1u64, 0u64);
        let not_identity = Permutation::new(not_identity_images);

        assert!(!not_identity.is_identity());

        let mut identity_images = HashMap::new();
        identity_images.insert(0u64, 0u64);
        identity_images.insert(1u64, 1u64);
        let identity = Permutation::new(identity_images);

        assert!(identity.is_identity());
    }

    #[test]
    fn multiplication_should_be_from_left_to_right() {
        let mut first_images = HashMap::new();
        first_images.insert(0u64, 1u64);
        first_images.insert(1u64, 0u64);
        first_images.insert(2u64, 2u64);
        let first = Permutation::new(first_images);

        let mut second_images = HashMap::new();
        second_images.insert(0u64, 0u64);
        second_images.insert(1u64, 2u64);
        second_images.insert(2u64, 1u64);
        let second = Permutation::new(second_images);

        let product = first.times(&second);

        let mut expected_images = HashMap::new();
        expected_images.insert(0u64, 2u64);
        expected_images.insert(1u64, 0u64);
        expected_images.insert(2u64, 1u64);
        let expected = Permutation::new(expected_images);

        assert_eq!(product, expected);
    }

    #[test]
    fn inverse_should_multiply_to_identity() {
        let mut first_images = HashMap::new();
        first_images.insert(0u64, 1u64);
        first_images.insert(1u64, 2u64);
        first_images.insert(2u64, 0u64);
        let first = Permutation::new(first_images);

        let second = first.inverse();

        let product = first.times(&second);

        assert!(product.is_identity());
    }

    #[test]
    fn permutation_should_act_upon_integers() {
        let mut permutation_images = HashMap::new();
        permutation_images.insert(0u64, 1u64);
        permutation_images.insert(1u64, 2u64);
        permutation_images.insert(2u64, 0u64);
        let permutation = Permutation::new(permutation_images);

        assert_eq!(permutation.act_on(&0u64), 1u64);
        assert_eq!(permutation.act_on(&1u64), 2u64);
        assert_eq!(permutation.act_on(&2u64), 0u64);
    }
}
