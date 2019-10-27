//! The core of working with groups.
//!
//! A *group* is a set _G_ with an associated operation _G_ * _G_ -> _G_ such that
//!
//! 1. The operation is associative. I.e. (_a_ * _b_) * _c_ = _a_ * (_b_ * _c_)
//!    for all _a_, _b_, _c_ in _G_.
//! 2. There exist an identity element. I.e. an _e_ in _G_ with _e_ * _g_ = _g_
//!    for all _g_ in _G_.
//! 3. For each element _g_ in _G_ there is an inverse. I.e. an element _h_ in
//!    _G_ such that _g_ * _h_ = _e_, the identity element in _G_.

pub mod calculation;
pub mod free;
pub mod permutation;
pub mod special;
pub mod tree;

use std::collections::HashMap;
use std::collections::VecDeque;
use std::fmt::{Display, Error, Formatter};
use std::hash::Hash;

use self::calculation::identity;

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
where
    Domain: Eq + Hash + Clone,
    G: GroupElement + GroupAction<Domain = Domain> + PartialEq,
{
    levels: Vec<BaseStrongGeneratorLevel<Domain, G>>,
}

impl<Domain, G> Group<Domain, G>
where
    Domain: Eq + Hash + Clone,
    G: GroupElement + GroupAction<Domain = Domain> + PartialEq,
{
    /// Creates a group with a given set of generators on a certain gset.
    pub fn new(gset: Vec<Domain>, generators: Vec<G>) -> Group<Domain, G> {
        let mut levels = vec![];
        let mut gs = generators;
        while gs.len() > 0 {
            let base: Domain = find_base(&gset, &gs).expect("generators should move something");
            let (level, stabilizers) = BaseStrongGeneratorLevel::new(base, gs);
            levels.push(level);
            gs = stabilizers;
        }
        Group { levels: levels }
    }

    /// The order of the group, i.e. the number of elements this group has.
    pub fn size(&self) -> usize {
        self.levels
            .iter()
            .fold(1usize, |acc, ref level| acc * level.length())
    }

    /// Determine if a group element is a member of this group.
    pub fn is_member(&self, element: G) -> bool {
        let candidate = self.strip(element);
        candidate.is_identity()
    }

    /// Strip element with current group
    pub fn strip(&self, element: G) -> G {
        let mut candidate = element;
        for level in &self.levels {
            if level.has_transversal_for(&candidate) {
                let transversal = level
                    .transversal_for(&candidate)
                    .expect("should have transversal");
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
where
    Domain: Eq + Hash + Clone,
    G: GroupElement + GroupAction<Domain = Domain>,
{
    for original in gset {
        for generator in generators {
            let image = generator.act_on(&original);
            if &image != original {
                return Some(image.clone());
            }
        }
    }
    None
}

impl<Domain, G> Display for Group<Domain, G>
where
    Domain: Eq + Hash + Clone + Display,
    G: GroupElement + GroupAction<Domain = Domain> + PartialEq + Display,
{
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        write!(f, "<\n")?;
        for level in &self.levels {
            level.fmt(f)?;
        }
        write!(f, ">\n")
    }
}

/// A level in the Schreier-Sims Base Strong generator algorithm.
///
/// It basically is a SchreierVector with some extra book-keeping.
pub struct BaseStrongGeneratorLevel<Domain, G>
where
    Domain: Eq + Hash + Clone,
    G: GroupElement + GroupAction<Domain = Domain> + PartialEq,
{
    /// The base element for this level.
    base: Domain,
    /// Generators that act on the base to form the orbit.
    generators: Vec<G>,
    /// A [Schreier vector](https://en.wikipedia.org/wiki/Schreier_vector) for
    /// this base and generators.
    indices: HashMap<Domain, isize>,
}

impl<Domain, G> BaseStrongGeneratorLevel<Domain, G>
where
    Domain: Eq + Hash + Clone,
    G: GroupElement + GroupAction<Domain = Domain> + PartialEq,
{
    /// Create a BaseStrongGeneratorLevel with a known base and generators.
    pub fn new(base: Domain, generators: Vec<G>) -> (Self, Vec<G>) {
        let mut to_visit: VecDeque<Domain> = VecDeque::new();
        let mut indices: HashMap<Domain, isize> = HashMap::new();
        let mut stabilizers: Vec<G> = vec![];
        to_visit.push_back(base.clone());
        indices.insert(base.clone(), -1);
        while !to_visit.is_empty() {
            let element = to_visit.pop_front().unwrap();
            for (index, generator) in generators.iter().enumerate() {
                let image = generator.act_on(&element);
                if !indices.contains_key(&image) {
                    indices.insert(image.clone(), index as isize);
                    to_visit.push_back(image.clone());
                } else {
                    let to = transversal_for(&element, &generators, &indices).unwrap();
                    let fro = transversal_for(&image, &generators, &indices)
                        .unwrap()
                        .inverse();
                    let stabilizer = to.times(&generator).times(&fro);
                    if add_to_stabilizers(&stabilizer, &stabilizers) {
                        stabilizers.push(stabilizer);
                    }
                }
            }
        }
        (
            BaseStrongGeneratorLevel {
                base,
                generators,
                indices,
            },
            stabilizers,
        )
    }

    /// Determine if this levels base is acted upon by `g` in a way compatible for this level.
    pub fn has_transversal_for(&self, g: &G) -> bool {
        let image = g.act_on(&self.base);
        self.indices.contains_key(&image)
    }

    /// The transversal corresponding with `g`.
    pub fn transversal_for(&self, g: &G) -> Option<G> {
        let image = g.act_on(&self.base);
        transversal_for(&image, &self.generators, &self.indices)
    }

    /// Length of the orbit
    pub fn length(&self) -> usize {
        self.indices.len()
    }
}

fn add_to_stabilizers<Domain, G>(stabilizer: &G, stabilizers: &Vec<G>) -> bool
where
    Domain: Eq + Hash + Clone,
    G: GroupElement + GroupAction<Domain = Domain> + PartialEq,
{
    !stabilizer.is_identity() && !stabilizers.contains(&stabilizer)
}

impl<Domain, G> Display for BaseStrongGeneratorLevel<Domain, G>
where
    Domain: Eq + Hash + Clone + Display,
    G: GroupElement + GroupAction<Domain = Domain> + PartialEq + Display,
{
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        write!(f, "[{};<", self.base)?;
        for g in &self.generators {
            write!(f, " {}", g)?;
        }
        write!(f, " >;")?;
        for (domain, index) in &self.indices {
            write!(f, " {}: {}", domain, index)?;
        }
        write!(f, "]\n")
    }
}

fn transversal_for<Domain, G>(
    start: &Domain,
    generators: &Vec<G>,
    indices: &HashMap<Domain, isize>,
) -> Option<G>
where
    Domain: Eq + Hash + Clone,
    G: GroupElement + GroupAction<Domain = Domain>,
{
    let mut image = start.clone();

    if indices.contains_key(&image) {
        let mut transversal = identity(&generators);
        let mut index = indices.get(&image).unwrap();
        while *index != (-1 as isize) {
            let generator = &generators[(*index as usize)];
            let inverse = generator.inverse();
            image = inverse.act_on(&image);
            transversal = transversal.times(&inverse);
            index = indices.get(&image).unwrap();
        }
        Some(transversal.inverse())
    } else {
        None
    }
}

/// Create a Morphism by specifying images
#[macro_export]
macro_rules! morphism {
    ( $($from: expr, $to: expr),* ) => {
        {
            let mut morphism_images = HashMap::new();
            $(
                morphism_images.insert(SLP::Generator($from), Word::generator($to));
            )*
            Morphism::new(morphism_images)
        }
    }
}

/// Morphism maps one Group to the other with respect of the group operation.
pub struct Morphism<G, H>
where
    G: GroupElement + Eq + Hash,
    H: GroupElement + Eq + Hash,
{
    generator_images: HashMap<G, H>,
}

impl<G, H> Morphism<G, H>
where
    G: GroupElement + Eq + Hash,
    H: GroupElement + Eq + Hash + Clone,
{
    /// Create a new morphism with a given set of images
    pub fn new(generator_images: HashMap<G, H>) -> Morphism<G, H> {
        Morphism {
            generator_images: generator_images,
        }
    }

    /// maps an G-element to the corresponding H-element.
    pub fn transform(&self, element: &G) -> H {
        self.generator_images
            .get(element)
            .expect("should have an image")
            .clone()
    }
}

#[cfg(test)]
mod tests {
    use super::permutation::Permutation;
    use super::*;
    use std::collections::HashMap;

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

        let gset = vec![0u64, 1u64, 2u64];
        let generators = vec![transposition, rotation];

        Group::new(gset, generators)
    }

    #[test]
    fn group_should_have_a_size() {
        let group = d3();
        println!("{}", group);

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

    #[test]
    fn transversal_for_should_correctly_determine_transversal() {
        let image = 4u64;
        let mut a_image: HashMap<u64, u64> = HashMap::new();
        a_image.insert(0u64, 1u64);
        a_image.insert(1u64, 2u64);
        a_image.insert(2u64, 0u64);
        a_image.insert(3u64, 4u64);
        a_image.insert(4u64, 5u64);
        a_image.insert(5u64, 3u64);
        let a = Permutation::new(a_image);
        let mut b_image: HashMap<u64, u64> = HashMap::new();
        b_image.insert(0u64, 3u64);
        b_image.insert(1u64, 1u64);
        b_image.insert(2u64, 2u64);
        b_image.insert(3u64, 0u64);
        b_image.insert(4u64, 4u64);
        b_image.insert(5u64, 5u64);
        let b = Permutation::new(b_image);
        let generators = vec![a.clone(), b.clone()];
        let mut indices: HashMap<u64, isize> = HashMap::new();
        indices.insert(0u64, -1isize);
        indices.insert(1u64, 0isize);
        indices.insert(2u64, 0isize);
        indices.insert(3u64, 1isize);
        indices.insert(4u64, 0isize);
        indices.insert(5u64, 0isize);

        let transversal = transversal_for(&image, &generators, &indices).unwrap();

        let expected = b.times(&a);
        assert_eq!(transversal, expected);
    }
}
