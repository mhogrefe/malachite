use rust_wheels::iterators::common::EXAMPLE_SEED;
use rust_wheels::iterators::naturals::{exhaustive_naturals, random_naturals};

pub fn demo_exhaustive_natural_to_u32(limit: usize) {
    for n in exhaustive_naturals().take(limit) {
        println!("to_u32({}) = {:?}", n, n.to_u32());
    }
}

pub fn demo_random_natural_to_u32(limit: usize) {
    for n in random_naturals(&EXAMPLE_SEED, 32).take(limit) {
        println!("to_u32({}) = {:?}", n, n.to_u32());
    }
}

pub fn demo_exhaustive_natural_to_u32_wrapping(limit: usize) {
    for n in exhaustive_naturals().take(limit) {
        println!("to_u32({}) = {:?}", n, n.to_u32_wrapping());
    }
}

pub fn demo_random_natural_to_u32_wrapping(limit: usize) {
    for n in random_naturals(&EXAMPLE_SEED, 32).take(limit) {
        println!("to_u32({}) = {:?}", n, n.to_u32_wrapping());
    }
}
