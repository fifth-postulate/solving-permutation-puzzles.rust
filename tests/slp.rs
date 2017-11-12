#[macro_use] extern crate permutation_rs as pr;

use std::collections::HashMap;
use pr::group::GroupElement;
use pr::group::tree::{SLPFactory, SLPWord};
use pr::group::permutation::Permutation;

#[test]
fn should_correctly_evaluate_to_elements() {
    let factory = SLPFactory::new();

    let g = permute!(0, 1, 1, 2, 2, 3, 3, 0);
    let h = permute!(0, 1, 1, 0, 2, 2, 3, 3);

    let u = factory.generator(g);
    let v = factory.generator(h);

    let expression = u.times(&v).inverse();
    let g = expression.evaluate();

    let expected = permute!(0, 0, 1, 3, 2, 1, 3, 2);

    assert_eq!(g, expected);
}
