//! In order to cut down on exponential growth of words when forming products
//! we are creating the structure of a calculation. When actual calculations
//! need to be done, we can use a morphism to determine the result.
//!
//! # Examples
//! Below you find an example of how an `SLP` can be used.
//!
//! ```rust
//! # #[macro_use] extern crate permutation_rs;
//! # use std::collections::HashMap;
//! # use permutation_rs::group::{GroupElement, Morphism};
//! # use permutation_rs::group::tree::SLP;
//! # use permutation_rs::group::free::Word;
//! # fn main() {
//! let left = SLP::Generator(0);
//! let right = SLP::Generator(1);
//! let expression = left.times(&right.inverse());
//!
//! let morphism = morphism!(
//!     0, 'a',
//!     1, 'b');
//!
//! let word = expression.transform(&morphism);
//!
//! let expected = Word::new(vec![('a', 1), ('b', -1)]);
//!
//! assert_eq!(word, expected);
//! # }
//! ```

use std::hash::Hash;
use std::collections::HashMap;
use std::rc::Rc;
use std::cell::{RefMut, RefCell};
use std::fmt;
use std::fmt::Display;
use super::{GroupElement, GroupAction, Morphism};
use super::free::Word;

/// A `SLPElement` keeps track of how a word is formed in a `SLPCollection`.
pub enum SLPElement {
    /// The base element, will evaluate to a group element.
    Generator(u64),
    /// A product of other words, which will be looked up by id in a `SLPCollection`.
    Product(u64, u64),
    /// An inverse of an other word, looked up by id in a `SLPCollection`.
    Inverse(u64),
}

/// A `SLPCollection` keeps tracks of various words that are build up from each
/// other.
pub struct SLPCollection<G> where G: GroupElement {
    next_id: u64,
    associations: HashMap<u64, SLPElement>,
    evaluator: HashMap<u64, G>,
}

impl<G> SLPCollection<G> where G: GroupElement + Clone {
    /// Create a `SLPCollection`
    pub fn new() -> SLPCollection<G> {
        SLPCollection {
            next_id: 0,
            associations: HashMap::new(),
            evaluator: HashMap::new(),
        }
    }

    /// Register a new word in the collection. Returns the id with which this
    /// element can be looked up.
    pub fn register(&mut self, element: SLPElement) -> u64 {
        let id = self.next_id;
        self.associations.insert(id, element);
        self.next_id = id + 1;

        id
    }

    /// Registers a generator that will evaluate to the group element `g`.
    /// Return the id with which this `Generator` element can be looked up.
    pub fn generator(&mut self, g: G) -> u64 {
        let id = self.next_id;
        self.associations.insert(id, SLPElement::Generator(id));
        self.evaluator.insert(id, g);
        self.next_id = id + 1;

        id
    }

    fn evaluate(&self, id: &u64) -> Option<G> {
        if self.associations.contains_key(id) {
            match *self.associations.get(id).unwrap() {
                SLPElement::Generator(id) => {
                    let g = self.evaluator.get(&id).unwrap();
                    let clone = (*g).clone();
                    Some(clone)
                },

                SLPElement::Product(left_id, right_id) => {
                    let left = self.evaluate(&left_id).unwrap();
                    let right = self.evaluate(&right_id).unwrap();
                    let product = left.times(&right);

                    Some(product)
                },

                SLPElement::Inverse(id) => {
                    let g = self.evaluate(&id).unwrap();

                    Some(g.inverse())
                }, 
            }
        } else {
            None
        }
    }
}

/// `SLPWord`s for the actual group elements of a SLP.
///
/// To create `SLPWord` generators you need a `SLPFactory`. Otherwise you can
/// form new words by forming products and taking inverses.
pub struct SLPWord<G> where G: GroupElement + Clone {
    collection: Rc<RefCell<SLPCollection<G>>>,
    id: u64,
}

impl<G> SLPWord<G> where G: GroupElement + Clone {
    /// Evaluate this `SLPWord` according to the evaluation setup by
    /// construction.
    pub fn evaluate(&self) -> G {
        let collection_ref = self.collection.borrow();
        (*collection_ref).evaluate(&self.id).unwrap()
    }
}

impl<G> GroupElement for SLPWord<G> where G: GroupElement + Clone {
    fn is_identity(&self) -> bool {
        unimplemented!();
    }

    fn times(&self, multiplicant: &Self) -> Self {
        let element = SLPElement::Product(self.id, multiplicant.id);
        let mut collection_ref: RefMut<SLPCollection<G>> = self.collection.borrow_mut();
        let id = (*collection_ref).register(element);

        SLPWord { collection: self.collection.clone(), id }
    }

    fn inverse(&self) -> Self {
        let element = SLPElement::Inverse(self.id);
        let mut collection_ref: RefMut<SLPCollection<G>> = self.collection.borrow_mut();
        let id = (*collection_ref).register(element);

        SLPWord { collection: self.collection.clone(), id }
    }
}

impl<Domain, G> GroupAction for SLPWord<G>
    where Domain: Eq + Hash + Clone, G: GroupElement + GroupAction<Domain=Domain> + Clone {
    type Domain = Domain;

    fn act_on(&self, element: &Self::Domain) -> Self::Domain {
        self.evaluate().act_on(&element)
    }
}

/// An `SLPFactory` creates `SLPWord`s that corresponds with generators.
pub struct SLPFactory<G> where G: GroupElement {
    collection: Rc<RefCell<SLPCollection<G>>>,
}

impl<G> SLPFactory<G> where G: GroupElement + Clone {
    /// Create a new `SLPFactory`.
    pub fn new() -> SLPFactory<G> {
        let collection = SLPCollection::new();

        SLPFactory { collection: Rc::new(RefCell::new(collection)) }
    }

    /// Create an `SLPWord` that evaluates to the group element `g`.
    pub fn generator(&self, g: G) -> SLPWord<G> {
        let mut collection_ref: RefMut<SLPCollection<G>> = self.collection.borrow_mut();
        let id = (*collection_ref).generator(g);

        SLPWord { collection: self.collection.clone(), id }
    }
}


/// Single Line Program (SLP) references various elements to form a expression
/// That can be evaluated to actual group elements.
#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub enum SLP {
    /// The identity element of a SLP.
    Identity,
    /// A generator, indexed by an integer.
    Generator(u64),
    /// Product of two SLPs.
    Product(Box<SLP>, Box<SLP>),
    /// Inverse of a SLP.
    Inverse(Box<SLP>),
}

impl SLP {
    /// Map the `SLP` in to a `Word` according to the `Morphism`.
    pub fn transform(&self, morphism: &Morphism<SLP, Word>) -> Word {
        match *self {
            SLP::Identity => Word::identity(),
            ref g @ SLP::Generator(_) => morphism.transform(&g),
            SLP::Product(ref left, ref right) => (*left).transform(&morphism).times(&(*right).transform(&morphism)),
            SLP::Inverse(ref g) => (*g).transform(&morphism).inverse(),
        }
    }
}

impl GroupElement for SLP {
    fn is_identity(&self) -> bool {
        match *self {
            SLP::Identity => true,
            _ => false,
        }
    }

    fn times(&self, multiplicant: &SLP) -> SLP {
        let left: SLP = self.clone();
        let right: SLP = multiplicant.clone();
        SLP::Product(Box::new(left), Box::new(right))
    }

    fn inverse(&self) -> SLP {
        SLP::Inverse(Box::new(self.clone()))
    }
}

impl Display for SLP {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            SLP::Identity => write!(f, "Id"),
            SLP::Generator(n) => write!(f, "G_{}", n),
            SLP::Product(ref left, ref right) => write!(f, "({}) * ({})", left, right),
            SLP::Inverse(ref term) => write!(f, "({})^-1", term),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::super::GroupElement;
    use super::*;

    #[test]
    fn slp_should_know_when_it_is_the_identity() {
        let not_identity = SLP::Generator(1);

        assert!(!not_identity.is_identity());

        let identity = SLP::Identity;

        assert!(identity.is_identity());
    }

    #[test]
    fn multiplication_should_be_from_left_to_right() {
        let first = SLP::Generator(1);

        let second = SLP::Generator(2);

        let product = first.times(&second);

        let expected = SLP::Product(Box::new(first), Box::new(second));

        assert_eq!(product, expected);
    }

    #[test]
    fn inverse_should_multiply_to_identity() {
        let first = SLP::Generator(1);

        let inverse = first.inverse();

        let expected = SLP::Inverse(Box::new(first));

        assert_eq!(inverse, expected);
    }

    #[test]
    fn should_display_correctly() {
        let identity = SLP::Identity;
        let generator = SLP::Generator(1);
        let product = SLP::Product(
            Box::new(SLP::Generator(1)),
            Box::new(SLP::Generator(2)));
        let inverse = SLP::Inverse(Box::new(SLP::Generator(1)));


        assert_eq!("Id", format!("{}", identity));
        assert_eq!("G_1", format!("{}", generator));
        assert_eq!("(G_1) * (G_2)", format!("{}", product));
        assert_eq!("(G_1)^-1", format!("{}",  inverse));
    }
}
