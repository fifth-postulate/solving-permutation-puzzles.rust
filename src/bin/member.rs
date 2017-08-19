#[macro_use]
extern crate permutation;

use std::collections::HashMap;
use permutation::group::{Group};
use permutation::group::permutation::Permutation;

fn main() {
    let group = d6();

    let element = permute!(
        0, 1,
        1, 0,
        2, 5,
        3, 4,
        4, 3,
        5, 2);

    println!("{0} a member", group.is_member(element));
}

fn d6() -> Group<u64, Permutation> {
    let transposition = permute!(
        0u64, 1u64,
        1u64, 0u64,
        2u64, 5u64,
        3u64, 4u64,
        4u64, 3u64,
        5u64, 2u64
    );

    let rotation = permute!(
        0u64, 1u64,
        1u64, 2u64,
        2u64, 3u64,
        3u64, 4u64,
        4u64, 5u64,
        5u64, 0u64
    );

    let gset = vec!(0u64, 1u64, 2u64, 3u64, 4u64, 5u64);
    let generators = vec!(transposition, rotation);

    Group::new(gset, generators)
}
