use common::{integer_to_bigint, integer_to_rug_integer, natural_to_rug_integer, GenerationMode};
use inputs::common::{reshape_1_2_to_3, reshape_2_1_to_3};
use malachite_base::num::{PrimitiveInteger, PrimitiveSigned, PrimitiveUnsigned};
use malachite_base::round::RoundingMode;
use malachite_nz::integer::Integer;
use malachite_nz::natural::Natural;
use num::BigInt;
use rug;
use rust_wheels::iterators::bools::exhaustive_bools;
use rust_wheels::iterators::common::EXAMPLE_SEED;
use rust_wheels::iterators::general::random;
use rust_wheels::iterators::integers::{exhaustive_integers, exhaustive_natural_integers,
                                       random_integers, random_natural_integers,
                                       special_random_integers, special_random_natural_integers};
use rust_wheels::iterators::integers_geometric::{i32s_geometric, u32s_geometric};
use rust_wheels::iterators::naturals::{exhaustive_naturals, random_naturals,
                                       special_random_naturals};
use rust_wheels::iterators::primitive_ints::{exhaustive_signed, exhaustive_unsigned,
                                             special_random_signed, special_random_unsigned};
use rust_wheels::iterators::rounding_modes::{exhaustive_rounding_modes, random_rounding_modes};
use rust_wheels::iterators::tuples::{exhaustive_pairs, exhaustive_pairs_from_single,
                                     exhaustive_triples, exhaustive_triples_from_single,
                                     lex_pairs, log_pairs, random_pairs, random_pairs_from_single,
                                     random_triples, random_triples_from_single};

pub fn integers(gm: GenerationMode) -> Box<Iterator<Item = Integer>> {
    match gm {
        GenerationMode::Exhaustive => Box::new(exhaustive_integers()),
        GenerationMode::Random(scale) => Box::new(random_integers(&EXAMPLE_SEED, scale)),
        GenerationMode::SpecialRandom(scale) => {
            Box::new(special_random_integers(&EXAMPLE_SEED, scale))
        }
    }
}

pub fn nm_integers(gm: GenerationMode) -> Box<Iterator<Item = (BigInt, Integer)>> {
    Box::new(integers(gm).map(|n| (integer_to_bigint(&n), n)))
}

pub fn nrm_integers(gm: GenerationMode) -> Box<Iterator<Item = (BigInt, rug::Integer, Integer)>> {
    Box::new(integers(gm).map(|n| (integer_to_bigint(&n), integer_to_rug_integer(&n), n)))
}

pub fn pairs_of_integers(gm: GenerationMode) -> Box<Iterator<Item = (Integer, Integer)>> {
    match gm {
        GenerationMode::Exhaustive => Box::new(exhaustive_pairs_from_single(exhaustive_integers())),
        GenerationMode::Random(scale) => Box::new(random_pairs_from_single(random_integers(
            &EXAMPLE_SEED,
            scale,
        ))),
        GenerationMode::SpecialRandom(scale) => Box::new(random_pairs_from_single(
            special_random_integers(&EXAMPLE_SEED, scale),
        )),
    }
}

pub fn rm_pairs_of_integers(
    gm: GenerationMode,
) -> Box<Iterator<Item = ((rug::Integer, rug::Integer), (Integer, Integer))>> {
    Box::new(pairs_of_integers(gm).map(|(x, y)| {
        (
            (integer_to_rug_integer(&x), integer_to_rug_integer(&y)),
            (x, y),
        )
    }))
}

pub fn nrm_pairs_of_integers(
    gm: GenerationMode,
) -> Box<
    Iterator<
        Item = (
            (BigInt, BigInt),
            (rug::Integer, rug::Integer),
            (Integer, Integer),
        ),
    >,
> {
    Box::new(pairs_of_integers(gm).map(|(x, y)| {
        (
            (integer_to_bigint(&x), integer_to_bigint(&y)),
            (integer_to_rug_integer(&x), integer_to_rug_integer(&y)),
            (x, y),
        )
    }))
}

pub fn triples_of_integers(
    gm: GenerationMode,
) -> Box<Iterator<Item = (Integer, Integer, Integer)>> {
    match gm {
        GenerationMode::Exhaustive => {
            Box::new(exhaustive_triples_from_single(exhaustive_integers()))
        }
        GenerationMode::Random(scale) => Box::new(random_triples_from_single(random_integers(
            &EXAMPLE_SEED,
            scale,
        ))),
        GenerationMode::SpecialRandom(scale) => Box::new(random_triples_from_single(
            special_random_integers(&EXAMPLE_SEED, scale),
        )),
    }
}

pub fn natural_integers(gm: GenerationMode) -> Box<Iterator<Item = Integer>> {
    match gm {
        GenerationMode::Exhaustive => Box::new(exhaustive_natural_integers()),
        GenerationMode::Random(scale) => Box::new(random_natural_integers(&EXAMPLE_SEED, scale)),
        GenerationMode::SpecialRandom(scale) => {
            Box::new(special_random_natural_integers(&EXAMPLE_SEED, scale))
        }
    }
}

fn random_pairs_of_integer_and_primitive<T: 'static + PrimitiveInteger>(
    scale: u32,
) -> Box<Iterator<Item = (Integer, T)>> {
    Box::new(random_pairs(
        &EXAMPLE_SEED,
        &(|seed| random_integers(seed, scale)),
        &(|seed| random(seed)),
    ))
}

fn random_pairs_of_primitive_and_integer<T: 'static + PrimitiveInteger>(
    scale: u32,
) -> Box<Iterator<Item = (T, Integer)>> {
    Box::new(random_pairs(
        &EXAMPLE_SEED,
        &(|seed| random(seed)),
        &(|seed| random_integers(seed, scale)),
    ))
}

pub fn pairs_of_integer_and_signed<T: 'static + PrimitiveSigned>(
    gm: GenerationMode,
) -> Box<Iterator<Item = (Integer, T)>> {
    match gm {
        GenerationMode::Exhaustive => {
            Box::new(exhaustive_pairs(exhaustive_integers(), exhaustive_signed()))
        }
        GenerationMode::Random(scale) => random_pairs_of_integer_and_primitive(scale),
        GenerationMode::SpecialRandom(scale) => Box::new(random_pairs(
            &EXAMPLE_SEED,
            &(|seed| special_random_integers(seed, scale)),
            &(|seed| special_random_signed(seed)),
        )),
    }
}

pub fn nrm_pairs_of_integer_and_signed<T: 'static + PrimitiveSigned>(
    gm: GenerationMode,
) -> Box<Iterator<Item = ((BigInt, T), (rug::Integer, T), (Integer, T))>> {
    Box::new(pairs_of_integer_and_signed(gm).map(|(x, y)| {
        (
            (integer_to_bigint(&x), y),
            (integer_to_rug_integer(&x), y),
            (x, y),
        )
    }))
}

pub fn rm_pairs_of_integer_and_signed<T: 'static + PrimitiveSigned>(
    gm: GenerationMode,
) -> Box<Iterator<Item = ((rug::Integer, T), (Integer, T))>> {
    Box::new(
        pairs_of_integer_and_signed(gm).map(|(x, y)| ((integer_to_rug_integer(&x), y), (x, y))),
    )
}

pub fn pairs_of_signed_and_integer<T: 'static + PrimitiveSigned>(
    gm: GenerationMode,
) -> Box<Iterator<Item = (T, Integer)>> {
    match gm {
        GenerationMode::Exhaustive => {
            Box::new(exhaustive_pairs(exhaustive_signed(), exhaustive_integers()))
        }
        GenerationMode::Random(scale) => random_pairs_of_primitive_and_integer(scale),
        GenerationMode::SpecialRandom(scale) => Box::new(random_pairs(
            &EXAMPLE_SEED,
            &(|seed| special_random_signed(seed)),
            &(|seed| special_random_integers(seed, scale)),
        )),
    }
}

pub fn rm_pairs_of_signed_and_integer<T: 'static + PrimitiveSigned>(
    gm: GenerationMode,
) -> Box<Iterator<Item = ((T, rug::Integer), (T, Integer))>> {
    Box::new(
        pairs_of_signed_and_integer(gm).map(|(x, y)| ((x, integer_to_rug_integer(&y)), (x, y))),
    )
}

pub fn pairs_of_integer_and_unsigned<T: 'static + PrimitiveUnsigned>(
    gm: GenerationMode,
) -> Box<Iterator<Item = (Integer, T)>> {
    match gm {
        GenerationMode::Exhaustive => Box::new(exhaustive_pairs(
            exhaustive_integers(),
            exhaustive_unsigned(),
        )),
        GenerationMode::Random(scale) => random_pairs_of_integer_and_primitive(scale),
        GenerationMode::SpecialRandom(scale) => Box::new(random_pairs(
            &EXAMPLE_SEED,
            &(|seed| special_random_integers(seed, scale)),
            &(|seed| special_random_unsigned(seed)),
        )),
    }
}

pub fn rm_pairs_of_integer_and_unsigned<T: 'static + PrimitiveUnsigned>(
    gm: GenerationMode,
) -> Box<Iterator<Item = ((rug::Integer, T), (Integer, T))>> {
    Box::new(
        pairs_of_integer_and_unsigned(gm).map(|(x, y)| ((integer_to_rug_integer(&x), y), (x, y))),
    )
}

pub fn nrm_pairs_of_integer_and_unsigned<T: 'static + PrimitiveUnsigned>(
    gm: GenerationMode,
) -> Box<Iterator<Item = ((BigInt, T), (rug::Integer, T), (Integer, T))>> {
    Box::new(pairs_of_integer_and_unsigned(gm).map(|(x, y)| {
        (
            (integer_to_bigint(&x), y),
            (integer_to_rug_integer(&x), y),
            (x, y),
        )
    }))
}

pub fn pairs_of_unsigned_and_integer<T: 'static + PrimitiveUnsigned>(
    gm: GenerationMode,
) -> Box<Iterator<Item = (T, Integer)>> {
    match gm {
        GenerationMode::Exhaustive => Box::new(exhaustive_pairs(
            exhaustive_unsigned(),
            exhaustive_integers(),
        )),
        GenerationMode::Random(scale) => random_pairs_of_primitive_and_integer(scale),
        GenerationMode::SpecialRandom(scale) => Box::new(random_pairs(
            &EXAMPLE_SEED,
            &(|seed| special_random_unsigned(seed)),
            &(|seed| special_random_integers(seed, scale)),
        )),
    }
}

pub fn nrm_pairs_of_unsigned_and_integer<T: 'static + PrimitiveUnsigned>(
    gm: GenerationMode,
) -> Box<Iterator<Item = ((T, BigInt), (T, rug::Integer), (T, Integer))>> {
    Box::new(pairs_of_unsigned_and_integer(gm).map(|(x, y)| {
        (
            (x, integer_to_bigint(&y)),
            (x, integer_to_rug_integer(&y)),
            (x, y),
        )
    }))
}

pub fn rm_pairs_of_unsigned_and_integer<T: 'static + PrimitiveUnsigned>(
    gm: GenerationMode,
) -> Box<Iterator<Item = ((T, rug::Integer), (T, Integer))>> {
    Box::new(
        pairs_of_unsigned_and_integer(gm).map(|(x, y)| ((x, integer_to_rug_integer(&y)), (x, y))),
    )
}

fn random_triples_of_integer_integer_and_primitive<T: 'static + PrimitiveInteger>(
    scale: u32,
) -> Box<Iterator<Item = (Integer, Integer, T)>> {
    Box::new(random_triples(
        &EXAMPLE_SEED,
        &(|seed| random_integers(seed, scale)),
        &(|seed| random_integers(seed, scale)),
        &(|seed| random(seed)),
    ))
}

pub fn triples_of_integer_integer_and_signed<T: 'static + PrimitiveSigned>(
    gm: GenerationMode,
) -> Box<Iterator<Item = (Integer, Integer, T)>> {
    match gm {
        GenerationMode::Exhaustive => Box::new(exhaustive_triples(
            exhaustive_integers(),
            exhaustive_integers(),
            exhaustive_signed(),
        )),
        GenerationMode::Random(scale) => random_triples_of_integer_integer_and_primitive(scale),
        GenerationMode::SpecialRandom(scale) => Box::new(random_triples(
            &EXAMPLE_SEED,
            &(|seed| special_random_integers(seed, scale)),
            &(|seed| special_random_integers(seed, scale)),
            &(|seed| special_random_signed(seed)),
        )),
    }
}

pub fn triples_of_integer_integer_and_unsigned<T: 'static + PrimitiveUnsigned>(
    gm: GenerationMode,
) -> Box<Iterator<Item = (Integer, Integer, T)>> {
    match gm {
        GenerationMode::Exhaustive => Box::new(exhaustive_triples(
            exhaustive_integers(),
            exhaustive_integers(),
            exhaustive_unsigned(),
        )),
        GenerationMode::Random(scale) => random_triples_of_integer_integer_and_primitive(scale),
        GenerationMode::SpecialRandom(scale) => Box::new(random_triples(
            &EXAMPLE_SEED,
            &(|seed| special_random_integers(seed, scale)),
            &(|seed| special_random_integers(seed, scale)),
            &(|seed| special_random_unsigned(seed)),
        )),
    }
}

fn log_pairs_of_integer_and_unsigned<T: 'static + PrimitiveUnsigned>(
) -> Box<Iterator<Item = (Integer, T)>> {
    Box::new(log_pairs(exhaustive_integers(), exhaustive_unsigned()))
}

pub fn pairs_of_integer_and_small_u32(gm: GenerationMode) -> Box<Iterator<Item = (Integer, u32)>> {
    match gm {
        GenerationMode::Exhaustive => log_pairs_of_integer_and_unsigned(),
        GenerationMode::Random(scale) => Box::new(random_pairs(
            &EXAMPLE_SEED,
            &(|seed| random_integers(seed, scale)),
            &(|seed| u32s_geometric(seed, scale)),
        )),
        GenerationMode::SpecialRandom(scale) => Box::new(random_pairs(
            &EXAMPLE_SEED,
            &(|seed| special_random_integers(seed, scale)),
            &(|seed| u32s_geometric(seed, scale)),
        )),
    }
}

pub fn rm_pairs_of_integer_and_small_u32(
    gm: GenerationMode,
) -> Box<Iterator<Item = ((rug::Integer, u32), (Integer, u32))>> {
    Box::new(
        pairs_of_integer_and_small_u32(gm).map(|(x, y)| ((integer_to_rug_integer(&x), y), (x, y))),
    )
}

// All pairs of `Integer` and `u32` where the `Integer` is divisible by 2 to the power of the `u32`.
pub fn pairs_of_integer_and_small_u32_var_1(
    gm: GenerationMode,
) -> Box<Iterator<Item = (Integer, u32)>> {
    Box::new(pairs_of_integer_and_small_u32(gm).map(|(n, u)| (n << u, u)))
}

// All pairs of `Natural` and `u32` where the `Natural` is not divisible by 2 to the power of the
// `u32`.
pub fn pairs_of_integer_and_small_u32_var_2(
    gm: GenerationMode,
) -> Box<Iterator<Item = (Integer, u32)>> {
    Box::new(
        pairs_of_integer_and_small_u32(gm).filter(|&(ref n, u)| !n.divisible_by_power_of_two(u)),
    )
}

pub fn pairs_of_integer_and_small_u64(gm: GenerationMode) -> Box<Iterator<Item = (Integer, u64)>> {
    match gm {
        GenerationMode::Exhaustive => log_pairs_of_integer_and_unsigned(),
        GenerationMode::Random(scale) => Box::new(random_pairs(
            &EXAMPLE_SEED,
            &(|seed| random_integers(seed, scale)),
            &(|seed| u32s_geometric(seed, scale).map(|i| i.into())),
        )),
        GenerationMode::SpecialRandom(scale) => Box::new(random_pairs(
            &EXAMPLE_SEED,
            &(|seed| special_random_integers(seed, scale)),
            &(|seed| u32s_geometric(seed, scale).map(|i| i.into())),
        )),
    }
}

pub fn triples_of_integer_small_u32_and_small_u32(
    gm: GenerationMode,
) -> Box<Iterator<Item = (Integer, u32, u32)>> {
    match gm {
        GenerationMode::Exhaustive => reshape_1_2_to_3(Box::new(log_pairs(
            exhaustive_integers(),
            exhaustive_pairs_from_single(exhaustive_unsigned()),
        ))),
        GenerationMode::Random(scale) => Box::new(random_triples(
            &EXAMPLE_SEED,
            &(|seed| random_integers(seed, scale)),
            &(|seed| u32s_geometric(seed, scale)),
            &(|seed| u32s_geometric(seed, scale)),
        )),
        GenerationMode::SpecialRandom(scale) => Box::new(random_triples(
            &EXAMPLE_SEED,
            &(|seed| special_random_integers(seed, scale)),
            &(|seed| u32s_geometric(seed, scale)),
            &(|seed| u32s_geometric(seed, scale)),
        )),
    }
}

fn log_pairs_of_integer_and_signed<T: 'static + PrimitiveSigned>(
) -> Box<Iterator<Item = (Integer, T)>> {
    Box::new(log_pairs(exhaustive_integers(), exhaustive_signed()))
}

pub fn pairs_of_integer_and_small_i32(gm: GenerationMode) -> Box<Iterator<Item = (Integer, i32)>> {
    match gm {
        GenerationMode::Exhaustive => log_pairs_of_integer_and_signed(),
        GenerationMode::Random(scale) => Box::new(random_pairs(
            &EXAMPLE_SEED,
            &(|seed| random_integers(seed, scale)),
            &(|seed| i32s_geometric(seed, scale)),
        )),
        GenerationMode::SpecialRandom(scale) => Box::new(random_pairs(
            &EXAMPLE_SEED,
            &(|seed| special_random_integers(seed, scale)),
            &(|seed| i32s_geometric(seed, scale)),
        )),
    }
}

pub fn rm_pairs_of_integer_and_small_i32(
    gm: GenerationMode,
) -> Box<Iterator<Item = ((rug::Integer, i32), (Integer, i32))>> {
    Box::new(
        pairs_of_integer_and_small_i32(gm).map(|(x, y)| ((integer_to_rug_integer(&x), y), (x, y))),
    )
}

fn random_triples_of_integer_primitive_and_integer<T: 'static + PrimitiveInteger>(
    scale: u32,
) -> Box<Iterator<Item = (Integer, T, Integer)>> {
    Box::new(random_triples(
        &EXAMPLE_SEED,
        &(|seed| random_integers(seed, scale)),
        &(|seed| random(seed)),
        &(|seed| random_integers(seed, scale)),
    ))
}

fn random_triples_of_primitive_integer_and_primitive<T: 'static + PrimitiveInteger>(
    scale: u32,
) -> Box<Iterator<Item = (T, Integer, T)>> {
    Box::new(random_triples(
        &EXAMPLE_SEED,
        &(|seed| random(seed)),
        &(|seed| random_integers(seed, scale)),
        &(|seed| random(seed)),
    ))
}

pub fn triples_of_integer_unsigned_and_integer<T: 'static + PrimitiveUnsigned>(
    gm: GenerationMode,
) -> Box<Iterator<Item = (Integer, T, Integer)>> {
    match gm {
        GenerationMode::Exhaustive => Box::new(exhaustive_triples(
            exhaustive_integers(),
            exhaustive_unsigned(),
            exhaustive_integers(),
        )),
        GenerationMode::Random(scale) => random_triples_of_integer_primitive_and_integer(scale),
        GenerationMode::SpecialRandom(scale) => Box::new(random_triples(
            &EXAMPLE_SEED,
            &(|seed| special_random_integers(seed, scale)),
            &(|seed| special_random_unsigned(seed)),
            &(|seed| special_random_integers(seed, scale)),
        )),
    }
}

pub fn triples_of_unsigned_integer_and_unsigned<T: 'static + PrimitiveUnsigned>(
    gm: GenerationMode,
) -> Box<Iterator<Item = (T, Integer, T)>> {
    match gm {
        GenerationMode::Exhaustive => Box::new(exhaustive_triples(
            exhaustive_unsigned(),
            exhaustive_integers(),
            exhaustive_unsigned(),
        )),
        GenerationMode::Random(scale) => random_triples_of_primitive_integer_and_primitive(scale),
        GenerationMode::SpecialRandom(scale) => Box::new(random_triples(
            &EXAMPLE_SEED,
            &(|seed| special_random_unsigned(seed)),
            &(|seed| special_random_integers(seed, scale)),
            &(|seed| special_random_unsigned(seed)),
        )),
    }
}

pub fn triples_of_integer_signed_and_integer<T: 'static + PrimitiveSigned>(
    gm: GenerationMode,
) -> Box<Iterator<Item = (Integer, T, Integer)>> {
    match gm {
        GenerationMode::Exhaustive => Box::new(exhaustive_triples(
            exhaustive_integers(),
            exhaustive_signed(),
            exhaustive_integers(),
        )),
        GenerationMode::Random(scale) => random_triples_of_integer_primitive_and_integer(scale),
        GenerationMode::SpecialRandom(scale) => Box::new(random_triples(
            &EXAMPLE_SEED,
            &(|seed| special_random_integers(seed, scale)),
            &(|seed| special_random_signed(seed)),
            &(|seed| special_random_integers(seed, scale)),
        )),
    }
}

pub fn triples_of_signed_integer_and_signed<T: 'static + PrimitiveSigned>(
    gm: GenerationMode,
) -> Box<Iterator<Item = (T, Integer, T)>> {
    match gm {
        GenerationMode::Exhaustive => Box::new(exhaustive_triples(
            exhaustive_signed(),
            exhaustive_integers(),
            exhaustive_signed(),
        )),
        GenerationMode::Random(scale) => random_triples_of_primitive_integer_and_primitive(scale),
        GenerationMode::SpecialRandom(scale) => Box::new(random_triples(
            &EXAMPLE_SEED,
            &(|seed| special_random_signed(seed)),
            &(|seed| special_random_integers(seed, scale)),
            &(|seed| special_random_signed(seed)),
        )),
    }
}

pub fn pairs_of_integer_and_natural(
    gm: GenerationMode,
) -> Box<Iterator<Item = (Integer, Natural)>> {
    match gm {
        GenerationMode::Exhaustive => Box::new(exhaustive_pairs(
            exhaustive_integers(),
            exhaustive_naturals(),
        )),
        GenerationMode::Random(scale) => Box::new(random_pairs(
            &EXAMPLE_SEED,
            &(|seed| random_integers(seed, scale)),
            &(|seed| random_naturals(seed, scale)),
        )),
        GenerationMode::SpecialRandom(scale) => Box::new(random_pairs(
            &EXAMPLE_SEED,
            &(|seed| special_random_integers(seed, scale)),
            &(|seed| special_random_naturals(seed, scale)),
        )),
    }
}

pub fn rm_pairs_of_integer_and_natural(
    gm: GenerationMode,
) -> Box<Iterator<Item = ((rug::Integer, rug::Integer), (Integer, Natural))>> {
    Box::new(pairs_of_integer_and_natural(gm).map(|(x, y)| {
        (
            (integer_to_rug_integer(&x), natural_to_rug_integer(&y)),
            (x, y),
        )
    }))
}

pub fn pairs_of_natural_and_integer(
    gm: GenerationMode,
) -> Box<Iterator<Item = (Natural, Integer)>> {
    match gm {
        GenerationMode::Exhaustive => Box::new(exhaustive_pairs(
            exhaustive_naturals(),
            exhaustive_integers(),
        )),
        GenerationMode::Random(scale) => Box::new(random_pairs(
            &EXAMPLE_SEED,
            &(|seed| random_naturals(seed, scale)),
            &(|seed| random_integers(seed, scale)),
        )),
        GenerationMode::SpecialRandom(scale) => Box::new(random_pairs(
            &EXAMPLE_SEED,
            &(|seed| special_random_naturals(seed, scale)),
            &(|seed| special_random_integers(seed, scale)),
        )),
    }
}

pub fn rm_pairs_of_natural_and_integer(
    gm: GenerationMode,
) -> Box<Iterator<Item = ((rug::Integer, rug::Integer), (Natural, Integer))>> {
    Box::new(pairs_of_natural_and_integer(gm).map(|(x, y)| {
        (
            (natural_to_rug_integer(&x), integer_to_rug_integer(&y)),
            (x, y),
        )
    }))
}

pub fn pairs_of_natural_and_natural_integer(
    gm: GenerationMode,
) -> Box<Iterator<Item = (Natural, Integer)>> {
    match gm {
        GenerationMode::Exhaustive => Box::new(exhaustive_pairs(
            exhaustive_naturals(),
            exhaustive_natural_integers(),
        )),
        GenerationMode::Random(scale) => Box::new(random_pairs(
            &EXAMPLE_SEED,
            &(|seed| random_naturals(seed, scale)),
            &(|seed| random_natural_integers(seed, scale)),
        )),
        GenerationMode::SpecialRandom(scale) => Box::new(random_pairs(
            &EXAMPLE_SEED,
            &(|seed| special_random_naturals(seed, scale)),
            &(|seed| special_random_natural_integers(seed, scale)),
        )),
    }
}

pub fn triples_of_integer_natural_and_integer(
    gm: GenerationMode,
) -> Box<Iterator<Item = (Integer, Natural, Integer)>> {
    match gm {
        GenerationMode::Exhaustive => Box::new(exhaustive_triples(
            exhaustive_integers(),
            exhaustive_naturals(),
            exhaustive_integers(),
        )),
        GenerationMode::Random(scale) => Box::new(random_triples(
            &EXAMPLE_SEED,
            &(|seed| random_integers(seed, scale)),
            &(|seed| random_naturals(seed, scale)),
            &(|seed| random_integers(seed, scale)),
        )),
        GenerationMode::SpecialRandom(scale) => Box::new(random_triples(
            &EXAMPLE_SEED,
            &(|seed| special_random_integers(seed, scale)),
            &(|seed| special_random_naturals(seed, scale)),
            &(|seed| special_random_integers(seed, scale)),
        )),
    }
}

pub fn triples_of_natural_integer_and_natural(
    gm: GenerationMode,
) -> Box<Iterator<Item = (Natural, Integer, Natural)>> {
    match gm {
        GenerationMode::Exhaustive => Box::new(exhaustive_triples(
            exhaustive_naturals(),
            exhaustive_integers(),
            exhaustive_naturals(),
        )),
        GenerationMode::Random(scale) => Box::new(random_triples(
            &EXAMPLE_SEED,
            &(|seed| random_naturals(seed, scale)),
            &(|seed| random_integers(seed, scale)),
            &(|seed| random_naturals(seed, scale)),
        )),
        GenerationMode::SpecialRandom(scale) => Box::new(random_triples(
            &EXAMPLE_SEED,
            &(|seed| special_random_naturals(seed, scale)),
            &(|seed| special_random_integers(seed, scale)),
            &(|seed| special_random_naturals(seed, scale)),
        )),
    }
}

pub fn triples_of_integer_small_u64_and_bool(
    gm: GenerationMode,
) -> Box<Iterator<Item = (Integer, u64, bool)>> {
    match gm {
        GenerationMode::Exhaustive => reshape_2_1_to_3(Box::new(lex_pairs(
            exhaustive_pairs(exhaustive_integers(), exhaustive_unsigned()),
            exhaustive_bools(),
        ))),
        GenerationMode::Random(scale) => Box::new(random_triples(
            &EXAMPLE_SEED,
            &(|seed| random_integers(seed, scale)),
            &(|seed| u32s_geometric(seed, scale).map(|i| i.into())),
            &(|seed| random(seed)),
        )),
        GenerationMode::SpecialRandom(scale) => Box::new(random_triples(
            &EXAMPLE_SEED,
            &(|seed| special_random_integers(seed, scale)),
            &(|seed| u32s_geometric(seed, scale).map(|i| i.into())),
            &(|seed| random(seed)),
        )),
    }
}

pub fn pairs_of_integer_and_rounding_mode(
    gm: GenerationMode,
) -> Box<Iterator<Item = (Integer, RoundingMode)>> {
    match gm {
        GenerationMode::Exhaustive => Box::new(lex_pairs(
            exhaustive_integers(),
            exhaustive_rounding_modes(),
        )),
        GenerationMode::Random(scale) => Box::new(random_pairs(
            &EXAMPLE_SEED,
            &(|seed| random_integers(seed, scale)),
            &(|seed| random_rounding_modes(seed)),
        )),
        GenerationMode::SpecialRandom(scale) => Box::new(random_pairs(
            &EXAMPLE_SEED,
            &(|seed| special_random_integers(seed, scale)),
            &(|seed| random_rounding_modes(seed)),
        )),
    }
}

fn triples_of_integer_small_i32_and_rounding_mode(
    gm: GenerationMode,
) -> Box<Iterator<Item = (Integer, i32, RoundingMode)>> {
    match gm {
        GenerationMode::Exhaustive => reshape_2_1_to_3(Box::new(lex_pairs(
            log_pairs_of_integer_and_signed(),
            exhaustive_rounding_modes(),
        ))),
        GenerationMode::Random(scale) => Box::new(random_triples(
            &EXAMPLE_SEED,
            &(|seed| random_integers(seed, scale)),
            &(|seed| i32s_geometric(seed, scale)),
            &(|seed| random_rounding_modes(seed)),
        )),
        GenerationMode::SpecialRandom(scale) => Box::new(random_triples(
            &EXAMPLE_SEED,
            &(|seed| special_random_integers(seed, scale)),
            &(|seed| i32s_geometric(seed, scale)),
            &(|seed| random_rounding_modes(seed)),
        )),
    }
}

// All triples of `Integer`, `i32`, and `RoundingMode`, such that if the `i32` is negative and the
// `RoundingMode` is `RoundingMode::Exact`, the `Integer` is divisible by 2 to the power of the
// negative of the `i32`.
pub fn triples_of_integer_small_i32_and_rounding_mode_var_1(
    gm: GenerationMode,
) -> Box<Iterator<Item = (Integer, i32, RoundingMode)>> {
    Box::new(
        triples_of_integer_small_i32_and_rounding_mode(gm).filter(|&(ref n, i, rm)| {
            i >= 0 || rm != RoundingMode::Exact
                || n.divisible_by_power_of_two(i.wrapping_neg() as u32)
        }),
    )
}

// All triples of `Integer`, `i32`, and `RoundingMode`, such that if the `i32` is positive and the
// `RoundingMode` is `RoundingMode::Exact`, the `Integer` is divisible by 2 to the power of the
// `i32`.
pub fn triples_of_integer_small_i32_and_rounding_mode_var_2(
    gm: GenerationMode,
) -> Box<Iterator<Item = (Integer, i32, RoundingMode)>> {
    Box::new(
        triples_of_integer_small_i32_and_rounding_mode(gm).filter(|&(ref n, i, rm)| {
            i <= 0 || rm != RoundingMode::Exact || n.divisible_by_power_of_two(i as u32)
        }),
    )
}

fn triples_of_integer_small_u32_and_rounding_mode(
    gm: GenerationMode,
) -> Box<Iterator<Item = (Integer, u32, RoundingMode)>> {
    match gm {
        GenerationMode::Exhaustive => reshape_2_1_to_3(Box::new(lex_pairs(
            log_pairs_of_integer_and_unsigned(),
            exhaustive_rounding_modes(),
        ))),
        GenerationMode::Random(scale) => Box::new(random_triples(
            &EXAMPLE_SEED,
            &(|seed| random_integers(seed, scale)),
            &(|seed| u32s_geometric(seed, scale)),
            &(|seed| random_rounding_modes(seed)),
        )),
        GenerationMode::SpecialRandom(scale) => Box::new(random_triples(
            &EXAMPLE_SEED,
            &(|seed| special_random_integers(seed, scale)),
            &(|seed| u32s_geometric(seed, scale)),
            &(|seed| random_rounding_modes(seed)),
        )),
    }
}

// All triples of `Integer`, `u32`, and `RoundingMode`, where if the `RoundingMode` is
// `RoundingMode::Exact`, the `Integer` is divisible by 2 to the power of the `u32`.
pub fn triples_of_integer_small_u32_and_rounding_mode_var_1(
    gm: GenerationMode,
) -> Box<Iterator<Item = (Integer, u32, RoundingMode)>> {
    Box::new(
        triples_of_integer_small_u32_and_rounding_mode(gm)
            .filter(|&(ref n, u, rm)| rm != RoundingMode::Exact || n.divisible_by_power_of_two(u)),
    )
}
