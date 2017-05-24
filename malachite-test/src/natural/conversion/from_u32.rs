use malachite_gmp::natural::Natural;
use rust_wheels::iterators::common::EXAMPLE_SEED;
use rust_wheels::iterators::general::random_x;
use rust_wheels::iterators::primitive_ints::exhaustive_u;

pub fn demo_exhaustive_natural_from_u32(limit: usize) {
    for u in exhaustive_u::<u32>().take(limit) {
        println!("from({}) = {}", u, Natural::from(u));
    }
}

pub fn demo_random_natural_from_u32(limit: usize) {
    for u in random_x::<u32>(&EXAMPLE_SEED).take(limit) {
        println!("from({}) = {}", u, Natural::from(u));
    }
}
