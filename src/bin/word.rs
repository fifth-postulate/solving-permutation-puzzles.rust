extern crate permutation;

use std::collections::HashMap;
use permutation::group::{Group, GroupElement, Morphism};
use permutation::group::special:: SLPPermutation;
use permutation::group::tree::SLP;
use permutation::group::free::Word;
use permutation::group::permutation::Permutation;

fn main() {
    let group = s6();

    let mut element_images = HashMap::new();
    element_images.insert(0u64, 1u64);
    element_images.insert(1u64, 0u64);
    element_images.insert(2u64, 5u64);
    element_images.insert(3u64, 4u64);
    element_images.insert(4u64, 3u64);
    element_images.insert(5u64, 2u64);
    let element = SLPPermutation::new(
        SLP::Identity,
        Permutation::new(element_images)
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
    let mut transposition_images = HashMap::new();
    transposition_images.insert(0u64, 1u64);
    transposition_images.insert(1u64, 0u64);
    transposition_images.insert(2u64, 2u64);
    transposition_images.insert(3u64, 3u64);
    transposition_images.insert(4u64, 4u64);
    transposition_images.insert(5u64, 5u64);
    let transposition = SLPPermutation::new(
        SLP::Generator(0),
        Permutation::new(transposition_images)
    );

    let mut rotation_images = HashMap::new();
    rotation_images.insert(0u64, 1u64);
    rotation_images.insert(1u64, 2u64);
    rotation_images.insert(2u64, 3u64);
    rotation_images.insert(3u64, 4u64);
    rotation_images.insert(4u64, 5u64);
    rotation_images.insert(5u64, 0u64);
    let rotation = SLPPermutation::new(
        SLP::Generator(1),
        Permutation::new(rotation_images)
    );

    let gset = vec!(0u64, 1u64, 2u64, 3u64, 4u64, 5u64);
    let generators = vec!(transposition, rotation);

    Group::new(gset, generators)
}
