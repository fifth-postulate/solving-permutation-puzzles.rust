//! The core of working with groups.
//!
//! A *group* is a set _G_ with an associated operation _G_*_G_ -> _G_ such that
//! 1. The operation is associative. I.e. (_a_ * _b_) * _c_ = _a_ * (_b_ * _c_)
//!    for all _a_, _b_, _c_ in _G_.
//! 2. There exist an identity element. I.e. an _e_ in _G_ with _e_ * _g_ = _g_
//!    for all _g_ in _G_.
//! 3. For each element _g_ in _G_ there is an inverse. I.e. an element _h_ in
//!    _G_ such that _g_ * _h_ = _e_, the identity element in _G_.

pub trait GroupElement {
    fn is_identity(&self) -> bool;
    fn times(&self, multiplicant: &Self) -> Self;
    fn inverse(&self) -> Self;
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
    }
}
