use common::GenerationMode;
use malachite_base::round::RoundingMode;
use rust_wheels::iterators::common::EXAMPLE_SEED;
use rust_wheels::iterators::rounding_modes::{exhaustive_rounding_modes, random_rounding_modes};

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

pub fn demo_rounding_mode_neg(gm: GenerationMode, limit: usize) {
    for rm in select_inputs(gm).take(limit) {
        println!("-{} = {}", rm, -rm);
    }
}
