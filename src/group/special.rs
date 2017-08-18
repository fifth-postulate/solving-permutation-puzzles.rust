//! Home for special groups.

use super::{GroupElement, GroupAction};
use super::tree::SLP;
use super::permutation::Permutation;

type SLPPermutation = (SLP, Permutation);

impl GroupElement for SLPPermutation {
    fn is_identity(&self) -> bool {
        self.1.is_identity()
    }

    fn times(&self, multiplicant: &SLPPermutation) -> SLPPermutation {
        (self.0.times(&multiplicant.0), self.1.times(&multiplicant.1))
    }

    fn inverse(&self) -> SLPPermutation {
        (self.0.inverse(), self.1.inverse())
    }
}

impl GroupAction for SLPPermutation {
    type Domain = u64;

    fn act_on(&self, original: &u64) -> u64 {
        self.1.act_on(original)
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;
    use super::super::{GroupElement, GroupAction};
    use super::super::tree::SLP;
    use super::super::permutation::Permutation;
    use super::SLPPermutation;

    #[test]
    fn slp_permutaion_should_know_when_it_is_the_identity() {
        let mut not_identity_images = HashMap::new();
        not_identity_images.insert(0u64, 1u64);
        not_identity_images.insert(1u64, 0u64);
        let not_identity: SLPPermutation = (SLP::Generator(1), Permutation::new(not_identity_images));

        assert!(!not_identity.is_identity());

        let mut identity_images = HashMap::new();
        identity_images.insert(0u64, 0u64);
        identity_images.insert(1u64, 1u64);
        let identity: SLPPermutation = (SLP::Identity, Permutation::new(identity_images));

        assert!(identity.is_identity());
    }

    #[test]
    fn multiplication_should_be_from_left_to_right() {
        let mut first_images = HashMap::new();
        first_images.insert(0u64, 1u64);
        first_images.insert(1u64, 0u64);
        first_images.insert(2u64, 2u64);
        let first: SLPPermutation = (SLP::Generator(1), Permutation::new(first_images));

        let mut second_images = HashMap::new();
        second_images.insert(0u64, 0u64);
        second_images.insert(1u64, 2u64);
        second_images.insert(2u64, 1u64);
        let second: SLPPermutation = (SLP::Generator(2), Permutation::new(second_images));

        let product = first.times(&second);

        let mut expected_images = HashMap::new();
        expected_images.insert(0u64, 2u64);
        expected_images.insert(1u64, 0u64);
        expected_images.insert(2u64, 1u64);
        let expected: SLPPermutation = (
            SLP::Product(Box::new(SLP::Generator(1)), Box::new(SLP::Generator(2))),
            Permutation::new(expected_images));

        assert_eq!(product, expected);
    }

    #[test]
    fn inverse_should_multiply_to_identity() {
        let mut first_images = HashMap::new();
        first_images.insert(0u64, 1u64);
        first_images.insert(1u64, 2u64);
        first_images.insert(2u64, 0u64);
        let first: SLPPermutation = (SLP::Generator(1), Permutation::new(first_images));

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
        let permutation: SLPPermutation = (SLP::Generator(1), Permutation::new(permutation_images));

        assert_eq!(permutation.act_on(&0u64), 1u64);
        assert_eq!(permutation.act_on(&1u64), 2u64);
        assert_eq!(permutation.act_on(&2u64), 0u64);
    }

    // #[test]
    // fn permutation_should_display_correctly() {
    //     let mut identity_images = HashMap::new();
    //     identity_images.insert(0u64, 0u64);
    //     identity_images.insert(1u64, 1u64);
    //     let identity = Permutation::new(identity_images);

    //     let mut permutation_images = HashMap::new();
    //     permutation_images.insert(0u64, 1u64);
    //     permutation_images.insert(1u64, 2u64);
    //     permutation_images.insert(2u64, 0u64);
    //     permutation_images.insert(3u64, 4u64);
    //     permutation_images.insert(4u64, 3u64);
    //     let permutation = Permutation::new(permutation_images);

    //     assert_eq!("Id", format!("{}", identity));
    //     assert_eq!("(0 1 2)(3 4)", format!("{}", permutation));
    // }
}
