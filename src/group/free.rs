//! A free group are the sequence of symbols and their inverses where there are
//! no occurrences of a symbol and its inverse next to each other.

use std::fmt;
use std::fmt::Display;
use super::GroupElement;

/// The element of a free group.
#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub struct Word {
    terms: Vec<(char, i64)>,
}

impl Word {
    /// Create the identity element in a free group.
    pub fn identity() -> Word {
        Word { terms: vec!() }
    }

    /// Constructor which creates a single generator.
    pub fn generator(symbol: char) -> Word {
        Word { terms: vec!((symbol, 1))}
    }

    /// Create a word with prescribed characters.
    pub fn new(elements: Vec<(char, i64)>) -> Word {
        Word { terms : normalize(&elements) }
    }
}


fn normalize(elements: &Vec<(char, i64)>) -> Vec<(char, i64)> {
    let mut not_normalized : Vec<(char, i64)> = vec!();
    not_normalized.extend(elements);

    if not_normalized.len() <= 1 {
        not_normalized
    } else {
        let mut normalized : Vec<(char, i64)> = vec!();
        let mut current: (char, i64) = not_normalized.get(0).expect("at least two elements").clone();
        let mut index = 1;
        while index < not_normalized.len() {
            let primitive = not_normalized.get(index).expect("index within bound").clone();
            if current.0 == primitive.0 {
                current = (current.0.clone(), current.1 + primitive.1)
            } else {
                if current.1 != 0 {
                    normalized.push(current)
                } else {
                    if !normalized.is_empty() {
                        current = normalized.pop().expect("non-empty stack");
                        continue;
                    }
                }
                current = primitive
            }
            index += 1;
        }
        if current.1 != 0 {
            normalized.push(current);
        }

        normalized
    }
}

impl GroupElement for Word {
    fn is_identity(&self) -> bool {
        self.terms.len() == 0
    }

    fn times(&self, multiplicant: &Word) -> Word {
        let mut terms: Vec<(char, i64)>= vec!();
        terms.extend(&self.terms);
        terms.extend(&multiplicant.terms);
        let terms = normalize(&terms);
        Word { terms: terms }
    }

    fn inverse(&self) -> Word {
        let mut terms: Vec<(char, i64)>  = vec!();
        terms.extend(&self.terms);
        terms.reverse();
        for element in terms.iter_mut() {
            element.1 *= -1;
        }
        Word { terms: terms }
    }
}

impl Display for Word {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.terms.len() > 0 {
            for primitive in &self.terms {
                write!(f, "{}^{}", primitive.0, primitive.1)?;
            }
            write!(f, "")
        } else {
            write!(f, "Id")
        }
    }
}

#[cfg(test)]
mod tests {
    use super::super::GroupElement;
    use super::*;

    #[test]
    fn permutaion_should_know_when_it_is_the_identity() {
        let not_identity = Word::generator('g');

        assert!(!not_identity.is_identity());

        let identity = Word::identity();

        assert!(identity.is_identity());
    }

    #[test]
    fn multiplication_should_be_from_left_to_right() {
        let first = Word::generator('g');
        let second = Word::generator('h');

        let product = first.times(&second);

        let expected = Word::new(vec!(('g', 1), ('h',1)));

        assert_eq!(product, expected);
    }

    #[test]
    fn inverse_should_multiply_to_identity() {
        let first = Word::new(vec!(('g', 1), ('h',1)));

        let second = first.inverse();

        let product = first.times(&second);

        assert!(product.is_identity());
    }
    
    #[test]
    fn word_should_display_correctly() {
        let identity = Word::identity();

        let word = Word::new(vec!(('x', 2), ('y', -3), ('x', -2), ('y', 3)));

        assert_eq!("Id", format!("{}", identity));
        assert_eq!("x^2y^-3x^-2y^3", format!("{}", word));
    }
}
