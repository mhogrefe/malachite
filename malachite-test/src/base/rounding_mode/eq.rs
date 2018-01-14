use common::GenerationMode;
use malachite_base::round::RoundingMode;
use rust_wheels::iterators::common::EXAMPLE_SEED;
use rust_wheels::iterators::rounding_modes::{exhaustive_rounding_modes, random_rounding_modes};
use rust_wheels::iterators::tuples::{lex_pairs, random_pairs_from_single};

type It = Iterator<Item = (RoundingMode, RoundingMode)>;

pub fn exhaustive_inputs() -> Box<It> {
    Box::new(lex_pairs(
        exhaustive_rounding_modes(),
        exhaustive_rounding_modes(),
    ))
}

pub fn random_inputs() -> Box<It> {
    Box::new(random_pairs_from_single(random_rounding_modes(
        &EXAMPLE_SEED,
    )))
}

pub fn select_inputs(gm: GenerationMode) -> Box<It> {
    match gm {
        GenerationMode::Exhaustive => exhaustive_inputs(),
        GenerationMode::Random(_) => random_inputs(),
    }
}

pub fn demo_rounding_mode_eq(gm: GenerationMode, limit: usize) {
    for (x, y) in select_inputs(gm).take(limit) {
        if x == y {
            println!("{} = {}", x, y);
        } else {
            println!("{} ≠ {}", x, y);
        }
    }
}
