use common::{GenerationMode, NoSpecialGenerationMode};
use inputs::common::{permute_1_2, permute_1_3_2, reshape_2_1_to_3};
use malachite_base::chars::NUMBER_OF_CHARS;
use malachite_base::limbs::limbs_test_zero;
use malachite_base::num::{
    BitAccess, Parity, PrimitiveInteger, PrimitiveSigned, PrimitiveUnsigned, UnsignedAbs,
};
use malachite_base::round::RoundingMode;
use malachite_nz::integer::logic::bit_access::limbs_vec_clear_bit_neg;
use malachite_nz::natural::arithmetic::mul::{
    _limbs_mul_to_out_toom_32_input_sizes_valid, _limbs_mul_to_out_toom_33_input_sizes_valid,
    _limbs_mul_to_out_toom_42_input_sizes_valid,
};
use malachite_nz::natural::arithmetic::mul_u32::limbs_mul_limb;
use rust_wheels::iterators::bools::exhaustive_bools;
use rust_wheels::iterators::chars::exhaustive_chars;
use rust_wheels::iterators::common::EXAMPLE_SEED;
use rust_wheels::iterators::general::{random, range_increasing};
use rust_wheels::iterators::integers_geometric::{positive_u32s_geometric, u32s_geometric};
use rust_wheels::iterators::orderings::{exhaustive_orderings, random_orderings};
use rust_wheels::iterators::primitive_ints::{
    exhaustive_natural_signed, exhaustive_negative_signed, exhaustive_nonzero_signed,
    exhaustive_positive, exhaustive_signed, exhaustive_unsigned, random_natural_signed,
    random_negative_signed, random_nonzero_signed, random_positive_signed,
    random_positive_unsigned, random_range, random_range_down, range_down_increasing,
    special_random_natural_signed, special_random_negative_signed, special_random_nonzero_signed,
    special_random_positive_signed, special_random_positive_unsigned, special_random_signed,
    special_random_unsigned,
};
use rust_wheels::iterators::rounding_modes::{exhaustive_rounding_modes, random_rounding_modes};
use rust_wheels::iterators::tuples::{
    exhaustive_pairs, exhaustive_pairs_from_single, exhaustive_quadruples, exhaustive_triples,
    exhaustive_triples_from_single, lex_pairs, lex_triples, log_pairs, random_pairs,
    random_pairs_from_single, random_quadruples, random_triples, random_triples_from_single,
    sqrt_pairs,
};
use rust_wheels::iterators::vecs::{
    exhaustive_vecs, exhaustive_vecs_min_length, random_vecs, random_vecs_min_length,
    special_random_bool_vecs, special_random_unsigned_vecs,
    special_random_unsigned_vecs_min_length,
};
use std::char;
use std::cmp::Ordering;
use std::ops::{Shl, Shr};

type It<T> = Box<Iterator<Item = T>>;

pub fn unsigneds<T: PrimitiveUnsigned>(gm: GenerationMode) -> It<T> {
    match gm {
        GenerationMode::Exhaustive => Box::new(exhaustive_unsigned()),
        GenerationMode::Random(_) => Box::new(random(&EXAMPLE_SEED)),
        GenerationMode::SpecialRandom(_) => Box::new(special_random_unsigned(&EXAMPLE_SEED)),
    }
}

pub fn signeds<T: PrimitiveSigned>(gm: GenerationMode) -> It<T> {
    match gm {
        GenerationMode::Exhaustive => Box::new(exhaustive_signed()),
        GenerationMode::Random(_) => Box::new(random(&EXAMPLE_SEED)),
        GenerationMode::SpecialRandom(_) => Box::new(special_random_signed(&EXAMPLE_SEED)),
    }
}

pub fn positive_unsigneds<T: PrimitiveUnsigned>(gm: GenerationMode) -> It<T> {
    match gm {
        GenerationMode::Exhaustive => Box::new(exhaustive_positive()),
        GenerationMode::Random(_) => Box::new(random_positive_unsigned(&EXAMPLE_SEED)),
        GenerationMode::SpecialRandom(_) => {
            Box::new(special_random_positive_unsigned(&EXAMPLE_SEED))
        }
    }
}

pub fn nonzero_signeds<T: PrimitiveSigned>(gm: GenerationMode) -> It<T> {
    match gm {
        GenerationMode::Exhaustive => Box::new(exhaustive_nonzero_signed()),
        GenerationMode::Random(_) => Box::new(random_nonzero_signed(&EXAMPLE_SEED)),
        GenerationMode::SpecialRandom(_) => Box::new(special_random_nonzero_signed(&EXAMPLE_SEED)),
    }
}

pub fn natural_signeds<T: PrimitiveSigned>(gm: GenerationMode) -> It<T> {
    match gm {
        GenerationMode::Exhaustive => Box::new(exhaustive_natural_signed()),
        GenerationMode::Random(_) => Box::new(random_natural_signed(&EXAMPLE_SEED)),
        GenerationMode::SpecialRandom(_) => Box::new(special_random_natural_signed(&EXAMPLE_SEED)),
    }
}

pub fn negative_signeds<T: PrimitiveSigned>(gm: GenerationMode) -> It<T> {
    match gm {
        GenerationMode::Exhaustive => Box::new(exhaustive_negative_signed()),
        GenerationMode::Random(_) => Box::new(random_negative_signed(&EXAMPLE_SEED)),
        GenerationMode::SpecialRandom(_) => Box::new(special_random_negative_signed(&EXAMPLE_SEED)),
    }
}

pub fn unsigneds_no_max<T: PrimitiveUnsigned>(gm: GenerationMode) -> It<T> {
    Box::new(unsigneds(gm).filter(|&u| u != T::MAX))
}

pub fn signeds_no_max<T: PrimitiveSigned>(gm: GenerationMode) -> It<T> {
    Box::new(signeds(gm).filter(|&i| i != T::MAX))
}

pub fn signeds_no_min<T: PrimitiveSigned>(gm: GenerationMode) -> It<T> {
    Box::new(signeds(gm).filter(|&i| i != T::MIN))
}

pub fn pairs_of_unsigneds<T: PrimitiveUnsigned>(
    gm: GenerationMode,
) -> Box<Iterator<Item = (T, T)>> {
    match gm {
        GenerationMode::Exhaustive => Box::new(exhaustive_pairs_from_single(exhaustive_unsigned())),
        GenerationMode::Random(_) => Box::new(random_pairs_from_single(random(&EXAMPLE_SEED))),
        GenerationMode::SpecialRandom(_) => Box::new(random_pairs_from_single(
            special_random_unsigned(&EXAMPLE_SEED),
        )),
    }
}

//TODO use subset_pairs
// All pairs of `T`s where `T` is unsigned and the first `T` is greater than or equal to the second.
pub fn pairs_of_unsigneds_var_1<T: PrimitiveUnsigned>(
    gm: GenerationMode,
) -> Box<Iterator<Item = (T, T)>> {
    Box::new(pairs_of_unsigneds(gm).filter(|&(x, y)| x >= y))
}

pub fn pairs_of_signeds<T: PrimitiveSigned>(gm: GenerationMode) -> Box<Iterator<Item = (T, T)>> {
    match gm {
        GenerationMode::Exhaustive => Box::new(exhaustive_pairs_from_single(exhaustive_signed())),
        GenerationMode::Random(_) => Box::new(random_pairs_from_single(random(&EXAMPLE_SEED))),
        GenerationMode::SpecialRandom(_) => Box::new(random_pairs_from_single(
            special_random_signed(&EXAMPLE_SEED),
        )),
    }
}

pub fn triples_of_signeds<T: PrimitiveSigned>(
    gm: GenerationMode,
) -> Box<Iterator<Item = (T, T, T)>> {
    match gm {
        GenerationMode::Exhaustive => Box::new(exhaustive_triples_from_single(exhaustive_signed())),
        GenerationMode::Random(_) => Box::new(random_triples_from_single(random(&EXAMPLE_SEED))),
        GenerationMode::SpecialRandom(_) => Box::new(random_triples_from_single(
            special_random_signed(&EXAMPLE_SEED),
        )),
    }
}

pub fn pairs_of_signed_and_nonzero_signed<T: PrimitiveSigned>(
    gm: GenerationMode,
) -> Box<Iterator<Item = (T, T)>> {
    match gm {
        GenerationMode::Exhaustive => Box::new(exhaustive_pairs(
            exhaustive_signed(),
            exhaustive_nonzero_signed(),
        )),
        GenerationMode::Random(_) => Box::new(random_pairs(
            &EXAMPLE_SEED,
            &(|seed| random(seed)),
            &(|seed| random_nonzero_signed(seed)),
        )),
        GenerationMode::SpecialRandom(_) => Box::new(random_pairs(
            &EXAMPLE_SEED,
            &(|seed| special_random_signed(seed)),
            &(|seed| special_random_nonzero_signed(seed)),
        )),
    }
}

// Pairs of `i32` and nonzero `i32` where each every `i32` is between -2<sup>15</sup> and
// 2<sup>15</sup> - 1, inclusive, and the first `i32` is divisible by the second.
pub fn pairs_of_i32_and_nonzero_i32_var_1(gm: GenerationMode) -> Box<Iterator<Item = (i32, i32)>> {
    Box::new(
        pairs_of_signed_and_nonzero_signed::<i16>(gm)
            .map(|(x, y)| (i32::from(x) * i32::from(y), i32::from(y))),
    )
}

pub fn triples_of_unsigneds<T: PrimitiveUnsigned>(
    gm: GenerationMode,
) -> Box<Iterator<Item = (T, T, T)>> {
    match gm {
        GenerationMode::Exhaustive => {
            Box::new(exhaustive_triples_from_single(exhaustive_unsigned()))
        }
        GenerationMode::Random(_) => Box::new(random_triples_from_single(random(&EXAMPLE_SEED))),
        GenerationMode::SpecialRandom(_) => Box::new(random_triples_from_single(
            special_random_unsigned(&EXAMPLE_SEED),
        )),
    }
}

// All `u32`s smaller than `NUMBER_OF_CHARS`.
pub fn u32s_range_1(gm: NoSpecialGenerationMode) -> It<u32> {
    match gm {
        NoSpecialGenerationMode::Exhaustive => Box::new(range_down_increasing(NUMBER_OF_CHARS - 1)),
        NoSpecialGenerationMode::Random(_) => {
            Box::new(random_range_down(&EXAMPLE_SEED, NUMBER_OF_CHARS - 1))
        }
    }
}

// All `u32`s smaller than 2<sup>31<sup>.
fn u32s_range_2(gm: GenerationMode) -> It<u32> {
    let upper = 1 << (u32::WIDTH - 1);
    match gm {
        GenerationMode::Exhaustive => Box::new(range_down_increasing(upper)),
        GenerationMode::Random(_) => Box::new(random_range_down(&EXAMPLE_SEED, upper)),
        GenerationMode::SpecialRandom(_) => {
            Box::new(special_random_unsigned::<u32>(&EXAMPLE_SEED).map(|u| {
                let mut u = u;
                u.clear_bit(u64::from(u32::WIDTH - 1));
                u
            }))
        }
    }
}

pub fn odd_u32s(gm: GenerationMode) -> Box<Iterator<Item = u32>> {
    Box::new(u32s_range_2(gm).map(|u| (u << 1) + 1))
}

// All pairs of `u32`s smaller than `NUMBER_OF_CHARS`.
pub fn pairs_of_u32s_range_1(gm: NoSpecialGenerationMode) -> Box<Iterator<Item = (u32, u32)>> {
    match gm {
        NoSpecialGenerationMode::Exhaustive => Box::new(exhaustive_pairs_from_single(
            range_down_increasing(NUMBER_OF_CHARS - 1),
        )),
        NoSpecialGenerationMode::Random(_) => Box::new(random_pairs_from_single(
            random_range_down(&EXAMPLE_SEED, NUMBER_OF_CHARS - 1),
        )),
    }
}

pub fn small_unsigneds<T: PrimitiveUnsigned>(
    gm: NoSpecialGenerationMode,
) -> Box<Iterator<Item = T>> {
    match gm {
        NoSpecialGenerationMode::Exhaustive => Box::new(exhaustive_unsigned()),
        NoSpecialGenerationMode::Random(scale) => {
            Box::new(u32s_geometric(&EXAMPLE_SEED, scale).flat_map(T::checked_from))
        }
    }
}

pub fn small_positive_unsigneds<T: PrimitiveUnsigned>(
    gm: NoSpecialGenerationMode,
) -> Box<Iterator<Item = T>> {
    match gm {
        NoSpecialGenerationMode::Exhaustive => Box::new(exhaustive_positive()),
        NoSpecialGenerationMode::Random(scale) => {
            Box::new(positive_u32s_geometric(&EXAMPLE_SEED, scale).flat_map(T::checked_from))
        }
    }
}

pub fn small_usizes(gm: NoSpecialGenerationMode) -> Box<Iterator<Item = usize>> {
    Box::new(small_unsigneds::<u64>(gm).map(|u| u as usize))
}

fn sqrt_pairs_of_unsigneds<T: PrimitiveUnsigned, U: PrimitiveUnsigned>(
) -> Box<Iterator<Item = (T, U)>> {
    Box::new(sqrt_pairs(exhaustive_unsigned(), exhaustive_unsigned()))
}

fn random_pairs_of_primitive_and_geometric_unsigned<T: PrimitiveInteger, U: PrimitiveInteger>(
    scale: u32,
) -> Box<Iterator<Item = (T, U)>> {
    Box::new(random_pairs(
        &EXAMPLE_SEED,
        &(|seed| random(seed)),
        &(|seed| u32s_geometric(seed, scale).flat_map(U::checked_from)),
    ))
}

pub fn pairs_of_unsigned_and_small_unsigned<T: PrimitiveUnsigned, U: PrimitiveUnsigned>(
    gm: GenerationMode,
) -> Box<Iterator<Item = (T, U)>> {
    match gm {
        GenerationMode::Exhaustive => sqrt_pairs_of_unsigneds(),
        GenerationMode::Random(scale) => random_pairs_of_primitive_and_geometric_unsigned(scale),
        GenerationMode::SpecialRandom(scale) => Box::new(random_pairs(
            &EXAMPLE_SEED,
            &(|seed| special_random_unsigned(seed)),
            &(|seed| u32s_geometric(seed, scale).flat_map(U::checked_from)),
        )),
    }
}

pub fn pairs_of_small_usize_and_unsigned<T: PrimitiveUnsigned>(
    gm: GenerationMode,
) -> Box<Iterator<Item = (usize, T)>> {
    match gm {
        GenerationMode::Exhaustive => permute_1_2(Box::new(log_pairs(
            exhaustive_unsigned(),
            exhaustive_unsigned::<u32>().map(|u| u as usize),
        ))),
        GenerationMode::Random(scale) => Box::new(random_pairs(
            &EXAMPLE_SEED,
            &(|seed| u32s_geometric(seed, scale).map(|u| u as usize)),
            &(|seed| random(seed)),
        )),
        GenerationMode::SpecialRandom(scale) => Box::new(random_pairs(
            &EXAMPLE_SEED,
            &(|seed| u32s_geometric(seed, scale).map(|u| u as usize)),
            &(|seed| special_random_unsigned(seed)),
        )),
    }
}

fn log_pairs_of_positive_primitive_and_unsigned<T: PrimitiveInteger, U: PrimitiveUnsigned>(
) -> Box<Iterator<Item = (T, U)>> {
    Box::new(log_pairs(exhaustive_positive(), exhaustive_unsigned()))
}

pub fn pairs_of_positive_unsigned_and_small_unsigned<T: PrimitiveUnsigned, U: PrimitiveUnsigned>(
    gm: GenerationMode,
) -> Box<Iterator<Item = (T, U)>> {
    match gm {
        GenerationMode::Exhaustive => log_pairs_of_positive_primitive_and_unsigned(),
        GenerationMode::Random(scale) => Box::new(random_pairs(
            &EXAMPLE_SEED,
            &(|seed| random_positive_unsigned(seed)),
            &(|seed| u32s_geometric(seed, scale).flat_map(U::checked_from)),
        )),
        GenerationMode::SpecialRandom(scale) => Box::new(random_pairs(
            &EXAMPLE_SEED,
            &(|seed| special_random_positive_unsigned(seed)),
            &(|seed| u32s_geometric(seed, scale).flat_map(U::checked_from)),
        )),
    }
}

pub fn pairs_of_positive_signed_and_small_unsigned<T: PrimitiveSigned, U: PrimitiveUnsigned>(
    gm: GenerationMode,
) -> Box<Iterator<Item = (T, U)>> {
    match gm {
        GenerationMode::Exhaustive => log_pairs_of_positive_primitive_and_unsigned(),
        GenerationMode::Random(scale) => Box::new(random_pairs(
            &EXAMPLE_SEED,
            &(|seed| random_positive_signed(seed)),
            &(|seed| u32s_geometric(seed, scale).flat_map(U::checked_from)),
        )),
        GenerationMode::SpecialRandom(scale) => Box::new(random_pairs(
            &EXAMPLE_SEED,
            &(|seed| special_random_positive_signed(seed)),
            &(|seed| u32s_geometric(seed, scale).flat_map(U::checked_from)),
        )),
    }
}

fn sqrt_pairs_of_signed_and_unsigned<T: PrimitiveSigned, U: PrimitiveUnsigned>(
) -> Box<Iterator<Item = (T, U)>> {
    Box::new(sqrt_pairs(exhaustive_signed(), exhaustive_unsigned()))
}

pub fn pairs_of_signed_and_small_unsigned<T: PrimitiveSigned, U: PrimitiveUnsigned>(
    gm: GenerationMode,
) -> It<(T, U)> {
    match gm {
        GenerationMode::Exhaustive => sqrt_pairs_of_signed_and_unsigned(),
        GenerationMode::Random(scale) => random_pairs_of_primitive_and_geometric_unsigned(scale),
        GenerationMode::SpecialRandom(scale) => Box::new(random_pairs(
            &EXAMPLE_SEED,
            &(|seed| special_random_signed(seed)),
            &(|seed| u32s_geometric(seed, scale).flat_map(U::checked_from)),
        )),
    }
}

type ItU<T> = It<(T, u64)>;
fn exhaustive_pairs_of_unsigned_and_u64_width_range<T: PrimitiveUnsigned>() -> ItU<T> {
    Box::new(lex_pairs(
        exhaustive_unsigned(),
        range_down_increasing(u64::from(T::WIDTH) - 1),
    ))
}

fn random_pairs_of_primitive_and_u64_width_range<T: PrimitiveInteger>() -> It<(T, u64)> {
    Box::new(random_pairs(
        &EXAMPLE_SEED,
        &(|seed| random(seed)),
        &(|seed| random_range_down(seed, u64::from(T::WIDTH) - 1)),
    ))
}

// All pairs of unsigned `T` and `u64`, where the `u64` is smaller that `T::WIDTH`.
pub fn pairs_of_unsigned_and_u64_width_range<T: PrimitiveUnsigned>(
    gm: GenerationMode,
) -> It<(T, u64)> {
    match gm {
        GenerationMode::Exhaustive => exhaustive_pairs_of_unsigned_and_u64_width_range(),
        GenerationMode::Random(_) => random_pairs_of_primitive_and_u64_width_range(),
        GenerationMode::SpecialRandom(_) => Box::new(random_pairs(
            &EXAMPLE_SEED,
            &(|seed| special_random_unsigned(seed)),
            &(|seed| random_range_down(seed, u64::from(T::WIDTH) - 1)),
        )),
    }
}

// All pairs of signed `T` and `u64`, where the `u64` is smaller that `T::WIDTH`.
pub fn pairs_of_signed_and_u64_width_range<T: PrimitiveSigned>(gm: GenerationMode) -> It<(T, u64)> {
    match gm {
        GenerationMode::Exhaustive => Box::new(lex_pairs(
            exhaustive_signed(),
            range_down_increasing(u64::from(T::WIDTH) - 1),
        )),
        GenerationMode::Random(_) => random_pairs_of_primitive_and_u64_width_range(),
        GenerationMode::SpecialRandom(_) => Box::new(random_pairs(
            &EXAMPLE_SEED,
            &(|seed| special_random_signed(seed)),
            &(|seed| random_range_down(seed, u64::from(T::WIDTH) - 1)),
        )),
    }
}

// All pairs of signed `T` and `u64`, where the signed `T` i s negative or the `u64` is smaller than
// `T::WIDTH`.
pub fn pairs_of_signed_and_u64_width_range_var_1<T: PrimitiveSigned>(
    gm: GenerationMode,
) -> It<(T, u64)> {
    Box::new(
        pairs_of_signed_and_small_unsigned(gm)
            .filter(|&(n, index)| n < T::ZERO || index < u64::from(T::WIDTH)),
    )
}

// All pairs of signed `T` and `u64`, where the signed `T` i s non-negative or the `u64` is smaller
// than `T::WIDTH`.
pub fn pairs_of_signed_and_u64_width_range_var_2<T: PrimitiveSigned>(
    gm: GenerationMode,
) -> It<(T, u64)> {
    Box::new(
        pairs_of_signed_and_small_unsigned(gm)
            .filter(|&(n, index)| n >= T::ZERO || index < u64::from(T::WIDTH)),
    )
}

pub fn pairs_of_unsigned_and_positive_unsigned<T: PrimitiveUnsigned>(
    gm: GenerationMode,
) -> Box<Iterator<Item = (T, T)>> {
    match gm {
        GenerationMode::Exhaustive => Box::new(exhaustive_pairs(
            exhaustive_unsigned(),
            exhaustive_positive(),
        )),
        GenerationMode::Random(_) => Box::new(random_pairs(
            &EXAMPLE_SEED,
            &(|seed| random(seed)),
            &(|seed| random_positive_unsigned(seed)),
        )),
        GenerationMode::SpecialRandom(_) => Box::new(random_pairs(
            &EXAMPLE_SEED,
            &(|seed| special_random_unsigned(seed)),
            &(|seed| special_random_positive_unsigned(seed)),
        )),
    }
}

// Pairs of `u32` and positive `u32` where each every `u32` is less than 2<sup>16</sup> and the
// first `u32` is divisible by the second.
pub fn pairs_of_u32_and_positive_u32_var_1(gm: GenerationMode) -> Box<Iterator<Item = (u32, u32)>> {
    Box::new(
        pairs_of_unsigned_and_positive_unsigned::<u16>(gm)
            .map(|(x, y)| (u32::from(x) * u32::from(y), u32::from(y))),
    )
}

pub fn pairs_of_signed_and_positive_unsigned<T: PrimitiveSigned, U: PrimitiveUnsigned>(
    gm: GenerationMode,
) -> Box<Iterator<Item = (T, U)>> {
    match gm {
        GenerationMode::Exhaustive => {
            Box::new(exhaustive_pairs(exhaustive_signed(), exhaustive_positive()))
        }
        GenerationMode::Random(_) => Box::new(random_pairs(
            &EXAMPLE_SEED,
            &(|seed| random(seed)),
            &(|seed| random_positive_unsigned(seed)),
        )),
        GenerationMode::SpecialRandom(_) => Box::new(random_pairs(
            &EXAMPLE_SEED,
            &(|seed| special_random_signed(seed)),
            &(|seed| special_random_positive_unsigned(seed)),
        )),
    }
}

// Pairs of `i32` and positive `u32` where the `i32` is between -2<sup>15</sup> and
// 2<sup>15</sup> - 1, inclusive, the `u32` is less than 2<sup>16</sup>, and the `i32` is divisible
// by the `u32`.
pub fn pairs_of_i32_and_positive_u32_var_1(gm: GenerationMode) -> Box<Iterator<Item = (i32, u32)>> {
    Box::new(
        pairs_of_signed_and_positive_unsigned::<i16, u16>(gm)
            .map(|(x, y)| (i32::from(x) * i32::from(y), u32::from(y))),
    )
}

pub fn pairs_of_unsigned_and_nonzero_signed<T: PrimitiveUnsigned, U: PrimitiveSigned>(
    gm: GenerationMode,
) -> Box<Iterator<Item = (T, U)>> {
    match gm {
        GenerationMode::Exhaustive => Box::new(exhaustive_pairs(
            exhaustive_unsigned(),
            exhaustive_nonzero_signed(),
        )),
        GenerationMode::Random(_) => Box::new(random_pairs(
            &EXAMPLE_SEED,
            &(|seed| random(seed)),
            &(|seed| random_nonzero_signed(seed)),
        )),
        GenerationMode::SpecialRandom(_) => Box::new(random_pairs(
            &EXAMPLE_SEED,
            &(|seed| special_random_unsigned(seed)),
            &(|seed| special_random_nonzero_signed(seed)),
        )),
    }
}

// Pairs of `u32` and nonzero `u32` where the `u32` is less than 2<sup>16</sup>, the `i32` is
// between -2<sup>15</sup> and 2<sup>15</sup> - 1, inclusive, and the `u32` is divisible by the
// `i32`.
pub fn pairs_of_u32_and_nonzero_i32_var_1(gm: GenerationMode) -> Box<Iterator<Item = (u32, i32)>> {
    Box::new(
        pairs_of_unsigned_and_nonzero_signed::<u16, i16>(gm)
            .map(|(x, y)| ((i32::from(x) * i32::from(y)).unsigned_abs(), i32::from(y))),
    )
}

// All triples of `T`, `U`, and `bool`, where `T` and `U` are unsigned and the `bool` is false or
// the `U` is smaller than `T::WIDTH`.
pub fn triples_of_unsigned_unsigned_width_range_and_bool_var_1<
    T: PrimitiveUnsigned,
    U: PrimitiveUnsigned,
>(
    gm: GenerationMode,
) -> It<(T, U, bool)> {
    let unfiltered: It<(T, U, bool)> = match gm {
        GenerationMode::Exhaustive => reshape_2_1_to_3(Box::new(lex_pairs(
            sqrt_pairs_of_unsigneds(),
            exhaustive_bools(),
        ))),
        GenerationMode::Random(scale) => Box::new(random_triples(
            &EXAMPLE_SEED,
            &(|seed| random(seed)),
            &(|seed| u32s_geometric(seed, scale).flat_map(U::checked_from)),
            &(|seed| random(seed)),
        )),
        GenerationMode::SpecialRandom(scale) => Box::new(random_triples(
            &EXAMPLE_SEED,
            &(|seed| special_random_unsigned(seed)),
            &(|seed| u32s_geometric(seed, scale).flat_map(U::checked_from)),
            &(|seed| random(seed)),
        )),
    };
    Box::new(
        unfiltered.filter(|&(_, index, bit)| !bit || index < U::checked_from(T::WIDTH).unwrap()),
    )
}

// All triples of signed `T`, `U`, and `bool`, where `T` is signed, `U` is unsigned, and `U` is
// smaller than `T::WIDTH` or the `bool` is equal to whether the `T` is negative.
pub fn triples_of_signed_unsigned_width_range_and_bool_var_1<
    T: PrimitiveSigned,
    U: PrimitiveUnsigned,
>(
    gm: GenerationMode,
) -> It<(T, U, bool)> {
    let unfiltered: It<(T, U, bool)> = match gm {
        GenerationMode::Exhaustive => reshape_2_1_to_3(Box::new(lex_pairs(
            sqrt_pairs_of_signed_and_unsigned(),
            exhaustive_bools(),
        ))),
        GenerationMode::Random(scale) => Box::new(random_triples(
            &EXAMPLE_SEED,
            &(|seed| random(seed)),
            &(|seed| u32s_geometric(seed, scale).flat_map(U::checked_from)),
            &(|seed| random(seed)),
        )),
        GenerationMode::SpecialRandom(scale) => Box::new(random_triples(
            &EXAMPLE_SEED,
            &(|seed| special_random_signed(seed)),
            &(|seed| u32s_geometric(seed, scale).flat_map(U::checked_from)),
            &(|seed| random(seed)),
        )),
    };
    Box::new(unfiltered.filter(|&(n, index, bit)| {
        index < U::checked_from(T::WIDTH).unwrap() || bit == (n < T::ZERO)
    }))
}

pub fn pairs_of_negative_signed_not_min_and_small_unsigned<
    T: PrimitiveSigned,
    U: PrimitiveUnsigned,
>(
    gm: GenerationMode,
) -> Box<Iterator<Item = (T, U)>> {
    match gm {
        GenerationMode::Exhaustive => Box::new(log_pairs(
            exhaustive_negative_signed().filter(|&i| i != T::MIN),
            exhaustive_unsigned(),
        )),
        GenerationMode::Random(scale) => Box::new(random_pairs(
            &EXAMPLE_SEED,
            &(|seed| random_negative_signed(seed).filter(|&i| i != T::MIN)),
            &(|seed| u32s_geometric(seed, scale).flat_map(U::checked_from)),
        )),
        GenerationMode::SpecialRandom(scale) => Box::new(random_pairs(
            &EXAMPLE_SEED,
            &(|seed| special_random_negative_signed(seed).filter(|&i| i != T::MIN)),
            &(|seed| u32s_geometric(seed, scale).flat_map(U::checked_from)),
        )),
    }
}

pub fn triples_of_signed_signed_and_small_unsigned<T: PrimitiveSigned, U: PrimitiveUnsigned>(
    gm: GenerationMode,
) -> It<(T, T, U)> {
    match gm {
        GenerationMode::Exhaustive => Box::new(reshape_2_1_to_3(Box::new(sqrt_pairs(
            exhaustive_pairs_from_single(exhaustive_signed()),
            exhaustive_unsigned(),
        )))),
        GenerationMode::Random(scale) => Box::new(random_triples(
            &EXAMPLE_SEED,
            &(|seed| random(seed)),
            &(|seed| random(seed)),
            &(|seed| u32s_geometric(seed, scale).flat_map(U::checked_from)),
        )),
        GenerationMode::SpecialRandom(scale) => Box::new(random_triples(
            &EXAMPLE_SEED,
            &(|seed| special_random_signed(seed)),
            &(|seed| special_random_signed(seed)),
            &(|seed| u32s_geometric(seed, scale).flat_map(U::checked_from)),
        )),
    }
}

pub fn chars(gm: NoSpecialGenerationMode) -> Box<Iterator<Item = char>> {
    match gm {
        NoSpecialGenerationMode::Exhaustive => Box::new(exhaustive_chars()),
        NoSpecialGenerationMode::Random(_) => Box::new(random(&EXAMPLE_SEED)),
    }
}

pub fn chars_not_min(gm: NoSpecialGenerationMode) -> Box<Iterator<Item = char>> {
    Box::new(chars(gm).filter(|&c| c != '\u{0}'))
}

pub fn chars_not_max(gm: NoSpecialGenerationMode) -> Box<Iterator<Item = char>> {
    Box::new(chars(gm).filter(|&c| c != char::MAX))
}

pub fn pairs_of_chars(gm: NoSpecialGenerationMode) -> Box<Iterator<Item = (char, char)>> {
    match gm {
        NoSpecialGenerationMode::Exhaustive => {
            Box::new(exhaustive_pairs_from_single(exhaustive_chars()))
        }
        NoSpecialGenerationMode::Random(_) => {
            Box::new(random_pairs_from_single(random(&EXAMPLE_SEED)))
        }
    }
}

pub fn rounding_modes(gm: NoSpecialGenerationMode) -> Box<Iterator<Item = RoundingMode>> {
    match gm {
        NoSpecialGenerationMode::Exhaustive => Box::new(exhaustive_rounding_modes()),
        NoSpecialGenerationMode::Random(_) => Box::new(random_rounding_modes(&EXAMPLE_SEED)),
    }
}

pub fn pairs_of_rounding_modes(
    gm: NoSpecialGenerationMode,
) -> Box<Iterator<Item = (RoundingMode, RoundingMode)>> {
    match gm {
        NoSpecialGenerationMode::Exhaustive => Box::new(lex_pairs(
            exhaustive_rounding_modes(),
            exhaustive_rounding_modes(),
        )),
        NoSpecialGenerationMode::Random(_) => Box::new(random_pairs_from_single(
            random_rounding_modes(&EXAMPLE_SEED),
        )),
    }
}

pub fn triples_of_rounding_modes(
    gm: NoSpecialGenerationMode,
) -> Box<Iterator<Item = (RoundingMode, RoundingMode, RoundingMode)>> {
    match gm {
        NoSpecialGenerationMode::Exhaustive => Box::new(lex_triples(
            exhaustive_rounding_modes(),
            exhaustive_rounding_modes(),
            exhaustive_rounding_modes(),
        )),
        NoSpecialGenerationMode::Random(_) => Box::new(random_triples_from_single(
            random_rounding_modes(&EXAMPLE_SEED),
        )),
    }
}

fn random_pairs_of_primitive_and_rounding_mode<T: PrimitiveInteger>(
) -> Box<Iterator<Item = (T, RoundingMode)>> {
    Box::new(random_pairs(
        &EXAMPLE_SEED,
        &(|seed| random(seed)),
        &(|seed| random_rounding_modes(seed)),
    ))
}

pub fn pairs_of_unsigned_and_rounding_mode<T: PrimitiveUnsigned>(
    gm: GenerationMode,
) -> Box<Iterator<Item = (T, RoundingMode)>> {
    match gm {
        GenerationMode::Exhaustive => Box::new(lex_pairs(
            exhaustive_unsigned(),
            exhaustive_rounding_modes(),
        )),
        GenerationMode::Random(_) => random_pairs_of_primitive_and_rounding_mode(),
        GenerationMode::SpecialRandom(_) => Box::new(random_pairs(
            &EXAMPLE_SEED,
            &(|seed| special_random_unsigned(seed)),
            &(|seed| random_rounding_modes(seed)),
        )),
    }
}

pub fn pairs_of_positive_unsigned_and_rounding_mode<T: PrimitiveUnsigned>(
    gm: GenerationMode,
) -> Box<Iterator<Item = (T, RoundingMode)>> {
    match gm {
        GenerationMode::Exhaustive => Box::new(lex_pairs(
            exhaustive_positive(),
            exhaustive_rounding_modes(),
        )),
        GenerationMode::Random(_) => Box::new(random_pairs(
            &EXAMPLE_SEED,
            &(|seed| random_positive_unsigned(seed)),
            &(|seed| random_rounding_modes(seed)),
        )),
        GenerationMode::SpecialRandom(_) => Box::new(random_pairs(
            &EXAMPLE_SEED,
            &(|seed| special_random_positive_unsigned(seed)),
            &(|seed| random_rounding_modes(seed)),
        )),
    }
}

pub fn pairs_of_signed_and_rounding_mode<T: PrimitiveSigned>(
    gm: GenerationMode,
) -> Box<Iterator<Item = (T, RoundingMode)>> {
    match gm {
        GenerationMode::Exhaustive => {
            Box::new(lex_pairs(exhaustive_signed(), exhaustive_rounding_modes()))
        }
        GenerationMode::Random(_) => random_pairs_of_primitive_and_rounding_mode(),
        GenerationMode::SpecialRandom(_) => Box::new(random_pairs(
            &EXAMPLE_SEED,
            &(|seed| special_random_signed(seed)),
            &(|seed| random_rounding_modes(seed)),
        )),
    }
}

pub fn pairs_of_nonzero_signed_and_rounding_mode<T: PrimitiveSigned>(
    gm: GenerationMode,
) -> Box<Iterator<Item = (T, RoundingMode)>> {
    match gm {
        GenerationMode::Exhaustive => Box::new(lex_pairs(
            exhaustive_nonzero_signed(),
            exhaustive_rounding_modes(),
        )),
        GenerationMode::Random(_) => Box::new(random_pairs(
            &EXAMPLE_SEED,
            &(|seed| random_nonzero_signed(seed)),
            &(|seed| random_rounding_modes(seed)),
        )),
        GenerationMode::SpecialRandom(_) => Box::new(random_pairs(
            &EXAMPLE_SEED,
            &(|seed| special_random_nonzero_signed(seed)),
            &(|seed| random_rounding_modes(seed)),
        )),
    }
}

fn triples_of_unsigned_positive_unsigned_and_rounding_mode<T: PrimitiveUnsigned>(
    gm: GenerationMode,
) -> Box<Iterator<Item = (T, T, RoundingMode)>> {
    match gm {
        GenerationMode::Exhaustive => reshape_2_1_to_3(Box::new(lex_pairs(
            exhaustive_pairs(exhaustive_unsigned(), exhaustive_positive()),
            exhaustive_rounding_modes(),
        ))),
        GenerationMode::Random(_) => Box::new(random_triples(
            &EXAMPLE_SEED,
            &(|seed| random(seed)),
            &(|seed| random_positive_unsigned(seed)),
            &(|seed| random_rounding_modes(seed)),
        )),
        GenerationMode::SpecialRandom(_) => Box::new(random_triples(
            &EXAMPLE_SEED,
            &(|seed| special_random_unsigned(seed)),
            &(|seed| special_random_positive_unsigned(seed)),
            &(|seed| random_rounding_modes(seed)),
        )),
    }
}

// All triples of `u32`, positive `u32`, and `RoundingMode`, where both `u32`s are less than
// 2<sup>16</sup> and if the `RoundingMode` is `RoundingMode::Exact`, the first `u32` is divisible
// by the second.
pub fn triples_of_u32_positive_u32_and_rounding_mode_var_1(
    gm: GenerationMode,
) -> Box<Iterator<Item = (u32, u32, RoundingMode)>> {
    Box::new(
        triples_of_unsigned_positive_unsigned_and_rounding_mode::<u16>(gm).map(|(x, y, rm)| {
            let x = u32::from(x);
            let y = u32::from(y);
            if rm == RoundingMode::Exact {
                (x * y, y, rm)
            } else {
                (x, y, rm)
            }
        }),
    )
}

fn triples_of_signed_nonzero_signed_and_rounding_mode<T: PrimitiveSigned>(
    gm: GenerationMode,
) -> Box<Iterator<Item = (T, T, RoundingMode)>> {
    match gm {
        GenerationMode::Exhaustive => reshape_2_1_to_3(Box::new(lex_pairs(
            exhaustive_pairs(exhaustive_signed(), exhaustive_nonzero_signed()),
            exhaustive_rounding_modes(),
        ))),
        GenerationMode::Random(_) => Box::new(random_triples(
            &EXAMPLE_SEED,
            &(|seed| random(seed)),
            &(|seed| random_nonzero_signed(seed)),
            &(|seed| random_rounding_modes(seed)),
        )),
        GenerationMode::SpecialRandom(_) => Box::new(random_triples(
            &EXAMPLE_SEED,
            &(|seed| special_random_signed(seed)),
            &(|seed| special_random_nonzero_signed(seed)),
            &(|seed| random_rounding_modes(seed)),
        )),
    }
}

// All triples of `i32`, nonzero `i32`, and `RoundingMode`, where both `i32`s are between
// -2<sup>15</sup> and 2<sup>15</sup> - 1, inclusive, and if the `RoundingMode` is
// `RoundingMode::Exact`, the first `i32` is divisible by the second.
pub fn triples_of_i32_nonzero_i32_and_rounding_mode_var_1(
    gm: GenerationMode,
) -> Box<Iterator<Item = (i32, i32, RoundingMode)>> {
    Box::new(
        triples_of_signed_nonzero_signed_and_rounding_mode::<i16>(gm).map(|(x, y, rm)| {
            let x = i32::from(x);
            let y = i32::from(y);
            if rm == RoundingMode::Exact {
                (x * y, y, rm)
            } else {
                (x, y, rm)
            }
        }),
    )
}

pub fn triples_of_unsigned_unsigned_and_small_unsigned<
    T: PrimitiveUnsigned,
    U: PrimitiveUnsigned,
>(
    gm: GenerationMode,
) -> Box<Iterator<Item = (T, T, U)>> {
    match gm {
        GenerationMode::Exhaustive => reshape_2_1_to_3(Box::new(log_pairs(
            exhaustive_pairs_from_single(exhaustive_unsigned()),
            exhaustive_unsigned(),
        ))),
        GenerationMode::Random(scale) => Box::new(random_triples(
            &EXAMPLE_SEED,
            &(|seed| random(seed)),
            &(|seed| random(seed)),
            &(|seed| u32s_geometric(seed, scale).flat_map(U::checked_from)),
        )),
        GenerationMode::SpecialRandom(scale) => Box::new(random_triples(
            &EXAMPLE_SEED,
            &(|seed| special_random_unsigned(seed)),
            &(|seed| special_random_unsigned(seed)),
            &(|seed| u32s_geometric(seed, scale).flat_map(U::checked_from)),
        )),
    }
}

pub fn vecs_of_unsigned<T: PrimitiveUnsigned>(gm: GenerationMode) -> Box<Iterator<Item = Vec<T>>> {
    match gm {
        GenerationMode::Exhaustive => Box::new(exhaustive_vecs(exhaustive_unsigned())),
        GenerationMode::Random(scale) => {
            Box::new(random_vecs(&EXAMPLE_SEED, scale, &(|seed| random(seed))))
        }
        GenerationMode::SpecialRandom(scale) => {
            Box::new(special_random_unsigned_vecs(&EXAMPLE_SEED, scale))
        }
    }
}

//TODO use vecs_at_least
pub fn nonempty_vecs_of_unsigned<T: PrimitiveUnsigned>(
    gm: GenerationMode,
) -> Box<Iterator<Item = Vec<T>>> {
    Box::new(vecs_of_unsigned(gm).filter(|xs| !xs.is_empty()))
}

// All `Vec<T>`, where `T` is unsigned, the `Vec` is nonempty, and its last `T` is nonzero.
pub fn vecs_of_unsigned_var_1<T: PrimitiveUnsigned>(
    gm: GenerationMode,
) -> Box<Iterator<Item = Vec<T>>> {
    Box::new(
        vecs_of_unsigned(gm).filter(|limbs| !limbs.is_empty() && *limbs.last().unwrap() != T::ZERO),
    )
}

// All `Vec<T>`, where `T` is unsigned and either the `Vec` is empty, or its last `T` is nonzero.
pub fn vecs_of_unsigned_var_2<T: PrimitiveUnsigned>(
    gm: GenerationMode,
) -> Box<Iterator<Item = Vec<T>>> {
    Box::new(
        vecs_of_unsigned(gm).filter(|limbs| limbs.is_empty() || *limbs.last().unwrap() != T::ZERO),
    )
}

// All `Vec<T>`, where `T` is unsigned and the `Vec` is nonempty and doesn't only contain zeros.
pub fn vecs_of_unsigned_var_3<T: PrimitiveUnsigned>(
    gm: GenerationMode,
) -> Box<Iterator<Item = Vec<T>>> {
    Box::new(vecs_of_unsigned(gm).filter(|limbs| !limbs_test_zero(limbs)))
}

// All `Vec<T>`, where `T` is unsigned and the `Vec` has length at least 2 and the last element is
// not zero.
pub fn vecs_of_unsigned_var_4<T: PrimitiveUnsigned>(
    gm: GenerationMode,
) -> Box<Iterator<Item = Vec<T>>> {
    Box::new(
        vecs_of_unsigned(gm).filter(|limbs| limbs.len() > 1 && *limbs.last().unwrap() != T::ZERO),
    )
}

// All `Vec<T>` that are nonempty and represent a `Natural` divisible by 3.
pub fn vecs_of_unsigned_var_5(gm: GenerationMode) -> Box<Iterator<Item = Vec<u32>>> {
    Box::new(
        vecs_of_unsigned(gm)
            .filter(|ref limbs| limbs.len() > 0)
            .map(|limbs| limbs_mul_limb(&limbs, 3)),
    )
}

pub fn pairs_of_unsigned_vec<T: PrimitiveUnsigned>(
    gm: GenerationMode,
) -> Box<Iterator<Item = (Vec<T>, Vec<T>)>> {
    match gm {
        GenerationMode::Exhaustive => Box::new(exhaustive_pairs_from_single(exhaustive_vecs(
            exhaustive_unsigned(),
        ))),
        GenerationMode::Random(scale) => Box::new(random_pairs_from_single(random_vecs(
            &EXAMPLE_SEED,
            scale,
            &(|seed| random(seed)),
        ))),
        GenerationMode::SpecialRandom(scale) => Box::new(random_pairs_from_single(
            special_random_unsigned_vecs(&EXAMPLE_SEED, scale),
        )),
    }
}

// All pairs of `Vec<T>` where `T` is unsigned and the two components of the pair have the same
// length.
pub fn pairs_of_unsigned_vec_var_1<T: PrimitiveUnsigned>(
    gm: GenerationMode,
) -> Box<Iterator<Item = (Vec<T>, Vec<T>)>> {
    Box::new(vecs_of_unsigned(gm).filter(|xs| xs.len().even()).map(|xs| {
        let half_length = xs.len() >> 1;
        (xs[..half_length].to_vec(), xs[half_length..].to_vec())
    }))
}

// All pairs of `Vec<T>`, where `T` is unsigned and the last `T`s of both `Vec`s are nonzero.
pub fn pairs_of_unsigned_vec_var_2<T: PrimitiveUnsigned>(
    gm: GenerationMode,
) -> Box<Iterator<Item = (Vec<T>, Vec<T>)>> {
    match gm {
        GenerationMode::Exhaustive => Box::new(exhaustive_pairs_from_single(
            exhaustive_vecs(exhaustive_unsigned())
                .filter(|limbs| limbs.is_empty() || *limbs.last().unwrap() != T::ZERO),
        )),
        _ => Box::new(random_pairs_from_single(vecs_of_unsigned_var_2(gm))),
    }
}

// All pairs of `Vec<T>`, where `T` is unsigned and first `Vec` is at least as long as the second.
pub fn pairs_of_unsigned_vec_var_3<T: PrimitiveUnsigned>(
    gm: GenerationMode,
) -> Box<Iterator<Item = (Vec<T>, Vec<T>)>> {
    Box::new(pairs_of_unsigned_vec(gm).filter(|&(ref xs, ref ys)| xs.len() >= ys.len()))
}

// All pairs of `Vec<u32>` where both elements are nonempty and don't only contain zeros.
pub fn pairs_of_u32_vec_var_1(gm: GenerationMode) -> Box<Iterator<Item = (Vec<u32>, Vec<u32>)>> {
    Box::new(
        pairs_of_unsigned_vec(gm)
            .filter(|&(ref xs, ref ys)| !limbs_test_zero(xs) && !limbs_test_zero(ys)),
    )
}

// All pairs of `Vec<u32>` where both elements are nonempty and don't only contain zeros, and the
// first element is at least as long as the second.
pub fn pairs_of_u32_vec_var_2(gm: GenerationMode) -> Box<Iterator<Item = (Vec<u32>, Vec<u32>)>> {
    Box::new(pairs_of_unsigned_vec(gm).filter(|&(ref xs, ref ys)| {
        xs.len() >= ys.len() && !limbs_test_zero(xs) && !limbs_test_zero(ys)
    }))
}

// All pairs of `Vec<u32>`, where the first `Vec` is at least as long as the second and the second
// `Vec` is nonempty and represents a `Natural` divisible by 3.
pub fn pairs_of_u32_vec_var_3(gm: GenerationMode) -> Box<Iterator<Item = (Vec<u32>, Vec<u32>)>> {
    Box::new(
        pairs_of_unsigned_vec(gm)
            .map(|(out_limbs, in_limbs)| (out_limbs, limbs_mul_limb(&in_limbs, 3)))
            .filter(|(ref out_limbs, ref in_limbs)| {
                out_limbs.len() >= in_limbs.len() && in_limbs.len() > 0
            }),
    )
}

fn pairs_of_two_unsigned_vec_and_bool<T: PrimitiveUnsigned>(
    gm: GenerationMode,
) -> Box<Iterator<Item = (Vec<T>, bool)>> {
    match gm {
        GenerationMode::Exhaustive => Box::new(log_pairs(
            exhaustive_vecs(exhaustive_unsigned()),
            exhaustive_bools(),
        )),
        GenerationMode::Random(scale) => Box::new(random_pairs(
            &EXAMPLE_SEED,
            &(|seed| random_vecs(seed, scale, &(|seed_2| random(seed_2)))),
            &(|seed| random(seed)),
        )),
        GenerationMode::SpecialRandom(scale) => Box::new(random_pairs(
            &EXAMPLE_SEED,
            &(|seed| special_random_unsigned_vecs(seed, scale)),
            &(|seed| random(seed)),
        )),
    }
}

// All triples of `Vec<T>`, `Vec<T>`, and `bool`, where `T` is unsigned and the two `Vec`s have the
// same length.
pub fn triples_of_two_unsigned_vecs_and_bool_var_1<T: PrimitiveUnsigned>(
    gm: GenerationMode,
) -> Box<Iterator<Item = (Vec<T>, Vec<T>, bool)>> {
    Box::new(
        pairs_of_two_unsigned_vec_and_bool(gm)
            .filter(|(xs, _)| xs.len().even())
            .map(|(xs, b)| {
                let half_length = xs.len() >> 1;
                (xs[..half_length].to_vec(), xs[half_length..].to_vec(), b)
            }),
    )
}

pub fn triples_of_unsigned_vec<T: PrimitiveUnsigned>(
    gm: GenerationMode,
) -> Box<Iterator<Item = (Vec<T>, Vec<T>, Vec<T>)>> {
    match gm {
        GenerationMode::Exhaustive => Box::new(exhaustive_triples_from_single(exhaustive_vecs(
            exhaustive_unsigned(),
        ))),
        GenerationMode::Random(scale) => Box::new(random_triples_from_single(random_vecs(
            &EXAMPLE_SEED,
            scale,
            &(|seed| random(seed)),
        ))),
        GenerationMode::SpecialRandom(scale) => Box::new(random_triples_from_single(
            special_random_unsigned_vecs(&EXAMPLE_SEED, scale),
        )),
    }
}

// All triples of `Vec<T>`, T being unsigned, where the three components of the triple have the same
// length.
pub fn triples_of_unsigned_vec_var_1<T: PrimitiveUnsigned>(
    gm: GenerationMode,
) -> Box<Iterator<Item = (Vec<T>, Vec<T>, Vec<T>)>> {
    Box::new(
        vecs_of_unsigned(gm)
            .filter(|xs| xs.len() % 3 == 0)
            .map(|xs| {
                let third_length = xs.len() / 3;
                let two_thirds_length = third_length << 1;
                (
                    xs[..third_length].to_vec(),
                    xs[third_length..two_thirds_length].to_vec(),
                    xs[two_thirds_length..].to_vec(),
                )
            }),
    )
}

// All triples of `Vec<T>`, where `T` is unsigned and the last `T`s of all `Vec`s are nonzero.
pub fn triples_of_unsigned_vec_var_2<T: PrimitiveUnsigned>(
    gm: GenerationMode,
) -> Box<Iterator<Item = (Vec<T>, Vec<T>, Vec<T>)>> {
    match gm {
        GenerationMode::Exhaustive => Box::new(exhaustive_triples_from_single(
            exhaustive_vecs(exhaustive_unsigned())
                .filter(|limbs| limbs.is_empty() || *limbs.last().unwrap() != T::ZERO),
        )),
        _ => Box::new(random_triples_from_single(vecs_of_unsigned_var_2(gm))),
    }
}

// All triples of `Vec<T>`, where `T` is unsigned, the first `Vec` is at least as long as the
// second, and the second and third are equally long.
pub fn triples_of_unsigned_vec_var_3<T: PrimitiveUnsigned>(
    gm: GenerationMode,
) -> Box<Iterator<Item = (Vec<T>, Vec<T>, Vec<T>)>> {
    Box::new(
        triples_of_unsigned_vec(gm)
            .filter(|&(ref xs, ref ys, ref zs)| ys.len() == zs.len() && xs.len() >= ys.len()),
    )
}

// All triples of `Vec<T>`, where `T` is unsigned and the first `Vec` is at least as long as each of
// the others.
pub fn triples_of_unsigned_vec_var_4<T: PrimitiveUnsigned>(
    gm: GenerationMode,
) -> Box<Iterator<Item = (Vec<T>, Vec<T>, Vec<T>)>> {
    Box::new(
        triples_of_unsigned_vec(gm)
            .filter(|&(ref xs, ref ys, ref zs)| xs.len() >= ys.len() && xs.len() >= zs.len()),
    )
}

// All triples of `Vec<u32>` where the second and third elements are nonempty and don't only contain
// zeros, and the first is at least as long as the second.
pub fn triples_of_u32_vec_var_5(
    gm: GenerationMode,
) -> Box<Iterator<Item = (Vec<u32>, Vec<u32>, Vec<u32>)>> {
    Box::new(
        triples_of_unsigned_vec(gm).filter(|&(ref xs, ref ys, ref zs)| {
            xs.len() >= ys.len() && !limbs_test_zero(ys) && !limbs_test_zero(zs)
        }),
    )
}

// All triples of `Vec<u32>` where the second and third elements are nonempty and don't only contain
// zeros, and the first is at least as long as the third.
pub fn triples_of_u32_vec_var_6(
    gm: GenerationMode,
) -> Box<Iterator<Item = (Vec<u32>, Vec<u32>, Vec<u32>)>> {
    Box::new(
        triples_of_unsigned_vec(gm).filter(|&(ref xs, ref ys, ref zs)| {
            xs.len() >= zs.len() && !limbs_test_zero(ys) && !limbs_test_zero(zs)
        }),
    )
}

// All triples of `Vec<u32>` where the second and third elements are nonempty and don't only contain
// zeros, and the first is at least as long as the second and at least as long as the third.
pub fn triples_of_u32_vec_var_7(
    gm: GenerationMode,
) -> Box<Iterator<Item = (Vec<u32>, Vec<u32>, Vec<u32>)>> {
    Box::new(
        triples_of_unsigned_vec(gm).filter(|&(ref xs, ref ys, ref zs)| {
            xs.len() >= ys.len()
                && xs.len() >= zs.len()
                && !limbs_test_zero(ys)
                && !limbs_test_zero(zs)
        }),
    )
}

// All triples of `Vec<u32>` where the second and third elements are nonempty and don't only contain
// zeros, and the first is at least as long as the second or at least as long as the third.
pub fn triples_of_u32_vec_var_8(
    gm: GenerationMode,
) -> Box<Iterator<Item = (Vec<u32>, Vec<u32>, Vec<u32>)>> {
    Box::new(
        triples_of_unsigned_vec(gm).filter(|&(ref xs, ref ys, ref zs)| {
            (xs.len() >= ys.len() || xs.len() >= zs.len())
                && !limbs_test_zero(ys)
                && !limbs_test_zero(zs)
        }),
    )
}

// All triples of `Vec<T>`, where `T` is unsigned, the first `Vec` is at least as long as the
// second, and the second is at least as long as the third.
pub fn triples_of_unsigned_vec_var_9<T: PrimitiveUnsigned>(
    gm: GenerationMode,
) -> Box<Iterator<Item = (Vec<T>, Vec<T>, Vec<T>)>> {
    Box::new(
        triples_of_unsigned_vec(gm)
            .filter(|&(ref xs, ref ys, ref zs)| xs.len() >= ys.len() && ys.len() >= zs.len()),
    )
}

// All triples of `Vec<T>`, where `T` is unsigned, the first `Vec` is at least as long as the sum of
// the second and the third, the second is at least as long as the third, and the third is nonempty.
pub fn triples_of_unsigned_vec_var_10<T: PrimitiveUnsigned>(
    gm: GenerationMode,
) -> Box<Iterator<Item = (Vec<T>, Vec<T>, Vec<T>)>> {
    Box::new(
        triples_of_unsigned_vec(gm).filter(|&(ref xs, ref ys, ref zs)| {
            !zs.is_empty() && ys.len() >= zs.len() && xs.len() >= ys.len() + zs.len()
        }),
    )
}

pub fn triples_of_unsigned_vec_min_sizes<T: PrimitiveUnsigned>(
    gm: GenerationMode,
    min_xs_len: u64,
    min_ys_len: u64,
    min_zs_len: u64,
) -> Box<Iterator<Item = (Vec<T>, Vec<T>, Vec<T>)>> {
    match gm {
        GenerationMode::Exhaustive => Box::new(exhaustive_triples(
            exhaustive_vecs_min_length(min_xs_len, exhaustive_unsigned()),
            exhaustive_vecs_min_length(min_ys_len, exhaustive_unsigned()),
            exhaustive_vecs_min_length(min_zs_len, exhaustive_unsigned()),
        )),
        GenerationMode::Random(scale) => Box::new(random_triples(
            &EXAMPLE_SEED,
            &(|seed| random_vecs_min_length(seed, scale, min_xs_len, &(|seed_2| random(seed_2)))),
            &(|seed| random_vecs_min_length(seed, scale, min_ys_len, &(|seed_2| random(seed_2)))),
            &(|seed| random_vecs_min_length(seed, scale, min_zs_len, &(|seed_2| random(seed_2)))),
        )),
        GenerationMode::SpecialRandom(scale) => Box::new(random_triples(
            &EXAMPLE_SEED,
            &(|seed| special_random_unsigned_vecs_min_length(seed, scale, min_xs_len)),
            &(|seed| special_random_unsigned_vecs_min_length(seed, scale, min_ys_len)),
            &(|seed| special_random_unsigned_vecs_min_length(seed, scale, min_zs_len)),
        )),
    }
}

// All triples of `Vec<T>`, where `T` is unsigned and `out_limbs`, `xs`, and `ys` meet the
// preconditions of `_limbs_mul_to_out_toom_22`.
pub fn triples_of_unsigned_vec_var_11<T: PrimitiveUnsigned>(
    gm: GenerationMode,
) -> Box<Iterator<Item = (Vec<T>, Vec<T>, Vec<T>)>> {
    Box::new(triples_of_unsigned_vec_min_sizes(gm, 3, 3, 1).filter(
        |&(ref out_limbs, ref xs, ref ys)| {
            xs.len() >= ys.len()
                && out_limbs.len() >= xs.len() + ys.len()
                && if xs.len().even() {
                    xs.len()
                } else {
                    xs.len() + 1
                } < 2 * ys.len()
        },
    ))
}

// All triples of `Vec<T>`, where `T` is unsigned and `out_limbs`, `xs`, and `ys` meet the
// preconditions of `_limbs_mul_to_out_toom_32`.
pub fn triples_of_unsigned_vec_var_12<T: PrimitiveUnsigned>(
    gm: GenerationMode,
) -> Box<Iterator<Item = (Vec<T>, Vec<T>, Vec<T>)>> {
    Box::new(triples_of_unsigned_vec_min_sizes(gm, 10, 6, 4).filter(
        |&(ref out_limbs, ref xs, ref ys)| {
            out_limbs.len() >= xs.len() + ys.len()
                && _limbs_mul_to_out_toom_32_input_sizes_valid(xs.len(), ys.len())
        },
    ))
}

// All triples of `Vec<T>`, where `T` is unsigned and `out_limbs`, `xs`, and `ys` meet the
// preconditions of `_limbs_mul_to_out_toom_33`.
pub fn triples_of_unsigned_vec_var_13<T: PrimitiveUnsigned>(
    gm: GenerationMode,
) -> Box<Iterator<Item = (Vec<T>, Vec<T>, Vec<T>)>> {
    Box::new(triples_of_unsigned_vec_min_sizes(gm, 10, 5, 5).filter(
        |&(ref out_limbs, ref xs, ref ys)| {
            out_limbs.len() >= xs.len() + ys.len()
                && _limbs_mul_to_out_toom_33_input_sizes_valid(xs.len(), ys.len())
        },
    ))
}

// All triples of `Vec<T>`, where `T` is unsigned and `out_limbs`, `xs`, and `ys` meet the
// preconditions of `_limbs_mul_to_out_toom_42`.
pub fn triples_of_unsigned_vec_var_14<T: PrimitiveUnsigned>(
    gm: GenerationMode,
) -> Box<Iterator<Item = (Vec<T>, Vec<T>, Vec<T>)>> {
    Box::new(triples_of_unsigned_vec_min_sizes(gm, 6, 4, 2).filter(
        |&(ref out_limbs, ref xs, ref ys)| {
            out_limbs.len() >= xs.len() + ys.len()
                && _limbs_mul_to_out_toom_42_input_sizes_valid(xs.len(), ys.len())
        },
    ))
}

pub fn quadruples_of_three_unsigned_vecs_and_bool<T: PrimitiveUnsigned>(
    gm: GenerationMode,
) -> Box<Iterator<Item = (Vec<T>, Vec<T>, Vec<T>, bool)>> {
    match gm {
        GenerationMode::Exhaustive => Box::new(exhaustive_quadruples(
            exhaustive_vecs(exhaustive_unsigned()),
            exhaustive_vecs(exhaustive_unsigned()),
            exhaustive_vecs(exhaustive_unsigned()),
            exhaustive_bools(),
        )),
        GenerationMode::Random(scale) => Box::new(random_quadruples(
            &EXAMPLE_SEED,
            &(|seed| random_vecs(seed, scale, &(|seed_2| random(seed_2)))),
            &(|seed| random_vecs(seed, scale, &(|seed_2| random(seed_2)))),
            &(|seed| random_vecs(seed, scale, &(|seed_2| random(seed_2)))),
            &(|seed| random(seed)),
        )),
        GenerationMode::SpecialRandom(scale) => Box::new(random_quadruples(
            &EXAMPLE_SEED,
            &(|seed| special_random_unsigned_vecs(seed, scale)),
            &(|seed| special_random_unsigned_vecs(seed, scale)),
            &(|seed| special_random_unsigned_vecs(seed, scale)),
            &(|seed| random(seed)),
        )),
    }
}

// All quadruples of `Vec<T>`, `Vec<T>`, `Vec<T>`, and `bool`, where `T` is unsigned, the first
// `Vec` is at least as long as the second, and the second and third are equally long.
pub fn quadruples_of_three_unsigned_vecs_and_bool_var_1<T: PrimitiveUnsigned>(
    gm: GenerationMode,
) -> Box<Iterator<Item = (Vec<T>, Vec<T>, Vec<T>, bool)>> {
    Box::new(
        quadruples_of_three_unsigned_vecs_and_bool(gm)
            .filter(|&(ref xs, ref ys, ref zs, _)| ys.len() == zs.len() && xs.len() >= ys.len()),
    )
}

fn pairs_of_ordering_and_vec_of_unsigned<T: PrimitiveUnsigned>(
    gm: GenerationMode,
) -> Box<Iterator<Item = (Ordering, Vec<T>)>> {
    match gm {
        GenerationMode::Exhaustive => permute_1_2(Box::new(lex_pairs(
            exhaustive_vecs(exhaustive_unsigned()),
            exhaustive_orderings(),
        ))),
        GenerationMode::Random(scale) => Box::new(random_pairs(
            &EXAMPLE_SEED,
            &(|seed| random_orderings(seed)),
            &(|seed| random_vecs(seed, scale, &(|seed_2| random(seed_2)))),
        )),
        GenerationMode::SpecialRandom(scale) => Box::new(random_pairs(
            &EXAMPLE_SEED,
            &(|seed| random_orderings(seed)),
            &(|seed| special_random_unsigned_vecs(seed, scale)),
        )),
    }
}

// All pairs of `Ordering` and `Vec<T>` where `T` is unsigned, such that the `Ordering` is
// `Ordering::Equal` iff all `T`s in the `Vec` are zero.
pub fn pairs_of_ordering_and_vec_of_unsigned_var_1(
    gm: GenerationMode,
) -> Box<Iterator<Item = (Ordering, Vec<u32>)>> {
    Box::new(
        pairs_of_ordering_and_vec_of_unsigned(gm)
            .filter(|&(sign, ref limbs)| limbs_test_zero(limbs) == (sign == Ordering::Equal)),
    )
}

fn exhaustive_pairs_of_unsigned_vec_and_unsigned<T: PrimitiveUnsigned>(
) -> Box<Iterator<Item = (Vec<T>, T)>> {
    Box::new(exhaustive_pairs(
        exhaustive_vecs(exhaustive_unsigned()),
        exhaustive_unsigned(),
    ))
}

pub fn triples_of_unsigned_vec_small_usize_and_unsigned<T: PrimitiveUnsigned>(
    gm: GenerationMode,
) -> Box<Iterator<Item = (Vec<T>, usize, T)>> {
    match gm {
        GenerationMode::Exhaustive => permute_1_3_2(reshape_2_1_to_3(Box::new(log_pairs(
            exhaustive_pairs_of_unsigned_vec_and_unsigned(),
            exhaustive_unsigned::<u32>().map(|u| u as usize),
        )))),
        GenerationMode::Random(scale) => Box::new(random_triples(
            &EXAMPLE_SEED,
            &(|seed| random_vecs(seed, scale, &(|seed_2| random(seed_2)))),
            &(|seed| u32s_geometric(seed, scale).map(|u| u as usize)),
            &(|seed| random(seed)),
        )),
        GenerationMode::SpecialRandom(scale) => Box::new(random_triples(
            &EXAMPLE_SEED,
            &(|seed| special_random_unsigned_vecs(seed, scale)),
            &(|seed| u32s_geometric(seed, scale).map(|u| u as usize)),
            &(|seed| special_random_unsigned(seed)),
        )),
    }
}

fn pairs_of_unsigned_vec_and_small_usize<T: PrimitiveUnsigned>(
    gm: GenerationMode,
) -> Box<Iterator<Item = (Vec<T>, usize)>> {
    match gm {
        GenerationMode::Exhaustive => Box::new(log_pairs(
            exhaustive_vecs(exhaustive_unsigned()),
            exhaustive_unsigned::<u32>().map(|u| u as usize),
        )),
        GenerationMode::Random(scale) => Box::new(random_pairs(
            &EXAMPLE_SEED,
            &(|seed| random_vecs(seed, scale, &(|seed_2| random(seed_2)))),
            &(|seed| u32s_geometric(seed, scale).map(|u| u as usize)),
        )),
        GenerationMode::SpecialRandom(scale) => Box::new(random_pairs(
            &EXAMPLE_SEED,
            &(|seed| special_random_unsigned_vecs(seed, scale)),
            &(|seed| u32s_geometric(seed, scale).map(|u| u as usize)),
        )),
    }
}

// All pairs of `Vec<T>`, where `T` is unsigned, and `usize`, where the `usize` is no larger than
// the length of the `Vec`.
pub fn pairs_of_unsigned_vec_and_small_usize_var_1<T: PrimitiveUnsigned>(
    gm: GenerationMode,
) -> Box<Iterator<Item = (Vec<T>, usize)>> {
    Box::new(pairs_of_unsigned_vec_and_small_usize(gm).filter(|&(ref xs, u)| u <= xs.len()))
}

pub fn pairs_of_unsigned_vec_and_unsigned<T: PrimitiveUnsigned>(
    gm: GenerationMode,
) -> Box<Iterator<Item = (Vec<T>, T)>> {
    match gm {
        GenerationMode::Exhaustive => exhaustive_pairs_of_unsigned_vec_and_unsigned(),
        GenerationMode::Random(scale) => Box::new(random_pairs(
            &EXAMPLE_SEED,
            &(|seed| random_vecs(seed, scale, &(|seed_2| random(seed_2)))),
            &(|seed| random(seed)),
        )),
        GenerationMode::SpecialRandom(scale) => Box::new(random_pairs(
            &EXAMPLE_SEED,
            &(|seed| special_random_unsigned_vecs(seed, scale)),
            &(|seed| special_random_unsigned(seed)),
        )),
    }
}

pub fn pairs_of_nonempty_unsigned_vec_and_unsigned<T: PrimitiveUnsigned>(
    gm: GenerationMode,
) -> Box<Iterator<Item = (Vec<T>, T)>> {
    Box::new(pairs_of_unsigned_vec_and_unsigned(gm).filter(|&(ref xs, _)| !xs.is_empty()))
}

pub fn pairs_of_unsigned_vec_and_positive_unsigned<T: PrimitiveUnsigned>(
    gm: GenerationMode,
) -> Box<Iterator<Item = (Vec<T>, T)>> {
    match gm {
        GenerationMode::Exhaustive => Box::new(Box::new(exhaustive_pairs(
            exhaustive_vecs(exhaustive_unsigned()),
            exhaustive_positive(),
        ))),
        GenerationMode::Random(scale) => Box::new(random_pairs(
            &EXAMPLE_SEED,
            &(|seed| random_vecs(seed, scale, &(|seed_2| random(seed_2)))),
            &(|seed| random_positive_unsigned(seed)),
        )),
        GenerationMode::SpecialRandom(scale) => Box::new(random_pairs(
            &EXAMPLE_SEED,
            &(|seed| special_random_unsigned_vecs(seed, scale)),
            &(|seed| special_random_positive_unsigned(seed)),
        )),
    }
}

// All pairs of `Vec<T>` and positive `T`, where `T` is unsigned and the `Vec`'s length is greater
// than 1.
pub fn pairs_of_unsigned_vec_and_positive_unsigned_var_1<T: PrimitiveUnsigned>(
    gm: GenerationMode,
) -> Box<Iterator<Item = (Vec<T>, T)>> {
    Box::new(pairs_of_unsigned_vec_and_positive_unsigned(gm).filter(|&(ref xs, _)| xs.len() > 1))
}

// All pairs of `Vec<T>` and positive `u32`, where the `Vec` is nonempty and represents a `Natural`
// divisible by the `u32`.
pub fn pairs_of_u32_vec_and_positive_u32_var_2(
    gm: GenerationMode,
) -> Box<Iterator<Item = (Vec<u32>, u32)>> {
    Box::new(
        pairs_of_unsigned_vec_and_positive_unsigned(gm)
            .filter(|(ref limbs, _)| limbs.len() > 0)
            .map(|(limbs, limb)| (limbs_mul_limb(&limbs, limb), limb)),
    )
}

// All pairs of `Vec<T>` where `T` is unsigned, and a `u32` between 1 and 31, inclusive.
pub fn pairs_of_unsigned_vec_and_u32_var_1<T: PrimitiveUnsigned>(
    gm: GenerationMode,
) -> Box<Iterator<Item = (Vec<T>, u32)>> {
    match gm {
        GenerationMode::Exhaustive => Box::new(exhaustive_pairs(
            exhaustive_vecs(exhaustive_unsigned()),
            range_increasing(1, u32::WIDTH - 1),
        )),
        GenerationMode::Random(scale) => Box::new(random_pairs(
            &EXAMPLE_SEED,
            &(|seed| random_vecs(seed, scale, &(|seed_2| random(seed_2)))),
            &(|seed| random_range(seed, 1, u32::WIDTH - 1)),
        )),
        GenerationMode::SpecialRandom(scale) => Box::new(random_pairs(
            &EXAMPLE_SEED,
            &(|seed| special_random_unsigned_vecs(seed, scale)),
            &(|seed| random_range(seed, 1, u32::WIDTH - 1)),
        )),
    }
}

// All pairs of `Vec<T>` where `T` is unsigned, and a `u32` between 1 and 31, inclusive, where the
// `Vec` is nonempty.
pub fn pairs_of_unsigned_vec_and_u32_var_2<T: PrimitiveUnsigned>(
    gm: GenerationMode,
) -> Box<Iterator<Item = (Vec<T>, u32)>> {
    Box::new(pairs_of_unsigned_vec_and_u32_var_1(gm).filter(|&(ref xs, _)| !xs.is_empty()))
}

pub fn pairs_of_unsigned_vec_and_small_unsigned<T: PrimitiveUnsigned, U: PrimitiveUnsigned>(
    gm: GenerationMode,
) -> Box<Iterator<Item = (Vec<T>, U)>> {
    match gm {
        GenerationMode::Exhaustive => Box::new(log_pairs(
            exhaustive_vecs(exhaustive_unsigned()),
            exhaustive_unsigned(),
        )),
        GenerationMode::Random(scale) => Box::new(random_pairs(
            &EXAMPLE_SEED,
            &(|seed| random_vecs(seed, scale, &(|seed_2| random(seed_2)))),
            &(|seed| u32s_geometric(seed, scale).flat_map(U::checked_from)),
        )),
        GenerationMode::SpecialRandom(scale) => Box::new(random_pairs(
            &EXAMPLE_SEED,
            &(|seed| special_random_unsigned_vecs(seed, scale)),
            &(|seed| u32s_geometric(seed, scale).flat_map(U::checked_from)),
        )),
    }
}

// All pairs of `Vec<T>` and small `U` where `T` and `U` are unsigned and the `Vec<T>` is nonempty
// and doesn't only contain zeros.
pub fn pairs_of_unsigned_vec_and_small_unsigned_var_1<
    T: PrimitiveUnsigned,
    U: PrimitiveUnsigned,
>(
    gm: GenerationMode,
) -> Box<Iterator<Item = (Vec<T>, U)>> {
    Box::new(
        pairs_of_unsigned_vec_and_small_unsigned(gm)
            .filter(|&(ref limbs, _)| !limbs_test_zero(limbs)),
    )
}

// All pairs of `Vec<u32>` and small `u64` where the u64 is less than 32 * the length of the `Vec`.
pub fn pairs_of_u32_vec_and_small_u64_var_2(
    gm: GenerationMode,
) -> Box<Iterator<Item = (Vec<u32>, u64)>> {
    Box::new(
        pairs_of_unsigned_vec_and_small_unsigned(gm)
            .filter(|&(ref limbs, index)| index < (limbs.len() as u64) << u32::LOG_WIDTH),
    )
}

// All pairs of `Vec<u32>` and small `u64` where `limbs_slice_clear_bit_neg` applied to the `Vec`
// and `u64` doesn't panic.
pub fn pairs_of_u32_vec_and_small_u64_var_3(
    gm: GenerationMode,
) -> Box<Iterator<Item = (Vec<u32>, u64)>> {
    Box::new(
        pairs_of_unsigned_vec_and_small_unsigned_var_1(gm).filter(|&(ref limbs, index)| {
            let mut mut_limbs = limbs.clone();
            limbs_vec_clear_bit_neg(&mut mut_limbs, index);
            mut_limbs.len() == limbs.len()
        }),
    )
}

// All pairs of `Vec<u32>` and `u32`, where the `Vec<u32>` are nonempty and don't only contain
// zeros.
pub fn pairs_of_u32_vec_and_u32_var_1(gm: GenerationMode) -> Box<Iterator<Item = (Vec<u32>, u32)>> {
    Box::new(
        pairs_of_unsigned_vec_and_unsigned(gm).filter(|&(ref limbs, _)| !limbs_test_zero(limbs)),
    )
}

// All pairs of `Vec<u32>` and positive `u32`, where the `Vec<u32>` are nonempty and don't only
// contain zeros.
pub fn pairs_of_u32_vec_and_positive_u32_var_1(
    gm: GenerationMode,
) -> Box<Iterator<Item = (Vec<u32>, u32)>> {
    Box::new(
        pairs_of_unsigned_vec_and_unsigned(gm)
            .filter(|&(ref limbs, limb)| limb != 0 && !limbs_test_zero(limbs)),
    )
}

pub fn vecs_of_bool(gm: GenerationMode) -> Box<Iterator<Item = Vec<bool>>> {
    match gm {
        //TODO shortlex would be better
        GenerationMode::Exhaustive => Box::new(exhaustive_vecs(exhaustive_bools())),
        GenerationMode::Random(scale) => {
            Box::new(random_vecs(&EXAMPLE_SEED, scale, &(|seed| random(seed))))
        }
        GenerationMode::SpecialRandom(scale) => {
            Box::new(special_random_bool_vecs(&EXAMPLE_SEED, scale))
        }
    }
}

// All `Vec<bool>` that are nonempty and don't only contain `false`s.
pub fn vecs_of_bool_var_1(gm: GenerationMode) -> Box<Iterator<Item = Vec<bool>>> {
    Box::new(vecs_of_bool(gm).filter(|bits| bits.iter().any(|&bit| bit)))
}

fn triples_of_unsigned_vec_unsigned_vec_and_unsigned<T: PrimitiveUnsigned>(
    gm: GenerationMode,
) -> Box<Iterator<Item = (Vec<T>, Vec<T>, T)>> {
    match gm {
        GenerationMode::Exhaustive => Box::new(exhaustive_triples(
            exhaustive_vecs(exhaustive_unsigned()),
            exhaustive_vecs(exhaustive_unsigned()),
            exhaustive_unsigned(),
        )),
        GenerationMode::Random(scale) => Box::new(random_triples(
            &EXAMPLE_SEED,
            &(|seed| random_vecs(seed, scale, &(|seed_2| random(seed_2)))),
            &(|seed| random_vecs(seed, scale, &(|seed_2| random(seed_2)))),
            &(|seed| random(seed)),
        )),
        GenerationMode::SpecialRandom(scale) => Box::new(random_triples(
            &EXAMPLE_SEED,
            &(|seed| special_random_unsigned_vecs(seed, scale)),
            &(|seed| special_random_unsigned_vecs(seed, scale)),
            &(|seed| special_random_unsigned(seed)),
        )),
    }
}

fn triples_of_unsigned_vec_unsigned_vec_and_positive_unsigned<T: PrimitiveUnsigned>(
    gm: GenerationMode,
) -> Box<Iterator<Item = (Vec<T>, Vec<T>, T)>> {
    match gm {
        GenerationMode::Exhaustive => Box::new(exhaustive_triples(
            exhaustive_vecs(exhaustive_unsigned()),
            exhaustive_vecs(exhaustive_unsigned()),
            exhaustive_positive(),
        )),
        GenerationMode::Random(scale) => Box::new(random_triples(
            &EXAMPLE_SEED,
            &(|seed| random_vecs(seed, scale, &(|seed_2| random(seed_2)))),
            &(|seed| random_vecs(seed, scale, &(|seed_2| random(seed_2)))),
            &(|seed| random_positive_unsigned(seed)),
        )),
        GenerationMode::SpecialRandom(scale) => Box::new(random_triples(
            &EXAMPLE_SEED,
            &(|seed| special_random_unsigned_vecs(seed, scale)),
            &(|seed| special_random_unsigned_vecs(seed, scale)),
            &(|seed| special_random_positive_unsigned(seed)),
        )),
    }
}

// All triples of `Vec<T>`, `Vec<T>`, and `T` where `T` is unsigned and the first `Vec` is at least
// as long as the second.
pub fn triples_of_unsigned_vec_unsigned_vec_and_unsigned_var_1<T: PrimitiveUnsigned>(
    gm: GenerationMode,
) -> Box<Iterator<Item = (Vec<T>, Vec<T>, T)>> {
    Box::new(
        triples_of_unsigned_vec_unsigned_vec_and_unsigned(gm)
            .filter(|&(ref out_limbs, ref in_limbs, _)| out_limbs.len() >= in_limbs.len()),
    )
}

// All triples of `Vec<T>`, `Vec<T>`, and `T` where `T` is unsigned, the first `Vec` is at least as
// long as the second, and the second is nonempty.
pub fn triples_of_unsigned_vec_unsigned_vec_and_unsigned_var_2<T: PrimitiveUnsigned>(
    gm: GenerationMode,
) -> Box<Iterator<Item = (Vec<T>, Vec<T>, T)>> {
    Box::new(
        triples_of_unsigned_vec_unsigned_vec_and_unsigned(gm).filter(
            |&(ref out_limbs, ref in_limbs, _)| {
                !in_limbs.is_empty() && out_limbs.len() >= in_limbs.len()
            },
        ),
    )
}

// All triples of `Vec<T>`, `Vec<T>`, and `T` where `T` is unsigned, the first `Vec` is at least as
// long as the second, and the second doesn't only contain zeros.
pub fn triples_of_unsigned_vec_unsigned_vec_and_unsigned_var_3<T: PrimitiveUnsigned>(
    gm: GenerationMode,
) -> Box<Iterator<Item = (Vec<T>, Vec<T>, T)>> {
    Box::new(
        triples_of_unsigned_vec_unsigned_vec_and_unsigned(gm).filter(
            |&(ref out_limbs, ref in_limbs, _)| {
                out_limbs.len() >= in_limbs.len() && !limbs_test_zero(in_limbs)
            },
        ),
    )
}

// All triples of `Vec<T>`, `Vec<T>`, and `u32` where `T` is unsigned and the `u32` is between 1 and
// 31, inclusive.
fn triples_of_unsigned_vec_unsigned_vec_and_u32_var_4<T: PrimitiveUnsigned>(
    gm: GenerationMode,
) -> Box<Iterator<Item = (Vec<T>, Vec<T>, u32)>> {
    match gm {
        GenerationMode::Exhaustive => Box::new(exhaustive_triples(
            exhaustive_vecs(exhaustive_unsigned()),
            exhaustive_vecs(exhaustive_unsigned()),
            range_increasing(1, u32::WIDTH - 1),
        )),
        GenerationMode::Random(scale) => Box::new(random_triples(
            &EXAMPLE_SEED,
            &(|seed| random_vecs(seed, scale, &(|seed_2| random(seed_2)))),
            &(|seed| random_vecs(seed, scale, &(|seed_2| random(seed_2)))),
            &(|seed| random_range(seed, 1, u32::WIDTH - 1)),
        )),
        GenerationMode::SpecialRandom(scale) => Box::new(random_triples(
            &EXAMPLE_SEED,
            &(|seed| special_random_unsigned_vecs(seed, scale)),
            &(|seed| special_random_unsigned_vecs(seed, scale)),
            &(|seed| random_range(seed, 1, u32::WIDTH - 1)),
        )),
    }
}

// All triples of `Vec<T>`, `Vec<T>`, and `u32` where `T` is unsigned, the first `Vec` is at least
// as long as the second, and the `u32` is between 1 and 31, inclusive.
pub fn triples_of_unsigned_vec_unsigned_vec_and_u32_var_5<T: PrimitiveUnsigned>(
    gm: GenerationMode,
) -> Box<Iterator<Item = (Vec<T>, Vec<T>, u32)>> {
    Box::new(
        triples_of_unsigned_vec_unsigned_vec_and_u32_var_4(gm)
            .filter(|&(ref out_limbs, ref in_limbs, _)| out_limbs.len() >= in_limbs.len()),
    )
}

// All triples of `Vec<T>`, `Vec<T>`, and `u32` where `T` is unsigned, the first `Vec` is at least
// as long as the second, the second is nonempty, and the `u32` is between 1 and 31, inclusive.
pub fn triples_of_unsigned_vec_unsigned_vec_and_u32_var_6<T: PrimitiveUnsigned>(
    gm: GenerationMode,
) -> Box<Iterator<Item = (Vec<T>, Vec<T>, u32)>> {
    Box::new(
        triples_of_unsigned_vec_unsigned_vec_and_u32_var_4(gm).filter(
            |&(ref out_limbs, ref in_limbs, _)| {
                !in_limbs.is_empty() && out_limbs.len() >= in_limbs.len()
            },
        ),
    )
}

// All triples of `Vec<T>`, `Vec<T>`, and `T` where `T` is unsigned, the first `Vec` is at least as
// long as the second, and the length of the second `Vec` is greater than 1.
pub fn triples_of_unsigned_vec_unsigned_vec_and_positive_unsigned_var_1<T: PrimitiveUnsigned>(
    gm: GenerationMode,
) -> Box<Iterator<Item = (Vec<T>, Vec<T>, T)>> {
    Box::new(
        triples_of_unsigned_vec_unsigned_vec_and_positive_unsigned(gm).filter(
            |&(ref out_limbs, ref in_limbs, _)| {
                out_limbs.len() >= in_limbs.len() && in_limbs.len() > 1
            },
        ),
    )
}

// All triples of `Vec<u32>`, `Vec<u32>`, and positive `u32`, where the first `Vec` is at least as
// long as the second and the second `Vec` is nonempty and represents a `Natural` divisible by the
// `u32`.
pub fn triples_of_u32_vec_u32_vec_and_positive_u32_var_2(
    gm: GenerationMode,
) -> Box<Iterator<Item = (Vec<u32>, Vec<u32>, u32)>> {
    Box::new(
        triples_of_unsigned_vec_unsigned_vec_and_positive_unsigned(gm)
            .map(|(out_limbs, in_limbs, limb)| (out_limbs, limbs_mul_limb(&in_limbs, limb), limb))
            .filter(|(ref out_limbs, ref in_limbs, _)| {
                out_limbs.len() >= in_limbs.len() && in_limbs.len() > 0
            }),
    )
}

fn triples_of_unsigned_vec_unsigned_vec_and_small_unsigned<
    T: PrimitiveUnsigned,
    U: PrimitiveUnsigned,
>(
    gm: GenerationMode,
) -> Box<Iterator<Item = (Vec<T>, Vec<T>, U)>> {
    match gm {
        GenerationMode::Exhaustive => Box::new(exhaustive_triples(
            exhaustive_vecs(exhaustive_unsigned()),
            exhaustive_vecs(exhaustive_unsigned()),
            exhaustive_unsigned(),
        )),
        GenerationMode::Random(scale) => Box::new(random_triples(
            &EXAMPLE_SEED,
            &(|seed| random_vecs(seed, scale, &(|seed_2| random(seed_2)))),
            &(|seed| random_vecs(seed, scale, &(|seed_2| random(seed_2)))),
            &(|seed| u32s_geometric(seed, scale).flat_map(U::checked_from)),
        )),
        GenerationMode::SpecialRandom(scale) => Box::new(random_triples(
            &EXAMPLE_SEED,
            &(|seed| special_random_unsigned_vecs(seed, scale)),
            &(|seed| special_random_unsigned_vecs(seed, scale)),
            &(|seed| u32s_geometric(seed, scale).flat_map(U::checked_from)),
        )),
    }
}

// All triples of `Vec<T>`, `Vec<T>`, and small `U` where `T` and `U` are unsigned and the `Vec`s
// are nonempty and have no trailing zeros.
pub fn triples_of_unsigned_vec_unsigned_vec_and_small_unsigned_var_1<
    T: PrimitiveUnsigned,
    U: PrimitiveUnsigned,
>(
    gm: GenerationMode,
) -> Box<Iterator<Item = (Vec<T>, Vec<T>, U)>> {
    Box::new(
        triples_of_unsigned_vec_unsigned_vec_and_small_unsigned(gm).filter(
            |&(ref xs, ref ys, _)| {
                !xs.is_empty()
                    && !ys.is_empty()
                    && *xs.last().unwrap() != T::ZERO
                    && *ys.last().unwrap() != T::ZERO
            },
        ),
    )
}

// All triples of `Vec<T>`, T, and `T`, where `T` is unsigned, the second `T` is positive, the `Vec`
// has at least two elements, and the `Vec`'s last element is nonzero.
pub fn triples_of_unsigned_vec_unsigned_and_positive_unsigned_var_1<T: PrimitiveUnsigned>(
    gm: GenerationMode,
) -> Box<Iterator<Item = (Vec<T>, T, T)>> {
    match gm {
        GenerationMode::Exhaustive => Box::new(exhaustive_triples(
            exhaustive_vecs(exhaustive_unsigned())
                .filter(|limbs| limbs.len() > 1 && *limbs.last().unwrap() != T::ZERO),
            exhaustive_unsigned(),
            exhaustive_positive(),
        )),
        GenerationMode::Random(scale) => Box::new(random_triples(
            &EXAMPLE_SEED,
            &(|seed| {
                random_vecs(seed, scale, &(|seed_2| random(seed_2)))
                    .filter(|limbs| limbs.len() > 1 && *limbs.last().unwrap() != T::ZERO)
            }),
            &(|seed| random(seed)),
            &(|seed| random_positive_unsigned(seed)),
        )),
        GenerationMode::SpecialRandom(scale) => Box::new(random_triples(
            &EXAMPLE_SEED,
            &(|seed| {
                special_random_unsigned_vecs(seed, scale)
                    .filter(|limbs| limbs.len() > 1 && *limbs.last().unwrap() != T::ZERO)
            }),
            &(|seed| special_random_unsigned(seed)),
            &(|seed| special_random_positive_unsigned(seed)),
        )),
    }
}

// All triples of `Vec<T>`, T, and small `U`, where `T` and `U` are unsigned, the `Vec` is
// non-empty, and its last element is nonzero.
pub fn triples_of_unsigned_vec_unsigned_and_small_unsigned_var_1<
    T: PrimitiveUnsigned,
    U: PrimitiveUnsigned,
>(
    gm: GenerationMode,
) -> Box<Iterator<Item = (Vec<T>, T, U)>> {
    match gm {
        GenerationMode::Exhaustive => Box::new(exhaustive_triples(
            exhaustive_vecs(exhaustive_unsigned())
                .filter(|limbs| !limbs.is_empty() && *limbs.last().unwrap() != T::ZERO),
            exhaustive_unsigned(),
            exhaustive_unsigned(),
        )),
        GenerationMode::Random(scale) => Box::new(random_triples(
            &EXAMPLE_SEED,
            &(|seed| {
                random_vecs(seed, scale, &(|seed_2| random(seed_2)))
                    .filter(|limbs| !limbs.is_empty() && *limbs.last().unwrap() != T::ZERO)
            }),
            &(|seed| random(seed)),
            &(|seed| u32s_geometric(seed, scale).flat_map(U::checked_from)),
        )),
        GenerationMode::SpecialRandom(scale) => Box::new(random_triples(
            &EXAMPLE_SEED,
            &(|seed| {
                special_random_unsigned_vecs(seed, scale)
                    .filter(|limbs| !limbs.is_empty() && *limbs.last().unwrap() != T::ZERO)
            }),
            &(|seed| special_random_unsigned(seed)),
            &(|seed| u32s_geometric(seed, scale).flat_map(U::checked_from)),
        )),
    }
}

// All triples of `Vec<T>`, T, and small `U`, where `T` and `U` are unsigned, the `Vec` has at least
// 2 elements, and its last element is nonzero.
pub fn triples_of_unsigned_vec_unsigned_and_small_unsigned_var_2<
    T: PrimitiveUnsigned,
    U: PrimitiveUnsigned,
>(
    gm: GenerationMode,
) -> Box<Iterator<Item = (Vec<T>, T, U)>> {
    match gm {
        GenerationMode::Exhaustive => Box::new(exhaustive_triples(
            exhaustive_vecs(exhaustive_unsigned())
                .filter(|limbs| limbs.len() > 1 && *limbs.last().unwrap() != T::ZERO),
            exhaustive_unsigned(),
            exhaustive_unsigned(),
        )),
        GenerationMode::Random(scale) => Box::new(random_triples(
            &EXAMPLE_SEED,
            &(|seed| {
                random_vecs(seed, scale, &(|seed_2| random(seed_2)))
                    .filter(|limbs| limbs.len() > 1 && *limbs.last().unwrap() != T::ZERO)
            }),
            &(|seed| random(seed)),
            &(|seed| u32s_geometric(seed, scale).flat_map(U::checked_from)),
        )),
        GenerationMode::SpecialRandom(scale) => Box::new(random_triples(
            &EXAMPLE_SEED,
            &(|seed| {
                special_random_unsigned_vecs(seed, scale)
                    .filter(|limbs| limbs.len() > 1 && *limbs.last().unwrap() != T::ZERO)
            }),
            &(|seed| special_random_unsigned(seed)),
            &(|seed| u32s_geometric(seed, scale).flat_map(U::checked_from)),
        )),
    }
}

fn triples_of_unsigned_vec_small_unsigned_and_rounding_mode<
    T: PrimitiveUnsigned,
    U: PrimitiveUnsigned,
>(
    gm: GenerationMode,
) -> Box<Iterator<Item = (Vec<T>, U, RoundingMode)>> {
    match gm {
        GenerationMode::Exhaustive => Box::new(exhaustive_triples(
            exhaustive_vecs(exhaustive_unsigned()),
            exhaustive_unsigned(),
            exhaustive_rounding_modes(),
        )),
        GenerationMode::Random(scale) => Box::new(random_triples(
            &EXAMPLE_SEED,
            &(|seed| random_vecs(seed, scale, &(|seed_2| random(seed_2)))),
            &(|seed| u32s_geometric(seed, scale).flat_map(U::checked_from)),
            &(|seed| random_rounding_modes(seed)),
        )),
        GenerationMode::SpecialRandom(scale) => Box::new(random_triples(
            &EXAMPLE_SEED,
            &(|seed| special_random_unsigned_vecs(seed, scale)),
            &(|seed| u32s_geometric(seed, scale).flat_map(U::checked_from)),
            &(|seed| random_rounding_modes(seed)),
        )),
    }
}

// All triples of `Vec<T>`, small `U`, and `RoundingMode` where `T` and `U` are unsigned and the
// `Vec` doesn't only contain zeros.
pub fn triples_of_unsigned_vec_small_unsigned_and_rounding_mode_var_1<
    T: PrimitiveUnsigned,
    U: PrimitiveUnsigned,
>(
    gm: GenerationMode,
) -> Box<Iterator<Item = (Vec<T>, U, RoundingMode)>> {
    Box::new(
        triples_of_unsigned_vec_small_unsigned_and_rounding_mode(gm)
            .filter(|&(ref limbs, _, _)| !limbs_test_zero(limbs)),
    )
}

fn triples_of_unsigned_unsigned_vec_and_rounding_mode<T: PrimitiveUnsigned>(
    gm: GenerationMode,
) -> Box<Iterator<Item = (T, Vec<T>, RoundingMode)>> {
    match gm {
        GenerationMode::Exhaustive => Box::new(exhaustive_triples(
            exhaustive_unsigned(),
            exhaustive_vecs(exhaustive_unsigned()),
            exhaustive_rounding_modes(),
        )),
        GenerationMode::Random(scale) => Box::new(random_triples(
            &EXAMPLE_SEED,
            &(|seed| random(seed)),
            &(|seed| random_vecs(seed, scale, &(|seed_2| random(seed_2)))),
            &(|seed| random_rounding_modes(seed)),
        )),
        GenerationMode::SpecialRandom(scale) => Box::new(random_triples(
            &EXAMPLE_SEED,
            &(|seed| special_random_unsigned(seed)),
            &(|seed| special_random_unsigned_vecs(seed, scale)),
            &(|seed| random_rounding_modes(seed)),
        )),
    }
}

// All triples of `T`, `Vec<T>`, and `RoundingMode`, where `T` is unsigned and the `Vec` has length
// greater than one and its last element is nonzero.
pub fn triples_of_unsigned_unsigned_vec_and_rounding_mode_var_1<T: PrimitiveUnsigned>(
    gm: GenerationMode,
) -> Box<Iterator<Item = (T, Vec<T>, RoundingMode)>> {
    Box::new(
        triples_of_unsigned_unsigned_vec_and_rounding_mode(gm)
            .filter(|&(_, ref limbs, _)| limbs.len() > 1 && *limbs.last().unwrap() != T::ZERO),
    )
}

fn triples_of_unsigned_small_unsigned_and_rounding_mode<
    T: PrimitiveUnsigned,
    U: PrimitiveUnsigned,
>(
    gm: GenerationMode,
) -> Box<Iterator<Item = (T, U, RoundingMode)>> {
    match gm {
        GenerationMode::Exhaustive => reshape_2_1_to_3(Box::new(lex_pairs(
            log_pairs(exhaustive_unsigned(), exhaustive_unsigned()),
            exhaustive_rounding_modes(),
        ))),
        GenerationMode::Random(scale) => Box::new(random_triples(
            &EXAMPLE_SEED,
            &(|seed| random(seed)),
            &(|seed| u32s_geometric(seed, scale).flat_map(U::checked_from)),
            &(|seed| random_rounding_modes(seed)),
        )),
        GenerationMode::SpecialRandom(scale) => Box::new(random_triples(
            &EXAMPLE_SEED,
            &(|seed| special_random_unsigned(seed)),
            &(|seed| u32s_geometric(seed, scale).flat_map(U::checked_from)),
            &(|seed| random_rounding_modes(seed)),
        )),
    }
}

// All triples of `T`, small `U`, and `RoundingMode`, where `T` and `U` are unsigned and if the
// `RoundingMode` is `RoundingMode::Exact`, the `T` is divisible by 2 to the power of the `U`.
pub fn triples_of_unsigned_small_unsigned_and_rounding_mode_var_1<
    T: PrimitiveUnsigned,
    U: PrimitiveUnsigned,
>(
    gm: GenerationMode,
) -> Box<Iterator<Item = (T, U, RoundingMode)>>
where
    T: Shl<U, Output = T>,
    T: Shr<U, Output = T>,
{
    Box::new(
        triples_of_unsigned_small_unsigned_and_rounding_mode(gm).filter_map(|(n, u, rm)| {
            if n != T::ZERO && u >= U::checked_from(T::WIDTH).unwrap() {
                None
            } else if rm == RoundingMode::Exact {
                let shifted = n << u;
                if shifted >> u == n {
                    Some((shifted, u, rm))
                } else {
                    None
                }
            } else {
                Some((n, u, rm))
            }
        }),
    )
}

fn triples_of_signed_small_unsigned_and_rounding_mode<T: PrimitiveSigned, U: PrimitiveUnsigned>(
    gm: GenerationMode,
) -> Box<Iterator<Item = (T, U, RoundingMode)>> {
    match gm {
        GenerationMode::Exhaustive => reshape_2_1_to_3(Box::new(lex_pairs(
            log_pairs(exhaustive_signed(), exhaustive_unsigned()),
            exhaustive_rounding_modes(),
        ))),
        GenerationMode::Random(scale) => Box::new(random_triples(
            &EXAMPLE_SEED,
            &(|seed| random(seed)),
            &(|seed| u32s_geometric(seed, scale).flat_map(U::checked_from)),
            &(|seed| random_rounding_modes(seed)),
        )),
        GenerationMode::SpecialRandom(scale) => Box::new(random_triples(
            &EXAMPLE_SEED,
            &(|seed| special_random_signed(seed)),
            &(|seed| u32s_geometric(seed, scale).flat_map(U::checked_from)),
            &(|seed| random_rounding_modes(seed)),
        )),
    }
}

// All triples of `T`, small `U`, and `RoundingMode`, where `T` is signed, `U` is unsigned, and if
// the `RoundingMode` is `RoundingMode::Exact`, the `T` is divisible by 2 to the power of the `U`.
pub fn triples_of_signed_small_unsigned_and_rounding_mode_var_1<
    T: PrimitiveSigned,
    U: PrimitiveUnsigned,
>(
    gm: GenerationMode,
) -> Box<Iterator<Item = (T, U, RoundingMode)>>
where
    T: Shl<U, Output = T>,
    T: Shr<U, Output = T>,
{
    Box::new(
        triples_of_signed_small_unsigned_and_rounding_mode(gm).filter_map(|(n, u, rm)| {
            if n != T::ZERO && u >= U::checked_from(T::WIDTH).unwrap() {
                None
            } else if rm == RoundingMode::Exact {
                let shifted = n << u;
                if shifted >> u == n {
                    Some((shifted, u, rm))
                } else {
                    None
                }
            } else {
                Some((n, u, rm))
            }
        }),
    )
}
