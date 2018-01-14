use common::GenerationMode;
use malachite_base::round::RoundingMode;
use rust_wheels::iterators::common::EXAMPLE_SEED;
use rust_wheels::iterators::rounding_modes::{exhaustive_rounding_modes, random_rounding_modes};
use rust_wheels::iterators::tuples::{lex_pairs, random_pairs_from_single};

type It1 = Iterator<Item = RoundingMode>;

pub fn exhaustive_inputs_1() -> Box<It1> {
    Box::new(exhaustive_rounding_modes())
}

pub fn random_inputs_1() -> Box<It1> {
    Box::new(random_rounding_modes(&EXAMPLE_SEED))
}

pub fn select_inputs_1(gm: GenerationMode) -> Box<It1> {
    match gm {
        GenerationMode::Exhaustive => exhaustive_inputs_1(),
        GenerationMode::Random(_) => random_inputs_1(),
    }
}

type It2 = Iterator<Item = (RoundingMode, RoundingMode)>;

pub fn exhaustive_inputs_2() -> Box<It2> {
    Box::new(lex_pairs(
        exhaustive_rounding_modes(),
        exhaustive_rounding_modes(),
    ))
}

pub fn random_inputs_2() -> Box<It2> {
    Box::new(random_pairs_from_single(random_rounding_modes(
        &EXAMPLE_SEED,
    )))
}

pub fn select_inputs_2(gm: GenerationMode) -> Box<It2> {
    match gm {
        GenerationMode::Exhaustive => exhaustive_inputs_2(),
        GenerationMode::Random(_) => random_inputs_2(),
    }
}

#[allow(unknown_lints, clone_on_copy)]
pub fn demo_rounding_mode_clone(gm: GenerationMode, limit: usize) {
    for rm in select_inputs_1(gm).take(limit) {
        println!("clone({}) = {}", rm, rm.clone());
    }
}

#[allow(unknown_lints, clone_on_copy)]
pub fn demo_rounding_mode_clone_from(gm: GenerationMode, limit: usize) {
    for (mut x, y) in select_inputs_2(gm).take(limit) {
        let x_old = x.clone();
        x.clone_from(&y);
        println!("x := {}; x.clone_from({}); x = {}", x_old, y, x);
    }
}
