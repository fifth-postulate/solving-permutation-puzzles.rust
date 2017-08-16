//! The core of working with groups.
//!
//! A *group* is a set _G_ with an associated operation _G_*_G_ -> _G_ such that
//! 1. The operation is associative. I.e. (_a_ * _b_) * _c_ = _a_ * (_b_ * _c_)
//!    for all _a_, _b_, _c_ in _G_.
//! 2. There exist an identity element. I.e. an _e_ in _G_ with _e_ * _g_ = _g_
//!    for all _g_ in _G_.
//! 3. For each element _g_ in _G_ there is an inverse. I.e. an element _h_ in
//!    _G_ such that _g_ * _h_ = _e_, the identity element in _G_.

pub mod permutation;

use std::hash::Hash;
use std::collections::HashMap;
use std::collections::VecDeque;

/// The contract for a group element.
pub trait GroupElement {
    /// Determine if the group element is the identity.
    fn is_identity(&self) -> bool;
    /// The associated operation of the Group.
    fn times(&self, multiplicant: &Self) -> Self;
    /// Returns the inverse of the group element.
    fn inverse(&self) -> Self;
}

/// A group can _act_ on a set. (See [Group Action](https://en.wikipedia.org/wiki/Group_action)).
pub trait GroupAction {
    /// The set the group acts on.
    type Domain;

    /// The action that the group has on the domain.
    fn act_on(&self, element: &Self::Domain) -> Self::Domain;
}


struct BaseStrongGeneratorLevel<Domain, G> where G: GroupElement + GroupAction<Domain=Domain> {
    base: Domain,
    generators: Vec<G>,
    transversal: HashMap<Domain, G>,
    stabilizers: Vec<G>,
}

impl<Domain, G> BaseStrongGeneratorLevel<Domain, G>
    where Domain: Eq + Hash + Clone, G: GroupElement + GroupAction<Domain=Domain> {
    fn new(base: Domain, generators: Vec<G>) -> BaseStrongGeneratorLevel<Domain, G> {
        let (transversal, stabilizers) = calculate_transversal(base.clone(), &generators);
        BaseStrongGeneratorLevel {
            base: base,
            generators: generators,
            transversal: transversal,
            stabilizers: stabilizers,
        }
    }
}


fn calculate_transversal<Domain, G>(base: Domain, generators: &Vec<G>) -> (HashMap<Domain, G>, Vec<G>)
    where Domain: Eq + Hash + Clone, G: GroupElement + GroupAction<Domain=Domain> {
    let mut to_visit: VecDeque<Domain> = VecDeque::new();
    let mut transversals: HashMap<Domain, G> = HashMap::new();
    let mut stabilizers: Vec<G> = vec!();
    to_visit.push_back(base.clone());
    transversals.insert(base.clone(), identity(&generators));
    while !to_visit.is_empty() {
        let element = to_visit.pop_front().unwrap();
        for ref generator in generators {
            let image = generator.act_on(&element);
            if !transversals.contains_key(&image) {
                let transversal = transversals.get(&element).unwrap().times(&generator);
                transversals.insert(image.clone(), transversal);
                to_visit.push_back(image.clone());
            } else {
                let to = transversals.get(&element).unwrap();
                let fro = transversals.get(&image).unwrap().inverse();
                let stabilizer = to.times(&generator).times(&fro);
                stabilizers.push(stabilizer);
            }
        }
    }
    (transversals, stabilizers)
}

fn identity<G>(generators: &Vec<G>) -> G where G: GroupElement {
    let g = generators.get(0).expect("at least one generator");
    let inverse = g.inverse();
    g.times(&inverse)
}
