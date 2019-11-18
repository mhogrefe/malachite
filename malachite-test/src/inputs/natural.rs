use std::ops::{Add, Mul, Shl, Shr};

use malachite_base::crement::Crementable;
use malachite_base::num::arithmetic::traits::{
    DivisibleBy, DivisibleByPowerOfTwo, EqMod, EqModPowerOfTwo,
};
use malachite_base::num::basic::integers::PrimitiveInteger;
use malachite_base::num::basic::signeds::PrimitiveSigned;
use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base::num::conversion::traits::{
    CheckedFrom, CheckedInto, ConvertibleFrom, RoundingFrom,
};
use malachite_base::num::floats::PrimitiveFloat;
use malachite_base::num::logic::traits::SignificantBits;
use malachite_base::round::RoundingMode;
use malachite_nz::natural::Natural;
use malachite_nz::platform::Limb;
use num::BigUint;
use rand::{IsaacRng, Rand, Rng, SeedableRng};
use rug;
use rust_wheels::iterators::bools::exhaustive_bools;
use rust_wheels::iterators::common::{scramble, EXAMPLE_SEED};
use rust_wheels::iterators::dependent_pairs::dependent_pairs;
use rust_wheels::iterators::general::random;
use rust_wheels::iterators::integers_geometric::{i32s_geometric, u32s_geometric};
use rust_wheels::iterators::naturals::{
    exhaustive_naturals, exhaustive_positive_naturals, random_naturals, random_positive_naturals,
    random_range_up_natural, range_up_increasing_natural, special_random_naturals,
    special_random_positive_naturals, special_random_range_up_natural,
};
use rust_wheels::iterators::primitive_ints::{
    exhaustive_positive, exhaustive_signed, exhaustive_unsigned, random_positive_unsigned,
    range_up_increasing, special_random_positive_unsigned, special_random_unsigned,
};
use rust_wheels::iterators::rounding_modes::{exhaustive_rounding_modes, random_rounding_modes};
use rust_wheels::iterators::tuples::{
    exhaustive_pairs, exhaustive_pairs_from_single, exhaustive_quadruples, exhaustive_triples,
    exhaustive_triples_from_single, lex_pairs, log_pairs, random_pairs, random_pairs_from_single,
    random_quadruples, random_triples, random_triples_from_single,
};
use rust_wheels::iterators::vecs::exhaustive_fixed_size_vecs_from_single;

use common::{natural_to_biguint, natural_to_rug_integer, GenerationMode};
use inputs::base::{finite_f32s, finite_f64s, pairs_of_unsigneds, It};
use inputs::common::{reshape_1_2_to_3, reshape_2_1_to_3};

pub fn naturals(gm: GenerationMode) -> It<Natural> {
    match gm {
        GenerationMode::Exhaustive => Box::new(exhaustive_naturals()),
        GenerationMode::Random(scale) => Box::new(random_naturals(&EXAMPLE_SEED, scale)),
        GenerationMode::SpecialRandom(scale) => {
            Box::new(special_random_naturals(&EXAMPLE_SEED, scale))
        }
    }
}

pub fn rm_naturals(gm: GenerationMode) -> It<(rug::Integer, Natural)> {
    Box::new(naturals(gm).map(|n| (natural_to_rug_integer(&n), n)))
}

// All `Natural` multiples of 3.
pub fn naturals_var_1(gm: GenerationMode) -> It<Natural> {
    Box::new(naturals(gm).map(|n| n * 3 as Limb))
}

// All pairs of `Natural` multiples of 3, and `3`.
pub fn pairs_of_natural_var_1_and_3(gm: GenerationMode) -> It<(Natural, Limb)> {
    Box::new(naturals_var_1(gm).map(|n| (n, 3 as Limb)))
}

pub fn nrm_naturals(gm: GenerationMode) -> It<(BigUint, rug::Integer, Natural)> {
    Box::new(naturals(gm).map(|n| (natural_to_biguint(&n), natural_to_rug_integer(&n), n)))
}

pub fn positive_naturals(gm: GenerationMode) -> It<Natural> {
    match gm {
        GenerationMode::Exhaustive => Box::new(exhaustive_positive_naturals()),
        GenerationMode::Random(scale) => Box::new(random_positive_naturals(&EXAMPLE_SEED, scale)),
        GenerationMode::SpecialRandom(scale) => {
            Box::new(special_random_positive_naturals(&EXAMPLE_SEED, scale))
        }
    }
}

pub fn pairs_of_naturals(gm: GenerationMode) -> It<(Natural, Natural)> {
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
) -> It<(
    (BigUint, BigUint),
    (rug::Integer, rug::Integer),
    (Natural, Natural),
)> {
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
) -> It<((rug::Integer, rug::Integer), (Natural, Natural))> {
    Box::new(pairs_of_naturals(gm).map(|(x, y)| {
        (
            (natural_to_rug_integer(&x), natural_to_rug_integer(&y)),
            (x, y),
        )
    }))
}

//TODO use subset_pairs
// All pairs of `Natural`s where the first is greater than or equal to the second.
pub fn pairs_of_naturals_var_1(gm: GenerationMode) -> It<(Natural, Natural)> {
    Box::new(pairs_of_naturals(gm).filter(|&(ref x, ref y)| x >= y))
}

//TODO use subset_pairs
pub fn rm_pairs_of_naturals_var_1(
    gm: GenerationMode,
) -> It<((rug::Integer, rug::Integer), (Natural, Natural))> {
    Box::new(pairs_of_naturals_var_1(gm).map(|(x, y)| {
        (
            (natural_to_rug_integer(&x), natural_to_rug_integer(&y)),
            (x, y),
        )
    }))
}

//TODO use subset_pairs
pub fn nrm_pairs_of_naturals_var_1(
    gm: GenerationMode,
) -> It<(
    (BigUint, BigUint),
    (rug::Integer, rug::Integer),
    (Natural, Natural),
)> {
    Box::new(pairs_of_naturals_var_1(gm).map(|(x, y)| {
        (
            (natural_to_biguint(&x), natural_to_biguint(&y)),
            (natural_to_rug_integer(&x), natural_to_rug_integer(&y)),
            (x, y),
        )
    }))
}

pub fn triples_of_naturals(gm: GenerationMode) -> It<(Natural, Natural, Natural)> {
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
pub fn triples_of_naturals_var_1(gm: GenerationMode) -> It<(Natural, Natural, Natural)> {
    Box::new(triples_of_naturals(gm).filter(|&(ref a, ref b, ref c)| a >= &(b * c)))
}

pub fn triples_of_natural_natural_and_positive_natural(
    gm: GenerationMode,
) -> It<(Natural, Natural, Natural)> {
    match gm {
        GenerationMode::Exhaustive => Box::new(exhaustive_triples(
            exhaustive_naturals(),
            exhaustive_naturals(),
            exhaustive_positive_naturals(),
        )),
        GenerationMode::Random(scale) => Box::new(random_triples(
            &EXAMPLE_SEED,
            &(|seed| random_naturals(seed, scale)),
            &(|seed| random_naturals(seed, scale)),
            &(|seed| random_positive_naturals(seed, scale)),
        )),
        GenerationMode::SpecialRandom(scale) => Box::new(random_triples(
            &EXAMPLE_SEED,
            &(|seed| special_random_naturals(seed, scale)),
            &(|seed| special_random_naturals(seed, scale)),
            &(|seed| special_random_positive_naturals(seed, scale)),
        )),
    }
}

fn random_pairs_of_natural_and_primitive<T: PrimitiveInteger + Rand>(
    scale: u32,
) -> It<(Natural, T)> {
    Box::new(random_pairs(
        &EXAMPLE_SEED,
        &(|seed| random_naturals(seed, scale)),
        &(|seed| random(seed)),
    ))
}

fn random_pairs_of_natural_and_positive_unsigned<T: PrimitiveUnsigned + Rand>(
    scale: u32,
) -> It<(Natural, T)> {
    Box::new(random_pairs(
        &EXAMPLE_SEED,
        &(|seed| random_naturals(seed, scale)),
        &(|seed| random_positive_unsigned(seed)),
    ))
}

fn random_pairs_of_primitive_and_natural<T: PrimitiveInteger + Rand>(
    scale: u32,
) -> It<(T, Natural)> {
    Box::new(random_pairs(
        &EXAMPLE_SEED,
        &(|seed| random(seed)),
        &(|seed| random_naturals(seed, scale)),
    ))
}

pub fn pairs_of_natural_and_unsigned<T: PrimitiveUnsigned + Rand>(
    gm: GenerationMode,
) -> It<(Natural, T)> {
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

pub fn rm_pairs_of_natural_and_unsigned<T: PrimitiveUnsigned + Rand>(
    gm: GenerationMode,
) -> It<((rug::Integer, T), (Natural, T))> {
    Box::new(
        pairs_of_natural_and_unsigned(gm).map(|(x, y)| ((natural_to_rug_integer(&x), y), (x, y))),
    )
}

pub fn nm_pairs_of_natural_and_unsigned<T: PrimitiveUnsigned + Rand>(
    gm: GenerationMode,
) -> It<((BigUint, T), (Natural, T))> {
    Box::new(pairs_of_natural_and_unsigned(gm).map(|(x, y)| ((natural_to_biguint(&x), y), (x, y))))
}

pub fn nrm_pairs_of_natural_and_unsigned<T: PrimitiveUnsigned + Rand>(
    gm: GenerationMode,
) -> It<((BigUint, T), (rug::Integer, T), (Natural, T))> {
    Box::new(pairs_of_natural_and_unsigned(gm).map(|(x, y)| {
        (
            (natural_to_biguint(&x), y),
            (natural_to_rug_integer(&x), y),
            (x, y),
        )
    }))
}

// All pairs of `Natural` and `Limb` where the `Natural` is greater than or equal to the `Limb`.
pub fn pairs_of_natural_and_limb_var_1(gm: GenerationMode) -> It<(Natural, Limb)> {
    Box::new(pairs_of_natural_and_unsigned(gm).filter(|&(ref n, u)| *n >= u))
}

pub fn rm_pairs_of_natural_and_limb_var_1(
    gm: GenerationMode,
) -> It<((rug::Integer, Limb), (Natural, Limb))> {
    Box::new(
        pairs_of_natural_and_limb_var_1(gm).map(|(x, y)| ((natural_to_rug_integer(&x), y), (x, y))),
    )
}

pub fn nm_pairs_of_natural_and_limb_var_1(
    gm: GenerationMode,
) -> It<((BigUint, Limb), (Natural, Limb))> {
    Box::new(
        pairs_of_natural_and_limb_var_1(gm).map(|(x, y)| ((natural_to_biguint(&x), y), (x, y))),
    )
}

pub fn nrm_pairs_of_natural_and_limb_var_1(
    gm: GenerationMode,
) -> It<((BigUint, Limb), (rug::Integer, Limb), (Natural, Limb))> {
    Box::new(pairs_of_natural_and_limb_var_1(gm).map(|(x, y)| {
        (
            (natural_to_biguint(&x), y),
            (natural_to_rug_integer(&x), y),
            (x, y),
        )
    }))
}

// All pairs of `Natural` and `T` where `T` is unsigned and the most-significant bit of the `T` is
// set.
pub fn pairs_of_natural_and_unsigned_var_2<T: PrimitiveUnsigned + Rand>(
    gm: GenerationMode,
) -> It<(Natural, T)> {
    match gm {
        GenerationMode::Exhaustive => Box::new(exhaustive_pairs(
            exhaustive_naturals(),
            range_up_increasing(T::ONE << (T::WIDTH - 1)),
        )),
        GenerationMode::Random(scale) => Box::new(
            random_pairs_of_natural_and_primitive::<T>(scale).map(|(n, mut u)| {
                u.set_bit(u64::from(T::WIDTH - 1));
                (n, u)
            }),
        ),
        GenerationMode::SpecialRandom(scale) => Box::new(
            random_pairs(
                &EXAMPLE_SEED,
                &(|seed| special_random_naturals(seed, scale)),
                &(|seed| special_random_unsigned::<T>(seed)),
            )
            .map(|(n, mut u)| {
                u.set_bit(u64::from(T::WIDTH - 1));
                (n, u)
            }),
        ),
    }
}

pub fn pairs_of_unsigned_and_natural<T: PrimitiveUnsigned + Rand>(
    gm: GenerationMode,
) -> It<(T, Natural)> {
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

pub fn rm_pairs_of_unsigned_and_natural<T: PrimitiveUnsigned + Rand>(
    gm: GenerationMode,
) -> It<((T, rug::Integer), (T, Natural))> {
    Box::new(
        pairs_of_unsigned_and_natural(gm).map(|(x, y)| ((x, natural_to_rug_integer(&y)), (x, y))),
    )
}

// All pairs of `Limb` and `Natural` where the `Limb` is greater than or equal to the `Natural`.
pub fn pairs_of_limb_and_natural_var_1(gm: GenerationMode) -> It<(Limb, Natural)> {
    Box::new(
        pairs_of_unsigneds(gm)
            .filter(|(x, y)| x >= y)
            .map(|(x, y)| (x, Natural::from(y))),
    )
}

pub fn rm_pairs_of_limb_and_natural_var_1(
    gm: GenerationMode,
) -> It<((Limb, rug::Integer), (Limb, Natural))> {
    Box::new(
        pairs_of_limb_and_natural_var_1(gm).map(|(x, y)| ((x, natural_to_rug_integer(&y)), (x, y))),
    )
}

pub fn pairs_of_natural_and_positive_unsigned<T: PrimitiveUnsigned + Rand>(
    gm: GenerationMode,
) -> It<(Natural, T)> {
    match gm {
        GenerationMode::Exhaustive => Box::new(exhaustive_pairs(
            exhaustive_naturals(),
            exhaustive_positive(),
        )),
        GenerationMode::Random(scale) => random_pairs_of_natural_and_positive_unsigned(scale),
        GenerationMode::SpecialRandom(scale) => Box::new(random_pairs(
            &EXAMPLE_SEED,
            &(|seed| special_random_naturals(seed, scale)),
            &(|seed| special_random_positive_unsigned(seed)),
        )),
    }
}

pub fn rm_pairs_of_natural_and_positive_unsigned<T: PrimitiveUnsigned + Rand>(
    gm: GenerationMode,
) -> It<((rug::Integer, T), (Natural, T))> {
    Box::new(
        pairs_of_natural_and_positive_unsigned(gm)
            .map(|(x, y)| ((natural_to_rug_integer(&x), y), (x, y))),
    )
}

pub fn nrm_pairs_of_natural_and_positive_unsigned<T: PrimitiveUnsigned + Rand>(
    gm: GenerationMode,
) -> It<((BigUint, T), (rug::Integer, T), (Natural, T))> {
    Box::new(pairs_of_natural_and_positive_unsigned(gm).map(|(x, y)| {
        (
            (natural_to_biguint(&x), y),
            (natural_to_rug_integer(&x), y),
            (x, y),
        )
    }))
}

pub fn nm_pairs_of_natural_and_positive_unsigned<T: PrimitiveUnsigned + Rand>(
    gm: GenerationMode,
) -> It<((BigUint, T), (Natural, T))> {
    Box::new(
        pairs_of_natural_and_positive_unsigned(gm)
            .map(|(x, y)| ((natural_to_biguint(&x), y), (x, y))),
    )
}

// All pairs of `Natural` and positive `Limb` where the `Natural` is divisible by the `Limb`.
pub fn pairs_of_natural_and_positive_limb_var_1(gm: GenerationMode) -> It<(Natural, Limb)> {
    Box::new(pairs_of_natural_and_positive_unsigned(gm).map(|(n, u)| (n * u, u)))
}

pub fn nrm_pairs_of_natural_and_positive_limb_var_1(
    gm: GenerationMode,
) -> It<((BigUint, Limb), (rug::Integer, Limb), (Natural, Limb))> {
    Box::new(pairs_of_natural_and_positive_limb_var_1(gm).map(|(x, y)| {
        (
            (natural_to_biguint(&x), y),
            (natural_to_rug_integer(&x), y),
            (x, y),
        )
    }))
}

// All pairs of `Natural` and positive `Limb`, where the `Natural` is not divisible by the `Limb`.
pub fn pairs_of_natural_and_positive_limb_var_2(gm: GenerationMode) -> It<(Natural, Limb)> {
    Box::new(pairs_of_natural_and_positive_unsigned(gm).filter(|&(ref n, u)| !n.divisible_by(u)))
}

// All pairs of `Natural` and `Limb` where the most-significant bit of the `Limb` is set and the
// `Natural` is divisible by the `Limb`.
pub fn pairs_of_natural_and_limb_var_3(gm: GenerationMode) -> It<(Natural, Limb)> {
    Box::new(pairs_of_natural_and_unsigned_var_2(gm).map(|(n, u)| (n * u, u)))
}

pub fn pairs_of_unsigned_and_positive_natural<T: PrimitiveUnsigned + Rand>(
    gm: GenerationMode,
) -> It<(T, Natural)> {
    match gm {
        GenerationMode::Exhaustive => Box::new(exhaustive_pairs(
            exhaustive_unsigned(),
            exhaustive_positive_naturals(),
        )),
        GenerationMode::Random(scale) => Box::new(random_pairs(
            &EXAMPLE_SEED,
            &(|seed| random(seed)),
            &(|seed| random_positive_naturals(seed, scale)),
        )),
        GenerationMode::SpecialRandom(scale) => Box::new(random_pairs(
            &EXAMPLE_SEED,
            &(|seed| special_random_unsigned(seed)),
            &(|seed| special_random_positive_naturals(seed, scale)),
        )),
    }
}

// All pairs of `Limb` and positive `Natural` where the `Limb` is not divisible by the `Natural`.
pub fn pairs_of_limb_and_positive_natural_var_1(gm: GenerationMode) -> It<(Limb, Natural)> {
    Box::new(
        pairs_of_unsigned_and_positive_natural::<Limb>(gm).filter(|&(u, ref n)| !u.divisible_by(n)),
    )
}

// All pairs of `Limb` and positive `Natural` where the `Limb` is divisible by the `Natural`.
pub fn pairs_of_limb_and_positive_natural_var_2(gm: GenerationMode) -> It<(Limb, Natural)> {
    Box::new(
        pairs_of_unsigned_and_positive_natural::<Limb>(gm)
            .filter_map(|(u, n)| Limb::checked_from(u * n.clone()).map(|u| (u, n))),
    )
}

pub fn pairs_of_natural_and_positive_natural(gm: GenerationMode) -> It<(Natural, Natural)> {
    match gm {
        GenerationMode::Exhaustive => Box::new(exhaustive_pairs(
            exhaustive_naturals(),
            exhaustive_positive_naturals(),
        )),
        GenerationMode::Random(scale) => Box::new(random_pairs(
            &EXAMPLE_SEED,
            &(|seed| random_naturals(seed, scale)),
            &(|seed| random_positive_naturals(seed, scale)),
        )),
        GenerationMode::SpecialRandom(scale) => Box::new(random_pairs(
            &EXAMPLE_SEED,
            &(|seed| special_random_naturals(seed, scale)),
            &(|seed| special_random_positive_naturals(seed, scale)),
        )),
    }
}

pub fn rm_pairs_of_natural_and_positive_natural(
    gm: GenerationMode,
) -> It<((rug::Integer, rug::Integer), (Natural, Natural))> {
    Box::new(pairs_of_natural_and_positive_natural(gm).map(|(x, y)| {
        (
            (natural_to_rug_integer(&x), natural_to_rug_integer(&y)),
            (x, y),
        )
    }))
}

pub fn nrm_pairs_of_natural_and_positive_natural(
    gm: GenerationMode,
) -> It<(
    (BigUint, BigUint),
    (rug::Integer, rug::Integer),
    (Natural, Natural),
)> {
    Box::new(pairs_of_natural_and_positive_natural(gm).map(|(x, y)| {
        (
            (natural_to_biguint(&x), natural_to_biguint(&y)),
            (natural_to_rug_integer(&x), natural_to_rug_integer(&y)),
            (x, y),
        )
    }))
}

// All pairs of `Natural` and positive `Natural` where the first `Natural` is divisible by the
// second.
pub fn pairs_of_natural_and_positive_natural_var_1(gm: GenerationMode) -> It<(Natural, Natural)> {
    Box::new(pairs_of_natural_and_positive_natural(gm).map(|(n, u)| (n * &u, u)))
}

pub fn nrm_pairs_of_natural_and_positive_natural_var_1(
    gm: GenerationMode,
) -> It<(
    (BigUint, BigUint),
    (rug::Integer, rug::Integer),
    (Natural, Natural),
)> {
    Box::new(
        pairs_of_natural_and_positive_natural_var_1(gm).map(|(x, y)| {
            (
                (natural_to_biguint(&x), natural_to_biguint(&y)),
                (natural_to_rug_integer(&x), natural_to_rug_integer(&y)),
                (x, y),
            )
        }),
    )
}

// All pairs of `Natural` and positive `Natural`, where the first `Natural` is not divisible by the
// second.
pub fn pairs_of_natural_and_positive_natural_var_2(gm: GenerationMode) -> It<(Natural, Natural)> {
    Box::new(pairs_of_natural_and_positive_natural(gm).filter(|(x, y)| !x.divisible_by(y)))
}

fn random_triples_of_natural_natural_and_primitive<T: PrimitiveInteger + Rand>(
    scale: u32,
) -> It<(Natural, Natural, T)> {
    Box::new(random_triples(
        &EXAMPLE_SEED,
        &(|seed| random_naturals(seed, scale)),
        &(|seed| random_naturals(seed, scale)),
        &(|seed| random(seed)),
    ))
}

pub fn triples_of_natural_natural_and_unsigned<T: PrimitiveUnsigned + Rand>(
    gm: GenerationMode,
) -> It<(Natural, Natural, T)> {
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

// All triples of `Natural`, `Natural`, and `Limb`, where the first `Natural` is greater than or
// equal to the product of the second `Natural` and the `Limb`.
#[allow(op_ref)]
pub fn triples_of_natural_natural_and_limb_var_1(
    gm: GenerationMode,
) -> It<(Natural, Natural, Limb)> {
    Box::new(triples_of_natural_natural_and_unsigned(gm).filter(|&(ref a, ref b, c)| a >= &(b * c)))
}

fn log_pairs_of_natural_and_unsigned<T: PrimitiveUnsigned>() -> It<(Natural, T)> {
    Box::new(log_pairs(exhaustive_naturals(), exhaustive_unsigned()))
}

pub fn pairs_of_natural_and_small_unsigned<T: PrimitiveUnsigned + Rand>(
    gm: GenerationMode,
) -> It<(Natural, T)> {
    match gm {
        GenerationMode::Exhaustive => log_pairs_of_natural_and_unsigned(),
        GenerationMode::Random(scale) => Box::new(random_pairs(
            &EXAMPLE_SEED,
            &(|seed| random_naturals(seed, scale)),
            &(|seed| u32s_geometric(seed, scale).flat_map(T::checked_from)),
        )),
        GenerationMode::SpecialRandom(scale) => Box::new(random_pairs(
            &EXAMPLE_SEED,
            &(|seed| special_random_naturals(seed, scale)),
            &(|seed| u32s_geometric(seed, scale).flat_map(T::checked_from)),
        )),
    }
}

pub fn rm_pairs_of_natural_and_small_unsigned<T: PrimitiveUnsigned + Rand>(
    gm: GenerationMode,
) -> It<((rug::Integer, T), (Natural, T))> {
    Box::new(
        pairs_of_natural_and_small_unsigned(gm)
            .map(|(x, y)| ((natural_to_rug_integer(&x), y), (x, y))),
    )
}

// All pairs of `Natural` and small `T`, where `T` is unsigned and the `Natural` is divisible by 2
// to the power of the `T`.
pub fn pairs_of_natural_and_small_unsigned_var_1<T: PrimitiveUnsigned + Rand>(
    gm: GenerationMode,
) -> It<(Natural, T)>
where
    Natural: Shl<T, Output = Natural>,
{
    Box::new(pairs_of_natural_and_small_unsigned::<T>(gm).map(|(n, u)| (n << u, u)))
}

// All pairs of `Natural` and small `T`, where `T` is unsigned and the `Natural` is not divisible by
// 2 to the power of the `T`.
pub fn pairs_of_natural_and_small_unsigned_var_2<T: PrimitiveUnsigned + Rand>(
    gm: GenerationMode,
) -> It<(Natural, T)> {
    Box::new(
        pairs_of_natural_and_small_unsigned::<T>(gm)
            .filter(|&(ref n, u)| !n.divisible_by_power_of_two(u.checked_into().unwrap())),
    )
}

fn log_pairs_of_natural_and_signed<T: PrimitiveSigned>() -> It<(Natural, T)> {
    Box::new(log_pairs(exhaustive_naturals(), exhaustive_signed()))
}

pub fn pairs_of_natural_and_small_signed<T: PrimitiveSigned + Rand>(
    gm: GenerationMode,
) -> It<(Natural, T)> {
    match gm {
        GenerationMode::Exhaustive => log_pairs_of_natural_and_signed(),
        GenerationMode::Random(scale) => Box::new(random_pairs(
            &EXAMPLE_SEED,
            &(|seed| random_naturals(seed, scale)),
            &(|seed| i32s_geometric(seed, scale).flat_map(|i| T::checked_from(i))),
        )),
        GenerationMode::SpecialRandom(scale) => Box::new(random_pairs(
            &EXAMPLE_SEED,
            &(|seed| special_random_naturals(seed, scale)),
            &(|seed| i32s_geometric(seed, scale).flat_map(|i| T::checked_from(i))),
        )),
    }
}

pub fn rm_pairs_of_natural_and_small_signed<T: PrimitiveSigned + Rand>(
    gm: GenerationMode,
) -> It<((rug::Integer, T), (Natural, T))> {
    Box::new(
        pairs_of_natural_and_small_signed(gm)
            .map(|(x, y)| ((natural_to_rug_integer(&x), y), (x, y))),
    )
}

pub fn nm_pairs_of_natural_and_small_u64(
    gm: GenerationMode,
) -> It<((BigUint, u64), (Natural, u64))> {
    Box::new(
        pairs_of_natural_and_small_unsigned(gm).map(|(x, y)| ((natural_to_biguint(&x), y), (x, y))),
    )
}

pub fn nrm_pairs_of_natural_and_small_unsigned(
    gm: GenerationMode,
) -> It<((BigUint, u64), (rug::Integer, u64), (Natural, u64))> {
    Box::new(pairs_of_natural_and_small_unsigned(gm).map(|(x, y)| {
        (
            (natural_to_biguint(&x), y),
            (natural_to_rug_integer(&x), y),
            (x, y),
        )
    }))
}

pub fn triples_of_natural_small_unsigned_and_small_unsigned<T: PrimitiveUnsigned + Rand>(
    gm: GenerationMode,
) -> It<(Natural, T, T)> {
    match gm {
        GenerationMode::Exhaustive => reshape_1_2_to_3(Box::new(log_pairs(
            exhaustive_naturals(),
            exhaustive_pairs_from_single(exhaustive_unsigned()),
        ))),
        GenerationMode::Random(scale) => Box::new(random_triples(
            &EXAMPLE_SEED,
            &(|seed| random_naturals(seed, scale)),
            &(|seed| u32s_geometric(seed, scale).flat_map(T::checked_from)),
            &(|seed| u32s_geometric(seed, scale).flat_map(T::checked_from)),
        )),
        GenerationMode::SpecialRandom(scale) => Box::new(random_triples(
            &EXAMPLE_SEED,
            &(|seed| special_random_naturals(seed, scale)),
            &(|seed| u32s_geometric(seed, scale).flat_map(T::checked_from)),
            &(|seed| u32s_geometric(seed, scale).flat_map(T::checked_from)),
        )),
    }
}

pub fn triples_of_natural_natural_and_positive_unsigned<T: PrimitiveUnsigned + Rand>(
    gm: GenerationMode,
) -> It<(Natural, Natural, T)> {
    match gm {
        GenerationMode::Exhaustive => Box::new(exhaustive_triples(
            exhaustive_naturals(),
            exhaustive_naturals(),
            exhaustive_positive(),
        )),
        GenerationMode::Random(scale) => Box::new(random_triples(
            &EXAMPLE_SEED,
            &(|seed| random_naturals(seed, scale)),
            &(|seed| random_naturals(seed, scale)),
            &(|seed| random_positive_unsigned(seed)),
        )),
        GenerationMode::SpecialRandom(scale) => Box::new(random_triples(
            &EXAMPLE_SEED,
            &(|seed| special_random_naturals(seed, scale)),
            &(|seed| special_random_naturals(seed, scale)),
            &(|seed| special_random_positive_unsigned(seed)),
        )),
    }
}

fn random_triples_of_natural_primitive_and_natural<T: PrimitiveInteger + Rand>(
    scale: u32,
) -> It<(Natural, T, Natural)> {
    Box::new(random_triples(
        &EXAMPLE_SEED,
        &(|seed| random_naturals(seed, scale)),
        &(|seed| random(seed)),
        &(|seed| random_naturals(seed, scale)),
    ))
}

fn random_triples_of_primitive_natural_and_primitive<T: PrimitiveInteger + Rand>(
    scale: u32,
) -> It<(T, Natural, T)> {
    Box::new(random_triples(
        &EXAMPLE_SEED,
        &(|seed| random(seed)),
        &(|seed| random_naturals(seed, scale)),
        &(|seed| random(seed)),
    ))
}

fn random_triples_of_primitive_primitive_and_natural<T: PrimitiveInteger + Rand>(
    scale: u32,
) -> It<(T, T, Natural)> {
    Box::new(random_triples(
        &EXAMPLE_SEED,
        &(|seed| random(seed)),
        &(|seed| random(seed)),
        &(|seed| random_naturals(seed, scale)),
    ))
}

fn random_triples_of_natural_primitive_and_primitive<T: PrimitiveInteger + Rand>(
    scale: u32,
) -> It<(Natural, T, T)> {
    Box::new(random_triples(
        &EXAMPLE_SEED,
        &(|seed| random_naturals(seed, scale)),
        &(|seed| random(seed)),
        &(|seed| random(seed)),
    ))
}

pub fn triples_of_natural_unsigned_and_natural<T: PrimitiveUnsigned + Rand>(
    gm: GenerationMode,
) -> It<(Natural, T, Natural)> {
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

pub fn triples_of_unsigned_natural_and_unsigned<T: PrimitiveUnsigned + Rand>(
    gm: GenerationMode,
) -> It<(T, Natural, T)> {
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

pub fn triples_of_natural_natural_and_small_unsigned<T: PrimitiveUnsigned + Rand>(
    gm: GenerationMode,
) -> It<(Natural, Natural, T)> {
    match gm {
        GenerationMode::Exhaustive => Box::new(exhaustive_triples(
            exhaustive_naturals(),
            exhaustive_naturals(),
            exhaustive_unsigned(),
        )),
        GenerationMode::Random(scale) => Box::new(random_triples(
            &EXAMPLE_SEED,
            &(|seed| random_naturals(seed, scale)),
            &(|seed| random_naturals(seed, scale)),
            &(|seed| u32s_geometric(seed, scale).flat_map(T::checked_from)),
        )),
        GenerationMode::SpecialRandom(scale) => Box::new(random_triples(
            &EXAMPLE_SEED,
            &(|seed| special_random_naturals(seed, scale)),
            &(|seed| special_random_naturals(seed, scale)),
            &(|seed| u32s_geometric(seed, scale).flat_map(T::checked_from)),
        )),
    }
}

pub fn rm_triples_of_natural_natural_and_small_unsigned<T: PrimitiveUnsigned + Rand>(
    gm: GenerationMode,
) -> It<((rug::Integer, rug::Integer, T), (Natural, Natural, T))> {
    Box::new(
        triples_of_natural_natural_and_small_unsigned(gm).map(|(x, y, z)| {
            (
                (natural_to_rug_integer(&x), natural_to_rug_integer(&y), z),
                (x, y, z),
            )
        }),
    )
}

// All triples of `Natural`, `Natural`, and small `T`, where `T` is unsigned and the `Natural`s are
// equal mod 2 to the power of the `T`.
pub fn triples_of_natural_natural_and_small_unsigned_var_1<T: PrimitiveUnsigned + Rand>(
    gm: GenerationMode,
) -> It<(Natural, Natural, T)>
where
    Natural: Shl<T, Output = Natural>,
{
    Box::new(
        triples_of_natural_natural_and_small_unsigned(gm)
            .map(|(x, y, pow)| ((x << pow) + &y, y, pow)),
    )
}

// All triples of `Natural`, `Natural`, and small `T`, where `T` is unsigned and the `Natural`s are
// not equal mod 2 to the power of the `T`.
pub fn triples_of_natural_natural_and_small_unsigned_var_2<T: PrimitiveUnsigned + Rand>(
    gm: GenerationMode,
) -> It<(Natural, Natural, T)> {
    Box::new(
        triples_of_natural_natural_and_small_unsigned::<T>(gm)
            .filter(|&(ref x, ref y, pow)| !x.eq_mod_power_of_two(y, pow.checked_into().unwrap())),
    )
}

pub fn triples_of_unsigned_unsigned_and_natural<T: PrimitiveUnsigned + Rand>(
    gm: GenerationMode,
) -> It<(T, T, Natural)> {
    match gm {
        GenerationMode::Exhaustive => Box::new(exhaustive_triples(
            exhaustive_unsigned(),
            exhaustive_unsigned(),
            exhaustive_naturals(),
        )),
        GenerationMode::Random(scale) => random_triples_of_primitive_primitive_and_natural(scale),
        GenerationMode::SpecialRandom(scale) => Box::new(random_triples(
            &EXAMPLE_SEED,
            &(|seed| special_random_unsigned(seed)),
            &(|seed| special_random_unsigned(seed)),
            &(|seed| special_random_naturals(seed, scale)),
        )),
    }
}

pub fn triples_of_natural_unsigned_and_unsigned<T: PrimitiveUnsigned + Rand>(
    gm: GenerationMode,
) -> It<(Natural, T, T)> {
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

pub fn rm_triples_of_natural_unsigned_and_unsigned<T: PrimitiveUnsigned + Rand>(
    gm: GenerationMode,
) -> It<((rug::Integer, T, T), (Natural, T, T))> {
    Box::new(
        triples_of_natural_unsigned_and_unsigned(gm)
            .map(|(x, y, z)| ((natural_to_rug_integer(&x), y, z), (x, y, z))),
    )
}

// All triples of `Natural`, `T`, `T`, where `T` is unsigned and the `Natural` is equal to the first
// `T` mod the second `T`.
pub fn triples_of_natural_unsigned_and_unsigned_var_1<T: PrimitiveUnsigned + Rand>(
    gm: GenerationMode,
) -> It<(Natural, T, T)>
where
    Natural: Mul<T, Output = Natural> + Add<T, Output = Natural>,
{
    Box::new(
        triples_of_natural_unsigned_and_unsigned(gm)
            .map(|(n, u, modulus)| (n * modulus + u, u, modulus)),
    )
}

// All triples of `Natural`, `Limb`, `Limb`, where and the `Natural` is not equal to the first
// `Limb` mod the second `Limb`.
pub fn triples_of_natural_limb_and_limb_var_2(gm: GenerationMode) -> It<(Natural, Limb, Limb)> {
    Box::new(
        triples_of_natural_unsigned_and_unsigned::<Limb>(gm).filter(|&(ref n, u, modulus)| {
            let u: Limb = u.checked_into().unwrap();
            let modulus: Limb = modulus.checked_into().unwrap();
            !n.eq_mod(u, modulus)
        }),
    )
}

pub fn triples_of_natural_unsigned_and_small_unsigned<
    T: PrimitiveUnsigned + Rand,
    U: PrimitiveUnsigned + Rand,
>(
    gm: GenerationMode,
) -> It<(Natural, T, U)> {
    match gm {
        GenerationMode::Exhaustive => Box::new(exhaustive_triples(
            exhaustive_naturals(),
            exhaustive_unsigned(),
            exhaustive_unsigned(),
        )),
        GenerationMode::Random(scale) => Box::new(random_triples(
            &EXAMPLE_SEED,
            &(|seed| random_naturals(seed, scale)),
            &(|seed| random(seed)),
            &(|seed| u32s_geometric(seed, scale).flat_map(U::checked_from)),
        )),
        GenerationMode::SpecialRandom(scale) => Box::new(random_triples(
            &EXAMPLE_SEED,
            &(|seed| special_random_naturals(seed, scale)),
            &(|seed| special_random_unsigned(seed)),
            &(|seed| u32s_geometric(seed, scale).flat_map(U::checked_from)),
        )),
    }
}

pub fn triples_of_unsigned_natural_and_small_unsigned<
    T: PrimitiveUnsigned + Rand,
    U: PrimitiveUnsigned + Rand,
>(
    gm: GenerationMode,
) -> It<(T, Natural, U)> {
    match gm {
        GenerationMode::Exhaustive => Box::new(exhaustive_triples(
            exhaustive_unsigned(),
            exhaustive_naturals(),
            exhaustive_unsigned(),
        )),
        GenerationMode::Random(scale) => Box::new(random_triples(
            &EXAMPLE_SEED,
            &(|seed| random(seed)),
            &(|seed| random_naturals(seed, scale)),
            &(|seed| u32s_geometric(seed, scale).flat_map(U::checked_from)),
        )),
        GenerationMode::SpecialRandom(scale) => Box::new(random_triples(
            &EXAMPLE_SEED,
            &(|seed| special_random_unsigned(seed)),
            &(|seed| special_random_naturals(seed, scale)),
            &(|seed| u32s_geometric(seed, scale).flat_map(U::checked_from)),
        )),
    }
}

pub fn rm_triples_of_natural_unsigned_and_small_unsigned<
    T: PrimitiveUnsigned + Rand,
    U: PrimitiveUnsigned + Rand,
>(
    gm: GenerationMode,
) -> It<((rug::Integer, T, U), (Natural, T, U))> {
    Box::new(
        triples_of_natural_unsigned_and_small_unsigned(gm)
            .map(|(x, y, z)| ((natural_to_rug_integer(&x), y, z), (x, y, z))),
    )
}

// All triples of `Natural`, `T`, and small `U`, where `T` and `U` are unsigned and the `Natural` is
// equal to the `T` mod 2 to the power of the `T`.
pub fn triples_of_natural_unsigned_and_small_unsigned_var_1<
    T: PrimitiveUnsigned + Rand,
    U: PrimitiveUnsigned + Rand,
>(
    gm: GenerationMode,
) -> It<(Natural, T, U)>
where
    Natural: Shl<U, Output = Natural> + Add<T, Output = Natural>,
{
    Box::new(
        triples_of_natural_unsigned_and_small_unsigned(gm)
            .map(|(n, u, pow)| ((n << pow) + u, u, pow)),
    )
}

// All triples of `Natural`, `Limb`, and small `T`, where `T` is unsigned and the `Natural` is not
// equal to the `Limb` mod 2 to the power of the `T`.
pub fn triples_of_natural_limb_and_small_unsigned_var_2<T: PrimitiveUnsigned + Rand>(
    gm: GenerationMode,
) -> It<(Natural, Limb, T)> {
    Box::new(
        triples_of_natural_unsigned_and_small_unsigned::<Limb, T>(gm)
            .filter(|&(ref n, u, pow)| !n.eq_mod_power_of_two(u, pow.checked_into().unwrap())),
    )
}

pub fn triples_of_natural_small_u64_and_bool(gm: GenerationMode) -> It<(Natural, u64, bool)> {
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
) -> It<((rug::Integer, u64, bool), (Natural, u64, bool))> {
    Box::new(
        triples_of_natural_small_u64_and_bool(gm)
            .map(|(x, y, z)| ((natural_to_rug_integer(&x), y, z), (x, y, z))),
    )
}

pub fn pairs_of_natural_and_rounding_mode(gm: GenerationMode) -> It<(Natural, RoundingMode)> {
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

pub fn pairs_of_positive_natural_and_rounding_mode(
    gm: GenerationMode,
) -> It<(Natural, RoundingMode)> {
    match gm {
        GenerationMode::Exhaustive => Box::new(lex_pairs(
            exhaustive_positive_naturals(),
            exhaustive_rounding_modes(),
        )),
        GenerationMode::Random(scale) => Box::new(random_pairs(
            &EXAMPLE_SEED,
            &(|seed| random_positive_naturals(seed, scale)),
            &(|seed| random_rounding_modes(seed)),
        )),
        GenerationMode::SpecialRandom(scale) => Box::new(random_pairs(
            &EXAMPLE_SEED,
            &(|seed| special_random_positive_naturals(seed, scale)),
            &(|seed| random_rounding_modes(seed)),
        )),
    }
}

macro_rules! float_gen {
    (
        $f: ident,
        $finite_floats: ident,
        $pairs_of_natural_and_rounding_mode_var_1: ident,
        $naturals_exactly_equal_to_float: ident,
        $floats_exactly_equal_to_natural: ident,
        $naturals_not_exactly_equal_to_float: ident,
        $floats_var_2: ident,
        $floats_var_3: ident,
        $naturals_var_2: ident
    ) => {
        pub fn $pairs_of_natural_and_rounding_mode_var_1(
            gm: GenerationMode,
        ) -> It<(Natural, RoundingMode)> {
            Box::new(
                pairs_of_natural_and_rounding_mode(gm)
                    .filter(|&(ref n, rm)| rm != RoundingMode::Exact || $f::convertible_from(n)),
            )
        }

        pub fn $naturals_exactly_equal_to_float(gm: GenerationMode) -> It<Natural> {
            Box::new(naturals(gm).filter(|n| $f::convertible_from(n)))
        }

        pub fn $floats_exactly_equal_to_natural(gm: GenerationMode) -> It<$f> {
            Box::new(naturals(gm).flat_map($f::checked_from))
        }

        pub fn $naturals_not_exactly_equal_to_float(gm: GenerationMode) -> It<Natural> {
            let n = Natural::from($f::SMALLEST_UNREPRESENTABLE_UINT);
            let xs: It<Natural> = match gm {
                GenerationMode::Exhaustive => Box::new(range_up_increasing_natural(n)),
                GenerationMode::Random(scale) => {
                    Box::new(random_range_up_natural(&EXAMPLE_SEED, scale, n))
                }
                GenerationMode::SpecialRandom(scale) => {
                    Box::new(special_random_range_up_natural(&EXAMPLE_SEED, scale, n))
                }
            };
            Box::new(xs.filter(|n| !$f::convertible_from(n)))
        }

        // floats that are positive, not infinite, not NaN, and not exactly equal to a Natural.
        pub fn $floats_var_2(gm: GenerationMode) -> It<$f> {
            Box::new($finite_floats(gm).filter(|&f| f > 0.0 && !Natural::convertible_from(f)))
        }

        // positive floats exactly in between two adjacent Naturals.
        pub fn $floats_var_3(gm: GenerationMode) -> It<$f> {
            Box::new($floats_exactly_equal_to_natural(gm).flat_map(|f| {
                let f_plus_half = f + 0.5;
                if Natural::checked_from(f_plus_half).is_some() {
                    None
                } else {
                    Some(f_plus_half)
                }
            }))
        }

        // Naturals exactly in between two adjacent floats.
        pub fn $naturals_var_2(gm: GenerationMode) -> It<Natural> {
            Box::new($naturals_not_exactly_equal_to_float(gm).flat_map(|n| {
                let f_below = $f::rounding_from(&n, RoundingMode::Floor);
                let on_below = Natural::checked_from(f_below);
                if on_below.is_none() {
                    return None;
                }
                let n_below = on_below.unwrap();
                let mut f_above = f_below;
                f_above.increment();
                let on_above = Natural::checked_from(f_above);
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
    pairs_of_natural_and_rounding_mode_var_1_f32,
    naturals_exactly_equal_to_f32,
    f32s_exactly_equal_to_natural,
    naturals_not_exactly_equal_to_f32,
    f32s_var_2,
    f32s_var_3,
    naturals_var_2_f32
);
float_gen!(
    f64,
    finite_f64s,
    pairs_of_natural_and_rounding_mode_var_1_f64,
    naturals_exactly_equal_to_f64,
    f64s_exactly_equal_to_natural,
    naturals_not_exactly_equal_to_f64,
    f64s_var_2,
    f64s_var_3,
    naturals_var_2_f64
);

fn triples_of_natural_positive_natural_and_rounding_mode(
    gm: GenerationMode,
) -> It<(Natural, Natural, RoundingMode)> {
    match gm {
        GenerationMode::Exhaustive => reshape_2_1_to_3(Box::new(lex_pairs(
            exhaustive_pairs(exhaustive_naturals(), exhaustive_positive_naturals()),
            exhaustive_rounding_modes(),
        ))),
        GenerationMode::Random(scale) => Box::new(random_triples(
            &EXAMPLE_SEED,
            &(|seed| random_naturals(seed, scale)),
            &(|seed| random_positive_naturals(seed, scale)),
            &(|seed| random_rounding_modes(seed)),
        )),
        GenerationMode::SpecialRandom(scale) => Box::new(random_triples(
            &EXAMPLE_SEED,
            &(|seed| special_random_naturals(seed, scale)),
            &(|seed| special_random_positive_naturals(seed, scale)),
            &(|seed| random_rounding_modes(seed)),
        )),
    }
}

// All triples of `Natural`, positive `Nagtural`, and `RoundingMode`, where if the `RoundingMode` is
// `RoundingMode::Exact`, the first `Natural` is divisible by the second.
pub fn triples_of_natural_positive_natural_and_rounding_mode_var_1(
    gm: GenerationMode,
) -> It<(Natural, Natural, RoundingMode)> {
    Box::new(
        triples_of_natural_positive_natural_and_rounding_mode(gm).map(|(x, y, rm)| {
            if rm == RoundingMode::Exact {
                (x * &y, y, rm)
            } else {
                (x, y, rm)
            }
        }),
    )
}

fn triples_of_natural_small_signed_and_rounding_mode<T: PrimitiveSigned + Rand>(
    gm: GenerationMode,
) -> It<(Natural, T, RoundingMode)> {
    match gm {
        GenerationMode::Exhaustive => reshape_2_1_to_3(Box::new(lex_pairs(
            log_pairs_of_natural_and_signed(),
            exhaustive_rounding_modes(),
        ))),
        GenerationMode::Random(scale) => Box::new(random_triples(
            &EXAMPLE_SEED,
            &(|seed| random_naturals(seed, scale)),
            &(|seed| i32s_geometric(seed, scale).flat_map(|i| T::checked_from(i))),
            &(|seed| random_rounding_modes(seed)),
        )),
        GenerationMode::SpecialRandom(scale) => Box::new(random_triples(
            &EXAMPLE_SEED,
            &(|seed| special_random_naturals(seed, scale)),
            &(|seed| i32s_geometric(seed, scale).flat_map(|i| T::checked_from(i))),
            &(|seed| random_rounding_modes(seed)),
        )),
    }
}

// All triples of `Natural`, small `T`, and `RoundingMode`, where `T` is signed, such that if the
// `T` is negative and the `RoundingMode` is `RoundingMode::Exact`, the `Natural` is divisible by 2
// to the power of the negative of the `T`.
pub fn triples_of_natural_small_signed_and_rounding_mode_var_1<T: PrimitiveSigned + Rand>(
    gm: GenerationMode,
) -> It<(Natural, T, RoundingMode)>
where
    Natural: Shr<T, Output = Natural>,
{
    Box::new(
        triples_of_natural_small_signed_and_rounding_mode::<T>(gm).map(|(n, i, rm)| {
            (
                if i < T::ZERO && rm == RoundingMode::Exact {
                    n >> i
                } else {
                    n
                },
                i,
                rm,
            )
        }),
    )
}

// All triples of `Natural`, small `T`, and `RoundingMode`, where `T` is signed, such that if the
// `i32` is positive and the `RoundingMode` is `RoundingMode::Exact`, the `Natural` is divisible by
// 2 to the power of the `T`.
pub fn triples_of_natural_small_signed_and_rounding_mode_var_2<T: PrimitiveSigned + Rand>(
    gm: GenerationMode,
) -> It<(Natural, T, RoundingMode)>
where
    Natural: Shl<T, Output = Natural>,
{
    Box::new(
        triples_of_natural_small_signed_and_rounding_mode(gm)
            .map(|(n, i, rm)| (if i > T::ZERO { n << i } else { n }, i, rm)),
    )
}

fn triples_of_natural_small_unsigned_and_rounding_mode<T: PrimitiveUnsigned + Rand>(
    gm: GenerationMode,
) -> It<(Natural, T, RoundingMode)> {
    match gm {
        GenerationMode::Exhaustive => reshape_2_1_to_3(Box::new(lex_pairs(
            log_pairs_of_natural_and_unsigned(),
            exhaustive_rounding_modes(),
        ))),
        GenerationMode::Random(scale) => Box::new(random_triples(
            &EXAMPLE_SEED,
            &(|seed| random_naturals(seed, scale)),
            &(|seed| u32s_geometric(seed, scale).flat_map(T::checked_from)),
            &(|seed| random_rounding_modes(seed)),
        )),
        GenerationMode::SpecialRandom(scale) => Box::new(random_triples(
            &EXAMPLE_SEED,
            &(|seed| special_random_naturals(seed, scale)),
            &(|seed| u32s_geometric(seed, scale).flat_map(T::checked_from)),
            &(|seed| random_rounding_modes(seed)),
        )),
    }
}

fn triples_of_natural_positive_unsigned_and_rounding_mode<T: PrimitiveUnsigned + Rand>(
    gm: GenerationMode,
) -> It<(Natural, T, RoundingMode)> {
    match gm {
        GenerationMode::Exhaustive => reshape_2_1_to_3(Box::new(lex_pairs(
            exhaustive_pairs(exhaustive_naturals(), exhaustive_positive()),
            exhaustive_rounding_modes(),
        ))),
        GenerationMode::Random(scale) => Box::new(random_triples(
            &EXAMPLE_SEED,
            &(|seed| random_naturals(seed, scale)),
            &(|seed| random_positive_unsigned(seed)),
            &(|seed| random_rounding_modes(seed)),
        )),
        GenerationMode::SpecialRandom(scale) => Box::new(random_triples(
            &EXAMPLE_SEED,
            &(|seed| special_random_naturals(seed, scale)),
            &(|seed| special_random_positive_unsigned(seed)),
            &(|seed| random_rounding_modes(seed)),
        )),
    }
}

// All triples of `Natural`, small `T`, and `RoundingMode`, where `T` is unsigned and if the
// `RoundingMode` is `RoundingMode::Exact`, the `Natural` is divisible by 2 to the power of the `T`.
pub fn triples_of_natural_small_unsigned_and_rounding_mode_var_1<T: PrimitiveUnsigned + Rand>(
    gm: GenerationMode,
) -> It<(Natural, T, RoundingMode)>
where
    Natural: Shl<T, Output = Natural>,
{
    Box::new(
        triples_of_natural_small_unsigned_and_rounding_mode::<T>(gm).map(|(n, u, rm)| {
            if rm == RoundingMode::Exact {
                (n << u, u, rm)
            } else {
                (n, u, rm)
            }
        }),
    )
}

// All triples of `Natural`, positive `T`, and `RoundingMode`, where `T` is unsigned and if the
// `RoundingMode` is `RoundingMode::Exact`, the `Natural` is divisible by the `T`.
pub fn triples_of_natural_positive_unsigned_and_rounding_mode_var_1<T: PrimitiveUnsigned + Rand>(
    gm: GenerationMode,
) -> It<(Natural, T, RoundingMode)>
where
    Natural: Mul<T, Output = Natural>,
{
    Box::new(
        triples_of_natural_positive_unsigned_and_rounding_mode::<T>(gm).map(|(n, u, rm)| {
            if rm == RoundingMode::Exact {
                (n * u, u, rm)
            } else {
                (n, u, rm)
            }
        }),
    )
}

fn triples_of_unsigned_positive_natural_and_rounding_mode<T: PrimitiveUnsigned + Rand>(
    gm: GenerationMode,
) -> It<(T, Natural, RoundingMode)> {
    match gm {
        GenerationMode::Exhaustive => reshape_2_1_to_3(Box::new(lex_pairs(
            exhaustive_pairs(exhaustive_unsigned(), exhaustive_positive_naturals()),
            exhaustive_rounding_modes(),
        ))),
        GenerationMode::Random(scale) => Box::new(random_triples(
            &EXAMPLE_SEED,
            &(|seed| random(seed)),
            &(|seed| random_positive_naturals(seed, scale)),
            &(|seed| random_rounding_modes(seed)),
        )),
        GenerationMode::SpecialRandom(scale) => Box::new(random_triples(
            &EXAMPLE_SEED,
            &(|seed| special_random_unsigned(seed)),
            &(|seed| special_random_positive_naturals(seed, scale)),
            &(|seed| random_rounding_modes(seed)),
        )),
    }
}

// All triples of `T`, positive `Natural`, and `RoundingMode`, where `T` is unsigned and if the
// `RoundingMode` is `RoundingMode::Exact`, the `T` is divisible by the `Natural`.
pub fn triples_of_unsigned_positive_natural_and_rounding_mode_var_1<T: PrimitiveUnsigned + Rand>(
    gm: GenerationMode,
) -> It<(T, Natural, RoundingMode)>
where
    T: Mul<Natural, Output = Natural>,
    T: CheckedFrom<Natural>,
{
    Box::new(
        triples_of_unsigned_positive_natural_and_rounding_mode::<T>(gm).filter_map(|(u, n, rm)| {
            if rm == RoundingMode::Exact {
                T::checked_from(u * n.clone()).map(|u| (u, n, rm))
            } else {
                Some((u, n, rm))
            }
        }),
    )
}

struct RandomNaturalAndVecOfBoolVar1 {
    naturals: It<Natural>,
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
pub fn pairs_of_natural_and_vec_of_bool_var_1(gm: GenerationMode) -> It<(Natural, Vec<bool>)> {
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
) -> It<(Natural, Vec<bool>)> {
    Box::new(pairs_of_natural_and_vec_of_bool_var_1(gm).filter(|&(ref n, _)| *n != 0 as Limb))
}

struct RandomNaturalAndVecOfBoolVar2 {
    naturals: It<Natural>,
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
pub fn pairs_of_natural_and_vec_of_bool_var_2(gm: GenerationMode) -> It<(Natural, Vec<bool>)> {
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

pub fn quadruples_of_natural_natural_natural_and_small_unsigned<T: PrimitiveUnsigned + Rand>(
    gm: GenerationMode,
) -> It<(Natural, Natural, Natural, T)> {
    match gm {
        GenerationMode::Exhaustive => Box::new(exhaustive_quadruples(
            exhaustive_naturals(),
            exhaustive_naturals(),
            exhaustive_naturals(),
            exhaustive_unsigned(),
        )),
        GenerationMode::Random(scale) => Box::new(random_quadruples(
            &EXAMPLE_SEED,
            &(|seed| random_naturals(seed, scale)),
            &(|seed| random_naturals(seed, scale)),
            &(|seed| random_naturals(seed, scale)),
            &(|seed| u32s_geometric(seed, scale).flat_map(T::checked_from)),
        )),
        GenerationMode::SpecialRandom(scale) => Box::new(random_quadruples(
            &EXAMPLE_SEED,
            &(|seed| special_random_naturals(seed, scale)),
            &(|seed| special_random_naturals(seed, scale)),
            &(|seed| special_random_naturals(seed, scale)),
            &(|seed| u32s_geometric(seed, scale).flat_map(T::checked_from)),
        )),
    }
}
