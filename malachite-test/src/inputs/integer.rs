use common::{integer_to_bigint, integer_to_rug_integer, natural_to_rug_integer, GenerationMode};
use inputs::common::{reshape_1_2_to_3, reshape_2_1_to_3};
use malachite_base::misc::CheckedFrom;
use malachite_base::num::{
    DivisibleByPowerOfTwo, PrimitiveInteger, PrimitiveSigned, PrimitiveUnsigned, UnsignedAbs,
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
    exhaustive_integers, exhaustive_natural_integers, random_integers, random_natural_integers,
    special_random_integers, special_random_natural_integers,
};
use rust_wheels::iterators::integers_geometric::{i32s_geometric, u32s_geometric};
use rust_wheels::iterators::naturals::{
    exhaustive_naturals, random_naturals, special_random_naturals,
};
use rust_wheels::iterators::primitive_ints::{
    exhaustive_natural_signed, exhaustive_signed, exhaustive_unsigned, random_natural_signed,
    special_random_natural_signed, special_random_signed, special_random_unsigned,
};
use rust_wheels::iterators::rounding_modes::{exhaustive_rounding_modes, random_rounding_modes};
use rust_wheels::iterators::tuples::{
    exhaustive_pairs, exhaustive_pairs_from_single, exhaustive_triples,
    exhaustive_triples_from_single, lex_pairs, log_pairs, random_pairs, random_pairs_from_single,
    random_triples, random_triples_from_single,
};
use rust_wheels::iterators::vecs::exhaustive_fixed_size_vecs_from_single;
use std::ops::Shl;

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

// All pairs of `Natural` and `T` where `T` is unsigned and the `Natural` is not divisible by 2 to
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

// All triples of `Integer`, `T`, and `RoundingMode`, where `T` is signed, such that if the `T` is
// negative and the `RoundingMode` is `RoundingMode::Exact`, the `Integer` is divisible by 2 to the
// power of the negative of the `T`.
pub fn triples_of_integer_small_signed_and_rounding_mode_var_1<T: PrimitiveSigned>(
    gm: GenerationMode,
) -> Box<Iterator<Item = (Integer, T, RoundingMode)>>
where
    u64: From<<T as UnsignedAbs>::Output>,
{
    Box::new(
        triples_of_integer_small_signed_and_rounding_mode::<T>(gm).filter(|&(ref n, i, rm)| {
            i >= T::ZERO
                || rm != RoundingMode::Exact
                || n.divisible_by_power_of_two(i.unsigned_abs().into())
        }),
    )
}

// All triples of `Integer`, `T`, and `RoundingMode`, where `T` is signed, such that if the `i32` is
// positive and the `RoundingMode` is `RoundingMode::Exact`, the `Integer` is divisible by 2 to the
// power of the `T`.
pub fn triples_of_integer_small_signed_and_rounding_mode_var_2<T: PrimitiveSigned>(
    gm: GenerationMode,
) -> Box<Iterator<Item = (Integer, T, RoundingMode)>>
where
    u64: CheckedFrom<T>,
{
    Box::new(
        triples_of_integer_small_signed_and_rounding_mode::<T>(gm).filter(|&(ref n, i, rm)| {
            i <= T::ZERO
                || rm != RoundingMode::Exact
                || n.divisible_by_power_of_two(u64::checked_from(i).unwrap())
        }),
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

// All triples of `Integer`, `T`, and `RoundingMode`, where `T` is unsigned and if the
// `RoundingMode` is `RoundingMode::Exact`, the `Integer` is divisible by 2 to the power of the `T`.
pub fn triples_of_integer_small_unsigned_and_rounding_mode_var_1<T: PrimitiveUnsigned>(
    gm: GenerationMode,
) -> Box<Iterator<Item = (Integer, T, RoundingMode)>> {
    Box::new(
        triples_of_integer_small_unsigned_and_rounding_mode::<T>(gm).filter(|&(ref n, u, rm)| {
            rm != RoundingMode::Exact || n.divisible_by_power_of_two(u.checked_into().unwrap())
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
