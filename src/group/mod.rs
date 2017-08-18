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
pub mod free;
pub mod tree;

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

/// The actual group.
pub struct Group<Domain, G>
    where Domain: Eq + Hash + Clone, G: GroupElement + GroupAction<Domain=Domain> {
    levels: Vec<BaseStrongGeneratorLevel<Domain, G>>,
}

impl<Domain, G> Group<Domain, G>
    where Domain: Eq + Hash + Clone, G: GroupElement + GroupAction<Domain=Domain> {
    /// Creates a group with a given set of generators on a certain gset.
    pub fn new(gset: Vec<Domain>, generators: Vec<G>) -> Group<Domain, G> {
        let mut levels = vec!();
        let mut gs = generators;
        while gs.len() > 0 {
            let base: Domain = find_base(&gset, &gs).expect("generators should move something");
            let (level, stabilizers) = BaseStrongGeneratorLevel::new(base, &gs);
            levels.push(level);
            gs = stabilizers;
        }
        Group { levels: levels }
    }

    /// The order of the group, i.e. the number of elements this group has.
    pub fn size(&self) -> usize {
        self.levels
            .iter()
            .fold(1usize, |acc, ref level| acc * level.transversal.len() )
    }

    /// Determine if a group element is a member of this group.
    pub fn is_member(&self, element: G) -> bool {
        let candidate = self.strip(element);
        candidate.is_identity()
    }

    fn strip(&self, element: G) -> G {
        let mut candidate = element;
        for level in &self.levels {
            if level.has_transversal_for(&candidate) {
                let transversal = level.transversal_for(&candidate).expect("should have transversal");
                let inverse = transversal.inverse();
                candidate = candidate.times(&inverse);
            } else {
                break;
            }
        }
        candidate
    }
}

fn find_base<Domain, G>(gset: &Vec<Domain>, generators: &Vec<G>) -> Option<Domain>
    where Domain: Eq + Hash + Clone, G: GroupElement + GroupAction<Domain=Domain> {
    for original in gset {
        for generator in generators {
            let image = generator.act_on(&original);
            if &image != original {
                return Some(image.clone())
            }
        }
    }
    None
}

struct BaseStrongGeneratorLevel<Domain, G>
    where Domain: Eq + Hash + Clone, G: GroupElement + GroupAction<Domain=Domain> {
    base: Domain,
    transversal: HashMap<Domain, G>,
}

impl<Domain, G> BaseStrongGeneratorLevel<Domain, G>
    where Domain: Eq + Hash + Clone, G: GroupElement + GroupAction<Domain=Domain> {
    fn new(base: Domain, generators: &Vec<G>) -> (BaseStrongGeneratorLevel<Domain, G>, Vec<G>) {
        let (transversal, stabilizers) = calculate_transversal(base.clone(), &generators);
        (
            BaseStrongGeneratorLevel  {
                base: base,
                transversal: transversal,
            },
            stabilizers
        )
    }

    fn has_transversal_for(&self, g: &G) -> bool {
        let image = g.act_on(&self.base);
        self.transversal.contains_key(&image)
    }

    fn transversal_for(&self, g: &G) -> Option<&G> {
        let image = g.act_on(&self.base);
        self.transversal.get(&image)
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
                if !stabilizer.is_identity() {
                    stabilizers.push(stabilizer);
                }
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

/// Morphism maps one Group to the other with respect of the group operation.
pub struct Morphism<G, H>
    where G: GroupElement + Eq + Hash, H: GroupElement + Eq + Hash {
    generator_images: HashMap<G, H>
}

impl<G, H> Morphism<G, H>
    where G: GroupElement + Eq + Hash, H: GroupElement + Eq + Hash {
    /// Create a new morphism with a given set of images
    pub fn new(generator_images: HashMap<G, H>) -> Morphism<G, H> {
        Morphism { generator_images: generator_images }
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;
    use super::permutation::Permutation;
    use super::*;

    fn d3() -> Group<u64, Permutation> {
        let mut transposition_images = HashMap::new();
        transposition_images.insert(0u64, 1u64);
        transposition_images.insert(1u64, 0u64);
        transposition_images.insert(2u64, 2u64);
        let transposition = Permutation::new(transposition_images);

        let mut rotation_images = HashMap::new();
        rotation_images.insert(0u64, 1u64);
        rotation_images.insert(1u64, 2u64);
        rotation_images.insert(2u64, 0u64);
        let rotation = Permutation::new(rotation_images);

        let gset = vec!(0u64, 1u64, 2u64);
        let generators = vec!(transposition, rotation);

        Group::new(gset, generators)
    }

    #[test]
    fn group_should_have_a_size() {
        let group = d3();

        assert_eq!(group.size(), 6);
    }

    #[test]
    fn group_should_determine_if_an_element_is_a_member() {
        let mut transposition_images = HashMap::new();
        transposition_images.insert(0u64, 2u64);
        transposition_images.insert(1u64, 1u64);
        transposition_images.insert(2u64, 0u64);
        let transposition = Permutation::new(transposition_images);

        let group = d3();

        assert!(group.is_member(transposition));
    }
}
