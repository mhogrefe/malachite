use std::iter::repeat;
use std::ops::{Shl, Shr};

use malachite_base::bools::exhaustive::exhaustive_bools;
use malachite_base::num::arithmetic::traits::{
    DivRound, DivisibleBy, DivisibleByPowerOfTwo, EqMod, EqModPowerOfTwo, PowerOfTwo,
};
use malachite_base::num::basic::integers::PrimitiveInt;
use malachite_base::num::basic::signeds::PrimitiveSigned;
use malachite_base::num::basic::traits::Zero;
use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base::num::conversion::traits::{
    CheckedFrom, ConvertibleFrom, ExactFrom, RoundingFrom, WrappingFrom,
};
use malachite_base::num::exhaustive::{
    exhaustive_natural_signeds, exhaustive_positive_primitive_ints, exhaustive_signeds,
    exhaustive_unsigneds, primitive_int_increasing_inclusive_range,
};
use malachite_base::num::floats::{increment_float, PrimitiveFloat};
use malachite_base::num::logic::traits::{LowMask, SignificantBits};
use malachite_base::rounding_modes::exhaustive::exhaustive_rounding_modes;
use malachite_base::rounding_modes::RoundingMode;
use malachite_base::tuples::exhaustive::{
    exhaustive_pairs, exhaustive_pairs_from_single, exhaustive_quadruples,
    exhaustive_quadruples_from_single, exhaustive_triples, exhaustive_triples_from_single,
    exhaustive_triples_xxy, exhaustive_triples_xyy, lex_pairs, lex_pairs_from_single,
    lex_triples_from_single,
};
use malachite_base::vecs::exhaustive::{
    exhaustive_fixed_length_vecs_from_single, exhaustive_vecs, shortlex_vecs,
};
use malachite_base_test_util::generators::common::It;
use malachite_base_test_util::generators::{exhaustive_pairs_big_small, exhaustive_pairs_big_tiny};
use malachite_nz::natural::exhaustive::{
    exhaustive_natural_range, exhaustive_natural_range_to_infinity, exhaustive_naturals,
    exhaustive_positive_naturals,
};
use malachite_nz::natural::Natural;
use malachite_nz_test_util::common::{natural_to_biguint, natural_to_rug_integer};
use num::BigUint;
use rand::{IsaacRng, Rand, Rng, SeedableRng};
use rug;
use rust_wheels::iterators::common::{scramble, EXAMPLE_SEED};
use rust_wheels::iterators::dependent_pairs::{
    dependent_pairs, exhaustive_dependent_pairs_infinite, exhaustive_dependent_pairs_infinite_log,
    exhaustive_dependent_pairs_infinite_sqrt, random_dependent_pairs,
};
use rust_wheels::iterators::general::random;
use rust_wheels::iterators::integers_geometric::{
    i32s_geometric, range_up_geometric_u32, u32s_geometric,
};
use rust_wheels::iterators::naturals::{
    random_naturals, random_positive_naturals, random_range_natural, random_range_up_natural,
    special_random_naturals, special_random_positive_naturals, special_random_range_natural,
    special_random_range_up_natural,
};
use rust_wheels::iterators::primitive_ints::{
    random_natural_signed, random_range, special_random_natural_signed, special_random_signed,
    special_random_unsigned,
};
use rust_wheels::iterators::rounding_modes::random_rounding_modes;
use rust_wheels::iterators::tuples::{
    random_pairs, random_pairs_from_single, random_quadruples, random_quadruples_from_single,
    random_triples, random_triples_from_single,
};
use rust_wheels::iterators::vecs::random_vecs;

use common::GenerationMode;
use inputs::base::{finite_f32s, finite_f64s, natural_signeds, unsigneds, RandomValueAndVecOfBool};
use inputs::common::{
    permute_1_3_4_2, permute_2_1, reshape_1_2_to_3, reshape_2_1_to_3, reshape_2_2_to_4,
    reshape_3_1_to_4,
};

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

pub fn naturals_var_1<T: PrimitiveUnsigned + Rand>(gm: GenerationMode) -> It<Natural>
where
    Natural: From<T>,
{
    Box::new(unsigneds::<T>(gm).map(Natural::from))
}

pub fn naturals_var_2<T: PrimitiveSigned + Rand>(gm: GenerationMode) -> It<Natural>
where
    Natural: ExactFrom<T>,
    T::UnsignedOfEqualWidth: Rand,
    T: WrappingFrom<<T as PrimitiveSigned>::UnsignedOfEqualWidth>,
{
    Box::new(natural_signeds::<T>(gm).map(Natural::exact_from))
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

//TODO use subset_pairs
// All pairs of `Natural`s where the first is smaller than the second.
pub fn pairs_of_naturals_var_2(gm: GenerationMode) -> It<(Natural, Natural)> {
    Box::new(pairs_of_naturals(gm).filter(|&(ref x, ref y)| x < y))
}

// All pairs of `Natural`s where the first equals the second.
pub fn pairs_of_naturals_var_3(gm: GenerationMode) -> It<(Natural, Natural)> {
    Box::new(naturals(gm).map(|x| (x.clone(), x)))
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

pub fn rm_triples_of_naturals(
    gm: GenerationMode,
) -> It<(
    (rug::Integer, rug::Integer, rug::Integer),
    (Natural, Natural, Natural),
)> {
    Box::new(triples_of_naturals(gm).map(|(x, y, z)| {
        (
            (
                natural_to_rug_integer(&x),
                natural_to_rug_integer(&y),
                natural_to_rug_integer(&z),
            ),
            (x, y, z),
        )
    }))
}

// All triples of `Natural`s where the first is greater than or equal to the product of the second
// and third.
pub fn triples_of_naturals_var_1(gm: GenerationMode) -> It<(Natural, Natural, Natural)> {
    Box::new(triples_of_naturals(gm).filter(|&(ref a, ref b, ref c)| a >= &(b * c)))
}

// All triples of `Natural`s, where the first `Natural` is equal to the second mod the third.
pub fn triples_of_naturals_var_2(gm: GenerationMode) -> It<(Natural, Natural, Natural)> {
    Box::new(triples_of_naturals(gm).map(|(x, y, m)| (x * &m + &y, y, m)))
}

// All triples of `Natural`s, where the first `Natural` is not equal to the second mod the third.
pub fn triples_of_naturals_var_3(gm: GenerationMode) -> It<(Natural, Natural, Natural)> {
    Box::new(triples_of_naturals(gm).filter(|&(ref x, ref y, ref m)| !x.eq_mod(y, m)))
}

// All triples of `Natural`s where the first and the second are smaller than the third.
pub fn triples_of_naturals_var_4(gm: GenerationMode) -> It<(Natural, Natural, Natural)> {
    Box::new(triples_of_naturals(gm).filter(|&(ref x, ref y, ref m)| x < m && y < m))
}

// All triples of `Natural`s where the first is smaller than the third.
pub fn triples_of_naturals_var_5(gm: GenerationMode) -> It<(Natural, Natural, Natural)> {
    Box::new(triples_of_naturals(gm).filter(|&(ref x, _, ref m)| x < m))
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

fn random_pairs_of_natural_and_primitive<T: PrimitiveInt + Rand>(scale: u32) -> It<(Natural, T)> {
    Box::new(random_pairs(
        &EXAMPLE_SEED,
        &(|seed| random_naturals(seed, scale)),
        &(|seed| random(seed)),
    ))
}

fn random_pairs_of_primitive_and_natural<T: PrimitiveInt + Rand>(scale: u32) -> It<(T, Natural)> {
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
            exhaustive_unsigneds(),
        )),
        GenerationMode::Random(scale) => random_pairs_of_natural_and_primitive(scale),
        GenerationMode::SpecialRandom(scale) => Box::new(random_pairs(
            &EXAMPLE_SEED,
            &(|seed| special_random_naturals(seed, scale)),
            &(|seed| special_random_unsigned(seed)),
        )),
    }
}

pub fn pairs_of_positive_natural_and_unsigned<T: PrimitiveUnsigned + Rand>(
    gm: GenerationMode,
) -> It<(Natural, T)> {
    match gm {
        GenerationMode::Exhaustive => Box::new(exhaustive_pairs(
            exhaustive_positive_naturals(),
            exhaustive_unsigneds(),
        )),
        GenerationMode::Random(scale) => Box::new(random_pairs(
            &EXAMPLE_SEED,
            &(|seed| random_positive_naturals(seed, scale)),
            &(|seed| random(seed)),
        )),
        GenerationMode::SpecialRandom(scale) => Box::new(random_pairs(
            &EXAMPLE_SEED,
            &(|seed| special_random_positive_naturals(seed, scale)),
            &(|seed| special_random_unsigned(seed)),
        )),
    }
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

// All pairs of `Natural` and `u64`, where the `u64` is greater than or equal to the number of
// significant bits of the `Natural`.
pub fn pairs_of_natural_and_u64_var_1(gm: GenerationMode) -> It<(Natural, u64)> {
    let ps: It<(u64, Natural)> = match gm {
        GenerationMode::Exhaustive => Box::new(
            exhaustive_dependent_pairs_infinite_log((), exhaustive_unsigneds(), |_, &pow| {
                Box::new(
                    exhaustive_natural_range(Natural::ZERO, Natural::power_of_two(pow))
                        .map(Option::Some)
                        .chain(repeat(None)),
                )
            })
            .flat_map(|(pow, n)| {
                if let Some(n) = n {
                    Some((pow, n))
                } else {
                    None
                }
            }),
        ),
        GenerationMode::Random(scale) => Box::new(random_dependent_pairs(
            (),
            u32s_geometric(&scramble(&EXAMPLE_SEED, "pow"), scale).map(u64::from),
            |_, &pow| {
                random_range_natural(
                    &scramble(&EXAMPLE_SEED, "n"),
                    Natural::ZERO,
                    Natural::low_mask(pow),
                )
            },
        )),
        GenerationMode::SpecialRandom(scale) => Box::new(random_dependent_pairs(
            (),
            u32s_geometric(&scramble(&EXAMPLE_SEED, "pow"), scale).map(u64::from),
            |_, &pow| {
                special_random_range_natural(
                    &scramble(&EXAMPLE_SEED, "n"),
                    Natural::ZERO,
                    Natural::low_mask(pow),
                )
            },
        )),
    };
    permute_2_1(ps)
}

pub fn pairs_of_unsigned_and_natural<T: PrimitiveUnsigned + Rand>(
    gm: GenerationMode,
) -> It<(T, Natural)> {
    match gm {
        GenerationMode::Exhaustive => Box::new(exhaustive_pairs(
            exhaustive_unsigneds(),
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

pub fn pairs_of_natural_and_signed<T: PrimitiveSigned + Rand>(
    gm: GenerationMode,
) -> It<(Natural, T)>
where
    T::UnsignedOfEqualWidth: Rand,
    T: WrappingFrom<<T as PrimitiveSigned>::UnsignedOfEqualWidth>,
{
    match gm {
        GenerationMode::Exhaustive => Box::new(exhaustive_pairs(
            exhaustive_naturals(),
            exhaustive_signeds(),
        )),
        GenerationMode::Random(scale) => random_pairs_of_natural_and_primitive(scale),
        GenerationMode::SpecialRandom(scale) => Box::new(random_pairs(
            &EXAMPLE_SEED,
            &(|seed| special_random_naturals(seed, scale)),
            &(|seed| special_random_signed(seed)),
        )),
    }
}

pub fn pairs_of_positive_natural_and_signed<T: PrimitiveSigned + Rand>(
    gm: GenerationMode,
) -> It<(Natural, T)>
where
    T::UnsignedOfEqualWidth: Rand,
    T: WrappingFrom<<T as PrimitiveSigned>::UnsignedOfEqualWidth>,
{
    match gm {
        GenerationMode::Exhaustive => Box::new(exhaustive_pairs(
            exhaustive_positive_naturals(),
            exhaustive_signeds(),
        )),
        GenerationMode::Random(scale) => Box::new(random_pairs(
            &EXAMPLE_SEED,
            &(|seed| random_positive_naturals(seed, scale)),
            &(|seed| random(seed)),
        )),
        GenerationMode::SpecialRandom(scale) => Box::new(random_pairs(
            &EXAMPLE_SEED,
            &(|seed| special_random_positive_naturals(seed, scale)),
            &(|seed| special_random_signed(seed)),
        )),
    }
}

pub fn rm_pairs_of_natural_and_signed<T: PrimitiveSigned + Rand>(
    gm: GenerationMode,
) -> It<((rug::Integer, T), (Natural, T))>
where
    T::UnsignedOfEqualWidth: Rand,
    T: WrappingFrom<<T as PrimitiveSigned>::UnsignedOfEqualWidth>,
{
    Box::new(
        pairs_of_natural_and_signed(gm).map(|(x, y)| ((natural_to_rug_integer(&x), y), (x, y))),
    )
}

pub fn pairs_of_signed_and_natural<T: PrimitiveSigned + Rand>(
    gm: GenerationMode,
) -> It<(T, Natural)>
where
    T::UnsignedOfEqualWidth: Rand,
    T: WrappingFrom<<T as PrimitiveSigned>::UnsignedOfEqualWidth>,
{
    match gm {
        GenerationMode::Exhaustive => Box::new(exhaustive_pairs(
            exhaustive_signeds(),
            exhaustive_naturals(),
        )),
        GenerationMode::Random(scale) => random_pairs_of_primitive_and_natural(scale),
        GenerationMode::SpecialRandom(scale) => Box::new(random_pairs(
            &EXAMPLE_SEED,
            &(|seed| special_random_signed(seed)),
            &(|seed| special_random_naturals(seed, scale)),
        )),
    }
}

pub fn rm_pairs_of_signed_and_natural<T: PrimitiveSigned + Rand>(
    gm: GenerationMode,
) -> It<((T, rug::Integer), (T, Natural))>
where
    T::UnsignedOfEqualWidth: Rand,
    T: WrappingFrom<<T as PrimitiveSigned>::UnsignedOfEqualWidth>,
{
    Box::new(
        pairs_of_signed_and_natural(gm).map(|(x, y)| ((x, natural_to_rug_integer(&y)), (x, y))),
    )
}

pub fn pairs_of_natural_and_natural_signed<T: PrimitiveSigned + Rand>(
    gm: GenerationMode,
) -> It<(Natural, T)>
where
    T::UnsignedOfEqualWidth: Rand,
    T: WrappingFrom<<T as PrimitiveSigned>::UnsignedOfEqualWidth>,
{
    match gm {
        GenerationMode::Exhaustive => Box::new(exhaustive_pairs(
            exhaustive_naturals(),
            exhaustive_natural_signeds(),
        )),
        GenerationMode::Random(scale) => Box::new(random_pairs(
            &EXAMPLE_SEED,
            &(|seed| random_naturals(seed, scale)),
            &(|seed| random_natural_signed(seed)),
        )),
        GenerationMode::SpecialRandom(scale) => Box::new(random_pairs(
            &EXAMPLE_SEED,
            &(|seed| special_random_naturals(seed, scale)),
            &(|seed| special_random_natural_signed(seed)),
        )),
    }
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
    Box::new(pairs_of_natural_and_positive_natural(gm).map(|(x, y)| (x * &y, y)))
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

fn log_pairs_of_natural_and_unsigned<T: PrimitiveUnsigned>() -> It<(Natural, T)> {
    Box::new(exhaustive_pairs_big_tiny(
        exhaustive_naturals(),
        exhaustive_unsigneds(),
    ))
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
            .filter(|&(ref n, u)| !n.divisible_by_power_of_two(u.exact_into())),
    )
}

// All pairs of `Natural` and positive small unsigned.
pub fn pairs_of_natural_and_small_unsigned_var_3<T: PrimitiveUnsigned + Rand>(
    gm: GenerationMode,
) -> It<(Natural, T)> {
    Box::new(pairs_of_natural_and_small_unsigned(gm).filter(|&(_, u)| u != T::ZERO))
}

// All pairs of `Natural` and `u64`, where the `u64` is between 1 and `T::WIDTH`, inclusive.
pub fn pairs_of_natural_and_small_u64_var_3<T: PrimitiveUnsigned>(
    gm: GenerationMode,
) -> It<(Natural, u64)> {
    match gm {
        GenerationMode::Exhaustive => Box::new(lex_pairs(
            exhaustive_naturals(),
            primitive_int_increasing_inclusive_range(1, T::WIDTH),
        )),
        GenerationMode::Random(scale) => Box::new(random_pairs(
            &EXAMPLE_SEED,
            &(|seed| random_naturals(seed, scale)),
            &(|seed| random_range(seed, 1, T::WIDTH)),
        )),
        GenerationMode::SpecialRandom(scale) => Box::new(random_pairs(
            &EXAMPLE_SEED,
            &(|seed| special_random_naturals(seed, scale)),
            &(|seed| random_range(seed, 1, T::WIDTH)),
        )),
    }
}

pub fn pairs_of_positive_natural_and_small_unsigned<T: PrimitiveUnsigned + Rand>(
    gm: GenerationMode,
) -> It<(Natural, T)> {
    match gm {
        GenerationMode::Exhaustive => Box::new(exhaustive_pairs_big_tiny(
            exhaustive_positive_naturals(),
            exhaustive_unsigneds(),
        )),
        GenerationMode::Random(scale) => Box::new(random_pairs(
            &EXAMPLE_SEED,
            &(|seed| random_positive_naturals(seed, scale)),
            &(|seed| u32s_geometric(seed, scale).flat_map(T::checked_from)),
        )),
        GenerationMode::SpecialRandom(scale) => Box::new(random_pairs(
            &EXAMPLE_SEED,
            &(|seed| special_random_positive_naturals(seed, scale)),
            &(|seed| u32s_geometric(seed, scale).flat_map(T::checked_from)),
        )),
    }
}

fn log_pairs_of_natural_and_signed<T: PrimitiveSigned>() -> It<(Natural, T)> {
    Box::new(exhaustive_pairs_big_tiny(
        exhaustive_naturals(),
        exhaustive_signeds(),
    ))
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

pub fn nm_pairs_of_natural_and_small_unsigned<T: PrimitiveUnsigned + Rand>(
    gm: GenerationMode,
) -> It<((BigUint, T), (Natural, T))> {
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
        GenerationMode::Exhaustive => reshape_1_2_to_3(Box::new(exhaustive_pairs_big_tiny(
            exhaustive_naturals(),
            exhaustive_pairs_from_single(exhaustive_unsigneds()),
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

// All triples of `Natural`, small `T`, and small `T`, where `T` is unsigned and the first `T` is
// less than or equal to the second.
pub fn triples_of_natural_small_unsigned_and_small_unsigned_var_1<T: PrimitiveUnsigned + Rand>(
    gm: GenerationMode,
) -> It<(Natural, T, T)> {
    Box::new(
        triples_of_natural_small_unsigned_and_small_unsigned(gm)
            .filter(|&(_, start, end)| start <= end),
    )
}

// All triples of `Natural`, `u64`, and small `u64`, where the first `u64` is between 1 and
// `T::WIDTH`, inclusive.
pub fn triples_of_natural_small_u64_and_small_u64_var_2<T: PrimitiveUnsigned>(
    gm: GenerationMode,
) -> It<(Natural, u64, u64)> {
    match gm {
        GenerationMode::Exhaustive => Box::new(exhaustive_triples(
            exhaustive_naturals(),
            primitive_int_increasing_inclusive_range(1, T::WIDTH),
            exhaustive_unsigneds(),
        )),
        GenerationMode::Random(scale) => Box::new(random_triples(
            &EXAMPLE_SEED,
            &(|seed| random_naturals(seed, scale)),
            &(|seed| random_range(seed, 1, T::WIDTH)),
            &(|seed| u32s_geometric(seed, scale).map(u64::from)),
        )),
        GenerationMode::SpecialRandom(scale) => Box::new(random_triples(
            &EXAMPLE_SEED,
            &(|seed| special_random_naturals(seed, scale)),
            &(|seed| random_range(seed, 1, T::WIDTH)),
            &(|seed| u32s_geometric(seed, scale).map(u64::from)),
        )),
    }
}

// All triples of `Natural`, `u64`, and small `u64`, where the first `u64` is positive.
pub fn triples_of_natural_small_u64_and_small_u64_var_3(
    gm: GenerationMode,
) -> It<(Natural, u64, u64)> {
    match gm {
        GenerationMode::Exhaustive => Box::new(exhaustive_triples(
            exhaustive_naturals(),
            exhaustive_positive_primitive_ints(),
            exhaustive_unsigneds(),
        )),
        GenerationMode::Random(scale) => Box::new(random_triples(
            &EXAMPLE_SEED,
            &(|seed| random_naturals(seed, scale)),
            &(|seed| range_up_geometric_u32(seed, scale, 1).map(u64::from)),
            &(|seed| u32s_geometric(seed, scale).map(u64::from)),
        )),
        GenerationMode::SpecialRandom(scale) => Box::new(random_triples(
            &EXAMPLE_SEED,
            &(|seed| special_random_naturals(seed, scale)),
            &(|seed| range_up_geometric_u32(seed, scale, 1).map(u64::from)),
            &(|seed| u32s_geometric(seed, scale).map(u64::from)),
        )),
    }
}

fn random_triples_of_natural_primitive_and_natural<T: PrimitiveInt + Rand>(
    scale: u32,
) -> It<(Natural, T, Natural)> {
    Box::new(random_triples(
        &EXAMPLE_SEED,
        &(|seed| random_naturals(seed, scale)),
        &(|seed| random(seed)),
        &(|seed| random_naturals(seed, scale)),
    ))
}

fn random_triples_of_primitive_natural_and_primitive<T: PrimitiveInt + Rand>(
    scale: u32,
) -> It<(T, Natural, T)> {
    Box::new(random_triples(
        &EXAMPLE_SEED,
        &(|seed| random(seed)),
        &(|seed| random_naturals(seed, scale)),
        &(|seed| random(seed)),
    ))
}

pub fn triples_of_natural_unsigned_and_natural<T: PrimitiveUnsigned + Rand>(
    gm: GenerationMode,
) -> It<(Natural, T, Natural)> {
    match gm {
        GenerationMode::Exhaustive => Box::new(exhaustive_triples(
            exhaustive_naturals(),
            exhaustive_unsigneds(),
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
            exhaustive_unsigneds(),
            exhaustive_naturals(),
            exhaustive_unsigneds(),
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

pub fn triples_of_natural_signed_and_natural<T: PrimitiveSigned + Rand>(
    gm: GenerationMode,
) -> It<(Natural, T, Natural)>
where
    T::UnsignedOfEqualWidth: Rand,
    T: WrappingFrom<<T as PrimitiveSigned>::UnsignedOfEqualWidth>,
{
    match gm {
        GenerationMode::Exhaustive => Box::new(exhaustive_triples(
            exhaustive_naturals(),
            exhaustive_signeds(),
            exhaustive_naturals(),
        )),
        GenerationMode::Random(scale) => Box::new(random_triples(
            &EXAMPLE_SEED,
            &(|seed| random_naturals(seed, scale)),
            &(|seed| random(seed)),
            &(|seed| random_naturals(seed, scale)),
        )),
        GenerationMode::SpecialRandom(scale) => Box::new(random_triples(
            &EXAMPLE_SEED,
            &(|seed| special_random_naturals(seed, scale)),
            &(|seed| special_random_signed(seed)),
            &(|seed| special_random_naturals(seed, scale)),
        )),
    }
}

pub fn triples_of_signed_natural_and_signed<T: PrimitiveSigned + Rand>(
    gm: GenerationMode,
) -> It<(T, Natural, T)>
where
    T::UnsignedOfEqualWidth: Rand,
    T: WrappingFrom<<T as PrimitiveSigned>::UnsignedOfEqualWidth>,
{
    match gm {
        GenerationMode::Exhaustive => Box::new(exhaustive_triples(
            exhaustive_signeds(),
            exhaustive_naturals(),
            exhaustive_signeds(),
        )),
        GenerationMode::Random(scale) => random_triples_of_primitive_natural_and_primitive(scale),
        GenerationMode::SpecialRandom(scale) => Box::new(random_triples(
            &EXAMPLE_SEED,
            &(|seed| special_random_signed(seed)),
            &(|seed| special_random_naturals(seed, scale)),
            &(|seed| special_random_signed(seed)),
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
            exhaustive_unsigneds(),
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
            .filter(|&(ref x, ref y, pow)| !x.eq_mod_power_of_two(y, pow.exact_into())),
    )
}

// All triples of `Natural`, `Natural` and `u64`, where the `u64` is greater than or equal to n,
// where n is the maximum number of significant bits of the `Natural`s.
pub fn triples_of_natural_natural_and_u64_var_1(gm: GenerationMode) -> It<(Natural, Natural, u64)> {
    let ps: It<(u64, (Natural, Natural))> = match gm {
        GenerationMode::Exhaustive => Box::new(dependent_pairs(exhaustive_unsigneds(), |&pow| {
            lex_pairs_from_single(exhaustive_natural_range(
                Natural::ZERO,
                Natural::power_of_two(pow),
            ))
        })),
        GenerationMode::Random(scale) => Box::new(random_dependent_pairs(
            (),
            u32s_geometric(&scramble(&EXAMPLE_SEED, "pow"), scale).map(u64::from),
            |_, &pow| {
                random_pairs_from_single(random_range_natural(
                    &scramble(&EXAMPLE_SEED, "n"),
                    Natural::ZERO,
                    Natural::low_mask(pow),
                ))
            },
        )),
        GenerationMode::SpecialRandom(scale) => Box::new(random_dependent_pairs(
            (),
            u32s_geometric(&scramble(&EXAMPLE_SEED, "pow"), scale).map(u64::from),
            |_, &pow| {
                random_pairs_from_single(special_random_range_natural(
                    &scramble(&EXAMPLE_SEED, "n"),
                    Natural::ZERO,
                    Natural::low_mask(pow),
                ))
            },
        )),
    };
    reshape_2_1_to_3(permute_2_1(ps))
}

// All triples of `Natural`, `Natural` and `u64`, where the `u64` is greater than or equal to the
// number of significant bits of the first `Natural`.
pub fn triples_of_natural_natural_and_u64_var_2(gm: GenerationMode) -> It<(Natural, Natural, u64)> {
    let ps: It<(u64, (Natural, Natural))> = match gm {
        GenerationMode::Exhaustive => Box::new(exhaustive_dependent_pairs_infinite(
            (),
            exhaustive_unsigneds(),
            |_, &pow| {
                exhaustive_pairs(
                    exhaustive_natural_range(Natural::ZERO, Natural::power_of_two(pow)),
                    exhaustive_naturals(),
                )
            },
        )),
        GenerationMode::Random(scale) => Box::new(random_dependent_pairs(
            scale,
            u32s_geometric(&scramble(&EXAMPLE_SEED, "pow"), scale).map(u64::from),
            |&scale, &pow| {
                random_pairs(
                    &scramble(&EXAMPLE_SEED, "n"),
                    &|seed| random_range_natural(seed, Natural::ZERO, Natural::low_mask(pow)),
                    &|seed| random_naturals(seed, scale),
                )
            },
        )),
        GenerationMode::SpecialRandom(scale) => Box::new(random_dependent_pairs(
            scale,
            u32s_geometric(&scramble(&EXAMPLE_SEED, "pow"), scale).map(u64::from),
            |&scale, &pow| {
                random_pairs(
                    &scramble(&EXAMPLE_SEED, "n"),
                    &|seed| {
                        special_random_range_natural(seed, Natural::ZERO, Natural::low_mask(pow))
                    },
                    &|seed| random_naturals(seed, scale),
                )
            },
        )),
    };
    reshape_2_1_to_3(permute_2_1(ps))
}

pub fn triples_of_natural_small_u64_and_bool(gm: GenerationMode) -> It<(Natural, u64, bool)> {
    match gm {
        GenerationMode::Exhaustive => reshape_2_1_to_3(Box::new(lex_pairs(
            exhaustive_pairs(exhaustive_naturals(), exhaustive_unsigneds()),
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

// All triples of `Natural`, `T` and `u64`, where `T` is unsigned and the `u64` is greater than or
// equal to the number of significant bits of the `Natural`.
pub fn triples_of_natural_small_unsigned_and_u64_var_1<T: PrimitiveUnsigned>(
    gm: GenerationMode,
) -> It<(Natural, T, u64)> {
    let ps: It<(u64, (Natural, T))> = match gm {
        GenerationMode::Exhaustive => Box::new(
            exhaustive_dependent_pairs_infinite((), exhaustive_unsigneds(), |_, &pow| {
                Box::new(
                    exhaustive_pairs_big_small(
                        exhaustive_natural_range(Natural::ZERO, Natural::power_of_two(pow)),
                        exhaustive_unsigneds(),
                    )
                    .map(Option::Some)
                    .chain(repeat(None)),
                )
            })
            .flat_map(|(pow, p)| {
                if let Some(p) = p {
                    Some((pow, p))
                } else {
                    None
                }
            }),
        ),
        GenerationMode::Random(scale) => Box::new(random_dependent_pairs(
            scale,
            u32s_geometric(&scramble(&EXAMPLE_SEED, "pow"), scale).map(u64::from),
            |&scale, &pow| {
                random_pairs(
                    &scramble(&EXAMPLE_SEED, "n and u"),
                    &(|seed| random_range_natural(seed, Natural::ZERO, Natural::low_mask(pow))),
                    &(|seed| u32s_geometric(seed, scale).flat_map(T::checked_from)),
                )
            },
        )),
        GenerationMode::SpecialRandom(scale) => Box::new(random_dependent_pairs(
            scale,
            u32s_geometric(&scramble(&EXAMPLE_SEED, "pow"), scale).map(u64::from),
            |&scale, &pow| {
                random_pairs(
                    &scramble(&EXAMPLE_SEED, "n and u"),
                    &(|seed| {
                        special_random_range_natural(seed, Natural::ZERO, Natural::low_mask(pow))
                    }),
                    &(|seed| u32s_geometric(seed, scale).flat_map(T::checked_from)),
                )
            },
        )),
    };
    reshape_2_1_to_3(permute_2_1(ps))
}

// All triples of `Natural`, `T` and `u64`, where `T` is signed and the `u64` is greater than or
// equal to the number of significant bits of the `Natural`.
pub fn triples_of_natural_small_signed_and_u64_var_1<T: PrimitiveSigned>(
    gm: GenerationMode,
) -> It<(Natural, T, u64)> {
    let ps: It<(u64, (Natural, T))> = match gm {
        GenerationMode::Exhaustive => Box::new(
            exhaustive_dependent_pairs_infinite((), exhaustive_unsigneds(), |_, &pow| {
                Box::new(
                    exhaustive_pairs_big_small(
                        exhaustive_natural_range(Natural::ZERO, Natural::power_of_two(pow)),
                        exhaustive_signeds(),
                    )
                    .map(Option::Some)
                    .chain(repeat(None)),
                )
            })
            .flat_map(|(pow, p)| {
                if let Some(p) = p {
                    Some((pow, p))
                } else {
                    None
                }
            }),
        ),
        GenerationMode::Random(scale) => Box::new(random_dependent_pairs(
            scale,
            u32s_geometric(&scramble(&EXAMPLE_SEED, "pow"), scale).map(u64::from),
            |&scale, &pow| {
                random_pairs(
                    &scramble(&EXAMPLE_SEED, "n and u"),
                    &(|seed| random_range_natural(seed, Natural::ZERO, Natural::low_mask(pow))),
                    &(|seed| i32s_geometric(seed, scale).flat_map(T::checked_from)),
                )
            },
        )),
        GenerationMode::SpecialRandom(scale) => Box::new(random_dependent_pairs(
            scale,
            u32s_geometric(&scramble(&EXAMPLE_SEED, "pow"), scale).map(u64::from),
            |&scale, &pow| {
                random_pairs(
                    &scramble(&EXAMPLE_SEED, "n and u"),
                    &(|seed| {
                        special_random_range_natural(seed, Natural::ZERO, Natural::low_mask(pow))
                    }),
                    &(|seed| i32s_geometric(seed, scale).flat_map(T::checked_from)),
                )
            },
        )),
    };
    reshape_2_1_to_3(permute_2_1(ps))
}

fn triples_of_natural_small_unsigned_and_natural<T: PrimitiveUnsigned + Rand>(
    gm: GenerationMode,
) -> It<(Natural, T, Natural)> {
    match gm {
        GenerationMode::Exhaustive => Box::new(exhaustive_triples(
            exhaustive_naturals(),
            exhaustive_unsigneds(),
            exhaustive_naturals(),
        )),
        GenerationMode::Random(scale) => Box::new(random_triples(
            &EXAMPLE_SEED,
            &(|seed| random_naturals(seed, scale)),
            &(|seed| u32s_geometric(seed, scale).flat_map(T::checked_from)),
            &(|seed| random_naturals(seed, scale)),
        )),
        GenerationMode::SpecialRandom(scale) => Box::new(random_triples(
            &EXAMPLE_SEED,
            &(|seed| special_random_naturals(seed, scale)),
            &(|seed| u32s_geometric(seed, scale).flat_map(T::checked_from)),
            &(|seed| special_random_naturals(seed, scale)),
        )),
    }
}

// All `(Natural, T, Natural)` where `T` is unsigned and the first `Natural` is less than the
// second.
pub fn triples_of_natural_small_unsigned_and_natural_var_1<T: PrimitiveUnsigned + Rand>(
    gm: GenerationMode,
) -> It<(Natural, T, Natural)> {
    Box::new(triples_of_natural_small_unsigned_and_natural(gm).filter(|&(ref x, _, ref z)| x < z))
}

fn triples_of_natural_small_signed_and_natural<T: PrimitiveSigned + Rand>(
    gm: GenerationMode,
) -> It<(Natural, T, Natural)> {
    match gm {
        GenerationMode::Exhaustive => Box::new(exhaustive_triples(
            exhaustive_naturals(),
            exhaustive_signeds(),
            exhaustive_naturals(),
        )),
        GenerationMode::Random(scale) => Box::new(random_triples(
            &EXAMPLE_SEED,
            &(|seed| random_naturals(seed, scale)),
            &(|seed| i32s_geometric(seed, scale).flat_map(T::checked_from)),
            &(|seed| random_naturals(seed, scale)),
        )),
        GenerationMode::SpecialRandom(scale) => Box::new(random_triples(
            &EXAMPLE_SEED,
            &(|seed| special_random_naturals(seed, scale)),
            &(|seed| i32s_geometric(seed, scale).flat_map(T::checked_from)),
            &(|seed| special_random_naturals(seed, scale)),
        )),
    }
}

// All `(Natural, T, Natural)` where `T` is signed and the first `Natural` is less than the second.
pub fn triples_of_natural_small_signed_and_natural_var_1<T: PrimitiveSigned + Rand>(
    gm: GenerationMode,
) -> It<(Natural, T, Natural)> {
    Box::new(triples_of_natural_small_signed_and_natural(gm).filter(|&(ref x, _, ref z)| x < z))
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

        //TODO limit output length
        pub fn $naturals_exactly_equal_to_float(gm: GenerationMode) -> It<Natural> {
            Box::new(naturals(gm).filter(|n| $f::convertible_from(n)))
        }

        //TODO limit output length
        pub fn $floats_exactly_equal_to_natural(gm: GenerationMode) -> It<$f> {
            Box::new(naturals(gm).flat_map($f::checked_from))
        }

        pub fn $naturals_not_exactly_equal_to_float(gm: GenerationMode) -> It<Natural> {
            let n = Natural::from($f::SMALLEST_UNREPRESENTABLE_UINT);
            let xs: It<Natural> = match gm {
                GenerationMode::Exhaustive => Box::new(exhaustive_natural_range_to_infinity(n)),
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
                increment_float(&mut f_above);
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

fn triples_of_natural_natural_and_rounding_mode(
    gm: GenerationMode,
) -> It<(Natural, Natural, RoundingMode)> {
    match gm {
        GenerationMode::Exhaustive => reshape_2_1_to_3(Box::new(lex_pairs(
            exhaustive_pairs_from_single(exhaustive_naturals()),
            exhaustive_rounding_modes(),
        ))),
        GenerationMode::Random(scale) => Box::new(random_triples(
            &EXAMPLE_SEED,
            &(|seed| random_naturals(seed, scale)),
            &(|seed| random_naturals(seed, scale)),
            &(|seed| random_rounding_modes(seed)),
        )),
        GenerationMode::SpecialRandom(scale) => Box::new(random_triples(
            &EXAMPLE_SEED,
            &(|seed| special_random_naturals(seed, scale)),
            &(|seed| special_random_naturals(seed, scale)),
            &(|seed| random_rounding_modes(seed)),
        )),
    }
}

// All triples of `Natural`, positive `Natural`, and `RoundingMode`, where if the `RoundingMode` is
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

// All triples of `Natural`, `Natural`, and `RoundingMode`, where the first `Natural` can be rounded
// to a multiple of the second, according to the rounding mode.
pub fn triples_of_natural_natural_and_rounding_mode_var_2(
    gm: GenerationMode,
) -> It<(Natural, Natural, RoundingMode)> {
    Box::new(
        triples_of_natural_natural_and_rounding_mode(gm).filter_map(|(x, y, rm)| {
            if x == y {
                Some((x, y, rm))
            } else if y == 0 {
                if rm == RoundingMode::Down
                    || rm == RoundingMode::Floor
                    || rm == RoundingMode::Nearest
                {
                    Some((x, y, rm))
                } else {
                    None
                }
            } else if rm == RoundingMode::Exact {
                Some((x * &y, y, rm))
            } else {
                Some((x, y, rm))
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
                exhaustive_fixed_length_vecs_from_single(n.limb_count(), exhaustive_bools())
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

fn random_pairs_of_natural_and_vec_of_bool_var_2(
    seed: &[u32],
    scale: u32,
) -> RandomValueAndVecOfBool<Natural> {
    RandomValueAndVecOfBool {
        xs: Box::new(random_naturals(&scramble(seed, "naturals"), scale)),
        rng: Box::new(IsaacRng::from_seed(&scramble(seed, "bools"))),
    }
}

fn special_random_pairs_of_natural_and_vec_of_bool_var_2(
    seed: &[u32],
    scale: u32,
) -> RandomValueAndVecOfBool<Natural> {
    RandomValueAndVecOfBool {
        xs: Box::new(special_random_naturals(&scramble(seed, "naturals"), scale)),
        rng: Box::new(IsaacRng::from_seed(&scramble(seed, "bools"))),
    }
}

// All pairs of `Natural` and `Vec<bool>` where the length of the `Vec` is equal to the significant
// bit count of the `Natural`.
pub fn pairs_of_natural_and_vec_of_bool_var_2(gm: GenerationMode) -> It<(Natural, Vec<bool>)> {
    match gm {
        GenerationMode::Exhaustive => {
            let f = move |n: &Natural| {
                exhaustive_fixed_length_vecs_from_single(n.significant_bits(), exhaustive_bools())
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

fn quadruples_of_naturals(gm: GenerationMode) -> It<(Natural, Natural, Natural, Natural)> {
    match gm {
        GenerationMode::Exhaustive => {
            Box::new(exhaustive_quadruples_from_single(exhaustive_naturals()))
        }
        GenerationMode::Random(scale) => Box::new(random_quadruples_from_single(random_naturals(
            &EXAMPLE_SEED,
            scale,
        ))),
        GenerationMode::SpecialRandom(scale) => Box::new(random_quadruples_from_single(
            special_random_naturals(&EXAMPLE_SEED, scale),
        )),
    }
}

// All quadruples of `Natural`s where the first three are smaller than the fourth.
pub fn quadruples_of_naturals_var_1(
    gm: GenerationMode,
) -> It<(Natural, Natural, Natural, Natural)> {
    Box::new(
        quadruples_of_naturals(gm).filter(|&(ref x, ref y, ref z, ref m)| x < m && y < m && z < m),
    )
}

// All quadruples of `Natural`s where the first two are smaller than the fourth.
pub fn quadruples_of_naturals_var_2(
    gm: GenerationMode,
) -> It<(Natural, Natural, Natural, Natural)> {
    Box::new(quadruples_of_naturals(gm).filter(|&(ref x, ref y, _, ref m)| x < m && y < m))
}

// All quadruples of `Natural`s where the first is smaller than the fourth.
pub fn quadruples_of_naturals_var_3(
    gm: GenerationMode,
) -> It<(Natural, Natural, Natural, Natural)> {
    Box::new(quadruples_of_naturals(gm).filter(|&(ref x, _, _, ref m)| x < m))
}

// All quadruples of `Natural`, `Natural`, `Natural` and `u64`, where the `u64` is greater than or
// equal to n, where n is the maximum number of significant bits of the `Natural`s.
pub fn quadruples_of_three_naturals_and_u64_var_1(
    gm: GenerationMode,
) -> It<(Natural, Natural, Natural, u64)> {
    let ps: It<(u64, (Natural, Natural, Natural))> = match gm {
        GenerationMode::Exhaustive => Box::new(dependent_pairs(exhaustive_unsigneds(), |&pow| {
            lex_triples_from_single(exhaustive_natural_range(
                Natural::ZERO,
                Natural::power_of_two(pow),
            ))
        })),
        GenerationMode::Random(scale) => Box::new(random_dependent_pairs(
            (),
            u32s_geometric(&scramble(&EXAMPLE_SEED, "pow"), scale).map(u64::from),
            |_, &pow| {
                random_triples_from_single(random_range_natural(
                    &scramble(&EXAMPLE_SEED, "n"),
                    Natural::ZERO,
                    Natural::low_mask(pow),
                ))
            },
        )),
        GenerationMode::SpecialRandom(scale) => Box::new(random_dependent_pairs(
            (),
            u32s_geometric(&scramble(&EXAMPLE_SEED, "pow"), scale).map(u64::from),
            |_, &pow| {
                random_triples_from_single(special_random_range_natural(
                    &scramble(&EXAMPLE_SEED, "n"),
                    Natural::ZERO,
                    Natural::low_mask(pow),
                ))
            },
        )),
    };
    reshape_3_1_to_4(permute_2_1(ps))
}

// All quadruples of `Natural`, `Natural`, `Natural` and `u64`, where the `u64` is greater than or
// equal to n, where n is the maximum number of significant bits of the first two `Natural`s.
pub fn quadruples_of_three_naturals_and_u64_var_2(
    gm: GenerationMode,
) -> It<(Natural, Natural, Natural, u64)> {
    let ps: It<(u64, (Natural, Natural, Natural))> = match gm {
        GenerationMode::Exhaustive => Box::new(exhaustive_dependent_pairs_infinite(
            (),
            exhaustive_unsigneds(),
            |_, &pow| {
                exhaustive_triples_xxy(
                    exhaustive_natural_range(Natural::ZERO, Natural::power_of_two(pow)),
                    exhaustive_naturals(),
                )
            },
        )),
        GenerationMode::Random(scale) => Box::new(random_dependent_pairs(
            scale,
            u32s_geometric(&scramble(&EXAMPLE_SEED, "pow"), scale).map(u64::from),
            |&scale, &pow| {
                random_triples(
                    &scramble(&EXAMPLE_SEED, "n"),
                    &|seed| random_range_natural(seed, Natural::ZERO, Natural::low_mask(pow)),
                    &|seed| random_range_natural(seed, Natural::ZERO, Natural::low_mask(pow)),
                    &|seed| random_naturals(seed, scale),
                )
            },
        )),
        GenerationMode::SpecialRandom(scale) => Box::new(random_dependent_pairs(
            scale,
            u32s_geometric(&scramble(&EXAMPLE_SEED, "pow"), scale).map(u64::from),
            |&scale, &pow| {
                random_triples(
                    &scramble(&EXAMPLE_SEED, "n"),
                    &|seed| {
                        special_random_range_natural(seed, Natural::ZERO, Natural::low_mask(pow))
                    },
                    &|seed| {
                        special_random_range_natural(seed, Natural::ZERO, Natural::low_mask(pow))
                    },
                    &|seed| special_random_naturals(seed, scale),
                )
            },
        )),
    };
    reshape_3_1_to_4(permute_2_1(ps))
}

// All quadruples of `Natural`, `Natural`, `Natural` and `u64`, where the `u64` is greater than or
// equal to the number of significant bits of the first `Natural`.
pub fn quadruples_of_three_naturals_and_u64_var_3(
    gm: GenerationMode,
) -> It<(Natural, Natural, Natural, u64)> {
    let ps: It<(u64, (Natural, Natural, Natural))> = match gm {
        GenerationMode::Exhaustive => Box::new(exhaustive_dependent_pairs_infinite(
            (),
            exhaustive_unsigneds(),
            |_, &pow| {
                exhaustive_triples_xyy(
                    exhaustive_natural_range(Natural::ZERO, Natural::power_of_two(pow)),
                    exhaustive_naturals(),
                )
            },
        )),
        GenerationMode::Random(scale) => Box::new(random_dependent_pairs(
            scale,
            u32s_geometric(&scramble(&EXAMPLE_SEED, "pow"), scale).map(u64::from),
            |&scale, &pow| {
                random_triples(
                    &scramble(&EXAMPLE_SEED, "n"),
                    &|seed| random_range_natural(seed, Natural::ZERO, Natural::low_mask(pow)),
                    &|seed| random_naturals(seed, scale),
                    &|seed| random_naturals(seed, scale),
                )
            },
        )),
        GenerationMode::SpecialRandom(scale) => Box::new(random_dependent_pairs(
            scale,
            u32s_geometric(&scramble(&EXAMPLE_SEED, "pow"), scale).map(u64::from),
            |&scale, &pow| {
                random_triples(
                    &scramble(&EXAMPLE_SEED, "n"),
                    &|seed| {
                        special_random_range_natural(seed, Natural::ZERO, Natural::low_mask(pow))
                    },
                    &|seed| special_random_naturals(seed, scale),
                    &|seed| special_random_naturals(seed, scale),
                )
            },
        )),
    };
    reshape_3_1_to_4(permute_2_1(ps))
}

pub fn quadruples_of_natural_natural_natural_and_small_unsigned<T: PrimitiveUnsigned + Rand>(
    gm: GenerationMode,
) -> It<(Natural, Natural, Natural, T)> {
    match gm {
        GenerationMode::Exhaustive => Box::new(exhaustive_quadruples(
            exhaustive_naturals(),
            exhaustive_naturals(),
            exhaustive_naturals(),
            exhaustive_unsigneds(),
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

fn quadruples_of_natural_small_unsigned_small_unsigned_and_natural<T: PrimitiveUnsigned + Rand>(
    gm: GenerationMode,
) -> It<(Natural, T, T, Natural)> {
    permute_1_3_4_2(reshape_2_2_to_4(match gm {
        GenerationMode::Exhaustive => Box::new(exhaustive_pairs_big_small(
            exhaustive_pairs_from_single(exhaustive_naturals()),
            exhaustive_pairs_from_single(exhaustive_unsigneds()),
        )),
        GenerationMode::Random(scale) => Box::new(random_pairs(
            &EXAMPLE_SEED,
            &(|seed| random_pairs_from_single(random_naturals(seed, scale))),
            &(|seed| {
                random_pairs_from_single(u32s_geometric(seed, scale).flat_map(T::checked_from))
            }),
        )),
        GenerationMode::SpecialRandom(scale) => Box::new(random_pairs(
            &EXAMPLE_SEED,
            &(|seed| random_pairs_from_single(special_random_naturals(seed, scale))),
            &(|seed| {
                random_pairs_from_single(u32s_geometric(seed, scale).flat_map(T::checked_from))
            }),
        )),
    }))
}

pub fn quadruples_of_natural_small_unsigned_small_unsigned_and_natural_var_1<
    T: PrimitiveUnsigned + Rand,
>(
    gm: GenerationMode,
) -> It<(Natural, T, T, Natural)> {
    Box::new(
        quadruples_of_natural_small_unsigned_small_unsigned_and_natural(gm)
            .filter(|&(_, start, end, _)| start < end),
    )
}

fn pairs_of_u64_and_natural_vec_var_1_random_helper(
    &scale: &u32,
    &log_base: &u64,
) -> It<Vec<Natural>> {
    Box::new(random_vecs(
        &EXAMPLE_SEED,
        scale,
        &(|seed| special_random_range_natural(seed, Natural::ZERO, Natural::low_mask(log_base))),
    ))
}

fn pairs_of_u64_and_natural_vec_var_1_special_random_helper(
    &scale: &u32,
    &log_base: &u64,
) -> It<Vec<Natural>> {
    Box::new(random_vecs(
        &EXAMPLE_SEED,
        scale,
        &(|seed| special_random_range_natural(seed, Natural::ZERO, Natural::low_mask(log_base))),
    ))
}

// All pairs of `u64` and `Vec<Natural>`, where each pair is a valid input to
// `from_power_of_two_digits_asc<Natural, Natural>`.
pub fn pairs_of_u64_and_natural_vec_var_1(gm: GenerationMode) -> It<(u64, Vec<Natural>)> {
    match gm {
        GenerationMode::Exhaustive => {
            let f = |_: &(), &log_base: &u64| -> It<Vec<Natural>> {
                let digits =
                    exhaustive_natural_range(Natural::ZERO, Natural::power_of_two(log_base));
                if log_base == 1 {
                    Box::new(shortlex_vecs(digits))
                } else {
                    Box::new(exhaustive_vecs(digits))
                }
            };
            Box::new(exhaustive_dependent_pairs_infinite_sqrt(
                (),
                exhaustive_positive_primitive_ints(),
                f,
            ))
        }
        GenerationMode::Random(scale) => Box::new(random_dependent_pairs(
            scale,
            range_up_geometric_u32(&EXAMPLE_SEED, scale, 1).map(u64::from),
            pairs_of_u64_and_natural_vec_var_1_random_helper,
        )),
        GenerationMode::SpecialRandom(scale) => Box::new(random_dependent_pairs(
            scale,
            range_up_geometric_u32(&EXAMPLE_SEED, scale, 1).map(u64::from),
            pairs_of_u64_and_natural_vec_var_1_special_random_helper,
        )),
    }
}

pub(crate) struct RandomNaturalSmallU64AndVecOfBool {
    pub(crate) ps: It<(Natural, u64)>,
    pub(crate) rng: Box<IsaacRng>,
}

impl Iterator for RandomNaturalSmallU64AndVecOfBool {
    type Item = (Natural, u64, Vec<bool>);

    fn next(&mut self) -> Option<(Natural, u64, Vec<bool>)> {
        let (n, log_base) = self.ps.next().unwrap();
        let mut bools = Vec::new();
        for _ in 0..n
            .significant_bits()
            .div_round(log_base, RoundingMode::Ceiling)
        {
            bools.push(self.rng.gen::<bool>());
        }
        Some((n, log_base, bools))
    }
}

fn random_triples_of_natural_small_u64_and_vec_of_bool_var_1<T: PrimitiveUnsigned>(
    seed: &[u32],
    scale: u32,
) -> RandomNaturalSmallU64AndVecOfBool {
    RandomNaturalSmallU64AndVecOfBool {
        ps: Box::new(random_pairs(
            &EXAMPLE_SEED,
            &(|seed| random_naturals(seed, scale)),
            &(|seed| random_range(seed, 1, T::WIDTH)),
        )),
        rng: Box::new(IsaacRng::from_seed(&scramble(seed, "bools"))),
    }
}

fn special_random_triples_of_natural_small_u64_and_vec_of_bool_var_1<T: PrimitiveUnsigned>(
    seed: &[u32],
    scale: u32,
) -> RandomNaturalSmallU64AndVecOfBool {
    RandomNaturalSmallU64AndVecOfBool {
        ps: Box::new(random_pairs(
            &EXAMPLE_SEED,
            &(|seed| special_random_naturals(seed, scale)),
            &(|seed| random_range(seed, 1, T::WIDTH)),
        )),
        rng: Box::new(IsaacRng::from_seed(&scramble(seed, "bools"))),
    }
}

// All pairs of `Natural`, `u64` and `Vec<bool>`, where `T` is unsigned, the length of the `Vec` is
// equal to the significant base-2<sup>`log_base`</sup>-digit count of the `Natural`, and the `u64`
// is between 1 and `T::WIDTH`, inclusive.
pub fn triples_of_natural_small_u64_and_vec_of_bool_var_1<T: PrimitiveUnsigned>(
    gm: GenerationMode,
) -> It<(Natural, u64, Vec<bool>)> {
    match gm {
        GenerationMode::Exhaustive => {
            let f = move |(n, log_base): &(Natural, u64)| {
                exhaustive_fixed_length_vecs_from_single(
                    n.significant_bits()
                        .div_round(*log_base, RoundingMode::Ceiling),
                    exhaustive_bools(),
                )
            };
            reshape_2_1_to_3(Box::new(dependent_pairs(
                lex_pairs(
                    exhaustive_naturals(),
                    primitive_int_increasing_inclusive_range(1, T::WIDTH),
                ),
                f,
            )))
        }
        GenerationMode::Random(scale) => Box::new(
            random_triples_of_natural_small_u64_and_vec_of_bool_var_1::<T>(&EXAMPLE_SEED, scale),
        ),
        GenerationMode::SpecialRandom(scale) => Box::new(
            special_random_triples_of_natural_small_u64_and_vec_of_bool_var_1::<T>(
                &EXAMPLE_SEED,
                scale,
            ),
        ),
    }
}

fn random_triples_of_natural_small_u64_and_vec_of_bool_var_2(
    seed: &[u32],
    scale: u32,
) -> RandomNaturalSmallU64AndVecOfBool {
    RandomNaturalSmallU64AndVecOfBool {
        ps: Box::new(random_pairs(
            &EXAMPLE_SEED,
            &(|seed| random_naturals(seed, scale)),
            &(|seed| range_up_geometric_u32(seed, scale, 1).map(u64::from)),
        )),
        rng: Box::new(IsaacRng::from_seed(&scramble(seed, "bools"))),
    }
}

fn special_random_triples_of_natural_small_u64_and_vec_of_bool_var_2(
    seed: &[u32],
    scale: u32,
) -> RandomNaturalSmallU64AndVecOfBool {
    RandomNaturalSmallU64AndVecOfBool {
        ps: Box::new(random_pairs(
            &EXAMPLE_SEED,
            &(|seed| special_random_naturals(seed, scale)),
            &(|seed| range_up_geometric_u32(seed, scale, 1).map(u64::from)),
        )),
        rng: Box::new(IsaacRng::from_seed(&scramble(seed, "bools"))),
    }
}

// All pairs of `Natural`, `u64` and `Vec<bool>`, where the length of the `Vec` is equal to the
// significant base-2<sup>`log_base`</sup>-digit count of the `Natural`.
pub fn triples_of_natural_small_u64_and_vec_of_bool_var_2(
    gm: GenerationMode,
) -> It<(Natural, u64, Vec<bool>)> {
    match gm {
        GenerationMode::Exhaustive => {
            let f = move |(n, log_base): &(Natural, u64)| {
                exhaustive_fixed_length_vecs_from_single(
                    n.significant_bits()
                        .div_round(*log_base, RoundingMode::Ceiling),
                    exhaustive_bools(),
                )
            };
            reshape_2_1_to_3(Box::new(dependent_pairs(
                exhaustive_pairs_big_tiny(
                    exhaustive_naturals(),
                    exhaustive_positive_primitive_ints(),
                ),
                f,
            )))
        }
        GenerationMode::Random(scale) => Box::new(
            random_triples_of_natural_small_u64_and_vec_of_bool_var_2(&EXAMPLE_SEED, scale),
        ),
        GenerationMode::SpecialRandom(scale) => Box::new(
            special_random_triples_of_natural_small_u64_and_vec_of_bool_var_2(&EXAMPLE_SEED, scale),
        ),
    }
}
