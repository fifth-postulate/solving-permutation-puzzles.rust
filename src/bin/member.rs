extern crate permutation;

use std::collections::HashMap;
use permutation::group::{Group};
use permutation::group::permutation::Permutation;

fn main() {
    let group = d6();

    let mut element_images = HashMap::new();
    element_images.insert(0u64, 1u64);
    element_images.insert(1u64, 0u64);
    element_images.insert(2u64, 5u64);
    element_images.insert(3u64, 4u64);
    element_images.insert(4u64, 3u64);
    element_images.insert(5u64, 2u64);
    let element = Permutation::new(element_images);

    println!("{0} a member", group.is_member(element));
}

fn d6() -> Group<u64, Permutation> {
    let mut transposition_images = HashMap::new();
    transposition_images.insert(0u64, 1u64);
    transposition_images.insert(1u64, 0u64);
    transposition_images.insert(2u64, 5u64);
    transposition_images.insert(3u64, 4u64);
    transposition_images.insert(4u64, 3u64);
    transposition_images.insert(5u64, 2u64);
    let transposition = Permutation::new(transposition_images);

    let mut rotation_images = HashMap::new();
    rotation_images.insert(0u64, 1u64);
    rotation_images.insert(1u64, 2u64);
    rotation_images.insert(2u64, 3u64);
    rotation_images.insert(3u64, 4u64);
    rotation_images.insert(4u64, 5u64);
    rotation_images.insert(5u64, 0u64);
    let rotation = Permutation::new(rotation_images);

    let gset = vec!(0u64, 1u64, 2u64, 3u64, 4u64, 5u64);
    let generators = vec!(transposition, rotation);

    Group::new(gset, generators)
}
