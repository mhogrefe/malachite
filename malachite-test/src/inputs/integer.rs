use std::ops::{Add, Mul, Shl, Shr};

use itertools::Itertools;
use malachite_base::conversion::{CheckedFrom, RoundingFrom};
use malachite_base::crement::Crementable;
use malachite_base::num::floats::PrimitiveFloat;
use malachite_base::num::integers::PrimitiveInteger;
use malachite_base::num::signeds::PrimitiveSigned;
use malachite_base::num::traits::{
    Abs, DivisibleBy, DivisibleByPowerOfTwo, EqMod, EqModPowerOfTwo,
};
use malachite_base::num::unsigneds::PrimitiveUnsigned;
use malachite_base::round::RoundingMode;
use malachite_nz::integer::Integer;
use malachite_nz::natural::Natural;
use malachite_nz::platform::{Limb, SignedLimb};
use num::BigInt;
use rand::{IsaacRng, Rand, Rng, SeedableRng};
use rug;
use rust_wheels::iterators::bools::exhaustive_bools;
use rust_wheels::iterators::common::{scramble, EXAMPLE_SEED};
use rust_wheels::iterators::dependent_pairs::dependent_pairs;
use rust_wheels::iterators::general::{random, Random};
use rust_wheels::iterators::integers::{
    exhaustive_integers, exhaustive_natural_integers, exhaustive_nonzero_integers, random_integers,
    random_natural_integers, random_nonzero_integers, range_down_decreasing_integer,
    range_up_increasing_integer, special_random_integers, special_random_natural_integers,
    special_random_nonzero_integers,
};
use rust_wheels::iterators::integers_geometric::{i32s_geometric, u32s_geometric};
use rust_wheels::iterators::naturals::{
    exhaustive_naturals, random_naturals, random_range_up_natural, special_random_naturals,
    special_random_range_up_natural,
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

use common::{integer_to_bigint, integer_to_rug_integer, natural_to_rug_integer, GenerationMode};
use inputs::base::{finite_f32s, finite_f64s, It};
use inputs::common::{reshape_1_2_to_3, reshape_2_1_to_3};

pub fn integers(gm: GenerationMode) -> It<Integer> {
    match gm {
        GenerationMode::Exhaustive => Box::new(exhaustive_integers()),
        GenerationMode::Random(scale) => Box::new(random_integers(&EXAMPLE_SEED, scale)),
        GenerationMode::SpecialRandom(scale) => {
            Box::new(special_random_integers(&EXAMPLE_SEED, scale))
        }
    }
}

pub fn rm_integers(gm: GenerationMode) -> It<(rug::Integer, Integer)> {
    Box::new(integers(gm).map(|n| (integer_to_rug_integer(&n), n)))
}

pub fn nm_integers(gm: GenerationMode) -> It<(BigInt, Integer)> {
    Box::new(integers(gm).map(|n| (integer_to_bigint(&n), n)))
}

pub fn nrm_integers(gm: GenerationMode) -> It<(BigInt, rug::Integer, Integer)> {
    Box::new(integers(gm).map(|n| (integer_to_bigint(&n), integer_to_rug_integer(&n), n)))
}

pub fn nonzero_integers(gm: GenerationMode) -> It<Integer> {
    match gm {
        GenerationMode::Exhaustive => Box::new(exhaustive_nonzero_integers()),
        GenerationMode::Random(scale) => Box::new(random_nonzero_integers(&EXAMPLE_SEED, scale)),
        GenerationMode::SpecialRandom(scale) => {
            Box::new(special_random_nonzero_integers(&EXAMPLE_SEED, scale))
        }
    }
}

pub fn pairs_of_integers(gm: GenerationMode) -> It<(Integer, Integer)> {
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
) -> It<((rug::Integer, rug::Integer), (Integer, Integer))> {
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

pub fn triples_of_integers(gm: GenerationMode) -> It<(Integer, Integer, Integer)> {
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

pub fn natural_integers(gm: GenerationMode) -> It<Integer> {
    match gm {
        GenerationMode::Exhaustive => Box::new(exhaustive_natural_integers()),
        GenerationMode::Random(scale) => Box::new(random_natural_integers(&EXAMPLE_SEED, scale)),
        GenerationMode::SpecialRandom(scale) => {
            Box::new(special_random_natural_integers(&EXAMPLE_SEED, scale))
        }
    }
}

pub fn triples_of_natural_integers(gm: GenerationMode) -> It<(Integer, Integer, Integer)> {
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

fn random_pairs_of_integer_and_primitive<T: PrimitiveInteger + Rand>(
    scale: u32,
) -> It<(Integer, T)> {
    Box::new(random_pairs(
        &EXAMPLE_SEED,
        &(|seed| random_integers(seed, scale)),
        &(|seed| random(seed)),
    ))
}

fn random_pairs_of_primitive_and_integer<T: PrimitiveInteger + Rand>(
    scale: u32,
) -> It<(T, Integer)> {
    Box::new(random_pairs(
        &EXAMPLE_SEED,
        &(|seed| random(seed)),
        &(|seed| random_integers(seed, scale)),
    ))
}

pub fn pairs_of_integer_and_signed<T: PrimitiveSigned + Rand>(
    gm: GenerationMode,
) -> It<(Integer, T)>
where
    T::UnsignedOfEqualWidth: Rand,
{
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

pub fn nm_pairs_of_integer_and_signed<T: PrimitiveSigned + Rand>(
    gm: GenerationMode,
) -> It<((BigInt, T), (Integer, T))>
where
    T::UnsignedOfEqualWidth: Rand,
{
    Box::new(pairs_of_integer_and_signed(gm).map(|(x, y)| ((integer_to_bigint(&x), y), (x, y))))
}

pub fn rm_pairs_of_integer_and_signed<T: PrimitiveSigned + Rand>(
    gm: GenerationMode,
) -> It<((rug::Integer, T), (Integer, T))>
where
    T::UnsignedOfEqualWidth: Rand,
{
    Box::new(
        pairs_of_integer_and_signed(gm).map(|(x, y)| ((integer_to_rug_integer(&x), y), (x, y))),
    )
}

pub fn nrm_pairs_of_integer_and_signed<T: PrimitiveSigned + Rand>(
    gm: GenerationMode,
) -> It<((BigInt, T), (rug::Integer, T), (Integer, T))>
where
    T::UnsignedOfEqualWidth: Rand,
{
    Box::new(pairs_of_integer_and_signed(gm).map(|(x, y)| {
        (
            (integer_to_bigint(&x), y),
            (integer_to_rug_integer(&x), y),
            (x, y),
        )
    }))
}

pub fn pairs_of_signed_and_integer<T: PrimitiveSigned + Rand>(
    gm: GenerationMode,
) -> It<(T, Integer)>
where
    T::UnsignedOfEqualWidth: Rand,
{
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

pub fn rm_pairs_of_signed_and_integer<T: PrimitiveSigned + Rand>(
    gm: GenerationMode,
) -> It<((T, rug::Integer), (T, Integer))>
where
    T::UnsignedOfEqualWidth: Rand,
{
    Box::new(
        pairs_of_signed_and_integer(gm).map(|(x, y)| ((x, integer_to_rug_integer(&y)), (x, y))),
    )
}

pub fn pairs_of_integer_and_unsigned<T: PrimitiveUnsigned + Rand>(
    gm: GenerationMode,
) -> It<(Integer, T)> {
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

pub fn rm_pairs_of_integer_and_unsigned<T: PrimitiveUnsigned + Rand>(
    gm: GenerationMode,
) -> It<((rug::Integer, T), (Integer, T))> {
    Box::new(
        pairs_of_integer_and_unsigned(gm).map(|(x, y)| ((integer_to_rug_integer(&x), y), (x, y))),
    )
}

pub fn nm_pairs_of_integer_and_unsigned<T: PrimitiveUnsigned + Rand>(
    gm: GenerationMode,
) -> It<((BigInt, T), (Integer, T))> {
    Box::new(pairs_of_integer_and_unsigned(gm).map(|(x, y)| ((integer_to_bigint(&x), y), (x, y))))
}

pub fn nrm_pairs_of_integer_and_unsigned<T: PrimitiveUnsigned + Rand>(
    gm: GenerationMode,
) -> It<((BigInt, T), (rug::Integer, T), (Integer, T))> {
    Box::new(pairs_of_integer_and_unsigned(gm).map(|(x, y)| {
        (
            (integer_to_bigint(&x), y),
            (integer_to_rug_integer(&x), y),
            (x, y),
        )
    }))
}

pub fn pairs_of_integer_and_positive_unsigned<T: PrimitiveUnsigned + Rand>(
    gm: GenerationMode,
) -> It<(Integer, T)> {
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

pub fn rm_pairs_of_integer_and_positive_unsigned<T: PrimitiveUnsigned + Rand>(
    gm: GenerationMode,
) -> It<((rug::Integer, T), (Integer, T))> {
    Box::new(
        pairs_of_integer_and_positive_unsigned(gm)
            .map(|(x, y)| ((integer_to_rug_integer(&x), y), (x, y))),
    )
}

pub fn nrm_pairs_of_integer_and_positive_unsigned<T: PrimitiveUnsigned + Rand>(
    gm: GenerationMode,
) -> It<((BigInt, T), (rug::Integer, T), (Integer, T))> {
    Box::new(pairs_of_integer_and_positive_unsigned(gm).map(|(x, y)| {
        (
            (integer_to_bigint(&x), y),
            (integer_to_rug_integer(&x), y),
            (x, y),
        )
    }))
}

pub fn nm_pairs_of_integer_and_positive_unsigned<T: PrimitiveUnsigned + Rand>(
    gm: GenerationMode,
) -> It<((BigInt, T), (Integer, T))> {
    Box::new(
        pairs_of_integer_and_positive_unsigned(gm)
            .map(|(x, y)| ((integer_to_bigint(&x), y), (x, y))),
    )
}

pub fn pairs_of_integer_and_nonzero_signed<T: PrimitiveSigned + Rand>(
    gm: GenerationMode,
) -> It<(Integer, T)>
where
    T::UnsignedOfEqualWidth: Rand,
{
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

pub fn rm_pairs_of_integer_and_nonzero_signed<T: PrimitiveSigned + Rand>(
    gm: GenerationMode,
) -> It<((rug::Integer, T), (Integer, T))>
where
    T::UnsignedOfEqualWidth: Rand,
{
    Box::new(
        pairs_of_integer_and_nonzero_signed(gm)
            .map(|(x, y)| ((integer_to_rug_integer(&x), y), (x, y))),
    )
}

pub fn nrm_pairs_of_integer_and_nonzero_signed<T: PrimitiveSigned + Rand>(
    gm: GenerationMode,
) -> It<((BigInt, T), (rug::Integer, T), (Integer, T))>
where
    T::UnsignedOfEqualWidth: Rand,
{
    Box::new(pairs_of_integer_and_nonzero_signed(gm).map(|(x, y)| {
        (
            (integer_to_bigint(&x), y),
            (integer_to_rug_integer(&x), y),
            (x, y),
        )
    }))
}

pub fn nm_pairs_of_integer_and_nonzero_signed<T: PrimitiveSigned + Rand>(
    gm: GenerationMode,
) -> It<((BigInt, T), (Integer, T))>
where
    T::UnsignedOfEqualWidth: Rand,
{
    Box::new(
        pairs_of_integer_and_nonzero_signed(gm).map(|(x, y)| ((integer_to_bigint(&x), y), (x, y))),
    )
}

// All triples of `Integer` and positive `Limb` where the `Integer` is divisible by the `T`.
pub fn pairs_of_integer_and_positive_limb_var_1(gm: GenerationMode) -> It<(Integer, Limb)> {
    Box::new(pairs_of_integer_and_positive_unsigned(gm).map(|(n, u)| (n * u, u)))
}

pub fn nrm_pairs_of_integer_and_positive_limb_var_1(
    gm: GenerationMode,
) -> It<((BigInt, Limb), (rug::Integer, Limb), (Integer, Limb))> {
    Box::new(pairs_of_integer_and_positive_limb_var_1(gm).map(|(x, y)| {
        (
            (integer_to_bigint(&x), y),
            (integer_to_rug_integer(&x), y),
            (x, y),
        )
    }))
}

// All pairs of `Integer` and positive `Limb`, where the `Integer` is not divisible by the `Limb`.
pub fn pairs_of_integer_and_positive_limb_var_2(gm: GenerationMode) -> It<(Integer, Limb)> {
    Box::new(pairs_of_integer_and_positive_unsigned(gm).filter(|&(ref n, u)| !n.divisible_by(u)))
}

pub fn pairs_of_unsigned_and_integer<T: PrimitiveUnsigned + Rand>(
    gm: GenerationMode,
) -> It<(T, Integer)> {
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

// All triples of `Integer` and nonzero `T` where `T` is signed and the `Integer` is divisible by
// the `T`.
pub fn pairs_of_integer_and_nonzero_signed_limb_var_1<T: PrimitiveSigned + Rand>(
    gm: GenerationMode,
) -> It<(Integer, T)>
where
    T::UnsignedOfEqualWidth: Rand,
    Integer: Mul<T, Output = Integer>,
{
    Box::new(pairs_of_integer_and_nonzero_signed(gm).map(|(n, i)| (n * i, i)))
}

pub fn nrm_pairs_of_integer_and_nonzero_signed_limb_var_1<T: PrimitiveSigned + Rand>(
    gm: GenerationMode,
) -> It<((BigInt, T), (rug::Integer, T), (Integer, T))>
where
    T::UnsignedOfEqualWidth: Rand,
    Integer: Mul<T, Output = Integer>,
{
    Box::new(
        pairs_of_integer_and_nonzero_signed_limb_var_1(gm).map(|(x, y)| {
            (
                (integer_to_bigint(&x), y),
                (integer_to_rug_integer(&x), y),
                (x, y),
            )
        }),
    )
}

// All pairs of `Integer` and nonzero `SignedLimb`, where the `Integer` is not divisible by the
// `SignedLimb`.
pub fn pairs_of_integer_and_nonzero_signed_limb_var_2(
    gm: GenerationMode,
) -> It<(Integer, SignedLimb)> {
    Box::new(pairs_of_integer_and_nonzero_signed(gm).filter(|&(ref n, i)| !n.divisible_by(i)))
}

pub fn nrm_pairs_of_unsigned_and_integer<T: PrimitiveUnsigned + Rand>(
    gm: GenerationMode,
) -> It<((T, BigInt), (T, rug::Integer), (T, Integer))> {
    Box::new(pairs_of_unsigned_and_integer(gm).map(|(x, y)| {
        (
            (x, integer_to_bigint(&y)),
            (x, integer_to_rug_integer(&y)),
            (x, y),
        )
    }))
}

pub fn rm_pairs_of_unsigned_and_integer<T: PrimitiveUnsigned + Rand>(
    gm: GenerationMode,
) -> It<((T, rug::Integer), (T, Integer))> {
    Box::new(
        pairs_of_unsigned_and_integer(gm).map(|(x, y)| ((x, integer_to_rug_integer(&y)), (x, y))),
    )
}

pub fn pairs_of_unsigned_and_nonzero_integer<T: PrimitiveUnsigned + Rand>(
    gm: GenerationMode,
) -> It<(T, Integer)> {
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

// All pairs of `Limb` and positive `Integer` where the `Limb` is not divisible by the `Integer`.
pub fn pairs_of_limb_and_nonzero_integer_var_1(gm: GenerationMode) -> It<(Limb, Integer)> {
    Box::new(
        pairs_of_unsigned_and_nonzero_integer::<Limb>(gm).filter(|&(u, ref n)| !u.divisible_by(n)),
    )
}

// All pairs of `Limb` and nonzero `Integer` where the `Limb` is divisible by the `Integer`.
pub fn pairs_of_limb_and_nonzero_integer_var_2(gm: GenerationMode) -> It<(Limb, Integer)> {
    Box::new(
        pairs_of_unsigned_and_nonzero_integer::<Limb>(gm)
            .filter_map(|(u, n)| Limb::checked_from(u * (&n).abs()).map(|u| (u, n))),
    )
}

pub fn pairs_of_signed_and_nonzero_integer<T: PrimitiveSigned + Rand>(
    gm: GenerationMode,
) -> It<(T, Integer)>
where
    T::UnsignedOfEqualWidth: Rand,
{
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

// All pairs of `SignedLimb` and positive `Integer` where the `SignedLimb` is not divisible by the
// `Integer`.
pub fn pairs_of_signed_limb_and_nonzero_integer_var_1(
    gm: GenerationMode,
) -> It<(SignedLimb, Integer)> {
    Box::new(
        pairs_of_signed_and_nonzero_integer::<SignedLimb>(gm)
            .filter(|&(i, ref n)| !i.divisible_by(n)),
    )
}

// All pairs of `SignedLimb` and nonzero `Integer` where the `SignedLimb` is divisible by the
// `Integer`.
pub fn pairs_of_signed_limb_and_nonzero_integer_var_2(
    gm: GenerationMode,
) -> It<(SignedLimb, Integer)> {
    Box::new(
        pairs_of_signed_and_nonzero_integer::<SignedLimb>(gm)
            .filter_map(|(i, n)| SignedLimb::checked_from(i * (&n).abs()).map(|i| (i, n))),
    )
}

pub fn pairs_of_natural_integer_and_unsigned<T: PrimitiveUnsigned + Rand>(
    gm: GenerationMode,
) -> It<(Integer, T)> {
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

fn random_triples_of_integer_integer_and_primitive<T: PrimitiveInteger + Rand>(
    scale: u32,
) -> It<(Integer, Integer, T)> {
    Box::new(random_triples(
        &EXAMPLE_SEED,
        &(|seed| random_integers(seed, scale)),
        &(|seed| random_integers(seed, scale)),
        &(|seed| random(seed)),
    ))
}

pub fn triples_of_integer_integer_and_signed<T: PrimitiveSigned + Rand>(
    gm: GenerationMode,
) -> It<(Integer, Integer, T)>
where
    T::UnsignedOfEqualWidth: Rand,
{
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

pub fn triples_of_integer_integer_and_unsigned<T: PrimitiveUnsigned + Rand>(
    gm: GenerationMode,
) -> It<(Integer, Integer, T)> {
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

pub fn triples_of_natural_integer_unsigned_and_unsigned<T: PrimitiveUnsigned + Rand>(
    gm: GenerationMode,
) -> It<(Integer, T, T)> {
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

pub fn triples_of_natural_integer_natural_signed_and_natural_signed<T: PrimitiveSigned + Rand>(
    gm: GenerationMode,
) -> It<(Integer, T, T)>
where
    T::UnsignedOfEqualWidth: Rand,
{
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

pub fn triples_of_natural_integer_natural_integer_and_natural_signed<T: PrimitiveSigned + Rand>(
    gm: GenerationMode,
) -> It<(Integer, Integer, T)>
where
    T::UnsignedOfEqualWidth: Rand,
{
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

fn log_pairs_of_integer_and_unsigned<T: PrimitiveUnsigned>() -> It<(Integer, T)> {
    Box::new(log_pairs(exhaustive_integers(), exhaustive_unsigned()))
}

pub fn pairs_of_integer_and_small_unsigned<T: PrimitiveUnsigned + Rand>(
    gm: GenerationMode,
) -> It<(Integer, T)> {
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

pub fn rm_pairs_of_integer_and_small_unsigned<T: PrimitiveUnsigned + Rand>(
    gm: GenerationMode,
) -> It<((rug::Integer, T), (Integer, T))> {
    Box::new(
        pairs_of_integer_and_small_unsigned(gm)
            .map(|(x, y)| ((integer_to_rug_integer(&x), y), (x, y))),
    )
}

// All pairs of `Integer` and `T` where `T` is unsigned and the `Integer` is divisible by 2 to the
// power of the `T`.
pub fn pairs_of_integer_and_small_unsigned_var_1<T: PrimitiveUnsigned + Rand>(
    gm: GenerationMode,
) -> It<(Integer, T)>
where
    Integer: Shl<T, Output = Integer>,
{
    Box::new(pairs_of_integer_and_small_unsigned::<T>(gm).map(|(n, u)| (n << u, u)))
}

// All pairs of `Integer` and `T` where `T` is unsigned and the `Integer` is not divisible by 2 to
// the power of the `T`.
pub fn pairs_of_integer_and_small_unsigned_var_2<T: PrimitiveUnsigned + Rand>(
    gm: GenerationMode,
) -> It<(Integer, T)> {
    Box::new(
        pairs_of_integer_and_small_unsigned::<T>(gm)
            .filter(|&(ref n, u)| !n.divisible_by_power_of_two(u.checked_into().unwrap())),
    )
}

pub fn pairs_of_integer_and_small_u64(gm: GenerationMode) -> It<(Integer, u64)> {
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
) -> It<((rug::Integer, u64), (Integer, u64))> {
    Box::new(
        pairs_of_integer_and_small_u64(gm).map(|(x, y)| ((integer_to_rug_integer(&x), y), (x, y))),
    )
}

pub fn nm_pairs_of_integer_and_small_u64(
    gm: GenerationMode,
) -> It<((BigInt, u64), (Integer, u64))> {
    Box::new(pairs_of_integer_and_small_u64(gm).map(|(x, y)| ((integer_to_bigint(&x), y), (x, y))))
}

pub fn nrm_pairs_of_integer_and_small_u64(
    gm: GenerationMode,
) -> It<((BigInt, u64), (rug::Integer, u64), (Integer, u64))> {
    Box::new(pairs_of_integer_and_small_u64(gm).map(|(x, y)| {
        (
            (integer_to_bigint(&x), y),
            (integer_to_rug_integer(&x), y),
            (x, y),
        )
    }))
}

pub fn pairs_of_integer_and_small_usize(gm: GenerationMode) -> It<(Integer, usize)> {
    Box::new(pairs_of_integer_and_small_unsigned::<u64>(gm).map(|(n, u)| (n, u as usize)))
}

pub fn triples_of_integer_small_unsigned_and_small_unsigned<
    T: PrimitiveUnsigned + Rand,
    U: PrimitiveUnsigned + Rand,
>(
    gm: GenerationMode,
) -> It<(Integer, T, U)> {
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

pub fn triples_of_integer_integer_and_positive_unsigned<T: PrimitiveUnsigned + Rand>(
    gm: GenerationMode,
) -> It<(Integer, Integer, T)> {
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

pub fn triples_of_integer_integer_and_nonzero_signed<T: PrimitiveSigned + Rand>(
    gm: GenerationMode,
) -> It<(Integer, Integer, T)>
where
    T::UnsignedOfEqualWidth: Rand,
{
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

fn log_pairs_of_integer_and_signed<T: PrimitiveSigned>() -> It<(Integer, T)> {
    Box::new(log_pairs(exhaustive_integers(), exhaustive_signed()))
}

pub fn pairs_of_integer_and_small_signed<T: PrimitiveSigned + Rand>(
    gm: GenerationMode,
) -> It<(Integer, T)> {
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

pub fn rm_pairs_of_integer_and_small_signed<T: PrimitiveSigned + Rand>(
    gm: GenerationMode,
) -> It<((rug::Integer, T), (Integer, T))> {
    Box::new(
        pairs_of_integer_and_small_signed(gm)
            .map(|(x, y)| ((integer_to_rug_integer(&x), y), (x, y))),
    )
}

fn random_triples_of_integer_primitive_and_integer<T: PrimitiveInteger + Rand>(
    scale: u32,
) -> It<(Integer, T, Integer)> {
    Box::new(random_triples(
        &EXAMPLE_SEED,
        &(|seed| random_integers(seed, scale)),
        &(|seed| random(seed)),
        &(|seed| random_integers(seed, scale)),
    ))
}

fn random_triples_of_primitive_integer_and_primitive<T: PrimitiveInteger + Rand>(
    scale: u32,
) -> It<(T, Integer, T)> {
    Box::new(random_triples(
        &EXAMPLE_SEED,
        &(|seed| random(seed)),
        &(|seed| random_integers(seed, scale)),
        &(|seed| random(seed)),
    ))
}

fn random_triples_of_primitive_primitive_and_integer<T: PrimitiveInteger + Rand>(
    scale: u32,
) -> It<(T, T, Integer)> {
    Box::new(random_triples(
        &EXAMPLE_SEED,
        &(|seed| random(seed)),
        &(|seed| random(seed)),
        &(|seed| random_integers(seed, scale)),
    ))
}

fn random_triples_of_integer_primitive_and_primitive<T: PrimitiveInteger + Rand>(
    scale: u32,
) -> It<(Integer, T, T)> {
    Box::new(random_triples(
        &EXAMPLE_SEED,
        &(|seed| random_integers(seed, scale)),
        &(|seed| random(seed)),
        &(|seed| random(seed)),
    ))
}

pub fn triples_of_integer_unsigned_and_integer<T: PrimitiveUnsigned + Rand>(
    gm: GenerationMode,
) -> It<(Integer, T, Integer)> {
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

pub fn triples_of_unsigned_integer_and_unsigned<T: PrimitiveUnsigned + Rand>(
    gm: GenerationMode,
) -> It<(T, Integer, T)> {
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

pub fn triples_of_integer_unsigned_and_unsigned<T: PrimitiveUnsigned + Rand>(
    gm: GenerationMode,
) -> It<(Integer, T, T)> {
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

pub fn rm_triples_of_integer_unsigned_and_unsigned<T: PrimitiveUnsigned + Rand>(
    gm: GenerationMode,
) -> It<((rug::Integer, T, T), (Integer, T, T))> {
    Box::new(
        triples_of_integer_unsigned_and_unsigned(gm)
            .map(|(x, y, z)| ((integer_to_rug_integer(&x), y, z), (x, y, z))),
    )
}

// All triples of `Integer`, `T`, `T`, where `T` is unsigned and the `Integer` is equal to the first
// `T` mod the second `T`.
pub fn triples_of_integer_unsigned_and_unsigned_var_1<T: PrimitiveUnsigned + Rand>(
    gm: GenerationMode,
) -> It<(Integer, T, T)>
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
pub fn triples_of_integer_unsigned_and_unsigned_var_2<T: PrimitiveUnsigned + Rand>(
    gm: GenerationMode,
) -> It<(Integer, T, T)> {
    Box::new(
        triples_of_integer_unsigned_and_unsigned::<T>(gm).filter(|&(ref n, u, modulus)| {
            let u: Limb = u.checked_into().unwrap();
            let modulus: Limb = modulus.checked_into().unwrap();
            !n.eq_mod(u, modulus)
        }),
    )
}

pub fn triples_of_unsigned_unsigned_and_integer<T: PrimitiveUnsigned + Rand>(
    gm: GenerationMode,
) -> It<(T, T, Integer)> {
    match gm {
        GenerationMode::Exhaustive => Box::new(exhaustive_triples(
            exhaustive_unsigned(),
            exhaustive_unsigned(),
            exhaustive_integers(),
        )),
        GenerationMode::Random(scale) => random_triples_of_primitive_primitive_and_integer(scale),
        GenerationMode::SpecialRandom(scale) => Box::new(random_triples(
            &EXAMPLE_SEED,
            &(|seed| special_random_unsigned(seed)),
            &(|seed| special_random_unsigned(seed)),
            &(|seed| special_random_integers(seed, scale)),
        )),
    }
}

pub fn triples_of_signed_signed_and_integer<T: PrimitiveSigned + Rand>(
    gm: GenerationMode,
) -> It<(T, T, Integer)>
where
    T::UnsignedOfEqualWidth: Rand,
{
    match gm {
        GenerationMode::Exhaustive => Box::new(exhaustive_triples(
            exhaustive_signed(),
            exhaustive_signed(),
            exhaustive_integers(),
        )),
        GenerationMode::Random(scale) => random_triples_of_primitive_primitive_and_integer(scale),
        GenerationMode::SpecialRandom(scale) => Box::new(random_triples(
            &EXAMPLE_SEED,
            &(|seed| special_random_signed(seed)),
            &(|seed| special_random_signed(seed)),
            &(|seed| special_random_integers(seed, scale)),
        )),
    }
}

pub fn triples_of_integer_signed_and_signed<T: PrimitiveSigned + Rand>(
    gm: GenerationMode,
) -> It<(Integer, T, T)>
where
    T::UnsignedOfEqualWidth: Rand,
{
    match gm {
        GenerationMode::Exhaustive => Box::new(exhaustive_triples(
            exhaustive_integers(),
            exhaustive_signed(),
            exhaustive_signed(),
        )),
        GenerationMode::Random(scale) => random_triples_of_integer_primitive_and_primitive(scale),
        GenerationMode::SpecialRandom(scale) => Box::new(random_triples(
            &EXAMPLE_SEED,
            &(|seed| special_random_integers(seed, scale)),
            &(|seed| special_random_signed(seed)),
            &(|seed| special_random_signed(seed)),
        )),
    }
}

pub fn rm_triples_of_integer_signed_and_signed<T: PrimitiveSigned + Rand>(
    gm: GenerationMode,
) -> It<((rug::Integer, T, T), (Integer, T, T))>
where
    T::UnsignedOfEqualWidth: Rand,
{
    Box::new(
        triples_of_integer_signed_and_signed(gm)
            .map(|(x, y, z)| ((integer_to_rug_integer(&x), y, z), (x, y, z))),
    )
}

// All triples of `Integer`, `T`, `T`, where `T` is signed and the `Integer` is equal to the first
// `T` mod the second `T`.
pub fn triples_of_integer_signed_and_signed_var_1<T: PrimitiveSigned + Rand>(
    gm: GenerationMode,
) -> It<(Integer, T, T)>
where
    Integer: Mul<T, Output = Integer> + Add<T, Output = Integer>,
    T::UnsignedOfEqualWidth: Rand,
{
    Box::new(
        triples_of_integer_signed_and_signed(gm)
            .map(|(n, i, modulus)| (n * modulus + i, i, modulus)),
    )
}

// All triples of `Integer`, `T`, `T`, where `T` is signed and the `Integer` is not equal to the
// first `T` mod the second `T`.
pub fn triples_of_integer_signed_and_signed_var_2<T: PrimitiveSigned + Rand>(
    gm: GenerationMode,
) -> It<(Integer, T, T)>
where
    T::UnsignedOfEqualWidth: Rand,
{
    Box::new(
        triples_of_integer_signed_and_signed::<T>(gm).filter(|&(ref n, i, modulus)| {
            let i: SignedLimb = i.checked_into().unwrap();
            let modulus: SignedLimb = modulus.checked_into().unwrap();
            !n.eq_mod(i, modulus)
        }),
    )
}

pub fn triples_of_integer_signed_and_integer<T: PrimitiveSigned + Rand>(
    gm: GenerationMode,
) -> It<(Integer, T, Integer)>
where
    T::UnsignedOfEqualWidth: Rand,
{
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

pub fn triples_of_signed_integer_and_signed<T: PrimitiveSigned + Rand>(
    gm: GenerationMode,
) -> It<(T, Integer, T)>
where
    T::UnsignedOfEqualWidth: Rand,
{
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
    T: PrimitiveUnsigned + Rand,
    U: PrimitiveUnsigned + Rand,
>(
    gm: GenerationMode,
) -> It<(Integer, T, U)> {
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
    T: PrimitiveUnsigned + Rand,
    U: PrimitiveUnsigned + Rand,
>(
    gm: GenerationMode,
) -> It<((rug::Integer, T, U), (Integer, T, U))> {
    Box::new(
        triples_of_integer_unsigned_and_small_unsigned(gm)
            .map(|(x, y, z)| ((integer_to_rug_integer(&x), y, z), (x, y, z))),
    )
}

// All triples of `Integer`, `T`, and small `U`, where `T` and `U` are unsigned and the `Integer` is
// equal to the first `T` mod 2 to the power of the second `T`.
pub fn triples_of_integer_unsigned_and_small_unsigned_var_1<
    T: PrimitiveUnsigned + Rand,
    U: PrimitiveUnsigned + Rand,
>(
    gm: GenerationMode,
) -> It<(Integer, T, U)>
where
    Integer: Shl<U, Output = Integer> + Add<T, Output = Integer>,
{
    Box::new(
        triples_of_integer_unsigned_and_small_unsigned(gm)
            .map(|(n, u, pow)| ((n << pow) + u, u, pow)),
    )
}

// All triples of `Integer`, `Limb`, and small `T`, where `T` is unsigned and the `Integer` is not
// equal to the `Limb` mod 2 to the power of the `T`.
pub fn triples_of_integer_limb_and_small_unsigned_var_2<T: PrimitiveUnsigned + Rand>(
    gm: GenerationMode,
) -> It<(Integer, Limb, T)> {
    Box::new(
        triples_of_integer_unsigned_and_small_unsigned::<Limb, T>(gm)
            .filter(|&(ref n, u, pow)| !n.eq_mod_power_of_two(u, pow.checked_into().unwrap())),
    )
}

pub fn triples_of_unsigned_integer_and_small_unsigned<
    T: PrimitiveUnsigned + Rand,
    U: PrimitiveUnsigned + Rand,
>(
    gm: GenerationMode,
) -> It<(T, Integer, U)> {
    match gm {
        GenerationMode::Exhaustive => Box::new(exhaustive_triples(
            exhaustive_unsigned(),
            exhaustive_integers(),
            exhaustive_unsigned(),
        )),
        GenerationMode::Random(scale) => Box::new(random_triples(
            &EXAMPLE_SEED,
            &(|seed| random(seed)),
            &(|seed| random_integers(seed, scale)),
            &(|seed| u32s_geometric(seed, scale).flat_map(U::checked_from)),
        )),
        GenerationMode::SpecialRandom(scale) => Box::new(random_triples(
            &EXAMPLE_SEED,
            &(|seed| special_random_unsigned(seed)),
            &(|seed| special_random_integers(seed, scale)),
            &(|seed| u32s_geometric(seed, scale).flat_map(U::checked_from)),
        )),
    }
}

pub fn triples_of_integer_signed_and_small_unsigned<
    T: PrimitiveSigned + Rand,
    U: PrimitiveUnsigned + Rand,
>(
    gm: GenerationMode,
) -> It<(Integer, T, U)>
where
    T::UnsignedOfEqualWidth: Rand,
{
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

pub fn rm_triples_of_integer_signed_and_small_unsigned<
    T: PrimitiveSigned + Rand,
    U: PrimitiveUnsigned + Rand,
>(
    gm: GenerationMode,
) -> It<((rug::Integer, T, U), (Integer, T, U))>
where
    T::UnsignedOfEqualWidth: Rand,
{
    Box::new(
        triples_of_integer_signed_and_small_unsigned(gm)
            .map(|(x, y, z)| ((integer_to_rug_integer(&x), y, z), (x, y, z))),
    )
}

// All triples of `Integer`, `T`, and small `U`, where `T` is signed, `U` is unsigned, and the
// `Integer` is equal to the `T` mod 2 to the power of the `T`.
pub fn triples_of_integer_signed_and_small_unsigned_var_1<
    T: PrimitiveSigned + Rand,
    U: PrimitiveUnsigned + Rand,
>(
    gm: GenerationMode,
) -> It<(Integer, T, U)>
where
    Integer: Shl<U, Output = Integer> + Add<T, Output = Integer>,
    T::UnsignedOfEqualWidth: Rand,
{
    Box::new(
        triples_of_integer_signed_and_small_unsigned(gm)
            .map(|(n, u, pow)| ((n << pow) + u, u, pow)),
    )
}

// All triples of `Integer`, `SignedLimb`, and small `T`, where `U` is unsigned and the `Integer` is
// not equal to the `SignedLimb` mod 2 to the power of the `T`.
pub fn triples_of_integer_signed_limb_and_small_unsigned_var_2<T: PrimitiveUnsigned + Rand>(
    gm: GenerationMode,
) -> It<(Integer, SignedLimb, T)> {
    Box::new(
        triples_of_integer_signed_and_small_unsigned::<SignedLimb, T>(gm)
            .filter(|&(ref n, i, pow)| !n.eq_mod_power_of_two(i, pow.checked_into().unwrap())),
    )
}

pub fn triples_of_signed_integer_and_small_unsigned<
    T: PrimitiveSigned + Rand,
    U: PrimitiveUnsigned + Rand,
>(
    gm: GenerationMode,
) -> It<(T, Integer, U)>
where
    T::UnsignedOfEqualWidth: Rand,
{
    match gm {
        GenerationMode::Exhaustive => Box::new(exhaustive_triples(
            exhaustive_signed(),
            exhaustive_integers(),
            exhaustive_unsigned(),
        )),
        GenerationMode::Random(scale) => Box::new(random_triples(
            &EXAMPLE_SEED,
            &(|seed| random(seed)),
            &(|seed| random_integers(seed, scale)),
            &(|seed| u32s_geometric(seed, scale).flat_map(U::checked_from)),
        )),
        GenerationMode::SpecialRandom(scale) => Box::new(random_triples(
            &EXAMPLE_SEED,
            &(|seed| special_random_signed(seed)),
            &(|seed| special_random_integers(seed, scale)),
            &(|seed| u32s_geometric(seed, scale).flat_map(U::checked_from)),
        )),
    }
}

pub fn pairs_of_integer_and_natural(gm: GenerationMode) -> It<(Integer, Natural)> {
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
) -> It<((rug::Integer, rug::Integer), (Integer, Natural))> {
    Box::new(pairs_of_integer_and_natural(gm).map(|(x, y)| {
        (
            (integer_to_rug_integer(&x), natural_to_rug_integer(&y)),
            (x, y),
        )
    }))
}

pub fn pairs_of_natural_and_integer(gm: GenerationMode) -> It<(Natural, Integer)> {
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
) -> It<((rug::Integer, rug::Integer), (Natural, Integer))> {
    Box::new(pairs_of_natural_and_integer(gm).map(|(x, y)| {
        (
            (natural_to_rug_integer(&x), integer_to_rug_integer(&y)),
            (x, y),
        )
    }))
}

pub fn pairs_of_natural_and_natural_integer(gm: GenerationMode) -> It<(Natural, Integer)> {
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
) -> It<((rug::Integer, rug::Integer), (Natural, Integer))> {
    Box::new(pairs_of_natural_and_natural_integer(gm).map(|(x, y)| {
        (
            (natural_to_rug_integer(&x), integer_to_rug_integer(&y)),
            (x, y),
        )
    }))
}

pub fn triples_of_integer_natural_and_integer(
    gm: GenerationMode,
) -> It<(Integer, Natural, Integer)> {
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
) -> It<(Natural, Integer, Natural)> {
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

pub fn triples_of_integer_integer_and_small_unsigned<T: PrimitiveUnsigned + Rand>(
    gm: GenerationMode,
) -> It<(Integer, Integer, T)> {
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

pub fn rm_triples_of_integer_integer_and_small_unsigned<T: PrimitiveUnsigned + Rand>(
    gm: GenerationMode,
) -> It<((rug::Integer, rug::Integer, T), (Integer, Integer, T))> {
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
pub fn triples_of_integer_integer_and_small_unsigned_var_1<T: PrimitiveUnsigned + Rand>(
    gm: GenerationMode,
) -> It<(Integer, Integer, T)>
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
pub fn triples_of_integer_integer_and_small_unsigned_var_2<T: PrimitiveUnsigned + Rand>(
    gm: GenerationMode,
) -> It<(Integer, Integer, T)> {
    Box::new(
        triples_of_integer_integer_and_small_unsigned::<T>(gm)
            .filter(|&(ref x, ref y, pow)| !x.eq_mod_power_of_two(y, pow.checked_into().unwrap())),
    )
}

pub fn triples_of_integer_natural_and_small_unsigned<T: PrimitiveUnsigned + Rand>(
    gm: GenerationMode,
) -> It<(Integer, Natural, T)> {
    match gm {
        GenerationMode::Exhaustive => Box::new(exhaustive_triples(
            exhaustive_integers(),
            exhaustive_naturals(),
            exhaustive_unsigned(),
        )),
        GenerationMode::Random(scale) => Box::new(random_triples(
            &EXAMPLE_SEED,
            &(|seed| random_integers(seed, scale)),
            &(|seed| random_naturals(seed, scale)),
            &(|seed| u32s_geometric(seed, scale).flat_map(T::checked_from)),
        )),
        GenerationMode::SpecialRandom(scale) => Box::new(random_triples(
            &EXAMPLE_SEED,
            &(|seed| special_random_integers(seed, scale)),
            &(|seed| special_random_naturals(seed, scale)),
            &(|seed| u32s_geometric(seed, scale).flat_map(T::checked_from)),
        )),
    }
}

// All triples of `Integer`, `Natural`, and small `T`, where `T` is unsigned and the `Integer`s are
// equal mod 2 to the power of the `T`.
pub fn triples_of_integer_natural_and_small_unsigned_var_1<T: PrimitiveUnsigned + Rand>(
    gm: GenerationMode,
) -> It<(Integer, Natural, T)>
where
    Integer: Shl<T, Output = Integer>,
{
    Box::new(
        triples_of_integer_natural_and_small_unsigned(gm)
            .map(|(x, y, pow)| ((x << pow) + &y, y, pow)),
    )
}

// All triples of `Integer`, `Natural`, and small `T`, where `T` is unsigned and the `Integer`s are
// not equal mod 2 to the power of the `T`.
pub fn triples_of_integer_natural_and_small_unsigned_var_2<T: PrimitiveUnsigned + Rand>(
    gm: GenerationMode,
) -> It<(Integer, Natural, T)> {
    Box::new(
        triples_of_integer_natural_and_small_unsigned::<T>(gm)
            .filter(|&(ref x, ref y, pow)| !x.eq_mod_power_of_two(y, pow.checked_into().unwrap())),
    )
}

pub fn triples_of_natural_integer_and_small_unsigned<T: PrimitiveUnsigned + Rand>(
    gm: GenerationMode,
) -> It<(Natural, Integer, T)> {
    match gm {
        GenerationMode::Exhaustive => Box::new(exhaustive_triples(
            exhaustive_naturals(),
            exhaustive_integers(),
            exhaustive_unsigned(),
        )),
        GenerationMode::Random(scale) => Box::new(random_triples(
            &EXAMPLE_SEED,
            &(|seed| random_naturals(seed, scale)),
            &(|seed| random_integers(seed, scale)),
            &(|seed| u32s_geometric(seed, scale).flat_map(T::checked_from)),
        )),
        GenerationMode::SpecialRandom(scale) => Box::new(random_triples(
            &EXAMPLE_SEED,
            &(|seed| special_random_naturals(seed, scale)),
            &(|seed| special_random_integers(seed, scale)),
            &(|seed| u32s_geometric(seed, scale).flat_map(T::checked_from)),
        )),
    }
}

pub fn triples_of_integer_small_u64_and_bool(gm: GenerationMode) -> It<(Integer, u64, bool)> {
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
) -> It<((rug::Integer, u64, bool), (Integer, u64, bool))> {
    Box::new(
        triples_of_integer_small_u64_and_bool(gm)
            .map(|(x, y, z)| ((integer_to_rug_integer(&x), y, z), (x, y, z))),
    )
}

pub fn pairs_of_integer_and_rounding_mode(gm: GenerationMode) -> It<(Integer, RoundingMode)> {
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
) -> It<(Integer, RoundingMode)> {
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

macro_rules! float_gen {
    (
        $f: ident,
        $finite_floats: ident,
        $pairs_of_integer_and_rounding_mode_var_1: ident,
        $integers_exactly_equal_to_float: ident,
        $integers_not_exactly_equal_to_float: ident,
        $floats_exactly_equal_to_integer: ident,
        $random_integers_not_exactly_equal_to_float_s: ident,
        $random_integers_not_exactly_equal_to_float_f: ident,
        $special_random_integers_not_exactly_equal_to_float: ident,
        $floats_var_4: ident,
        $floats_var_5: ident,
        $integers_var_1: ident
    ) => {
        pub fn $pairs_of_integer_and_rounding_mode_var_1(
            gm: GenerationMode,
        ) -> It<(Integer, RoundingMode)> {
            Box::new(
                pairs_of_integer_and_rounding_mode(gm).filter(|&(ref n, rm)| {
                    rm != RoundingMode::Exact || $f::checked_from(n).is_some()
                }),
            )
        }

        pub fn $integers_exactly_equal_to_float(gm: GenerationMode) -> It<Integer> {
            Box::new(integers(gm).filter(|n| $f::checked_from(n).is_some()))
        }

        pub fn $floats_exactly_equal_to_integer(gm: GenerationMode) -> It<$f> {
            Box::new(integers(gm).flat_map($f::checked_from))
        }

        struct $random_integers_not_exactly_equal_to_float_s {
            sign_gen: Random<bool>,
            abs_gen: It<Natural>,
        }

        impl Iterator for $random_integers_not_exactly_equal_to_float_s {
            type Item = Integer;

            fn next(&mut self) -> Option<Integer> {
                if self.sign_gen.next().unwrap() {
                    self.abs_gen.next().map(Integer::from)
                } else {
                    self.abs_gen.next().map(|n| -n)
                }
            }
        }

        fn $random_integers_not_exactly_equal_to_float_f(
            scale: u32,
        ) -> $random_integers_not_exactly_equal_to_float_s {
            let n = Natural::from($f::SMALLEST_UNREPRESENTABLE_UINT);
            $random_integers_not_exactly_equal_to_float_s {
                sign_gen: random(&scramble(&EXAMPLE_SEED, "sign")),
                abs_gen: Box::new(random_range_up_natural(
                    &scramble(&EXAMPLE_SEED, "abs"),
                    scale,
                    n,
                )),
            }
        }

        fn $special_random_integers_not_exactly_equal_to_float(
            scale: u32,
        ) -> $random_integers_not_exactly_equal_to_float_s {
            let n = Natural::from($f::SMALLEST_UNREPRESENTABLE_UINT);
            $random_integers_not_exactly_equal_to_float_s {
                sign_gen: random(&scramble(&EXAMPLE_SEED, "sign")),
                abs_gen: Box::new(special_random_range_up_natural(
                    &scramble(&EXAMPLE_SEED, "abs"),
                    scale,
                    n,
                )),
            }
        }

        pub fn $integers_not_exactly_equal_to_float(gm: GenerationMode) -> It<Integer> {
            let n = Natural::from($f::SMALLEST_UNREPRESENTABLE_UINT);
            let xs: It<Integer> = match gm {
                GenerationMode::Exhaustive => Box::new(
                    range_up_increasing_integer(Integer::from(&n))
                        .interleave(range_down_decreasing_integer(-n)),
                ),
                GenerationMode::Random(scale) => {
                    Box::new($random_integers_not_exactly_equal_to_float_f(scale))
                }
                GenerationMode::SpecialRandom(scale) => {
                    Box::new($special_random_integers_not_exactly_equal_to_float(scale))
                }
            };
            Box::new(xs.filter(|n| $f::checked_from(n).is_none()))
        }

        // finite floats that are not exactly equal to an Integer.
        pub fn $floats_var_4(gm: GenerationMode) -> It<$f> {
            Box::new($finite_floats(gm).filter(|&f| Integer::checked_from(f).is_none()))
        }

        // positive floats exactly in between two adjacent Integers.
        pub fn $floats_var_5(gm: GenerationMode) -> It<$f> {
            Box::new($floats_exactly_equal_to_integer(gm).flat_map(|f| {
                let f_plus_half = f + 0.5;
                if Integer::checked_from(f_plus_half).is_some() {
                    None
                } else {
                    Some(f_plus_half)
                }
            }))
        }

        // Integers exactly in between two adjacent floats.
        pub fn $integers_var_1(gm: GenerationMode) -> It<Integer> {
            Box::new($integers_not_exactly_equal_to_float(gm).flat_map(|n| {
                let f_below = $f::rounding_from(&n, RoundingMode::Floor);
                let on_below = Integer::checked_from(f_below);
                if on_below.is_none() {
                    return None;
                }
                let n_below = on_below.unwrap();
                let mut f_above = f_below;
                f_above.increment();
                let on_above = Integer::checked_from(f_above);
                if on_above.is_none() {
                    return None;
                }
                let n_above = on_above.unwrap();
                if n_above - &n == &n - n_below {
                    Some(n)
                } else {
                    None
                }
            }))
        }
    };
}

float_gen!(
    f32,
    finite_f32s,
    pairs_of_integer_and_rounding_mode_var_1_f32,
    integers_exactly_equal_to_f32,
    integers_not_exactly_equal_to_f32,
    f32s_exactly_equal_to_integer,
    RandomIntegersNotExactlyEqualToF32,
    random_integers_not_exactly_equal_to_f32,
    special_random_integers_not_exactly_equal_to_f32,
    f32s_var_4,
    f32s_var_5,
    integers_var_1_f32
);
float_gen!(
    f64,
    finite_f64s,
    pairs_of_integer_and_rounding_mode_var_1_f64,
    integers_exactly_equal_to_f64,
    integers_not_exactly_equal_to_f64,
    f64s_exactly_equal_to_integer,
    RandomIntegersNotExactlyEqualToF64,
    random_integers_not_exactly_equal_to_f64,
    special_random_integers_not_exactly_equal_to_f64,
    f64s_var_4,
    f64s_var_5,
    integers_var_1_f64
);

fn triples_of_integer_small_signed_and_rounding_mode<T: PrimitiveSigned + Rand>(
    gm: GenerationMode,
) -> It<(Integer, T, RoundingMode)> {
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
pub fn triples_of_integer_small_signed_and_rounding_mode_var_1<T: PrimitiveSigned + Rand>(
    gm: GenerationMode,
) -> It<(Integer, T, RoundingMode)>
where
    Integer: Shr<T, Output = Integer>,
{
    Box::new(
        triples_of_integer_small_signed_and_rounding_mode::<T>(gm)
            .map(|(n, i, rm)| (if i < T::ZERO { n >> i } else { n }, i, rm)),
    )
}

// All triples of `Integer`, small `T`, and `RoundingMode`, where `T` is signed, such that if the
// `SignedLimb` is positive and the `RoundingMode` is `RoundingMode::Exact`, the `Integer` is
// divisible by 2 to the power of the `T`.
pub fn triples_of_integer_small_signed_and_rounding_mode_var_2<T: PrimitiveSigned + Rand>(
    gm: GenerationMode,
) -> It<(Integer, T, RoundingMode)>
where
    Integer: Shl<T, Output = Integer>,
{
    Box::new(
        triples_of_integer_small_signed_and_rounding_mode::<T>(gm)
            .map(|(n, i, rm)| (if i > T::ZERO { n << i } else { n }, i, rm)),
    )
}

fn triples_of_integer_small_unsigned_and_rounding_mode<T: PrimitiveUnsigned + Rand>(
    gm: GenerationMode,
) -> It<(Integer, T, RoundingMode)> {
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
pub fn triples_of_integer_small_unsigned_and_rounding_mode_var_1<T: PrimitiveUnsigned + Rand>(
    gm: GenerationMode,
) -> It<(Integer, T, RoundingMode)>
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
    integers: It<Integer>,
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
pub fn pairs_of_integer_and_vec_of_bool_var_1(gm: GenerationMode) -> It<(Integer, Vec<bool>)> {
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
    integers: It<Integer>,
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
pub fn pairs_of_integer_and_vec_of_bool_var_2(gm: GenerationMode) -> It<(Integer, Vec<bool>)> {
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

pub fn quadruples_of_integer_integer_integer_and_small_unsigned<T: PrimitiveUnsigned + Rand>(
    gm: GenerationMode,
) -> It<(Integer, Integer, Integer, T)> {
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

fn triples_of_integer_positive_unsigned_and_rounding_mode<T: PrimitiveUnsigned + Rand>(
    gm: GenerationMode,
) -> It<(Integer, T, RoundingMode)> {
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
pub fn triples_of_integer_positive_unsigned_and_rounding_mode_var_1<T: PrimitiveUnsigned + Rand>(
    gm: GenerationMode,
) -> It<(Integer, T, RoundingMode)>
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

fn triples_of_unsigned_nonzero_integer_and_rounding_mode<T: PrimitiveUnsigned + Rand>(
    gm: GenerationMode,
) -> It<(T, Integer, RoundingMode)> {
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
pub fn triples_of_unsigned_nonzero_integer_and_rounding_mode_var_1<T: PrimitiveUnsigned + Rand>(
    gm: GenerationMode,
) -> It<(T, Integer, RoundingMode)>
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

fn triples_of_integer_nonzero_signed_and_rounding_mode<T: PrimitiveSigned + Rand>(
    gm: GenerationMode,
) -> It<(Integer, T, RoundingMode)>
where
    T::UnsignedOfEqualWidth: Rand,
{
    match gm {
        GenerationMode::Exhaustive => reshape_2_1_to_3(Box::new(lex_pairs(
            exhaustive_pairs(exhaustive_integers(), exhaustive_nonzero_signed()),
            exhaustive_rounding_modes(),
        ))),
        GenerationMode::Random(scale) => Box::new(random_triples(
            &EXAMPLE_SEED,
            &(|seed| random_integers(seed, scale)),
            &(|seed| random_nonzero_signed(seed)),
            &(|seed| random_rounding_modes(seed)),
        )),
        GenerationMode::SpecialRandom(scale) => Box::new(random_triples(
            &EXAMPLE_SEED,
            &(|seed| special_random_integers(seed, scale)),
            &(|seed| special_random_nonzero_signed(seed)),
            &(|seed| random_rounding_modes(seed)),
        )),
    }
}

// All triples of `Integer`, positive `T`, and `RoundingMode`, where `T` is signed and if the
// `RoundingMode` is `RoundingMode::Exact`, the `Integer` is divisible by the `T`.
pub fn triples_of_integer_nonzero_signed_and_rounding_mode_var_1<T: PrimitiveSigned + Rand>(
    gm: GenerationMode,
) -> It<(Integer, T, RoundingMode)>
where
    Integer: Mul<T, Output = Integer>,
    T::UnsignedOfEqualWidth: Rand,
{
    Box::new(
        triples_of_integer_nonzero_signed_and_rounding_mode::<T>(gm).map(|(n, i, rm)| {
            if rm == RoundingMode::Exact {
                (n * i, i, rm)
            } else {
                (n, i, rm)
            }
        }),
    )
}

fn triples_of_signed_nonzero_integer_and_rounding_mode<T: PrimitiveSigned + Rand>(
    gm: GenerationMode,
) -> It<(T, Integer, RoundingMode)>
where
    T::UnsignedOfEqualWidth: Rand,
{
    match gm {
        GenerationMode::Exhaustive => reshape_2_1_to_3(Box::new(lex_pairs(
            exhaustive_pairs(exhaustive_signed(), exhaustive_nonzero_integers()),
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
            &(|seed| special_random_signed(seed)),
            &(|seed| special_random_nonzero_integers(seed, scale)),
            &(|seed| random_rounding_modes(seed)),
        )),
    }
}

// All triples of `T`, nonzero `Integer`, and `RoundingMode`, where `T` is signed and if the
// `RoundingMode` is `RoundingMode::Exact`, the `T` is divisible by the `Integer`.
pub fn triples_of_signed_nonzero_integer_and_rounding_mode_var_1<T: PrimitiveSigned + Rand>(
    gm: GenerationMode,
) -> It<(T, Integer, RoundingMode)>
where
    T: Mul<Integer, Output = Integer>,
    T: CheckedFrom<Integer>,
    T::UnsignedOfEqualWidth: Rand,
{
    Box::new(
        triples_of_signed_nonzero_integer_and_rounding_mode::<T>(gm).filter_map(|(i, n, rm)| {
            if rm == RoundingMode::Exact {
                T::checked_from(i * (&n).abs()).map(|i| (i, n, rm))
            } else {
                Some((i, n, rm))
            }
        }),
    )
}
