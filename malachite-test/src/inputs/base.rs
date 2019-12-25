use std::char;
use std::cmp::Ordering;
use std::ops::{Shl, Shr};

use malachite_base::chars::NUMBER_OF_CHARS;
use malachite_base::limbs::limbs_test_zero;
use malachite_base::num::arithmetic::traits::{EqMod, Parity};
use malachite_base::num::basic::integers::PrimitiveInteger;
use malachite_base::num::basic::signeds::PrimitiveSigned;
use malachite_base::num::basic::traits::Zero;
use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base::num::conversion::traits::{CheckedFrom, ConvertibleFrom, WrappingFrom};
use malachite_base::num::logic::traits::BitAccess;
use malachite_base::round::RoundingMode;
use malachite_nz::integer::logic::bit_access::limbs_vec_clear_bit_neg;
use malachite_nz::integer::Integer;
use malachite_nz::natural::arithmetic::add::{
    limbs_vec_add_in_place_left, limbs_vec_add_limb_in_place,
};
use malachite_nz::natural::arithmetic::div_exact::{
    limbs_modular_invert_limb, limbs_modular_invert_scratch_len,
};
use malachite_nz::natural::arithmetic::div_mod::{
    _limbs_div_mod_barrett_is_len, _limbs_div_mod_barrett_scratch_len,
    limbs_two_limb_inverse_helper,
};
use malachite_nz::natural::arithmetic::eq_mod::{
    limbs_eq_limb_mod_limb, limbs_eq_limb_mod_ref_ref, limbs_eq_mod_limb_ref_ref,
    limbs_eq_mod_ref_ref_ref,
};
use malachite_nz::natural::arithmetic::mul::fft::*;
use malachite_nz::natural::arithmetic::mul::limb::{
    limbs_mul_limb, limbs_slice_mul_limb_in_place, limbs_vec_mul_limb_in_place,
};
use malachite_nz::natural::arithmetic::mul::limbs_mul;
use malachite_nz::natural::arithmetic::mul::mul_mod::*;
use malachite_nz::natural::arithmetic::mul::toom::{
    _limbs_mul_greater_to_out_toom_22_input_sizes_valid,
    _limbs_mul_greater_to_out_toom_32_input_sizes_valid,
    _limbs_mul_greater_to_out_toom_33_input_sizes_valid,
    _limbs_mul_greater_to_out_toom_42_input_sizes_valid,
    _limbs_mul_greater_to_out_toom_43_input_sizes_valid,
    _limbs_mul_greater_to_out_toom_44_input_sizes_valid,
    _limbs_mul_greater_to_out_toom_52_input_sizes_valid,
    _limbs_mul_greater_to_out_toom_53_input_sizes_valid,
    _limbs_mul_greater_to_out_toom_54_input_sizes_valid,
    _limbs_mul_greater_to_out_toom_62_input_sizes_valid,
    _limbs_mul_greater_to_out_toom_63_input_sizes_valid,
    _limbs_mul_greater_to_out_toom_6h_input_sizes_valid,
    _limbs_mul_greater_to_out_toom_8h_input_sizes_valid,
};
use malachite_nz::natural::arithmetic::sub::limbs_sub_in_place_left;
use malachite_nz::natural::Natural;
use malachite_nz::platform::Limb;
use rand::distributions::range::SampleRange;
use rand::Rand;
use rust_wheels::iterators::bools::exhaustive_bools;
use rust_wheels::iterators::chars::{exhaustive_ascii_chars, exhaustive_chars, random_ascii_chars};
use rust_wheels::iterators::common::EXAMPLE_SEED;
use rust_wheels::iterators::general::{random, random_from_vector, range_increasing};
use rust_wheels::iterators::integers_geometric::{
    positive_u32s_geometric, range_up_geometric_u32, u32s_geometric,
};
use rust_wheels::iterators::orderings::{exhaustive_orderings, random_orderings};
use rust_wheels::iterators::primitive_floats::{
    exhaustive_f32s, exhaustive_f64s, exhaustive_finite_f32s, exhaustive_finite_f64s,
    random_finite_primitive_floats, random_primitive_floats, special_random_f32s,
    special_random_f64s, special_random_finite_f32s, special_random_finite_f64s,
};
use rust_wheels::iterators::primitive_ints::{
    exhaustive_natural_signed, exhaustive_negative_signed, exhaustive_nonzero_signed,
    exhaustive_positive, exhaustive_signed, exhaustive_unsigned, random_natural_signed,
    random_negative_signed, random_nonzero_signed, random_positive_signed,
    random_positive_unsigned, random_range, random_range_down, random_range_up,
    range_down_increasing, range_up_increasing, special_random_natural_signed,
    special_random_negative_signed, special_random_nonzero_signed, special_random_positive_signed,
    special_random_positive_unsigned, special_random_signed, special_random_unsigned,
};
use rust_wheels::iterators::rounding_modes::{exhaustive_rounding_modes, random_rounding_modes};
use rust_wheels::iterators::strings::{
    exhaustive_strings, exhaustive_strings_with_chars, random_strings, random_strings_with_chars,
};
use rust_wheels::iterators::tuples::{
    exhaustive_pairs, exhaustive_pairs_from_single, exhaustive_quadruples, exhaustive_quintuples,
    exhaustive_triples, exhaustive_triples_from_single, lex_pairs, lex_triples, log_pairs,
    random_pairs, random_pairs_from_single, random_quadruples, random_quintuples, random_triples,
    random_triples_from_single, sqrt_pairs,
};
use rust_wheels::iterators::vecs::{
    exhaustive_vecs, exhaustive_vecs_min_length, random_vecs, random_vecs_min_length,
    special_random_bool_vecs, special_random_unsigned_vecs,
    special_random_unsigned_vecs_min_length,
};

use common::{GenerationMode, NoSpecialGenerationMode};
use inputs::common::{
    permute_1_2_4_3, permute_1_3_2, permute_2_1, reshape_1_2_to_3, reshape_2_1_to_3,
    reshape_2_2_to_4, reshape_3_1_to_4,
};

pub fn bools(gm: NoSpecialGenerationMode) -> It<bool> {
    match gm {
        NoSpecialGenerationMode::Exhaustive => Box::new(exhaustive_bools()),
        NoSpecialGenerationMode::Random(_) => Box::new(random(&EXAMPLE_SEED)),
    }
}

pub(crate) type It<T> = Box<dyn Iterator<Item = T>>;

pub fn unsigneds<T: PrimitiveUnsigned + Rand>(gm: GenerationMode) -> It<T> {
    match gm {
        GenerationMode::Exhaustive => Box::new(exhaustive_unsigned()),
        GenerationMode::Random(_) => Box::new(random(&EXAMPLE_SEED)),
        GenerationMode::SpecialRandom(_) => Box::new(special_random_unsigned(&EXAMPLE_SEED)),
    }
}

pub fn signeds<T: PrimitiveSigned + Rand>(gm: GenerationMode) -> It<T>
where
    T::UnsignedOfEqualWidth: Rand,
    T: WrappingFrom<<T as PrimitiveSigned>::UnsignedOfEqualWidth>,
{
    match gm {
        GenerationMode::Exhaustive => Box::new(exhaustive_signed()),
        GenerationMode::Random(_) => Box::new(random(&EXAMPLE_SEED)),
        GenerationMode::SpecialRandom(_) => Box::new(special_random_signed(&EXAMPLE_SEED)),
    }
}

pub fn positive_unsigneds<T: PrimitiveUnsigned + Rand>(gm: GenerationMode) -> It<T> {
    match gm {
        GenerationMode::Exhaustive => Box::new(exhaustive_positive()),
        GenerationMode::Random(_) => Box::new(random_positive_unsigned(&EXAMPLE_SEED)),
        GenerationMode::SpecialRandom(_) => {
            Box::new(special_random_positive_unsigned(&EXAMPLE_SEED))
        }
    }
}

pub fn nonzero_signeds<T: PrimitiveSigned + Rand>(gm: GenerationMode) -> It<T>
where
    T::UnsignedOfEqualWidth: Rand,
    T: WrappingFrom<<T as PrimitiveSigned>::UnsignedOfEqualWidth>,
{
    match gm {
        GenerationMode::Exhaustive => Box::new(exhaustive_nonzero_signed()),
        GenerationMode::Random(_) => Box::new(random_nonzero_signed(&EXAMPLE_SEED)),
        GenerationMode::SpecialRandom(_) => Box::new(special_random_nonzero_signed(&EXAMPLE_SEED)),
    }
}

pub fn natural_signeds<T: PrimitiveSigned + Rand>(gm: GenerationMode) -> It<T>
where
    T::UnsignedOfEqualWidth: Rand,
    T: WrappingFrom<<T as PrimitiveSigned>::UnsignedOfEqualWidth>,
{
    match gm {
        GenerationMode::Exhaustive => Box::new(exhaustive_natural_signed()),
        GenerationMode::Random(_) => Box::new(random_natural_signed(&EXAMPLE_SEED)),
        GenerationMode::SpecialRandom(_) => Box::new(special_random_natural_signed(&EXAMPLE_SEED)),
    }
}

pub fn negative_signeds<T: PrimitiveSigned + Rand>(gm: GenerationMode) -> It<T>
where
    T::UnsignedOfEqualWidth: Rand,
    T: WrappingFrom<<T as PrimitiveSigned>::UnsignedOfEqualWidth>,
{
    match gm {
        GenerationMode::Exhaustive => Box::new(exhaustive_negative_signed()),
        GenerationMode::Random(_) => Box::new(random_negative_signed(&EXAMPLE_SEED)),
        GenerationMode::SpecialRandom(_) => Box::new(special_random_negative_signed(&EXAMPLE_SEED)),
    }
}

pub fn unsigneds_no_max<T: PrimitiveUnsigned + Rand>(gm: GenerationMode) -> It<T> {
    Box::new(unsigneds(gm).filter(|&u| u != T::MAX))
}

pub fn signeds_no_max<T: PrimitiveSigned + Rand>(gm: GenerationMode) -> It<T>
where
    T::UnsignedOfEqualWidth: Rand,
    T: WrappingFrom<<T as PrimitiveSigned>::UnsignedOfEqualWidth>,
{
    Box::new(signeds(gm).filter(|&i| i != T::MAX))
}

pub fn signeds_no_min<T: PrimitiveSigned + Rand>(gm: GenerationMode) -> It<T>
where
    T::UnsignedOfEqualWidth: Rand,
    T: WrappingFrom<<T as PrimitiveSigned>::UnsignedOfEqualWidth>,
{
    Box::new(signeds(gm).filter(|&i| i != T::MIN))
}

// All `T`s, where `T` is unsigned and its most-significant bit is set.
pub fn unsigneds_var_1<T: PrimitiveUnsigned + Rand>(gm: GenerationMode) -> It<T> {
    match gm {
        GenerationMode::Exhaustive => Box::new(range_up_increasing(T::ONE << (T::WIDTH - 1))),
        GenerationMode::Random(_) => Box::new(random::<T>(&EXAMPLE_SEED).map(|mut u| {
            u.set_bit(u64::from(T::WIDTH - 1));
            u
        })),
        GenerationMode::SpecialRandom(_) => {
            Box::new(special_random_unsigned::<T>(&EXAMPLE_SEED).map(|mut u| {
                u.set_bit(u64::from(T::WIDTH - 1));
                u
            }))
        }
    }
}

macro_rules! float_gen {
    (
        $f: ident,
        $exhaustive: ident,
        $exhaustive_finite: ident,
        $special_random: ident,
        $special_random_finite: ident,
        $floats: ident,
        $finite_floats: ident,
        $floats_var_1: ident,
        $pairs_of_finite_float_and_rounding_mode: ident,
        $pairs_of_finite_float_and_rounding_mode_var_1: ident,
        $pairs_of_finite_float_and_rounding_mode_var_2: ident
    ) => {
        pub fn $floats(gm: GenerationMode) -> It<$f> {
            match gm {
                GenerationMode::Exhaustive => Box::new($exhaustive()),
                GenerationMode::Random(_) => Box::new(random_primitive_floats(&EXAMPLE_SEED)),
                GenerationMode::SpecialRandom(scale) => {
                    Box::new($special_random(&EXAMPLE_SEED, scale))
                }
            }
        }

        pub fn $finite_floats(gm: GenerationMode) -> It<$f> {
            match gm {
                GenerationMode::Exhaustive => Box::new($exhaustive_finite()),
                GenerationMode::Random(_) => {
                    Box::new(random_finite_primitive_floats(&EXAMPLE_SEED))
                }
                GenerationMode::SpecialRandom(scale) => {
                    Box::new($special_random_finite(&EXAMPLE_SEED, scale))
                }
            }
        }

        // All floats that are not NaN, not infinite, and are greater than or equal to -0.5.
        pub fn $floats_var_1(gm: GenerationMode) -> It<$f> {
            Box::new($floats(gm).filter(|&f| !f.is_nan() && !f.is_infinite() && f >= -0.5))
        }

        fn $pairs_of_finite_float_and_rounding_mode(gm: GenerationMode) -> It<($f, RoundingMode)> {
            match gm {
                GenerationMode::Exhaustive => {
                    Box::new(lex_pairs($exhaustive_finite(), exhaustive_rounding_modes()))
                }
                GenerationMode::Random(_) => Box::new(random_pairs(
                    &EXAMPLE_SEED,
                    &(|seed| random_finite_primitive_floats(seed)),
                    &(|seed| random_rounding_modes(seed)),
                )),
                GenerationMode::SpecialRandom(scale) => Box::new(random_pairs(
                    &EXAMPLE_SEED,
                    &(|seed| $special_random_finite(seed, scale)),
                    &(|seed| random_rounding_modes(seed)),
                )),
            }
        }

        // All pairs of finite float and `RoundingMode` that are acceptable to pass into
        // `Natural::rounding_from`.
        pub fn $pairs_of_finite_float_and_rounding_mode_var_1(
            gm: GenerationMode,
        ) -> It<($f, RoundingMode)> {
            Box::new(
                $pairs_of_finite_float_and_rounding_mode(gm).filter(|&(f, rm)| {
                    if rm == RoundingMode::Exact {
                        return Natural::convertible_from(f);
                    }
                    match rm {
                        RoundingMode::Floor | RoundingMode::Up => f >= 0.0,
                        RoundingMode::Down | RoundingMode::Ceiling => f > -1.0,
                        RoundingMode::Nearest => f >= -0.5,
                        _ => unreachable!(),
                    }
                }),
            )
        }

        // All pairs of finite float and `RoundingMode` that are acceptable to pass into
        // `Integer::rounding_from`.
        pub fn $pairs_of_finite_float_and_rounding_mode_var_2(
            gm: GenerationMode,
        ) -> It<($f, RoundingMode)> {
            Box::new(
                $pairs_of_finite_float_and_rounding_mode(gm).filter(|&(f, rm)| {
                    rm != RoundingMode::Exact || Natural::checked_from(f).is_some()
                }),
            )
        }
    };
}

float_gen!(
    f32,
    exhaustive_f32s,
    exhaustive_finite_f32s,
    special_random_f32s,
    special_random_finite_f32s,
    f32s,
    finite_f32s,
    f32s_var_1,
    pairs_of_finite_f32_and_rounding_mode,
    pairs_of_finite_f32_and_rounding_mode_var_1,
    pairs_of_finite_f32_and_rounding_mode_var_2
);
float_gen!(
    f64,
    exhaustive_f64s,
    exhaustive_finite_f64s,
    special_random_f64s,
    special_random_finite_f64s,
    f64s,
    finite_f64s,
    f64s_var_1,
    pairs_of_finite_f64_and_rounding_mode,
    pairs_of_finite_f64_and_rounding_mode_var_1,
    pairs_of_finite_f64_and_rounding_mode_var_2
);

fn pairs_of_unsigneds_with_seed<T: PrimitiveUnsigned + Rand>(
    gm: GenerationMode,
    seed: &[u32],
) -> It<(T, T)> {
    match gm {
        GenerationMode::Exhaustive => Box::new(exhaustive_pairs_from_single(exhaustive_unsigned())),
        GenerationMode::Random(_) => Box::new(random_pairs_from_single(random(seed))),
        GenerationMode::SpecialRandom(_) => {
            Box::new(random_pairs_from_single(special_random_unsigned(seed)))
        }
    }
}

pub fn pairs_of_unsigneds<T: PrimitiveUnsigned + Rand>(gm: GenerationMode) -> It<(T, T)> {
    pairs_of_unsigneds_with_seed(gm, &EXAMPLE_SEED)
}

//TODO use subset_pairs
// All pairs of `T`s where `T` is unsigned and the first `T` is greater than or equal to the second.
pub fn pairs_of_unsigneds_var_1<T: PrimitiveUnsigned + Rand>(gm: GenerationMode) -> It<(T, T)> {
    Box::new(pairs_of_unsigneds(gm).filter(|&(x, y)| x >= y))
}

fn pairs_of_unsigneds_var_2_with_seed<T: PrimitiveUnsigned + Rand>(
    gm: GenerationMode,
    seed: &[u32],
) -> It<(T, T)> {
    match gm {
        GenerationMode::Exhaustive => Box::new(exhaustive_pairs(
            range_up_increasing(T::ONE << (T::WIDTH - 1)),
            exhaustive_unsigned(),
        )),
        GenerationMode::Random(_) => Box::new(random_pairs_from_single(random::<T>(seed)).map(
            |(mut u, v)| {
                u.set_bit(u64::from(T::WIDTH - 1));
                (u, v)
            },
        )),
        GenerationMode::SpecialRandom(_) => Box::new(
            random_pairs_from_single(special_random_unsigned::<T>(seed)).map(|(mut u, v)| {
                u.set_bit(u64::from(T::WIDTH - 1));
                (u, v)
            }),
        ),
    }
}

// All pairs of `T`s, where `T` is unsigned and the most-significant bit of the first `T` is set.
pub fn pairs_of_unsigneds_var_2<T: PrimitiveUnsigned + Rand>(gm: GenerationMode) -> It<(T, T)> {
    pairs_of_unsigneds_var_2_with_seed(gm, &EXAMPLE_SEED)
}

pub fn pairs_of_signeds<T: PrimitiveSigned + Rand>(gm: GenerationMode) -> It<(T, T)>
where
    T::UnsignedOfEqualWidth: Rand,
    T: WrappingFrom<<T as PrimitiveSigned>::UnsignedOfEqualWidth>,
{
    match gm {
        GenerationMode::Exhaustive => Box::new(exhaustive_pairs_from_single(exhaustive_signed())),
        GenerationMode::Random(_) => Box::new(random_pairs_from_single(random(&EXAMPLE_SEED))),
        GenerationMode::SpecialRandom(_) => Box::new(random_pairs_from_single(
            special_random_signed(&EXAMPLE_SEED),
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

// All `Limb`s smaller than 2<sup>Limb::WIDTH - 1<sup>.
fn u32s_range_2(gm: GenerationMode) -> It<Limb> {
    let upper = (1 as Limb) << (Limb::WIDTH - 1);
    match gm {
        GenerationMode::Exhaustive => Box::new(range_down_increasing(upper)),
        GenerationMode::Random(_) => Box::new(random_range_down(&EXAMPLE_SEED, upper)),
        GenerationMode::SpecialRandom(_) => {
            Box::new(special_random_unsigned::<Limb>(&EXAMPLE_SEED).map(|u| {
                let mut u = u;
                u.clear_bit(u64::from(u32::WIDTH - 1));
                u
            }))
        }
    }
}

pub fn odd_limbs(gm: GenerationMode) -> It<Limb> {
    Box::new(u32s_range_2(gm).map(|u| (u << 1) + 1))
}

// All pairs of `u32`s smaller than `NUMBER_OF_CHARS`.
pub fn pairs_of_limbs_range_1(gm: NoSpecialGenerationMode) -> It<(u32, u32)> {
    match gm {
        NoSpecialGenerationMode::Exhaustive => Box::new(exhaustive_pairs_from_single(
            range_down_increasing(NUMBER_OF_CHARS - 1),
        )),
        NoSpecialGenerationMode::Random(_) => Box::new(random_pairs_from_single(
            random_range_down(&EXAMPLE_SEED, NUMBER_OF_CHARS - 1),
        )),
    }
}

pub fn small_unsigneds<T: PrimitiveUnsigned + Rand>(gm: NoSpecialGenerationMode) -> It<T> {
    match gm {
        NoSpecialGenerationMode::Exhaustive => Box::new(exhaustive_unsigned()),
        NoSpecialGenerationMode::Random(scale) => {
            Box::new(u32s_geometric(&EXAMPLE_SEED, scale).flat_map(T::checked_from))
        }
    }
}

pub fn small_positive_unsigneds<T: PrimitiveUnsigned + Rand>(gm: NoSpecialGenerationMode) -> It<T> {
    match gm {
        NoSpecialGenerationMode::Exhaustive => Box::new(exhaustive_positive()),
        NoSpecialGenerationMode::Random(scale) => {
            Box::new(positive_u32s_geometric(&EXAMPLE_SEED, scale).flat_map(T::checked_from))
        }
    }
}

fn sqrt_pairs_of_unsigneds<T: PrimitiveUnsigned, U: PrimitiveUnsigned>() -> It<(T, U)> {
    Box::new(sqrt_pairs(exhaustive_unsigned(), exhaustive_unsigned()))
}

fn random_pairs_of_primitive_and_geometric_unsigned<
    T: PrimitiveInteger + Rand,
    U: PrimitiveInteger,
>(
    scale: u32,
) -> It<(T, U)> {
    Box::new(random_pairs(
        &EXAMPLE_SEED,
        &(|seed| random(seed)),
        &(|seed| u32s_geometric(seed, scale).flat_map(U::checked_from)),
    ))
}

pub fn pairs_of_unsigned_and_small_unsigned<
    T: PrimitiveUnsigned + Rand,
    U: PrimitiveUnsigned + Rand,
>(
    gm: GenerationMode,
) -> It<(T, U)> {
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

pub fn pairs_of_small_usize_and_unsigned<T: PrimitiveUnsigned + Rand>(
    gm: GenerationMode,
) -> It<(usize, T)> {
    match gm {
        GenerationMode::Exhaustive => permute_2_1(Box::new(log_pairs(
            exhaustive_unsigned(),
            exhaustive_unsigned(),
        ))),
        GenerationMode::Random(scale) => Box::new(random_pairs(
            &EXAMPLE_SEED,
            &(|seed| u32s_geometric(seed, scale).map(usize::wrapping_from)),
            &(|seed| random(seed)),
        )),
        GenerationMode::SpecialRandom(scale) => Box::new(random_pairs(
            &EXAMPLE_SEED,
            &(|seed| u32s_geometric(seed, scale).map(usize::wrapping_from)),
            &(|seed| special_random_unsigned(seed)),
        )),
    }
}

pub(crate) fn pairs_of_small_usizes(gm: NoSpecialGenerationMode) -> It<(usize, usize)> {
    match gm {
        NoSpecialGenerationMode::Exhaustive => permute_2_1(Box::new(exhaustive_pairs_from_single(
            exhaustive_unsigned(),
        ))),
        NoSpecialGenerationMode::Random(scale) => Box::new(random_pairs_from_single(
            u32s_geometric(&EXAMPLE_SEED, scale).map(usize::wrapping_from),
        )),
    }
}

fn log_pairs_of_positive_primitive_and_unsigned<
    T: PrimitiveInteger,
    U: PrimitiveUnsigned + Rand,
>() -> It<(T, U)> {
    Box::new(log_pairs(exhaustive_positive(), exhaustive_unsigned()))
}

pub fn pairs_of_positive_unsigned_and_small_unsigned<
    T: PrimitiveUnsigned + Rand,
    U: PrimitiveUnsigned + Rand,
>(
    gm: GenerationMode,
) -> It<(T, U)> {
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

pub fn pairs_of_positive_signed_and_small_unsigned<
    T: PrimitiveSigned + Rand,
    U: PrimitiveUnsigned + Rand,
>(
    gm: GenerationMode,
) -> It<(T, U)>
where
    T::UnsignedOfEqualWidth: Rand,
    T: WrappingFrom<<T as PrimitiveSigned>::UnsignedOfEqualWidth>,
{
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

fn sqrt_pairs_of_signed_and_unsigned<T: PrimitiveSigned, U: PrimitiveUnsigned>() -> It<(T, U)> {
    Box::new(sqrt_pairs(exhaustive_signed(), exhaustive_unsigned()))
}

pub fn pairs_of_signed_and_small_unsigned<T: PrimitiveSigned + Rand, U: PrimitiveUnsigned + Rand>(
    gm: GenerationMode,
) -> It<(T, U)>
where
    T::UnsignedOfEqualWidth: Rand,
    T: WrappingFrom<<T as PrimitiveSigned>::UnsignedOfEqualWidth>,
{
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

fn exhaustive_pairs_of_unsigned_and_u64_width_range<T: PrimitiveUnsigned + Rand>() -> ItU<T> {
    Box::new(lex_pairs(
        exhaustive_unsigned(),
        range_down_increasing(u64::from(T::WIDTH) - 1),
    ))
}

fn random_pairs_of_primitive_and_u64_width_range<T: PrimitiveInteger + Rand>() -> It<(T, u64)> {
    Box::new(random_pairs(
        &EXAMPLE_SEED,
        &(|seed| random(seed)),
        &(|seed| random_range_down(seed, u64::from(T::WIDTH) - 1)),
    ))
}

// All pairs of unsigned `T` and `u64`, where the `u64` is smaller that `T::WIDTH`.
pub fn pairs_of_unsigned_and_u64_width_range<T: PrimitiveUnsigned + Rand>(
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
pub fn pairs_of_signed_and_u64_width_range<T: PrimitiveSigned + Rand>(
    gm: GenerationMode,
) -> It<(T, u64)>
where
    T::UnsignedOfEqualWidth: Rand,
    T: WrappingFrom<<T as PrimitiveSigned>::UnsignedOfEqualWidth>,
{
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
pub fn pairs_of_signed_and_u64_width_range_var_1<T: PrimitiveSigned + Rand>(
    gm: GenerationMode,
) -> It<(T, u64)>
where
    T::UnsignedOfEqualWidth: Rand,
    T: WrappingFrom<<T as PrimitiveSigned>::UnsignedOfEqualWidth>,
{
    Box::new(
        pairs_of_signed_and_small_unsigned(gm)
            .filter(|&(n, index)| n < T::ZERO || index < u64::from(T::WIDTH)),
    )
}

// All pairs of signed `T` and `u64`, where the signed `T` i s non-negative or the `u64` is smaller
// than `T::WIDTH`.
pub fn pairs_of_signed_and_u64_width_range_var_2<T: PrimitiveSigned + Rand>(
    gm: GenerationMode,
) -> It<(T, u64)>
where
    T::UnsignedOfEqualWidth: Rand,
    T: WrappingFrom<<T as PrimitiveSigned>::UnsignedOfEqualWidth>,
{
    Box::new(
        pairs_of_signed_and_small_unsigned(gm)
            .filter(|&(n, index)| n >= T::ZERO || index < u64::from(T::WIDTH)),
    )
}

// All triples of `T`, `U`, and `bool`, where `T` and `U` are unsigned and the `bool` is false or
// the `U` is smaller than `T::WIDTH`.
pub fn triples_of_unsigned_unsigned_width_range_and_bool_var_1<
    T: PrimitiveUnsigned + Rand,
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
    T: PrimitiveSigned + Rand,
    U: PrimitiveUnsigned,
>(
    gm: GenerationMode,
) -> It<(T, U, bool)>
where
    T::UnsignedOfEqualWidth: Rand,
    T: WrappingFrom<<T as PrimitiveSigned>::UnsignedOfEqualWidth>,
{
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
    T: PrimitiveSigned + Rand,
    U: PrimitiveUnsigned,
>(
    gm: GenerationMode,
) -> It<(T, U)>
where
    T::UnsignedOfEqualWidth: Rand,
    T: WrappingFrom<<T as PrimitiveSigned>::UnsignedOfEqualWidth>,
{
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

pub fn triples_of_signed_signed_and_small_unsigned<
    T: PrimitiveSigned + Rand,
    U: PrimitiveUnsigned + Rand,
>(
    gm: GenerationMode,
) -> It<(T, T, U)>
where
    T::UnsignedOfEqualWidth: Rand,
    T: WrappingFrom<<T as PrimitiveSigned>::UnsignedOfEqualWidth>,
{
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

pub fn chars(gm: NoSpecialGenerationMode) -> It<char> {
    match gm {
        NoSpecialGenerationMode::Exhaustive => Box::new(exhaustive_chars()),
        NoSpecialGenerationMode::Random(_) => Box::new(random(&EXAMPLE_SEED)),
    }
}

pub fn chars_not_min(gm: NoSpecialGenerationMode) -> It<char> {
    Box::new(chars(gm).filter(|&c| c != '\u{0}'))
}

pub fn chars_not_max(gm: NoSpecialGenerationMode) -> It<char> {
    Box::new(chars(gm).filter(|&c| c != char::MAX))
}

pub fn pairs_of_chars(gm: NoSpecialGenerationMode) -> It<(char, char)> {
    match gm {
        NoSpecialGenerationMode::Exhaustive => {
            Box::new(exhaustive_pairs_from_single(exhaustive_chars()))
        }
        NoSpecialGenerationMode::Random(_) => {
            Box::new(random_pairs_from_single(random(&EXAMPLE_SEED)))
        }
    }
}

pub fn rounding_modes(gm: NoSpecialGenerationMode) -> It<RoundingMode> {
    match gm {
        NoSpecialGenerationMode::Exhaustive => Box::new(exhaustive_rounding_modes()),
        NoSpecialGenerationMode::Random(_) => Box::new(random_rounding_modes(&EXAMPLE_SEED)),
    }
}

pub fn pairs_of_rounding_modes(gm: NoSpecialGenerationMode) -> It<(RoundingMode, RoundingMode)> {
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
) -> It<(RoundingMode, RoundingMode, RoundingMode)> {
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

type ItR<T> = It<(T, RoundingMode)>;

fn random_pairs_of_primitive_and_rounding_mode<T: PrimitiveInteger + Rand>() -> ItR<T> {
    Box::new(random_pairs(
        &EXAMPLE_SEED,
        &(|seed| random(seed)),
        &(|seed| random_rounding_modes(seed)),
    ))
}

pub fn pairs_of_unsigned_and_rounding_mode<T: PrimitiveUnsigned + Rand>(
    gm: GenerationMode,
) -> It<(T, RoundingMode)> {
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

pub fn pairs_of_signed_and_rounding_mode<T: PrimitiveSigned + Rand>(
    gm: GenerationMode,
) -> It<(T, RoundingMode)>
where
    T::UnsignedOfEqualWidth: Rand,
    T: WrappingFrom<<T as PrimitiveSigned>::UnsignedOfEqualWidth>,
{
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

// All sextuples of `Limb`s that are valid inputs to `limbs_div_mod_three_limb_by_two_limb`.
pub fn sextuples_of_limbs_var_1(gm: GenerationMode) -> It<(Limb, Limb, Limb, Limb, Limb, Limb)> {
    let quads: &dyn Fn(&[u32]) -> It<((Limb, Limb), (Limb, Limb))> = &|seed| {
        let q: It<((Limb, Limb), (Limb, Limb))> = match gm {
            GenerationMode::Exhaustive => Box::new(exhaustive_pairs(
                pairs_of_unsigneds(gm),
                pairs_of_unsigneds_var_2(gm),
            )),
            _ => Box::new(random_pairs(
                seed,
                &(|seed_2| pairs_of_unsigneds_with_seed(gm, seed_2)),
                &(|seed_2| pairs_of_unsigneds_var_2_with_seed(gm, seed_2)),
            )),
        };
        q
    };
    let filtered_quads: &dyn Fn(&[u32]) -> It<((Limb, Limb), (Limb, Limb))> = &|seed| {
        Box::new(
            quads(seed).filter(|((n_2, n_1), (d_1, d_0))| n_2 < d_1 || n_2 == d_1 && n_1 < d_0),
        )
    };
    let quints: It<(((Limb, Limb), (Limb, Limb)), Limb)> = match gm {
        GenerationMode::Exhaustive => Box::new(sqrt_pairs(
            filtered_quads(&EXAMPLE_SEED),
            exhaustive_unsigned(),
        )),
        GenerationMode::Random(_) => Box::new(random_pairs(
            &EXAMPLE_SEED,
            &(|seed| filtered_quads(seed)),
            &(|seed| random(seed)),
        )),
        GenerationMode::SpecialRandom(_) => Box::new(random_pairs(
            &EXAMPLE_SEED,
            &(|seed| filtered_quads(seed)),
            &(|seed| special_random_unsigned(seed)),
        )),
    };
    Box::new(quints.map(|(((n_2, n_1), (d_1, d_0)), n_0)| {
        (
            n_2,
            n_1,
            n_0,
            d_1,
            d_0,
            limbs_two_limb_inverse_helper(d_1, d_0),
        )
    }))
}

fn vecs_of_unsigned_with_seed<T: PrimitiveUnsigned + Rand>(
    gm: GenerationMode,
    seed: &[u32],
) -> It<Vec<T>> {
    match gm {
        GenerationMode::Exhaustive => Box::new(exhaustive_vecs(exhaustive_unsigned())),
        GenerationMode::Random(scale) => {
            Box::new(random_vecs(seed, scale, &(|seed_2| random(seed_2))))
        }
        GenerationMode::SpecialRandom(scale) => Box::new(special_random_unsigned_vecs(seed, scale)),
    }
}

pub fn vecs_of_unsigned<T: PrimitiveUnsigned + Rand>(gm: GenerationMode) -> It<Vec<T>> {
    vecs_of_unsigned_with_seed(gm, &EXAMPLE_SEED)
}

//TODO use vecs_at_least
pub fn nonempty_vecs_of_unsigned<T: PrimitiveUnsigned + Rand>(gm: GenerationMode) -> It<Vec<T>> {
    Box::new(vecs_of_unsigned(gm).filter(|xs| !xs.is_empty()))
}

// All `Vec<T>`, where `T` is unsigned, the `Vec` is nonempty, and its last `T` is nonzero.
pub fn vecs_of_unsigned_var_1<T: PrimitiveUnsigned + Rand>(gm: GenerationMode) -> It<Vec<T>> {
    Box::new(
        vecs_of_unsigned(gm).filter(|limbs| !limbs.is_empty() && *limbs.last().unwrap() != T::ZERO),
    )
}

// All `Vec<T>`, where `T` is unsigned and either the `Vec` is empty, or its last `T` is nonzero.
pub fn vecs_of_unsigned_var_2<T: PrimitiveUnsigned + Rand>(gm: GenerationMode) -> It<Vec<T>> {
    Box::new(
        vecs_of_unsigned(gm).filter(|limbs| limbs.is_empty() || *limbs.last().unwrap() != T::ZERO),
    )
}

// All `Vec<T>`, where `T` is unsigned and the `Vec` is nonempty and doesn't only contain zeros.
pub fn vecs_of_unsigned_var_3<T: PrimitiveUnsigned + Rand>(gm: GenerationMode) -> It<Vec<T>> {
    Box::new(vecs_of_unsigned(gm).filter(|limbs| !limbs_test_zero(limbs)))
}

// All `Vec<Limb>` that are nonempty and represent a `Natural` divisible by 3.
pub fn vecs_of_unsigned_var_5(gm: GenerationMode) -> It<Vec<Limb>> {
    Box::new(
        vecs_of_unsigned(gm)
            .filter(|ref limbs| !limbs.is_empty())
            .map(|limbs| limbs_mul_limb(&limbs, 3)),
    )
}

pub fn pairs_of_unsigned_vec<T: PrimitiveUnsigned + Rand>(
    gm: GenerationMode,
) -> It<(Vec<T>, Vec<T>)> {
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

fn pairs_of_unsigned_vec_var_1_with_seed<T: PrimitiveUnsigned + Rand>(
    gm: GenerationMode,
    seed: &[u32],
) -> It<(Vec<T>, Vec<T>)> {
    Box::new(
        vecs_of_unsigned_with_seed(gm, seed)
            .filter(|xs| xs.len().even())
            .map(|xs| {
                let half_length = xs.len() >> 1;
                (xs[..half_length].to_vec(), xs[half_length..].to_vec())
            }),
    )
}

// All pairs of `Vec<T>` where `T` is unsigned and the two components of the pair have the same
// length.
pub fn pairs_of_unsigned_vec_var_1<T: PrimitiveUnsigned + Rand>(
    gm: GenerationMode,
) -> It<(Vec<T>, Vec<T>)> {
    pairs_of_unsigned_vec_var_1_with_seed(gm, &EXAMPLE_SEED)
}

// All pairs of `Vec<T>`, where `T` is unsigned and the last `T`s of both `Vec`s are nonzero.
pub fn pairs_of_unsigned_vec_var_2<T: PrimitiveUnsigned + Rand>(
    gm: GenerationMode,
) -> It<(Vec<T>, Vec<T>)> {
    match gm {
        GenerationMode::Exhaustive => Box::new(exhaustive_pairs_from_single(
            exhaustive_vecs(exhaustive_unsigned())
                .filter(|limbs| limbs.is_empty() || *limbs.last().unwrap() != T::ZERO),
        )),
        _ => Box::new(random_pairs_from_single(vecs_of_unsigned_var_2(gm))),
    }
}

// All pairs of `Vec<T>`, where `T` is unsigned and first `Vec` is at least as long as the second.
pub fn pairs_of_unsigned_vec_var_3<T: PrimitiveUnsigned + Rand>(
    gm: GenerationMode,
) -> It<(Vec<T>, Vec<T>)> {
    Box::new(pairs_of_unsigned_vec(gm).filter(|&(ref xs, ref ys)| xs.len() >= ys.len()))
}

// All pairs of `Vec<T>`, where `T` is unsigned and first `Vec` is at least as long as the second,
// and neither `Vec` is empty.
pub fn pairs_of_unsigned_vec_var_4<T: PrimitiveUnsigned + Rand>(
    gm: GenerationMode,
) -> It<(Vec<T>, Vec<T>)> {
    Box::new(
        pairs_of_unsigned_vec(gm)
            .filter(|&(ref xs, ref ys)| !ys.is_empty() && xs.len() >= ys.len()),
    )
}

// All pairs of `Vec<T>`, where `T` is unsigned and neither `Vec` is empty.
pub fn pairs_of_unsigned_vec_var_5<T: PrimitiveUnsigned + Rand>(
    gm: GenerationMode,
) -> It<(Vec<T>, Vec<T>)> {
    Box::new(pairs_of_unsigned_vec(gm).filter(|&(ref xs, ref ys)| !xs.is_empty() && !ys.is_empty()))
}

// All pairs of `Vec<T>` where `T` is unsigned and both elements are nonempty and don't only contain
// zeros.
pub fn pairs_of_unsigned_vec_var_6<T: PrimitiveUnsigned + Rand>(
    gm: GenerationMode,
) -> It<(Vec<T>, Vec<T>)> {
    Box::new(
        pairs_of_unsigned_vec(gm)
            .filter(|&(ref xs, ref ys)| !limbs_test_zero(xs) && !limbs_test_zero(ys)),
    )
}

// All pairs of `Vec<T>` where `T` is unsigned, both elements are nonempty and don't only contain
// zeros, and the first element is at least as long as the second.
pub fn pairs_of_unsigned_vec_var_7<T: PrimitiveUnsigned + Rand>(
    gm: GenerationMode,
) -> It<(Vec<T>, Vec<T>)> {
    Box::new(pairs_of_unsigned_vec(gm).filter(|&(ref xs, ref ys)| {
        xs.len() >= ys.len() && !limbs_test_zero(xs) && !limbs_test_zero(ys)
    }))
}

// All pairs of `Vec<Limb>`, where the first `Vec` is at least as long as the second and the second
// `Vec` is nonempty and represents a `Natural` divisible by 3.
pub fn pairs_of_unsigned_vec_var_8(gm: GenerationMode) -> It<(Vec<Limb>, Vec<Limb>)> {
    Box::new(
        pairs_of_unsigned_vec(gm)
            .map(|(out, in_limbs)| (out, limbs_mul_limb(&in_limbs, 3)))
            .filter(|(ref out, ref in_limbs)| out.len() >= in_limbs.len() && !in_limbs.is_empty()),
    )
}

// All pairs of `Vec<Limb>`, where `ns` and `ds` meet the preconditions of `limbs_div_mod`.
pub fn pairs_of_unsigned_vec_var_9(gm: GenerationMode) -> It<(Vec<Limb>, Vec<Limb>)> {
    let qs: It<(Vec<Limb>, Vec<Limb>)> = match gm {
        GenerationMode::Exhaustive => Box::new(exhaustive_pairs_from_single(
            exhaustive_vecs_min_length(2, exhaustive_unsigned()),
        )),
        GenerationMode::Random(scale) => Box::new(random_pairs_from_single(
            random_vecs_min_length(&EXAMPLE_SEED, scale, 2, &(|seed_2| random(seed_2))),
        )),
        GenerationMode::SpecialRandom(scale) => Box::new(random_pairs_from_single(
            special_random_unsigned_vecs_min_length(&EXAMPLE_SEED, scale, 2),
        )),
    };
    Box::new(qs.filter(|(n, d)| *d.last().unwrap() != Limb::ZERO && n.len() >= d.len()))
}

// All pairs of `Vec<T>`, where `T` is unsigned and `ns` and `ds` meet the preconditions of
// `limbs_mod_by_two_limb_normalized`.
pub fn pairs_of_unsigned_vec_var_10<T: PrimitiveUnsigned + Rand>(
    gm: GenerationMode,
) -> It<(Vec<T>, Vec<T>)> {
    let ps: It<(Vec<T>, (T, T))> = match gm {
        GenerationMode::Exhaustive => Box::new(exhaustive_pairs(
            exhaustive_vecs_min_length(2, exhaustive_unsigned()),
            pairs_of_unsigneds_var_2(gm),
        )),
        GenerationMode::Random(scale) => Box::new(random_pairs(
            &EXAMPLE_SEED,
            &(|seed| random_vecs_min_length(seed, scale, 2, &(|seed_2| random(seed_2)))),
            &(|seed| pairs_of_unsigneds_var_2_with_seed(gm, seed)),
        )),
        GenerationMode::SpecialRandom(scale) => Box::new(random_pairs(
            &EXAMPLE_SEED,
            &(|seed| special_random_unsigned_vecs_min_length(seed, scale, 2)),
            &(|seed| pairs_of_unsigneds_var_2_with_seed(gm, seed)),
        )),
    };
    Box::new(ps.map(|(n, (d_1, d_0))| (n, vec![d_0, d_1])))
}

fn pairs_of_unsigned_vec_var_11_with_seed<T: PrimitiveUnsigned + Rand>(
    gm: GenerationMode,
    seed: &[u32],
) -> It<(Vec<T>, Vec<T>)> {
    Box::new(pairs_of_unsigned_vec_var_1_with_seed(gm, seed).filter(|&(ref xs, _)| !xs.is_empty()))
}

// All pairs of `Vec<T>` where `T` is unsigned and the two components of the pair have the same
// length, which is greater than zero.
fn pairs_of_unsigned_vec_var_11<T: PrimitiveUnsigned + Rand>(
    gm: GenerationMode,
) -> It<(Vec<T>, Vec<T>)> {
    Box::new(pairs_of_unsigned_vec_var_1(gm).filter(|&(ref xs, _)| !xs.is_empty()))
}

// All pairs of `Vec<T>`, where `T` is unsigned and first `Vec` is at least as long as the second,
// neither `Vec` is empty, and the first element of the second `Vec` is odd.
pub fn pairs_of_unsigned_vec_var_12<T: PrimitiveUnsigned + Rand>(
    gm: GenerationMode,
) -> It<(Vec<T>, Vec<T>)> {
    Box::new(
        pairs_of_unsigned_vec::<T>(gm)
            .filter(|&(ref xs, ref ys)| !ys.is_empty() && xs.len() >= ys.len() && ys[0].odd()),
    )
}

// All pairs of `Vec<T>`, where `T` is unsigned, neither `Vec` is empty, and the last `T`s of both
// `Vec`s are nonzero.
pub fn pairs_of_unsigned_vec_var_13<T: PrimitiveUnsigned + Rand>(
    gm: GenerationMode,
) -> It<(Vec<T>, Vec<T>)> {
    let ps: It<(Vec<T>, Vec<T>)> = match gm {
        GenerationMode::Exhaustive => Box::new(exhaustive_pairs_from_single(
            exhaustive_vecs(exhaustive_unsigned())
                .filter(|limbs| !limbs.is_empty() && *limbs.last().unwrap() != T::ZERO),
        )),
        _ => Box::new(random_pairs_from_single(vecs_of_unsigned_var_1(gm))),
    };
    Box::new(ps.filter(|(n, d)| n.len() >= d.len()))
}

// All pairs of `Vec<Limb>`, where `ns` and `ds` meet the preconditions of both `limbs_div_mod` and
// `limbs_divisible_by`.
pub fn pairs_of_unsigned_vec_var_14(gm: GenerationMode) -> It<(Vec<Limb>, Vec<Limb>)> {
    let qs: It<(Vec<Limb>, Vec<Limb>)> = match gm {
        GenerationMode::Exhaustive => Box::new(exhaustive_pairs_from_single(
            exhaustive_vecs_min_length(2, exhaustive_unsigned()),
        )),
        GenerationMode::Random(scale) => Box::new(random_pairs_from_single(
            random_vecs_min_length(&EXAMPLE_SEED, scale, 2, &(|seed_2| random(seed_2))),
        )),
        GenerationMode::SpecialRandom(scale) => Box::new(random_pairs_from_single(
            special_random_unsigned_vecs_min_length(&EXAMPLE_SEED, scale, 2),
        )),
    };
    Box::new(qs.filter(|(n, d)| {
        *n.last().unwrap() != Limb::ZERO && *d.last().unwrap() != Limb::ZERO && n.len() >= d.len()
    }))
}

// All triples of `Vec<Limb>`, where `ns` and `ds` meet the preconditions of `limbs_div_exact` and
// `limbs_divisible_by`.
pub fn pairs_of_unsigned_vec_var_15(gm: GenerationMode) -> It<(Vec<Limb>, Vec<Limb>)> {
    Box::new(
        pairs_of_unsigned_vec_min_sizes_2(gm, 1)
            .filter(|&(_, ref ds)| *ds.last().unwrap() != 0)
            .map(|(ns, ds)| {
                let mut new_ns = limbs_mul(&ns, &ds);
                if *new_ns.last().unwrap() == 0 {
                    new_ns.pop();
                }
                (new_ns, ds)
            })
            .filter(|&(ref ns, _)| *ns.last().unwrap() != 0),
    )
}

// All triples of `Vec<Limb>`, where `ns` and `ds` meet the preconditions of `limbs_div_exact`.
pub(crate) fn pairs_of_unsigned_vec_var_16(gm: GenerationMode) -> It<(Vec<Limb>, Vec<Limb>)> {
    Box::new(
        pairs_of_unsigned_vec_min_sizes_2(gm, 1)
            .filter(|&(_, ref ds)| *ds.last().unwrap() != 0)
            .map(|(ns, ds)| {
                let mut new_ns = limbs_mul(&ns, &ds);
                if *new_ns.last().unwrap() == 0 {
                    new_ns.pop();
                }
                (new_ns, ds)
            }),
    )
}

fn pairs_of_unsigned_vec_and_bool<T: PrimitiveUnsigned + Rand>(
    gm: GenerationMode,
) -> It<(Vec<T>, bool)> {
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
pub fn triples_of_two_unsigned_vecs_and_bool_var_1<T: PrimitiveUnsigned + Rand>(
    gm: GenerationMode,
) -> It<(Vec<T>, Vec<T>, bool)> {
    Box::new(
        pairs_of_unsigned_vec_and_bool(gm)
            .filter(|(xs, _)| xs.len().even())
            .map(|(xs, b)| {
                let half_length = xs.len() >> 1;
                (xs[..half_length].to_vec(), xs[half_length..].to_vec(), b)
            }),
    )
}

// All quadruples of `Vec<Limb>`, `Vec<Limb>`, and `Limb`, where `ns`, `ds`, and `inverse` meet the
// preconditions of `_limbs_mod_schoolbook`.
pub fn triples_of_two_unsigned_vecs_and_unsigned_var_1(
    gm: GenerationMode,
) -> It<(Vec<Limb>, Vec<Limb>, Limb)> {
    let ts: It<(Vec<Limb>, Vec<Limb>, Limb)> = match gm {
        GenerationMode::Exhaustive => Box::new(exhaustive_triples(
            exhaustive_vecs_min_length(3, exhaustive_unsigned()),
            exhaustive_vecs_min_length(2, exhaustive_unsigned()),
            range_up_increasing(1 << (Limb::WIDTH - 1)),
        )),
        GenerationMode::Random(scale) => Box::new(random_triples(
            &EXAMPLE_SEED,
            &(|seed| random_vecs_min_length(seed, scale, 3, &(|seed_2| random(seed_2)))),
            &(|seed| random_vecs_min_length(seed, scale, 2, &(|seed_2| random(seed_2)))),
            &(|seed| random_range_up(seed, 1 << (Limb::WIDTH - 1))),
        )),
        GenerationMode::SpecialRandom(scale) => Box::new(random_triples(
            &EXAMPLE_SEED,
            &(|seed| special_random_unsigned_vecs_min_length(seed, scale, 3)),
            &(|seed| special_random_unsigned_vecs_min_length(seed, scale, 2)),
            &(|seed| random_range_up(seed, 1 << (Limb::WIDTH - 1))),
        )),
    };
    Box::new(ts.filter(|(n, d_init, _)| n.len() >= d_init.len() + 1).map(
        |(n, mut d_init, d_last)| {
            d_init.push(d_last);
            let inverse =
                limbs_two_limb_inverse_helper(d_init[d_init.len() - 1], d_init[d_init.len() - 2]);
            (n, d_init, inverse)
        },
    ))
}

fn triples_of_unsigned_vec<T: PrimitiveUnsigned + Rand>(
    gm: GenerationMode,
) -> It<(Vec<T>, Vec<T>, Vec<T>)> {
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
pub fn triples_of_unsigned_vec_var_1<T: PrimitiveUnsigned + Rand>(
    gm: GenerationMode,
) -> It<(Vec<T>, Vec<T>, Vec<T>)> {
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

// All triples of `Vec<T>`, where `T` is unsigned and each `Vec` is either empty or the last `T` is
// nonzero.
pub fn triples_of_unsigned_vec_var_2<T: PrimitiveUnsigned + Rand>(
    gm: GenerationMode,
) -> It<(Vec<T>, Vec<T>, Vec<T>)> {
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
pub fn triples_of_unsigned_vec_var_3<T: PrimitiveUnsigned + Rand>(
    gm: GenerationMode,
) -> It<(Vec<T>, Vec<T>, Vec<T>)> {
    let ts: It<((Vec<T>, Vec<T>), Vec<T>)> = match gm {
        GenerationMode::Exhaustive => Box::new(sqrt_pairs(
            pairs_of_unsigned_vec_var_1(gm),
            exhaustive_vecs(exhaustive_unsigned()),
        )),
        GenerationMode::Random(scale) => Box::new(random_pairs(
            &EXAMPLE_SEED,
            &(|seed| pairs_of_unsigned_vec_var_1_with_seed(gm, seed)),
            &(|seed| random_vecs(seed, scale, &(|seed_2| random(seed_2)))),
        )),
        GenerationMode::SpecialRandom(scale) => Box::new(random_pairs(
            &EXAMPLE_SEED,
            &(|seed| pairs_of_unsigned_vec_var_1_with_seed(gm, seed)),
            &(|seed| special_random_unsigned_vecs(seed, scale)),
        )),
    };
    reshape_1_2_to_3(permute_2_1(Box::new(
        ts.filter(|&((ref xs, _), ref out)| out.len() >= xs.len()),
    )))
}

// All triples of `Vec<T>`, where `T` is unsigned and the first `Vec` is at least as long as each of
// the others.
pub fn triples_of_unsigned_vec_var_4<T: PrimitiveUnsigned + Rand>(
    gm: GenerationMode,
) -> It<(Vec<T>, Vec<T>, Vec<T>)> {
    Box::new(
        triples_of_unsigned_vec(gm)
            .filter(|&(ref xs, ref ys, ref zs)| xs.len() >= ys.len() && xs.len() >= zs.len()),
    )
}

// All triples of `Vec<Limb>` where the second and third elements are nonempty and don't only
// contain zeros, and the first is at least as long as the second.
pub fn triples_of_limb_vec_var_5(gm: GenerationMode) -> It<(Vec<Limb>, Vec<Limb>, Vec<Limb>)> {
    Box::new(
        triples_of_unsigned_vec(gm).filter(|&(ref xs, ref ys, ref zs)| {
            xs.len() >= ys.len() && !limbs_test_zero(ys) && !limbs_test_zero(zs)
        }),
    )
}

// All triples of `Vec<Limb>` where the second and third elements are nonempty and don't only
// contain zeros, and the first is at least as long as the third.
pub fn triples_of_limb_vec_var_6(gm: GenerationMode) -> It<(Vec<Limb>, Vec<Limb>, Vec<Limb>)> {
    Box::new(
        triples_of_unsigned_vec(gm).filter(|&(ref xs, ref ys, ref zs)| {
            xs.len() >= zs.len() && !limbs_test_zero(ys) && !limbs_test_zero(zs)
        }),
    )
}

// All triples of `Vec<Limb>` where the second and third elements are nonempty and don't only
// contain zeros, and the first is at least as long as the second and at least as long as the third.
pub fn triples_of_limb_vec_var_7(gm: GenerationMode) -> It<(Vec<Limb>, Vec<Limb>, Vec<Limb>)> {
    Box::new(
        triples_of_unsigned_vec(gm).filter(|&(ref xs, ref ys, ref zs)| {
            xs.len() >= ys.len()
                && xs.len() >= zs.len()
                && !limbs_test_zero(ys)
                && !limbs_test_zero(zs)
        }),
    )
}

// All triples of `Vec<Limb>` where the second and third elements are nonempty and don't only
// contain zeros, and the first is at least as long as the second or at least as long as the third.
pub fn triples_of_limb_vec_var_8(gm: GenerationMode) -> It<(Vec<Limb>, Vec<Limb>, Vec<Limb>)> {
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
pub fn triples_of_unsigned_vec_var_9<T: PrimitiveUnsigned + Rand>(
    gm: GenerationMode,
) -> It<(Vec<T>, Vec<T>, Vec<T>)> {
    Box::new(
        triples_of_unsigned_vec(gm)
            .filter(|&(ref xs, ref ys, ref zs)| xs.len() >= ys.len() && ys.len() >= zs.len()),
    )
}

// All triples of `Vec<T>`, where `T` is unsigned, the first `Vec` is at least as long as the sum of
// the lengths of the second and the third, the second is at least as long as the third, and the
// third is nonempty.
pub fn triples_of_unsigned_vec_var_10<T: PrimitiveUnsigned + Rand>(
    gm: GenerationMode,
) -> It<(Vec<T>, Vec<T>, Vec<T>)> {
    Box::new(
        triples_of_unsigned_vec(gm).filter(|&(ref xs, ref ys, ref zs)| {
            !zs.is_empty() && ys.len() >= zs.len() && xs.len() >= ys.len() + zs.len()
        }),
    )
}

// All triples of `Vec<T>`, where `T` is unsigned and `out`, `xs`, and `ys` meet the
// preconditions of `_limbs_mul_greater_to_out_toom_22`.
pub fn triples_of_unsigned_vec_var_11<T: PrimitiveUnsigned + Rand>(
    gm: GenerationMode,
) -> It<(Vec<T>, Vec<T>, Vec<T>)> {
    Box::new(
        triples_of_unsigned_vec_min_sizes(gm, 3, 3, 1).filter(|&(ref out, ref xs, ref ys)| {
            out.len() >= xs.len() + ys.len()
                && _limbs_mul_greater_to_out_toom_22_input_sizes_valid(xs.len(), ys.len())
        }),
    )
}

// All triples of `Vec<T>`, where `T` is unsigned and `out`, `xs`, and `ys` meet the
// preconditions of `_limbs_mul_greater_to_out_toom_32`.
pub fn triples_of_unsigned_vec_var_12<T: PrimitiveUnsigned + Rand>(
    gm: GenerationMode,
) -> It<(Vec<T>, Vec<T>, Vec<T>)> {
    Box::new(triples_of_unsigned_vec_min_sizes(gm, 10, 6, 4).filter(
        |&(ref out, ref xs, ref ys)| {
            out.len() >= xs.len() + ys.len()
                && _limbs_mul_greater_to_out_toom_32_input_sizes_valid(xs.len(), ys.len())
        },
    ))
}

// All triples of `Vec<T>`, where `T` is unsigned and `out`, `xs`, and `ys` meet the
// preconditions of `_limbs_mul_greater_to_out_toom_33`.
pub fn triples_of_unsigned_vec_var_13<T: PrimitiveUnsigned + Rand>(
    gm: GenerationMode,
) -> It<(Vec<T>, Vec<T>, Vec<T>)> {
    Box::new(triples_of_unsigned_vec_min_sizes(gm, 10, 5, 5).filter(
        |&(ref out, ref xs, ref ys)| {
            out.len() >= xs.len() + ys.len()
                && _limbs_mul_greater_to_out_toom_33_input_sizes_valid(xs.len(), ys.len())
        },
    ))
}

// All triples of `Vec<T>`, where `T` is unsigned and `out`, `xs`, and `ys` meet the
// preconditions of `_limbs_mul_greater_to_out_toom_42`.
pub fn triples_of_unsigned_vec_var_14<T: PrimitiveUnsigned + Rand>(
    gm: GenerationMode,
) -> It<(Vec<T>, Vec<T>, Vec<T>)> {
    Box::new(
        triples_of_unsigned_vec_min_sizes(gm, 6, 4, 2).filter(|&(ref out, ref xs, ref ys)| {
            out.len() >= xs.len() + ys.len()
                && _limbs_mul_greater_to_out_toom_42_input_sizes_valid(xs.len(), ys.len())
        }),
    )
}

// All triples of `Vec<T>`, where `T` is unsigned and `out`, `xs`, and `ys` meet the
// preconditions of `_limbs_mul_greater_to_out_toom_43`.
pub fn triples_of_unsigned_vec_var_15<T: PrimitiveUnsigned + Rand>(
    gm: GenerationMode,
) -> It<(Vec<T>, Vec<T>, Vec<T>)> {
    Box::new(triples_of_unsigned_vec_min_sizes(gm, 19, 11, 8).filter(
        |&(ref out, ref xs, ref ys)| {
            out.len() >= xs.len() + ys.len()
                && _limbs_mul_greater_to_out_toom_43_input_sizes_valid(xs.len(), ys.len())
        },
    ))
}

// All triples of `Vec<T>`, where `T` is unsigned and `out`, `xs`, and `ys` meet the
// preconditions of `_limbs_mul_greater_to_out_toom_44`.
pub fn triples_of_unsigned_vec_var_16<T: PrimitiveUnsigned + Rand>(
    gm: GenerationMode,
) -> It<(Vec<T>, Vec<T>, Vec<T>)> {
    Box::new(
        triples_of_unsigned_vec_min_sizes(gm, 8, 4, 4).filter(|&(ref out, ref xs, ref ys)| {
            out.len() >= xs.len() + ys.len()
                && _limbs_mul_greater_to_out_toom_44_input_sizes_valid(xs.len(), ys.len())
        }),
    )
}

// All triples of `Vec<T>`, where `T` is unsigned and `out`, `xs`, and `ys` meet the
// preconditions of `_limbs_mul_greater_to_out_toom_52`.
pub fn triples_of_unsigned_vec_var_17<T: PrimitiveUnsigned + Rand>(
    gm: GenerationMode,
) -> It<(Vec<T>, Vec<T>, Vec<T>)> {
    Box::new(triples_of_unsigned_vec_min_sizes(gm, 19, 14, 5).filter(
        |&(ref out, ref xs, ref ys)| {
            out.len() >= xs.len() + ys.len()
                && _limbs_mul_greater_to_out_toom_52_input_sizes_valid(xs.len(), ys.len())
        },
    ))
}

// All triples of `Vec<T>`, where `T` is unsigned and `out`, `xs`, and `ys` meet the
// preconditions of `_limbs_mul_greater_to_out_toom_53`.
pub fn triples_of_unsigned_vec_var_18<T: PrimitiveUnsigned + Rand>(
    gm: GenerationMode,
) -> It<(Vec<T>, Vec<T>, Vec<T>)> {
    Box::new(
        triples_of_unsigned_vec_min_sizes(gm, 8, 5, 3).filter(|&(ref out, ref xs, ref ys)| {
            out.len() >= xs.len() + ys.len()
                && _limbs_mul_greater_to_out_toom_53_input_sizes_valid(xs.len(), ys.len())
        }),
    )
}

// All triples of `Vec<T>`, where `T` is unsigned and `out`, `xs`, and `ys` meet the
// preconditions of `_limbs_mul_greater_to_out_toom_54`.
pub fn triples_of_unsigned_vec_var_19<T: PrimitiveUnsigned + Rand>(
    gm: GenerationMode,
) -> It<(Vec<T>, Vec<T>, Vec<T>)> {
    Box::new(triples_of_unsigned_vec_min_sizes(gm, 25, 14, 11).filter(
        |&(ref out, ref xs, ref ys)| {
            out.len() >= xs.len() + ys.len()
                && _limbs_mul_greater_to_out_toom_54_input_sizes_valid(xs.len(), ys.len())
        },
    ))
}

// All triples of `Vec<T>`, where `T` is unsigned and `out`, `xs`, and `ys` meet the
// preconditions of `_limbs_mul_greater_to_out_toom_62`.
pub fn triples_of_unsigned_vec_var_20<T: PrimitiveUnsigned + Rand>(
    gm: GenerationMode,
) -> It<(Vec<T>, Vec<T>, Vec<T>)> {
    Box::new(
        triples_of_unsigned_vec_min_sizes(gm, 8, 6, 2).filter(|&(ref out, ref xs, ref ys)| {
            out.len() >= xs.len() + ys.len()
                && _limbs_mul_greater_to_out_toom_62_input_sizes_valid(xs.len(), ys.len())
        }),
    )
}

// All triples of `Vec<T>`, where `T` is unsigned and `out`, `xs`, and `ys` meet the
// preconditions of `_limbs_mul_greater_to_out_toom_63`.
pub fn triples_of_unsigned_vec_var_21<T: PrimitiveUnsigned + Rand>(
    gm: GenerationMode,
) -> It<(Vec<T>, Vec<T>, Vec<T>)> {
    Box::new(triples_of_unsigned_vec_min_sizes(gm, 26, 17, 9).filter(
        |&(ref out, ref xs, ref ys)| {
            out.len() >= xs.len() + ys.len()
                && _limbs_mul_greater_to_out_toom_63_input_sizes_valid(xs.len(), ys.len())
        },
    ))
}

// All triples of `Vec<T>`, where `T` is unsigned and `out`, `xs`, and `ys` meet the
// preconditions of `_limbs_mul_greater_to_out_toom_6h`.
pub fn triples_of_unsigned_vec_var_22<T: PrimitiveUnsigned + Rand>(
    gm: GenerationMode,
) -> It<(Vec<T>, Vec<T>, Vec<T>)> {
    Box::new(triples_of_unsigned_vec_min_sizes(gm, 84, 42, 42).filter(
        |&(ref out, ref xs, ref ys)| {
            out.len() >= xs.len() + ys.len()
                && _limbs_mul_greater_to_out_toom_6h_input_sizes_valid(xs.len(), ys.len())
        },
    ))
}

// All triples of `Vec<T>`, where `T` is unsigned and `out`, `xs`, and `ys` meet the
// preconditions of `_limbs_mul_greater_to_out_toom_8h`.
pub fn triples_of_unsigned_vec_var_23<T: PrimitiveUnsigned + Rand>(
    gm: GenerationMode,
) -> It<(Vec<T>, Vec<T>, Vec<T>)> {
    Box::new(triples_of_unsigned_vec_min_sizes(gm, 172, 86, 86).filter(
        |&(ref out, ref xs, ref ys)| {
            out.len() >= xs.len() + ys.len()
                && _limbs_mul_greater_to_out_toom_8h_input_sizes_valid(xs.len(), ys.len())
        },
    ))
}

// Some triples of `Vec<T>`, where `T` is unsigned and `out`, `xs`, and `ys` would trigger the
// actual FFT code of `_limbs_mul_greater_to_out_fft`.
pub fn triples_of_unsigned_vec_var_24<T: PrimitiveUnsigned + Rand>(
    gm: GenerationMode,
) -> It<(Vec<T>, Vec<T>, Vec<T>)> {
    Box::new(triples_of_unsigned_vec_min_sizes(gm, 30, 15, 15).filter(
        |&(ref out, ref xs, ref ys)| {
            out.len() >= xs.len() + ys.len()
                && _limbs_mul_greater_to_out_fft_input_sizes_threshold(xs.len(), ys.len())
        },
    ))
}

// All triples of `Vec<T>`, where `T` is unsigned, `xs` and `ys` are nonempty and have the same
// lengths, and `out` is at least twice as long as `xs`.
pub fn triples_of_unsigned_vec_var_25<T: PrimitiveUnsigned + Rand>(
    gm: GenerationMode,
) -> It<(Vec<T>, Vec<T>, Vec<T>)> {
    let xs: It<(Vec<T>, (Vec<T>, Vec<T>))> = match gm {
        GenerationMode::Exhaustive => Box::new(sqrt_pairs(
            exhaustive_vecs_min_length(2, exhaustive_unsigned()),
            pairs_of_unsigned_vec_var_1(gm),
        )),
        GenerationMode::Random(scale) => Box::new(random_pairs(
            &EXAMPLE_SEED,
            &(|seed| random_vecs_min_length(seed, scale, 2, &(|seed_2| random(seed_2)))),
            &(|seed| pairs_of_unsigned_vec_var_1_with_seed(gm, seed)),
        )),
        GenerationMode::SpecialRandom(scale) => Box::new(random_pairs(
            &EXAMPLE_SEED,
            &(|seed| special_random_unsigned_vecs_min_length(seed, scale, 2)),
            &(|seed| pairs_of_unsigned_vec_var_1_with_seed(gm, seed)),
        )),
    };
    reshape_1_2_to_3(Box::new(xs.filter(|&(ref out, (ref xs, _))| {
        !xs.is_empty() && out.len() >= xs.len() << 1
    })))
}

// All triples of `Vec<T>`, where `T` is unsigned, `xs` and `ys` are nonempty, and `out.len()` is at
// least `xs.len() + ys.len()`.
pub fn triples_of_unsigned_vec_var_26<T: PrimitiveUnsigned + Rand>(
    gm: GenerationMode,
) -> It<(Vec<T>, Vec<T>, Vec<T>)> {
    Box::new(
        triples_of_unsigned_vec_min_sizes(gm, 2, 1, 1)
            .filter(|&(ref out, ref xs, ref ys)| out.len() >= xs.len() + ys.len()),
    )
}

// All triples of `Vec<T>`, where `T` is unsigned, `xs` and `ys` are nonempty, `zs` has at least two
// elements, and no slice has trailing zeros.
pub fn triples_of_unsigned_vec_var_27<T: PrimitiveUnsigned + Rand>(
    gm: GenerationMode,
) -> It<(Vec<T>, Vec<T>, Vec<T>)> {
    Box::new(
        triples_of_unsigned_vec_min_sizes(gm, 1, 1, 2).filter(|&(ref xs, ref ys, ref zs)| {
            *xs.last().unwrap() != T::ZERO
                && *ys.last().unwrap() != T::ZERO
                && *zs.last().unwrap() != T::ZERO
        }),
    )
}

// All triples of `Vec<T>`, where `T` is unsigned, `ys` and `zs` have at least two elements, `xs`
// has at least `ys.len() + zs.len() - 1` elements, and no slice has trailing zeros.
pub fn triples_of_unsigned_vec_var_28<T: PrimitiveUnsigned + Rand>(
    gm: GenerationMode,
) -> It<(Vec<T>, Vec<T>, Vec<T>)> {
    Box::new(
        triples_of_unsigned_vec_min_sizes(gm, 3, 2, 2).filter(|&(ref xs, ref ys, ref zs)| {
            xs.len() >= ys.len() + zs.len() - 1
                && *xs.last().unwrap() != T::ZERO
                && *ys.last().unwrap() != T::ZERO
                && *zs.last().unwrap() != T::ZERO
        }),
    )
}

// All triples of `Vec<T>`, where `T` is unsigned, none of the `Vec`s are empty, and the last `T`s
// of all `Vec`s are nonzero.
pub fn triples_of_unsigned_vec_var_29<T: PrimitiveUnsigned + Rand>(
    gm: GenerationMode,
) -> It<(Vec<T>, Vec<T>, Vec<T>)> {
    match gm {
        GenerationMode::Exhaustive => Box::new(exhaustive_triples_from_single(
            exhaustive_vecs(exhaustive_unsigned())
                .filter(|limbs| !limbs.is_empty() && *limbs.last().unwrap() != T::ZERO),
        )),
        _ => Box::new(random_triples_from_single(vecs_of_unsigned_var_1(gm))),
    }
}

// All triples of `Vec<T>`, where `T` is unsigned, `xs` and `ys` have the same length, and `out`,
// `xs`, and `ys` meet the preconditions of `_limbs_mul_greater_to_out_toom_33`.
pub(crate) fn triples_of_unsigned_vec_var_30<T: PrimitiveUnsigned + Rand>(
    gm: GenerationMode,
) -> It<(Vec<T>, Vec<T>, Vec<T>)> {
    Box::new(triples_of_unsigned_vec_min_sizes_1_2(gm, 10, 5).filter(
        |&(ref out, ref xs, ref ys)| {
            out.len() >= xs.len() + ys.len()
                && _limbs_mul_greater_to_out_toom_33_input_sizes_valid(xs.len(), ys.len())
        },
    ))
}

// All triples of `Vec<T>`, where `T` is unsigned, `xs` and `ys` have the same length, and `out`,
// `xs`, and `ys` meet the preconditions of `_limbs_mul_greater_to_out_toom_33` and
// `_limbs_mul_greater_to_out_toom_44`.
pub(crate) fn triples_of_unsigned_vec_var_31<T: PrimitiveUnsigned + Rand>(
    gm: GenerationMode,
) -> It<(Vec<T>, Vec<T>, Vec<T>)> {
    Box::new(triples_of_unsigned_vec_min_sizes_1_2(gm, 10, 5).filter(
        |&(ref out, ref xs, ref ys)| {
            out.len() >= xs.len() + ys.len()
                && _limbs_mul_greater_to_out_toom_33_input_sizes_valid(xs.len(), ys.len())
                && _limbs_mul_greater_to_out_toom_44_input_sizes_valid(xs.len(), ys.len())
        },
    ))
}

// All triples of `Vec<T>`, where `T` is unsigned, `xs` and `ys` have the same length, and `out`,
// `xs`, and `ys` meet the preconditions of `_limbs_mul_greater_to_out_toom_6h`.
pub(crate) fn triples_of_unsigned_vec_var_32<T: PrimitiveUnsigned + Rand>(
    gm: GenerationMode,
) -> It<(Vec<T>, Vec<T>, Vec<T>)> {
    Box::new(triples_of_unsigned_vec_min_sizes_1_2(gm, 84, 42).filter(
        |&(ref out, ref xs, ref ys)| {
            out.len() >= xs.len() + ys.len()
                && _limbs_mul_greater_to_out_toom_6h_input_sizes_valid(xs.len(), ys.len())
        },
    ))
}

// All triples of `Vec<T>`, where `T` is unsigned, `xs` and `ys` have the same length, and `out`,
// `xs`, and `ys` meet the preconditions of `_limbs_mul_greater_to_out_toom_8h`.
pub(crate) fn triples_of_unsigned_vec_var_33<T: PrimitiveUnsigned + Rand>(
    gm: GenerationMode,
) -> It<(Vec<T>, Vec<T>, Vec<T>)> {
    Box::new(triples_of_unsigned_vec_min_sizes_1_2(gm, 172, 86).filter(
        |&(ref out, ref xs, ref ys)| {
            out.len() >= xs.len() + ys.len()
                && _limbs_mul_greater_to_out_toom_8h_input_sizes_valid(xs.len(), ys.len())
        },
    ))
}

// Some triples of `Vec<T>`, where `T` is unsigned, `xs` and `ys` have the same length, `out`, `xs`,
// and `ys` meet the preconditions of `_limbs_mul_greater_to_out_toom_8h`, and `out`, `xs`, and `ys`
// would trigger the actual FFT code of `_limbs_mul_greater_to_out_fft`.
pub(crate) fn triples_of_unsigned_vec_var_34<T: PrimitiveUnsigned + Rand>(
    gm: GenerationMode,
) -> It<(Vec<T>, Vec<T>, Vec<T>)> {
    Box::new(triples_of_unsigned_vec_min_sizes_1_2(gm, 172, 86).filter(
        |&(ref out, ref xs, ref ys)| {
            out.len() >= xs.len() + ys.len()
                && _limbs_mul_greater_to_out_toom_8h_input_sizes_valid(xs.len(), ys.len())
                && _limbs_mul_greater_to_out_fft_input_sizes_threshold(xs.len(), ys.len())
        },
    ))
}

// All triples of `Vec<T>`, where `T` is unsigned and `out`, `xs`, and `ys` meet the preconditions
// of `_limbs_mul_greater_to_out_toom_32` and `_limbs_mul_greater_to_out_toom_43`.
pub(crate) fn triples_of_unsigned_vec_var_35<T: PrimitiveUnsigned + Rand>(
    gm: GenerationMode,
) -> It<(Vec<T>, Vec<T>, Vec<T>)> {
    Box::new(triples_of_unsigned_vec_min_sizes(gm, 19, 11, 8).filter(
        |&(ref out, ref xs, ref ys)| {
            out.len() >= xs.len() + ys.len()
                && _limbs_mul_greater_to_out_toom_32_input_sizes_valid(xs.len(), ys.len())
                && _limbs_mul_greater_to_out_toom_43_input_sizes_valid(xs.len(), ys.len())
        },
    ))
}

// All triples of `Vec<T>`, where `T` is unsigned and `out`, `xs`, and `ys` meet the preconditions
// of `_limbs_mul_greater_to_out_toom_42` and `_limbs_mul_greater_to_out_toom_53`.
pub(crate) fn triples_of_unsigned_vec_var_36<T: PrimitiveUnsigned + Rand>(
    gm: GenerationMode,
) -> It<(Vec<T>, Vec<T>, Vec<T>)> {
    Box::new(
        triples_of_unsigned_vec_min_sizes(gm, 8, 5, 3).filter(|&(ref out, ref xs, ref ys)| {
            out.len() >= xs.len() + ys.len()
                && _limbs_mul_greater_to_out_toom_42_input_sizes_valid(xs.len(), ys.len())
                && _limbs_mul_greater_to_out_toom_53_input_sizes_valid(xs.len(), ys.len())
        }),
    )
}

// All triples of `Vec<T>`, where `T` is unsigned and `qs`, `ns`, and `ds` meet the preconditions of
// `limbs_div_mod_by_two_limb_normalized`.
pub fn triples_of_unsigned_vec_var_37<T: PrimitiveUnsigned + Rand>(
    gm: GenerationMode,
) -> It<(Vec<T>, Vec<T>, Vec<T>)> {
    let ts: It<(Vec<T>, Vec<T>, (T, T))> = match gm {
        GenerationMode::Exhaustive => Box::new(exhaustive_triples(
            exhaustive_vecs(exhaustive_unsigned()),
            exhaustive_vecs_min_length(2, exhaustive_unsigned()),
            pairs_of_unsigneds_var_2(gm),
        )),
        GenerationMode::Random(scale) => Box::new(random_triples(
            &EXAMPLE_SEED,
            &(|seed| random_vecs(seed, scale, &(|seed_2| random(seed_2)))),
            &(|seed| random_vecs_min_length(seed, scale, 2, &(|seed_2| random(seed_2)))),
            &(|seed| pairs_of_unsigneds_var_2_with_seed(gm, seed)),
        )),
        GenerationMode::SpecialRandom(scale) => Box::new(random_triples(
            &EXAMPLE_SEED,
            &(|seed| special_random_unsigned_vecs(seed, scale)),
            &(|seed| special_random_unsigned_vecs_min_length(seed, scale, 2)),
            &(|seed| pairs_of_unsigneds_var_2_with_seed(gm, seed)),
        )),
    };
    Box::new(
        ts.filter(|(q, n, _)| q.len() >= n.len() - 2)
            .map(|(q, n, (d_1, d_0))| (q, n, vec![d_0, d_1])),
    )
}

// All triples of `Vec<T>`, where `T` is unsigned, the first `Vec` is at least as long as the
// second, the third is at least as long as twice the length of the second, and the second is
// nonempty and its most significant bit is set.
pub fn triples_of_unsigned_vec_var_38(gm: GenerationMode) -> It<(Vec<Limb>, Vec<Limb>, Vec<Limb>)> {
    let qs: It<(Vec<Limb>, Vec<Limb>, Vec<Limb>, Limb)> = match gm {
        GenerationMode::Exhaustive => Box::new(exhaustive_quadruples(
            exhaustive_vecs(exhaustive_unsigned()),
            exhaustive_vecs_min_length(2, exhaustive_unsigned()),
            exhaustive_vecs(exhaustive_unsigned()),
            range_up_increasing(1 << (Limb::WIDTH - 1)),
        )),
        GenerationMode::Random(scale) => Box::new(random_quadruples(
            &EXAMPLE_SEED,
            &(|seed| random_vecs(seed, scale, &(|seed_2| random(seed_2)))),
            &(|seed| random_vecs_min_length(seed, scale, 2, &(|seed_2| random(seed_2)))),
            &(|seed| random_vecs(seed, scale, &(|seed_2| random(seed_2)))),
            &(|seed| random_range_up(seed, 1 << (Limb::WIDTH - 1))),
        )),
        GenerationMode::SpecialRandom(scale) => Box::new(random_quadruples(
            &EXAMPLE_SEED,
            &(|seed| special_random_unsigned_vecs(seed, scale)),
            &(|seed| special_random_unsigned_vecs_min_length(seed, scale, 2)),
            &(|seed| special_random_unsigned_vecs(seed, scale)),
            &(|seed| random_range_up(seed, 1 << (Limb::WIDTH - 1))),
        )),
    };
    Box::new(
        qs.filter(|&(ref j, ref scratch, ref d_init, _)| {
            let ds_len = d_init.len() + 1;
            j.len() >= ds_len && scratch.len() >= ds_len << 1
        })
        .map(|(i, scratch, mut d_init, d_last)| {
            d_init.push(d_last);
            (i, d_init, scratch)
        }),
    )
}

// All triples of `Vec<T>`, where `T` is unsigned, the first `Vec` is at least as long as the
// second, the third is at least as long as twice the length of the second, and the second has
// length at least 5 and its most significant bit is set.
pub fn triples_of_unsigned_vec_var_39(gm: GenerationMode) -> It<(Vec<Limb>, Vec<Limb>, Vec<Limb>)> {
    let qs: It<(Vec<Limb>, Vec<Limb>, Vec<Limb>, Limb)> = match gm {
        GenerationMode::Exhaustive => Box::new(exhaustive_quadruples(
            exhaustive_vecs(exhaustive_unsigned()),
            exhaustive_vecs_min_length(10, exhaustive_unsigned()),
            exhaustive_vecs_min_length(4, exhaustive_unsigned()),
            range_up_increasing(1 << (Limb::WIDTH - 1)),
        )),
        GenerationMode::Random(scale) => Box::new(random_quadruples(
            &EXAMPLE_SEED,
            &(|seed| random_vecs(seed, scale, &(|seed_2| random(seed_2)))),
            &(|seed| random_vecs_min_length(seed, scale, 10, &(|seed_2| random(seed_2)))),
            &(|seed| random_vecs_min_length(seed, scale, 4, &(|seed_2| random(seed_2)))),
            &(|seed| random_range_up(seed, 1 << (Limb::WIDTH - 1))),
        )),
        GenerationMode::SpecialRandom(scale) => Box::new(random_quadruples(
            &EXAMPLE_SEED,
            &(|seed| special_random_unsigned_vecs(seed, scale)),
            &(|seed| special_random_unsigned_vecs_min_length(seed, scale, 10)),
            &(|seed| special_random_unsigned_vecs_min_length(seed, scale, 4)),
            &(|seed| random_range_up(seed, 1 << (Limb::WIDTH - 1))),
        )),
    };
    Box::new(
        qs.filter(|&(ref j, ref scratch, ref d_init, _)| {
            let ds_len = d_init.len() + 1;
            j.len() >= ds_len && scratch.len() >= ds_len << 1
        })
        .map(|(i, scratch, mut d_init, d_last)| {
            d_init.push(d_last);
            (i, d_init, scratch)
        }),
    )
}

// All triples of `Vec<Limb>`, `Vec<Limb>`, `Vec<Limb>`,`qs`, `ns`, and `ds`, meet certain
// preconditions that enable comparing the performance of divide-and-conquer division and Barrett
// division.
pub(crate) fn triples_of_unsigned_vec_var_40(
    gm: GenerationMode,
) -> It<(Vec<Limb>, Vec<Limb>, Vec<Limb>)> {
    let qs: It<(Vec<Limb>, Vec<Limb>, Vec<Limb>, Limb)> = match gm {
        GenerationMode::Exhaustive => Box::new(exhaustive_quadruples(
            exhaustive_vecs_min_length(3, exhaustive_unsigned()),
            exhaustive_vecs_min_length(9, exhaustive_unsigned()),
            exhaustive_vecs_min_length(5, exhaustive_unsigned()),
            range_up_increasing(1 << (Limb::WIDTH - 1)),
        )),
        GenerationMode::Random(scale) => Box::new(random_quadruples(
            &EXAMPLE_SEED,
            &(|seed| random_vecs_min_length(seed, scale, 3, &(|seed_2| random(seed_2)))),
            &(|seed| random_vecs_min_length(seed, scale, 9, &(|seed_2| random(seed_2)))),
            &(|seed| random_vecs_min_length(seed, scale, 5, &(|seed_2| random(seed_2)))),
            &(|seed| random_range_up(seed, 1 << (Limb::WIDTH - 1))),
        )),
        GenerationMode::SpecialRandom(scale) => Box::new(random_quadruples(
            &EXAMPLE_SEED,
            &(|seed| special_random_unsigned_vecs_min_length(seed, scale, 3)),
            &(|seed| special_random_unsigned_vecs_min_length(seed, scale, 9)),
            &(|seed| special_random_unsigned_vecs_min_length(seed, scale, 5)),
            &(|seed| random_range_up(seed, 1 << (Limb::WIDTH - 1))),
        )),
    };
    Box::new(
        qs.filter(|(q, n, d_init, _)| {
            let d_len = d_init.len() + 1;
            if n.len() < d_len {
                return false;
            }
            let q_len = n.len() - d_len + 1;
            if (q_len << 1) > n.len() || q_len > d_len {
                return false;
            }
            let n_len = q_len << 1;
            let d_len = q_len;
            q.len() >= q_len && d_len >= 6 && n_len >= d_len + 3 && d_len >= q_len
        })
        .map(|(q, n, mut d_init, d_last)| {
            d_init.push(d_last);
            (q, n, d_init)
        }),
    )
}

// All triples of `Vec<Limb>`, where `qs`, `ns`, and `ds` meet the preconditions of
// `_limbs_div_barrett_approx`.
pub fn triples_of_unsigned_vec_var_41(gm: GenerationMode) -> It<(Vec<Limb>, Vec<Limb>, Vec<Limb>)> {
    let qs: It<(Vec<Limb>, Vec<Limb>, Vec<Limb>, Limb)> = match gm {
        GenerationMode::Exhaustive => Box::new(exhaustive_quadruples(
            exhaustive_vecs(exhaustive_unsigned()),
            exhaustive_vecs_min_length(2, exhaustive_unsigned()),
            exhaustive_vecs_min_length(1, exhaustive_unsigned()),
            range_up_increasing(1 << (Limb::WIDTH - 1)),
        )),
        GenerationMode::Random(scale) => Box::new(random_quadruples(
            &EXAMPLE_SEED,
            &(|seed| random_vecs(seed, scale, &(|seed_2| random(seed_2)))),
            &(|seed| random_vecs_min_length(seed, scale, 2, &(|seed_2| random(seed_2)))),
            &(|seed| random_vecs_min_length(seed, scale, 1, &(|seed_2| random(seed_2)))),
            &(|seed| random_range_up(seed, 1 << (Limb::WIDTH - 1))),
        )),
        GenerationMode::SpecialRandom(scale) => Box::new(random_quadruples(
            &EXAMPLE_SEED,
            &(|seed| special_random_unsigned_vecs(seed, scale)),
            &(|seed| special_random_unsigned_vecs_min_length(seed, scale, 2)),
            &(|seed| special_random_unsigned_vecs_min_length(seed, scale, 1)),
            &(|seed| random_range_up(seed, 1 << (Limb::WIDTH - 1))),
        )),
    };
    Box::new(
        qs.filter(|(q, n, d_init, _)| {
            let d_len = d_init.len() + 1;
            n.len() >= d_len && q.len() >= n.len() - d_len
        })
        .map(|(q, n, mut d_init, d_last)| {
            d_init.push(d_last);
            (q, n, d_init)
        }),
    )
}

// All triples of `Vec<Limb>`, where `qs`, `ns`, and `ds` meet the preconditions of
// `_limbs_div_barrett`.
pub fn triples_of_unsigned_vec_var_42(gm: GenerationMode) -> It<(Vec<Limb>, Vec<Limb>, Vec<Limb>)> {
    let qs: It<(Vec<Limb>, Vec<Limb>, Vec<Limb>, Limb)> = match gm {
        GenerationMode::Exhaustive => Box::new(exhaustive_quadruples(
            exhaustive_vecs(exhaustive_unsigned()),
            exhaustive_vecs_min_length(3, exhaustive_unsigned()),
            exhaustive_vecs_min_length(1, exhaustive_unsigned()),
            range_up_increasing(1 << (Limb::WIDTH - 1)),
        )),
        GenerationMode::Random(scale) => Box::new(random_quadruples(
            &EXAMPLE_SEED,
            &(|seed| random_vecs(seed, scale, &(|seed_2| random(seed_2)))),
            &(|seed| random_vecs_min_length(seed, scale, 3, &(|seed_2| random(seed_2)))),
            &(|seed| random_vecs_min_length(seed, scale, 1, &(|seed_2| random(seed_2)))),
            &(|seed| random_range_up(seed, 1 << (Limb::WIDTH - 1))),
        )),
        GenerationMode::SpecialRandom(scale) => Box::new(random_quadruples(
            &EXAMPLE_SEED,
            &(|seed| special_random_unsigned_vecs(seed, scale)),
            &(|seed| special_random_unsigned_vecs_min_length(seed, scale, 3)),
            &(|seed| special_random_unsigned_vecs_min_length(seed, scale, 1)),
            &(|seed| random_range_up(seed, 1 << (Limb::WIDTH - 1))),
        )),
    };
    Box::new(
        qs.filter(|(q, n, d_init, _)| {
            let d_len = d_init.len() + 1;
            n.len() > d_len && q.len() >= n.len() - d_len
        })
        .map(|(q, n, mut d_init, d_last)| {
            d_init.push(d_last);
            (q, n, d_init)
        }),
    )
}

// All triples of `Vec<Limb>`, where `qs`, `ns`, and `ds` meet the preconditions of
// `_limbs_div_to_out`.
pub fn triples_of_unsigned_vec_var_43(gm: GenerationMode) -> It<(Vec<Limb>, Vec<Limb>, Vec<Limb>)> {
    let ts: It<(Vec<Limb>, Vec<Limb>, Vec<Limb>)> = match gm {
        GenerationMode::Exhaustive => Box::new(exhaustive_triples(
            exhaustive_vecs_min_length(1, exhaustive_unsigned()),
            exhaustive_vecs_min_length(2, exhaustive_unsigned()),
            exhaustive_vecs_min_length(2, exhaustive_unsigned()),
        )),
        GenerationMode::Random(scale) => Box::new(random_triples(
            &EXAMPLE_SEED,
            &(|seed| random_vecs_min_length(seed, scale, 1, &(|seed_2| random(seed_2)))),
            &(|seed| random_vecs_min_length(seed, scale, 2, &(|seed_2| random(seed_2)))),
            &(|seed| random_vecs_min_length(seed, scale, 2, &(|seed_2| random(seed_2)))),
        )),
        GenerationMode::SpecialRandom(scale) => Box::new(random_triples(
            &EXAMPLE_SEED,
            &(|seed| special_random_unsigned_vecs_min_length(seed, scale, 1)),
            &(|seed| special_random_unsigned_vecs_min_length(seed, scale, 2)),
            &(|seed| special_random_unsigned_vecs_min_length(seed, scale, 2)),
        )),
    };
    Box::new(ts.filter(|(q, n, d)| {
        *d.last().unwrap() != Limb::ZERO && n.len() >= d.len() && q.len() >= n.len() - d.len() + 1
    }))
}

// All triples of `Vec<Limb>`, where `qs`, `ns`, and `ds` meet the preconditions of
// `_limbs_div_to_out` and both the balanced and unbalanced div helper functions.
pub(crate) fn triples_of_unsigned_vec_var_44(
    gm: GenerationMode,
) -> It<(Vec<Limb>, Vec<Limb>, Vec<Limb>)> {
    Box::new(triples_of_unsigned_vec_var_43(gm).filter(|(_, n, d)| n.len() < (d.len() - 1) << 1))
}

// All triples of `Vec<Limb>`, where `rs`, `ns`, and `ds` meet the preconditions of
// `_limbs_mod_to_out`.
pub fn triples_of_unsigned_vec_var_45(gm: GenerationMode) -> It<(Vec<Limb>, Vec<Limb>, Vec<Limb>)> {
    let ts: It<(Vec<Limb>, Vec<Limb>, Vec<Limb>)> = match gm {
        GenerationMode::Exhaustive => Box::new(exhaustive_triples_from_single(
            exhaustive_vecs_min_length(2, exhaustive_unsigned()),
        )),
        GenerationMode::Random(scale) => Box::new(random_triples_from_single(
            random_vecs_min_length(&EXAMPLE_SEED, scale, 2, &(|seed| random(seed))),
        )),
        GenerationMode::SpecialRandom(scale) => Box::new(random_triples_from_single(
            special_random_unsigned_vecs_min_length(&EXAMPLE_SEED, scale, 2),
        )),
    };
    Box::new(ts.filter(|(r, n, d)| {
        *d.last().unwrap() != Limb::ZERO && n.len() >= d.len() && r.len() >= d.len()
    }))
}

// All triples of `Vec<T>`, where `T` is unsigned, the first `Vec` is at least as long as the
// second, and the second and third are equally long and nonempty.
pub fn triples_of_unsigned_vec_var_46<T: PrimitiveUnsigned + Rand>(
    gm: GenerationMode,
) -> It<(Vec<T>, Vec<T>, Vec<T>)> {
    let ts: It<((Vec<T>, Vec<T>), Vec<T>)> = match gm {
        GenerationMode::Exhaustive => Box::new(sqrt_pairs(
            pairs_of_unsigned_vec_var_11(gm),
            exhaustive_vecs(exhaustive_unsigned()),
        )),
        GenerationMode::Random(scale) => Box::new(random_pairs(
            &EXAMPLE_SEED,
            &(|seed| pairs_of_unsigned_vec_var_11_with_seed(gm, seed)),
            &(|seed| random_vecs(seed, scale, &(|seed_2| random(seed_2)))),
        )),
        GenerationMode::SpecialRandom(scale) => Box::new(random_pairs(
            &EXAMPLE_SEED,
            &(|seed| pairs_of_unsigned_vec_var_11_with_seed(gm, seed)),
            &(|seed| special_random_unsigned_vecs(seed, scale)),
        )),
    };
    reshape_1_2_to_3(permute_2_1(Box::new(
        ts.filter(|&((ref xs, _), ref out)| out.len() >= xs.len()),
    )))
}

// All triples of `Vec<T>`, where `T` is unsigned, the first `Vec` is at least as long as the
// sum of the lengths of the second and third, and the second and third are equally long and
// nonempty.
pub(crate) fn triples_of_unsigned_vec_var_47<T: PrimitiveUnsigned + Rand>(
    gm: GenerationMode,
) -> It<(Vec<T>, Vec<T>, Vec<T>)> {
    let ts: It<((Vec<T>, Vec<T>), Vec<T>)> = match gm {
        GenerationMode::Exhaustive => Box::new(sqrt_pairs(
            pairs_of_unsigned_vec_var_11(gm),
            exhaustive_vecs(exhaustive_unsigned()),
        )),
        GenerationMode::Random(scale) => Box::new(random_pairs(
            &EXAMPLE_SEED,
            &(|seed| pairs_of_unsigned_vec_var_11_with_seed(gm, seed)),
            &(|seed| random_vecs(seed, scale, &(|seed_2| random(seed_2)))),
        )),
        GenerationMode::SpecialRandom(scale) => Box::new(random_pairs(
            &EXAMPLE_SEED,
            &(|seed| pairs_of_unsigned_vec_var_11_with_seed(gm, seed)),
            &(|seed| special_random_unsigned_vecs(seed, scale)),
        )),
    };
    reshape_1_2_to_3(permute_2_1(Box::new(
        ts.filter(|&((ref xs, _), ref out)| out.len() >= xs.len() << 1),
    )))
}

// All triples of `Vec<T>`, where `T` is unsigned, `xs` and `ys` have the same lengths, which are at
// least 2, and `out` is at least twice as long as `xs`.
pub fn triples_of_unsigned_vec_var_48<T: PrimitiveUnsigned + Rand>(
    gm: GenerationMode,
) -> It<(Vec<T>, Vec<T>, Vec<T>)> {
    Box::new(triples_of_unsigned_vec_var_25(gm).filter(|(_, xs, _)| xs.len() > 1))
}

// All triples of `Vec<T>`, where `T` is unsigned, the first `Vec` is at least as long as the
// second, and the second and third are equally long and have length at least 2.
pub(crate) fn triples_of_unsigned_vec_var_49<T: PrimitiveUnsigned + Rand>(
    gm: GenerationMode,
) -> It<(Vec<T>, Vec<T>, Vec<T>)> {
    Box::new(triples_of_unsigned_vec_var_46(gm).filter(|(_, xs, _)| xs.len() > 1))
}

// All triples of `Vec<Limb>`, where `qs`, `ns`, and `ds` meet the preconditions of
// `_limbs_modular_div_barrett`.
pub fn triples_of_unsigned_vec_var_50(gm: GenerationMode) -> It<(Vec<Limb>, Vec<Limb>, Vec<Limb>)> {
    let ts: It<(Vec<Limb>, Vec<Limb>, Vec<Limb>)> = match gm {
        GenerationMode::Exhaustive => Box::new(exhaustive_triples_from_single(
            exhaustive_vecs_min_length(2, exhaustive_unsigned()),
        )),
        GenerationMode::Random(scale) => Box::new(random_triples_from_single(
            random_vecs_min_length(&EXAMPLE_SEED, scale, 2, &(|seed| random(seed))),
        )),
        GenerationMode::SpecialRandom(scale) => Box::new(random_triples_from_single(
            special_random_unsigned_vecs_min_length(&EXAMPLE_SEED, scale, 2),
        )),
    };
    Box::new(ts.filter(|(q, n, d)| q.len() >= n.len() && n.len() >= d.len() && d[0].odd()))
}

// All triples of `Vec<Limb>`, where `qs`, `ns`, and `ds` meet the preconditions of
// `_limbs_modular_div`.
pub fn triples_of_unsigned_vec_var_51(gm: GenerationMode) -> It<(Vec<Limb>, Vec<Limb>, Vec<Limb>)> {
    let ts: It<(Vec<Limb>, Vec<Limb>, Vec<Limb>)> = match gm {
        GenerationMode::Exhaustive => Box::new(exhaustive_triples_from_single(
            exhaustive_vecs_min_length(1, exhaustive_unsigned()),
        )),
        GenerationMode::Random(scale) => Box::new(random_triples_from_single(
            random_vecs_min_length(&EXAMPLE_SEED, scale, 1, &(|seed| random(seed))),
        )),
        GenerationMode::SpecialRandom(scale) => Box::new(random_triples_from_single(
            special_random_unsigned_vecs_min_length(&EXAMPLE_SEED, scale, 1),
        )),
    };
    Box::new(ts.filter(|(q, n, d)| q.len() >= n.len() && n.len() >= d.len() && d[0].odd()))
}

// All triples of `Vec<T>`, T being unsigned, where the three components of the triple have the same
// length, which is at least 2.
pub(crate) fn triples_of_unsigned_vec_var_52<T: PrimitiveUnsigned + Rand>(
    gm: GenerationMode,
) -> It<(Vec<T>, Vec<T>, Vec<T>)> {
    Box::new(triples_of_unsigned_vec_var_1(gm).filter(|&(ref xs, _, _)| xs.len() >= 2))
}

// All triples of `Vec<Limb>`, where `qs`, `ns`, and `ds` meet the preconditions of
// `limbs_div_exact_to_out`.
pub fn triples_of_unsigned_vec_var_53(gm: GenerationMode) -> It<(Vec<Limb>, Vec<Limb>, Vec<Limb>)> {
    Box::new(
        triples_of_unsigned_vec_min_sizes_3(gm, 1)
            .filter(|&(ref qs, ref ns, ref ds)| {
                qs.len() >= ns.len() - 1 && *ds.last().unwrap() != 0
            })
            .map(|(out, ns, ds)| {
                let mut new_ns = limbs_mul(&ns, &ds);
                if *new_ns.last().unwrap() == 0 {
                    new_ns.pop();
                }
                (out, new_ns, ds)
            })
            .filter(|&(ref qs, ref ns, ref ds)| qs.len() >= ns.len() - ds.len() + 1),
    )
}

// All triples of `Vec<Limb>`, where `qs`, `ns`, and `ds` meet the preconditions of both
// `limbs_div_to_out` and `limbs_div_exact_to_out`.
pub(crate) fn triples_of_unsigned_vec_var_54(
    gm: GenerationMode,
) -> It<(Vec<Limb>, Vec<Limb>, Vec<Limb>)> {
    Box::new(triples_of_unsigned_vec_var_53(gm).filter(|&(_, _, ref ds)| ds.len() > 1))
}

// All triples of `Vec<T>` that meet the preconditions for `limbs_eq_mod`.
pub fn triples_of_unsigned_vec_var_55<T: PrimitiveUnsigned + Rand>(
    gm: GenerationMode,
) -> It<(Vec<T>, Vec<T>, Vec<T>)> {
    match gm {
        GenerationMode::Exhaustive => Box::new(exhaustive_triples_from_single(
            exhaustive_vecs_min_length(2, exhaustive_unsigned())
                .filter(|xs| *xs.last().unwrap() != T::ZERO),
        )),
        GenerationMode::Random(scale) => Box::new(random_triples_from_single(
            random_vecs_min_length(&EXAMPLE_SEED, scale, 2, &(|seed| random(seed)))
                .filter(|xs| *xs.last().unwrap() != T::ZERO),
        )),
        GenerationMode::SpecialRandom(scale) => Box::new(random_triples_from_single(
            special_random_unsigned_vecs_min_length(&EXAMPLE_SEED, scale, 2)
                .filter(|xs| *xs.last().unwrap() != T::ZERO),
        )),
    }
}

// All triples of `Vec<T>` that meet the preconditions for `limbs_eq_mod`, where the `Natural`
// represented by the first `Vec` is equal to the `Natural` represented by the second `Vec` mod the
// `Natural` represented by the third `Vec`.
pub fn triples_of_unsigned_vec_var_56(gm: GenerationMode) -> It<(Vec<Limb>, Vec<Limb>, Vec<Limb>)> {
    Box::new(triples_of_unsigned_vec_var_55(gm).map(|(xs, ys, modulus)| {
        let mut product_limbs = if xs.is_empty() {
            Vec::new()
        } else {
            limbs_mul(&xs, &modulus)
        };
        if product_limbs.last() == Some(&0) {
            product_limbs.pop();
        }
        limbs_vec_add_in_place_left(&mut product_limbs, &ys);
        (product_limbs, ys, modulus)
    }))
}

// All triples of `Vec<T>` that meet the preconditions for `limbs_eq_mod`, where the `Natural`
// represented by the first `Vec` is not equal to the `Natural` represented by the second `Vec` mod
// the `Natural` represented by the third `Vec`.
pub fn triples_of_unsigned_vec_var_57(gm: GenerationMode) -> It<(Vec<Limb>, Vec<Limb>, Vec<Limb>)> {
    Box::new(
        triples_of_unsigned_vec_var_55::<Limb>(gm)
            .filter(|(xs, ys, modulus)| !limbs_eq_mod_ref_ref_ref(&*xs, &*ys, &*modulus)),
    )
}

//TODO remove next 3!
pub fn triples_of_unsigned_vec_var_58<T: PrimitiveUnsigned + Rand>(
    gm: GenerationMode,
) -> It<(Vec<T>, Vec<T>, Vec<T>)> {
    match gm {
        GenerationMode::Exhaustive => Box::new(exhaustive_triples_from_single(
            exhaustive_vecs_min_length(1, exhaustive_unsigned())
                .filter(|xs| *xs.last().unwrap() != T::ZERO),
        )),
        GenerationMode::Random(scale) => Box::new(random_triples_from_single(
            random_vecs_min_length(&EXAMPLE_SEED, scale, 1, &(|seed| random(seed)))
                .filter(|xs| *xs.last().unwrap() != T::ZERO),
        )),
        GenerationMode::SpecialRandom(scale) => Box::new(random_triples_from_single(
            special_random_unsigned_vecs_min_length(&EXAMPLE_SEED, scale, 1)
                .filter(|xs| *xs.last().unwrap() != T::ZERO),
        )),
    }
}

pub fn triples_of_unsigned_vec_var_59(gm: GenerationMode) -> It<(Vec<Limb>, Vec<Limb>, Vec<Limb>)> {
    Box::new(
        triples_of_unsigned_vec_var_58(gm).filter_map(|(xs, ys, modulus)| {
            let mut product_limbs = if xs.is_empty() {
                Vec::new()
            } else {
                limbs_mul(&xs, &modulus)
            };
            if product_limbs.last() == Some(&0) {
                product_limbs.pop();
            }
            if product_limbs.len() < ys.len()
                || limbs_sub_in_place_left(&mut product_limbs, &ys)
                || *product_limbs.last().unwrap() == 0
            {
                None
            } else {
                Some((product_limbs, ys, modulus))
            }
        }),
    )
}

pub fn triples_of_unsigned_vec_var_60(gm: GenerationMode) -> It<(Vec<Limb>, Vec<Limb>, Vec<Limb>)> {
    Box::new(
        triples_of_unsigned_vec_var_58::<Limb>(gm).filter(|(xs, ys, modulus)| {
            !Integer::from(Natural::from_limbs_asc(xs)).eq_mod(
                -Natural::from_limbs_asc(ys),
                Natural::from_limbs_asc(modulus),
            )
        }),
    )
}

fn pairs_of_unsigned_vec_min_sizes_var_1_with_seed<T: PrimitiveUnsigned + Rand>(
    gm: GenerationMode,
    min_len: u64,
    seed: &[u32],
) -> It<(Vec<T>, Vec<T>)> {
    let xss: It<Vec<T>> =
        match gm {
            GenerationMode::Exhaustive => Box::new(exhaustive_vecs_min_length(
                min_len << 1,
                exhaustive_unsigned(),
            )),
            GenerationMode::Random(scale) => Box::new(random_vecs_min_length(
                seed,
                scale,
                min_len << 1,
                &(|seed| random(seed)),
            )),
            GenerationMode::SpecialRandom(scale) => Box::new(
                special_random_unsigned_vecs_min_length(seed, scale, min_len << 1),
            ),
        };
    Box::new(xss.filter(|xs| xs.len().even()).map(|xs| {
        let half_length = xs.len() >> 1;
        (xs[..half_length].to_vec(), xs[half_length..].to_vec())
    }))
}

fn pairs_of_unsigned_vec_min_sizes_var_1<T: PrimitiveUnsigned + Rand>(
    gm: GenerationMode,
    min_len: u64,
) -> It<(Vec<T>, Vec<T>)> {
    pairs_of_unsigned_vec_min_sizes_var_1_with_seed(gm, min_len, &EXAMPLE_SEED)
}

fn pairs_of_unsigned_vec_min_sizes_2<T: PrimitiveUnsigned + Rand>(
    gm: GenerationMode,
    min_len: u64,
) -> It<(Vec<T>, Vec<T>)> {
    match gm {
        GenerationMode::Exhaustive => Box::new(exhaustive_pairs_from_single(
            exhaustive_vecs_min_length(min_len, exhaustive_unsigned()),
        )),
        GenerationMode::Random(scale) => Box::new(random_pairs_from_single(
            random_vecs_min_length(&EXAMPLE_SEED, scale, min_len, &(|seed| random(seed))),
        )),
        GenerationMode::SpecialRandom(scale) => Box::new(random_pairs_from_single(
            special_random_unsigned_vecs_min_length(&EXAMPLE_SEED, scale, min_len),
        )),
    }
}

fn triples_of_unsigned_vec_min_sizes<T: PrimitiveUnsigned + Rand>(
    gm: GenerationMode,
    min_xs_len: u64,
    min_ys_len: u64,
    min_zs_len: u64,
) -> It<(Vec<T>, Vec<T>, Vec<T>)> {
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

fn triples_of_unsigned_vec_min_sizes_1_2<T: PrimitiveUnsigned + Rand>(
    gm: GenerationMode,
    min_xs_len: u64,
    min_ys_zs_len: u64,
) -> It<(Vec<T>, Vec<T>, Vec<T>)> {
    let xss: It<((Vec<T>, Vec<T>), Vec<T>)> = match gm {
        GenerationMode::Exhaustive => Box::new(sqrt_pairs(
            pairs_of_unsigned_vec_min_sizes_var_1(gm, min_ys_zs_len),
            exhaustive_vecs_min_length(min_xs_len, exhaustive_unsigned()),
        )),
        GenerationMode::Random(scale) => Box::new(random_pairs(
            &EXAMPLE_SEED,
            &(|seed| pairs_of_unsigned_vec_min_sizes_var_1_with_seed(gm, min_xs_len, seed)),
            &(|seed| random_vecs_min_length(seed, scale, min_xs_len, &(|seed_2| random(seed_2)))),
        )),
        GenerationMode::SpecialRandom(scale) => Box::new(random_pairs(
            &EXAMPLE_SEED,
            &(|seed| pairs_of_unsigned_vec_min_sizes_var_1_with_seed(gm, min_xs_len, seed)),
            &(|seed| special_random_unsigned_vecs_min_length(seed, scale, min_xs_len)),
        )),
    };
    reshape_1_2_to_3(permute_2_1(xss))
}

fn triples_of_unsigned_vec_min_sizes_3<T: PrimitiveUnsigned + Rand>(
    gm: GenerationMode,
    min_len: u64,
) -> It<(Vec<T>, Vec<T>, Vec<T>)> {
    match gm {
        GenerationMode::Exhaustive => Box::new(exhaustive_triples_from_single(
            exhaustive_vecs_min_length(min_len, exhaustive_unsigned()),
        )),
        GenerationMode::Random(scale) => Box::new(random_triples_from_single(
            random_vecs_min_length(&EXAMPLE_SEED, scale, min_len, &(|seed| random(seed))),
        )),
        GenerationMode::SpecialRandom(scale) => Box::new(random_triples_from_single(
            special_random_unsigned_vecs_min_length(&EXAMPLE_SEED, scale, min_len),
        )),
    }
}

fn quadruples_of_three_unsigned_vecs_and_bool<T: PrimitiveUnsigned + Rand>(
    gm: GenerationMode,
) -> It<(Vec<T>, Vec<T>, Vec<T>, bool)> {
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
pub fn quadruples_of_three_unsigned_vecs_and_bool_var_1<T: PrimitiveUnsigned + Rand>(
    gm: GenerationMode,
) -> It<(Vec<T>, Vec<T>, Vec<T>, bool)> {
    Box::new(
        quadruples_of_three_unsigned_vecs_and_bool(gm)
            .filter(|&(ref xs, ref ys, ref zs, _)| ys.len() == zs.len() && xs.len() >= ys.len()),
    )
}

#[cfg(feature = "32_bit_limbs")]
const PRIME_FACTORS_OF_LIMB_MAX: &[Limb] = &[3, 5, 17, 257, 65_537];
#[cfg(not(feature = "32_bit_limbs"))]
const PRIME_FACTORS_OF_LIMB_MAX: &[Limb] = &[3, 5, 17, 257, 641, 65_537, 6_700_417];

// TODO use a more generic solution
fn factors_of_limb_max() -> Vec<Limb> {
    let mut factors = Vec::new();
    for i in (0 as Limb)..(1 << PRIME_FACTORS_OF_LIMB_MAX.len()) {
        let mut factor = 1;
        for (index, bit) in Natural::from(i).bits().enumerate() {
            if bit {
                factor *= PRIME_FACTORS_OF_LIMB_MAX[index];
            }
        }
        factors.push(factor);
    }
    factors
}

// All quadruples of `Vec<Limb>`, where `qs`, `rs`, `ns`, and `ds` meet the preconditions of
// `_limbs_div_mod_barrett`.
pub fn quadruples_of_unsigned_vec_var_1(
    gm: GenerationMode,
) -> It<(Vec<Limb>, Vec<Limb>, Vec<Limb>, Vec<Limb>)> {
    let qs: It<(Vec<Limb>, Vec<Limb>, Vec<Limb>, Vec<Limb>, Limb)> = match gm {
        GenerationMode::Exhaustive => Box::new(exhaustive_quintuples(
            exhaustive_vecs_min_length(1, exhaustive_unsigned()),
            exhaustive_vecs_min_length(2, exhaustive_unsigned()),
            exhaustive_vecs_min_length(3, exhaustive_unsigned()),
            exhaustive_vecs_min_length(1, exhaustive_unsigned()),
            range_up_increasing(1 << (Limb::WIDTH - 1)),
        )),
        GenerationMode::Random(scale) => Box::new(random_quintuples(
            &EXAMPLE_SEED,
            &(|seed| random_vecs_min_length(seed, scale, 1, &(|seed_2| random(seed_2)))),
            &(|seed| random_vecs_min_length(seed, scale, 2, &(|seed_2| random(seed_2)))),
            &(|seed| random_vecs_min_length(seed, scale, 3, &(|seed_2| random(seed_2)))),
            &(|seed| random_vecs_min_length(seed, scale, 1, &(|seed_2| random(seed_2)))),
            &(|seed| random_range_up(seed, 1 << (Limb::WIDTH - 1))),
        )),
        GenerationMode::SpecialRandom(scale) => Box::new(random_quintuples(
            &EXAMPLE_SEED,
            &(|seed| special_random_unsigned_vecs_min_length(seed, scale, 1)),
            &(|seed| special_random_unsigned_vecs_min_length(seed, scale, 2)),
            &(|seed| special_random_unsigned_vecs_min_length(seed, scale, 3)),
            &(|seed| special_random_unsigned_vecs_min_length(seed, scale, 1)),
            &(|seed| random_range_up(seed, 1 << (Limb::WIDTH - 1))),
        )),
    };
    Box::new(
        qs.filter(|(q, r, n, d_init, _)| {
            let d_len = d_init.len() + 1;
            r.len() >= d_len && n.len() > d_len && q.len() >= n.len() - d_len
        })
        .map(|(q, r, n, mut d_init, d_last)| {
            d_init.push(d_last);
            (q, r, n, d_init)
        }),
    )
}

// All quadruples of `Vec<Limb>`, where `qs`, `rs`, `ns`, and `ds` meet the preconditions of
// `_limbs_div_mod_to_out`.
pub fn quadruples_of_unsigned_vec_var_2(
    gm: GenerationMode,
) -> It<(Vec<Limb>, Vec<Limb>, Vec<Limb>, Vec<Limb>)> {
    let qs: It<(Vec<Limb>, Vec<Limb>, Vec<Limb>, Vec<Limb>)> = match gm {
        GenerationMode::Exhaustive => Box::new(exhaustive_quadruples(
            exhaustive_vecs_min_length(1, exhaustive_unsigned()),
            exhaustive_vecs_min_length(2, exhaustive_unsigned()),
            exhaustive_vecs_min_length(2, exhaustive_unsigned()),
            exhaustive_vecs_min_length(2, exhaustive_unsigned()),
        )),
        GenerationMode::Random(scale) => Box::new(random_quadruples(
            &EXAMPLE_SEED,
            &(|seed| random_vecs_min_length(seed, scale, 1, &(|seed_2| random(seed_2)))),
            &(|seed| random_vecs_min_length(seed, scale, 2, &(|seed_2| random(seed_2)))),
            &(|seed| random_vecs_min_length(seed, scale, 2, &(|seed_2| random(seed_2)))),
            &(|seed| random_vecs_min_length(seed, scale, 2, &(|seed_2| random(seed_2)))),
        )),
        GenerationMode::SpecialRandom(scale) => Box::new(random_quadruples(
            &EXAMPLE_SEED,
            &(|seed| special_random_unsigned_vecs_min_length(seed, scale, 1)),
            &(|seed| special_random_unsigned_vecs_min_length(seed, scale, 2)),
            &(|seed| special_random_unsigned_vecs_min_length(seed, scale, 2)),
            &(|seed| special_random_unsigned_vecs_min_length(seed, scale, 2)),
        )),
    };
    Box::new(qs.filter(|(q, r, n, d)| {
        *d.last().unwrap() != Limb::ZERO
            && r.len() >= d.len()
            && n.len() >= d.len()
            && q.len() >= n.len() - d.len() + 1
    }))
}

// All quadruples of `Vec<Limb>`, where `qs`, `rs`, `ns`, and `ds` meet certain preconditions that
// enable comparing the performance of two kinds of Barrett division.
pub(crate) fn quadruples_of_unsigned_vec_var_3(
    gm: GenerationMode,
) -> It<(Vec<Limb>, Vec<Limb>, Vec<Limb>, Vec<Limb>)> {
    Box::new(quadruples_of_unsigned_vec_var_1(gm).filter(|(_, _, n, d)| 2 * d.len() > n.len() + 1))
}

// All quadruples of `Vec<Limb>`, where `qs`, `rs`, `ns`, and `ds` meet the preconditions of
// `_limbs_modular_div_mod_barrett`.
pub fn quadruples_of_unsigned_vec_var_4(
    gm: GenerationMode,
) -> It<(Vec<Limb>, Vec<Limb>, Vec<Limb>, Vec<Limb>)> {
    let qs: It<(Vec<Limb>, Vec<Limb>, Vec<Limb>, Vec<Limb>)> = match gm {
        GenerationMode::Exhaustive => Box::new(exhaustive_quadruples(
            exhaustive_vecs_min_length(2, exhaustive_unsigned()),
            exhaustive_vecs_min_length(2, exhaustive_unsigned()),
            exhaustive_vecs_min_length(4, exhaustive_unsigned()),
            exhaustive_vecs_min_length(2, exhaustive_unsigned()),
        )),
        GenerationMode::Random(scale) => Box::new(random_quadruples(
            &EXAMPLE_SEED,
            &(|seed| random_vecs_min_length(seed, scale, 2, &(|seed_2| random(seed_2)))),
            &(|seed| random_vecs_min_length(seed, scale, 2, &(|seed_2| random(seed_2)))),
            &(|seed| random_vecs_min_length(seed, scale, 4, &(|seed_2| random(seed_2)))),
            &(|seed| random_vecs_min_length(seed, scale, 2, &(|seed_2| random(seed_2)))),
        )),
        GenerationMode::SpecialRandom(scale) => Box::new(random_quadruples(
            &EXAMPLE_SEED,
            &(|seed| special_random_unsigned_vecs_min_length(seed, scale, 2)),
            &(|seed| special_random_unsigned_vecs_min_length(seed, scale, 2)),
            &(|seed| special_random_unsigned_vecs_min_length(seed, scale, 4)),
            &(|seed| special_random_unsigned_vecs_min_length(seed, scale, 2)),
        )),
    };
    Box::new(qs.filter(|(q, r, n, d)| {
        if n.len() < d.len() + 2 {
            return false;
        }
        let q_len = n.len() - d.len();
        q.len() >= q_len && r.len() >= d.len() && d[0].odd()
    }))
}

// All quadruples of `Vec<Limb>`, where `qs`, `rs`, `ns`, and `ds` meet the preconditions of
// `_limbs_modular_div_mod_barrett` and `qs`, `ns`, and `ds` would meet the preconditions of
// `_limbs_modular_div_mod_divide_and_conquer`, given the correct `inverse`.
pub(crate) fn quadruples_of_unsigned_vec_var_5(
    gm: GenerationMode,
) -> It<(Vec<Limb>, Vec<Limb>, Vec<Limb>, Vec<Limb>)> {
    let qs: It<(Vec<Limb>, Vec<Limb>, Vec<Limb>, Vec<Limb>)> = match gm {
        GenerationMode::Exhaustive => Box::new(exhaustive_quadruples(
            exhaustive_vecs_min_length(4, exhaustive_unsigned()),
            exhaustive_vecs_min_length(2, exhaustive_unsigned()),
            exhaustive_vecs_min_length(4, exhaustive_unsigned()),
            exhaustive_vecs_min_length(2, exhaustive_unsigned()),
        )),
        GenerationMode::Random(scale) => Box::new(random_quadruples(
            &EXAMPLE_SEED,
            &(|seed| random_vecs_min_length(seed, scale, 4, &(|seed_2| random(seed_2)))),
            &(|seed| random_vecs_min_length(seed, scale, 2, &(|seed_2| random(seed_2)))),
            &(|seed| random_vecs_min_length(seed, scale, 4, &(|seed_2| random(seed_2)))),
            &(|seed| random_vecs_min_length(seed, scale, 2, &(|seed_2| random(seed_2)))),
        )),
        GenerationMode::SpecialRandom(scale) => Box::new(random_quadruples(
            &EXAMPLE_SEED,
            &(|seed| special_random_unsigned_vecs_min_length(seed, scale, 4)),
            &(|seed| special_random_unsigned_vecs_min_length(seed, scale, 2)),
            &(|seed| special_random_unsigned_vecs_min_length(seed, scale, 4)),
            &(|seed| special_random_unsigned_vecs_min_length(seed, scale, 2)),
        )),
    };
    Box::new(qs.filter(|(q, r, n, d)| {
        if n.len() < d.len() + 2 {
            return false;
        }
        let q_len = n.len() - d.len();
        q.len() >= q_len && r.len() >= d.len() && d[0].odd()
    }))
}

pub(crate) fn sextuples_of_four_limb_vecs_and_two_usizes_var_1(
    gm: GenerationMode,
) -> It<(Vec<Limb>, Vec<Limb>, Vec<Limb>, Vec<Limb>, usize, usize)> {
    let qs: It<(Vec<Limb>, Vec<Limb>, Vec<Limb>, Vec<Limb>, u32)> = match gm {
        GenerationMode::Exhaustive => Box::new(exhaustive_quintuples(
            exhaustive_vecs_min_length(2, exhaustive_unsigned()),
            exhaustive_vecs(exhaustive_unsigned()),
            exhaustive_vecs(exhaustive_unsigned()),
            exhaustive_vecs(exhaustive_unsigned()),
            range_up_increasing(3),
        )),
        GenerationMode::Random(scale) => Box::new(random_quintuples(
            &EXAMPLE_SEED,
            &(|seed| random_vecs_min_length(seed, scale, 2, &(|seed_2| random(seed_2)))),
            &(|seed| random_vecs(seed, scale, &(|seed_2| random(seed_2)))),
            &(|seed| random_vecs(seed, scale, &(|seed_2| random(seed_2)))),
            &(|seed| random_vecs(seed, scale, &(|seed_2| random(seed_2)))),
            &(|seed| range_up_geometric_u32(seed, scale, 3)),
        )),
        GenerationMode::SpecialRandom(scale) => Box::new(random_quintuples(
            &EXAMPLE_SEED,
            &(|seed| special_random_unsigned_vecs_min_length(seed, scale, 2)),
            &(|seed| special_random_unsigned_vecs(seed, scale)),
            &(|seed| special_random_unsigned_vecs(seed, scale)),
            &(|seed| special_random_unsigned_vecs(seed, scale)),
            &(|seed| range_up_geometric_u32(seed, scale, 3)),
        )),
    };
    Box::new(
        qs.filter_map(|(ds, mut scratch, mut qs, mut rs_hi, n_len)| {
            let n_len = usize::wrapping_from(n_len);
            let d_len = ds.len();
            if n_len < d_len {
                return None;
            }
            let i_len = _limbs_div_mod_barrett_is_len(n_len - d_len, d_len);
            if i_len == 0 || qs.len() < i_len {
                return None;
            }
            qs.truncate(i_len);
            if rs_hi.len() < i_len {
                return None;
            }
            rs_hi.truncate(i_len);
            let scratch_len = _limbs_mul_mod_base_pow_n_minus_1_next_size(d_len + 1);
            let x = _limbs_div_mod_barrett_scratch_len(n_len, d_len);
            if x < i_len {
                return None;
            }
            let actual_scratch_len = x - i_len;
            if actual_scratch_len < d_len + i_len {
                return None;
            }
            if scratch.len() < actual_scratch_len {
                return None;
            }
            scratch.truncate(actual_scratch_len);
            Some((scratch, ds, qs, rs_hi, scratch_len, i_len))
        }),
    )
}

fn quadruples_of_unsigned_vec_unsigned_vec_unsigned_and_unsigned<T: PrimitiveUnsigned + Rand>(
    gm: GenerationMode,
) -> It<(Vec<T>, Vec<T>, T, T)> {
    reshape_2_2_to_4(match gm {
        GenerationMode::Exhaustive => Box::new(exhaustive_pairs(
            exhaustive_pairs_from_single(exhaustive_vecs(exhaustive_unsigned())),
            exhaustive_pairs_from_single(exhaustive_unsigned()),
        )),
        GenerationMode::Random(scale) => Box::new(random_pairs(
            &EXAMPLE_SEED,
            &(|seed| {
                random_pairs_from_single(random_vecs(seed, scale, &(|seed_2| random(seed_2))))
            }),
            &(|seed| random_pairs_from_single(random(seed))),
        )),
        GenerationMode::SpecialRandom(scale) => Box::new(random_pairs(
            &EXAMPLE_SEED,
            &(|seed| random_pairs_from_single(special_random_unsigned_vecs(seed, scale))),
            &(|seed| random_pairs_from_single(special_random_unsigned(seed))),
        )),
    })
}

// All quadruples of `Vec<Limb>`, `Vec<Limb>`, `Limb`, and `Limb`, where the first `Vec` is at least
// as long as the second.
pub fn quadruples_of_unsigned_vec_unsigned_vec_unsigned_and_unsigned_var_1<
    T: PrimitiveUnsigned + Rand,
>(
    gm: GenerationMode,
) -> It<(Vec<T>, Vec<T>, T, T)> {
    Box::new(
        quadruples_of_unsigned_vec_unsigned_vec_unsigned_and_unsigned(gm)
            .filter(|&(ref xs, ref ys, _, _)| xs.len() >= ys.len()),
    )
}

// All quadruples of `Vec<Limb>`, `Vec<Limb>`, `Limb`, and `Limb` where the first limb is a divisor
// of `Limb::MAX`.
fn quadruples_of_limb_vec_limb_vec_limb_and_limb_var_2(
    gm: GenerationMode,
) -> It<(Vec<Limb>, Vec<Limb>, Limb, Limb)> {
    match gm {
        GenerationMode::Exhaustive => permute_1_2_4_3(reshape_3_1_to_4(Box::new(sqrt_pairs(
            exhaustive_triples(
                exhaustive_vecs(exhaustive_unsigned()),
                exhaustive_vecs(exhaustive_unsigned()),
                exhaustive_unsigned(),
            ),
            factors_of_limb_max().into_iter(),
        )))),
        GenerationMode::Random(scale) => Box::new(random_quadruples(
            &EXAMPLE_SEED,
            &(|seed| random_vecs(seed, scale, &(|seed_2| random(seed_2)))),
            &(|seed| random_vecs(seed, scale, &(|seed_2| random(seed_2)))),
            &(|seed| random_from_vector(seed, factors_of_limb_max())),
            &(|seed| random(seed)),
        )),
        GenerationMode::SpecialRandom(scale) => Box::new(random_quadruples(
            &EXAMPLE_SEED,
            &(|seed| special_random_unsigned_vecs(seed, scale)),
            &(|seed| special_random_unsigned_vecs(seed, scale)),
            &(|seed| random_from_vector(seed, factors_of_limb_max())),
            &(|seed| special_random_unsigned(seed)),
        )),
    }
}

// All quadruples of `Vec<Limb>`, `Vec<Limb>`, `Limb`, and `Limb` where the first slice is at least
// as long as the second and the first limb is a divisor of `Limb::MAX`.
pub fn quadruples_of_limb_vec_limb_vec_limb_and_limb_var_3(
    gm: GenerationMode,
) -> It<(Vec<Limb>, Vec<Limb>, Limb, Limb)> {
    Box::new(
        quadruples_of_limb_vec_limb_vec_limb_and_limb_var_2(gm)
            .filter(|&(ref out, ref xs, _, _)| out.len() >= xs.len()),
    )
}

// All quadruples of `Vec<Limb>`, `Vec<Limb>`, `Vec<Limb>`, and `Limb`, where `qs`, `ns`, `ds`, and
// `inverse` meet the preconditions of `_limbs_div_mod_schoolbook`.
pub fn quadruples_of_three_unsigned_vecs_and_unsigned_var_1(
    gm: GenerationMode,
) -> It<(Vec<Limb>, Vec<Limb>, Vec<Limb>, Limb)> {
    let qs: It<(Vec<Limb>, Vec<Limb>, Vec<Limb>, Limb)> = match gm {
        GenerationMode::Exhaustive => Box::new(exhaustive_quadruples(
            exhaustive_vecs(exhaustive_unsigned()),
            exhaustive_vecs_min_length(3, exhaustive_unsigned()),
            exhaustive_vecs_min_length(2, exhaustive_unsigned()),
            range_up_increasing(1 << (Limb::WIDTH - 1)),
        )),
        GenerationMode::Random(scale) => Box::new(random_quadruples(
            &EXAMPLE_SEED,
            &(|seed| random_vecs(seed, scale, &(|seed_2| random(seed_2)))),
            &(|seed| random_vecs_min_length(seed, scale, 3, &(|seed_2| random(seed_2)))),
            &(|seed| random_vecs_min_length(seed, scale, 2, &(|seed_2| random(seed_2)))),
            &(|seed| random_range_up(seed, 1 << (Limb::WIDTH - 1))),
        )),
        GenerationMode::SpecialRandom(scale) => Box::new(random_quadruples(
            &EXAMPLE_SEED,
            &(|seed| special_random_unsigned_vecs(seed, scale)),
            &(|seed| special_random_unsigned_vecs_min_length(seed, scale, 3)),
            &(|seed| special_random_unsigned_vecs_min_length(seed, scale, 2)),
            &(|seed| random_range_up(seed, 1 << (Limb::WIDTH - 1))),
        )),
    };
    Box::new(
        qs.filter(|(q, n, d_init, _)| {
            let d_len = d_init.len() + 1;
            n.len() >= d_len && q.len() >= n.len() - d_len
        })
        .map(|(q, n, mut d_init, d_last)| {
            d_init.push(d_last);
            let inverse =
                limbs_two_limb_inverse_helper(d_init[d_init.len() - 1], d_init[d_init.len() - 2]);
            (q, n, d_init, inverse)
        }),
    )
}

// All quadruples of `Vec<Limb>`, `Vec<Limb>`, `Vec<Limb>`, and `Limb`, where `qs`, `ns`, `ds`, and
// `inverse` meet the preconditions of `_limbs_div_mod_divide_and_conquer`.
pub fn quadruples_of_three_unsigned_vecs_and_unsigned_var_2(
    gm: GenerationMode,
) -> It<(Vec<Limb>, Vec<Limb>, Vec<Limb>, Limb)> {
    let qs: It<(Vec<Limb>, Vec<Limb>, Vec<Limb>, Limb)> = match gm {
        GenerationMode::Exhaustive => Box::new(exhaustive_quadruples(
            exhaustive_vecs_min_length(3, exhaustive_unsigned()),
            exhaustive_vecs_min_length(9, exhaustive_unsigned()),
            exhaustive_vecs_min_length(5, exhaustive_unsigned()),
            range_up_increasing(1 << (Limb::WIDTH - 1)),
        )),
        GenerationMode::Random(scale) => Box::new(random_quadruples(
            &EXAMPLE_SEED,
            &(|seed| random_vecs_min_length(seed, scale, 3, &(|seed_2| random(seed_2)))),
            &(|seed| random_vecs_min_length(seed, scale, 9, &(|seed_2| random(seed_2)))),
            &(|seed| random_vecs_min_length(seed, scale, 5, &(|seed_2| random(seed_2)))),
            &(|seed| random_range_up(seed, 1 << (Limb::WIDTH - 1))),
        )),
        GenerationMode::SpecialRandom(scale) => Box::new(random_quadruples(
            &EXAMPLE_SEED,
            &(|seed| special_random_unsigned_vecs_min_length(seed, scale, 3)),
            &(|seed| special_random_unsigned_vecs_min_length(seed, scale, 9)),
            &(|seed| special_random_unsigned_vecs_min_length(seed, scale, 5)),
            &(|seed| random_range_up(seed, 1 << (Limb::WIDTH - 1))),
        )),
    };
    Box::new(
        qs.filter(|(q, n, d_init, _)| {
            let d_len = d_init.len() + 1;
            n.len() >= d_len + 3 && q.len() >= n.len() - d_len
        })
        .map(|(q, n, mut d_init, d_last)| {
            d_init.push(d_last);
            let inverse =
                limbs_two_limb_inverse_helper(d_init[d_init.len() - 1], d_init[d_init.len() - 2]);
            (q, n, d_init, inverse)
        }),
    )
}

// All quadruples of `Vec<Limb>`, `Vec<Limb>`, `Vec<Limb>`, and `Limb`, where `qs`, `ns`, `ds`, and
// `inverse` meet the preconditions of `_limbs_modular_div_schoolbook`.
pub fn quadruples_of_three_unsigned_vecs_and_unsigned_var_3(
    gm: GenerationMode,
) -> It<(Vec<Limb>, Vec<Limb>, Vec<Limb>, Limb)> {
    let ts: It<(Vec<Limb>, Vec<Limb>, Vec<Limb>)> = match gm {
        GenerationMode::Exhaustive => Box::new(exhaustive_triples_from_single(
            exhaustive_vecs_min_length(1, exhaustive_unsigned()),
        )),
        GenerationMode::Random(scale) => Box::new(random_triples_from_single(
            random_vecs_min_length(&EXAMPLE_SEED, scale, 1, &(|seed| random(seed))),
        )),
        GenerationMode::SpecialRandom(scale) => Box::new(random_triples_from_single(
            special_random_unsigned_vecs_min_length(&EXAMPLE_SEED, scale, 1),
        )),
    };
    Box::new(
        ts.filter(|(q, n, d)| q.len() >= n.len() && n.len() >= d.len() && d[0].odd())
            .map(|(q, n, d)| {
                let inverse = limbs_modular_invert_limb(d[0]).wrapping_neg();
                (q, n, d, inverse)
            }),
    )
}

// All quadruples of `Vec<Limb>`, `Vec<Limb>`, `Vec<Limb>`, and `Limb`, where `qs`, `ns`, `ds`, and
// `inverse` meet the preconditions of `_limbs_modular_div_mod_schoolbook`.
pub fn quadruples_of_three_unsigned_vecs_and_unsigned_var_4(
    gm: GenerationMode,
) -> It<(Vec<Limb>, Vec<Limb>, Vec<Limb>, Limb)> {
    let ts: It<(Vec<Limb>, Vec<Limb>, Vec<Limb>)> = match gm {
        GenerationMode::Exhaustive => Box::new(exhaustive_triples(
            exhaustive_vecs_min_length(2, exhaustive_unsigned()),
            exhaustive_vecs_min_length(2, exhaustive_unsigned()),
            exhaustive_vecs_min_length(1, exhaustive_unsigned()),
        )),
        GenerationMode::Random(scale) => Box::new(random_triples(
            &EXAMPLE_SEED,
            &(|seed| random_vecs_min_length(seed, scale, 2, &(|seed_2| random(seed_2)))),
            &(|seed| random_vecs_min_length(seed, scale, 2, &(|seed_2| random(seed_2)))),
            &(|seed| random_vecs_min_length(seed, scale, 1, &(|seed_2| random(seed_2)))),
        )),
        GenerationMode::SpecialRandom(scale) => Box::new(random_triples(
            &EXAMPLE_SEED,
            &(|seed| special_random_unsigned_vecs_min_length(seed, scale, 2)),
            &(|seed| special_random_unsigned_vecs_min_length(seed, scale, 2)),
            &(|seed| special_random_unsigned_vecs_min_length(seed, scale, 1)),
        )),
    };
    Box::new(
        ts.filter(|(q, n, d)| q.len() >= n.len() && n.len() > d.len() && d[0].odd())
            .map(|(q, n, d)| {
                let inverse = limbs_modular_invert_limb(d[0]).wrapping_neg();
                (q, n, d, inverse)
            }),
    )
}

// All quadruples of `Vec<Limb>`, `Vec<Limb>`, `Vec<Limb>`, and `Limb`, where `qs`, `ns`, `ds`, and
// `inverse` meet the preconditions of `_limbs_modular_div_mod_divide_and_conquer`.
pub fn quadruples_of_three_unsigned_vecs_and_unsigned_var_5(
    gm: GenerationMode,
) -> It<(Vec<Limb>, Vec<Limb>, Vec<Limb>, Limb)> {
    let ts: It<(Vec<Limb>, Vec<Limb>, Vec<Limb>)> = match gm {
        GenerationMode::Exhaustive => Box::new(exhaustive_triples(
            exhaustive_vecs_min_length(3, exhaustive_unsigned()),
            exhaustive_vecs_min_length(3, exhaustive_unsigned()),
            exhaustive_vecs_min_length(2, exhaustive_unsigned()),
        )),
        GenerationMode::Random(scale) => Box::new(random_triples(
            &EXAMPLE_SEED,
            &(|seed| random_vecs_min_length(seed, scale, 3, &(|seed_2| random(seed_2)))),
            &(|seed| random_vecs_min_length(seed, scale, 3, &(|seed_2| random(seed_2)))),
            &(|seed| random_vecs_min_length(seed, scale, 2, &(|seed_2| random(seed_2)))),
        )),
        GenerationMode::SpecialRandom(scale) => Box::new(random_triples(
            &EXAMPLE_SEED,
            &(|seed| special_random_unsigned_vecs_min_length(seed, scale, 3)),
            &(|seed| special_random_unsigned_vecs_min_length(seed, scale, 3)),
            &(|seed| special_random_unsigned_vecs_min_length(seed, scale, 2)),
        )),
    };
    Box::new(
        ts.filter(|(q, n, d)| q.len() >= n.len() && n.len() > d.len() && d[0].odd())
            .map(|(q, n, d)| {
                let inverse = limbs_modular_invert_limb(d[0]).wrapping_neg();
                (q, n, d, inverse)
            }),
    )
}

// All quadruples of `Vec<Limb>`, `Vec<Limb>`, `Vec<Limb>`, and `Limb`, where `qs`, `ns`, `ds`, and
// `inverse` meet the preconditions of `_limbs_modular_div_divide_and_conquer`.
pub fn quadruples_of_three_unsigned_vecs_and_unsigned_var_6(
    gm: GenerationMode,
) -> It<(Vec<Limb>, Vec<Limb>, Vec<Limb>, Limb)> {
    let ts: It<(Vec<Limb>, Vec<Limb>, Vec<Limb>)> = match gm {
        GenerationMode::Exhaustive => Box::new(exhaustive_triples_from_single(
            exhaustive_vecs_min_length(2, exhaustive_unsigned()),
        )),
        GenerationMode::Random(scale) => Box::new(random_triples_from_single(
            random_vecs_min_length(&EXAMPLE_SEED, scale, 2, &(|seed| random(seed))),
        )),
        GenerationMode::SpecialRandom(scale) => Box::new(random_triples_from_single(
            special_random_unsigned_vecs_min_length(&EXAMPLE_SEED, scale, 2),
        )),
    };
    Box::new(
        ts.filter(|(q, n, d)| q.len() >= n.len() && n.len() >= d.len() && d[0].odd())
            .map(|(q, n, d)| {
                let inverse = limbs_modular_invert_limb(d[0]).wrapping_neg();
                (q, n, d, inverse)
            }),
    )
}

// All quadruples of `Vec<Limb>`, `Vec<Limb>`, `Vec<Limb>`, and `Limb`, where `is`, `scratch`, `ds`,
// and `inverse` meet the preconditions of `_limbs_modular_invert_small`.
pub(crate) fn quadruples_of_three_unsigned_vecs_and_unsigned_var_7(
    gm: GenerationMode,
) -> It<(Vec<Limb>, Vec<Limb>, Vec<Limb>, Limb)> {
    let vs: It<Vec<Limb>> =
        match gm {
            GenerationMode::Exhaustive => {
                Box::new(exhaustive_vecs_min_length(1, exhaustive_unsigned()))
            }
            GenerationMode::Random(scale) => Box::new(random_vecs_min_length(
                &EXAMPLE_SEED,
                scale,
                1,
                &(|seed| random(seed)),
            )),
            GenerationMode::SpecialRandom(scale) => Box::new(
                special_random_unsigned_vecs_min_length(&EXAMPLE_SEED, scale, 1),
            ),
        };
    Box::new(vs.filter(|d| d[0].odd()).map(|d| {
        let inverse = limbs_modular_invert_limb(d[0]).wrapping_neg();
        let is = vec![0; d.len()];
        let scratch = vec![0; limbs_modular_invert_scratch_len(d.len())];
        (is, scratch, d, inverse)
    }))
}

// All triples of `Vec<Limb>`, `Limb`, and `Limb` where the first limb is a divisor of `Limb::MAX`.
pub fn triples_of_limb_vec_limb_and_limb_var_1(gm: GenerationMode) -> It<(Vec<Limb>, Limb, Limb)> {
    match gm {
        GenerationMode::Exhaustive => permute_1_3_2(reshape_2_1_to_3(Box::new(sqrt_pairs(
            exhaustive_pairs(
                exhaustive_vecs(exhaustive_unsigned()),
                exhaustive_unsigned(),
            ),
            factors_of_limb_max().into_iter(),
        )))),
        GenerationMode::Random(scale) => Box::new(random_triples(
            &EXAMPLE_SEED,
            &(|seed| random_vecs(seed, scale, &(|seed_2| random(seed_2)))),
            &(|seed| random_from_vector(seed, factors_of_limb_max())),
            &(|seed| random(seed)),
        )),
        GenerationMode::SpecialRandom(scale) => Box::new(random_triples(
            &EXAMPLE_SEED,
            &(|seed| special_random_unsigned_vecs(seed, scale)),
            &(|seed| random_from_vector(seed, factors_of_limb_max())),
            &(|seed| special_random_unsigned(seed)),
        )),
    }
}

fn pairs_of_ordering_and_vec_of_unsigned<T: PrimitiveUnsigned + Rand>(
    gm: GenerationMode,
) -> It<(Ordering, Vec<T>)> {
    match gm {
        GenerationMode::Exhaustive => permute_2_1(Box::new(lex_pairs(
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
pub fn pairs_of_ordering_and_vec_of_unsigned_var_1<T: PrimitiveUnsigned + Rand>(
    gm: GenerationMode,
) -> It<(Ordering, Vec<T>)> {
    Box::new(
        pairs_of_ordering_and_vec_of_unsigned(gm)
            .filter(|&(sign, ref limbs)| limbs_test_zero(limbs) == (sign == Ordering::Equal)),
    )
}

fn exhaustive_pairs_of_unsigned_vec_and_unsigned<T: PrimitiveUnsigned + Rand>() -> It<(Vec<T>, T)> {
    Box::new(exhaustive_pairs(
        exhaustive_vecs(exhaustive_unsigned()),
        exhaustive_unsigned(),
    ))
}

pub fn triples_of_unsigned_vec_small_usize_and_unsigned<T: PrimitiveUnsigned + Rand>(
    gm: GenerationMode,
) -> It<(Vec<T>, usize, T)> {
    match gm {
        GenerationMode::Exhaustive => permute_1_3_2(reshape_2_1_to_3(Box::new(log_pairs(
            exhaustive_pairs_of_unsigned_vec_and_unsigned(),
            exhaustive_unsigned::<u32>().map(usize::wrapping_from),
        )))),
        GenerationMode::Random(scale) => Box::new(random_triples(
            &EXAMPLE_SEED,
            &(|seed| random_vecs(seed, scale, &(|seed_2| random(seed_2)))),
            &(|seed| u32s_geometric(seed, scale).map(usize::wrapping_from)),
            &(|seed| random(seed)),
        )),
        GenerationMode::SpecialRandom(scale) => Box::new(random_triples(
            &EXAMPLE_SEED,
            &(|seed| special_random_unsigned_vecs(seed, scale)),
            &(|seed| u32s_geometric(seed, scale).map(usize::wrapping_from)),
            &(|seed| special_random_unsigned(seed)),
        )),
    }
}

// All pairs of `Vec<T>`, where `T` is unsigned, and `usize`, where the `usize` is no larger than
// the length of the `Vec`.
pub fn pairs_of_unsigned_vec_and_small_usize_var_1<T: PrimitiveUnsigned + Rand>(
    gm: GenerationMode,
) -> It<(Vec<T>, usize)> {
    Box::new(pairs_of_unsigned_vec_and_small_unsigned(gm).filter(|&(ref xs, u)| u <= xs.len()))
}

pub fn pairs_of_unsigned_vec_and_unsigned<T: PrimitiveUnsigned + Rand>(
    gm: GenerationMode,
) -> It<(Vec<T>, T)> {
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

// All pairs of `Vec<T>` and `T`, where `T` is unsigned, the `Vec` has length at least 2, and the
// most-significant bit of the `T` is set.
pub(crate) fn pairs_of_unsigned_vec_and_unsigned_var_1<T: PrimitiveUnsigned + Rand>(
    gm: GenerationMode,
) -> It<(Vec<T>, T)> {
    match gm {
        GenerationMode::Exhaustive => Box::new(exhaustive_pairs(
            exhaustive_vecs_min_length(2, exhaustive_unsigned()),
            range_up_increasing(T::ONE << (T::WIDTH - 1)),
        )),
        GenerationMode::Random(scale) => Box::new(random_pairs(
            &EXAMPLE_SEED,
            &(|seed| random_vecs_min_length(seed, scale, 2, &(|seed_2| random(seed_2)))),
            &(|seed| {
                random::<T>(seed).map(|mut u| {
                    u.set_bit(u64::from(T::WIDTH - 1));
                    u
                })
            }),
        )),
        GenerationMode::SpecialRandom(scale) => Box::new(random_pairs(
            &EXAMPLE_SEED,
            &(|seed| special_random_unsigned_vecs_min_length(seed, scale, 2)),
            &(|seed| {
                special_random_unsigned::<T>(seed).map(|mut u| {
                    u.set_bit(u64::from(T::WIDTH - 1));
                    u
                })
            }),
        )),
    }
}

pub fn pairs_of_nonempty_unsigned_vec_and_unsigned<T: PrimitiveUnsigned + Rand>(
    gm: GenerationMode,
) -> It<(Vec<T>, T)> {
    Box::new(pairs_of_unsigned_vec_and_unsigned(gm).filter(|&(ref xs, _)| !xs.is_empty()))
}

// All pairs of nonempty `Vec<T>` and `T`, where `T` is unsigned and the most-significant bit of the
// `T` is set.
pub fn pairs_of_nonempty_unsigned_vec_and_unsigned_var_1<T: PrimitiveUnsigned + Rand>(
    gm: GenerationMode,
) -> It<(Vec<T>, T)> {
    match gm {
        GenerationMode::Exhaustive => Box::new(exhaustive_pairs(
            exhaustive_vecs_min_length(1, exhaustive_unsigned()),
            range_up_increasing(T::ONE << (T::WIDTH - 1)),
        )),
        GenerationMode::Random(scale) => Box::new(random_pairs(
            &EXAMPLE_SEED,
            &(|seed| random_vecs_min_length(seed, scale, 1, &(|seed_2| random(seed_2)))),
            &(|seed| {
                random::<T>(seed).map(|mut u| {
                    u.set_bit(u64::from(T::WIDTH - 1));
                    u
                })
            }),
        )),
        GenerationMode::SpecialRandom(scale) => Box::new(random_pairs(
            &EXAMPLE_SEED,
            &(|seed| special_random_unsigned_vecs_min_length(seed, scale, 1)),
            &(|seed| {
                special_random_unsigned::<T>(seed).map(|mut u| {
                    u.set_bit(u64::from(T::WIDTH - 1));
                    u
                })
            }),
        )),
    }
}

// All pairs of nonempty `Vec<T>` and `T`, where `T` is unsigned, the most-significant bit of the
// `T` is unset, and the `T` is positive.
pub fn pairs_of_nonempty_unsigned_vec_and_positive_unsigned_var_1<
    T: PrimitiveUnsigned + Rand + SampleRange,
>(
    gm: GenerationMode,
) -> It<(Vec<T>, T)> {
    match gm {
        GenerationMode::Exhaustive => Box::new(exhaustive_pairs(
            exhaustive_vecs_min_length(1, exhaustive_unsigned()),
            range_increasing(T::ONE, (T::ONE << (T::WIDTH - 1)) - T::ONE),
        )),
        GenerationMode::Random(scale) => Box::new(random_pairs(
            &EXAMPLE_SEED,
            &(|seed| random_vecs_min_length(seed, scale, 1, &(|seed_2| random(seed_2)))),
            &(|seed| random_range::<T>(seed, T::ONE, (T::ONE << (T::WIDTH - 1)) - T::ONE)),
        )),
        GenerationMode::SpecialRandom(scale) => Box::new(random_pairs(
            &EXAMPLE_SEED,
            &(|seed| special_random_unsigned_vecs_min_length(seed, scale, 1)),
            &(|seed| {
                special_random_unsigned::<T>(seed)
                    .map(|mut u| {
                        u.clear_bit(u64::from(T::WIDTH - 1));
                        u
                    })
                    .filter(|&u| u != T::ZERO)
            }),
        )),
    }
}

// All pairs of nonempty `Vec<T>` and `T`, where `T` is unsigned, the two most-significant bit of
// the `T` are unset, and the `T` is positive.
pub fn pairs_of_nonempty_unsigned_vec_and_positive_unsigned_var_2<
    T: PrimitiveUnsigned + Rand + SampleRange,
>(
    gm: GenerationMode,
) -> It<(Vec<T>, T)> {
    match gm {
        GenerationMode::Exhaustive => Box::new(exhaustive_pairs(
            exhaustive_vecs_min_length(1, exhaustive_unsigned()),
            range_increasing(T::ONE, (T::ONE << (T::WIDTH - 2)) - T::ONE),
        )),
        GenerationMode::Random(scale) => Box::new(random_pairs(
            &EXAMPLE_SEED,
            &(|seed| random_vecs_min_length(seed, scale, 1, &(|seed_2| random(seed_2)))),
            &(|seed| random_range::<T>(seed, T::ONE, (T::ONE << (T::WIDTH - 2)) - T::ONE)),
        )),
        GenerationMode::SpecialRandom(scale) => Box::new(random_pairs(
            &EXAMPLE_SEED,
            &(|seed| special_random_unsigned_vecs_min_length(seed, scale, 1)),
            &(|seed| {
                special_random_unsigned::<T>(seed)
                    .map(|mut u| {
                        u.clear_bit(u64::from(T::WIDTH - 1));
                        u.clear_bit(u64::from(T::WIDTH - 2));
                        u
                    })
                    .filter(|&u| u != T::ZERO)
            }),
        )),
    }
}

fn pairs_of_unsigned_vec_and_positive_unsigned<T: PrimitiveUnsigned + Rand>(
    gm: GenerationMode,
) -> It<(Vec<T>, T)> {
    match gm {
        GenerationMode::Exhaustive => Box::new(exhaustive_pairs(
            exhaustive_vecs(exhaustive_unsigned()),
            exhaustive_positive(),
        )),
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
pub fn pairs_of_unsigned_vec_and_positive_unsigned_var_1<T: PrimitiveUnsigned + Rand>(
    gm: GenerationMode,
) -> It<(Vec<T>, T)> {
    match gm {
        GenerationMode::Exhaustive => Box::new(exhaustive_pairs(
            exhaustive_vecs_min_length(2, exhaustive_unsigned()),
            exhaustive_positive(),
        )),
        GenerationMode::Random(scale) => Box::new(random_pairs(
            &EXAMPLE_SEED,
            &(|seed| random_vecs_min_length(seed, scale, 2, &(|seed_2| random(seed_2)))),
            &(|seed| random_positive_unsigned(seed)),
        )),
        GenerationMode::SpecialRandom(scale) => Box::new(random_pairs(
            &EXAMPLE_SEED,
            &(|seed| special_random_unsigned_vecs_min_length(seed, scale, 2)),
            &(|seed| special_random_positive_unsigned(seed)),
        )),
    }
}

// All pairs of `Vec<Limb>` and positive `Limb`, where the `Vec` is nonempty and represents a
// `Natural` divisible by the `Limb`.
pub fn pairs_of_limb_vec_and_positive_limb_var_2(gm: GenerationMode) -> It<(Vec<Limb>, Limb)> {
    Box::new(
        pairs_of_unsigned_vec_and_positive_unsigned(gm)
            .filter(|(ref limbs, _)| !limbs.is_empty())
            .map(|(limbs, limb)| (limbs_mul_limb(&limbs, limb), limb)),
    )
}

// All pairs of `Vec<T>` and `T`, where `T` is unsigned, the `Vec` has length at least 2, the most-
// significant bit of the `T` is unset, and the `T` is positive.
pub(crate) fn pairs_of_unsigned_vec_and_positive_unsigned_var_3<
    T: PrimitiveUnsigned + Rand + SampleRange,
>(
    gm: GenerationMode,
) -> It<(Vec<T>, T)> {
    match gm {
        GenerationMode::Exhaustive => Box::new(exhaustive_pairs(
            exhaustive_vecs_min_length(2, exhaustive_unsigned()),
            range_increasing(T::ONE, (T::ONE << (T::WIDTH - 1)) - T::ONE),
        )),
        GenerationMode::Random(scale) => Box::new(random_pairs(
            &EXAMPLE_SEED,
            &(|seed| random_vecs_min_length(seed, scale, 2, &(|seed_2| random(seed_2)))),
            &(|seed| random_range::<T>(seed, T::ONE, (T::ONE << (T::WIDTH - 1)) - T::ONE)),
        )),
        GenerationMode::SpecialRandom(scale) => Box::new(random_pairs(
            &EXAMPLE_SEED,
            &(|seed| special_random_unsigned_vecs_min_length(seed, scale, 2)),
            &(|seed| {
                special_random_unsigned::<T>(seed)
                    .map(|mut u| {
                        u.clear_bit(u64::from(T::WIDTH - 1));
                        u
                    })
                    .filter(|&u| u != T::ZERO)
            }),
        )),
    }
}

// All pairs of `Vec<T>` where `T` is unsigned, and a `u32` between 1 and 31, inclusive.
pub fn pairs_of_unsigned_vec_and_limb_var_1<T: PrimitiveUnsigned + Rand>(
    gm: GenerationMode,
) -> It<(Vec<T>, u32)> {
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
pub fn pairs_of_unsigned_vec_and_limb_var_2<T: PrimitiveUnsigned + Rand>(
    gm: GenerationMode,
) -> It<(Vec<T>, u32)> {
    Box::new(pairs_of_unsigned_vec_and_limb_var_1(gm).filter(|&(ref xs, _)| !xs.is_empty()))
}

pub fn pairs_of_unsigned_vec_and_small_unsigned<
    T: PrimitiveUnsigned + Rand,
    U: PrimitiveUnsigned + Rand,
>(
    gm: GenerationMode,
) -> It<(Vec<T>, U)> {
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
    T: PrimitiveUnsigned + Rand,
    U: PrimitiveUnsigned + Rand,
>(
    gm: GenerationMode,
) -> It<(Vec<T>, U)> {
    Box::new(
        pairs_of_unsigned_vec_and_small_unsigned(gm)
            .filter(|&(ref limbs, _)| !limbs_test_zero(limbs)),
    )
}

// All pairs of `Vec<Limb>` and small `u64` where the u64 is less than `Limb::WIDTH` * the length of
// the `Vec`.
pub fn pairs_of_limb_vec_and_small_u64_var_2(gm: GenerationMode) -> It<(Vec<Limb>, u64)> {
    Box::new(
        pairs_of_unsigned_vec_and_small_unsigned(gm).filter(|&(ref limbs, index)| {
            index < u64::wrapping_from(limbs.len()) << Limb::LOG_WIDTH
        }),
    )
}

// All pairs of `Vec<Limb>` and small `u64` where `limbs_slice_clear_bit_neg` applied to the `Vec`
// and `u64` doesn't panic.
pub fn pairs_of_limb_vec_and_small_u64_var_3(gm: GenerationMode) -> It<(Vec<Limb>, u64)> {
    Box::new(
        pairs_of_unsigned_vec_and_small_unsigned_var_1(gm).filter(|&(ref limbs, index)| {
            let mut mut_limbs = limbs.clone();
            limbs_vec_clear_bit_neg(&mut mut_limbs, index);
            mut_limbs.len() == limbs.len()
        }),
    )
}

// All pairs of `Vec<u32>` and `T`, where `T` is unsigned and the `Vec<T>` is nonempty and doesn't
// only contain zeros.
pub fn pairs_of_limb_vec_and_limb_var_1<T: PrimitiveUnsigned + Rand>(
    gm: GenerationMode,
) -> It<(Vec<T>, T)> {
    Box::new(
        pairs_of_unsigned_vec_and_unsigned(gm).filter(|&(ref limbs, _)| !limbs_test_zero(limbs)),
    )
}

// All pairs of `Vec<T>` and positive `T`, where `T` is unsigned and the `Vec<T>` is nonempty and
// doesn't only contain zeros.
pub fn pairs_of_limb_vec_and_positive_limb_var_1<T: PrimitiveUnsigned + Rand>(
    gm: GenerationMode,
) -> It<(Vec<T>, T)> {
    Box::new(
        pairs_of_unsigned_vec_and_unsigned(gm)
            .filter(|&(ref limbs, limb)| limb != T::ZERO && !limbs_test_zero(limbs)),
    )
}

pub fn vecs_of_bool(gm: GenerationMode) -> It<Vec<bool>> {
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
pub fn vecs_of_bool_var_1(gm: GenerationMode) -> It<Vec<bool>> {
    Box::new(vecs_of_bool(gm).filter(|bits| bits.iter().any(|&bit| bit)))
}

fn triples_of_unsigned_vec_unsigned_vec_and_unsigned<T: PrimitiveUnsigned + Rand>(
    gm: GenerationMode,
) -> It<(Vec<T>, Vec<T>, T)> {
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

fn triples_of_unsigned_vec_unsigned_vec_and_positive_unsigned<T: PrimitiveUnsigned + Rand>(
    gm: GenerationMode,
) -> It<(Vec<T>, Vec<T>, T)> {
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
pub fn triples_of_unsigned_vec_unsigned_vec_and_unsigned_var_1<T: PrimitiveUnsigned + Rand>(
    gm: GenerationMode,
) -> It<(Vec<T>, Vec<T>, T)> {
    Box::new(
        triples_of_unsigned_vec_unsigned_vec_and_unsigned(gm)
            .filter(|&(ref out, ref in_limbs, _)| out.len() >= in_limbs.len()),
    )
}

// All triples of `Vec<T>`, `Vec<T>`, and `T` where `T` is unsigned, the first `Vec` is at least as
// long as the second, and the second is nonempty.
pub fn triples_of_unsigned_vec_unsigned_vec_and_unsigned_var_2<T: PrimitiveUnsigned + Rand>(
    gm: GenerationMode,
) -> It<(Vec<T>, Vec<T>, T)> {
    Box::new(
        triples_of_unsigned_vec_unsigned_vec_and_unsigned(gm).filter(
            |&(ref out, ref in_limbs, _)| !in_limbs.is_empty() && out.len() >= in_limbs.len(),
        ),
    )
}

// All triples of `Vec<T>`, `Vec<T>`, and `T` where `T` is unsigned, the first `Vec` is at least as
// long as the second, and the second doesn't only contain zeros.
pub fn triples_of_unsigned_vec_unsigned_vec_and_unsigned_var_3<T: PrimitiveUnsigned + Rand>(
    gm: GenerationMode,
) -> It<(Vec<T>, Vec<T>, T)> {
    Box::new(
        triples_of_unsigned_vec_unsigned_vec_and_unsigned(gm).filter(
            |&(ref out, ref in_limbs, _)| out.len() >= in_limbs.len() && !limbs_test_zero(in_limbs),
        ),
    )
}

// All triples of `Vec<T>`, `Vec<T>`, and `u32` where `T` is unsigned and the `u32` is between 1 and
// 31, inclusive.
fn triples_of_unsigned_vec_unsigned_vec_and_limb_var_4<T: PrimitiveUnsigned + Rand>(
    gm: GenerationMode,
) -> It<(Vec<T>, Vec<T>, u32)> {
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
pub fn triples_of_unsigned_vec_unsigned_vec_and_limb_var_5<T: PrimitiveUnsigned + Rand>(
    gm: GenerationMode,
) -> It<(Vec<T>, Vec<T>, u32)> {
    Box::new(
        triples_of_unsigned_vec_unsigned_vec_and_limb_var_4(gm)
            .filter(|&(ref out, ref in_limbs, _)| out.len() >= in_limbs.len()),
    )
}

// All triples of `Vec<T>`, `Vec<T>`, and `u32` where `T` is unsigned, the first `Vec` is at least
// as long as the second, the second is nonempty, and the `u32` is between 1 and 31, inclusive.
pub fn triples_of_unsigned_vec_unsigned_vec_and_limb_var_6<T: PrimitiveUnsigned + Rand>(
    gm: GenerationMode,
) -> It<(Vec<T>, Vec<T>, u32)> {
    Box::new(
        triples_of_unsigned_vec_unsigned_vec_and_limb_var_4(gm).filter(
            |&(ref out, ref in_limbs, _)| !in_limbs.is_empty() && out.len() >= in_limbs.len(),
        ),
    )
}

// All triples of `Vec<T>`, `Vec<T>`, and `T` where `T` is unsigned and the `Vec`s have the same
// length.
pub fn triples_of_unsigned_vec_unsigned_vec_and_unsigned_var_7<T: PrimitiveUnsigned + Rand>(
    gm: GenerationMode,
) -> It<(Vec<T>, Vec<T>, T)> {
    Box::new(
        triples_of_unsigned_vec_unsigned_vec_and_unsigned(gm)
            .filter(|&(ref xs, ref ys, _)| xs.len() == ys.len()),
    )
}

// All triples of `Vec<T>`, and `Vec<T>`, and `T` that meet the preconditions for
// `limbs_eq_mod_limb_ref_ref`.
pub fn triples_of_unsigned_vec_unsigned_vec_and_unsigned_var_8<T: PrimitiveUnsigned + Rand>(
    gm: GenerationMode,
) -> It<(Vec<T>, Vec<T>, T)> {
    let ps: It<((Vec<T>, Vec<T>), T)> = match gm {
        GenerationMode::Exhaustive => Box::new(sqrt_pairs(
            exhaustive_pairs_from_single(
                exhaustive_vecs_min_length(2, exhaustive_unsigned())
                    .filter(|xs| *xs.last().unwrap() != T::ZERO),
            ),
            exhaustive_positive(),
        )),
        GenerationMode::Random(scale) => Box::new(random_pairs(
            &EXAMPLE_SEED,
            &(|seed| {
                random_pairs_from_single(
                    random_vecs_min_length(seed, scale, 2, &(|seed_2| random(seed_2)))
                        .filter(|xs| *xs.last().unwrap() != T::ZERO),
                )
            }),
            &(|seed| random_positive_unsigned(seed)),
        )),
        GenerationMode::SpecialRandom(scale) => Box::new(random_pairs(
            &EXAMPLE_SEED,
            &(|seed| {
                random_pairs_from_single(
                    special_random_unsigned_vecs_min_length(seed, scale, 2)
                        .filter(|xs| *xs.last().unwrap() != T::ZERO),
                )
            }),
            &(|seed| special_random_positive_unsigned(seed)),
        )),
    };
    reshape_2_1_to_3(ps)
}

// All triples of `Vec<T>`, and `Vec<T>`, and `T` that meet the preconditions for
// `limbs_eq_mod_limb_ref_ref`, where the `Natural` represented by the first `Vec` is equal to the
// `Natural` represented by the second `Vec` mod the `T`.
pub fn triples_of_unsigned_vec_unsigned_vec_and_unsigned_var_9(
    gm: GenerationMode,
) -> It<(Vec<Limb>, Vec<Limb>, Limb)> {
    Box::new(
        triples_of_unsigned_vec_unsigned_vec_and_unsigned_var_8(gm).map(|(mut xs, ys, modulus)| {
            limbs_vec_mul_limb_in_place(&mut xs, modulus);
            if xs.last() == Some(&0) {
                xs.pop();
            }
            limbs_vec_add_in_place_left(&mut xs, &ys);
            (xs, ys, modulus)
        }),
    )
}

// All triples of `Vec<T>`, and `Vec<T>`, and `T` that meet the preconditions for
// `limbs_eq_mod_limb_ref_ref`, where the `Natural` represented by the first `Vec` is not equal to
// the `Natural` represented by the second `Vec` mod the `T`.
pub fn triples_of_unsigned_vec_unsigned_vec_and_unsigned_var_10(
    gm: GenerationMode,
) -> It<(Vec<Limb>, Vec<Limb>, Limb)> {
    Box::new(
        triples_of_unsigned_vec_unsigned_vec_and_unsigned_var_8::<Limb>(gm)
            .filter(|(xs, ys, modulus)| !limbs_eq_mod_limb_ref_ref(&*xs, &*ys, *modulus)),
    )
}

// All triples of `Vec<T>`, `Vec<T>`, and `T` where `T` is unsigned, the first `Vec` is at least as
// long as the second, and the length of the second `Vec` is greater than 1.
pub fn triples_of_unsigned_vec_unsigned_vec_and_positive_unsigned_var_1<
    T: PrimitiveUnsigned + Rand,
>(
    gm: GenerationMode,
) -> It<(Vec<T>, Vec<T>, T)> {
    Box::new(
        triples_of_unsigned_vec_unsigned_vec_and_positive_unsigned(gm).filter(
            |&(ref out, ref in_limbs, _)| out.len() >= in_limbs.len() && in_limbs.len() > 1,
        ),
    )
}

// All triples of `Vec<Limb>`, `Vec<Limb>`, and positive `Limb`, where the first `Vec` is at least
// as long as the second and the second `Vec` is nonempty and represents a `Natural` divisible by
// the `Limb`.
pub fn triples_of_limb_vec_limb_vec_and_positive_limb_var_2(
    gm: GenerationMode,
) -> It<(Vec<Limb>, Vec<Limb>, Limb)> {
    Box::new(
        triples_of_unsigned_vec_unsigned_vec_and_positive_unsigned(gm)
            .map(|(out, in_limbs, limb)| (out, limbs_mul_limb(&in_limbs, limb), limb))
            .filter(|(ref out, ref in_limbs, _)| {
                out.len() >= in_limbs.len() && !in_limbs.is_empty()
            }),
    )
}

// All triples of `Vec<T>`, `Vec<T>`, and positive `T`, where `T` is unsigned and the `Vec`s are
// nonempty and have no trailing zeros.
pub fn triples_of_unsigned_vec_unsigned_vec_and_positive_unsigned_var_3<
    T: PrimitiveUnsigned + Rand,
>(
    gm: GenerationMode,
) -> It<(Vec<T>, Vec<T>, T)> {
    Box::new(
        triples_of_unsigned_vec_unsigned_vec_and_positive_unsigned(gm).filter(
            |&(ref xs, ref ys, _)| {
                !xs.is_empty()
                    && !ys.is_empty()
                    && *xs.last().unwrap() != T::ZERO
                    && *ys.last().unwrap() != T::ZERO
            },
        ),
    )
}

fn triples_of_unsigned_vec_unsigned_vec_and_small_unsigned<
    T: PrimitiveUnsigned + Rand,
    U: PrimitiveUnsigned,
>(
    gm: GenerationMode,
) -> It<(Vec<T>, Vec<T>, U)> {
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

pub fn triples_of_unsigned_vec_unsigned_and_unsigned<T: PrimitiveUnsigned + Rand>(
    gm: GenerationMode,
) -> It<(Vec<T>, T, T)> {
    match gm {
        GenerationMode::Exhaustive => Box::new(exhaustive_triples(
            exhaustive_vecs(exhaustive_unsigned()),
            exhaustive_unsigned(),
            exhaustive_unsigned(),
        )),
        GenerationMode::Random(scale) => Box::new(random_triples(
            &EXAMPLE_SEED,
            &(|seed| random_vecs(seed, scale, &(|seed_2| random(seed_2)))),
            &(|seed| random(seed)),
            &(|seed| random(seed)),
        )),
        GenerationMode::SpecialRandom(scale) => Box::new(random_triples(
            &EXAMPLE_SEED,
            &(|seed| special_random_unsigned_vecs(seed, scale)),
            &(|seed| special_random_unsigned(seed)),
            &(|seed| special_random_unsigned(seed)),
        )),
    }
}

// All triples of `Vec<T>`, `Vec<T>`, and small `U` where `T` and `U` are unsigned and the `Vec`s
// are nonempty and have no trailing zeros.
pub fn triples_of_unsigned_vec_unsigned_vec_and_small_unsigned_var_1<
    T: PrimitiveUnsigned + Rand,
    U: PrimitiveUnsigned,
>(
    gm: GenerationMode,
) -> It<(Vec<T>, Vec<T>, U)> {
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

// All triples of `Vec<T>`, `Vec<T>`, and small unsigned, where `T` is unsigned, the second `Vec` is
// as long as the first, and the `usize` is less than or equal to the length of the first `Vec`.
pub fn triples_of_unsigned_vec_unsigned_and_small_usize_var_1<T: PrimitiveUnsigned + Rand>(
    gm: GenerationMode,
) -> It<(Vec<T>, Vec<T>, usize)> {
    let ts: It<((Vec<T>, Vec<T>), usize)> = match gm {
        GenerationMode::Exhaustive => Box::new(sqrt_pairs(
            pairs_of_unsigned_vec_var_1(gm),
            exhaustive_unsigned(),
        )),
        GenerationMode::Random(scale) => Box::new(random_pairs(
            &EXAMPLE_SEED,
            &(|seed| pairs_of_unsigned_vec_var_1_with_seed(gm, seed)),
            &(|seed| u32s_geometric(seed, scale).flat_map(usize::checked_from)),
        )),
        GenerationMode::SpecialRandom(scale) => Box::new(random_pairs(
            &EXAMPLE_SEED,
            &(|seed| pairs_of_unsigned_vec_var_1_with_seed(gm, seed)),
            &(|seed| u32s_geometric(seed, scale).flat_map(usize::checked_from)),
        )),
    };
    reshape_2_1_to_3(Box::new(ts.filter(|&((ref xs, _), len)| len <= xs.len())))
}

// All triples of `Vec<T>`, `T`, and `T`, where `T` is unsigned, the second `T` is positive, the
// `Vec` has at least two elements, and the `Vec`'s last element is nonzero.
pub fn triples_of_unsigned_vec_unsigned_and_positive_unsigned_var_1<T: PrimitiveUnsigned + Rand>(
    gm: GenerationMode,
) -> It<(Vec<T>, T, T)> {
    match gm {
        GenerationMode::Exhaustive => Box::new(exhaustive_triples(
            exhaustive_vecs_min_length(2, exhaustive_unsigned())
                .filter(|limbs| *limbs.last().unwrap() != T::ZERO),
            exhaustive_unsigned(),
            exhaustive_positive(),
        )),
        GenerationMode::Random(scale) => Box::new(random_triples(
            &EXAMPLE_SEED,
            &(|seed| {
                random_vecs_min_length(seed, scale, 2, &(|seed_2| random(seed_2)))
                    .filter(|limbs| *limbs.last().unwrap() != T::ZERO)
            }),
            &(|seed| random(seed)),
            &(|seed| random_positive_unsigned(seed)),
        )),
        GenerationMode::SpecialRandom(scale) => Box::new(random_triples(
            &EXAMPLE_SEED,
            &(|seed| {
                special_random_unsigned_vecs_min_length(seed, scale, 2)
                    .filter(|limbs| *limbs.last().unwrap() != T::ZERO)
            }),
            &(|seed| special_random_unsigned(seed)),
            &(|seed| special_random_positive_unsigned(seed)),
        )),
    }
}

// All triples of `Vec<T>`, `T`, and `T`, where `T` is unsigned, the second `T` is positive, the
// `Vec` is nonempty, and the `Vec`'s last element is nonzero.
fn triples_of_unsigned_vec_unsigned_and_positive_unsigned_var_2<T: PrimitiveUnsigned + Rand>(
    gm: GenerationMode,
) -> It<(Vec<T>, T, T)> {
    match gm {
        GenerationMode::Exhaustive => Box::new(exhaustive_triples(
            exhaustive_vecs_min_length(1, exhaustive_unsigned())
                .filter(|limbs| *limbs.last().unwrap() != T::ZERO),
            exhaustive_unsigned(),
            exhaustive_positive(),
        )),
        GenerationMode::Random(scale) => Box::new(random_triples(
            &EXAMPLE_SEED,
            &(|seed| {
                random_vecs_min_length(seed, scale, 1, &(|seed_2| random(seed_2)))
                    .filter(|limbs| *limbs.last().unwrap() != T::ZERO)
            }),
            &(|seed| random(seed)),
            &(|seed| random_positive_unsigned(seed)),
        )),
        GenerationMode::SpecialRandom(scale) => Box::new(random_triples(
            &EXAMPLE_SEED,
            &(|seed| {
                special_random_unsigned_vecs_min_length(seed, scale, 1)
                    .filter(|limbs| *limbs.last().unwrap() != T::ZERO)
            }),
            &(|seed| special_random_unsigned(seed)),
            &(|seed| special_random_positive_unsigned(seed)),
        )),
    }
}

// All triples of `Vec<T>`, `T`, and `T`, where `T` is unsigned, the second `T` is positive, the
// `Vec` has at least two elements, the `Vec`'s last element is nonzero, and the `Natural`
// represented by the `Vec` is equal to the first `T` mod the second `T`.
pub fn triples_of_limb_vec_limb_and_positive_limb_var_3(
    gm: GenerationMode,
) -> It<(Vec<Limb>, Limb, Limb)> {
    Box::new(
        triples_of_unsigned_vec_unsigned_and_positive_unsigned_var_2(gm)
            .map(|(mut limbs, limb, modulus)| {
                let carry = limbs_slice_mul_limb_in_place(&mut limbs, modulus);
                if carry != 0 {
                    limbs.push(carry);
                } else if *limbs.last().unwrap() == 0 {
                    limbs.pop();
                }
                limbs_vec_add_limb_in_place(&mut limbs, limb);
                (limbs, limb, modulus)
            })
            .filter(|(limbs, _, _)| limbs.len() > 1),
    )
}

// All triples of `Vec<T>`, `T`, and `T`, where `T` is unsigned, the second `T` is positive, the
// `Vec` has at least two elements, the `Vec`'s last element is nonzero, and the `Natural`
// represented by the `Vec` is not equal to the first `T` mod the second `T`.
pub fn triples_of_limb_vec_limb_and_positive_limb_var_4(
    gm: GenerationMode,
) -> It<(Vec<Limb>, Limb, Limb)> {
    Box::new(
        triples_of_unsigned_vec_unsigned_and_positive_unsigned_var_1::<Limb>(gm)
            .filter(|(limbs, limb, modulus)| !limbs_eq_limb_mod_limb(&*limbs, *limb, *modulus)),
    )
}

// All triples of `Vec<T>`, T, and small `U`, where `T` and `U` are unsigned, the `Vec` is
// non-empty, and its last element is nonzero.
pub(crate) fn triples_of_unsigned_vec_unsigned_and_small_unsigned_var_1<
    T: PrimitiveUnsigned + Rand,
    U: PrimitiveUnsigned,
>(
    gm: GenerationMode,
) -> It<(Vec<T>, T, U)> {
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
    T: PrimitiveUnsigned + Rand,
    U: PrimitiveUnsigned,
>(
    gm: GenerationMode,
) -> It<(Vec<T>, T, U)> {
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

fn triples_of_unsigned_vec_usize_and_unsigned_vec<T: PrimitiveUnsigned + Rand>(
    gm: GenerationMode,
) -> It<(Vec<T>, usize, Vec<T>)> {
    match gm {
        GenerationMode::Exhaustive => Box::new(exhaustive_triples(
            exhaustive_vecs(exhaustive_unsigned()),
            exhaustive_unsigned::<u64>().map(usize::wrapping_from),
            exhaustive_vecs(exhaustive_unsigned()),
        )),
        GenerationMode::Random(scale) => Box::new(random_triples(
            &EXAMPLE_SEED,
            &(|seed| random_vecs(seed, scale, &(|seed_2| random(seed_2)))),
            &(|seed| u32s_geometric(seed, scale).map(usize::wrapping_from)),
            &(|seed| random_vecs(seed, scale, &(|seed_2| random(seed_2)))),
        )),
        GenerationMode::SpecialRandom(scale) => Box::new(random_triples(
            &EXAMPLE_SEED,
            &(|seed| special_random_unsigned_vecs(seed, scale)),
            &(|seed| u32s_geometric(seed, scale).map(usize::wrapping_from)),
            &(|seed| special_random_unsigned_vecs(seed, scale)),
        )),
    }
}

// All triples of `Vec<T>`, usize, and `Vec<T>`, where `T` is unsigned, the first `Vec` is at least
// as long as the second, and the `usize` is no greater than the length of the second `Vec`.
pub fn triples_of_unsigned_vec_usize_and_unsigned_vec_var_1<T: PrimitiveUnsigned + Rand>(
    gm: GenerationMode,
) -> It<(Vec<T>, usize, Vec<T>)> {
    Box::new(
        triples_of_unsigned_vec_usize_and_unsigned_vec(gm)
            .filter(|&(ref xs, y, ref zs)| xs.len() >= zs.len() && y <= zs.len()),
    )
}

// All triples of `Vec<T>`, `T`, and `Vec<T>` that meet the preconditions for
// `limbs_eq_limb_mod_ref_ref`.
pub fn triples_of_unsigned_vec_unsigned_and_unsigned_vec_var_1<T: PrimitiveUnsigned + Rand>(
    gm: GenerationMode,
) -> It<(Vec<T>, T, Vec<T>)> {
    permute_1_3_2(triples_of_unsigned_vec_unsigned_vec_and_unsigned_var_8(gm))
}

// All triples of `Vec<T>`, `T`, and `Vec<T>` that meet the preconditions for
// `limbs_eq_limb_mod_ref_ref`, where the `Natural` represented by the first `Vec` is equal to the
// `T` mod the `Natural` represented by the second `Vec`.
pub fn triples_of_unsigned_vec_unsigned_and_unsigned_vec_var_2(
    gm: GenerationMode,
) -> It<(Vec<Limb>, Limb, Vec<Limb>)> {
    Box::new(
        triples_of_unsigned_vec_unsigned_and_unsigned_vec_var_1(gm).map(|(xs, y, modulus)| {
            let mut product_limbs = if xs.is_empty() {
                Vec::new()
            } else {
                limbs_mul(&xs, &modulus)
            };
            if product_limbs.last() == Some(&0) {
                product_limbs.pop();
            }
            limbs_vec_add_limb_in_place(&mut product_limbs, y);
            (product_limbs, y, modulus)
        }),
    )
}

// All triples of `Vec<T>`, `T`, and `Vec<T>` that meet the preconditions for
// `limbs_eq_limb_mod_ref_ref`, where the `Natural` represented by the first `Vec` is not equal to
// the `T` mod the `Natural` represented by the second `Vec`.
pub fn triples_of_unsigned_vec_unsigned_and_unsigned_vec_var_3(
    gm: GenerationMode,
) -> It<(Vec<Limb>, Limb, Vec<Limb>)> {
    Box::new(
        triples_of_unsigned_vec_unsigned_and_unsigned_vec_var_1::<Limb>(gm)
            .filter(|(xs, y, modulus)| !limbs_eq_limb_mod_ref_ref(&*xs, *y, &*modulus)),
    )
}

fn triples_of_unsigned_vec_small_unsigned_and_rounding_mode<
    T: PrimitiveUnsigned + Rand,
    U: PrimitiveUnsigned,
>(
    gm: GenerationMode,
) -> It<(Vec<T>, U, RoundingMode)> {
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
    T: PrimitiveUnsigned + Rand,
    U: PrimitiveUnsigned,
>(
    gm: GenerationMode,
) -> It<(Vec<T>, U, RoundingMode)> {
    Box::new(
        triples_of_unsigned_vec_small_unsigned_and_rounding_mode(gm)
            .filter(|&(ref limbs, _, _)| !limbs_test_zero(limbs)),
    )
}

fn triples_of_unsigned_unsigned_vec_and_rounding_mode<T: PrimitiveUnsigned + Rand>(
    gm: GenerationMode,
) -> It<(T, Vec<T>, RoundingMode)> {
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
pub(crate) fn triples_of_unsigned_unsigned_vec_and_rounding_mode_var_1<
    T: PrimitiveUnsigned + Rand,
>(
    gm: GenerationMode,
) -> It<(T, Vec<T>, RoundingMode)> {
    Box::new(
        triples_of_unsigned_unsigned_vec_and_rounding_mode(gm)
            .filter(|&(_, ref limbs, _)| limbs.len() > 1 && *limbs.last().unwrap() != T::ZERO),
    )
}

fn triples_of_unsigned_small_unsigned_and_rounding_mode<
    T: PrimitiveUnsigned + Rand,
    U: PrimitiveUnsigned,
>(
    gm: GenerationMode,
) -> It<(T, U, RoundingMode)> {
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
    T: PrimitiveUnsigned + Rand,
    U: PrimitiveUnsigned,
>(
    gm: GenerationMode,
) -> It<(T, U, RoundingMode)>
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

fn triples_of_signed_small_unsigned_and_rounding_mode<
    T: PrimitiveSigned + Rand,
    U: PrimitiveUnsigned + Rand,
>(
    gm: GenerationMode,
) -> It<(T, U, RoundingMode)>
where
    T::UnsignedOfEqualWidth: Rand,
    T: WrappingFrom<<T as PrimitiveSigned>::UnsignedOfEqualWidth>,
{
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
    T: PrimitiveSigned + Rand,
    U: PrimitiveUnsigned + Rand,
>(
    gm: GenerationMode,
) -> It<(T, U, RoundingMode)>
where
    T: Shl<U, Output = T>,
    T: Shr<U, Output = T>,
    T::UnsignedOfEqualWidth: Rand,
    T: WrappingFrom<<T as PrimitiveSigned>::UnsignedOfEqualWidth>,
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

pub fn strings(gm: NoSpecialGenerationMode) -> It<String> {
    match gm {
        NoSpecialGenerationMode::Exhaustive => Box::new(exhaustive_strings()),
        NoSpecialGenerationMode::Random(scale) => Box::new(random_strings(&EXAMPLE_SEED, scale)),
    }
}

pub(crate) fn ascii_strings(gm: NoSpecialGenerationMode) -> It<String> {
    match gm {
        NoSpecialGenerationMode::Exhaustive => {
            Box::new(exhaustive_strings_with_chars(exhaustive_ascii_chars()))
        }
        NoSpecialGenerationMode::Random(scale) => {
            Box::new(random_strings_with_chars(&EXAMPLE_SEED, scale, &|seed| {
                random_ascii_chars(seed)
            }))
        }
    }
}

pub const ROUNDING_MODE_CHARS: &str = "CDEFNUacegilnoprstwx";

// All `Strings` with characters that appear in the `String` representations of `RoundingMode`s
pub fn strings_var_1(gm: NoSpecialGenerationMode) -> It<String> {
    match gm {
        NoSpecialGenerationMode::Exhaustive => {
            Box::new(exhaustive_strings_with_chars(ROUNDING_MODE_CHARS.chars()))
        }
        NoSpecialGenerationMode::Random(scale) => {
            Box::new(random_strings_with_chars(&EXAMPLE_SEED, scale, &|seed| {
                random_from_vector(seed, ROUNDING_MODE_CHARS.chars().collect())
            }))
        }
    }
}

pub fn pairs_of_strings(gm: NoSpecialGenerationMode) -> It<(String, String)> {
    match gm {
        NoSpecialGenerationMode::Exhaustive => {
            Box::new(exhaustive_pairs_from_single(exhaustive_strings()))
        }
        NoSpecialGenerationMode::Random(scale) => Box::new(random_pairs_from_single(
            random_strings(&EXAMPLE_SEED, scale),
        )),
    }
}

pub(crate) fn pairs_of_ascii_strings(gm: NoSpecialGenerationMode) -> It<(String, String)> {
    match gm {
        NoSpecialGenerationMode::Exhaustive => Box::new(exhaustive_pairs_from_single(
            exhaustive_strings_with_chars(exhaustive_ascii_chars()),
        )),
        NoSpecialGenerationMode::Random(scale) => Box::new(random_pairs_from_single(
            random_strings_with_chars(&EXAMPLE_SEED, scale, &|seed| random_ascii_chars(seed)),
        )),
    }
}
