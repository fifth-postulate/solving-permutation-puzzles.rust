#[macro_use]
extern crate permutation_rs;

use permutation_rs::group::free::Word;
use permutation_rs::group::permutation::Permutation;
use permutation_rs::group::special::SLPPermutation;
use permutation_rs::group::tree::SLP;
use permutation_rs::group::{Group, GroupElement, Morphism};
use std::collections::HashMap;

#[test]
fn check_returned_word() {
    let group = s6();

    let element = SLPPermutation::new(SLP::Identity, permute!(0, 1, 1, 0, 2, 5, 3, 4, 4, 3, 5, 2));

    let stripped = group.strip(element);

    let morphism = morphism!(0, 't', 1, 'r');

    assert!(stripped.element.1.is_identity());
    assert_eq!(
        stripped.transform(&morphism).inverse(),
        Word::new(vec![
            ('r', 4),
            ('t', 1),
            ('r', -5),
            ('t', -1),
            ('r', 2),
            ('t', -1),
            ('r', 1),
            ('t', -1),
            ('r', -3),
            ('t', 1),
            ('r', 5),
            ('t', 1),
            ('r', -3),
            ('t', -1),
            ('r', -1),
            ('t', 1),
            ('r', 1),
            ('t', 1),
            ('r', 3),
            ('t', 1),
            ('r', -2),
            ('t', 1)
        ])
    );
}

fn s6() -> Group<u64, SLPPermutation> {
    let transposition = SLPPermutation::new(
        SLP::Generator(0),
        permute!(0, 1, 1, 0, 2, 2, 3, 3, 4, 4, 5, 5),
    );

    let rotation = SLPPermutation::new(
        SLP::Generator(1),
        permute!(0, 1, 1, 2, 2, 3, 3, 4, 4, 5, 5, 0),
    );

    let gset = vec![0u64, 1u64, 2u64, 3u64, 4u64, 5u64];
    let generators = vec![transposition, rotation];

    Group::new(gset, generators)
}
