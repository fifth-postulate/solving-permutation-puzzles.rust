#[macro_use]
extern crate permutation_rs as pr;

use pr::group::permutation::Permutation;
use pr::group::tree::SLPFactory;
use pr::group::GroupElement;
use std::collections::HashMap;

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
