use common::GenerationMode;
use inputs::common::{reshape_1_2_to_3, reshape_2_1_to_3};
use malachite_base::num::{PrimitiveInteger, PrimitiveSigned, PrimitiveUnsigned};
use malachite_base::round::RoundingMode;
use malachite_nz::natural::Natural;
use rust_wheels::iterators::bools::exhaustive_bools;
use rust_wheels::iterators::common::EXAMPLE_SEED;
use rust_wheels::iterators::general::random_x;
use rust_wheels::iterators::integers_geometric::{i32s_geometric, u32s_geometric};
use rust_wheels::iterators::naturals::{exhaustive_naturals, exhaustive_positive_naturals,
                                       random_naturals, random_positive_naturals};
use rust_wheels::iterators::primitive_ints::{exhaustive_i, exhaustive_u};
use rust_wheels::iterators::rounding_modes::{exhaustive_rounding_modes, random_rounding_modes};
use rust_wheels::iterators::tuples::{exhaustive_pairs, exhaustive_pairs_from_single,
                                     exhaustive_triples, exhaustive_triples_from_single,
                                     lex_pairs, log_pairs, random_pairs, random_pairs_from_single,
                                     random_triples, random_triples_from_single};

pub fn naturals(gm: GenerationMode) -> Box<Iterator<Item = Natural>> {
    match gm {
        GenerationMode::Exhaustive => Box::new(exhaustive_naturals()),
        GenerationMode::Random(scale) => Box::new(random_naturals(&EXAMPLE_SEED, scale)),
    }
}

pub fn positive_naturals(gm: GenerationMode) -> Box<Iterator<Item = Natural>> {
    match gm {
        GenerationMode::Exhaustive => Box::new(exhaustive_positive_naturals()),
        GenerationMode::Random(scale) => Box::new(random_positive_naturals(&EXAMPLE_SEED, scale)),
    }
}

pub fn pairs_of_naturals(gm: GenerationMode) -> Box<Iterator<Item = (Natural, Natural)>> {
    match gm {
        GenerationMode::Exhaustive => Box::new(exhaustive_pairs_from_single(exhaustive_naturals())),
        GenerationMode::Random(scale) => Box::new(random_pairs_from_single(random_naturals(
            &EXAMPLE_SEED,
            scale,
        ))),
    }
}

//TODO use subset_pairs
pub fn pairs_of_naturals_var_1(gm: GenerationMode) -> Box<Iterator<Item = (Natural, Natural)>> {
    Box::new(pairs_of_naturals(gm).filter(|&(ref x, ref y)| x >= y))
}

pub fn triples_of_naturals(
    gm: GenerationMode,
) -> Box<Iterator<Item = (Natural, Natural, Natural)>> {
    match gm {
        GenerationMode::Exhaustive => {
            Box::new(exhaustive_triples_from_single(exhaustive_naturals()))
        }
        GenerationMode::Random(scale) => Box::new(random_triples_from_single(random_naturals(
            &EXAMPLE_SEED,
            scale,
        ))),
    }
}

pub fn triples_of_naturals_var_1(
    gm: GenerationMode,
) -> Box<Iterator<Item = (Natural, Natural, Natural)>> {
    Box::new(triples_of_naturals(gm).filter(|&(ref a, ref b, ref c)| a >= &(b * c)))
}

fn random_pairs_of_natural_and_primitive<T: 'static + PrimitiveInteger>(
    scale: u32,
) -> Box<Iterator<Item = (Natural, T)>> {
    Box::new(random_pairs(
        &EXAMPLE_SEED,
        &(|seed| random_naturals(seed, scale)),
        &(|seed| random_x(seed)),
    ))
}

fn random_pairs_of_primitive_and_natural<T: 'static + PrimitiveInteger>(
    scale: u32,
) -> Box<Iterator<Item = (T, Natural)>> {
    Box::new(random_pairs(
        &EXAMPLE_SEED,
        &(|seed| random_x(seed)),
        &(|seed| random_naturals(seed, scale)),
    ))
}

pub fn pairs_of_natural_and_unsigned<T: 'static + PrimitiveUnsigned>(
    gm: GenerationMode,
) -> Box<Iterator<Item = (Natural, T)>> {
    match gm {
        GenerationMode::Exhaustive => {
            Box::new(exhaustive_pairs(exhaustive_naturals(), exhaustive_u()))
        }
        GenerationMode::Random(scale) => random_pairs_of_natural_and_primitive(scale),
    }
}

pub fn pairs_of_natural_and_u32_var_1(gm: GenerationMode) -> Box<Iterator<Item = (Natural, u32)>> {
    Box::new(pairs_of_natural_and_unsigned(gm).filter(|&(ref n, u)| *n >= u))
}

pub fn pairs_of_unsigned_and_natural<T: 'static + PrimitiveUnsigned>(
    gm: GenerationMode,
) -> Box<Iterator<Item = (T, Natural)>> {
    match gm {
        GenerationMode::Exhaustive => {
            Box::new(exhaustive_pairs(exhaustive_u(), exhaustive_naturals()))
        }
        GenerationMode::Random(scale) => random_pairs_of_primitive_and_natural(scale),
    }
}

fn random_triples_of_natural_natural_and_primitive<T: 'static + PrimitiveInteger>(
    scale: u32,
) -> Box<Iterator<Item = (Natural, Natural, T)>> {
    Box::new(random_triples(
        &EXAMPLE_SEED,
        &(|seed| random_naturals(seed, scale)),
        &(|seed| random_naturals(seed, scale)),
        &(|seed| random_x(seed)),
    ))
}

pub fn triples_of_natural_natural_and_unsigned<T: 'static + PrimitiveUnsigned>(
    gm: GenerationMode,
) -> Box<Iterator<Item = (Natural, Natural, T)>> {
    match gm {
        GenerationMode::Exhaustive => Box::new(exhaustive_triples(
            exhaustive_naturals(),
            exhaustive_naturals(),
            exhaustive_u(),
        )),
        GenerationMode::Random(scale) => random_triples_of_natural_natural_and_primitive(scale),
    }
}

pub fn triples_of_natural_natural_and_u32_var_1(
    gm: GenerationMode,
) -> Box<Iterator<Item = (Natural, Natural, u32)>> {
    Box::new(triples_of_natural_natural_and_unsigned(gm).filter(|&(ref a, ref b, c)| a >= &(b * c)))
}

fn log_pairs_of_natural_and_unsigned<T: 'static + PrimitiveUnsigned>(
) -> Box<Iterator<Item = (Natural, T)>> {
    Box::new(log_pairs(exhaustive_naturals(), exhaustive_u()))
}

pub fn pairs_of_natural_and_small_u32(gm: GenerationMode) -> Box<Iterator<Item = (Natural, u32)>> {
    match gm {
        GenerationMode::Exhaustive => log_pairs_of_natural_and_unsigned(),
        GenerationMode::Random(scale) => Box::new(random_pairs(
            &EXAMPLE_SEED,
            &(|seed| random_naturals(seed, scale)),
            &(|seed| u32s_geometric(seed, scale)),
        )),
    }
}

pub fn pairs_of_natural_and_small_u32_var_1(
    gm: GenerationMode,
) -> Box<Iterator<Item = (Natural, u32)>> {
    Box::new(pairs_of_natural_and_small_u32(gm).map(|(n, u)| (n << u, u)))
}

pub fn pairs_of_natural_and_small_u32_var_2(
    gm: GenerationMode,
) -> Box<Iterator<Item = (Natural, u32)>> {
    Box::new(
        pairs_of_natural_and_small_u32(gm).filter(|&(ref n, u)| !n.divisible_by_power_of_two(u)),
    )
}

fn log_pairs_of_natural_and_signed<T: 'static + PrimitiveSigned>(
) -> Box<Iterator<Item = (Natural, T)>> {
    Box::new(log_pairs(exhaustive_naturals(), exhaustive_i()))
}

pub fn pairs_of_natural_and_small_i32(gm: GenerationMode) -> Box<Iterator<Item = (Natural, i32)>> {
    match gm {
        GenerationMode::Exhaustive => log_pairs_of_natural_and_signed(),
        GenerationMode::Random(scale) => Box::new(random_pairs(
            &EXAMPLE_SEED,
            &(|seed| random_naturals(seed, scale)),
            &(|seed| i32s_geometric(seed, scale)),
        )),
    }
}

pub fn pairs_of_natural_and_small_u64(gm: GenerationMode) -> Box<Iterator<Item = (Natural, u64)>> {
    match gm {
        GenerationMode::Exhaustive => log_pairs_of_natural_and_unsigned(),
        GenerationMode::Random(scale) => Box::new(random_pairs(
            &EXAMPLE_SEED,
            &(|seed| random_naturals(seed, scale)),
            &(|seed| u32s_geometric(seed, scale).map(|i| i.into())),
        )),
    }
}

pub fn triples_of_natural_small_u32_and_small_u32(
    gm: GenerationMode,
) -> Box<Iterator<Item = (Natural, u32, u32)>> {
    match gm {
        GenerationMode::Exhaustive => reshape_1_2_to_3(Box::new(log_pairs(
            exhaustive_naturals(),
            exhaustive_pairs_from_single(exhaustive_u()),
        ))),
        GenerationMode::Random(scale) => Box::new(random_triples(
            &EXAMPLE_SEED,
            &(|seed| random_naturals(seed, scale)),
            &(|seed| u32s_geometric(seed, scale)),
            &(|seed| u32s_geometric(seed, scale)),
        )),
    }
}

fn random_triples_of_natural_primitive_and_natural<T: 'static + PrimitiveInteger>(
    scale: u32,
) -> Box<Iterator<Item = (Natural, T, Natural)>> {
    Box::new(random_triples(
        &EXAMPLE_SEED,
        &(|seed| random_naturals(seed, scale)),
        &(|seed| random_x(seed)),
        &(|seed| random_naturals(seed, scale)),
    ))
}

fn random_triples_of_primitive_natural_and_primitive<T: 'static + PrimitiveInteger>(
    scale: u32,
) -> Box<Iterator<Item = (T, Natural, T)>> {
    Box::new(random_triples(
        &EXAMPLE_SEED,
        &(|seed| random_x(seed)),
        &(|seed| random_naturals(seed, scale)),
        &(|seed| random_x(seed)),
    ))
}

pub fn triples_of_natural_unsigned_and_natural<T: 'static + PrimitiveUnsigned>(
    gm: GenerationMode,
) -> Box<Iterator<Item = (Natural, T, Natural)>> {
    match gm {
        GenerationMode::Exhaustive => Box::new(exhaustive_triples(
            exhaustive_naturals(),
            exhaustive_u(),
            exhaustive_naturals(),
        )),
        GenerationMode::Random(scale) => random_triples_of_natural_primitive_and_natural(scale),
    }
}

pub fn triples_of_unsigned_natural_and_unsigned<T: 'static + PrimitiveUnsigned>(
    gm: GenerationMode,
) -> Box<Iterator<Item = (T, Natural, T)>> {
    match gm {
        GenerationMode::Exhaustive => Box::new(exhaustive_triples(
            exhaustive_u(),
            exhaustive_naturals(),
            exhaustive_u(),
        )),
        GenerationMode::Random(scale) => random_triples_of_primitive_natural_and_primitive(scale),
    }
}

pub fn triples_of_natural_small_u64_and_bool(
    gm: GenerationMode,
) -> Box<Iterator<Item = (Natural, u64, bool)>> {
    match gm {
        GenerationMode::Exhaustive => reshape_2_1_to_3(Box::new(lex_pairs(
            exhaustive_pairs(exhaustive_naturals(), exhaustive_u()),
            exhaustive_bools(),
        ))),
        GenerationMode::Random(scale) => Box::new(random_triples(
            &EXAMPLE_SEED,
            &(|seed| random_naturals(seed, scale)),
            &(|seed| u32s_geometric(seed, scale).map(|i| i.into())),
            &(|seed| random_x(seed)),
        )),
    }
}

pub fn pairs_of_natural_and_rounding_mode(
    gm: GenerationMode,
) -> Box<Iterator<Item = (Natural, RoundingMode)>> {
    match gm {
        GenerationMode::Exhaustive => Box::new(lex_pairs(
            exhaustive_naturals(),
            exhaustive_rounding_modes(),
        )),
        GenerationMode::Random(scale) => Box::new(random_pairs(
            &EXAMPLE_SEED,
            &(|seed| random_naturals(seed, scale)),
            &(|seed| random_rounding_modes(seed)),
        )),
    }
}

fn triples_of_natural_small_i32_and_rounding_mode(
    gm: GenerationMode,
) -> Box<Iterator<Item = (Natural, i32, RoundingMode)>> {
    match gm {
        GenerationMode::Exhaustive => reshape_2_1_to_3(Box::new(lex_pairs(
            log_pairs_of_natural_and_signed(),
            exhaustive_rounding_modes(),
        ))),
        GenerationMode::Random(scale) => Box::new(random_triples(
            &EXAMPLE_SEED,
            &(|seed| random_naturals(seed, scale)),
            &(|seed| i32s_geometric(seed, scale)),
            &(|seed| random_rounding_modes(seed)),
        )),
    }
}

pub fn triples_of_natural_small_i32_and_rounding_mode_var_1(
    gm: GenerationMode,
) -> Box<Iterator<Item = (Natural, i32, RoundingMode)>> {
    Box::new(
        triples_of_natural_small_i32_and_rounding_mode(gm).filter(|&(ref n, i, rm)| {
            i >= 0 || rm != RoundingMode::Exact
                || n.divisible_by_power_of_two(i.wrapping_neg() as u32)
        }),
    )
}

pub fn triples_of_natural_small_i32_and_rounding_mode_var_2(
    gm: GenerationMode,
) -> Box<Iterator<Item = (Natural, i32, RoundingMode)>> {
    Box::new(
        triples_of_natural_small_i32_and_rounding_mode(gm).filter(|&(ref n, i, rm)| {
            i <= 0 || rm != RoundingMode::Exact || n.divisible_by_power_of_two(i as u32)
        }),
    )
}

fn triples_of_natural_small_u32_and_rounding_mode(
    gm: GenerationMode,
) -> Box<Iterator<Item = (Natural, u32, RoundingMode)>> {
    match gm {
        GenerationMode::Exhaustive => reshape_2_1_to_3(Box::new(lex_pairs(
            log_pairs_of_natural_and_unsigned(),
            exhaustive_rounding_modes(),
        ))),
        GenerationMode::Random(scale) => Box::new(random_triples(
            &EXAMPLE_SEED,
            &(|seed| random_naturals(seed, scale)),
            &(|seed| u32s_geometric(seed, scale)),
            &(|seed| random_rounding_modes(seed)),
        )),
    }
}

pub fn triples_of_natural_small_u32_and_rounding_mode_var_1(
    gm: GenerationMode,
) -> Box<Iterator<Item = (Natural, u32, RoundingMode)>> {
    Box::new(
        triples_of_natural_small_u32_and_rounding_mode(gm)
            .filter(|&(ref n, u, rm)| rm != RoundingMode::Exact || n.divisible_by_power_of_two(u)),
    )
}
