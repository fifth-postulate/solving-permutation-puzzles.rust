//! A permutation is a bijection of a set. Together with function composition
//! this forms a group.
//!
//! # Examples
//! A permutation is a `GroupElement` and can therefore be multiplied together.
//! From the following computation it is clear that we compose from left to
//! right.
//!
//! ```rust
//! use std::collections::HashMap;
//! use permutation_rs::group::GroupElement;
//! use permutation_rs::group::permutation::Permutation;
//!
//! let mut left_image = HashMap::new();
//! left_image.insert(0, 1);
//! left_image.insert(1, 0);
//! left_image.insert(2, 2);
//! let left = Permutation::new(left_image);
//!
//! let mut right_image = HashMap::new();
//! right_image.insert(0, 0);
//! right_image.insert(1, 2);
//! right_image.insert(2, 1);
//! let right = Permutation::new(right_image);
//!
//! let answer = left.times(&right);
//!
//! let mut expected_image = HashMap::new();
//! expected_image.insert(0, 2);
//! expected_image.insert(1, 0);
//! expected_image.insert(2, 1);
//! let expected = Permutation::new(expected_image);
//!
//! assert_eq!(answer, expected);
//! ```
//!
//! Writing the above permutations can get a bit unwieldy. You can use the
//! `permute!` macro to alleviate the tedium a bit. For example, the `left`
//! permutation could have been written like this
//!
//! ```rust
//! # #[macro_use] extern crate permutation_rs;
//! # use std::collections::HashMap;
//! # use permutation_rs::group::permutation::Permutation;
//! # fn main() {
//! let left = permute!(
//!     0, 1,
//!     1, 0,
//!     2, 2
//! );
//! # }
//! ```

use std::collections::HashMap;
use std::collections::HashSet;
use std::fmt;
use std::fmt::Display;
use super::{GroupElement, GroupAction};

#[macro_export]
macro_rules! permute {
    ( $($from: expr, $to: expr),* ) => {
        {
            let mut permutation_images = HashMap::new();
            $(
                permutation_images.insert($from, $to);
            )*
            Permutation::new(permutation_images)
        }
    }
}

/// A permutation of the set 0..n for a suitable choice of n.
#[derive(Debug, PartialEq, Clone)]
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

impl GroupAction for Permutation {
    type Domain = u64;

    fn act_on(&self, original: &u64) -> u64 {
        self.images.get(&original).unwrap_or(&original).clone()
    }
}

impl Display for Permutation {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let cycles: Vec<Vec<u64>> = cycles(self.n, &self.images);
        if cycles.len() > 0 {
            for cycle in cycles {
                let representations: Vec<String> =
                    cycle
                    .into_iter()
                    .map(|element| format!("{}", element))
                    .collect();
                let representation: String = representations.join(" ");
                write!(f, "(")?;
                write!(f, "{}", representation)?;
                write!(f, ")")?
            }
            write!(f, "")
        } else {
            write!(f, "Id")
        }
    }
}

fn cycles(n: usize, images: &HashMap<u64, u64>) -> Vec<Vec<u64>> {
    let mut cycles = vec!();
    let mut visited = HashSet::new();
    for i in 0..n {
        let original = i as u64;
        if !visited.contains(&original) {
            visited.insert(original.clone());
            let mut cycle = vec!(original.clone());
            let mut image = images.get(&original).unwrap_or(&original).clone();
            while !visited.contains(&image) {
                visited.insert(image.clone());
                cycle.push(image.clone());
                image = images.get(&image).unwrap_or(&image).clone();
            }
            if cycle.len() > 1 {
                cycles.push(cycle);
            }
        }
    }
    cycles
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;
    use super::super::{GroupElement, GroupAction};
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

    #[test]
    fn permutation_should_display_correctly() {
        let mut identity_images = HashMap::new();
        identity_images.insert(0u64, 0u64);
        identity_images.insert(1u64, 1u64);
        let identity = Permutation::new(identity_images);

        let mut permutation_images = HashMap::new();
        permutation_images.insert(0u64, 1u64);
        permutation_images.insert(1u64, 2u64);
        permutation_images.insert(2u64, 0u64);
        permutation_images.insert(3u64, 4u64);
        permutation_images.insert(4u64, 3u64);
        let permutation = Permutation::new(permutation_images);

        assert_eq!("Id", format!("{}", identity));
        assert_eq!("(0 1 2)(3 4)", format!("{}", permutation));
    }
}
