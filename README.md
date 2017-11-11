# permutation-rs [![Build Status](https://travis-ci.org/fifth-postulate/solving-permutation-puzzles.rust.svg?branch=master)](https://travis-ci.org/fifth-postulate/solving-permutation-puzzles.rust)[![Crate](https://img.shields.io/crates/v/permutation-rs.svg)](https://crates.io/crates/permutation-rs)[![Coverage Status](https://coveralls.io/repos/github/fifth-postulate/solving-permutation-puzzles.rust/badge.svg?branch=master)](https://coveralls.io/github/fifth-postulate/solving-permutation-puzzles.rust?branch=master)
Rust code to solve permutation puzzles.

## Tutorial
In this tutorial we will learn to solve [the brainbow][brainbow]. Let's start by
creating a new project.

```sh
cargo new --bin --name brainbow solve-brainbow
```

Open the `Cargo.toml` and add a dependency for `permutation.rs`.

```toml
permutation-rs = "1.1.0"
```

open `src/main.rs` and announce the `permutation-rs`.

```rust
#[macro_use]
extern crate permutation_rs;
```

Next we are going to import everything to start working with the brainbow group.

```rust
use std::collections::HashMap;
use permutation_rs::group::{Group, GroupElement, Morphism};
use permutation_rs::group::special::SLPPermutation;
use permutation_rs::group::tree::SLP;
use permutation_rs::group::free::Word;
use permutation_rs::group::permutation::Permutation;
```

We are going to focus on creating the corresponding brainbow group. We introduce
a function for that.

```rust
fn brainbow() -> Group<u64, SLPPermutation> {
    let transposition = SLPPermutation::new(
        SLP::Generator(0),
        permute!(
            0, 0,
            1, 1,
            2, 2,
            3, 5,
            4, 4,
            5, 3
        ),
    );

    let rotation = SLPPermutation::new(
        SLP::Generator(1),
        permute!(
            0, 1,
            1, 2,
            2, 3,
            3, 4,
            4, 5,
            5, 0
        ),
    );

    let gset: Vec<u64> = vec!(0, 1, 2, 3, 4, 5);
    let generators = vec!(transposition, rotation);

    Group::new(gset, generators)
}
```

Let's talk about what is going on here. From the signature we learn that we are
returning a `Group<u64, SLPPermutation>`. The group elements are made up of
`SLPPermutation` and act upon `u64`. An `SLPPermutation` is the combination of a
`SLP` and a `Permutation` with some extra bookkeeping. We will get into what
this all means.

In the function we then define two generators. You can see how an
`SLPPermutation` is composed of a `SLP` and a `Permutation`. Notice that we
create a `Permutation` with the `permute!` macro. For example

```rust
permute!(
  0, 0,
  1, 1,
  2, 2,
  3, 5,
  4, 4,
  5, 3
)
```

corresponds to the following permutation in disjoint cycle notation `(3 5)`. We
create a `gset` by listing the domain elements that our group is acting upon,
and we also gather the generators of our group. From these we create our actual
group.

With the possibility of creating a group we should start to make use of it. In
the main function call the `brainbow` function and assign it to a variable.

```
let group = brainbow();
```

Now it is time to scramble up the brainbow and create a corresponding group
element to examine.

```rust
let element = SLPPermutation::new(
    SLP::Identity,
    permute!(
        0, 1,
        1, 0,
        2, 5,
        3, 4,
        4, 3,
        5, 2
    ),
);
```

We are going to use our `group` to strip this element. This determines a couple
of things. It can tell use if this element is in the group. An other thing which
we will learn is a word that will solve the scrambled brainbow if the element is
in the group.

```rust
let stripped = group.strip(element);
```

In order to now a sequence of moves that solves this instance of the brainbow
puzzle, we need a `Morphism`. A `Morphism` tells how `SLP` elements should map
into instruction.

```rust
let morphism = morphism!(0, 't', 1, 'r');
```

Let's use it to solve our puzzle.

```rust
if stripped.element.1.is_identity() {
    println!("{}", stripped.transform(&morphism).inverse());
} else {
    println!("Not solveable");
}
```

Running this program will output

```text
t^1r^4t^-1r^-1t^-1r^1t^1r^1
```

Which solves our puzzle.

## What is an SLPPermutation? 
We said before that an `SLPPermutation` is the combination of a `SLP` and a
`Permutation`. If we learn what these individual concepts mean, we get an
insight into the combination.

### What is an permutation?
A permutation is a bijection from one set to the other. Basically it sends every
element of a set for example `0, 1, 2` to an element of that set. For example

```text
0 -> 1
1 -> 2
2 -> 0
```

### What is an SLP?
`SLP` is short for [_straight line program_][slp]. It is an description of a
calculation which can be performed with other group elements. The implementation
used in this project deviate a little from more traditional programs. It is more
akin a [_Abstract Syntax Tree_][ast].

For example, the following representation of a `SLP`,

```text
(Product
  (Generator 0)
  (Inverse (Generator 1)))
```

together with a morphism that sends `Generator 0` to `(1 2)` and `Generator 1`
to `(1 2 3)` corresponds with the permutation `(2 3)` 

[brainbow]: http://fifth-postulate.nl/brainbow/
[slp]: https://en.wikipedia.org/wiki/Straight-line_program
[ast]: https://en.wikipedia.org/wiki/Abstract_syntax_tree
