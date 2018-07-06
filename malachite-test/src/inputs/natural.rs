use common::{natural_to_biguint, natural_to_rug_integer, GenerationMode};
use inputs::base::pairs_of_unsigneds;
use inputs::common::{reshape_1_2_to_3, reshape_2_1_to_3};
use malachite_base::num::{PrimitiveInteger, PrimitiveSigned, PrimitiveUnsigned, SignificantBits};
use malachite_base::round::RoundingMode;
use malachite_nz::natural::Natural;
use num::BigUint;
use rand::{IsaacRng, Rng, SeedableRng};
use rug;
use rust_wheels::iterators::bools::exhaustive_bools;
use rust_wheels::iterators::common::{scramble, EXAMPLE_SEED};
use rust_wheels::iterators::dependent_pairs::dependent_pairs;
use rust_wheels::iterators::general::random;
use rust_wheels::iterators::integers_geometric::{i32s_geometric, u32s_geometric};
use rust_wheels::iterators::naturals::{
    exhaustive_naturals, exhaustive_positive_naturals, random_naturals, random_positive_naturals,
    special_random_naturals, special_random_positive_naturals,
};
use rust_wheels::iterators::primitive_ints::{
    exhaustive_signed, exhaustive_unsigned, special_random_unsigned,
};
use rust_wheels::iterators::rounding_modes::{exhaustive_rounding_modes, random_rounding_modes};
use rust_wheels::iterators::tuples::{
    exhaustive_pairs, exhaustive_pairs_from_single, exhaustive_triples,
    exhaustive_triples_from_single, lex_pairs, log_pairs, random_pairs, random_pairs_from_single,
    random_triples, random_triples_from_single,
};
use rust_wheels::iterators::vecs::exhaustive_fixed_size_vecs_from_single;

pub fn naturals(gm: GenerationMode) -> Box<Iterator<Item = Natural>> {
    match gm {
        GenerationMode::Exhaustive => Box::new(exhaustive_naturals()),
        GenerationMode::Random(scale) => Box::new(random_naturals(&EXAMPLE_SEED, scale)),
        GenerationMode::SpecialRandom(scale) => {
            Box::new(special_random_naturals(&EXAMPLE_SEED, scale))
        }
    }
}

pub fn rm_naturals(gm: GenerationMode) -> Box<Iterator<Item = (rug::Integer, Natural)>> {
    Box::new(naturals(gm).map(|n| (natural_to_rug_integer(&n), n)))
}

pub fn nrm_naturals(gm: GenerationMode) -> Box<Iterator<Item = (BigUint, rug::Integer, Natural)>> {
    Box::new(naturals(gm).map(|n| (natural_to_biguint(&n), natural_to_rug_integer(&n), n)))
}

pub fn positive_naturals(gm: GenerationMode) -> Box<Iterator<Item = Natural>> {
    match gm {
        GenerationMode::Exhaustive => Box::new(exhaustive_positive_naturals()),
        GenerationMode::Random(scale) => Box::new(random_positive_naturals(&EXAMPLE_SEED, scale)),
        GenerationMode::SpecialRandom(scale) => {
            Box::new(special_random_positive_naturals(&EXAMPLE_SEED, scale))
        }
    }
}

pub fn pairs_of_naturals(gm: GenerationMode) -> Box<Iterator<Item = (Natural, Natural)>> {
    match gm {
        GenerationMode::Exhaustive => Box::new(exhaustive_pairs_from_single(exhaustive_naturals())),
        GenerationMode::Random(scale) => Box::new(random_pairs_from_single(random_naturals(
            &EXAMPLE_SEED,
            scale,
        ))),
        GenerationMode::SpecialRandom(scale) => Box::new(random_pairs_from_single(
            special_random_naturals(&EXAMPLE_SEED, scale),
        )),
    }
}

pub fn nrm_pairs_of_naturals(
    gm: GenerationMode,
) -> Box<
    Iterator<
        Item = (
            (BigUint, BigUint),
            (rug::Integer, rug::Integer),
            (Natural, Natural),
        ),
    >,
> {
    Box::new(pairs_of_naturals(gm).map(|(x, y)| {
        (
            (natural_to_biguint(&x), natural_to_biguint(&y)),
            (natural_to_rug_integer(&x), natural_to_rug_integer(&y)),
            (x, y),
        )
    }))
}

pub fn rm_pairs_of_naturals(
    gm: GenerationMode,
) -> Box<Iterator<Item = ((rug::Integer, rug::Integer), (Natural, Natural))>> {
    Box::new(pairs_of_naturals(gm).map(|(x, y)| {
        (
            (natural_to_rug_integer(&x), natural_to_rug_integer(&y)),
            (x, y),
        )
    }))
}

//TODO use subset_pairs
// All pairs of `Natural`s where the first is greater than or equal to the second.
pub fn pairs_of_naturals_var_1(gm: GenerationMode) -> Box<Iterator<Item = (Natural, Natural)>> {
    Box::new(pairs_of_naturals(gm).filter(|&(ref x, ref y)| x >= y))
}

//TODO use subset_pairs
pub fn rm_pairs_of_naturals_var_1(
    gm: GenerationMode,
) -> Box<Iterator<Item = ((rug::Integer, rug::Integer), (Natural, Natural))>> {
    Box::new(pairs_of_naturals_var_1(gm).map(|(x, y)| {
        (
            (natural_to_rug_integer(&x), natural_to_rug_integer(&y)),
            (x, y),
        )
    }))
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
        GenerationMode::SpecialRandom(scale) => Box::new(random_triples_from_single(
            special_random_naturals(&EXAMPLE_SEED, scale),
        )),
    }
}

// All triples of `Natural`s where the first is greater than or equal to the product of the second
// and third.
#[allow(op_ref)]
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
        &(|seed| random(seed)),
    ))
}

fn random_pairs_of_primitive_and_natural<T: 'static + PrimitiveInteger>(
    scale: u32,
) -> Box<Iterator<Item = (T, Natural)>> {
    Box::new(random_pairs(
        &EXAMPLE_SEED,
        &(|seed| random(seed)),
        &(|seed| random_naturals(seed, scale)),
    ))
}

pub fn pairs_of_natural_and_unsigned<T: 'static + PrimitiveUnsigned>(
    gm: GenerationMode,
) -> Box<Iterator<Item = (Natural, T)>> {
    match gm {
        GenerationMode::Exhaustive => Box::new(exhaustive_pairs(
            exhaustive_naturals(),
            exhaustive_unsigned(),
        )),
        GenerationMode::Random(scale) => random_pairs_of_natural_and_primitive(scale),
        GenerationMode::SpecialRandom(scale) => Box::new(random_pairs(
            &EXAMPLE_SEED,
            &(|seed| special_random_naturals(seed, scale)),
            &(|seed| special_random_unsigned(seed)),
        )),
    }
}

pub fn rm_pairs_of_natural_and_unsigned<T: 'static + PrimitiveUnsigned>(
    gm: GenerationMode,
) -> Box<Iterator<Item = ((rug::Integer, T), (Natural, T))>> {
    Box::new(
        pairs_of_natural_and_unsigned(gm).map(|(x, y)| ((natural_to_rug_integer(&x), y), (x, y))),
    )
}

pub fn nm_pairs_of_natural_and_unsigned<T: 'static + PrimitiveUnsigned>(
    gm: GenerationMode,
) -> Box<Iterator<Item = ((BigUint, T), (Natural, T))>> {
    Box::new(pairs_of_natural_and_unsigned(gm).map(|(x, y)| ((natural_to_biguint(&x), y), (x, y))))
}

pub fn nrm_pairs_of_natural_and_unsigned<T: 'static + PrimitiveUnsigned>(
    gm: GenerationMode,
) -> Box<Iterator<Item = ((BigUint, T), (rug::Integer, T), (Natural, T))>> {
    Box::new(pairs_of_natural_and_unsigned(gm).map(|(x, y)| {
        (
            (natural_to_biguint(&x), y),
            (natural_to_rug_integer(&x), y),
            (x, y),
        )
    }))
}

// All pairs of `Natural` and `u32` where the `Natural` is greater than or equal to the `u32`.
pub fn pairs_of_natural_and_u32_var_1(gm: GenerationMode) -> Box<Iterator<Item = (Natural, u32)>> {
    Box::new(pairs_of_natural_and_unsigned(gm).filter(|&(ref n, u)| *n >= u))
}

pub fn rm_pairs_of_natural_and_u32_var_1(
    gm: GenerationMode,
) -> Box<Iterator<Item = ((rug::Integer, u32), (Natural, u32))>> {
    Box::new(
        pairs_of_natural_and_u32_var_1(gm).map(|(x, y)| ((natural_to_rug_integer(&x), y), (x, y))),
    )
}

pub fn nrm_pairs_of_natural_and_u32_var_1(
    gm: GenerationMode,
) -> Box<Iterator<Item = ((BigUint, u32), (rug::Integer, u32), (Natural, u32))>> {
    Box::new(pairs_of_natural_and_u32_var_1(gm).map(|(x, y)| {
        (
            (natural_to_biguint(&x), y),
            (natural_to_rug_integer(&x), y),
            (x, y),
        )
    }))
}

pub fn pairs_of_unsigned_and_natural<T: 'static + PrimitiveUnsigned>(
    gm: GenerationMode,
) -> Box<Iterator<Item = (T, Natural)>> {
    match gm {
        GenerationMode::Exhaustive => Box::new(exhaustive_pairs(
            exhaustive_unsigned(),
            exhaustive_naturals(),
        )),
        GenerationMode::Random(scale) => random_pairs_of_primitive_and_natural(scale),
        GenerationMode::SpecialRandom(scale) => Box::new(random_pairs(
            &EXAMPLE_SEED,
            &(|seed| special_random_unsigned(seed)),
            &(|seed| special_random_naturals(seed, scale)),
        )),
    }
}

pub fn rm_pairs_of_unsigned_and_natural<T: 'static + PrimitiveUnsigned>(
    gm: GenerationMode,
) -> Box<Iterator<Item = ((T, rug::Integer), (T, Natural))>> {
    Box::new(
        pairs_of_unsigned_and_natural(gm).map(|(x, y)| ((x, natural_to_rug_integer(&y)), (x, y))),
    )
}

// All pairs of `u32` and `Natural` where the `u32` is greater than or equal to the `Natural`.
pub fn pairs_of_u32_and_natural_var_1(gm: GenerationMode) -> Box<Iterator<Item = (u32, Natural)>> {
    Box::new(
        pairs_of_unsigneds(gm)
            .filter(|(x, y)| x >= y)
            .map(|(x, y)| (x, Natural::from(y))),
    )
}

pub fn rm_pairs_of_u32_and_natural_var_1(
    gm: GenerationMode,
) -> Box<Iterator<Item = ((u32, rug::Integer), (u32, Natural))>> {
    Box::new(
        pairs_of_u32_and_natural_var_1(gm).map(|(x, y)| ((x, natural_to_rug_integer(&y)), (x, y))),
    )
}

fn random_triples_of_natural_natural_and_primitive<T: 'static + PrimitiveInteger>(
    scale: u32,
) -> Box<Iterator<Item = (Natural, Natural, T)>> {
    Box::new(random_triples(
        &EXAMPLE_SEED,
        &(|seed| random_naturals(seed, scale)),
        &(|seed| random_naturals(seed, scale)),
        &(|seed| random(seed)),
    ))
}

pub fn triples_of_natural_natural_and_unsigned<T: 'static + PrimitiveUnsigned>(
    gm: GenerationMode,
) -> Box<Iterator<Item = (Natural, Natural, T)>> {
    match gm {
        GenerationMode::Exhaustive => Box::new(exhaustive_triples(
            exhaustive_naturals(),
            exhaustive_naturals(),
            exhaustive_unsigned(),
        )),
        GenerationMode::Random(scale) => random_triples_of_natural_natural_and_primitive(scale),
        GenerationMode::SpecialRandom(scale) => Box::new(random_triples(
            &EXAMPLE_SEED,
            &(|seed| special_random_naturals(seed, scale)),
            &(|seed| special_random_naturals(seed, scale)),
            &(|seed| special_random_unsigned(seed)),
        )),
    }
}

// All triples of `Natural`, `Natural`, and `u32`, where the first `Natural` is greater than or
// equal to the product of the second `Natural` and the `u32`.
#[allow(op_ref)]
pub fn triples_of_natural_natural_and_u32_var_1(
    gm: GenerationMode,
) -> Box<Iterator<Item = (Natural, Natural, u32)>> {
    Box::new(triples_of_natural_natural_and_unsigned(gm).filter(|&(ref a, ref b, c)| a >= &(b * c)))
}

fn log_pairs_of_natural_and_unsigned<T: 'static + PrimitiveUnsigned>(
) -> Box<Iterator<Item = (Natural, T)>> {
    Box::new(log_pairs(exhaustive_naturals(), exhaustive_unsigned()))
}

pub fn pairs_of_natural_and_small_u32(gm: GenerationMode) -> Box<Iterator<Item = (Natural, u32)>> {
    match gm {
        GenerationMode::Exhaustive => log_pairs_of_natural_and_unsigned(),
        GenerationMode::Random(scale) => Box::new(random_pairs(
            &EXAMPLE_SEED,
            &(|seed| random_naturals(seed, scale)),
            &(|seed| u32s_geometric(seed, scale)),
        )),
        GenerationMode::SpecialRandom(scale) => Box::new(random_pairs(
            &EXAMPLE_SEED,
            &(|seed| special_random_naturals(seed, scale)),
            &(|seed| u32s_geometric(seed, scale)),
        )),
    }
}

pub fn rm_pairs_of_natural_and_small_u32(
    gm: GenerationMode,
) -> Box<Iterator<Item = ((rug::Integer, u32), (Natural, u32))>> {
    Box::new(
        pairs_of_natural_and_small_u32(gm).map(|(x, y)| ((natural_to_rug_integer(&x), y), (x, y))),
    )
}

// All pairs of `Natural` and `u32` where the `Natural` is divisible by 2 to the power of the `u32`.
pub fn pairs_of_natural_and_small_u32_var_1(
    gm: GenerationMode,
) -> Box<Iterator<Item = (Natural, u32)>> {
    Box::new(pairs_of_natural_and_small_u32(gm).map(|(n, u)| (n << u, u)))
}

// All pairs of `Natural` and `u32` where the `Natural` is not divisible by 2 to the power of the
// `u32`.
pub fn pairs_of_natural_and_small_u32_var_2(
    gm: GenerationMode,
) -> Box<Iterator<Item = (Natural, u32)>> {
    Box::new(
        pairs_of_natural_and_small_u32(gm).filter(|&(ref n, u)| !n.divisible_by_power_of_two(u)),
    )
}

fn log_pairs_of_natural_and_signed<T: 'static + PrimitiveSigned>(
) -> Box<Iterator<Item = (Natural, T)>> {
    Box::new(log_pairs(exhaustive_naturals(), exhaustive_signed()))
}

pub fn pairs_of_natural_and_small_i32(gm: GenerationMode) -> Box<Iterator<Item = (Natural, i32)>> {
    match gm {
        GenerationMode::Exhaustive => log_pairs_of_natural_and_signed(),
        GenerationMode::Random(scale) => Box::new(random_pairs(
            &EXAMPLE_SEED,
            &(|seed| random_naturals(seed, scale)),
            &(|seed| i32s_geometric(seed, scale)),
        )),
        GenerationMode::SpecialRandom(scale) => Box::new(random_pairs(
            &EXAMPLE_SEED,
            &(|seed| special_random_naturals(seed, scale)),
            &(|seed| i32s_geometric(seed, scale)),
        )),
    }
}

pub fn rm_pairs_of_natural_and_small_i32(
    gm: GenerationMode,
) -> Box<Iterator<Item = ((rug::Integer, i32), (Natural, i32))>> {
    Box::new(
        pairs_of_natural_and_small_i32(gm).map(|(x, y)| ((natural_to_rug_integer(&x), y), (x, y))),
    )
}

pub fn pairs_of_natural_and_small_u64(gm: GenerationMode) -> Box<Iterator<Item = (Natural, u64)>> {
    match gm {
        GenerationMode::Exhaustive => log_pairs_of_natural_and_unsigned(),
        GenerationMode::Random(scale) => Box::new(random_pairs(
            &EXAMPLE_SEED,
            &(|seed| random_naturals(seed, scale)),
            &(|seed| u32s_geometric(seed, scale).map(|i| i.into())),
        )),
        GenerationMode::SpecialRandom(scale) => Box::new(random_pairs(
            &EXAMPLE_SEED,
            &(|seed| special_random_naturals(seed, scale)),
            &(|seed| u32s_geometric(seed, scale).map(|i| i.into())),
        )),
    }
}

pub fn rm_pairs_of_natural_and_small_u64(
    gm: GenerationMode,
) -> Box<Iterator<Item = ((rug::Integer, u64), (Natural, u64))>> {
    Box::new(
        pairs_of_natural_and_small_u64(gm).map(|(x, y)| ((natural_to_rug_integer(&x), y), (x, y))),
    )
}

pub fn nm_pairs_of_natural_and_small_u64(
    gm: GenerationMode,
) -> Box<Iterator<Item = ((BigUint, u64), (Natural, u64))>> {
    Box::new(pairs_of_natural_and_small_u64(gm).map(|(x, y)| ((natural_to_biguint(&x), y), (x, y))))
}

pub fn nrm_pairs_of_natural_and_small_u64(
    gm: GenerationMode,
) -> Box<Iterator<Item = ((BigUint, u64), (rug::Integer, u64), (Natural, u64))>> {
    Box::new(pairs_of_natural_and_small_u64(gm).map(|(x, y)| {
        (
            (natural_to_biguint(&x), y),
            (natural_to_rug_integer(&x), y),
            (x, y),
        )
    }))
}

pub fn pairs_of_natural_and_small_usize(
    gm: GenerationMode,
) -> Box<Iterator<Item = (Natural, usize)>> {
    Box::new(pairs_of_natural_and_small_u32(gm).map(|(n, u)| (n, u as usize)))
}

pub fn triples_of_natural_small_u32_and_small_u32(
    gm: GenerationMode,
) -> Box<Iterator<Item = (Natural, u32, u32)>> {
    match gm {
        GenerationMode::Exhaustive => reshape_1_2_to_3(Box::new(log_pairs(
            exhaustive_naturals(),
            exhaustive_pairs_from_single(exhaustive_unsigned()),
        ))),
        GenerationMode::Random(scale) => Box::new(random_triples(
            &EXAMPLE_SEED,
            &(|seed| random_naturals(seed, scale)),
            &(|seed| u32s_geometric(seed, scale)),
            &(|seed| u32s_geometric(seed, scale)),
        )),
        GenerationMode::SpecialRandom(scale) => Box::new(random_triples(
            &EXAMPLE_SEED,
            &(|seed| special_random_naturals(seed, scale)),
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
        &(|seed| random(seed)),
        &(|seed| random_naturals(seed, scale)),
    ))
}

fn random_triples_of_primitive_natural_and_primitive<T: 'static + PrimitiveInteger>(
    scale: u32,
) -> Box<Iterator<Item = (T, Natural, T)>> {
    Box::new(random_triples(
        &EXAMPLE_SEED,
        &(|seed| random(seed)),
        &(|seed| random_naturals(seed, scale)),
        &(|seed| random(seed)),
    ))
}

fn random_triples_of_natural_primitive_and_primitive<T: 'static + PrimitiveInteger>(
    scale: u32,
) -> Box<Iterator<Item = (Natural, T, T)>> {
    Box::new(random_triples(
        &EXAMPLE_SEED,
        &(|seed| random_naturals(seed, scale)),
        &(|seed| random(seed)),
        &(|seed| random(seed)),
    ))
}

pub fn triples_of_natural_unsigned_and_natural<T: 'static + PrimitiveUnsigned>(
    gm: GenerationMode,
) -> Box<Iterator<Item = (Natural, T, Natural)>> {
    match gm {
        GenerationMode::Exhaustive => Box::new(exhaustive_triples(
            exhaustive_naturals(),
            exhaustive_unsigned(),
            exhaustive_naturals(),
        )),
        GenerationMode::Random(scale) => random_triples_of_natural_primitive_and_natural(scale),
        GenerationMode::SpecialRandom(scale) => Box::new(random_triples(
            &EXAMPLE_SEED,
            &(|seed| special_random_naturals(seed, scale)),
            &(|seed| special_random_unsigned(seed)),
            &(|seed| special_random_naturals(seed, scale)),
        )),
    }
}

pub fn triples_of_unsigned_natural_and_unsigned<T: 'static + PrimitiveUnsigned>(
    gm: GenerationMode,
) -> Box<Iterator<Item = (T, Natural, T)>> {
    match gm {
        GenerationMode::Exhaustive => Box::new(exhaustive_triples(
            exhaustive_unsigned(),
            exhaustive_naturals(),
            exhaustive_unsigned(),
        )),
        GenerationMode::Random(scale) => random_triples_of_primitive_natural_and_primitive(scale),
        GenerationMode::SpecialRandom(scale) => Box::new(random_triples(
            &EXAMPLE_SEED,
            &(|seed| special_random_unsigned(seed)),
            &(|seed| special_random_naturals(seed, scale)),
            &(|seed| special_random_unsigned(seed)),
        )),
    }
}

pub fn triples_of_natural_unsigned_and_unsigned<T: 'static + PrimitiveUnsigned>(
    gm: GenerationMode,
) -> Box<Iterator<Item = (Natural, T, T)>> {
    match gm {
        GenerationMode::Exhaustive => Box::new(exhaustive_triples(
            exhaustive_naturals(),
            exhaustive_unsigned(),
            exhaustive_unsigned(),
        )),
        GenerationMode::Random(scale) => random_triples_of_natural_primitive_and_primitive(scale),
        GenerationMode::SpecialRandom(scale) => Box::new(random_triples(
            &EXAMPLE_SEED,
            &(|seed| special_random_naturals(seed, scale)),
            &(|seed| special_random_unsigned(seed)),
            &(|seed| special_random_unsigned(seed)),
        )),
    }
}

pub fn triples_of_natural_small_u64_and_bool(
    gm: GenerationMode,
) -> Box<Iterator<Item = (Natural, u64, bool)>> {
    match gm {
        GenerationMode::Exhaustive => reshape_2_1_to_3(Box::new(lex_pairs(
            exhaustive_pairs(exhaustive_naturals(), exhaustive_unsigned()),
            exhaustive_bools(),
        ))),
        GenerationMode::Random(scale) => Box::new(random_triples(
            &EXAMPLE_SEED,
            &(|seed| random_naturals(seed, scale)),
            &(|seed| u32s_geometric(seed, scale).map(|i| i.into())),
            &(|seed| random(seed)),
        )),
        GenerationMode::SpecialRandom(scale) => Box::new(random_triples(
            &EXAMPLE_SEED,
            &(|seed| special_random_naturals(seed, scale)),
            &(|seed| u32s_geometric(seed, scale).map(|i| i.into())),
            &(|seed| random(seed)),
        )),
    }
}

pub fn rm_triples_of_natural_small_u64_and_bool(
    gm: GenerationMode,
) -> Box<Iterator<Item = ((rug::Integer, u64, bool), (Natural, u64, bool))>> {
    Box::new(
        triples_of_natural_small_u64_and_bool(gm)
            .map(|(x, y, z)| ((natural_to_rug_integer(&x), y, z), (x, y, z))),
    )
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
        GenerationMode::SpecialRandom(scale) => Box::new(random_pairs(
            &EXAMPLE_SEED,
            &(|seed| special_random_naturals(seed, scale)),
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
        GenerationMode::SpecialRandom(scale) => Box::new(random_triples(
            &EXAMPLE_SEED,
            &(|seed| special_random_naturals(seed, scale)),
            &(|seed| i32s_geometric(seed, scale)),
            &(|seed| random_rounding_modes(seed)),
        )),
    }
}

// All triples of `Natural`, `i32`, and `RoundingMode`, such that if the `i32` is negative and the
// `RoundingMode` is `RoundingMode::Exact`, the `Natural` is divisible by 2 to the power of the
// negative of the `i32`.
pub fn triples_of_natural_small_i32_and_rounding_mode_var_1(
    gm: GenerationMode,
) -> Box<Iterator<Item = (Natural, i32, RoundingMode)>> {
    Box::new(
        triples_of_natural_small_i32_and_rounding_mode(gm).filter(|&(ref n, i, rm)| {
            i >= 0
                || rm != RoundingMode::Exact
                || n.divisible_by_power_of_two(i.wrapping_neg() as u32)
        }),
    )
}

// All triples of `Natural`, `i32`, and `RoundingMode`, such that if the `i32` is positive and the
// `RoundingMode` is `RoundingMode::Exact`, the `Natural` is divisible by 2 to the power of the
// `i32`.
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
        GenerationMode::SpecialRandom(scale) => Box::new(random_triples(
            &EXAMPLE_SEED,
            &(|seed| special_random_naturals(seed, scale)),
            &(|seed| u32s_geometric(seed, scale)),
            &(|seed| random_rounding_modes(seed)),
        )),
    }
}

// All triples of `Natural`, `u32`, and `RoundingMode`, where if the `RoundingMode` is
// `RoundingMode::Exact`, the `Natural` is divisible by 2 to the power of the `u32`.
pub fn triples_of_natural_small_u32_and_rounding_mode_var_1(
    gm: GenerationMode,
) -> Box<Iterator<Item = (Natural, u32, RoundingMode)>> {
    Box::new(
        triples_of_natural_small_u32_and_rounding_mode(gm)
            .filter(|&(ref n, u, rm)| rm != RoundingMode::Exact || n.divisible_by_power_of_two(u)),
    )
}

struct RandomNaturalAndVecOfBoolVar1 {
    naturals: Box<Iterator<Item = Natural>>,
    rng: Box<IsaacRng>,
}

impl Iterator for RandomNaturalAndVecOfBoolVar1 {
    type Item = (Natural, Vec<bool>);

    fn next(&mut self) -> Option<(Natural, Vec<bool>)> {
        let n = self.naturals.next().unwrap();
        let mut bools = Vec::new();
        for _ in 0..n.limb_count() {
            bools.push(self.rng.gen::<bool>());
        }
        Some((n, bools))
    }
}

fn random_pairs_of_natural_and_vec_of_bool_var_1(
    seed: &[u32],
    scale: u32,
) -> RandomNaturalAndVecOfBoolVar1 {
    RandomNaturalAndVecOfBoolVar1 {
        naturals: Box::new(random_naturals(&scramble(seed, "naturals"), scale)),
        rng: Box::new(IsaacRng::from_seed(&scramble(seed, "bools"))),
    }
}

fn special_random_pairs_of_natural_and_vec_of_bool_var_1(
    seed: &[u32],
    scale: u32,
) -> RandomNaturalAndVecOfBoolVar1 {
    RandomNaturalAndVecOfBoolVar1 {
        naturals: Box::new(special_random_naturals(&scramble(seed, "naturals"), scale)),
        rng: Box::new(IsaacRng::from_seed(&scramble(seed, "bools"))),
    }
}

// All pairs of `Natural` and `Vec<bool>` where the length of the `Vec` is equal to the limb count
// of the `Natural`.
pub fn pairs_of_natural_and_vec_of_bool_var_1(
    gm: GenerationMode,
) -> Box<Iterator<Item = (Natural, Vec<bool>)>> {
    match gm {
        GenerationMode::Exhaustive => {
            let f = move |n: &Natural| {
                exhaustive_fixed_size_vecs_from_single(n.limb_count(), exhaustive_bools())
            };
            Box::new(dependent_pairs(exhaustive_naturals(), f))
        }
        GenerationMode::Random(scale) => Box::new(random_pairs_of_natural_and_vec_of_bool_var_1(
            &EXAMPLE_SEED,
            scale,
        )),
        GenerationMode::SpecialRandom(scale) => Box::new(
            special_random_pairs_of_natural_and_vec_of_bool_var_1(&EXAMPLE_SEED, scale),
        ),
    }
}

// All pairs of positive `Natural` and `Vec<bool>` where the length of the `Vec` is equal to the
// limb count of the `Natural`.
pub fn pairs_of_positive_natural_and_vec_of_bool_var_1(
    gm: GenerationMode,
) -> Box<Iterator<Item = (Natural, Vec<bool>)>> {
    Box::new(pairs_of_natural_and_vec_of_bool_var_1(gm).filter(|&(ref n, _)| *n != 0))
}

struct RandomNaturalAndVecOfBoolVar2 {
    naturals: Box<Iterator<Item = Natural>>,
    rng: Box<IsaacRng>,
}

impl Iterator for RandomNaturalAndVecOfBoolVar2 {
    type Item = (Natural, Vec<bool>);

    fn next(&mut self) -> Option<(Natural, Vec<bool>)> {
        let n = self.naturals.next().unwrap();
        let mut bools = Vec::new();
        for _ in 0..n.significant_bits() {
            bools.push(self.rng.gen::<bool>());
        }
        Some((n, bools))
    }
}

fn random_pairs_of_natural_and_vec_of_bool_var_2(
    seed: &[u32],
    scale: u32,
) -> RandomNaturalAndVecOfBoolVar2 {
    RandomNaturalAndVecOfBoolVar2 {
        naturals: Box::new(random_naturals(&scramble(seed, "naturals"), scale)),
        rng: Box::new(IsaacRng::from_seed(&scramble(seed, "bools"))),
    }
}

fn special_random_pairs_of_natural_and_vec_of_bool_var_2(
    seed: &[u32],
    scale: u32,
) -> RandomNaturalAndVecOfBoolVar2 {
    RandomNaturalAndVecOfBoolVar2 {
        naturals: Box::new(special_random_naturals(&scramble(seed, "naturals"), scale)),
        rng: Box::new(IsaacRng::from_seed(&scramble(seed, "bools"))),
    }
}

// All pairs of `Natural` and `Vec<bool>` where the length of the `Vec` is equal to the significant
// bit count of the `Natural`.
pub fn pairs_of_natural_and_vec_of_bool_var_2(
    gm: GenerationMode,
) -> Box<Iterator<Item = (Natural, Vec<bool>)>> {
    match gm {
        GenerationMode::Exhaustive => {
            let f = move |n: &Natural| {
                exhaustive_fixed_size_vecs_from_single(n.significant_bits(), exhaustive_bools())
            };
            Box::new(dependent_pairs(exhaustive_naturals(), f))
        }
        GenerationMode::Random(scale) => Box::new(random_pairs_of_natural_and_vec_of_bool_var_2(
            &EXAMPLE_SEED,
            scale,
        )),
        GenerationMode::SpecialRandom(scale) => Box::new(
            special_random_pairs_of_natural_and_vec_of_bool_var_2(&EXAMPLE_SEED, scale),
        ),
    }
}
