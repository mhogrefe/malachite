use common::GenerationMode;
use malachite_base::round::RoundingMode;
use rust_wheels::iterators::common::EXAMPLE_SEED;
use rust_wheels::iterators::rounding_modes::{exhaustive_rounding_modes, random_rounding_modes};
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

pub fn hash<T: Hash>(n: &T) -> u64 {
    let mut s = DefaultHasher::new();
    n.hash(&mut s);
    s.finish()
}

type It = Iterator<Item = RoundingMode>;

pub fn exhaustive_inputs() -> Box<It> {
    Box::new(exhaustive_rounding_modes())
}

pub fn random_inputs() -> Box<It> {
    Box::new(random_rounding_modes(&EXAMPLE_SEED))
}

pub fn select_inputs(gm: GenerationMode) -> Box<It> {
    match gm {
        GenerationMode::Exhaustive => exhaustive_inputs(),
        GenerationMode::Random(_) => random_inputs(),
    }
}

pub fn demo_rounding_mode_hash(gm: GenerationMode, limit: usize) {
    for rm in select_inputs(gm).take(limit) {
        println!("hash({}) = {}", rm, hash(&rm));
    }
}
