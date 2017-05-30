use malachite_gmp::traits::Assign;
use rust_wheels::iterators::common::EXAMPLE_SEED;
use rust_wheels::iterators::integers::{exhaustive_natural_integers, random_natural_integers};
use rust_wheels::iterators::naturals::{exhaustive_naturals, random_naturals};
use rust_wheels::iterators::tuples::{exhaustive_pairs, random_pairs};

pub fn demo_exhaustive_natural_assign_integer(limit: usize) {
    for (mut x, y) in exhaustive_pairs(exhaustive_naturals(), exhaustive_natural_integers())
            .take(limit) {
        let x_old = x.clone();
        let y_old = y.clone();
        x.assign(y);
        println!("x := {}; x.assign({}); x = {}", x_old, y_old, x);
    }
}

pub fn demo_random_natural_assign_integer(limit: usize) {
    for (mut x, y) in random_pairs(&EXAMPLE_SEED,
                                   &(|seed| random_naturals(seed, 32)),
                                   &(|seed| random_natural_integers(seed, 32)))
                .take(limit) {
        let x_old = x.clone();
        let y_old = y.clone();
        x.assign(y);
        println!("x := {}; x.assign({}); x = {}", x_old, y_old, x);
    }
}

pub fn demo_exhaustive_natural_assign_integer_ref(limit: usize) {
    for (mut x, y) in exhaustive_pairs(exhaustive_naturals(), exhaustive_natural_integers())
            .take(limit) {
        let x_old = x.clone();
        x.assign(&y);
        println!("x := {}; x.assign(&{}); x = {}", x_old, y, x);
    }
}

pub fn demo_random_natural_assign_integer_ref(limit: usize) {
    for (mut x, y) in random_pairs(&EXAMPLE_SEED,
                                   &(|seed| random_naturals(seed, 32)),
                                   &(|seed| random_natural_integers(seed, 32)))
                .take(limit) {
        let x_old = x.clone();
        x.assign(&y);
        println!("x := {}; x.assign(&{}); x = {}", x_old, y, x);
    }
}
