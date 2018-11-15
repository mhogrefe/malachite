use common::{integer_to_bigint, integer_to_rug_integer, natural_to_rug_integer, GenerationMode};
use inputs::common::{reshape_1_2_to_3, reshape_2_1_to_3};
use malachite_base::misc::CheckedFrom;
use malachite_base::num::{
    Abs, DivisibleBy, DivisibleByPowerOfTwo, EqMod, EqModPowerOfTwo, PrimitiveInteger,
    PrimitiveSigned, PrimitiveUnsigned,
};
use malachite_base::round::RoundingMode;
use malachite_nz::integer::Integer;
use malachite_nz::natural::Natural;
use num::BigInt;
use rand::{IsaacRng, Rng, SeedableRng};
use rug;
use rust_wheels::iterators::bools::exhaustive_bools;
use rust_wheels::iterators::common::{scramble, EXAMPLE_SEED};
use rust_wheels::iterators::dependent_pairs::dependent_pairs;
use rust_wheels::iterators::general::random;
use rust_wheels::iterators::integers::{
    exhaustive_integers, exhaustive_natural_integers, exhaustive_nonzero_integers, random_integers,
    random_natural_integers, random_nonzero_integers, special_random_integers,
    special_random_natural_integers, special_random_nonzero_integers,
};
use rust_wheels::iterators::integers_geometric::{i32s_geometric, u32s_geometric};
use rust_wheels::iterators::naturals::{
    exhaustive_naturals, random_naturals, special_random_naturals,
};
use rust_wheels::iterators::primitive_ints::{
    exhaustive_natural_signed, exhaustive_nonzero_signed, exhaustive_positive, exhaustive_signed,
    exhaustive_unsigned, random_natural_signed, random_nonzero_signed, random_positive_unsigned,
    special_random_natural_signed, special_random_nonzero_signed, special_random_positive_unsigned,
    special_random_signed, special_random_unsigned,
};
use rust_wheels::iterators::rounding_modes::{exhaustive_rounding_modes, random_rounding_modes};
use rust_wheels::iterators::tuples::{
    exhaustive_pairs, exhaustive_pairs_from_single, exhaustive_quadruples, exhaustive_triples,
    exhaustive_triples_from_single, lex_pairs, log_pairs, random_pairs, random_pairs_from_single,
    random_quadruples, random_triples, random_triples_from_single,
};
use rust_wheels::iterators::vecs::exhaustive_fixed_size_vecs_from_single;
use std::ops::{Add, Mul, Shl, Shr};

pub fn integers(gm: GenerationMode) -> Box<Iterator<Item = Integer>> {
    match gm {
        GenerationMode::Exhaustive => Box::new(exhaustive_integers()),
        GenerationMode::Random(scale) => Box::new(random_integers(&EXAMPLE_SEED, scale)),
        GenerationMode::SpecialRandom(scale) => {
            Box::new(special_random_integers(&EXAMPLE_SEED, scale))
        }
    }
}

pub fn rm_integers(gm: GenerationMode) -> Box<Iterator<Item = (rug::Integer, Integer)>> {
    Box::new(integers(gm).map(|n| (integer_to_rug_integer(&n), n)))
}

pub fn nm_integers(gm: GenerationMode) -> Box<Iterator<Item = (BigInt, Integer)>> {
    Box::new(integers(gm).map(|n| (integer_to_bigint(&n), n)))
}

pub fn nrm_integers(gm: GenerationMode) -> Box<Iterator<Item = (BigInt, rug::Integer, Integer)>> {
    Box::new(integers(gm).map(|n| (integer_to_bigint(&n), integer_to_rug_integer(&n), n)))
}

pub fn nonzero_integers(gm: GenerationMode) -> Box<Iterator<Item = Integer>> {
    match gm {
        GenerationMode::Exhaustive => Box::new(exhaustive_nonzero_integers()),
        GenerationMode::Random(scale) => Box::new(random_nonzero_integers(&EXAMPLE_SEED, scale)),
        GenerationMode::SpecialRandom(scale) => {
            Box::new(special_random_nonzero_integers(&EXAMPLE_SEED, scale))
        }
    }
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

pub fn triples_of_natural_integers(
    gm: GenerationMode,
) -> Box<Iterator<Item = (Integer, Integer, Integer)>> {
    match gm {
        GenerationMode::Exhaustive => {
            Box::new(exhaustive_triples_from_single(exhaustive_natural_integers()))
        }
        GenerationMode::Random(scale) => Box::new(random_triples_from_single(
            random_natural_integers(&EXAMPLE_SEED, scale),
        )),
        GenerationMode::SpecialRandom(scale) => Box::new(random_triples_from_single(
            special_random_natural_integers(&EXAMPLE_SEED, scale),
        )),
    }
}

fn random_pairs_of_integer_and_primitive<T: PrimitiveInteger>(
    scale: u32,
) -> Box<Iterator<Item = (Integer, T)>> {
    Box::new(random_pairs(
        &EXAMPLE_SEED,
        &(|seed| random_integers(seed, scale)),
        &(|seed| random(seed)),
    ))
}

fn random_pairs_of_primitive_and_integer<T: PrimitiveInteger>(
    scale: u32,
) -> Box<Iterator<Item = (T, Integer)>> {
    Box::new(random_pairs(
        &EXAMPLE_SEED,
        &(|seed| random(seed)),
        &(|seed| random_integers(seed, scale)),
    ))
}

pub fn pairs_of_integer_and_signed<T: PrimitiveSigned>(
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

pub fn nm_pairs_of_integer_and_signed<T: PrimitiveSigned>(
    gm: GenerationMode,
) -> Box<Iterator<Item = ((BigInt, T), (Integer, T))>> {
    Box::new(pairs_of_integer_and_signed(gm).map(|(x, y)| ((integer_to_bigint(&x), y), (x, y))))
}

pub fn rm_pairs_of_integer_and_signed<T: PrimitiveSigned>(
    gm: GenerationMode,
) -> Box<Iterator<Item = ((rug::Integer, T), (Integer, T))>> {
    Box::new(
        pairs_of_integer_and_signed(gm).map(|(x, y)| ((integer_to_rug_integer(&x), y), (x, y))),
    )
}

pub fn nrm_pairs_of_integer_and_signed<T: PrimitiveSigned>(
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

pub fn pairs_of_signed_and_integer<T: PrimitiveSigned>(
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

pub fn rm_pairs_of_signed_and_integer<T: PrimitiveSigned>(
    gm: GenerationMode,
) -> Box<Iterator<Item = ((T, rug::Integer), (T, Integer))>> {
    Box::new(
        pairs_of_signed_and_integer(gm).map(|(x, y)| ((x, integer_to_rug_integer(&y)), (x, y))),
    )
}

pub fn pairs_of_integer_and_unsigned<T: PrimitiveUnsigned>(
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

pub fn rm_pairs_of_integer_and_unsigned<T: PrimitiveUnsigned>(
    gm: GenerationMode,
) -> Box<Iterator<Item = ((rug::Integer, T), (Integer, T))>> {
    Box::new(
        pairs_of_integer_and_unsigned(gm).map(|(x, y)| ((integer_to_rug_integer(&x), y), (x, y))),
    )
}

pub fn nm_pairs_of_integer_and_unsigned<T: PrimitiveUnsigned>(
    gm: GenerationMode,
) -> Box<Iterator<Item = ((BigInt, T), (Integer, T))>> {
    Box::new(pairs_of_integer_and_unsigned(gm).map(|(x, y)| ((integer_to_bigint(&x), y), (x, y))))
}

pub fn nrm_pairs_of_integer_and_unsigned<T: PrimitiveUnsigned>(
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

pub fn pairs_of_integer_and_positive_unsigned<T: PrimitiveUnsigned>(
    gm: GenerationMode,
) -> Box<Iterator<Item = (Integer, T)>> {
    match gm {
        GenerationMode::Exhaustive => Box::new(exhaustive_pairs(
            exhaustive_integers(),
            exhaustive_positive(),
        )),
        GenerationMode::Random(scale) => Box::new(random_pairs(
            &EXAMPLE_SEED,
            &(|seed| random_integers(seed, scale)),
            &(|seed| random_positive_unsigned(seed)),
        )),
        GenerationMode::SpecialRandom(scale) => Box::new(random_pairs(
            &EXAMPLE_SEED,
            &(|seed| special_random_integers(seed, scale)),
            &(|seed| special_random_positive_unsigned(seed)),
        )),
    }
}

pub fn rm_pairs_of_integer_and_positive_unsigned<T: PrimitiveUnsigned>(
    gm: GenerationMode,
) -> Box<Iterator<Item = ((rug::Integer, T), (Integer, T))>> {
    Box::new(
        pairs_of_integer_and_positive_unsigned(gm)
            .map(|(x, y)| ((integer_to_rug_integer(&x), y), (x, y))),
    )
}

pub fn nrm_pairs_of_integer_and_positive_unsigned<T: PrimitiveUnsigned>(
    gm: GenerationMode,
) -> Box<Iterator<Item = ((BigInt, T), (rug::Integer, T), (Integer, T))>> {
    Box::new(pairs_of_integer_and_positive_unsigned(gm).map(|(x, y)| {
        (
            (integer_to_bigint(&x), y),
            (integer_to_rug_integer(&x), y),
            (x, y),
        )
    }))
}

pub fn pairs_of_integer_and_nonzero_signed<T: PrimitiveSigned>(
    gm: GenerationMode,
) -> Box<Iterator<Item = (Integer, T)>> {
    match gm {
        GenerationMode::Exhaustive => Box::new(exhaustive_pairs(
            exhaustive_integers(),
            exhaustive_nonzero_signed(),
        )),
        GenerationMode::Random(scale) => Box::new(random_pairs(
            &EXAMPLE_SEED,
            &(|seed| random_integers(seed, scale)),
            &(|seed| random_nonzero_signed(seed)),
        )),
        GenerationMode::SpecialRandom(scale) => Box::new(random_pairs(
            &EXAMPLE_SEED,
            &(|seed| special_random_integers(seed, scale)),
            &(|seed| special_random_nonzero_signed(seed)),
        )),
    }
}

pub fn rm_pairs_of_integer_and_nonzero_signed<T: PrimitiveSigned>(
    gm: GenerationMode,
) -> Box<Iterator<Item = ((rug::Integer, T), (Integer, T))>> {
    Box::new(
        pairs_of_integer_and_nonzero_signed(gm)
            .map(|(x, y)| ((integer_to_rug_integer(&x), y), (x, y))),
    )
}

pub fn nrm_pairs_of_integer_and_nonzero_signed<T: PrimitiveSigned>(
    gm: GenerationMode,
) -> Box<Iterator<Item = ((BigInt, T), (rug::Integer, T), (Integer, T))>> {
    Box::new(pairs_of_integer_and_nonzero_signed(gm).map(|(x, y)| {
        (
            (integer_to_bigint(&x), y),
            (integer_to_rug_integer(&x), y),
            (x, y),
        )
    }))
}

// All triples of `Integer` and positive `u32` where the `Integer` is divisible by the `T`.
pub fn pairs_of_integer_and_positive_u32_var_1(
    gm: GenerationMode,
) -> Box<Iterator<Item = (Integer, u32)>> {
    Box::new(pairs_of_integer_and_positive_unsigned(gm).map(|(n, u)| (n * u, u)))
}

pub fn nrm_pairs_of_integer_and_positive_u32_var_1(
    gm: GenerationMode,
) -> Box<Iterator<Item = ((BigInt, u32), (rug::Integer, u32), (Integer, u32))>> {
    Box::new(pairs_of_integer_and_positive_u32_var_1(gm).map(|(x, y)| {
        (
            (integer_to_bigint(&x), y),
            (integer_to_rug_integer(&x), y),
            (x, y),
        )
    }))
}

// All pairs of `Integer` and positive `u32`, where the `Integer` is not divisible by the `u32`.
pub fn pairs_of_integer_and_positive_u32_var_2(
    gm: GenerationMode,
) -> Box<Iterator<Item = (Integer, u32)>> {
    Box::new(pairs_of_integer_and_positive_unsigned(gm).filter(|&(ref n, u)| !n.divisible_by(u)))
}

pub fn pairs_of_unsigned_and_integer<T: PrimitiveUnsigned>(
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

// All triples of `Integer` and nonzero `i32` where the `Integer` is divisible by the `T`.
pub fn pairs_of_integer_and_nonzero_i32_var_1(
    gm: GenerationMode,
) -> Box<Iterator<Item = (Integer, i32)>> {
    Box::new(pairs_of_integer_and_nonzero_signed(gm).map(|(n, i)| (n * i, i)))
}

pub fn nrm_pairs_of_unsigned_and_integer<T: PrimitiveUnsigned>(
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

pub fn rm_pairs_of_unsigned_and_integer<T: PrimitiveUnsigned>(
    gm: GenerationMode,
) -> Box<Iterator<Item = ((T, rug::Integer), (T, Integer))>> {
    Box::new(
        pairs_of_unsigned_and_integer(gm).map(|(x, y)| ((x, integer_to_rug_integer(&y)), (x, y))),
    )
}

pub fn pairs_of_unsigned_and_nonzero_integer<T: PrimitiveUnsigned>(
    gm: GenerationMode,
) -> Box<Iterator<Item = (T, Integer)>> {
    match gm {
        GenerationMode::Exhaustive => Box::new(exhaustive_pairs(
            exhaustive_unsigned(),
            exhaustive_nonzero_integers(),
        )),
        GenerationMode::Random(scale) => Box::new(random_pairs(
            &EXAMPLE_SEED,
            &(|seed| random(seed)),
            &(|seed| random_nonzero_integers(seed, scale)),
        )),
        GenerationMode::SpecialRandom(scale) => Box::new(random_pairs(
            &EXAMPLE_SEED,
            &(|seed| special_random_unsigned(seed)),
            &(|seed| special_random_nonzero_integers(seed, scale)),
        )),
    }
}

// All pairs of `u32` and positive `Integer` where the `u32` is not divisible by the `Integer`.
pub fn pairs_of_u32_and_nonzero_integer_var_1(
    gm: GenerationMode,
) -> Box<Iterator<Item = (u32, Integer)>> {
    Box::new(
        pairs_of_unsigned_and_nonzero_integer::<u32>(gm).filter(|&(u, ref n)| !u.divisible_by(n)),
    )
}

// All pairs of `u32` and nonzero `Integer` where the `u32` is divisible by the `Integer`.
pub fn pairs_of_u32_and_nonzero_integer_var_2(
    gm: GenerationMode,
) -> Box<Iterator<Item = (u32, Integer)>> {
    Box::new(
        pairs_of_unsigned_and_nonzero_integer::<u32>(gm)
            .filter_map(|(u, n)| u32::checked_from(u * (&n).abs()).map(|u| (u, n))),
    )
}

pub fn pairs_of_signed_and_nonzero_integer<T: PrimitiveSigned>(
    gm: GenerationMode,
) -> Box<Iterator<Item = (T, Integer)>> {
    match gm {
        GenerationMode::Exhaustive => Box::new(exhaustive_pairs(
            exhaustive_signed(),
            exhaustive_nonzero_integers(),
        )),
        GenerationMode::Random(scale) => Box::new(random_pairs(
            &EXAMPLE_SEED,
            &(|seed| random(seed)),
            &(|seed| random_nonzero_integers(seed, scale)),
        )),
        GenerationMode::SpecialRandom(scale) => Box::new(random_pairs(
            &EXAMPLE_SEED,
            &(|seed| special_random_signed(seed)),
            &(|seed| special_random_nonzero_integers(seed, scale)),
        )),
    }
}

pub fn pairs_of_natural_integer_and_unsigned<T: PrimitiveUnsigned>(
    gm: GenerationMode,
) -> Box<Iterator<Item = (Integer, T)>> {
    match gm {
        GenerationMode::Exhaustive => Box::new(exhaustive_pairs(
            exhaustive_natural_integers(),
            exhaustive_unsigned(),
        )),
        GenerationMode::Random(scale) => Box::new(random_pairs(
            &EXAMPLE_SEED,
            &(|seed| random_natural_integers(seed, scale)),
            &(|seed| random(seed)),
        )),
        GenerationMode::SpecialRandom(scale) => Box::new(random_pairs(
            &EXAMPLE_SEED,
            &(|seed| special_random_natural_integers(seed, scale)),
            &(|seed| special_random_unsigned(seed)),
        )),
    }
}

fn random_triples_of_integer_integer_and_primitive<T: PrimitiveInteger>(
    scale: u32,
) -> Box<Iterator<Item = (Integer, Integer, T)>> {
    Box::new(random_triples(
        &EXAMPLE_SEED,
        &(|seed| random_integers(seed, scale)),
        &(|seed| random_integers(seed, scale)),
        &(|seed| random(seed)),
    ))
}

pub fn triples_of_integer_integer_and_signed<T: PrimitiveSigned>(
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

pub fn triples_of_integer_integer_and_unsigned<T: PrimitiveUnsigned>(
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

pub fn triples_of_natural_integer_unsigned_and_unsigned<T: PrimitiveUnsigned>(
    gm: GenerationMode,
) -> Box<Iterator<Item = (Integer, T, T)>> {
    match gm {
        GenerationMode::Exhaustive => Box::new(exhaustive_triples(
            exhaustive_natural_integers(),
            exhaustive_unsigned(),
            exhaustive_unsigned(),
        )),
        GenerationMode::Random(scale) => Box::new(random_triples(
            &EXAMPLE_SEED,
            &(|seed| random_natural_integers(seed, scale)),
            &(|seed| random(seed)),
            &(|seed| random(seed)),
        )),
        GenerationMode::SpecialRandom(scale) => Box::new(random_triples(
            &EXAMPLE_SEED,
            &(|seed| special_random_natural_integers(seed, scale)),
            &(|seed| special_random_unsigned(seed)),
            &(|seed| special_random_unsigned(seed)),
        )),
    }
}

pub fn triples_of_natural_integer_natural_signed_and_natural_signed<T: PrimitiveSigned>(
    gm: GenerationMode,
) -> Box<Iterator<Item = (Integer, T, T)>> {
    match gm {
        GenerationMode::Exhaustive => Box::new(exhaustive_triples(
            exhaustive_natural_integers(),
            exhaustive_natural_signed(),
            exhaustive_natural_signed(),
        )),
        GenerationMode::Random(scale) => Box::new(random_triples(
            &EXAMPLE_SEED,
            &(|seed| random_natural_integers(seed, scale)),
            &(|seed| random_natural_signed(seed)),
            &(|seed| random_natural_signed(seed)),
        )),
        GenerationMode::SpecialRandom(scale) => Box::new(random_triples(
            &EXAMPLE_SEED,
            &(|seed| special_random_natural_integers(seed, scale)),
            &(|seed| special_random_natural_signed(seed)),
            &(|seed| special_random_natural_signed(seed)),
        )),
    }
}

pub fn triples_of_natural_integer_natural_integer_and_natural_signed<T: PrimitiveSigned>(
    gm: GenerationMode,
) -> Box<Iterator<Item = (Integer, Integer, T)>> {
    match gm {
        GenerationMode::Exhaustive => Box::new(exhaustive_triples(
            exhaustive_natural_integers(),
            exhaustive_natural_integers(),
            exhaustive_natural_signed(),
        )),
        GenerationMode::Random(scale) => Box::new(random_triples(
            &EXAMPLE_SEED,
            &(|seed| random_natural_integers(seed, scale)),
            &(|seed| random_natural_integers(seed, scale)),
            &(|seed| random_natural_signed(seed)),
        )),
        GenerationMode::SpecialRandom(scale) => Box::new(random_triples(
            &EXAMPLE_SEED,
            &(|seed| special_random_natural_integers(seed, scale)),
            &(|seed| special_random_natural_integers(seed, scale)),
            &(|seed| special_random_natural_signed(seed)),
        )),
    }
}

fn log_pairs_of_integer_and_unsigned<T: PrimitiveUnsigned>() -> Box<Iterator<Item = (Integer, T)>> {
    Box::new(log_pairs(exhaustive_integers(), exhaustive_unsigned()))
}

pub fn pairs_of_integer_and_small_unsigned<T: PrimitiveUnsigned>(
    gm: GenerationMode,
) -> Box<Iterator<Item = (Integer, T)>> {
    match gm {
        GenerationMode::Exhaustive => log_pairs_of_integer_and_unsigned(),
        GenerationMode::Random(scale) => Box::new(random_pairs(
            &EXAMPLE_SEED,
            &(|seed| random_integers(seed, scale)),
            &(|seed| u32s_geometric(seed, scale).flat_map(T::checked_from)),
        )),
        GenerationMode::SpecialRandom(scale) => Box::new(random_pairs(
            &EXAMPLE_SEED,
            &(|seed| special_random_integers(seed, scale)),
            &(|seed| u32s_geometric(seed, scale).flat_map(T::checked_from)),
        )),
    }
}

pub fn rm_pairs_of_integer_and_small_unsigned<T: PrimitiveUnsigned>(
    gm: GenerationMode,
) -> Box<Iterator<Item = ((rug::Integer, T), (Integer, T))>> {
    Box::new(
        pairs_of_integer_and_small_unsigned(gm)
            .map(|(x, y)| ((integer_to_rug_integer(&x), y), (x, y))),
    )
}

// All pairs of `Integer` and `T` where `T` is unsigned and the `Integer` is divisible by 2 to the
// power of the `T`.
pub fn pairs_of_integer_and_small_unsigned_var_1<T: PrimitiveUnsigned>(
    gm: GenerationMode,
) -> Box<Iterator<Item = (Integer, T)>>
where
    Integer: Shl<T, Output = Integer>,
{
    Box::new(pairs_of_integer_and_small_unsigned::<T>(gm).map(|(n, u)| (n << u, u)))
}

// All pairs of `Integer` and `T` where `T` is unsigned and the `Integer` is not divisible by 2 to
// the power of the `T`.
pub fn pairs_of_integer_and_small_unsigned_var_2<T: PrimitiveUnsigned>(
    gm: GenerationMode,
) -> Box<Iterator<Item = (Integer, T)>> {
    Box::new(
        pairs_of_integer_and_small_unsigned::<T>(gm)
            .filter(|&(ref n, u)| !n.divisible_by_power_of_two(u.into())),
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

pub fn rm_pairs_of_integer_and_small_u64(
    gm: GenerationMode,
) -> Box<Iterator<Item = ((rug::Integer, u64), (Integer, u64))>> {
    Box::new(
        pairs_of_integer_and_small_u64(gm).map(|(x, y)| ((integer_to_rug_integer(&x), y), (x, y))),
    )
}

pub fn nm_pairs_of_integer_and_small_u64(
    gm: GenerationMode,
) -> Box<Iterator<Item = ((BigInt, u64), (Integer, u64))>> {
    Box::new(pairs_of_integer_and_small_u64(gm).map(|(x, y)| ((integer_to_bigint(&x), y), (x, y))))
}

pub fn nrm_pairs_of_integer_and_small_u64(
    gm: GenerationMode,
) -> Box<Iterator<Item = ((BigInt, u64), (rug::Integer, u64), (Integer, u64))>> {
    Box::new(pairs_of_integer_and_small_u64(gm).map(|(x, y)| {
        (
            (integer_to_bigint(&x), y),
            (integer_to_rug_integer(&x), y),
            (x, y),
        )
    }))
}

pub fn pairs_of_integer_and_small_usize(
    gm: GenerationMode,
) -> Box<Iterator<Item = (Integer, usize)>> {
    Box::new(pairs_of_integer_and_small_unsigned::<u64>(gm).map(|(n, u)| (n, u as usize)))
}

pub fn triples_of_integer_small_unsigned_and_small_unsigned<
    T: PrimitiveUnsigned,
    U: PrimitiveUnsigned,
>(
    gm: GenerationMode,
) -> Box<Iterator<Item = (Integer, T, U)>> {
    match gm {
        GenerationMode::Exhaustive => reshape_1_2_to_3(Box::new(log_pairs(
            exhaustive_integers(),
            exhaustive_pairs(exhaustive_unsigned(), exhaustive_unsigned()),
        ))),
        GenerationMode::Random(scale) => Box::new(random_triples(
            &EXAMPLE_SEED,
            &(|seed| random_integers(seed, scale)),
            &(|seed| u32s_geometric(seed, scale).flat_map(T::checked_from)),
            &(|seed| u32s_geometric(seed, scale).flat_map(U::checked_from)),
        )),
        GenerationMode::SpecialRandom(scale) => Box::new(random_triples(
            &EXAMPLE_SEED,
            &(|seed| special_random_integers(seed, scale)),
            &(|seed| u32s_geometric(seed, scale).flat_map(T::checked_from)),
            &(|seed| u32s_geometric(seed, scale).flat_map(U::checked_from)),
        )),
    }
}

pub fn triples_of_integer_integer_and_positive_unsigned<T: PrimitiveUnsigned>(
    gm: GenerationMode,
) -> Box<Iterator<Item = (Integer, Integer, T)>> {
    match gm {
        GenerationMode::Exhaustive => Box::new(exhaustive_triples(
            exhaustive_integers(),
            exhaustive_integers(),
            exhaustive_positive(),
        )),
        GenerationMode::Random(scale) => Box::new(random_triples(
            &EXAMPLE_SEED,
            &(|seed| random_integers(seed, scale)),
            &(|seed| random_integers(seed, scale)),
            &(|seed| random_positive_unsigned(seed)),
        )),
        GenerationMode::SpecialRandom(scale) => Box::new(random_triples(
            &EXAMPLE_SEED,
            &(|seed| special_random_integers(seed, scale)),
            &(|seed| special_random_integers(seed, scale)),
            &(|seed| special_random_positive_unsigned(seed)),
        )),
    }
}

pub fn triples_of_integer_integer_and_nonzero_signed<T: PrimitiveSigned>(
    gm: GenerationMode,
) -> Box<Iterator<Item = (Integer, Integer, T)>> {
    match gm {
        GenerationMode::Exhaustive => Box::new(exhaustive_triples(
            exhaustive_integers(),
            exhaustive_integers(),
            exhaustive_nonzero_signed(),
        )),
        GenerationMode::Random(scale) => Box::new(random_triples(
            &EXAMPLE_SEED,
            &(|seed| random_integers(seed, scale)),
            &(|seed| random_integers(seed, scale)),
            &(|seed| random_nonzero_signed(seed)),
        )),
        GenerationMode::SpecialRandom(scale) => Box::new(random_triples(
            &EXAMPLE_SEED,
            &(|seed| special_random_integers(seed, scale)),
            &(|seed| special_random_integers(seed, scale)),
            &(|seed| special_random_nonzero_signed(seed)),
        )),
    }
}

fn log_pairs_of_integer_and_signed<T: PrimitiveSigned>() -> Box<Iterator<Item = (Integer, T)>> {
    Box::new(log_pairs(exhaustive_integers(), exhaustive_signed()))
}

pub fn pairs_of_integer_and_small_signed<T: PrimitiveSigned>(
    gm: GenerationMode,
) -> Box<Iterator<Item = (Integer, T)>> {
    match gm {
        GenerationMode::Exhaustive => log_pairs_of_integer_and_signed(),
        GenerationMode::Random(scale) => Box::new(random_pairs(
            &EXAMPLE_SEED,
            &(|seed| random_integers(seed, scale)),
            &(|seed| i32s_geometric(seed, scale).flat_map(T::checked_from)),
        )),
        GenerationMode::SpecialRandom(scale) => Box::new(random_pairs(
            &EXAMPLE_SEED,
            &(|seed| special_random_integers(seed, scale)),
            &(|seed| i32s_geometric(seed, scale).flat_map(T::checked_from)),
        )),
    }
}

pub fn rm_pairs_of_integer_and_small_signed<T: PrimitiveSigned>(
    gm: GenerationMode,
) -> Box<Iterator<Item = ((rug::Integer, T), (Integer, T))>> {
    Box::new(
        pairs_of_integer_and_small_signed(gm)
            .map(|(x, y)| ((integer_to_rug_integer(&x), y), (x, y))),
    )
}

fn random_triples_of_integer_primitive_and_integer<T: PrimitiveInteger>(
    scale: u32,
) -> Box<Iterator<Item = (Integer, T, Integer)>> {
    Box::new(random_triples(
        &EXAMPLE_SEED,
        &(|seed| random_integers(seed, scale)),
        &(|seed| random(seed)),
        &(|seed| random_integers(seed, scale)),
    ))
}

fn random_triples_of_primitive_integer_and_primitive<T: PrimitiveInteger>(
    scale: u32,
) -> Box<Iterator<Item = (T, Integer, T)>> {
    Box::new(random_triples(
        &EXAMPLE_SEED,
        &(|seed| random(seed)),
        &(|seed| random_integers(seed, scale)),
        &(|seed| random(seed)),
    ))
}

fn random_triples_of_integer_primitive_and_primitive<T: PrimitiveInteger>(
    scale: u32,
) -> Box<Iterator<Item = (Integer, T, T)>> {
    Box::new(random_triples(
        &EXAMPLE_SEED,
        &(|seed| random_integers(seed, scale)),
        &(|seed| random(seed)),
        &(|seed| random(seed)),
    ))
}

pub fn triples_of_integer_unsigned_and_integer<T: PrimitiveUnsigned>(
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

pub fn triples_of_unsigned_integer_and_unsigned<T: PrimitiveUnsigned>(
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

pub fn triples_of_integer_unsigned_and_unsigned<T: PrimitiveUnsigned>(
    gm: GenerationMode,
) -> Box<Iterator<Item = (Integer, T, T)>> {
    match gm {
        GenerationMode::Exhaustive => Box::new(exhaustive_triples(
            exhaustive_integers(),
            exhaustive_unsigned(),
            exhaustive_unsigned(),
        )),
        GenerationMode::Random(scale) => random_triples_of_integer_primitive_and_primitive(scale),
        GenerationMode::SpecialRandom(scale) => Box::new(random_triples(
            &EXAMPLE_SEED,
            &(|seed| special_random_integers(seed, scale)),
            &(|seed| special_random_unsigned(seed)),
            &(|seed| special_random_unsigned(seed)),
        )),
    }
}

pub fn rm_triples_of_integer_unsigned_and_unsigned<T: PrimitiveUnsigned>(
    gm: GenerationMode,
) -> Box<Iterator<Item = ((rug::Integer, T, T), (Integer, T, T))>> {
    Box::new(
        triples_of_integer_unsigned_and_unsigned(gm)
            .map(|(x, y, z)| ((integer_to_rug_integer(&x), y, z), (x, y, z))),
    )
}

// All triples of `Integer`, `T`, `T`, where `T` is unsigned and the `Integer` is equal to the first
// `T` mod the second `T`.
pub fn triples_of_integer_unsigned_and_unsigned_var_1<T: PrimitiveUnsigned>(
    gm: GenerationMode,
) -> Box<Iterator<Item = (Integer, T, T)>>
where
    Integer: Mul<T, Output = Integer> + Add<T, Output = Integer>,
{
    Box::new(
        triples_of_integer_unsigned_and_unsigned(gm)
            .map(|(n, u, modulus)| (n * modulus + u, u, modulus)),
    )
}

// All triples of `Integer`, `T`, `T`, where `T` is unsigned and the `Integer` is not equal to the
// first `T` mod the second `T`.
pub fn triples_of_integer_unsigned_and_unsigned_var_2<T: PrimitiveUnsigned>(
    gm: GenerationMode,
) -> Box<Iterator<Item = (Integer, T, T)>> {
    Box::new(
        triples_of_integer_unsigned_and_unsigned::<T>(gm).filter(|&(ref n, u, modulus)| {
            let u: u32 = u.checked_into().unwrap();
            let modulus: u32 = modulus.checked_into().unwrap();
            !n.eq_mod(u, modulus)
        }),
    )
}

pub fn triples_of_integer_signed_and_integer<T: PrimitiveSigned>(
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

pub fn triples_of_signed_integer_and_signed<T: PrimitiveSigned>(
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

pub fn triples_of_integer_unsigned_and_small_unsigned<
    T: PrimitiveUnsigned,
    U: PrimitiveUnsigned,
>(
    gm: GenerationMode,
) -> Box<Iterator<Item = (Integer, T, U)>> {
    match gm {
        GenerationMode::Exhaustive => Box::new(exhaustive_triples(
            exhaustive_integers(),
            exhaustive_unsigned(),
            exhaustive_unsigned(),
        )),
        GenerationMode::Random(scale) => Box::new(random_triples(
            &EXAMPLE_SEED,
            &(|seed| random_integers(seed, scale)),
            &(|seed| random(seed)),
            &(|seed| u32s_geometric(seed, scale).flat_map(U::checked_from)),
        )),
        GenerationMode::SpecialRandom(scale) => Box::new(random_triples(
            &EXAMPLE_SEED,
            &(|seed| special_random_integers(seed, scale)),
            &(|seed| special_random_unsigned(seed)),
            &(|seed| u32s_geometric(seed, scale).flat_map(U::checked_from)),
        )),
    }
}

pub fn rm_triples_of_integer_unsigned_and_small_unsigned<
    T: PrimitiveUnsigned,
    U: PrimitiveUnsigned,
>(
    gm: GenerationMode,
) -> Box<Iterator<Item = ((rug::Integer, T, U), (Integer, T, U))>> {
    Box::new(
        triples_of_integer_unsigned_and_small_unsigned(gm)
            .map(|(x, y, z)| ((integer_to_rug_integer(&x), y, z), (x, y, z))),
    )
}

// All triples of `Integer`, `T`, and small `U`, where `T` and `U` are unsigned and the `Integer` is
// equal to the first `T` mod 2 to the power of the second `T`.
pub fn triples_of_integer_unsigned_and_small_unsigned_var_1<
    T: PrimitiveUnsigned,
    U: PrimitiveUnsigned,
>(
    gm: GenerationMode,
) -> Box<Iterator<Item = (Integer, T, U)>>
where
    Integer: Shl<U, Output = Integer> + Add<T, Output = Integer>,
{
    Box::new(
        triples_of_integer_unsigned_and_small_unsigned(gm)
            .map(|(n, u, pow)| ((n << pow) + u, u, pow)),
    )
}

// All triples of `Integer`, `u32`, and small `T`, where `T` is unsigned and the `Integer` is not
// equal to the `u32` mod 2 to the power of the `T`.
pub fn triples_of_integer_u32_and_small_unsigned_var_2<T: PrimitiveUnsigned>(
    gm: GenerationMode,
) -> Box<Iterator<Item = (Integer, u32, T)>> {
    Box::new(
        triples_of_integer_unsigned_and_small_unsigned::<u32, T>(gm)
            .filter(|&(ref n, u, pow)| !n.eq_mod_power_of_two(u, pow.into())),
    )
}

pub fn triples_of_integer_signed_and_small_unsigned<T: PrimitiveSigned, U: PrimitiveUnsigned>(
    gm: GenerationMode,
) -> Box<Iterator<Item = (Integer, T, U)>> {
    match gm {
        GenerationMode::Exhaustive => Box::new(exhaustive_triples(
            exhaustive_integers(),
            exhaustive_signed(),
            exhaustive_unsigned(),
        )),
        GenerationMode::Random(scale) => Box::new(random_triples(
            &EXAMPLE_SEED,
            &(|seed| random_integers(seed, scale)),
            &(|seed| random(seed)),
            &(|seed| u32s_geometric(seed, scale).flat_map(U::checked_from)),
        )),
        GenerationMode::SpecialRandom(scale) => Box::new(random_triples(
            &EXAMPLE_SEED,
            &(|seed| special_random_integers(seed, scale)),
            &(|seed| special_random_signed(seed)),
            &(|seed| u32s_geometric(seed, scale).flat_map(U::checked_from)),
        )),
    }
}

pub fn rm_triples_of_integer_signed_and_small_unsigned<T: PrimitiveSigned, U: PrimitiveUnsigned>(
    gm: GenerationMode,
) -> Box<Iterator<Item = ((rug::Integer, T, U), (Integer, T, U))>> {
    Box::new(
        triples_of_integer_signed_and_small_unsigned(gm)
            .map(|(x, y, z)| ((integer_to_rug_integer(&x), y, z), (x, y, z))),
    )
}

// All triples of `Integer`, `T`, and small `U`, where `T` is signed, `U` is unsigned, and the
// `Integer` is equal to the `T` mod 2 to the power of the `T`.
pub fn triples_of_integer_signed_and_small_unsigned_var_1<
    T: PrimitiveSigned,
    U: PrimitiveUnsigned,
>(
    gm: GenerationMode,
) -> Box<Iterator<Item = (Integer, T, U)>>
where
    Integer: Shl<U, Output = Integer> + Add<T, Output = Integer>,
{
    Box::new(
        triples_of_integer_signed_and_small_unsigned(gm)
            .map(|(n, u, pow)| ((n << pow) + u, u, pow)),
    )
}

// All triples of `Integer`, `i32`, and small `T`, where `U` is unsigned and the `Integer` is not
// equal to the `i32` mod 2 to the power of the `T`.
pub fn triples_of_integer_i32_and_small_unsigned_var_2<T: PrimitiveUnsigned>(
    gm: GenerationMode,
) -> Box<Iterator<Item = (Integer, i32, T)>> {
    Box::new(
        triples_of_integer_signed_and_small_unsigned::<i32, T>(gm)
            .filter(|&(ref n, i, pow)| !n.eq_mod_power_of_two(i, pow.into())),
    )
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

pub fn rm_pairs_of_natural_and_natural_integer(
    gm: GenerationMode,
) -> Box<Iterator<Item = ((rug::Integer, rug::Integer), (Natural, Integer))>> {
    Box::new(pairs_of_natural_and_natural_integer(gm).map(|(x, y)| {
        (
            (natural_to_rug_integer(&x), integer_to_rug_integer(&y)),
            (x, y),
        )
    }))
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

pub fn triples_of_integer_integer_and_small_unsigned<T: PrimitiveUnsigned>(
    gm: GenerationMode,
) -> Box<Iterator<Item = (Integer, Integer, T)>> {
    match gm {
        GenerationMode::Exhaustive => Box::new(exhaustive_triples(
            exhaustive_integers(),
            exhaustive_integers(),
            exhaustive_unsigned(),
        )),
        GenerationMode::Random(scale) => Box::new(random_triples(
            &EXAMPLE_SEED,
            &(|seed| random_integers(seed, scale)),
            &(|seed| random_integers(seed, scale)),
            &(|seed| u32s_geometric(seed, scale).flat_map(T::checked_from)),
        )),
        GenerationMode::SpecialRandom(scale) => Box::new(random_triples(
            &EXAMPLE_SEED,
            &(|seed| special_random_integers(seed, scale)),
            &(|seed| special_random_integers(seed, scale)),
            &(|seed| u32s_geometric(seed, scale).flat_map(T::checked_from)),
        )),
    }
}

pub fn rm_triples_of_integer_integer_and_small_unsigned<T: PrimitiveUnsigned>(
    gm: GenerationMode,
) -> Box<Iterator<Item = ((rug::Integer, rug::Integer, T), (Integer, Integer, T))>> {
    Box::new(
        triples_of_integer_integer_and_small_unsigned(gm).map(|(x, y, z)| {
            (
                (integer_to_rug_integer(&x), integer_to_rug_integer(&y), z),
                (x, y, z),
            )
        }),
    )
}

// All triples of `Integer`, `Integer`, and small `T`, where `T` is unsigned and the `Integer`s are
// equal mod 2 to the power of the `T`.
pub fn triples_of_integer_integer_and_small_unsigned_var_1<T: PrimitiveUnsigned>(
    gm: GenerationMode,
) -> Box<Iterator<Item = (Integer, Integer, T)>>
where
    Integer: Shl<T, Output = Integer>,
{
    Box::new(
        triples_of_integer_integer_and_small_unsigned(gm)
            .map(|(x, y, pow)| ((x << pow) + &y, y, pow)),
    )
}

// All triples of `Integer`, `Integer`, and small `T`, where `T` is unsigned and the `Integer`s are
// not equal mod 2 to the power of the `T`.
pub fn triples_of_integer_integer_and_small_unsigned_var_2<T: PrimitiveUnsigned>(
    gm: GenerationMode,
) -> Box<Iterator<Item = (Integer, Integer, T)>> {
    Box::new(
        triples_of_integer_integer_and_small_unsigned::<T>(gm)
            .filter(|&(ref x, ref y, pow)| !x.eq_mod_power_of_two(y, pow.into())),
    )
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

pub fn rm_triples_of_integer_small_u64_and_bool(
    gm: GenerationMode,
) -> Box<Iterator<Item = ((rug::Integer, u64, bool), (Integer, u64, bool))>> {
    Box::new(
        triples_of_integer_small_u64_and_bool(gm)
            .map(|(x, y, z)| ((integer_to_rug_integer(&x), y, z), (x, y, z))),
    )
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

pub fn pairs_of_nonzero_integer_and_rounding_mode(
    gm: GenerationMode,
) -> Box<Iterator<Item = (Integer, RoundingMode)>> {
    match gm {
        GenerationMode::Exhaustive => Box::new(lex_pairs(
            exhaustive_nonzero_integers(),
            exhaustive_rounding_modes(),
        )),
        GenerationMode::Random(scale) => Box::new(random_pairs(
            &EXAMPLE_SEED,
            &(|seed| random_nonzero_integers(seed, scale)),
            &(|seed| random_rounding_modes(seed)),
        )),
        GenerationMode::SpecialRandom(scale) => Box::new(random_pairs(
            &EXAMPLE_SEED,
            &(|seed| special_random_nonzero_integers(seed, scale)),
            &(|seed| random_rounding_modes(seed)),
        )),
    }
}

fn triples_of_integer_small_signed_and_rounding_mode<T: PrimitiveSigned>(
    gm: GenerationMode,
) -> Box<Iterator<Item = (Integer, T, RoundingMode)>> {
    match gm {
        GenerationMode::Exhaustive => reshape_2_1_to_3(Box::new(lex_pairs(
            log_pairs_of_integer_and_signed(),
            exhaustive_rounding_modes(),
        ))),
        GenerationMode::Random(scale) => Box::new(random_triples(
            &EXAMPLE_SEED,
            &(|seed| random_integers(seed, scale)),
            &(|seed| i32s_geometric(seed, scale).flat_map(T::checked_from)),
            &(|seed| random_rounding_modes(seed)),
        )),
        GenerationMode::SpecialRandom(scale) => Box::new(random_triples(
            &EXAMPLE_SEED,
            &(|seed| special_random_integers(seed, scale)),
            &(|seed| i32s_geometric(seed, scale).flat_map(T::checked_from)),
            &(|seed| random_rounding_modes(seed)),
        )),
    }
}

// All triples of `Integer`, small `T`, and `RoundingMode`, where `T` is signed, such that if the
// `T` is negative and the `RoundingMode` is `RoundingMode::Exact`, the `Integer` is divisible by 2
// to the power of the negative of the `T`.
pub fn triples_of_integer_small_signed_and_rounding_mode_var_1<T: PrimitiveSigned>(
    gm: GenerationMode,
) -> Box<Iterator<Item = (Integer, T, RoundingMode)>>
where
    Integer: Shr<T, Output = Integer>,
{
    Box::new(
        triples_of_integer_small_signed_and_rounding_mode::<T>(gm)
            .map(|(n, i, rm)| (if i < T::ZERO { n >> i } else { n }, i, rm)),
    )
}

// All triples of `Integer`, small `T`, and `RoundingMode`, where `T` is signed, such that if the
// `i32` is positive and the `RoundingMode` is `RoundingMode::Exact`, the `Integer` is divisible by
// 2 to the power of the `T`.
pub fn triples_of_integer_small_signed_and_rounding_mode_var_2<T: PrimitiveSigned>(
    gm: GenerationMode,
) -> Box<Iterator<Item = (Integer, T, RoundingMode)>>
where
    Integer: Shl<T, Output = Integer>,
{
    Box::new(
        triples_of_integer_small_signed_and_rounding_mode::<T>(gm)
            .map(|(n, i, rm)| (if i > T::ZERO { n << i } else { n }, i, rm)),
    )
}

fn triples_of_integer_small_unsigned_and_rounding_mode<T: PrimitiveUnsigned>(
    gm: GenerationMode,
) -> Box<Iterator<Item = (Integer, T, RoundingMode)>> {
    match gm {
        GenerationMode::Exhaustive => reshape_2_1_to_3(Box::new(lex_pairs(
            log_pairs_of_integer_and_unsigned(),
            exhaustive_rounding_modes(),
        ))),
        GenerationMode::Random(scale) => Box::new(random_triples(
            &EXAMPLE_SEED,
            &(|seed| random_integers(seed, scale)),
            &(|seed| u32s_geometric(seed, scale).flat_map(T::checked_from)),
            &(|seed| random_rounding_modes(seed)),
        )),
        GenerationMode::SpecialRandom(scale) => Box::new(random_triples(
            &EXAMPLE_SEED,
            &(|seed| special_random_integers(seed, scale)),
            &(|seed| u32s_geometric(seed, scale).flat_map(T::checked_from)),
            &(|seed| random_rounding_modes(seed)),
        )),
    }
}

// All triples of `Integer`, small `T`, and `RoundingMode`, where `T` is unsigned and if the
// `RoundingMode` is `RoundingMode::Exact`, the `Integer` is divisible by 2 to the power of the `T`.
pub fn triples_of_integer_small_unsigned_and_rounding_mode_var_1<T: PrimitiveUnsigned>(
    gm: GenerationMode,
) -> Box<Iterator<Item = (Integer, T, RoundingMode)>>
where
    Integer: Shl<T, Output = Integer>,
{
    Box::new(
        triples_of_integer_small_unsigned_and_rounding_mode::<T>(gm).map(|(n, u, rm)| {
            if rm == RoundingMode::Exact {
                (n << u, u, rm)
            } else {
                (n, u, rm)
            }
        }),
    )
}

struct RandomIntegerAndVecOfBoolVar1 {
    integers: Box<Iterator<Item = Integer>>,
    rng: Box<IsaacRng>,
}

impl Iterator for RandomIntegerAndVecOfBoolVar1 {
    type Item = (Integer, Vec<bool>);

    fn next(&mut self) -> Option<(Integer, Vec<bool>)> {
        let n = self.integers.next().unwrap();
        let mut bools = Vec::new();
        for _ in 0..n.to_twos_complement_limbs_asc().len() {
            bools.push(self.rng.gen::<bool>());
        }
        Some((n, bools))
    }
}

fn random_pairs_of_integer_and_vec_of_bool_var_1(
    seed: &[u32],
    scale: u32,
) -> RandomIntegerAndVecOfBoolVar1 {
    RandomIntegerAndVecOfBoolVar1 {
        integers: Box::new(random_integers(&scramble(seed, "integers"), scale)),
        rng: Box::new(IsaacRng::from_seed(&scramble(seed, "bools"))),
    }
}

fn special_random_pairs_of_integer_and_vec_of_bool_var_1(
    seed: &[u32],
    scale: u32,
) -> RandomIntegerAndVecOfBoolVar1 {
    RandomIntegerAndVecOfBoolVar1 {
        integers: Box::new(special_random_integers(&scramble(seed, "integers"), scale)),
        rng: Box::new(IsaacRng::from_seed(&scramble(seed, "bools"))),
    }
}

// All pairs of `Integer` and `Vec<bool>` where the length of the `Vec` is equal to the twos'
// complement limb count of the `Integer` (including sign extension limbs, if necessary).
pub fn pairs_of_integer_and_vec_of_bool_var_1(
    gm: GenerationMode,
) -> Box<Iterator<Item = (Integer, Vec<bool>)>> {
    match gm {
        GenerationMode::Exhaustive => {
            let f = move |i: &Integer| {
                exhaustive_fixed_size_vecs_from_single(
                    i.to_twos_complement_limbs_asc().len() as u64,
                    exhaustive_bools(),
                )
            };
            Box::new(dependent_pairs(exhaustive_integers(), f))
        }
        GenerationMode::Random(scale) => Box::new(random_pairs_of_integer_and_vec_of_bool_var_1(
            &EXAMPLE_SEED,
            scale,
        )),
        GenerationMode::SpecialRandom(scale) => Box::new(
            special_random_pairs_of_integer_and_vec_of_bool_var_1(&EXAMPLE_SEED, scale),
        ),
    }
}

struct RandomIntegerAndVecOfBoolVar2 {
    integers: Box<Iterator<Item = Integer>>,
    rng: Box<IsaacRng>,
}

impl Iterator for RandomIntegerAndVecOfBoolVar2 {
    type Item = (Integer, Vec<bool>);

    fn next(&mut self) -> Option<(Integer, Vec<bool>)> {
        let n = self.integers.next().unwrap();
        let mut bools = Vec::new();
        for _ in 0..n.to_twos_complement_bits_asc().len() {
            bools.push(self.rng.gen::<bool>());
        }
        Some((n, bools))
    }
}

fn random_pairs_of_integer_and_vec_of_bool_var_2(
    seed: &[u32],
    scale: u32,
) -> RandomIntegerAndVecOfBoolVar2 {
    RandomIntegerAndVecOfBoolVar2 {
        integers: Box::new(random_integers(&scramble(seed, "integers"), scale)),
        rng: Box::new(IsaacRng::from_seed(&scramble(seed, "bools"))),
    }
}

fn special_random_pairs_of_integer_and_vec_of_bool_var_2(
    seed: &[u32],
    scale: u32,
) -> RandomIntegerAndVecOfBoolVar2 {
    RandomIntegerAndVecOfBoolVar2 {
        integers: Box::new(special_random_integers(&scramble(seed, "integers"), scale)),
        rng: Box::new(IsaacRng::from_seed(&scramble(seed, "bools"))),
    }
}

// All pairs of `Integer` and `Vec<bool>` where the length of the `Vec` is equal to the two's
// complement bit count of the `Integer`.
pub fn pairs_of_integer_and_vec_of_bool_var_2(
    gm: GenerationMode,
) -> Box<Iterator<Item = (Integer, Vec<bool>)>> {
    match gm {
        GenerationMode::Exhaustive => {
            let f = move |n: &Integer| {
                exhaustive_fixed_size_vecs_from_single(
                    n.to_twos_complement_bits_asc().len() as u64,
                    exhaustive_bools(),
                )
            };
            Box::new(dependent_pairs(exhaustive_integers(), f))
        }
        GenerationMode::Random(scale) => Box::new(random_pairs_of_integer_and_vec_of_bool_var_2(
            &EXAMPLE_SEED,
            scale,
        )),
        GenerationMode::SpecialRandom(scale) => Box::new(
            special_random_pairs_of_integer_and_vec_of_bool_var_2(&EXAMPLE_SEED, scale),
        ),
    }
}

pub fn quadruples_of_integer_integer_integer_and_small_unsigned<T: PrimitiveUnsigned>(
    gm: GenerationMode,
) -> Box<Iterator<Item = (Integer, Integer, Integer, T)>> {
    match gm {
        GenerationMode::Exhaustive => Box::new(exhaustive_quadruples(
            exhaustive_integers(),
            exhaustive_integers(),
            exhaustive_integers(),
            exhaustive_unsigned(),
        )),
        GenerationMode::Random(scale) => Box::new(random_quadruples(
            &EXAMPLE_SEED,
            &(|seed| random_integers(seed, scale)),
            &(|seed| random_integers(seed, scale)),
            &(|seed| random_integers(seed, scale)),
            &(|seed| u32s_geometric(seed, scale).flat_map(T::checked_from)),
        )),
        GenerationMode::SpecialRandom(scale) => Box::new(random_quadruples(
            &EXAMPLE_SEED,
            &(|seed| special_random_integers(seed, scale)),
            &(|seed| special_random_integers(seed, scale)),
            &(|seed| special_random_integers(seed, scale)),
            &(|seed| u32s_geometric(seed, scale).flat_map(T::checked_from)),
        )),
    }
}

fn triples_of_integer_positive_unsigned_and_rounding_mode<T: PrimitiveUnsigned>(
    gm: GenerationMode,
) -> Box<Iterator<Item = (Integer, T, RoundingMode)>> {
    match gm {
        GenerationMode::Exhaustive => reshape_2_1_to_3(Box::new(lex_pairs(
            exhaustive_pairs(exhaustive_integers(), exhaustive_positive()),
            exhaustive_rounding_modes(),
        ))),
        GenerationMode::Random(scale) => Box::new(random_triples(
            &EXAMPLE_SEED,
            &(|seed| random_integers(seed, scale)),
            &(|seed| random_positive_unsigned(seed)),
            &(|seed| random_rounding_modes(seed)),
        )),
        GenerationMode::SpecialRandom(scale) => Box::new(random_triples(
            &EXAMPLE_SEED,
            &(|seed| special_random_integers(seed, scale)),
            &(|seed| special_random_positive_unsigned(seed)),
            &(|seed| random_rounding_modes(seed)),
        )),
    }
}

// All triples of `Integer`, positive `T`, and `RoundingMode`, where `T` is unsigned and if the
// `RoundingMode` is `RoundingMode::Exact`, the `Integer` is divisible by the `T`.
pub fn triples_of_integer_positive_unsigned_and_rounding_mode_var_1<T: PrimitiveUnsigned>(
    gm: GenerationMode,
) -> Box<Iterator<Item = (Integer, T, RoundingMode)>>
where
    Integer: Mul<T, Output = Integer>,
{
    Box::new(
        triples_of_integer_positive_unsigned_and_rounding_mode::<T>(gm).map(|(n, u, rm)| {
            if rm == RoundingMode::Exact {
                (n * u, u, rm)
            } else {
                (n, u, rm)
            }
        }),
    )
}

fn triples_of_unsigned_nonzero_integer_and_rounding_mode<T: PrimitiveUnsigned>(
    gm: GenerationMode,
) -> Box<Iterator<Item = (T, Integer, RoundingMode)>> {
    match gm {
        GenerationMode::Exhaustive => reshape_2_1_to_3(Box::new(lex_pairs(
            exhaustive_pairs(exhaustive_unsigned(), exhaustive_nonzero_integers()),
            exhaustive_rounding_modes(),
        ))),
        GenerationMode::Random(scale) => Box::new(random_triples(
            &EXAMPLE_SEED,
            &(|seed| random(seed)),
            &(|seed| random_nonzero_integers(seed, scale)),
            &(|seed| random_rounding_modes(seed)),
        )),
        GenerationMode::SpecialRandom(scale) => Box::new(random_triples(
            &EXAMPLE_SEED,
            &(|seed| special_random_unsigned(seed)),
            &(|seed| special_random_nonzero_integers(seed, scale)),
            &(|seed| random_rounding_modes(seed)),
        )),
    }
}

// All triples of `T`, nonzero `Integer`, and `RoundingMode`, where `T` is unsigned and if the
// `RoundingMode` is `RoundingMode::Exact`, the `T` is divisible by the `Integer`.
pub fn triples_of_unsigned_nonzero_integer_and_rounding_mode_var_1<T: PrimitiveUnsigned>(
    gm: GenerationMode,
) -> Box<Iterator<Item = (T, Integer, RoundingMode)>>
where
    T: Mul<Integer, Output = Integer>,
    T: CheckedFrom<Integer>,
{
    Box::new(
        triples_of_unsigned_nonzero_integer_and_rounding_mode::<T>(gm).filter_map(|(u, n, rm)| {
            if rm == RoundingMode::Exact {
                T::checked_from(u * (&n).abs()).map(|u| (u, n, rm))
            } else {
                Some((u, n, rm))
            }
        }),
    )
}
