#[macro_use]
extern crate permutation;

use std::collections::HashMap;
use permutation::group::{Group, GroupElement, Morphism};
use permutation::group::special:: SLPPermutation;
use permutation::group::tree::SLP;
use permutation::group::free::Word;
use permutation::group::permutation::Permutation;

fn main() {
    let group = s6();

    let element = SLPPermutation::new(
        SLP::Identity,
        permute!(
            0, 1,
            1, 0,
            2, 5,
            3, 4,
            4, 3,
            5, 2
        ),
    );

    let stripped = group.strip(element);

    let mut generator_images = HashMap::new();
    generator_images.insert(SLP::Generator(0), Word::generator('t'));
    generator_images.insert(SLP::Generator(1), Word::generator('r'));
    let morphism = Morphism::new(generator_images);

    println!("{0} {1}",
             stripped.element.1.inverse(),
             stripped.transform(&morphism).inverse());
}

fn s6() -> Group<u64, SLPPermutation> {
    let transposition = SLPPermutation::new(
        SLP::Generator(0),
        permute!(
            0, 1,
            1, 0,
            2, 2,
            3, 3,
            4, 4,
            5, 5
        ),
    );

    let rotation = SLPPermutation::new(
        SLP::Generator(1),
        permute!(
            0, 1,
            1, 2,
            2, 3,
            3, 4,
            4, 5,
            5, 0
        ),
    );

    let gset = vec!(0u64, 1u64, 2u64, 3u64, 4u64, 5u64);
    let generators = vec!(transposition, rotation);

    Group::new(gset, generators)
}
