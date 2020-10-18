use std::char;
use std::cmp::{max, Ordering};
use std::iter::repeat;
use std::ops::{Shl, Shr};

use itertools::Itertools;
use malachite_base::bools::exhaustive::exhaustive_bools;
use malachite_base::chars::constants::NUMBER_OF_CHARS;
use malachite_base::chars::exhaustive::{exhaustive_ascii_chars, exhaustive_chars};
use malachite_base::num::arithmetic::traits::{
    ArithmeticCheckedShl, ArithmeticCheckedShr, CheckedNeg, DivRound, EqMod, ModPowerOfTwo, Parity,
    PowerOfTwo, RoundToMultiple, UnsignedAbs,
};
use malachite_base::num::basic::integers::PrimitiveInt;
use malachite_base::num::basic::signeds::PrimitiveSigned;
use malachite_base::num::basic::traits::Zero;
use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base::num::conversion::traits::{
    CheckedFrom, ConvertibleFrom, ExactFrom, HasHalf, JoinHalves, SplitInHalf, WrappingFrom,
};
use malachite_base::num::exhaustive::{
    exhaustive_natural_signeds, exhaustive_negative_signeds, exhaustive_nonzero_signeds,
    exhaustive_positive_primitive_ints, exhaustive_signeds, exhaustive_unsigneds,
};
use malachite_base::num::exhaustive::{
    exhaustive_signed_range, primitive_int_increasing_inclusive_range,
    primitive_int_increasing_range,
};
use malachite_base::num::logic::traits::{
    BitAccess, BitBlockAccess, BitConvertible, BitIterable, LeadingZeros, SignificantBits,
};
use malachite_base::rounding_modes::exhaustive::exhaustive_rounding_modes;
use malachite_base::rounding_modes::RoundingMode;
use malachite_base::slices::{slice_test_zero, slice_trailing_zeros};
use malachite_base::tuples::exhaustive::{lex_pairs, lex_triples};
use malachite_base::vecs::exhaustive::exhaustive_fixed_length_vecs_from_single;
use malachite_base_test_util::generators::common::It;
use malachite_base_test_util::num::arithmetic::mod_mul::limbs_invert_limb_naive;
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
use malachite_nz::natural::arithmetic::mod_mul::_limbs_precompute_mod_mul_two_limbs;
use malachite_nz::natural::arithmetic::mod_power_of_two::limbs_slice_mod_power_of_two_in_place;
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
use malachite_nz::natural::arithmetic::square::{
    _limbs_square_to_out_toom_3_input_size_valid, _limbs_square_to_out_toom_4_input_size_valid,
    _limbs_square_to_out_toom_6_input_size_valid, _limbs_square_to_out_toom_8_input_size_valid,
};
use malachite_nz::natural::arithmetic::sub::{limbs_sub_in_place_left, limbs_sub_limb_in_place};
use malachite_nz::natural::comparison::ord::limbs_cmp;
use malachite_nz::natural::Natural;
use malachite_nz::platform::{Limb, SQR_TOOM2_THRESHOLD};
use rand::distributions::range::SampleRange;
use rand::distributions::{IndependentSample, Range};
use rand::{IsaacRng, Rand, Rng, SeedableRng};
use rust_wheels::iterators::chars::random_ascii_chars;
use rust_wheels::iterators::common::{scramble, EXAMPLE_SEED};
use rust_wheels::iterators::dependent_pairs::{
    dependent_pairs, exhaustive_dependent_pairs_infinite_sqrt, random_dependent_pairs,
};
use rust_wheels::iterators::general::{random, random_from_vector};
use rust_wheels::iterators::integers_geometric::{
    i32s_geometric, positive_u32s_geometric, range_up_geometric_u32, u32s_geometric,
};
use rust_wheels::iterators::primitive_floats::{
    exhaustive_f32s, exhaustive_f64s, exhaustive_finite_f32s, exhaustive_finite_f64s,
    random_finite_primitive_floats, random_primitive_floats, special_random_f32s,
    special_random_f64s, special_random_finite_f32s, special_random_finite_f64s,
};
use rust_wheels::iterators::primitive_ints::{
    random_natural_signed, random_negative_signed, random_nonzero_signed, random_positive_signed,
    random_positive_unsigned, random_range, random_range_down, random_range_up,
    special_random_natural_signed, special_random_negative_signed, special_random_nonzero_signed,
    special_random_positive_signed, special_random_positive_unsigned, special_random_signed,
    special_random_unsigned,
};
use rust_wheels::iterators::rounding_modes::random_rounding_modes;
use rust_wheels::iterators::strings::{
    exhaustive_strings, exhaustive_strings_with_chars, random_strings, random_strings_with_chars,
};
use rust_wheels::iterators::tuples::{
    exhaustive_octuples_from_single, exhaustive_pairs, exhaustive_pairs_from_single,
    exhaustive_quadruples, exhaustive_quadruples_from_single, exhaustive_quintuples,
    exhaustive_sextuples_from_single, exhaustive_triples, exhaustive_triples_from_single,
    log_pairs, random_octuples_from_single, random_pairs, random_pairs_from_single,
    random_quadruples, random_quadruples_from_single, random_quintuples,
    random_sextuples_from_single, random_triples, random_triples_from_single, sqrt_pairs,
};
use rust_wheels::iterators::vecs::{
    exhaustive_vecs, exhaustive_vecs_min_length, exhaustive_vecs_shortlex, random_vecs,
    random_vecs_min_length, special_random_bool_vecs, special_random_unsigned_vecs,
    special_random_unsigned_vecs_min_length,
};

use common::{GenerationMode, NoSpecialGenerationMode};
use inputs::common::{
    permute_1_2_4_3, permute_1_3_2, permute_1_3_4_2, permute_2_1, permute_2_1_3, reshape_1_2_to_3,
    reshape_2_1_to_3, reshape_2_2_to_4, reshape_3_1_to_4, reshape_3_3_3_to_9, reshape_4_4_4_to_12,
};

//TODO replace with unsigned_gen in Malachite
pub fn unsigneds<T: PrimitiveUnsigned + Rand>(gm: GenerationMode) -> It<T> {
    match gm {
        GenerationMode::Exhaustive => Box::new(exhaustive_unsigneds()),
        GenerationMode::Random(_) => Box::new(random(&EXAMPLE_SEED)),
        GenerationMode::SpecialRandom(_) => Box::new(special_random_unsigned(&EXAMPLE_SEED)),
    }
}

//TODO replace with signed_gen in Malachite
pub fn signeds<T: PrimitiveSigned + Rand>(gm: GenerationMode) -> It<T>
where
    T::UnsignedOfEqualWidth: Rand,
    T: WrappingFrom<<T as PrimitiveSigned>::UnsignedOfEqualWidth>,
{
    match gm {
        GenerationMode::Exhaustive => Box::new(exhaustive_signeds()),
        GenerationMode::Random(_) => Box::new(random(&EXAMPLE_SEED)),
        GenerationMode::SpecialRandom(_) => Box::new(special_random_signed(&EXAMPLE_SEED)),
    }
}

//TODO replace with unsigned_gen_var_1 in Malachite
pub fn positive_unsigneds<T: PrimitiveUnsigned + Rand>(gm: GenerationMode) -> It<T> {
    match gm {
        GenerationMode::Exhaustive => Box::new(exhaustive_positive_primitive_ints()),
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
        GenerationMode::Exhaustive => Box::new(exhaustive_nonzero_signeds()),
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
        GenerationMode::Exhaustive => Box::new(exhaustive_natural_signeds()),
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
        GenerationMode::Exhaustive => Box::new(exhaustive_negative_signeds()),
        GenerationMode::Random(_) => Box::new(random_negative_signed(&EXAMPLE_SEED)),
        GenerationMode::SpecialRandom(_) => Box::new(special_random_negative_signed(&EXAMPLE_SEED)),
    }
}

pub fn signeds_no_max<T: PrimitiveSigned + Rand>(gm: GenerationMode) -> It<T>
where
    T::UnsignedOfEqualWidth: Rand,
    T: WrappingFrom<<T as PrimitiveSigned>::UnsignedOfEqualWidth>,
{
    Box::new(signeds(gm).filter(|&i| i != T::MAX))
}

//TODO replace with signed_gen_var_1 in Malachite
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
        GenerationMode::Exhaustive => Box::new(primitive_int_increasing_inclusive_range(
            T::power_of_two(T::WIDTH - 1),
            T::MAX,
        )),
        GenerationMode::Random(_) => Box::new(random::<T>(&EXAMPLE_SEED).map(|mut u| {
            u.set_bit(T::WIDTH - 1);
            u
        })),
        GenerationMode::SpecialRandom(_) => {
            Box::new(special_random_unsigned::<T>(&EXAMPLE_SEED).map(|mut u| {
                u.set_bit(T::WIDTH - 1);
                u
            }))
        }
    }
}

// All unsigned `T`s that are less than or equal to the greatest power of 2 that fits in a `T`.
pub fn unsigneds_var_2<T: PrimitiveUnsigned + Rand>(gm: GenerationMode) -> It<T> {
    Box::new(unsigneds(gm).filter(|&x| x <= T::power_of_two(T::WIDTH - 1)))
}

// All `u64`s less than `T::WIDTH`.
pub fn unsigneds_var_3<T: PrimitiveInt>(gm: NoSpecialGenerationMode) -> It<u64> {
    match gm {
        NoSpecialGenerationMode::Exhaustive => {
            Box::new(primitive_int_increasing_range(0, T::WIDTH))
        }
        NoSpecialGenerationMode::Random(_) => {
            Box::new(random_range::<u64>(&EXAMPLE_SEED, 0, T::WIDTH))
        }
    }
}

// All signed `T`s that are not 0 or -1.
pub fn signeds_var_1<T: PrimitiveSigned + Rand>(gm: GenerationMode) -> It<T>
where
    T::UnsignedOfEqualWidth: Rand,
    T: WrappingFrom<<T as PrimitiveSigned>::UnsignedOfEqualWidth>,
{
    Box::new(nonzero_signeds(gm).filter(|&i| i != T::NEGATIVE_ONE))
}

// All `T`s, where `T` is signed and the square of the `T` is representable.
pub fn signeds_var_2<T: PrimitiveSigned + Rand + SampleRange>(
    gm: NoSpecialGenerationMode,
) -> It<T> {
    let max = T::power_of_two(T::WIDTH >> 1) - T::ONE;
    let xs: It<T> = match gm {
        NoSpecialGenerationMode::Exhaustive => {
            Box::new(exhaustive_signed_range(-max, max + T::ONE))
        }
        NoSpecialGenerationMode::Random(_) => Box::new(random_range(&EXAMPLE_SEED, -max, max)),
    };
    Box::new(xs.filter(|&x| x.checked_square().is_some()))
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
        GenerationMode::Exhaustive => {
            Box::new(exhaustive_pairs_from_single(exhaustive_unsigneds()))
        }
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
            primitive_int_increasing_inclusive_range(T::power_of_two(T::WIDTH - 1), T::MAX),
            exhaustive_unsigneds(),
        )),
        GenerationMode::Random(_) => Box::new(random_pairs_from_single(random::<T>(seed)).map(
            |(mut u, v)| {
                u.set_bit(T::WIDTH - 1);
                (u, v)
            },
        )),
        GenerationMode::SpecialRandom(_) => Box::new(
            random_pairs_from_single(special_random_unsigned::<T>(seed)).map(|(mut u, v)| {
                u.set_bit(T::WIDTH - 1);
                (u, v)
            }),
        ),
    }
}

// All pairs of `T`s, where `T` is unsigned and the most-significant bit of the first `T` is set.
pub fn pairs_of_unsigneds_var_2<T: PrimitiveUnsigned + Rand>(gm: GenerationMode) -> It<(T, T)> {
    pairs_of_unsigneds_var_2_with_seed(gm, &EXAMPLE_SEED)
}

//TODO use subset_pairs
// All pairs of `T`s where `T` is unsigned and the first `T` is less than or equal to the second.
pub fn pairs_of_unsigneds_var_3<T: PrimitiveUnsigned + Rand>(gm: GenerationMode) -> It<(T, T)> {
    Box::new(pairs_of_unsigneds(gm).filter(|&(x, y)| x <= y))
}

// All pairs of `T`s where `T` is unsigned and the second `T` is nonzero.
pub fn pairs_of_unsigneds_var_4<T: PrimitiveUnsigned + Rand>(gm: GenerationMode) -> It<(T, T)> {
    Box::new(pairs_of_unsigneds(gm).filter(|&(_, y)| y != T::ZERO))
}

//TODO use subset_pairs
// All pairs of `T`s where `T` is unsigned and the first `T` is smaller than the second.
pub fn pairs_of_unsigneds_var_5<T: PrimitiveUnsigned + Rand>(gm: GenerationMode) -> It<(T, T)> {
    Box::new(pairs_of_unsigneds(gm).filter(|&(x, y)| x < y))
}

// All pairs of `T`s that are valid inputs to _limbs_precompute_mod_mul_two_limbs.
pub fn pairs_of_unsigneds_var_6<T: PrimitiveUnsigned + Rand>(gm: GenerationMode) -> It<(T, T)> {
    Box::new(
        pairs_of_unsigneds(gm)
            .filter(|&(m_1, m_0)| m_1 != T::ZERO && (m_1 != T::ONE || m_0 != T::ZERO)),
    )
}

// All pairs of `T`s where `T` is unsigned, the second `T` is nonzero, and the first `T` is
// divisible by the second.
pub fn pairs_of_unsigneds_var_7<T: PrimitiveUnsigned + Rand>(gm: GenerationMode) -> It<(T, T)> {
    let ps = pairs_of_unsigneds::<T>(gm).filter_map(|(x, y)| {
        if y == T::ZERO {
            None
        } else {
            Some((x.round_to_multiple(y, RoundingMode::Down), y))
        }
    });
    if gm == GenerationMode::Exhaustive {
        Box::new(ps.unique())
    } else {
        Box::new(ps)
    }
}

// All `T`s, where `T` is unsigned and the square of the `T` is representable.
pub fn unsigneds_var_8<T: PrimitiveUnsigned + Rand + SampleRange>(
    gm: NoSpecialGenerationMode,
) -> It<T> {
    let max = T::power_of_two(T::WIDTH >> 1);
    match gm {
        NoSpecialGenerationMode::Exhaustive => {
            Box::new(primitive_int_increasing_range(T::ZERO, max))
        }
        NoSpecialGenerationMode::Random(_) => {
            Box::new(random_range_down(&EXAMPLE_SEED, max - T::ONE))
        }
    }
}

pub fn triples_of_unsigneds<T: PrimitiveUnsigned + Rand>(gm: GenerationMode) -> It<(T, T, T)> {
    match gm {
        GenerationMode::Exhaustive => {
            Box::new(exhaustive_triples_from_single(exhaustive_unsigneds()))
        }
        GenerationMode::Random(_) => Box::new(random_triples_from_single(random(&EXAMPLE_SEED))),
        GenerationMode::SpecialRandom(_) => Box::new(random_triples_from_single(
            special_random_unsigned(&EXAMPLE_SEED),
        )),
    }
}

// All triples of unsigned `T` where the first and the second `T`s are smaller than the third.
pub fn triples_of_unsigneds_var_1<T: PrimitiveUnsigned + Rand>(
    gm: GenerationMode,
) -> It<(T, T, T)> {
    Box::new(triples_of_unsigneds(gm).filter(|&(x, y, m)| x < m && y < m))
}

// All triples of unsigned `T` where the first `T` is are smaller than the third.
pub fn triples_of_unsigneds_var_2<T: PrimitiveUnsigned + Rand>(
    gm: GenerationMode,
) -> It<(T, T, T)> {
    Box::new(triples_of_unsigneds(gm).filter(|&(x_1, _, y)| x_1 < y))
}

fn add_mul_inputs_valid<T: PrimitiveInt>(x: T, y: T, z: T) -> bool {
    x.checked_add_mul(y, z).is_some()
}

pub(crate) struct ValidAddMulInputs<T> {
    ts: It<(T, T, T)>,
    range: Range<u64>,
    pub(crate) rng: Box<IsaacRng>,
}

impl<T> ValidAddMulInputs<T> {
    fn new(ts: It<(T, T, T)>, rng: Box<IsaacRng>) -> ValidAddMulInputs<T> {
        ValidAddMulInputs {
            ts,
            range: Range::new(0, 3),
            rng,
        }
    }
}

impl<T: PrimitiveInt> Iterator for ValidAddMulInputs<T> {
    type Item = (T, T, T);

    fn next(&mut self) -> Option<(T, T, T)> {
        self.ts.next().map(|(mut x, mut y, mut z)| {
            while !add_mul_inputs_valid(x, y, z) {
                match self.range.ind_sample(&mut self.rng) {
                    0 => x >>= 1,
                    1 => y >>= 1,
                    2 => z >>= 1,
                    _ => unreachable!(),
                }
            }
            (x, y, z)
        })
    }
}

// All triples (x, y, z) of unsigned `T`, where x + y * z doesn't overflow.
pub fn triples_of_unsigneds_var_3<T: PrimitiveUnsigned + Rand>(
    gm: GenerationMode,
) -> It<(T, T, T)> {
    match gm {
        GenerationMode::Exhaustive => Box::new(
            exhaustive_triples_from_single(exhaustive_unsigneds())
                .filter(|&(x, y, z)| add_mul_inputs_valid(x, y, z)),
        ),
        GenerationMode::Random(_) => Box::new(ValidAddMulInputs::new(
            Box::new(random_triples_from_single(random(&scramble(
                &EXAMPLE_SEED,
                "triples",
            )))),
            Box::new(IsaacRng::from_seed(&scramble(&EXAMPLE_SEED, "reducer"))),
        )),
        GenerationMode::SpecialRandom(_) => Box::new(ValidAddMulInputs::new(
            Box::new(random_triples_from_single(special_random_unsigned(
                &scramble(&EXAMPLE_SEED, "triples"),
            ))),
            Box::new(IsaacRng::from_seed(&scramble(&EXAMPLE_SEED, "reducer"))),
        )),
    }
}

fn sub_mul_inputs_valid<T: PrimitiveInt>(x: T, y: T, z: T) -> bool {
    x.checked_sub_mul(y, z).is_some()
}

pub(crate) struct ValidSubMulInputs<T> {
    ts: It<(T, T, T)>,
    range: Range<u64>,
    pub(crate) rng: Box<IsaacRng>,
}

impl<T> ValidSubMulInputs<T> {
    fn new(ts: It<(T, T, T)>, rng: Box<IsaacRng>) -> ValidSubMulInputs<T> {
        ValidSubMulInputs {
            ts,
            range: Range::new(0, 3),
            rng,
        }
    }
}

impl<T: PrimitiveInt> Iterator for ValidSubMulInputs<T> {
    type Item = (T, T, T);

    fn next(&mut self) -> Option<(T, T, T)> {
        self.ts.next().map(|(mut x, mut y, mut z)| {
            while !sub_mul_inputs_valid(x, y, z) {
                match self.range.ind_sample(&mut self.rng) {
                    0 => x >>= 1,
                    1 => y >>= 1,
                    2 => z >>= 1,
                    _ => unreachable!(),
                }
            }
            (x, y, z)
        })
    }
}

// All triples (x, y, z) of unsigned `T`, where x - y * z doesn't overflow.
pub fn triples_of_unsigneds_var_4<T: PrimitiveUnsigned + Rand>(
    gm: GenerationMode,
) -> It<(T, T, T)> {
    match gm {
        GenerationMode::Exhaustive => Box::new(
            exhaustive_triples_from_single(exhaustive_unsigneds())
                .filter(|&(x, y, z)| sub_mul_inputs_valid(x, y, z)),
        ),
        GenerationMode::Random(_) => Box::new(ValidSubMulInputs::new(
            Box::new(random_triples_from_single(random(&scramble(
                &EXAMPLE_SEED,
                "triples",
            )))),
            Box::new(IsaacRng::from_seed(&scramble(&EXAMPLE_SEED, "reducer"))),
        )),
        GenerationMode::SpecialRandom(_) => Box::new(ValidSubMulInputs::new(
            Box::new(random_triples_from_single(special_random_unsigned(
                &scramble(&EXAMPLE_SEED, "triples"),
            ))),
            Box::new(IsaacRng::from_seed(&scramble(&EXAMPLE_SEED, "reducer"))),
        )),
    }
}

// All triples of `T`s, where `T` is unsigned and the first `T` is not equal to the second mod the
// third.
pub fn triples_of_unsigneds_var_5<T: PrimitiveUnsigned + Rand>(
    gm: GenerationMode,
) -> It<(T, T, T)> {
    Box::new(triples_of_unsigneds::<T>(gm).filter(|&(x, y, m)| !x.eq_mod(y, m)))
}

// All triples of `T`s, where `T` is unsigned and the second `T` is odd.
pub fn triples_of_unsigneds_var_6<T: PrimitiveUnsigned + Rand>(
    gm: GenerationMode,
) -> It<(T, T, T)> {
    Box::new(triples_of_unsigneds::<T>(gm).filter(|&(_, d, _)| d.odd()))
}

pub fn pairs_of_signeds<T: PrimitiveSigned + Rand>(gm: GenerationMode) -> It<(T, T)>
where
    T::UnsignedOfEqualWidth: Rand,
    T: WrappingFrom<<T as PrimitiveSigned>::UnsignedOfEqualWidth>,
{
    match gm {
        GenerationMode::Exhaustive => Box::new(exhaustive_pairs_from_single(exhaustive_signeds())),
        GenerationMode::Random(_) => Box::new(random_pairs_from_single(random(&EXAMPLE_SEED))),
        GenerationMode::SpecialRandom(_) => Box::new(random_pairs_from_single(
            special_random_signed(&EXAMPLE_SEED),
        )),
    }
}

// All pairs of signeds with the same sign.
pub fn pairs_of_signeds_var_1<T: PrimitiveSigned + Rand>(gm: GenerationMode) -> It<(T, T)>
where
    T::UnsignedOfEqualWidth: Rand,
    T: WrappingFrom<<T as PrimitiveSigned>::UnsignedOfEqualWidth>,
{
    Box::new(pairs_of_signeds(gm).filter(|&(x, y)| (x >= T::ZERO) == (y >= T::ZERO)))
}

// All pairs of signeds where the second is nonzero, and (T::MIN, T::NEGATIVE_ONE) is excluded.
pub fn pairs_of_signeds_var_2<T: PrimitiveSigned + Rand>(gm: GenerationMode) -> It<(T, T)>
where
    T::UnsignedOfEqualWidth: Rand,
    T: WrappingFrom<<T as PrimitiveSigned>::UnsignedOfEqualWidth>,
{
    Box::new(
        pairs_of_signed_and_nonzero_signed(gm)
            .filter(|&(x, y)| x != T::MIN || y != T::NEGATIVE_ONE),
    )
}

// All pairs of signeds where the second is nonzero and the first is not divisible by the second.
pub fn pairs_of_signeds_var_3<T: PrimitiveSigned + Rand>(gm: GenerationMode) -> It<(T, T)>
where
    T::UnsignedOfEqualWidth: Rand,
    T: WrappingFrom<<T as PrimitiveSigned>::UnsignedOfEqualWidth>,
{
    Box::new(pairs_of_signed_and_nonzero_signed::<T, T>(gm).filter(|&(x, y)| !x.divisible_by(y)))
}

// All pairs of `T`s where `T` is signed, the second `T` is nonzero, and the first `T` is divisible
// by the second.
pub fn pairs_of_signeds_var_4<T: PrimitiveSigned + Rand>(gm: GenerationMode) -> It<(T, T)>
where
    T::UnsignedOfEqualWidth: Rand,
    T: WrappingFrom<<T as PrimitiveSigned>::UnsignedOfEqualWidth>,
{
    let ps = pairs_of_signeds::<T>(gm).filter_map(|(x, y)| {
        if y == T::ZERO || x == T::MIN && y == T::NEGATIVE_ONE {
            None
        } else {
            Some((x.round_to_multiple(y, RoundingMode::Down), y))
        }
    });
    if gm == GenerationMode::Exhaustive {
        Box::new(ps.unique())
    } else {
        Box::new(ps)
    }
}

pub fn triples_of_signeds<T: PrimitiveSigned + Rand>(gm: GenerationMode) -> It<(T, T, T)>
where
    T::UnsignedOfEqualWidth: Rand,
    T: WrappingFrom<<T as PrimitiveSigned>::UnsignedOfEqualWidth>,
{
    match gm {
        GenerationMode::Exhaustive => {
            Box::new(exhaustive_triples_from_single(exhaustive_signeds()))
        }
        GenerationMode::Random(_) => Box::new(random_triples_from_single(random(&EXAMPLE_SEED))),
        GenerationMode::SpecialRandom(_) => Box::new(random_triples_from_single(
            special_random_signed(&EXAMPLE_SEED),
        )),
    }
}

// All triples of signeds with the same sign.
pub fn triples_of_signeds_var_1<T: PrimitiveSigned + Rand>(gm: GenerationMode) -> It<(T, T, T)>
where
    T::UnsignedOfEqualWidth: Rand,
    T: WrappingFrom<<T as PrimitiveSigned>::UnsignedOfEqualWidth>,
{
    Box::new(
        triples_of_signeds(gm).filter(|&(x, y, z)| {
            (x >= T::ZERO) == (y >= T::ZERO) && (y >= T::ZERO) == (z >= T::ZERO)
        }),
    )
}

// All triples (x, y, z) of signed `T`, where x + y * z doesn't overflow.
pub fn triples_of_signeds_var_2<T: PrimitiveSigned + Rand>(gm: GenerationMode) -> It<(T, T, T)>
where
    T::UnsignedOfEqualWidth: Rand,
    T: WrappingFrom<<T as PrimitiveSigned>::UnsignedOfEqualWidth>,
{
    match gm {
        GenerationMode::Exhaustive => Box::new(
            exhaustive_triples_from_single(exhaustive_signeds())
                .filter(|&(x, y, z)| add_mul_inputs_valid(x, y, z)),
        ),
        GenerationMode::Random(_) => Box::new(ValidAddMulInputs::new(
            Box::new(random_triples_from_single(random(&scramble(
                &EXAMPLE_SEED,
                "triples",
            )))),
            Box::new(IsaacRng::from_seed(&scramble(&EXAMPLE_SEED, "reducer"))),
        )),
        GenerationMode::SpecialRandom(_) => Box::new(ValidAddMulInputs::new(
            Box::new(random_triples_from_single(special_random_signed(
                &scramble(&EXAMPLE_SEED, "triples"),
            ))),
            Box::new(IsaacRng::from_seed(&scramble(&EXAMPLE_SEED, "reducer"))),
        )),
    }
}

// All triples (x, y, z) of signed `T`, where x - y * z doesn't overflow.
pub fn triples_of_signeds_var_3<T: PrimitiveSigned + Rand>(gm: GenerationMode) -> It<(T, T, T)>
where
    T::UnsignedOfEqualWidth: Rand,
    T: WrappingFrom<<T as PrimitiveSigned>::UnsignedOfEqualWidth>,
{
    match gm {
        GenerationMode::Exhaustive => Box::new(
            exhaustive_triples_from_single(exhaustive_signeds())
                .filter(|&(x, y, z)| sub_mul_inputs_valid(x, y, z)),
        ),
        GenerationMode::Random(_) => Box::new(ValidSubMulInputs::new(
            Box::new(random_triples_from_single(random(&scramble(
                &EXAMPLE_SEED,
                "triples",
            )))),
            Box::new(IsaacRng::from_seed(&scramble(&EXAMPLE_SEED, "reducer"))),
        )),
        GenerationMode::SpecialRandom(_) => Box::new(ValidSubMulInputs::new(
            Box::new(random_triples_from_single(special_random_signed(
                &scramble(&EXAMPLE_SEED, "triples"),
            ))),
            Box::new(IsaacRng::from_seed(&scramble(&EXAMPLE_SEED, "reducer"))),
        )),
    }
}

// All triples of `T`s, where `T` is signed and the first `T` is not equal to the second mod the
// third.
pub fn triples_of_signeds_var_4<T: PrimitiveSigned + Rand>(gm: GenerationMode) -> It<(T, T, T)>
where
    T::UnsignedOfEqualWidth: Rand,
    T: WrappingFrom<<T as PrimitiveSigned>::UnsignedOfEqualWidth>,
{
    Box::new(triples_of_signeds::<T>(gm).filter(|&(x, y, m)| !x.eq_mod(y, m)))
}

pub fn pairs_of_natural_signeds<T: PrimitiveSigned + Rand>(gm: GenerationMode) -> It<(T, T)>
where
    T::UnsignedOfEqualWidth: Rand,
    T: WrappingFrom<<T as PrimitiveSigned>::UnsignedOfEqualWidth>,
{
    match gm {
        GenerationMode::Exhaustive => {
            Box::new(exhaustive_pairs_from_single(exhaustive_natural_signeds()))
        }
        GenerationMode::Random(_) => Box::new(random_pairs_from_single(random_natural_signed(
            &EXAMPLE_SEED,
        ))),
        GenerationMode::SpecialRandom(_) => Box::new(random_pairs_from_single(
            special_random_natural_signed(&EXAMPLE_SEED),
        )),
    }
}

// All `u32`s smaller than `NUMBER_OF_CHARS`.
pub fn u32s_range_1(gm: NoSpecialGenerationMode) -> It<u32> {
    match gm {
        NoSpecialGenerationMode::Exhaustive => {
            Box::new(primitive_int_increasing_range(0, NUMBER_OF_CHARS))
        }
        NoSpecialGenerationMode::Random(_) => {
            Box::new(random_range_down(&EXAMPLE_SEED, NUMBER_OF_CHARS - 1))
        }
    }
}

// All `Limb`s smaller than 2<sup>Limb::WIDTH - 1<sup>.
fn limbs_range_2(gm: GenerationMode) -> It<Limb> {
    let upper = Limb::power_of_two(Limb::WIDTH - 1);
    match gm {
        GenerationMode::Exhaustive => Box::new(primitive_int_increasing_range(0, upper)),
        GenerationMode::Random(_) => Box::new(random_range_down(&EXAMPLE_SEED, upper - 1)),
        GenerationMode::SpecialRandom(_) => {
            Box::new(special_random_unsigned::<Limb>(&EXAMPLE_SEED).map(|u| {
                let mut u = u;
                u.clear_bit(u32::WIDTH - 1);
                u
            }))
        }
    }
}

pub fn odd_limbs(gm: GenerationMode) -> It<Limb> {
    Box::new(limbs_range_2(gm).map(|u| (u << 1) + 1))
}

// All pairs of `u32`s smaller than `NUMBER_OF_CHARS`.
pub fn pairs_of_u32s_range_1(gm: NoSpecialGenerationMode) -> It<(u32, u32)> {
    match gm {
        NoSpecialGenerationMode::Exhaustive => Box::new(exhaustive_pairs_from_single(
            primitive_int_increasing_range(0, NUMBER_OF_CHARS),
        )),
        NoSpecialGenerationMode::Random(_) => Box::new(random_pairs_from_single(
            random_range_down(&EXAMPLE_SEED, NUMBER_OF_CHARS - 1),
        )),
    }
}

pub fn pairs_of_unsigned_and_unsigned<T: PrimitiveUnsigned + Rand, U: PrimitiveUnsigned + Rand>(
    gm: GenerationMode,
) -> It<(T, U)> {
    match gm {
        GenerationMode::Exhaustive => Box::new(exhaustive_pairs(
            exhaustive_unsigneds(),
            exhaustive_unsigneds(),
        )),
        GenerationMode::Random(_) => Box::new(random_pairs(
            &EXAMPLE_SEED,
            &(|seed| random(seed)),
            &(|seed| random(seed)),
        )),
        GenerationMode::SpecialRandom(_) => Box::new(random_pairs(
            &EXAMPLE_SEED,
            &(|seed| special_random_unsigned(seed)),
            &(|seed| special_random_unsigned(seed)),
        )),
    }
}

pub fn triples_of_unsigned_unsigned_and_unsigned<
    T: PrimitiveUnsigned + Rand,
    U: PrimitiveUnsigned + Rand,
    V: PrimitiveUnsigned + Rand,
>(
    gm: GenerationMode,
) -> It<(T, U, V)> {
    match gm {
        GenerationMode::Exhaustive => Box::new(exhaustive_triples(
            exhaustive_unsigneds(),
            exhaustive_unsigneds(),
            exhaustive_unsigneds(),
        )),
        GenerationMode::Random(_) => Box::new(random_triples(
            &EXAMPLE_SEED,
            &(|seed| random(seed)),
            &(|seed| random(seed)),
            &(|seed| random(seed)),
        )),
        GenerationMode::SpecialRandom(_) => Box::new(random_triples(
            &EXAMPLE_SEED,
            &(|seed| special_random_unsigned(seed)),
            &(|seed| special_random_unsigned(seed)),
            &(|seed| special_random_unsigned(seed)),
        )),
    }
}

pub fn triples_of_unsigned_signed_and_unsigned<
    T: PrimitiveUnsigned + Rand,
    U: PrimitiveSigned + Rand,
    V: PrimitiveUnsigned + Rand,
>(
    gm: GenerationMode,
) -> It<(T, U, V)>
where
    U::UnsignedOfEqualWidth: Rand,
    U: WrappingFrom<<U as PrimitiveSigned>::UnsignedOfEqualWidth>,
{
    match gm {
        GenerationMode::Exhaustive => Box::new(exhaustive_triples(
            exhaustive_unsigneds(),
            exhaustive_signeds(),
            exhaustive_unsigneds(),
        )),
        GenerationMode::Random(_) => Box::new(random_triples(
            &EXAMPLE_SEED,
            &(|seed| random(seed)),
            &(|seed| random(seed)),
            &(|seed| random(seed)),
        )),
        GenerationMode::SpecialRandom(_) => Box::new(random_triples(
            &EXAMPLE_SEED,
            &(|seed| special_random_unsigned(seed)),
            &(|seed| special_random_signed(seed)),
            &(|seed| special_random_unsigned(seed)),
        )),
    }
}

pub fn quadruples_of_four_unsigneds<
    T: PrimitiveUnsigned + Rand,
    U: PrimitiveUnsigned + Rand,
    V: PrimitiveUnsigned + Rand,
    W: PrimitiveUnsigned + Rand,
>(
    gm: GenerationMode,
) -> It<(T, U, V, W)> {
    match gm {
        GenerationMode::Exhaustive => Box::new(exhaustive_quadruples(
            exhaustive_unsigneds(),
            exhaustive_unsigneds(),
            exhaustive_unsigneds(),
            exhaustive_unsigneds(),
        )),
        GenerationMode::Random(_) => Box::new(random_quadruples(
            &EXAMPLE_SEED,
            &(|seed| random(seed)),
            &(|seed| random(seed)),
            &(|seed| random(seed)),
            &(|seed| random(seed)),
        )),
        GenerationMode::SpecialRandom(_) => Box::new(random_quadruples(
            &EXAMPLE_SEED,
            &(|seed| special_random_unsigned(seed)),
            &(|seed| special_random_unsigned(seed)),
            &(|seed| special_random_unsigned(seed)),
            &(|seed| special_random_unsigned(seed)),
        )),
    }
}

// All triples of `T`, `U`, and `T`, where `T` and `U` are unsigned and the first `T`s is smaller
// than the second.
pub fn triples_of_unsigned_unsigned_and_unsigned_var_1<
    T: PrimitiveUnsigned + Rand,
    U: PrimitiveUnsigned + Rand,
>(
    gm: GenerationMode,
) -> It<(T, U, T)> {
    Box::new(triples_of_unsigned_unsigned_and_unsigned(gm).filter(|&(x, _, m)| x < m))
}

// All triples of `T`, `U`, and `T`, where `T` is unsigned, `U` is signed, and the first `T`s is
// smaller than the second.
pub fn triples_of_unsigned_signed_and_unsigned_var_1<
    T: PrimitiveUnsigned + Rand,
    U: PrimitiveSigned + Rand,
>(
    gm: GenerationMode,
) -> It<(T, U, T)>
where
    U::UnsignedOfEqualWidth: Rand,
    U: WrappingFrom<<U as PrimitiveSigned>::UnsignedOfEqualWidth>,
{
    Box::new(triples_of_unsigned_signed_and_unsigned(gm).filter(|&(x, _, m)| x < m))
}

pub fn pairs_of_unsigned_and_positive_unsigned<
    T: PrimitiveUnsigned + Rand,
    U: PrimitiveUnsigned + Rand,
>(
    gm: GenerationMode,
) -> It<(T, U)> {
    match gm {
        GenerationMode::Exhaustive => Box::new(exhaustive_pairs(
            exhaustive_unsigneds(),
            exhaustive_positive_primitive_ints(),
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

pub fn pairs_of_signed_and_positive_unsigned<
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
        GenerationMode::Exhaustive => Box::new(exhaustive_pairs(
            exhaustive_signeds(),
            exhaustive_positive_primitive_ints(),
        )),
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

// All pairs of `T`s`, where `T` is unsigned, the second `T` is nonzero, and the first `T` is not
// divisible by the second.
pub fn pairs_of_unsigned_and_positive_unsigned_var_1<T: PrimitiveUnsigned + Rand>(
    gm: GenerationMode,
) -> It<(T, T)> {
    Box::new(
        pairs_of_unsigned_and_positive_unsigned::<T, T>(gm).filter(|&(x, y)| !x.divisible_by(y)),
    )
}

pub fn pairs_of_signed_and_unsigned<T: PrimitiveSigned + Rand, U: PrimitiveUnsigned + Rand>(
    gm: GenerationMode,
) -> It<(T, U)>
where
    T::UnsignedOfEqualWidth: Rand,
    T: WrappingFrom<<T as PrimitiveSigned>::UnsignedOfEqualWidth>,
{
    match gm {
        GenerationMode::Exhaustive => Box::new(exhaustive_pairs(
            exhaustive_signeds(),
            exhaustive_unsigneds(),
        )),
        GenerationMode::Random(_) => Box::new(random_pairs(
            &EXAMPLE_SEED,
            &(|seed| random(seed)),
            &(|seed| random(seed)),
        )),
        GenerationMode::SpecialRandom(_) => Box::new(random_pairs(
            &EXAMPLE_SEED,
            &(|seed| special_random_signed(seed)),
            &(|seed| special_random_unsigned(seed)),
        )),
    }
}

pub fn pairs_of_signed_and_nonzero_signed<T: PrimitiveSigned + Rand, U: PrimitiveSigned + Rand>(
    gm: GenerationMode,
) -> It<(T, U)>
where
    T::UnsignedOfEqualWidth: Rand,
    T: WrappingFrom<<T as PrimitiveSigned>::UnsignedOfEqualWidth>,
    U::UnsignedOfEqualWidth: Rand,
    U: WrappingFrom<<U as PrimitiveSigned>::UnsignedOfEqualWidth>,
{
    match gm {
        GenerationMode::Exhaustive => Box::new(exhaustive_pairs(
            exhaustive_signeds(),
            exhaustive_nonzero_signeds(),
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

pub fn small_unsigneds<T: PrimitiveUnsigned>(gm: NoSpecialGenerationMode) -> It<T> {
    match gm {
        NoSpecialGenerationMode::Exhaustive => Box::new(exhaustive_unsigneds()),
        NoSpecialGenerationMode::Random(scale) => {
            Box::new(u32s_geometric(&EXAMPLE_SEED, scale).flat_map(T::checked_from))
        }
    }
}

pub fn small_positive_unsigneds<T: PrimitiveInt + Rand>(gm: NoSpecialGenerationMode) -> It<T> {
    match gm {
        NoSpecialGenerationMode::Exhaustive => Box::new(exhaustive_positive_primitive_ints()),
        NoSpecialGenerationMode::Random(scale) => {
            Box::new(positive_u32s_geometric(&EXAMPLE_SEED, scale).flat_map(T::checked_from))
        }
    }
}

// All `u64`s where the `u64` is between 1 and T::WIDTH, inclusive.
pub fn small_u64s_var_1<T: PrimitiveInt>(gm: NoSpecialGenerationMode) -> It<u64> {
    match gm {
        NoSpecialGenerationMode::Exhaustive => {
            Box::new(primitive_int_increasing_inclusive_range(1, T::WIDTH))
        }
        NoSpecialGenerationMode::Random(_) => Box::new(random_range(&EXAMPLE_SEED, 1, T::WIDTH)),
    }
}

// All `u64`s where the `u64` is between 0 and T::WIDTH - 1, inclusive.
pub fn small_u64s_var_2<T: PrimitiveInt>(gm: NoSpecialGenerationMode) -> It<u64> {
    match gm {
        NoSpecialGenerationMode::Exhaustive => {
            Box::new(primitive_int_increasing_range(0, T::WIDTH))
        }
        NoSpecialGenerationMode::Random(_) => {
            Box::new(random_range(&EXAMPLE_SEED, 0, T::WIDTH - 1))
        }
    }
}

// All `u64`s where the `u64` is between 0 and T::WIDTH - 2, inclusive.
pub fn small_u64s_var_3<T: PrimitiveInt>(gm: NoSpecialGenerationMode) -> It<u64> {
    match gm {
        NoSpecialGenerationMode::Exhaustive => {
            Box::new(primitive_int_increasing_range(0, T::WIDTH - 1))
        }
        NoSpecialGenerationMode::Random(_) => {
            Box::new(random_range(&EXAMPLE_SEED, 0, T::WIDTH - 2))
        }
    }
}

// All `u64`s where the `u64` is between 0 and T::WIDTH, inclusive.
pub fn small_u64s_var_4<T: PrimitiveInt>(gm: NoSpecialGenerationMode) -> It<u64> {
    match gm {
        NoSpecialGenerationMode::Exhaustive => {
            Box::new(primitive_int_increasing_inclusive_range(0, T::WIDTH))
        }
        NoSpecialGenerationMode::Random(_) => Box::new(random_range(&EXAMPLE_SEED, 0, T::WIDTH)),
    }
}

pub fn small_signeds<T: PrimitiveSigned>(gm: NoSpecialGenerationMode) -> It<T> {
    match gm {
        NoSpecialGenerationMode::Exhaustive => Box::new(exhaustive_signeds()),
        NoSpecialGenerationMode::Random(scale) => {
            Box::new(i32s_geometric(&EXAMPLE_SEED, scale).flat_map(T::checked_from))
        }
    }
}

fn sqrt_pairs_of_unsigneds<T: PrimitiveUnsigned, U: PrimitiveUnsigned>() -> It<(T, U)> {
    Box::new(sqrt_pairs(exhaustive_unsigneds(), exhaustive_unsigneds()))
}

fn sqrt_pairs_of_unsigned_and_signed<T: PrimitiveUnsigned, U: PrimitiveSigned>() -> It<(T, U)> {
    Box::new(sqrt_pairs(exhaustive_unsigneds(), exhaustive_signeds()))
}

fn sqrt_pairs_of_signed_and_unsigned<T: PrimitiveSigned, U: PrimitiveUnsigned>() -> It<(T, U)> {
    Box::new(sqrt_pairs(exhaustive_signeds(), exhaustive_unsigneds()))
}

fn sqrt_pairs_of_signeds<T: PrimitiveSigned, U: PrimitiveSigned>() -> It<(T, U)> {
    Box::new(sqrt_pairs(exhaustive_signeds(), exhaustive_signeds()))
}

fn random_pairs_of_primitive_and_geometric<T: PrimitiveInt + Rand, U: PrimitiveInt>(
    scale: u32,
) -> It<(T, U)> {
    Box::new(random_pairs(
        &EXAMPLE_SEED,
        &(|seed| random(seed)),
        &(|seed| u32s_geometric(seed, scale).flat_map(U::checked_from)),
    ))
}

pub fn pairs_of_unsigned_and_small_unsigned<T: PrimitiveUnsigned + Rand, U: PrimitiveUnsigned>(
    gm: GenerationMode,
) -> It<(T, U)> {
    match gm {
        GenerationMode::Exhaustive => sqrt_pairs_of_unsigneds(),
        GenerationMode::Random(scale) => random_pairs_of_primitive_and_geometric(scale),
        GenerationMode::SpecialRandom(scale) => Box::new(random_pairs(
            &EXAMPLE_SEED,
            &(|seed| special_random_unsigned(seed)),
            &(|seed| u32s_geometric(seed, scale).flat_map(U::checked_from)),
        )),
    }
}

// All pairs of `T` and small `U`, where `T` and `U` are unsigned and the `T` is not divisible by 2
// to the power of the `U`.
pub fn pairs_of_unsigned_and_small_unsigned_var_1<
    T: PrimitiveUnsigned + Rand,
    U: PrimitiveUnsigned,
>(
    gm: GenerationMode,
) -> It<(T, U)> {
    Box::new(
        pairs_of_unsigned_and_small_unsigned::<T, U>(gm)
            .filter(|&(n, u)| !n.divisible_by_power_of_two(u.exact_into())),
    )
}

pub fn pairs_of_unsigned_and_small_signed<T: PrimitiveUnsigned + Rand, U: PrimitiveSigned>(
    gm: GenerationMode,
) -> It<(T, U)> {
    match gm {
        GenerationMode::Exhaustive => sqrt_pairs_of_unsigned_and_signed(),
        GenerationMode::Random(scale) => random_pairs_of_primitive_and_geometric(scale),
        GenerationMode::SpecialRandom(scale) => Box::new(random_pairs(
            &EXAMPLE_SEED,
            &(|seed| special_random_unsigned(seed)),
            &(|seed| i32s_geometric(seed, scale).flat_map(U::checked_from)),
        )),
    }
}

fn random_pairs_of_primitive_and_small_unsigned_var_1_with_seed<
    T: PrimitiveInt + Rand,
    U: PrimitiveUnsigned,
>(
    seed: &[u32],
) -> It<(T, u64)> {
    Box::new(random_pairs(
        seed,
        &(|seed_2| random(seed_2)),
        &(|seed_2| random_range(seed_2, 1, U::WIDTH)),
    ))
}

fn random_pairs_of_primitive_and_small_unsigned_var_1<
    T: PrimitiveInt + Rand,
    U: PrimitiveUnsigned,
>() -> It<(T, u64)> {
    random_pairs_of_primitive_and_small_unsigned_var_1_with_seed::<T, U>(&EXAMPLE_SEED)
}

fn special_random_pairs_of_unsigned_and_small_unsigned_var_1_with_seed<
    T: PrimitiveUnsigned + Rand,
    U: PrimitiveUnsigned,
>(
    seed: &[u32],
) -> It<(T, u64)> {
    Box::new(random_pairs(
        seed,
        &(|seed_2| special_random_unsigned(seed_2)),
        &(|seed_2| random_range(seed_2, 1, U::WIDTH)),
    ))
}

// All pairs of unsigned `T` and `u64`, where the `u64` is between 1 and `U::WIDTH`, inclusive.
pub fn pairs_of_unsigned_and_small_u64_var_1<T: PrimitiveUnsigned + Rand, U: PrimitiveUnsigned>(
    gm: GenerationMode,
) -> It<(T, u64)> {
    match gm {
        GenerationMode::Exhaustive => Box::new(lex_pairs(
            exhaustive_unsigneds(),
            primitive_int_increasing_inclusive_range(1, U::WIDTH),
        )),
        GenerationMode::Random(_) => random_pairs_of_primitive_and_small_unsigned_var_1::<T, U>(),
        GenerationMode::SpecialRandom(_) => Box::new(random_pairs(
            &EXAMPLE_SEED,
            &(|seed| special_random_unsigned(seed)),
            &(|seed| random_range(seed, 1, U::WIDTH)),
        )),
    }
}

// All pairs of unsigned `T` and `u64`, where the `u64` is between the number of significant bits of
// the `T` and `T::WIDTH`, inclusive.
pub fn pairs_of_unsigned_and_small_u64_var_2<T: PrimitiveUnsigned + Rand + SampleRange>(
    gm: NoSpecialGenerationMode,
) -> It<(T, u64)> {
    match gm {
        NoSpecialGenerationMode::Exhaustive => {
            Box::new(dependent_pairs(exhaustive_unsigneds(), |&u: &T| {
                Box::new(primitive_int_increasing_inclusive_range(
                    u.significant_bits(),
                    T::WIDTH,
                ))
            }))
        }
        NoSpecialGenerationMode::Random(_) => permute_2_1(Box::new(random_dependent_pairs(
            (),
            random_range(&scramble(&EXAMPLE_SEED, "pow"), 0, T::WIDTH),
            |_, &pow| random_range::<T>(&scramble(&EXAMPLE_SEED, "u"), T::ZERO, T::low_mask(pow)),
        ))),
    }
}

// All pairs of unsigned `T` and `u64`, where the `u64` is between 0 and `U::WIDTH`, inclusive.
pub fn pairs_of_unsigned_and_small_u64_var_3<T: PrimitiveUnsigned + Rand, U: PrimitiveUnsigned>(
    gm: GenerationMode,
) -> It<(T, u64)> {
    match gm {
        GenerationMode::Exhaustive => Box::new(lex_pairs(
            exhaustive_unsigneds(),
            primitive_int_increasing_inclusive_range(0, U::WIDTH),
        )),
        GenerationMode::Random(_) => Box::new(random_pairs(
            &EXAMPLE_SEED,
            &(|seed| random(seed)),
            &(|seed| random_range_down(seed, U::WIDTH)),
        )),
        GenerationMode::SpecialRandom(_) => Box::new(random_pairs(
            &EXAMPLE_SEED,
            &(|seed| special_random_unsigned(seed)),
            &(|seed| random_range_down(seed, U::WIDTH)),
        )),
    }
}

// All pairs of unsigned `T` and small `u64`, where the `T` is 0 or the `u64` is smaller than
// `T::WIDTH`.
pub fn pairs_of_unsigned_and_small_u64_var_4<T: PrimitiveUnsigned + Rand>(
    gm: GenerationMode,
) -> It<(T, u64)> {
    match gm {
        GenerationMode::Exhaustive => Box::new(
            lex_pairs(
                exhaustive_unsigneds(),
                primitive_int_increasing_range(0, T::WIDTH),
            )
            .interleave(
                primitive_int_increasing_inclusive_range(T::WIDTH, u64::MAX)
                    .map(|pow| (T::ZERO, pow)),
            ),
        ),
        GenerationMode::Random(scale) => Box::new(
            random_pairs_of_primitive_and_geometric(scale)
                .filter(|&(x, y)| x == T::ZERO || y < T::WIDTH),
        ),
        GenerationMode::SpecialRandom(scale) => Box::new(
            random_pairs(
                &EXAMPLE_SEED,
                &(|seed| special_random_unsigned(seed)),
                &(|seed| u32s_geometric(seed, scale).map(u64::from)),
            )
            .filter(|&(x, y)| x == T::ZERO || y < T::WIDTH),
        ),
    }
}

pub fn pairs_of_small_usize_and_unsigned<T: PrimitiveUnsigned + Rand>(
    gm: GenerationMode,
) -> It<(usize, T)> {
    match gm {
        GenerationMode::Exhaustive => permute_2_1(Box::new(log_pairs(
            exhaustive_unsigneds(),
            exhaustive_unsigneds(),
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

pub fn pairs_of_small_unsigneds_single<T: PrimitiveUnsigned + Rand>(
    gm: NoSpecialGenerationMode,
) -> It<(T, T)> {
    match gm {
        NoSpecialGenerationMode::Exhaustive => {
            Box::new(exhaustive_pairs_from_single(exhaustive_unsigneds()))
        }
        NoSpecialGenerationMode::Random(scale) => Box::new(random_pairs_from_single(
            u32s_geometric(&EXAMPLE_SEED, scale).flat_map(T::checked_from),
        )),
    }
}

// All pairs of small `T`s, where `T` is unsigned and the first `T` is positive.
pub fn pairs_of_small_unsigneds_single_var_1<T: PrimitiveUnsigned + Rand>(
    gm: NoSpecialGenerationMode,
) -> It<(T, T)> {
    Box::new(pairs_of_small_unsigneds_single(gm).filter(|&(x, _)| x != T::ZERO))
}

pub fn pairs_of_small_unsigneds<T: PrimitiveUnsigned + Rand, U: PrimitiveUnsigned + Rand>(
    gm: NoSpecialGenerationMode,
) -> It<(T, U)> {
    match gm {
        NoSpecialGenerationMode::Exhaustive => Box::new(exhaustive_pairs(
            exhaustive_unsigneds(),
            exhaustive_unsigneds(),
        )),
        NoSpecialGenerationMode::Random(scale) => Box::new(random_pairs(
            &EXAMPLE_SEED,
            &(|seed| u32s_geometric(seed, scale).flat_map(T::checked_from)),
            &(|seed| u32s_geometric(seed, scale).flat_map(U::checked_from)),
        )),
    }
}

// All pairs of small unsigned `T` and small `u64` such that the `T` raised to the `u64` doesn't
// overflow.
pub fn pairs_of_small_unsigneds_var_2<T: PrimitiveUnsigned + Rand>(
    gm: NoSpecialGenerationMode,
) -> It<(T, u64)> {
    Box::new(pairs_of_small_unsigneds::<T, u64>(gm).filter(|&(x, y)| x.checked_pow(y).is_some()))
}

pub fn pairs_of_small_signed_and_small_unsigned<
    T: PrimitiveSigned + Rand,
    U: PrimitiveUnsigned + Rand,
>(
    gm: NoSpecialGenerationMode,
) -> It<(T, U)> {
    match gm {
        NoSpecialGenerationMode::Exhaustive => Box::new(exhaustive_pairs(
            exhaustive_signeds(),
            exhaustive_unsigneds(),
        )),
        NoSpecialGenerationMode::Random(scale) => Box::new(random_pairs(
            &EXAMPLE_SEED,
            &(|seed| i32s_geometric(seed, scale).flat_map(T::checked_from)),
            &(|seed| u32s_geometric(seed, scale).flat_map(U::checked_from)),
        )),
    }
}

// All pairs of u64 and small `U`, where `U` is unsigned and the `u64` is between 1 and `T::WIDTH`,
// inclusive.
pub fn pairs_of_u64_and_small_unsigned_var_1<T: PrimitiveUnsigned, U: PrimitiveUnsigned>(
    gm: NoSpecialGenerationMode,
) -> It<(u64, U)> {
    match gm {
        NoSpecialGenerationMode::Exhaustive => Box::new(exhaustive_pairs(
            primitive_int_increasing_inclusive_range(1, T::WIDTH),
            exhaustive_unsigneds(),
        )),
        NoSpecialGenerationMode::Random(scale) => Box::new(random_pairs(
            &EXAMPLE_SEED,
            &(|seed| random_range(seed, 1, T::WIDTH)),
            &(|seed| u32s_geometric(seed, scale).flat_map(U::checked_from)),
        )),
    }
}

// All pairs of small signed `T` and small `u64` such that the `T` raised to the `u64` doesn't
// overflow.
pub fn pairs_of_small_signed_and_small_u64_var_2<T: PrimitiveSigned + Rand>(
    gm: NoSpecialGenerationMode,
) -> It<(T, u64)> {
    Box::new(
        pairs_of_small_signed_and_small_unsigned::<T, u64>(gm)
            .filter(|&(x, y)| x.checked_pow(y).is_some()),
    )
}

// All pairs of u64 and `usize`, where the `u64` is positive.
pub fn pairs_of_small_u64_and_small_usize_var_2(gm: NoSpecialGenerationMode) -> It<(u64, usize)> {
    match gm {
        NoSpecialGenerationMode::Exhaustive => Box::new(exhaustive_pairs(
            exhaustive_positive_primitive_ints(),
            exhaustive_unsigneds(),
        )),
        NoSpecialGenerationMode::Random(scale) => Box::new(random_pairs(
            &EXAMPLE_SEED,
            &(|seed| range_up_geometric_u32(seed, scale, 1).map(u64::from)),
            &(|seed| u32s_geometric(seed, scale).map(usize::wrapping_from)),
        )),
    }
}

fn sqrt_pairs_of_positive_primitive_and_unsigned<T: PrimitiveInt, U: PrimitiveUnsigned + Rand>(
) -> It<(T, U)> {
    Box::new(sqrt_pairs(
        exhaustive_positive_primitive_ints(),
        exhaustive_unsigneds(),
    ))
}

pub fn pairs_of_positive_unsigned_and_small_unsigned<
    T: PrimitiveUnsigned + Rand,
    U: PrimitiveUnsigned + Rand,
>(
    gm: GenerationMode,
) -> It<(T, U)> {
    match gm {
        GenerationMode::Exhaustive => sqrt_pairs_of_positive_primitive_and_unsigned(),
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
        GenerationMode::Exhaustive => sqrt_pairs_of_positive_primitive_and_unsigned(),
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

pub fn pairs_of_signed_and_small_unsigned<T: PrimitiveSigned + Rand, U: PrimitiveUnsigned>(
    gm: GenerationMode,
) -> It<(T, U)>
where
    T::UnsignedOfEqualWidth: Rand,
    T: WrappingFrom<<T as PrimitiveSigned>::UnsignedOfEqualWidth>,
{
    match gm {
        GenerationMode::Exhaustive => sqrt_pairs_of_signed_and_unsigned(),
        GenerationMode::Random(scale) => random_pairs_of_primitive_and_geometric(scale),
        GenerationMode::SpecialRandom(scale) => Box::new(random_pairs(
            &EXAMPLE_SEED,
            &(|seed| special_random_signed(seed)),
            &(|seed| u32s_geometric(seed, scale).flat_map(U::checked_from)),
        )),
    }
}

// All pairs of `T` and small `U`, where `T` is signed, `U` is unsigned, and the `T` is not
// divisible by 2 to the power of the `U`.
pub fn pairs_of_signed_and_small_unsigned_var_1<
    T: PrimitiveSigned + Rand,
    U: PrimitiveUnsigned + Rand,
>(
    gm: GenerationMode,
) -> It<(T, U)>
where
    T::UnsignedOfEqualWidth: Rand,
    T: WrappingFrom<<T as PrimitiveSigned>::UnsignedOfEqualWidth>,
{
    Box::new(
        pairs_of_signed_and_small_unsigned::<T, U>(gm)
            .filter(|&(ref n, u)| !n.divisible_by_power_of_two(u.exact_into())),
    )
}

// All pairs of signed `T` and `u64`, where the `u64` is between 0 and `U::WIDTH`, inclusive.
pub fn pairs_of_signed_and_small_u64_var_1<T: PrimitiveSigned + Rand, U: PrimitiveUnsigned>(
    gm: GenerationMode,
) -> It<(T, u64)>
where
    T::UnsignedOfEqualWidth: Rand,
    T: WrappingFrom<<T as PrimitiveSigned>::UnsignedOfEqualWidth>,
{
    match gm {
        GenerationMode::Exhaustive => Box::new(lex_pairs(
            exhaustive_signeds(),
            primitive_int_increasing_inclusive_range(0, U::WIDTH),
        )),
        GenerationMode::Random(_) => Box::new(random_pairs(
            &EXAMPLE_SEED,
            &(|seed| random(seed)),
            &(|seed| random_range_down(seed, U::WIDTH)),
        )),
        GenerationMode::SpecialRandom(_) => Box::new(random_pairs(
            &EXAMPLE_SEED,
            &(|seed| special_random_signed(seed)),
            &(|seed| random_range_down(seed, U::WIDTH)),
        )),
    }
}

// All pairs of signed `T` and `u64`, where the `T` is non-negative or the `u64` is between 0 and
// `T::WIDTH`, inclusive.
pub fn pairs_of_signed_and_small_u64_var_2<T: PrimitiveSigned + Rand>(
    gm: GenerationMode,
) -> It<(T, u64)>
where
    T::UnsignedOfEqualWidth: Rand,
    T: WrappingFrom<<T as PrimitiveSigned>::UnsignedOfEqualWidth>,
{
    Box::new(
        pairs_of_signed_and_small_unsigned_var_1(gm)
            .filter(|&(x, y)| x >= T::ZERO || y <= T::WIDTH),
    )
}

// All pairs of signed `T` and `u64`, where the `T` is non-negative or the `u64` is between 0 and
// `T::WIDTH - 1`, inclusive.
pub fn pairs_of_signed_and_small_u64_var_3<T: PrimitiveSigned + Rand>(
    gm: GenerationMode,
) -> It<(T, u64)>
where
    T::UnsignedOfEqualWidth: Rand,
    T: WrappingFrom<<T as PrimitiveSigned>::UnsignedOfEqualWidth>,
{
    Box::new(
        pairs_of_signed_and_small_unsigned_var_1(gm).filter(|&(x, y)| x >= T::ZERO || y < T::WIDTH),
    )
}

/// All pairs of signed `T` and `u64`, where the `T` is non-positive and not `$t::MIN`, or the `u64`
/// is between 0 and `T::WIDTH - 1`, inclusive.
pub fn pairs_of_signed_and_small_u64_var_4<T: PrimitiveSigned + Rand>(
    gm: GenerationMode,
) -> It<(T, u64)>
where
    T::UnsignedOfEqualWidth: Rand,
    T: WrappingFrom<<T as PrimitiveSigned>::UnsignedOfEqualWidth>,
{
    Box::new(
        pairs_of_signed_and_small_unsigned_var_1(gm)
            .filter(|&(x, y)| y < T::WIDTH || (x < T::ZERO && x != T::MIN)),
    )
}

pub fn pairs_of_signed_and_small_signed<T: PrimitiveSigned + Rand, U: PrimitiveSigned>(
    gm: GenerationMode,
) -> It<(T, U)>
where
    T::UnsignedOfEqualWidth: Rand,
    T: WrappingFrom<<T as PrimitiveSigned>::UnsignedOfEqualWidth>,
{
    match gm {
        GenerationMode::Exhaustive => sqrt_pairs_of_signeds(),
        GenerationMode::Random(scale) => random_pairs_of_primitive_and_geometric(scale),
        GenerationMode::SpecialRandom(scale) => Box::new(random_pairs(
            &EXAMPLE_SEED,
            &(|seed| special_random_signed(seed)),
            &(|seed| i32s_geometric(seed, scale).flat_map(U::checked_from)),
        )),
    }
}

type ItU<T> = It<(T, u64)>;

fn exhaustive_pairs_of_unsigned_and_u64_width_range<T: PrimitiveUnsigned + Rand>() -> ItU<T> {
    Box::new(lex_pairs(
        exhaustive_unsigneds(),
        primitive_int_increasing_range(0, T::WIDTH),
    ))
}

fn random_pairs_of_primitive_and_u64_width_range<T: PrimitiveInt + Rand>() -> It<(T, u64)> {
    Box::new(random_pairs(
        &EXAMPLE_SEED,
        &(|seed| random(seed)),
        &(|seed| random_range_down(seed, T::WIDTH - 1)),
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
            &(|seed| random_range_down(seed, T::WIDTH - 1)),
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
            exhaustive_signeds(),
            primitive_int_increasing_range(0, T::WIDTH),
        )),
        GenerationMode::Random(_) => random_pairs_of_primitive_and_u64_width_range(),
        GenerationMode::SpecialRandom(_) => Box::new(random_pairs(
            &EXAMPLE_SEED,
            &(|seed| special_random_signed(seed)),
            &(|seed| random_range_down(seed, T::WIDTH - 1)),
        )),
    }
}

// All pairs of signed `T` and `u64`, where the signed `T` is negative or the `u64` is smaller than
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
            .filter(|&(n, index)| n < T::ZERO || index < T::WIDTH),
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
            .filter(|&(n, index)| n >= T::ZERO || index < T::WIDTH),
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
    Box::new(unfiltered.filter(|&(_, index, bit)| !bit || index < U::exact_from(T::WIDTH)))
}

pub fn triples_of_unsigned_small_unsigned_and_small_unsigned<
    T: PrimitiveUnsigned + Rand,
    U: PrimitiveUnsigned + Rand,
>(
    gm: GenerationMode,
) -> It<(T, U, U)> {
    match gm {
        GenerationMode::Exhaustive => reshape_1_2_to_3(Box::new(sqrt_pairs(
            exhaustive_unsigneds(),
            exhaustive_pairs_from_single(exhaustive_unsigneds()),
        ))),
        GenerationMode::Random(scale) => Box::new(random_triples(
            &EXAMPLE_SEED,
            &(|seed| random(seed)),
            &(|seed| u32s_geometric(seed, scale).flat_map(U::checked_from)),
            &(|seed| u32s_geometric(seed, scale).flat_map(U::checked_from)),
        )),
        GenerationMode::SpecialRandom(scale) => Box::new(random_triples(
            &EXAMPLE_SEED,
            &(|seed| special_random_unsigned(seed)),
            &(|seed| u32s_geometric(seed, scale).flat_map(U::checked_from)),
            &(|seed| u32s_geometric(seed, scale).flat_map(U::checked_from)),
        )),
    }
}

// All triples of T, U, and U, where T and U are unsigned and the first U is less than or equal to
// the second.
pub fn triples_of_unsigned_small_unsigned_and_small_unsigned_var_1<
    T: PrimitiveUnsigned + Rand,
    U: PrimitiveUnsigned + Rand,
>(
    gm: GenerationMode,
) -> It<(T, U, U)> {
    Box::new(
        triples_of_unsigned_small_unsigned_and_small_unsigned(gm)
            .filter(|&(_, start, end)| start <= end),
    )
}

// All triples of unsigned `T`, `u64`, and small `u64`, where the first `u64` is between 1 and
// `U::WIDTH`, inclusive.
pub fn triples_of_unsigned_small_u64_and_small_u64_var_1<
    T: PrimitiveUnsigned + Rand,
    U: PrimitiveUnsigned,
>(
    gm: GenerationMode,
) -> It<(T, u64, u64)> {
    match gm {
        GenerationMode::Exhaustive => Box::new(exhaustive_triples(
            exhaustive_unsigneds(),
            primitive_int_increasing_inclusive_range(1, U::WIDTH),
            exhaustive_unsigneds(),
        )),
        GenerationMode::Random(scale) => Box::new(random_triples(
            &EXAMPLE_SEED,
            &(|seed| random(seed)),
            &(|seed| random_range(seed, 1, U::WIDTH)),
            &(|seed| u32s_geometric(seed, scale).map(u64::from)),
        )),
        GenerationMode::SpecialRandom(scale) => Box::new(random_triples(
            &EXAMPLE_SEED,
            &(|seed| special_random_unsigned(seed)),
            &(|seed| random_range(seed, 1, U::WIDTH)),
            &(|seed| u32s_geometric(seed, scale).map(u64::from)),
        )),
    }
}

// All triples of `T`, `U` and `u64`, where `T` and `U` are unsigned and the `u64` is between the
// number of significant_bits of the first `T` and `T::WIDTH`, inclusive.
pub fn triples_of_unsigned_small_unsigned_and_small_unsigned_var_3<
    T: PrimitiveUnsigned + Rand + SampleRange,
    U: PrimitiveUnsigned,
>(
    gm: NoSpecialGenerationMode,
) -> It<(T, U, u64)> {
    match gm {
        NoSpecialGenerationMode::Exhaustive => reshape_2_1_to_3(Box::new(dependent_pairs(
            sqrt_pairs_of_unsigneds(),
            |&(x, _): &(T, _)| {
                Box::new(primitive_int_increasing_inclusive_range(
                    x.significant_bits(),
                    T::WIDTH,
                ))
            },
        ))),
        NoSpecialGenerationMode::Random(scale) => {
            reshape_2_1_to_3(permute_2_1(Box::new(random_dependent_pairs(
                scale,
                random_range(&scramble(&EXAMPLE_SEED, "pow"), 0, T::WIDTH),
                |&scale, &pow| {
                    Box::new(random_pairs(
                        &scramble(&EXAMPLE_SEED, "u"),
                        &(|seed| random_range::<T>(seed, T::ZERO, T::low_mask(pow))),
                        &(|seed| u32s_geometric(seed, scale).flat_map(U::checked_from)),
                    ))
                },
            ))))
        }
    }
}

// All triples of T, U, and U, where T is signed, U is unsigned, the first U is less than or equal
// to the second, and if the T is negative, the difference between the two Us is no greater than the
// width of T.
pub fn triples_of_signed_small_unsigned_and_small_unsigned_var_1<
    T: PrimitiveSigned + Rand,
    U: PrimitiveUnsigned + Rand + ExactFrom<u64>,
>(
    gm: GenerationMode,
) -> It<(T, U, U)>
where
    T::UnsignedOfEqualWidth: Rand,
    T: WrappingFrom<<T as PrimitiveSigned>::UnsignedOfEqualWidth>,
{
    let ts = match gm {
        GenerationMode::Exhaustive => reshape_1_2_to_3(Box::new(sqrt_pairs(
            exhaustive_signeds(),
            exhaustive_pairs_from_single(exhaustive_unsigneds()),
        ))),
        GenerationMode::Random(scale) => Box::new(random_triples(
            &EXAMPLE_SEED,
            &(|seed| random(seed)),
            &(|seed| u32s_geometric(seed, scale).flat_map(U::checked_from)),
            &(|seed| u32s_geometric(seed, scale).flat_map(U::checked_from)),
        )),
        GenerationMode::SpecialRandom(scale) => Box::new(random_triples(
            &EXAMPLE_SEED,
            &(|seed| special_random_signed(seed)),
            &(|seed| u32s_geometric(seed, scale).flat_map(U::checked_from)),
            &(|seed| u32s_geometric(seed, scale).flat_map(U::checked_from)),
        )),
    };
    Box::new(ts.filter(|&(n, start, end)| {
        start <= end && (n > T::ZERO || end - start <= U::exact_from(T::WIDTH))
    }))
}

// All triples of T, U, and U, where T and U are unsigned, T is positive, and the first U is less
// than or equal to the second.
pub fn triples_of_positive_unsigned_small_unsigned_and_small_unsigned_var_1<
    T: PrimitiveUnsigned + Rand,
    U: PrimitiveUnsigned + Rand,
>(
    gm: GenerationMode,
) -> It<(T, U, U)> {
    let ts = match gm {
        GenerationMode::Exhaustive => reshape_1_2_to_3(Box::new(sqrt_pairs(
            exhaustive_positive_primitive_ints(),
            exhaustive_pairs_from_single(exhaustive_unsigneds()),
        ))),
        GenerationMode::Random(scale) => Box::new(random_triples(
            &EXAMPLE_SEED,
            &(|seed| random_positive_unsigned(seed)),
            &(|seed| u32s_geometric(seed, scale).flat_map(U::checked_from)),
            &(|seed| u32s_geometric(seed, scale).flat_map(U::checked_from)),
        )),
        GenerationMode::SpecialRandom(scale) => Box::new(random_triples(
            &EXAMPLE_SEED,
            &(|seed| special_random_positive_unsigned(seed)),
            &(|seed| u32s_geometric(seed, scale).flat_map(U::checked_from)),
            &(|seed| u32s_geometric(seed, scale).flat_map(U::checked_from)),
        )),
    };
    Box::new(ts.filter(|&(_, start, end)| start <= end))
}

pub fn triples_of_unsigned_small_unsigned_and_unsigned<
    T: PrimitiveUnsigned + Rand,
    U: PrimitiveUnsigned + Rand,
>(
    gm: GenerationMode,
) -> It<(T, U, T)> {
    match gm {
        GenerationMode::Exhaustive => permute_1_3_2(reshape_2_1_to_3(Box::new(sqrt_pairs(
            exhaustive_pairs_from_single(exhaustive_unsigneds()),
            exhaustive_unsigneds(),
        )))),
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
            &(|seed| special_random_unsigned(seed)),
        )),
    }
}

// All triples of `T`, small `U`, and `T`, where `T` and `U` are unsigned and the first `T`s is
// smaller than the second.
pub fn triples_of_unsigned_small_unsigned_and_unsigned_var_1<
    T: PrimitiveUnsigned + Rand,
    U: PrimitiveUnsigned + Rand,
>(
    gm: GenerationMode,
) -> It<(T, U, T)> {
    Box::new(triples_of_unsigned_small_unsigned_and_unsigned(gm).filter(|&(x, _, m)| x < m))
}

pub fn triples_of_signed_small_unsigned_and_unsigned<
    T: PrimitiveSigned + Rand,
    U: PrimitiveUnsigned + Rand,
    V: PrimitiveUnsigned + Rand,
>(
    gm: GenerationMode,
) -> It<(T, U, V)>
where
    T::UnsignedOfEqualWidth: Rand,
    T: WrappingFrom<<T as PrimitiveSigned>::UnsignedOfEqualWidth>,
{
    match gm {
        GenerationMode::Exhaustive => permute_1_3_2(reshape_2_1_to_3(Box::new(sqrt_pairs(
            exhaustive_pairs(exhaustive_signeds(), exhaustive_unsigneds()),
            exhaustive_unsigneds(),
        )))),
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
            &(|seed| special_random_unsigned(seed)),
        )),
    }
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
    Box::new(
        unfiltered
            .filter(|&(n, index, bit)| index < U::exact_from(T::WIDTH) || bit == (n < T::ZERO)),
    )
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
        GenerationMode::Exhaustive => Box::new(sqrt_pairs(
            exhaustive_negative_signeds().filter(|&i| i != T::MIN),
            exhaustive_unsigneds(),
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
            exhaustive_pairs_from_single(exhaustive_signeds()),
            exhaustive_unsigneds(),
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

fn random_pairs_of_primitive_and_rounding_mode<T: PrimitiveInt + Rand>() -> ItR<T> {
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
            exhaustive_unsigneds(),
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

pub fn pairs_of_positive_unsigned_and_rounding_mode<T: PrimitiveUnsigned + Rand>(
    gm: GenerationMode,
) -> It<(T, RoundingMode)> {
    match gm {
        GenerationMode::Exhaustive => Box::new(lex_pairs(
            exhaustive_positive_primitive_ints(),
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

pub fn pairs_of_signed_and_rounding_mode<T: PrimitiveSigned + Rand>(
    gm: GenerationMode,
) -> It<(T, RoundingMode)>
where
    T::UnsignedOfEqualWidth: Rand,
    T: WrappingFrom<<T as PrimitiveSigned>::UnsignedOfEqualWidth>,
{
    match gm {
        GenerationMode::Exhaustive => {
            Box::new(lex_pairs(exhaustive_signeds(), exhaustive_rounding_modes()))
        }
        GenerationMode::Random(_) => random_pairs_of_primitive_and_rounding_mode(),
        GenerationMode::SpecialRandom(_) => Box::new(random_pairs(
            &EXAMPLE_SEED,
            &(|seed| special_random_signed(seed)),
            &(|seed| random_rounding_modes(seed)),
        )),
    }
}

pub fn pairs_of_nonzero_signed_and_rounding_mode<T: PrimitiveSigned + Rand>(
    gm: GenerationMode,
) -> It<(T, RoundingMode)>
where
    T::UnsignedOfEqualWidth: Rand,
    T: WrappingFrom<<T as PrimitiveSigned>::UnsignedOfEqualWidth>,
{
    Box::new(pairs_of_signed_and_rounding_mode(gm).filter(|&(x, _)| x != T::ZERO))
}

// All pairs of `T`, and `RoundingMode`, where `T` is signed and the `T` is not equal to `T::MIN`.
pub fn pairs_of_signed_and_rounding_mode_var_1<T: PrimitiveSigned + Rand>(
    gm: GenerationMode,
) -> It<(T, RoundingMode)>
where
    T::UnsignedOfEqualWidth: Rand,
    T: WrappingFrom<<T as PrimitiveSigned>::UnsignedOfEqualWidth>,
{
    Box::new(pairs_of_signed_and_rounding_mode(gm).filter(|&(x, _)| x != T::MIN))
}

// All pairs of `T`, and `RoundingMode`, where `T` is signed and the `T` is not equal to zero or
// `T::MIN`.
pub fn pairs_of_signed_and_rounding_mode_var_2<T: PrimitiveSigned + Rand>(
    gm: GenerationMode,
) -> It<(T, RoundingMode)>
where
    T::UnsignedOfEqualWidth: Rand,
    T: WrappingFrom<<T as PrimitiveSigned>::UnsignedOfEqualWidth>,
{
    Box::new(pairs_of_signed_and_rounding_mode(gm).filter(|&(x, _)| x != T::ZERO && x != T::MIN))
}

pub fn sextuples_of_unsigneds<T: PrimitiveUnsigned + Rand>(
    gm: GenerationMode,
) -> It<(T, T, T, T, T, T)> {
    match gm {
        GenerationMode::Exhaustive => {
            Box::new(exhaustive_sextuples_from_single(exhaustive_unsigneds()))
        }
        GenerationMode::Random(_) => Box::new(random_sextuples_from_single(random(&EXAMPLE_SEED))),
        GenerationMode::SpecialRandom(_) => Box::new(random_sextuples_from_single(
            special_random_unsigned(&EXAMPLE_SEED),
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
            exhaustive_unsigneds(),
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

pub fn octuples_of_unsigneds<T: PrimitiveUnsigned + Rand>(
    gm: GenerationMode,
) -> It<(T, T, T, T, T, T, T, T)> {
    match gm {
        GenerationMode::Exhaustive => {
            Box::new(exhaustive_octuples_from_single(exhaustive_unsigneds()))
        }
        GenerationMode::Random(_) => Box::new(random_octuples_from_single(random(&EXAMPLE_SEED))),
        GenerationMode::SpecialRandom(_) => Box::new(random_octuples_from_single(
            special_random_unsigned(&EXAMPLE_SEED),
        )),
    }
}

pub fn nonuples_of_unsigneds<T: PrimitiveUnsigned + Rand>(
    gm: GenerationMode,
) -> It<(T, T, T, T, T, T, T, T, T)> {
    let ts: It<((T, T, T), (T, T, T), (T, T, T))> = match gm {
        GenerationMode::Exhaustive => Box::new(exhaustive_triples_from_single(
            exhaustive_triples_from_single(exhaustive_unsigneds()),
        )),
        GenerationMode::Random(_) => Box::new(random_triples_from_single(
            random_triples_from_single(random(&EXAMPLE_SEED)),
        )),
        GenerationMode::SpecialRandom(_) => Box::new(random_triples_from_single(
            random_triples_from_single(special_random_unsigned(&EXAMPLE_SEED)),
        )),
    };
    reshape_3_3_3_to_9(ts)
}

/// All nonuples of `Limb` that are valid inputs to _limbs_mod_mul_two_limbs.
pub fn nonuples_of_limbs_var_1(
    gm: GenerationMode,
) -> It<(Limb, Limb, Limb, Limb, Limb, Limb, Limb, Limb, Limb)> {
    Box::new(
        sextuples_of_unsigneds(gm).filter_map(|(x_1, x_0, y_1, y_0, m_1, m_0)| {
            if m_1 == 0
                || m_1 == 1 && m_0 == 0
                || x_1 > m_1
                || y_1 > m_1
                || x_1 == m_1 && x_0 > m_0
                || y_1 == m_1 && y_0 > m_0
            {
                None
            } else {
                let (inv_2, inv_1, inv_0) = _limbs_precompute_mod_mul_two_limbs(m_1, m_0);
                Some((x_1, x_0, y_1, y_0, m_1, m_0, inv_2, inv_1, inv_0))
            }
        }),
    )
}

pub fn duodecuples_of_unsigneds<T: PrimitiveUnsigned + Rand>(
    gm: GenerationMode,
) -> It<(T, T, T, T, T, T, T, T, T, T, T, T)> {
    let qs: It<((T, T, T, T), (T, T, T, T), (T, T, T, T))> = match gm {
        GenerationMode::Exhaustive => Box::new(exhaustive_triples_from_single(
            exhaustive_quadruples_from_single(exhaustive_unsigneds()),
        )),
        GenerationMode::Random(_) => Box::new(random_triples_from_single(
            random_quadruples_from_single(random(&EXAMPLE_SEED)),
        )),
        GenerationMode::SpecialRandom(_) => Box::new(random_triples_from_single(
            random_quadruples_from_single(special_random_unsigned(&EXAMPLE_SEED)),
        )),
    };
    reshape_4_4_4_to_12(qs)
}

fn vecs_of_unsigned_with_seed<T: PrimitiveUnsigned + Rand>(
    gm: GenerationMode,
    seed: &[u32],
) -> It<Vec<T>> {
    match gm {
        GenerationMode::Exhaustive => Box::new(exhaustive_vecs(exhaustive_unsigneds())),
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
    Box::new(vecs_of_unsigned(gm).filter(|limbs| !slice_test_zero(limbs)))
}

// All `Vec<Limb>` that are nonempty and represent a `Natural` divisible by 3.
pub fn vecs_of_limb_var_4(gm: GenerationMode) -> It<Vec<Limb>> {
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
            exhaustive_unsigneds(),
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
            exhaustive_vecs(exhaustive_unsigneds())
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
            .filter(|&(ref xs, ref ys)| !slice_test_zero(xs) && !slice_test_zero(ys)),
    )
}

// All pairs of `Vec<T>` where `T` is unsigned, both elements are nonempty and don't only contain
// zeros, and the first element is at least as long as the second.
pub fn pairs_of_unsigned_vec_var_7<T: PrimitiveUnsigned + Rand>(
    gm: GenerationMode,
) -> It<(Vec<T>, Vec<T>)> {
    Box::new(pairs_of_unsigned_vec(gm).filter(|&(ref xs, ref ys)| {
        xs.len() >= ys.len() && !slice_test_zero(xs) && !slice_test_zero(ys)
    }))
}

// All pairs of `Vec<Limb>`, where the first `Vec` is at least as long as the second and the second
// `Vec` is nonempty and represents a `Natural` divisible by 3.
pub fn pairs_of_limb_vec_var_8(gm: GenerationMode) -> It<(Vec<Limb>, Vec<Limb>)> {
    Box::new(
        pairs_of_unsigned_vec(gm)
            .map(|(out, in_limbs)| (out, limbs_mul_limb(&in_limbs, 3)))
            .filter(|(ref out, ref in_limbs)| out.len() >= in_limbs.len() && !in_limbs.is_empty()),
    )
}

// All pairs of `Vec<Limb>`, where `ns` and `ds` meet the preconditions of `limbs_div_mod`.
pub fn pairs_of_limb_vec_var_9(gm: GenerationMode) -> It<(Vec<Limb>, Vec<Limb>)> {
    let qs: It<(Vec<Limb>, Vec<Limb>)> = match gm {
        GenerationMode::Exhaustive => Box::new(exhaustive_pairs_from_single(
            exhaustive_vecs_min_length(2, exhaustive_unsigneds()),
        )),
        GenerationMode::Random(scale) => Box::new(random_pairs_from_single(
            random_vecs_min_length(&EXAMPLE_SEED, scale, 2, &(|seed_2| random(seed_2))),
        )),
        GenerationMode::SpecialRandom(scale) => Box::new(random_pairs_from_single(
            special_random_unsigned_vecs_min_length(&EXAMPLE_SEED, scale, 2),
        )),
    };
    Box::new(qs.filter(|(n, d)| *d.last().unwrap() != 0 && n.len() >= d.len()))
}

// All pairs of `Vec<T>`, where `T` is unsigned and `ns` and `ds` meet the preconditions of
// `limbs_mod_by_two_limb_normalized`.
pub fn pairs_of_unsigned_vec_var_10<T: PrimitiveUnsigned + Rand>(
    gm: GenerationMode,
) -> It<(Vec<T>, Vec<T>)> {
    let ps: It<(Vec<T>, (T, T))> = match gm {
        GenerationMode::Exhaustive => Box::new(exhaustive_pairs(
            exhaustive_vecs_min_length(2, exhaustive_unsigneds()),
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
            exhaustive_vecs(exhaustive_unsigneds())
                .filter(|limbs| !limbs.is_empty() && *limbs.last().unwrap() != T::ZERO),
        )),
        _ => Box::new(random_pairs_from_single(vecs_of_unsigned_var_1(gm))),
    };
    Box::new(ps.filter(|(n, d)| n.len() >= d.len()))
}

// All pairs of `Vec<Limb>`, where `ns` and `ds` meet the preconditions of both `limbs_div_mod` and
// `limbs_divisible_by`.
pub fn pairs_of_limb_vec_var_14(gm: GenerationMode) -> It<(Vec<Limb>, Vec<Limb>)> {
    let qs: It<(Vec<Limb>, Vec<Limb>)> = match gm {
        GenerationMode::Exhaustive => Box::new(exhaustive_pairs_from_single(
            exhaustive_vecs_min_length(2, exhaustive_unsigneds()),
        )),
        GenerationMode::Random(scale) => Box::new(random_pairs_from_single(
            random_vecs_min_length(&EXAMPLE_SEED, scale, 2, &(|seed_2| random(seed_2))),
        )),
        GenerationMode::SpecialRandom(scale) => Box::new(random_pairs_from_single(
            special_random_unsigned_vecs_min_length(&EXAMPLE_SEED, scale, 2),
        )),
    };
    Box::new(
        qs.filter(|(n, d)| {
            *n.last().unwrap() != 0 && *d.last().unwrap() != 0 && n.len() >= d.len()
        }),
    )
}

// All triples of `Vec<Limb>`, where `ns` and `ds` meet the preconditions of `limbs_div_exact` and
// `limbs_divisible_by`.
pub fn pairs_of_limb_vec_var_15(gm: GenerationMode) -> It<(Vec<Limb>, Vec<Limb>)> {
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

// All pairs of `Vec<Limb>`, where `ns` and `ds` meet the preconditions of `limbs_div_exact`.
pub fn pairs_of_limb_vec_var_16(gm: GenerationMode) -> It<(Vec<Limb>, Vec<Limb>)> {
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

// All pairs of `Vec<T>`, where `T` is unsigned and `out` and `xs` meet the preconditions of
// `_limbs_square_to_out_basecase`.
pub fn pairs_of_unsigned_vec_var_17<T: PrimitiveUnsigned + Rand>(
    gm: GenerationMode,
) -> It<(Vec<T>, Vec<T>)> {
    Box::new(pairs_of_unsigned_vec(gm).filter(|&(ref out, ref xs)| {
        !xs.is_empty() && xs.len() <= SQR_TOOM2_THRESHOLD && out.len() >= xs.len() << 1
    }))
}

// All pairs of `Vec<T>`, where `T` is unsigned and `out` and `xs` meet the preconditions of
// `_limbs_square_to_out_toom_2`.
pub fn pairs_of_unsigned_vec_var_18<T: PrimitiveUnsigned + Rand>(
    gm: GenerationMode,
) -> It<(Vec<T>, Vec<T>)> {
    Box::new(
        pairs_of_unsigned_vec_min_sizes(gm, 4, 2)
            .filter(|&(ref out, ref xs)| out.len() >= xs.len() << 1),
    )
}

// All pairs of `Vec<T>`, where `T` is unsigned and `out` and `xs` meet the preconditions of
// `_limbs_square_to_out_toom_3`.
pub fn pairs_of_unsigned_vec_var_19<T: PrimitiveUnsigned + Rand>(
    gm: GenerationMode,
) -> It<(Vec<T>, Vec<T>)> {
    Box::new(
        pairs_of_unsigned_vec_min_sizes(gm, 6, 3).filter(|&(ref out, ref xs)| {
            out.len() >= xs.len() << 1 && _limbs_square_to_out_toom_3_input_size_valid(xs.len())
        }),
    )
}

// All pairs of `Vec<T>`, where `T` is unsigned and `out` and `xs` meet the preconditions of
// `_limbs_square_to_out_toom_4`.
pub fn pairs_of_unsigned_vec_var_20<T: PrimitiveUnsigned + Rand>(
    gm: GenerationMode,
) -> It<(Vec<T>, Vec<T>)> {
    Box::new(
        pairs_of_unsigned_vec_min_sizes(gm, 8, 4).filter(|&(ref out, ref xs)| {
            out.len() >= xs.len() << 1 && _limbs_square_to_out_toom_4_input_size_valid(xs.len())
        }),
    )
}

// All pairs of `Vec<T>`, where `T` is unsigned and `out` and `xs` meet the preconditions of both
// `_limbs_square_to_out_toom_3` and `_limbs_square_to_out_toom_4`.
pub fn pairs_of_unsigned_vec_var_21<T: PrimitiveUnsigned + Rand>(
    gm: GenerationMode,
) -> It<(Vec<T>, Vec<T>)> {
    Box::new(
        pairs_of_unsigned_vec_min_sizes(gm, 14, 7).filter(|&(ref out, ref xs)| {
            let xs_len = xs.len();
            out.len() >= xs_len << 1 && (xs_len == 7 || xs_len == 8 || xs_len > 9)
        }),
    )
}

// All pairs of `Vec<T>`, where `T` is unsigned and `out` and `xs` meet the preconditions of
// `_limbs_square_to_out_toom_6`.
pub fn pairs_of_unsigned_vec_var_22<T: PrimitiveUnsigned + Rand>(
    gm: GenerationMode,
) -> It<(Vec<T>, Vec<T>)> {
    Box::new(
        pairs_of_unsigned_vec_min_sizes(gm, 36, 18).filter(|&(ref out, ref xs)| {
            out.len() >= xs.len() << 1 && _limbs_square_to_out_toom_6_input_size_valid(xs.len())
        }),
    )
}

// All pairs of `Vec<T>`, where `T` is unsigned and `out` and `xs` meet the preconditions of
// `_limbs_square_to_out_toom_8`.
pub fn pairs_of_unsigned_vec_var_23<T: PrimitiveUnsigned + Rand>(
    gm: GenerationMode,
) -> It<(Vec<T>, Vec<T>)> {
    Box::new(
        pairs_of_unsigned_vec_min_sizes(gm, 80, 40).filter(|&(ref out, ref xs)| {
            out.len() >= xs.len() << 1 && _limbs_square_to_out_toom_8_input_size_valid(xs.len())
        }),
    )
}

// All pairs of `Vec<T>`, where `T` is unsigned, the second `Vec` is nonempty, and the first `Vec`
// is at least twice as long as the second.
pub fn pairs_of_unsigned_vec_var_24<T: PrimitiveUnsigned + Rand>(
    gm: GenerationMode,
) -> It<(Vec<T>, Vec<T>)> {
    Box::new(
        pairs_of_unsigned_vec_min_sizes(gm, 2, 1)
            .filter(|&(ref out, ref xs)| out.len() >= xs.len() << 1),
    )
}

// Some pairs of `Vec<T>`, where `T` is unsigned and `out`, `xs`, and `xs` would trigger the actual
// FFT code of `_limbs_mul_greater_to_out_fft`.
pub fn pairs_of_unsigned_vec_var_25<T: PrimitiveUnsigned + Rand>(
    gm: GenerationMode,
) -> It<(Vec<T>, Vec<T>)> {
    Box::new(
        pairs_of_unsigned_vec_min_sizes(gm, 30, 15).filter(|&(ref out, ref xs)| {
            out.len() >= xs.len() << 1
                && _limbs_mul_greater_to_out_fft_input_sizes_threshold(xs.len(), xs.len())
        }),
    )
}

fn pairs_of_unsigned_vec_and_bool<T: PrimitiveUnsigned + Rand>(
    gm: GenerationMode,
) -> It<(Vec<T>, bool)> {
    match gm {
        GenerationMode::Exhaustive => Box::new(log_pairs(
            exhaustive_vecs(exhaustive_unsigneds()),
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
pub fn triples_of_two_limb_vecs_and_limb_var_1(
    gm: GenerationMode,
) -> It<(Vec<Limb>, Vec<Limb>, Limb)> {
    let ts: It<(Vec<Limb>, Vec<Limb>, Limb)> = match gm {
        GenerationMode::Exhaustive => Box::new(exhaustive_triples(
            exhaustive_vecs_min_length(3, exhaustive_unsigneds()),
            exhaustive_vecs_min_length(2, exhaustive_unsigneds()),
            primitive_int_increasing_inclusive_range(
                Limb::power_of_two(Limb::WIDTH - 1),
                Limb::MAX,
            ),
        )),
        GenerationMode::Random(scale) => Box::new(random_triples(
            &EXAMPLE_SEED,
            &(|seed| random_vecs_min_length(seed, scale, 3, &(|seed_2| random(seed_2)))),
            &(|seed| random_vecs_min_length(seed, scale, 2, &(|seed_2| random(seed_2)))),
            &(|seed| random_range_up(seed, Limb::power_of_two(Limb::WIDTH - 1))),
        )),
        GenerationMode::SpecialRandom(scale) => Box::new(random_triples(
            &EXAMPLE_SEED,
            &(|seed| special_random_unsigned_vecs_min_length(seed, scale, 3)),
            &(|seed| special_random_unsigned_vecs_min_length(seed, scale, 2)),
            &(|seed| random_range_up(seed, Limb::power_of_two(Limb::WIDTH - 1))),
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
            exhaustive_unsigneds(),
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

fn quadruples_of_unsigned_vec<T: PrimitiveUnsigned + Rand>(
    gm: GenerationMode,
) -> It<(Vec<T>, Vec<T>, Vec<T>, Vec<T>)> {
    match gm {
        GenerationMode::Exhaustive => Box::new(exhaustive_quadruples_from_single(exhaustive_vecs(
            exhaustive_unsigneds(),
        ))),
        GenerationMode::Random(scale) => Box::new(random_quadruples_from_single(random_vecs(
            &EXAMPLE_SEED,
            scale,
            &(|seed| random(seed)),
        ))),
        GenerationMode::SpecialRandom(scale) => Box::new(random_quadruples_from_single(
            special_random_unsigned_vecs(&EXAMPLE_SEED, scale),
        )),
    }
}

// All quadruples of `Vec<Limb>` that are valid inputs to `limbs_mod_pow_odd`.
pub fn quadruples_of_unsigned_vec_var_1(
    gm: GenerationMode,
) -> It<(Vec<Limb>, Vec<Limb>, Vec<Limb>, Vec<Limb>)> {
    Box::new(
        quadruples_of_unsigned_vec::<Limb>(gm).filter(|&(ref out, ref bs, ref es, ref ms)| {
            !bs.is_empty()
                && !ms.is_empty()
                && out.len() >= ms.len()
                && (es.len() > 1 || es.len() == 1 && es[0] > 1)
                && *bs.last().unwrap() != 0
                && *es.last().unwrap() != 0
                && *ms.last().unwrap() != 0
                && ms[0].odd()
                && limbs_cmp(bs, ms) == Ordering::Less
        }),
    )
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
            exhaustive_vecs(exhaustive_unsigneds())
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
            exhaustive_vecs(exhaustive_unsigneds()),
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
            xs.len() >= ys.len() && !slice_test_zero(ys) && !slice_test_zero(zs)
        }),
    )
}

// All triples of `Vec<Limb>` where the second and third elements are nonempty and don't only
// contain zeros, and the first is at least as long as the third.
pub fn triples_of_limb_vec_var_6(gm: GenerationMode) -> It<(Vec<Limb>, Vec<Limb>, Vec<Limb>)> {
    Box::new(
        triples_of_unsigned_vec(gm).filter(|&(ref xs, ref ys, ref zs)| {
            xs.len() >= zs.len() && !slice_test_zero(ys) && !slice_test_zero(zs)
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
                && !slice_test_zero(ys)
                && !slice_test_zero(zs)
        }),
    )
}

// All triples of `Vec<Limb>` where the second and third elements are nonempty and don't only
// contain zeros, and the first is at least as long as the second or at least as long as the third.
pub fn triples_of_limb_vec_var_8(gm: GenerationMode) -> It<(Vec<Limb>, Vec<Limb>, Vec<Limb>)> {
    Box::new(
        triples_of_unsigned_vec(gm).filter(|&(ref xs, ref ys, ref zs)| {
            (xs.len() >= ys.len() || xs.len() >= zs.len())
                && !slice_test_zero(ys)
                && !slice_test_zero(zs)
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
        triples_of_unsigned_vec_min_sizes(gm, 4, 2, 2).filter(|&(ref out, ref xs, ref ys)| {
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
    Box::new(
        triples_of_unsigned_vec_min_sizes(gm, 6, 3, 3).filter(|&(ref out, ref xs, ref ys)| {
            out.len() >= xs.len() + ys.len()
                && _limbs_mul_greater_to_out_toom_33_input_sizes_valid(xs.len(), ys.len())
        }),
    )
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
            exhaustive_vecs_min_length(2, exhaustive_unsigneds()),
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
            exhaustive_vecs(exhaustive_unsigneds())
                .filter(|limbs| !limbs.is_empty() && *limbs.last().unwrap() != T::ZERO),
        )),
        _ => Box::new(random_triples_from_single(vecs_of_unsigned_var_1(gm))),
    }
}

// All triples of `Vec<T>`, where `T` is unsigned, `xs` and `ys` have the same length, and `out`,
// `xs`, and `ys` meet the preconditions of `_limbs_mul_greater_to_out_toom_33`.
pub fn triples_of_unsigned_vec_var_30<T: PrimitiveUnsigned + Rand>(
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
pub fn triples_of_unsigned_vec_var_31<T: PrimitiveUnsigned + Rand>(
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
pub fn triples_of_unsigned_vec_var_32<T: PrimitiveUnsigned + Rand>(
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
pub fn triples_of_unsigned_vec_var_33<T: PrimitiveUnsigned + Rand>(
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
pub fn triples_of_unsigned_vec_var_34<T: PrimitiveUnsigned + Rand>(
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
pub fn triples_of_unsigned_vec_var_35<T: PrimitiveUnsigned + Rand>(
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
pub fn triples_of_unsigned_vec_var_36<T: PrimitiveUnsigned + Rand>(
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
            exhaustive_vecs(exhaustive_unsigneds()),
            exhaustive_vecs_min_length(2, exhaustive_unsigneds()),
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

// All triples of `Vec<Limb>` where the first `Vec` is at least as long as the second, the third is
// at least as long as twice the length of the second, and the second is nonempty and its most
// significant bit is set.
pub fn triples_of_limb_vec_var_38(gm: GenerationMode) -> It<(Vec<Limb>, Vec<Limb>, Vec<Limb>)> {
    let qs: It<(Vec<Limb>, Vec<Limb>, Vec<Limb>, Limb)> = match gm {
        GenerationMode::Exhaustive => Box::new(exhaustive_quadruples(
            exhaustive_vecs(exhaustive_unsigneds()),
            exhaustive_vecs_min_length(2, exhaustive_unsigneds()),
            exhaustive_vecs(exhaustive_unsigneds()),
            primitive_int_increasing_inclusive_range(
                Limb::power_of_two(Limb::WIDTH - 1),
                Limb::MAX,
            ),
        )),
        GenerationMode::Random(scale) => Box::new(random_quadruples(
            &EXAMPLE_SEED,
            &(|seed| random_vecs(seed, scale, &(|seed_2| random(seed_2)))),
            &(|seed| random_vecs_min_length(seed, scale, 2, &(|seed_2| random(seed_2)))),
            &(|seed| random_vecs(seed, scale, &(|seed_2| random(seed_2)))),
            &(|seed| random_range_up(seed, Limb::power_of_two(Limb::WIDTH - 1))),
        )),
        GenerationMode::SpecialRandom(scale) => Box::new(random_quadruples(
            &EXAMPLE_SEED,
            &(|seed| special_random_unsigned_vecs(seed, scale)),
            &(|seed| special_random_unsigned_vecs_min_length(seed, scale, 2)),
            &(|seed| special_random_unsigned_vecs(seed, scale)),
            &(|seed| random_range_up(seed, Limb::power_of_two(Limb::WIDTH - 1))),
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

// All triples of `Vec<Limb>` where the first `Vec` is at least as long as the second, the third is
// at least as long as twice the length of the second, and the second has length at least 5 and its
// most significant bit is set.
pub fn triples_of_limb_vec_var_39(gm: GenerationMode) -> It<(Vec<Limb>, Vec<Limb>, Vec<Limb>)> {
    let qs: It<(Vec<Limb>, Vec<Limb>, Vec<Limb>, Limb)> = match gm {
        GenerationMode::Exhaustive => Box::new(exhaustive_quadruples(
            exhaustive_vecs(exhaustive_unsigneds()),
            exhaustive_vecs_min_length(10, exhaustive_unsigneds()),
            exhaustive_vecs_min_length(4, exhaustive_unsigneds()),
            primitive_int_increasing_inclusive_range(
                Limb::power_of_two(Limb::WIDTH - 1),
                Limb::MAX,
            ),
        )),
        GenerationMode::Random(scale) => Box::new(random_quadruples(
            &EXAMPLE_SEED,
            &(|seed| random_vecs(seed, scale, &(|seed_2| random(seed_2)))),
            &(|seed| random_vecs_min_length(seed, scale, 10, &(|seed_2| random(seed_2)))),
            &(|seed| random_vecs_min_length(seed, scale, 4, &(|seed_2| random(seed_2)))),
            &(|seed| random_range_up(seed, Limb::power_of_two(Limb::WIDTH - 1))),
        )),
        GenerationMode::SpecialRandom(scale) => Box::new(random_quadruples(
            &EXAMPLE_SEED,
            &(|seed| special_random_unsigned_vecs(seed, scale)),
            &(|seed| special_random_unsigned_vecs_min_length(seed, scale, 10)),
            &(|seed| special_random_unsigned_vecs_min_length(seed, scale, 4)),
            &(|seed| random_range_up(seed, Limb::power_of_two(Limb::WIDTH - 1))),
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
pub fn triples_of_limb_vec_var_40(gm: GenerationMode) -> It<(Vec<Limb>, Vec<Limb>, Vec<Limb>)> {
    let qs: It<(Vec<Limb>, Vec<Limb>, Vec<Limb>, Limb)> = match gm {
        GenerationMode::Exhaustive => Box::new(exhaustive_quadruples(
            exhaustive_vecs_min_length(3, exhaustive_unsigneds()),
            exhaustive_vecs_min_length(9, exhaustive_unsigneds()),
            exhaustive_vecs_min_length(5, exhaustive_unsigneds()),
            primitive_int_increasing_inclusive_range(
                Limb::power_of_two(Limb::WIDTH - 1),
                Limb::MAX,
            ),
        )),
        GenerationMode::Random(scale) => Box::new(random_quadruples(
            &EXAMPLE_SEED,
            &(|seed| random_vecs_min_length(seed, scale, 3, &(|seed_2| random(seed_2)))),
            &(|seed| random_vecs_min_length(seed, scale, 9, &(|seed_2| random(seed_2)))),
            &(|seed| random_vecs_min_length(seed, scale, 5, &(|seed_2| random(seed_2)))),
            &(|seed| random_range_up(seed, Limb::power_of_two(Limb::WIDTH - 1))),
        )),
        GenerationMode::SpecialRandom(scale) => Box::new(random_quadruples(
            &EXAMPLE_SEED,
            &(|seed| special_random_unsigned_vecs_min_length(seed, scale, 3)),
            &(|seed| special_random_unsigned_vecs_min_length(seed, scale, 9)),
            &(|seed| special_random_unsigned_vecs_min_length(seed, scale, 5)),
            &(|seed| random_range_up(seed, Limb::power_of_two(Limb::WIDTH - 1))),
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
pub fn triples_of_limb_vec_var_41(gm: GenerationMode) -> It<(Vec<Limb>, Vec<Limb>, Vec<Limb>)> {
    let qs: It<(Vec<Limb>, Vec<Limb>, Vec<Limb>, Limb)> = match gm {
        GenerationMode::Exhaustive => Box::new(exhaustive_quadruples(
            exhaustive_vecs(exhaustive_unsigneds()),
            exhaustive_vecs_min_length(2, exhaustive_unsigneds()),
            exhaustive_vecs_min_length(1, exhaustive_unsigneds()),
            primitive_int_increasing_inclusive_range(
                Limb::power_of_two(Limb::WIDTH - 1),
                Limb::MAX,
            ),
        )),
        GenerationMode::Random(scale) => Box::new(random_quadruples(
            &EXAMPLE_SEED,
            &(|seed| random_vecs(seed, scale, &(|seed_2| random(seed_2)))),
            &(|seed| random_vecs_min_length(seed, scale, 2, &(|seed_2| random(seed_2)))),
            &(|seed| random_vecs_min_length(seed, scale, 1, &(|seed_2| random(seed_2)))),
            &(|seed| random_range_up(seed, Limb::power_of_two(Limb::WIDTH - 1))),
        )),
        GenerationMode::SpecialRandom(scale) => Box::new(random_quadruples(
            &EXAMPLE_SEED,
            &(|seed| special_random_unsigned_vecs(seed, scale)),
            &(|seed| special_random_unsigned_vecs_min_length(seed, scale, 2)),
            &(|seed| special_random_unsigned_vecs_min_length(seed, scale, 1)),
            &(|seed| random_range_up(seed, Limb::power_of_two(Limb::WIDTH - 1))),
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
pub fn triples_of_limb_vec_var_42(gm: GenerationMode) -> It<(Vec<Limb>, Vec<Limb>, Vec<Limb>)> {
    let qs: It<(Vec<Limb>, Vec<Limb>, Vec<Limb>, Limb)> = match gm {
        GenerationMode::Exhaustive => Box::new(exhaustive_quadruples(
            exhaustive_vecs(exhaustive_unsigneds()),
            exhaustive_vecs_min_length(3, exhaustive_unsigneds()),
            exhaustive_vecs_min_length(1, exhaustive_unsigneds()),
            primitive_int_increasing_inclusive_range(
                Limb::power_of_two(Limb::WIDTH - 1),
                Limb::MAX,
            ),
        )),
        GenerationMode::Random(scale) => Box::new(random_quadruples(
            &EXAMPLE_SEED,
            &(|seed| random_vecs(seed, scale, &(|seed_2| random(seed_2)))),
            &(|seed| random_vecs_min_length(seed, scale, 3, &(|seed_2| random(seed_2)))),
            &(|seed| random_vecs_min_length(seed, scale, 1, &(|seed_2| random(seed_2)))),
            &(|seed| random_range_up(seed, Limb::power_of_two(Limb::WIDTH - 1))),
        )),
        GenerationMode::SpecialRandom(scale) => Box::new(random_quadruples(
            &EXAMPLE_SEED,
            &(|seed| special_random_unsigned_vecs(seed, scale)),
            &(|seed| special_random_unsigned_vecs_min_length(seed, scale, 3)),
            &(|seed| special_random_unsigned_vecs_min_length(seed, scale, 1)),
            &(|seed| random_range_up(seed, Limb::power_of_two(Limb::WIDTH - 1))),
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
pub fn triples_of_limb_vec_var_43(gm: GenerationMode) -> It<(Vec<Limb>, Vec<Limb>, Vec<Limb>)> {
    let ts: It<(Vec<Limb>, Vec<Limb>, Vec<Limb>)> = match gm {
        GenerationMode::Exhaustive => Box::new(exhaustive_triples(
            exhaustive_vecs_min_length(1, exhaustive_unsigneds()),
            exhaustive_vecs_min_length(2, exhaustive_unsigneds()),
            exhaustive_vecs_min_length(2, exhaustive_unsigneds()),
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
        *d.last().unwrap() != 0 && n.len() >= d.len() && q.len() >= n.len() - d.len() + 1
    }))
}

// All triples of `Vec<Limb>`, where `qs`, `ns`, and `ds` meet the preconditions of
// `_limbs_div_to_out` and both the balanced and unbalanced div helper functions.
pub fn triples_of_limb_vec_var_44(gm: GenerationMode) -> It<(Vec<Limb>, Vec<Limb>, Vec<Limb>)> {
    Box::new(triples_of_limb_vec_var_43(gm).filter(|(_, n, d)| n.len() < (d.len() - 1) << 1))
}

// All triples of `Vec<Limb>`, where `rs`, `ns`, and `ds` meet the preconditions of
// `_limbs_mod_to_out`.
pub fn triples_of_limb_vec_var_45(gm: GenerationMode) -> It<(Vec<Limb>, Vec<Limb>, Vec<Limb>)> {
    let ts: It<(Vec<Limb>, Vec<Limb>, Vec<Limb>)> = match gm {
        GenerationMode::Exhaustive => Box::new(exhaustive_triples_from_single(
            exhaustive_vecs_min_length(2, exhaustive_unsigneds()),
        )),
        GenerationMode::Random(scale) => Box::new(random_triples_from_single(
            random_vecs_min_length(&EXAMPLE_SEED, scale, 2, &(|seed| random(seed))),
        )),
        GenerationMode::SpecialRandom(scale) => Box::new(random_triples_from_single(
            special_random_unsigned_vecs_min_length(&EXAMPLE_SEED, scale, 2),
        )),
    };
    Box::new(
        ts.filter(|(r, n, d)| *d.last().unwrap() != 0 && n.len() >= d.len() && r.len() >= d.len()),
    )
}

// All triples of `Vec<T>`, where `T` is unsigned, the first `Vec` is at least as long as the
// second, and the second and third are equally long and nonempty.
pub fn triples_of_unsigned_vec_var_46<T: PrimitiveUnsigned + Rand>(
    gm: GenerationMode,
) -> It<(Vec<T>, Vec<T>, Vec<T>)> {
    let ts: It<((Vec<T>, Vec<T>), Vec<T>)> = match gm {
        GenerationMode::Exhaustive => Box::new(sqrt_pairs(
            pairs_of_unsigned_vec_var_11(gm),
            exhaustive_vecs(exhaustive_unsigneds()),
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
pub fn triples_of_unsigned_vec_var_47<T: PrimitiveUnsigned + Rand>(
    gm: GenerationMode,
) -> It<(Vec<T>, Vec<T>, Vec<T>)> {
    let ts: It<((Vec<T>, Vec<T>), Vec<T>)> = match gm {
        GenerationMode::Exhaustive => Box::new(sqrt_pairs(
            pairs_of_unsigned_vec_var_11(gm),
            exhaustive_vecs(exhaustive_unsigneds()),
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
pub fn triples_of_unsigned_vec_var_49<T: PrimitiveUnsigned + Rand>(
    gm: GenerationMode,
) -> It<(Vec<T>, Vec<T>, Vec<T>)> {
    Box::new(triples_of_unsigned_vec_var_46(gm).filter(|(_, xs, _)| xs.len() > 1))
}

// All triples of `Vec<Limb>`, where `qs`, `ns`, and `ds` meet the preconditions of
// `_limbs_modular_div_barrett`.
pub fn triples_of_limb_vec_var_50(gm: GenerationMode) -> It<(Vec<Limb>, Vec<Limb>, Vec<Limb>)> {
    let ts: It<(Vec<Limb>, Vec<Limb>, Vec<Limb>)> = match gm {
        GenerationMode::Exhaustive => Box::new(exhaustive_triples_from_single(
            exhaustive_vecs_min_length(2, exhaustive_unsigneds()),
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
pub fn triples_of_limb_vec_var_51(gm: GenerationMode) -> It<(Vec<Limb>, Vec<Limb>, Vec<Limb>)> {
    let ts: It<(Vec<Limb>, Vec<Limb>, Vec<Limb>)> = match gm {
        GenerationMode::Exhaustive => Box::new(exhaustive_triples_from_single(
            exhaustive_vecs_min_length(1, exhaustive_unsigneds()),
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
pub fn triples_of_unsigned_vec_var_52<T: PrimitiveUnsigned + Rand>(
    gm: GenerationMode,
) -> It<(Vec<T>, Vec<T>, Vec<T>)> {
    Box::new(triples_of_unsigned_vec_var_1(gm).filter(|&(ref xs, _, _)| xs.len() >= 2))
}

// All triples of `Vec<Limb>`, where `qs`, `ns`, and `ds` meet the preconditions of
// `limbs_div_exact_to_out`.
pub fn triples_of_limb_vec_var_53(gm: GenerationMode) -> It<(Vec<Limb>, Vec<Limb>, Vec<Limb>)> {
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
pub fn triples_of_limb_vec_var_54(gm: GenerationMode) -> It<(Vec<Limb>, Vec<Limb>, Vec<Limb>)> {
    Box::new(triples_of_limb_vec_var_53(gm).filter(|&(_, _, ref ds)| ds.len() > 1))
}

// All triples of `Vec<T>` that meet the preconditions for `limbs_eq_mod`.
pub fn triples_of_unsigned_vec_var_55<T: PrimitiveUnsigned + Rand>(
    gm: GenerationMode,
) -> It<(Vec<T>, Vec<T>, Vec<T>)> {
    match gm {
        GenerationMode::Exhaustive => Box::new(exhaustive_triples_from_single(
            exhaustive_vecs_min_length(2, exhaustive_unsigneds())
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

// All triples of `Vec<Limb>` that meet the preconditions for `limbs_eq_mod`, where the `Natural`
// represented by the first `Vec` is equal to the `Natural` represented by the second `Vec` mod the
// `Natural` represented by the third `Vec`.
pub fn triples_of_limb_vec_var_56(gm: GenerationMode) -> It<(Vec<Limb>, Vec<Limb>, Vec<Limb>)> {
    Box::new(triples_of_unsigned_vec_var_55(gm).map(|(xs, ys, m)| {
        let mut product_limbs = if xs.is_empty() {
            Vec::new()
        } else {
            limbs_mul(&xs, &m)
        };
        if product_limbs.last() == Some(&0) {
            product_limbs.pop();
        }
        limbs_vec_add_in_place_left(&mut product_limbs, &ys);
        (product_limbs, ys, m)
    }))
}

// All triples of `Vec<Limb>` that meet the preconditions for `limbs_eq_mod`, where the `Natural`
// represented by the first `Vec` is not equal to the `Natural` represented by the second `Vec` mod
// the `Natural` represented by the third `Vec`.
pub fn triples_of_limb_vec_var_57(gm: GenerationMode) -> It<(Vec<Limb>, Vec<Limb>, Vec<Limb>)> {
    Box::new(
        triples_of_unsigned_vec_var_55::<Limb>(gm)
            .filter(|(xs, ys, m)| !limbs_eq_mod_ref_ref_ref(&*xs, &*ys, &*m)),
    )
}

// All triples of `Vec<Limb>` that meet the preconditions for `limbs_eq_mod`, where the `Natural`
// represented by the first `Vec` is equal to the negative of `Natural` represented by the second
// `Vec` mod the `Natural` represented by the third `Vec`.
pub fn triples_of_limb_vec_var_58(gm: GenerationMode) -> It<(Vec<Limb>, Vec<Limb>, Vec<Limb>)> {
    Box::new(
        triples_of_unsigned_vec_var_55(gm).filter_map(|(xs, ys, m)| {
            let mut product_limbs = if xs.is_empty() {
                Vec::new()
            } else {
                limbs_mul(&xs, &m)
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
                Some((product_limbs, ys, m))
            }
        }),
    )
}

fn pairs_of_unsigned_vec_min_sizes<T: PrimitiveUnsigned + Rand>(
    gm: GenerationMode,
    min_xs_len: u64,
    min_ys_len: u64,
) -> It<(Vec<T>, Vec<T>)> {
    match gm {
        GenerationMode::Exhaustive => Box::new(exhaustive_pairs(
            exhaustive_vecs_min_length(min_xs_len, exhaustive_unsigneds()),
            exhaustive_vecs_min_length(min_ys_len, exhaustive_unsigneds()),
        )),
        GenerationMode::Random(scale) => Box::new(random_pairs(
            &EXAMPLE_SEED,
            &(|seed| random_vecs_min_length(seed, scale, min_xs_len, &(|seed_2| random(seed_2)))),
            &(|seed| random_vecs_min_length(seed, scale, min_ys_len, &(|seed_2| random(seed_2)))),
        )),
        GenerationMode::SpecialRandom(scale) => Box::new(random_pairs(
            &EXAMPLE_SEED,
            &(|seed| special_random_unsigned_vecs_min_length(seed, scale, min_xs_len)),
            &(|seed| special_random_unsigned_vecs_min_length(seed, scale, min_ys_len)),
        )),
    }
}

// All triples of `Vec<Limb>` that meet the preconditions for `limbs_eq_mod`, where the `Natural`
// represented by the first `Vec` is not equal to the negative of `Natural` represented by the
// second `Vec` mod the `Natural` represented by the third `Vec`.
pub fn triples_of_limb_vec_var_59(gm: GenerationMode) -> It<(Vec<Limb>, Vec<Limb>, Vec<Limb>)> {
    Box::new(
        triples_of_unsigned_vec_var_55::<Limb>(gm).filter(|(xs, ys, m)| {
            !Integer::from(Natural::from_limbs_asc(xs))
                .eq_mod(-Natural::from_limbs_asc(ys), Natural::from_limbs_asc(m))
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
                exhaustive_unsigneds(),
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
            exhaustive_vecs_min_length(min_len, exhaustive_unsigneds()),
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
            exhaustive_vecs_min_length(min_xs_len, exhaustive_unsigneds()),
            exhaustive_vecs_min_length(min_ys_len, exhaustive_unsigneds()),
            exhaustive_vecs_min_length(min_zs_len, exhaustive_unsigneds()),
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
            exhaustive_vecs_min_length(min_xs_len, exhaustive_unsigneds()),
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
            exhaustive_vecs_min_length(min_len, exhaustive_unsigneds()),
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
            exhaustive_vecs(exhaustive_unsigneds()),
            exhaustive_vecs(exhaustive_unsigneds()),
            exhaustive_vecs(exhaustive_unsigneds()),
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
    for i in Limb::ZERO..(1 << PRIME_FACTORS_OF_LIMB_MAX.len()) {
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
pub fn quadruples_of_limb_vec_var_1(
    gm: GenerationMode,
) -> It<(Vec<Limb>, Vec<Limb>, Vec<Limb>, Vec<Limb>)> {
    let qs: It<(Vec<Limb>, Vec<Limb>, Vec<Limb>, Vec<Limb>, Limb)> = match gm {
        GenerationMode::Exhaustive => Box::new(exhaustive_quintuples(
            exhaustive_vecs_min_length(1, exhaustive_unsigneds()),
            exhaustive_vecs_min_length(2, exhaustive_unsigneds()),
            exhaustive_vecs_min_length(3, exhaustive_unsigneds()),
            exhaustive_vecs_min_length(1, exhaustive_unsigneds()),
            primitive_int_increasing_inclusive_range(
                Limb::power_of_two(Limb::WIDTH - 1),
                Limb::MAX,
            ),
        )),
        GenerationMode::Random(scale) => Box::new(random_quintuples(
            &EXAMPLE_SEED,
            &(|seed| random_vecs_min_length(seed, scale, 1, &(|seed_2| random(seed_2)))),
            &(|seed| random_vecs_min_length(seed, scale, 2, &(|seed_2| random(seed_2)))),
            &(|seed| random_vecs_min_length(seed, scale, 3, &(|seed_2| random(seed_2)))),
            &(|seed| random_vecs_min_length(seed, scale, 1, &(|seed_2| random(seed_2)))),
            &(|seed| random_range_up(seed, Limb::power_of_two(Limb::WIDTH - 1))),
        )),
        GenerationMode::SpecialRandom(scale) => Box::new(random_quintuples(
            &EXAMPLE_SEED,
            &(|seed| special_random_unsigned_vecs_min_length(seed, scale, 1)),
            &(|seed| special_random_unsigned_vecs_min_length(seed, scale, 2)),
            &(|seed| special_random_unsigned_vecs_min_length(seed, scale, 3)),
            &(|seed| special_random_unsigned_vecs_min_length(seed, scale, 1)),
            &(|seed| random_range_up(seed, Limb::power_of_two(Limb::WIDTH - 1))),
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
pub fn quadruples_of_limb_vec_var_2(
    gm: GenerationMode,
) -> It<(Vec<Limb>, Vec<Limb>, Vec<Limb>, Vec<Limb>)> {
    let qs: It<(Vec<Limb>, Vec<Limb>, Vec<Limb>, Vec<Limb>)> = match gm {
        GenerationMode::Exhaustive => Box::new(exhaustive_quadruples(
            exhaustive_vecs_min_length(1, exhaustive_unsigneds()),
            exhaustive_vecs_min_length(2, exhaustive_unsigneds()),
            exhaustive_vecs_min_length(2, exhaustive_unsigneds()),
            exhaustive_vecs_min_length(2, exhaustive_unsigneds()),
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
        *d.last().unwrap() != 0
            && r.len() >= d.len()
            && n.len() >= d.len()
            && q.len() >= n.len() - d.len() + 1
    }))
}

// All quadruples of `Vec<Limb>`, where `qs`, `rs`, `ns`, and `ds` meet certain preconditions that
// enable comparing the performance of two kinds of Barrett division.
pub fn quadruples_of_limb_vec_var_3(
    gm: GenerationMode,
) -> It<(Vec<Limb>, Vec<Limb>, Vec<Limb>, Vec<Limb>)> {
    Box::new(quadruples_of_limb_vec_var_1(gm).filter(|(_, _, n, d)| 2 * d.len() > n.len() + 1))
}

// All quadruples of `Vec<Limb>`, where `qs`, `rs`, `ns`, and `ds` meet the preconditions of
// `_limbs_modular_div_mod_barrett`.
pub fn quadruples_of_limb_vec_var_4(
    gm: GenerationMode,
) -> It<(Vec<Limb>, Vec<Limb>, Vec<Limb>, Vec<Limb>)> {
    let qs: It<(Vec<Limb>, Vec<Limb>, Vec<Limb>, Vec<Limb>)> = match gm {
        GenerationMode::Exhaustive => Box::new(exhaustive_quadruples(
            exhaustive_vecs_min_length(2, exhaustive_unsigneds()),
            exhaustive_vecs_min_length(2, exhaustive_unsigneds()),
            exhaustive_vecs_min_length(4, exhaustive_unsigneds()),
            exhaustive_vecs_min_length(2, exhaustive_unsigneds()),
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
pub fn quadruples_of_limb_vec_var_5(
    gm: GenerationMode,
) -> It<(Vec<Limb>, Vec<Limb>, Vec<Limb>, Vec<Limb>)> {
    let qs: It<(Vec<Limb>, Vec<Limb>, Vec<Limb>, Vec<Limb>)> = match gm {
        GenerationMode::Exhaustive => Box::new(exhaustive_quadruples(
            exhaustive_vecs_min_length(4, exhaustive_unsigneds()),
            exhaustive_vecs_min_length(2, exhaustive_unsigneds()),
            exhaustive_vecs_min_length(4, exhaustive_unsigneds()),
            exhaustive_vecs_min_length(2, exhaustive_unsigneds()),
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

pub fn sextuples_of_four_limb_vecs_and_two_usizes_var_1(
    gm: GenerationMode,
) -> It<(Vec<Limb>, Vec<Limb>, Vec<Limb>, Vec<Limb>, usize, usize)> {
    let qs: It<(Vec<Limb>, Vec<Limb>, Vec<Limb>, Vec<Limb>, u32)> = match gm {
        GenerationMode::Exhaustive => Box::new(exhaustive_quintuples(
            exhaustive_vecs_min_length(2, exhaustive_unsigneds()),
            exhaustive_vecs(exhaustive_unsigneds()),
            exhaustive_vecs(exhaustive_unsigneds()),
            exhaustive_vecs(exhaustive_unsigneds()),
            primitive_int_increasing_inclusive_range(3, u32::MAX),
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

pub fn quadruples_of_unsigneds<T: PrimitiveUnsigned + Rand>(
    gm: GenerationMode,
) -> It<(T, T, T, T)> {
    match gm {
        GenerationMode::Exhaustive => {
            Box::new(exhaustive_quadruples_from_single(exhaustive_unsigneds()))
        }
        GenerationMode::Random(_) => Box::new(random_quadruples_from_single(random(&EXAMPLE_SEED))),
        GenerationMode::SpecialRandom(_) => Box::new(random_quadruples_from_single(
            special_random_unsigned(&EXAMPLE_SEED),
        )),
    }
}

// All quadruples of unsigned `T` where the first three `T`s are smaller than the fourth.
pub fn quadruples_of_unsigneds_var_1<T: PrimitiveUnsigned + Rand>(
    gm: GenerationMode,
) -> It<(T, T, T, T)> {
    Box::new(quadruples_of_unsigneds(gm).filter(|&(x, y, z, m)| x < m && y < m && z < m))
}

// All quadruples of unsigned `T` that are valid inputs to _limbs_mod_preinverted.
pub fn quadruples_of_unsigneds_var_2<
    T: PrimitiveUnsigned + Rand,
    DT: JoinHalves + PrimitiveUnsigned + SplitInHalf,
>(
    gm: GenerationMode,
) -> It<(T, T, T, T)>
where
    DT: From<T> + HasHalf<Half = T>,
    T: CheckedFrom<DT>,
{
    Box::new(
        triples_of_unsigneds(gm)
            .filter(|&(_, _, d)| d != T::ZERO)
            .map(|(x_1, x_0, d)| {
                let inv = limbs_invert_limb_naive::<T, DT>(d << LeadingZeros::leading_zeros(d));
                (x_1, x_0, d, inv)
            }),
    )
}

// All triples of `T`, `T`, `u64`, and `u64`, where the `T`s are unsigned and the second `u64` is
// between n and `T::WIDTH`, inclusive, where n is the maximum number of significant bits of the two
// `T`s.
pub fn quadruples_of_unsigneds_var_5<T: PrimitiveUnsigned + Rand + SampleRange>(
    gm: NoSpecialGenerationMode,
) -> It<(T, T, u64, u64)> {
    match gm {
        NoSpecialGenerationMode::Exhaustive => reshape_3_1_to_4(Box::new(dependent_pairs(
            exhaustive_triples(
                exhaustive_unsigneds(),
                exhaustive_unsigneds(),
                exhaustive_unsigneds(),
            ),
            |&(x, y, _): &(T, T, u64)| {
                Box::new(primitive_int_increasing_inclusive_range(
                    max(x.significant_bits(), y.significant_bits()),
                    T::WIDTH,
                ))
            },
        ))),
        NoSpecialGenerationMode::Random(_) => {
            reshape_3_1_to_4(permute_2_1(Box::new(random_dependent_pairs(
                (),
                random_range(&scramble(&EXAMPLE_SEED, "pow"), 0, T::WIDTH),
                |_, &pow| {
                    random_triples(
                        &scramble(&EXAMPLE_SEED, "ts"),
                        &(|seed| {
                            random_range::<T>(&scramble(&seed, "x"), T::ZERO, T::low_mask(pow))
                        }),
                        &(|seed| {
                            random_range::<T>(&scramble(&seed, "y"), T::ZERO, T::low_mask(pow))
                        }),
                        &(|seed| random(&scramble(&seed, "exp"))),
                    )
                },
            ))))
        }
    }
}

// All triples of `T`, `u64`, `u64`, and `u64`, where the `T`s is unsigned and the third `u64` is
// between n and `T::WIDTH`, inclusive, where n is the number of significant bits of the `T`.
pub fn quadruples_of_unsigneds_var_6<T: PrimitiveUnsigned + Rand + SampleRange>(
    gm: NoSpecialGenerationMode,
) -> It<(T, u64, u64, u64)> {
    match gm {
        NoSpecialGenerationMode::Exhaustive => reshape_3_1_to_4(Box::new(dependent_pairs(
            exhaustive_triples(
                exhaustive_unsigneds(),
                exhaustive_unsigneds(),
                exhaustive_unsigneds(),
            ),
            |&(x, _, _): &(T, u64, u64)| {
                Box::new(primitive_int_increasing_inclusive_range(
                    x.significant_bits(),
                    T::WIDTH,
                ))
            },
        ))),
        NoSpecialGenerationMode::Random(_) => {
            reshape_3_1_to_4(permute_2_1(Box::new(random_dependent_pairs(
                (),
                random_range(&scramble(&EXAMPLE_SEED, "pow"), 0, T::WIDTH),
                |_, &pow| {
                    random_triples(
                        &scramble(&EXAMPLE_SEED, "ts"),
                        &(|seed| {
                            random_range::<T>(&scramble(&seed, "u"), T::ZERO, T::low_mask(pow))
                        }),
                        &(|seed| random(&scramble(&seed, "e"))),
                        &(|seed| random(&scramble(&seed, "f"))),
                    )
                },
            ))))
        }
    }
}

fn quadruples_of_unsigned_small_unsigned_small_unsigned_and_unsigned<
    T: PrimitiveUnsigned + Rand,
    U: PrimitiveUnsigned + Rand,
    V: PrimitiveUnsigned + Rand,
>(
    gm: GenerationMode,
) -> It<(T, U, U, V)> {
    let ps: It<((T, V), (U, U))> = match gm {
        GenerationMode::Exhaustive => Box::new(log_pairs(
            exhaustive_pairs(exhaustive_unsigneds(), exhaustive_unsigneds()),
            exhaustive_pairs_from_single(exhaustive_unsigneds()),
        )),
        GenerationMode::Random(scale) => Box::new(random_pairs(
            &EXAMPLE_SEED,
            &(|seed| random_pairs(seed, &(|seed_2| random(seed_2)), &(|seed_2| random(seed_2)))),
            &(|seed| random_pairs_from_single(u32s_geometric(seed, scale).map(U::wrapping_from))),
        )),
        GenerationMode::SpecialRandom(scale) => Box::new(random_pairs(
            &EXAMPLE_SEED,
            &(|seed| {
                random_pairs(
                    seed,
                    &(|seed_2| special_random_unsigned(seed_2)),
                    &(|seed_2| special_random_unsigned(seed_2)),
                )
            }),
            &(|seed| random_pairs_from_single(u32s_geometric(seed, scale).map(U::wrapping_from))),
        )),
    };
    permute_1_3_4_2(reshape_2_2_to_4(ps))
}

// All quadruples of `T`, `T`, `U`, and `T`, where `T` and `U` are unsigned and the first and second
// `T`s are smaller than the third.
pub fn quadruples_of_unsigneds_var_3<T: PrimitiveUnsigned + Rand, U: PrimitiveUnsigned + Rand>(
    gm: GenerationMode,
) -> It<(T, T, U, T)> {
    Box::new(quadruples_of_four_unsigneds(gm).filter(|&(x, y, _, m)| x < m && y < m))
}

// All quadruples of `T`, `U`, `U`, and `T`, where `T` and `U` are unsigned and the first `T`s is
// smaller than the second.
pub fn quadruples_of_unsigneds_var_4<T: PrimitiveUnsigned + Rand, U: PrimitiveUnsigned + Rand>(
    gm: GenerationMode,
) -> It<(T, U, U, T)> {
    Box::new(quadruples_of_four_unsigneds(gm).filter(|&(x, _, _, m)| x < m))
}

fn unsigned_assign_bits_valid<T: PrimitiveUnsigned>(start: u64, end: u64, bits: T) -> bool {
    start <= end && {
        let bits_width = end - start;
        let bits = bits.mod_power_of_two(bits_width);
        bits == T::ZERO || LeadingZeros::leading_zeros(bits) >= start
    }
}

// All quadruples of T, small u64, small u64, and T, where T is unsigned and the four values are
// valid arguments to `assign_bits`.
pub fn quadruples_of_unsigned_small_u64_small_u64_and_unsigned_var_1<
    T: PrimitiveUnsigned + Rand,
    U: PrimitiveUnsigned + Rand,
>(
    gm: GenerationMode,
) -> It<(T, u64, u64, U)> {
    Box::new(
        quadruples_of_unsigned_small_unsigned_small_unsigned_and_unsigned::<T, u64, U>(gm)
            .filter(|&(_, start, end, bits)| unsigned_assign_bits_valid(start, end, bits)),
    )
}

fn quadruples_of_signed_small_unsigned_small_unsigned_and_unsigned<
    T: PrimitiveSigned + Rand,
    U: PrimitiveUnsigned + Rand,
    V: PrimitiveUnsigned + Rand,
>(
    gm: GenerationMode,
) -> It<(T, U, U, V)>
where
    T::UnsignedOfEqualWidth: Rand,
    T: WrappingFrom<<T as PrimitiveSigned>::UnsignedOfEqualWidth>,
{
    let ps: It<((T, V), (U, U))> = match gm {
        GenerationMode::Exhaustive => Box::new(sqrt_pairs(
            exhaustive_pairs(exhaustive_signeds(), exhaustive_unsigneds()),
            exhaustive_pairs_from_single(exhaustive_unsigneds()),
        )),
        GenerationMode::Random(scale) => Box::new(random_pairs(
            &EXAMPLE_SEED,
            &(|seed| random_pairs(seed, &(|seed_2| random(seed_2)), &(|seed_2| random(seed_2)))),
            &(|seed| random_pairs_from_single(u32s_geometric(seed, scale).map(U::wrapping_from))),
        )),
        GenerationMode::SpecialRandom(scale) => Box::new(random_pairs(
            &EXAMPLE_SEED,
            &(|seed| {
                random_pairs(
                    seed,
                    &(|seed_2| special_random_signed(seed_2)),
                    &(|seed_2| special_random_unsigned(seed_2)),
                )
            }),
            &(|seed| random_pairs_from_single(u32s_geometric(seed, scale).map(U::wrapping_from))),
        )),
    };
    permute_1_3_4_2(reshape_2_2_to_4(ps))
}

// All quadruples of T, small u64, small u64, and T, where T is signed and the four values are valid
// arguments to `assign_bits`.
pub fn quadruples_of_signed_small_u64_small_u64_and_unsigned_var_1<
    T: PrimitiveSigned + Rand,
    U: PrimitiveUnsigned + Rand,
>(
    gm: GenerationMode,
) -> It<(T, u64, u64, U)>
where
    T::UnsignedOfEqualWidth: Rand,
    T: WrappingFrom<<T as PrimitiveSigned>::UnsignedOfEqualWidth>,
    <T as UnsignedAbs>::Output: BitBlockAccess<Bits = U> + PrimitiveUnsigned,
{
    Box::new(
        quadruples_of_signed_small_unsigned_small_unsigned_and_unsigned::<T, u64, U>(gm).filter(
            |&(x, start, end, bits)| {
                if x >= T::ZERO {
                    unsigned_assign_bits_valid(start, end, bits) && {
                        let mut abs_self = x.unsigned_abs();
                        abs_self.assign_bits(start, end, &bits);
                        !abs_self.get_highest_bit()
                    }
                } else {
                    start <= end && {
                        let width = T::WIDTH;
                        let bits_width = end - start;
                        let bits = bits.mod_power_of_two(bits_width);
                        bits_width <= width
                            && if start >= width - 1 {
                                bits == U::MAX.mod_power_of_two(bits_width)
                            } else {
                                end < width
                                    || bits >> (width - 1 - start)
                                        == U::MAX.mod_power_of_two(end - width + 1)
                            }
                    }
                }
            },
        ),
    )
}

fn quadruples_of_unsigned_vec_unsigned_vec_unsigned_and_unsigned<T: PrimitiveUnsigned + Rand>(
    gm: GenerationMode,
) -> It<(Vec<T>, Vec<T>, T, T)> {
    reshape_2_2_to_4(match gm {
        GenerationMode::Exhaustive => Box::new(exhaustive_pairs(
            exhaustive_pairs_from_single(exhaustive_vecs(exhaustive_unsigneds())),
            exhaustive_pairs_from_single(exhaustive_unsigneds()),
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

// All quadruples of `Vec<T>`, `Vec<T>`, `T`, and `T`, where `T` is unsigned and the first `Vec` is
// at least as long as the second.
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
                exhaustive_vecs(exhaustive_unsigneds()),
                exhaustive_vecs(exhaustive_unsigneds()),
                exhaustive_unsigneds(),
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
pub fn quadruples_of_three_limb_vecs_and_limb_var_1(
    gm: GenerationMode,
) -> It<(Vec<Limb>, Vec<Limb>, Vec<Limb>, Limb)> {
    let qs: It<(Vec<Limb>, Vec<Limb>, Vec<Limb>, Limb)> = match gm {
        GenerationMode::Exhaustive => Box::new(exhaustive_quadruples(
            exhaustive_vecs(exhaustive_unsigneds()),
            exhaustive_vecs_min_length(3, exhaustive_unsigneds()),
            exhaustive_vecs_min_length(2, exhaustive_unsigneds()),
            primitive_int_increasing_inclusive_range(
                Limb::power_of_two(Limb::WIDTH - 1),
                Limb::MAX,
            ),
        )),
        GenerationMode::Random(scale) => Box::new(random_quadruples(
            &EXAMPLE_SEED,
            &(|seed| random_vecs(seed, scale, &(|seed_2| random(seed_2)))),
            &(|seed| random_vecs_min_length(seed, scale, 3, &(|seed_2| random(seed_2)))),
            &(|seed| random_vecs_min_length(seed, scale, 2, &(|seed_2| random(seed_2)))),
            &(|seed| random_range_up(seed, Limb::power_of_two(Limb::WIDTH - 1))),
        )),
        GenerationMode::SpecialRandom(scale) => Box::new(random_quadruples(
            &EXAMPLE_SEED,
            &(|seed| special_random_unsigned_vecs(seed, scale)),
            &(|seed| special_random_unsigned_vecs_min_length(seed, scale, 3)),
            &(|seed| special_random_unsigned_vecs_min_length(seed, scale, 2)),
            &(|seed| random_range_up(seed, Limb::power_of_two(Limb::WIDTH - 1))),
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
pub fn quadruples_of_three_limb_vecs_and_limb_var_2(
    gm: GenerationMode,
) -> It<(Vec<Limb>, Vec<Limb>, Vec<Limb>, Limb)> {
    let qs: It<(Vec<Limb>, Vec<Limb>, Vec<Limb>, Limb)> = match gm {
        GenerationMode::Exhaustive => Box::new(exhaustive_quadruples(
            exhaustive_vecs_min_length(3, exhaustive_unsigneds()),
            exhaustive_vecs_min_length(9, exhaustive_unsigneds()),
            exhaustive_vecs_min_length(5, exhaustive_unsigneds()),
            primitive_int_increasing_inclusive_range(
                Limb::power_of_two(Limb::WIDTH - 1),
                Limb::MAX,
            ),
        )),
        GenerationMode::Random(scale) => Box::new(random_quadruples(
            &EXAMPLE_SEED,
            &(|seed| random_vecs_min_length(seed, scale, 3, &(|seed_2| random(seed_2)))),
            &(|seed| random_vecs_min_length(seed, scale, 9, &(|seed_2| random(seed_2)))),
            &(|seed| random_vecs_min_length(seed, scale, 5, &(|seed_2| random(seed_2)))),
            &(|seed| random_range_up(seed, Limb::power_of_two(Limb::WIDTH - 1))),
        )),
        GenerationMode::SpecialRandom(scale) => Box::new(random_quadruples(
            &EXAMPLE_SEED,
            &(|seed| special_random_unsigned_vecs_min_length(seed, scale, 3)),
            &(|seed| special_random_unsigned_vecs_min_length(seed, scale, 9)),
            &(|seed| special_random_unsigned_vecs_min_length(seed, scale, 5)),
            &(|seed| random_range_up(seed, Limb::power_of_two(Limb::WIDTH - 1))),
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
pub fn quadruples_of_three_limb_vecs_and_limb_var_3(
    gm: GenerationMode,
) -> It<(Vec<Limb>, Vec<Limb>, Vec<Limb>, Limb)> {
    let ts: It<(Vec<Limb>, Vec<Limb>, Vec<Limb>)> = match gm {
        GenerationMode::Exhaustive => Box::new(exhaustive_triples_from_single(
            exhaustive_vecs_min_length(1, exhaustive_unsigneds()),
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
pub fn quadruples_of_three_limb_vecs_and_limb_var_4(
    gm: GenerationMode,
) -> It<(Vec<Limb>, Vec<Limb>, Vec<Limb>, Limb)> {
    let ts: It<(Vec<Limb>, Vec<Limb>, Vec<Limb>)> = match gm {
        GenerationMode::Exhaustive => Box::new(exhaustive_triples(
            exhaustive_vecs_min_length(2, exhaustive_unsigneds()),
            exhaustive_vecs_min_length(2, exhaustive_unsigneds()),
            exhaustive_vecs_min_length(1, exhaustive_unsigneds()),
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
pub fn quadruples_of_three_limb_vecs_and_limb_var_5(
    gm: GenerationMode,
) -> It<(Vec<Limb>, Vec<Limb>, Vec<Limb>, Limb)> {
    let ts: It<(Vec<Limb>, Vec<Limb>, Vec<Limb>)> = match gm {
        GenerationMode::Exhaustive => Box::new(exhaustive_triples(
            exhaustive_vecs_min_length(3, exhaustive_unsigneds()),
            exhaustive_vecs_min_length(3, exhaustive_unsigneds()),
            exhaustive_vecs_min_length(2, exhaustive_unsigneds()),
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
pub fn quadruples_of_three_limb_vecs_and_limb_var_6(
    gm: GenerationMode,
) -> It<(Vec<Limb>, Vec<Limb>, Vec<Limb>, Limb)> {
    let ts: It<(Vec<Limb>, Vec<Limb>, Vec<Limb>)> = match gm {
        GenerationMode::Exhaustive => Box::new(exhaustive_triples_from_single(
            exhaustive_vecs_min_length(2, exhaustive_unsigneds()),
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
pub fn quadruples_of_three_limb_vecs_and_limb_var_7(
    gm: GenerationMode,
) -> It<(Vec<Limb>, Vec<Limb>, Vec<Limb>, Limb)> {
    let vs: It<Vec<Limb>> =
        match gm {
            GenerationMode::Exhaustive => {
                Box::new(exhaustive_vecs_min_length(1, exhaustive_unsigneds()))
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
                exhaustive_vecs(exhaustive_unsigneds()),
                exhaustive_unsigneds(),
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

fn exhaustive_pairs_of_unsigned_vec_and_unsigned<T: PrimitiveUnsigned + Rand>() -> It<(Vec<T>, T)> {
    Box::new(exhaustive_pairs(
        exhaustive_vecs(exhaustive_unsigneds()),
        exhaustive_unsigneds(),
    ))
}

pub fn triples_of_unsigned_vec_small_usize_and_unsigned<T: PrimitiveUnsigned + Rand>(
    gm: GenerationMode,
) -> It<(Vec<T>, usize, T)> {
    match gm {
        GenerationMode::Exhaustive => permute_1_3_2(reshape_2_1_to_3(Box::new(log_pairs(
            exhaustive_pairs_of_unsigned_vec_and_unsigned(),
            exhaustive_unsigneds(),
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
pub fn pairs_of_unsigned_vec_and_unsigned_var_1<T: PrimitiveUnsigned + Rand>(
    gm: GenerationMode,
) -> It<(Vec<T>, T)> {
    match gm {
        GenerationMode::Exhaustive => Box::new(exhaustive_pairs(
            exhaustive_vecs_min_length(2, exhaustive_unsigneds()),
            primitive_int_increasing_inclusive_range(T::power_of_two(T::WIDTH - 1), T::MAX),
        )),
        GenerationMode::Random(scale) => Box::new(random_pairs(
            &EXAMPLE_SEED,
            &(|seed| random_vecs_min_length(seed, scale, 2, &(|seed_2| random(seed_2)))),
            &(|seed| {
                random::<T>(seed).map(|mut u| {
                    u.set_bit(T::WIDTH - 1);
                    u
                })
            }),
        )),
        GenerationMode::SpecialRandom(scale) => Box::new(random_pairs(
            &EXAMPLE_SEED,
            &(|seed| special_random_unsigned_vecs_min_length(seed, scale, 2)),
            &(|seed| {
                special_random_unsigned::<T>(seed).map(|mut u| {
                    u.set_bit(T::WIDTH - 1);
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
            exhaustive_vecs_min_length(1, exhaustive_unsigneds()),
            primitive_int_increasing_inclusive_range(T::power_of_two(T::WIDTH - 1), T::MAX),
        )),
        GenerationMode::Random(scale) => Box::new(random_pairs(
            &EXAMPLE_SEED,
            &(|seed| random_vecs_min_length(seed, scale, 1, &(|seed_2| random(seed_2)))),
            &(|seed| {
                random::<T>(seed).map(|mut u| {
                    u.set_bit(T::WIDTH - 1);
                    u
                })
            }),
        )),
        GenerationMode::SpecialRandom(scale) => Box::new(random_pairs(
            &EXAMPLE_SEED,
            &(|seed| special_random_unsigned_vecs_min_length(seed, scale, 1)),
            &(|seed| {
                special_random_unsigned::<T>(seed).map(|mut u| {
                    u.set_bit(T::WIDTH - 1);
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
            exhaustive_vecs_min_length(1, exhaustive_unsigneds()),
            primitive_int_increasing_inclusive_range(T::ONE, T::low_mask(T::WIDTH - 1)),
        )),
        GenerationMode::Random(scale) => Box::new(random_pairs(
            &EXAMPLE_SEED,
            &(|seed| random_vecs_min_length(seed, scale, 1, &(|seed_2| random(seed_2)))),
            &(|seed| random_range::<T>(seed, T::ONE, T::low_mask(T::WIDTH - 1))),
        )),
        GenerationMode::SpecialRandom(scale) => Box::new(random_pairs(
            &EXAMPLE_SEED,
            &(|seed| special_random_unsigned_vecs_min_length(seed, scale, 1)),
            &(|seed| {
                special_random_unsigned::<T>(seed)
                    .map(|mut u| {
                        u.clear_bit(T::WIDTH - 1);
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
            exhaustive_vecs_min_length(1, exhaustive_unsigneds()),
            primitive_int_increasing_inclusive_range(T::ONE, T::low_mask(T::WIDTH - 2)),
        )),
        GenerationMode::Random(scale) => Box::new(random_pairs(
            &EXAMPLE_SEED,
            &(|seed| random_vecs_min_length(seed, scale, 1, &(|seed_2| random(seed_2)))),
            &(|seed| random_range::<T>(seed, T::ONE, T::low_mask(T::WIDTH - 2))),
        )),
        GenerationMode::SpecialRandom(scale) => Box::new(random_pairs(
            &EXAMPLE_SEED,
            &(|seed| special_random_unsigned_vecs_min_length(seed, scale, 1)),
            &(|seed| {
                special_random_unsigned::<T>(seed)
                    .map(|mut u| {
                        u.clear_bit(T::WIDTH - 1);
                        u.clear_bit(T::WIDTH - 2);
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
            exhaustive_vecs(exhaustive_unsigneds()),
            exhaustive_positive_primitive_ints(),
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
            exhaustive_vecs_min_length(2, exhaustive_unsigneds()),
            exhaustive_positive_primitive_ints(),
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
pub fn pairs_of_unsigned_vec_and_positive_unsigned_var_3<
    T: PrimitiveUnsigned + Rand + SampleRange,
>(
    gm: GenerationMode,
) -> It<(Vec<T>, T)> {
    match gm {
        GenerationMode::Exhaustive => Box::new(exhaustive_pairs(
            exhaustive_vecs_min_length(2, exhaustive_unsigneds()),
            primitive_int_increasing_inclusive_range(T::ONE, T::low_mask(T::WIDTH - 1)),
        )),
        GenerationMode::Random(scale) => Box::new(random_pairs(
            &EXAMPLE_SEED,
            &(|seed| random_vecs_min_length(seed, scale, 2, &(|seed_2| random(seed_2)))),
            &(|seed| random_range::<T>(seed, T::ONE, T::low_mask(T::WIDTH - 1))),
        )),
        GenerationMode::SpecialRandom(scale) => Box::new(random_pairs(
            &EXAMPLE_SEED,
            &(|seed| special_random_unsigned_vecs_min_length(seed, scale, 2)),
            &(|seed| {
                special_random_unsigned::<T>(seed)
                    .map(|mut u| {
                        u.clear_bit(T::WIDTH - 1);
                        u
                    })
                    .filter(|&u| u != T::ZERO)
            }),
        )),
    }
}

// All pairs of `Vec<T>` where `T` is unsigned, and a `u64` between 1 and 31, inclusive.
pub fn pairs_of_unsigned_vec_and_u64_var_1<T: PrimitiveUnsigned + Rand>(
    gm: GenerationMode,
) -> It<(Vec<T>, u64)> {
    match gm {
        GenerationMode::Exhaustive => Box::new(exhaustive_pairs(
            exhaustive_vecs(exhaustive_unsigneds()),
            primitive_int_increasing_range(1, u32::WIDTH),
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

// All pairs of `Vec<T>` where `T` is unsigned, and a `u64` between 1 and 31, inclusive, where the
// `Vec` is nonempty.
pub fn pairs_of_unsigned_vec_and_u64_var_2<T: PrimitiveUnsigned + Rand>(
    gm: GenerationMode,
) -> It<(Vec<T>, u64)> {
    Box::new(pairs_of_unsigned_vec_and_u64_var_1(gm).filter(|&(ref xs, _)| !xs.is_empty()))
}

pub fn pairs_of_unsigned_vec_and_small_unsigned<
    T: PrimitiveUnsigned + Rand,
    U: PrimitiveUnsigned + Rand,
>(
    gm: GenerationMode,
) -> It<(Vec<T>, U)> {
    match gm {
        GenerationMode::Exhaustive => Box::new(log_pairs(
            exhaustive_vecs(exhaustive_unsigneds()),
            exhaustive_unsigneds(),
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
            .filter(|&(ref limbs, _)| !slice_test_zero(limbs)),
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
pub fn pairs_of_unsigned_vec_and_unsigned_var_2<T: PrimitiveUnsigned + Rand>(
    gm: GenerationMode,
) -> It<(Vec<T>, T)> {
    Box::new(
        pairs_of_unsigned_vec_and_unsigned(gm).filter(|&(ref limbs, _)| !slice_test_zero(limbs)),
    )
}

// All pairs of `Vec<T>` and positive `T`, where `T` is unsigned and the `Vec<T>` is nonempty and
// doesn't only contain zeros.
pub fn pairs_of_unsigned_vec_and_positive_unsigned_var_2<T: PrimitiveUnsigned + Rand>(
    gm: GenerationMode,
) -> It<(Vec<T>, T)> {
    Box::new(
        pairs_of_unsigned_vec_and_unsigned(gm)
            .filter(|&(ref limbs, limb)| limb != T::ZERO && !slice_test_zero(limbs)),
    )
}

// All pairs of `Vec<Limb>` and `u64`, where the `Vec` is nonempty, its last element is nonzero, and
// the `u64` is least 2.
pub fn pairs_of_unsigned_vec_and_small_unsigned_var_3(gm: GenerationMode) -> It<(Vec<Limb>, u64)> {
    let ps: It<(Vec<Limb>, u64)> = match gm {
        GenerationMode::Exhaustive => Box::new(exhaustive_pairs(
            exhaustive_vecs_min_length(1, exhaustive_unsigneds()),
            exhaustive_unsigneds(),
        )),
        GenerationMode::Random(scale) => Box::new(random_pairs(
            &EXAMPLE_SEED,
            &(|seed| random_vecs_min_length(seed, scale, 1, &(|seed_2| random(seed_2)))),
            &(|seed| u32s_geometric(seed, scale).flat_map(u64::checked_from)),
        )),
        GenerationMode::SpecialRandom(scale) => Box::new(random_pairs(
            &EXAMPLE_SEED,
            &(|seed| special_random_unsigned_vecs_min_length(seed, scale, 1)),
            &(|seed| u32s_geometric(seed, scale).flat_map(u64::checked_from)),
        )),
    };
    Box::new(ps.filter(|&(ref xs, exp)| exp > 1 && *xs.last().unwrap() != 0))
}

fn triples_of_unsigned_vec_small_unsigned_and_small_unsigned<
    T: PrimitiveUnsigned + Rand,
    U: PrimitiveUnsigned + Rand,
>(
    gm: GenerationMode,
) -> It<(Vec<T>, U, U)> {
    match gm {
        GenerationMode::Exhaustive => reshape_1_2_to_3(Box::new(log_pairs(
            exhaustive_vecs(exhaustive_unsigneds()),
            exhaustive_pairs_from_single(exhaustive_unsigneds()),
        ))),
        GenerationMode::Random(scale) => Box::new(random_triples(
            &EXAMPLE_SEED,
            &(|seed| random_vecs(seed, scale, &(|seed_2| random(seed_2)))),
            &(|seed| u32s_geometric(seed, scale).flat_map(U::checked_from)),
            &(|seed| u32s_geometric(seed, scale).flat_map(U::checked_from)),
        )),
        GenerationMode::SpecialRandom(scale) => Box::new(random_triples(
            &EXAMPLE_SEED,
            &(|seed| special_random_unsigned_vecs(seed, scale)),
            &(|seed| u32s_geometric(seed, scale).flat_map(U::checked_from)),
            &(|seed| u32s_geometric(seed, scale).flat_map(U::checked_from)),
        )),
    }
}

// All triples of `Vec<T>`, `U`, and `U`, where `T` and `U` are unsigned and the first `U` is less
// than or equal to the second.
pub fn triples_of_unsigned_vec_small_unsigned_and_small_unsigned_var_1<
    T: PrimitiveUnsigned + Rand,
    U: PrimitiveUnsigned + Rand,
>(
    gm: GenerationMode,
) -> It<(Vec<T>, U, U)> {
    Box::new(
        triples_of_unsigned_vec_small_unsigned_and_small_unsigned(gm)
            .filter(|&(_, start, end)| start <= end),
    )
}

// All triples of `Vec<Limb>`, `T`, and `T`, where `T` is unsigned, the `Vec` does not only contain
// zeros, and the first `T` is less than or equal to the second.
pub fn triples_of_limb_vec_small_unsigned_and_small_unsigned_var_2<T: PrimitiveUnsigned + Rand>(
    gm: GenerationMode,
) -> It<(Vec<Limb>, T, T)> {
    Box::new(
        triples_of_unsigned_vec_small_unsigned_and_small_unsigned(gm)
            .filter(|&(ref limbs, start, end)| !slice_test_zero(limbs) && start <= end),
    )
}

pub fn vecs_of_bool(gm: GenerationMode) -> It<Vec<bool>> {
    match gm {
        GenerationMode::Exhaustive => Box::new(exhaustive_vecs_shortlex(exhaustive_bools())),
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

// All `Vec<bool>` that could be the bits, in ascending order, of a value of type `T`, which is
// unsigned.
pub fn vecs_of_bool_var_2<T: PrimitiveUnsigned>(gm: GenerationMode) -> It<Vec<bool>> {
    Box::new(vecs_of_bool(gm).filter(|bits| {
        let width = usize::exact_from(T::WIDTH);
        bits.len() <= width || bits[width..].iter().all(|&bit| !bit)
    }))
}

// All `Vec<bool>` that could be the bits, in ascending order, of a value of type `T`, which is
// signed.
pub fn vecs_of_bool_var_3<T: PrimitiveSigned>(gm: GenerationMode) -> It<Vec<bool>> {
    Box::new(vecs_of_bool(gm).filter(|bits| {
        if bits.is_empty() {
            return true;
        }
        let width = usize::exact_from(T::WIDTH);
        if !*bits.last().unwrap() {
            bits.len() < width || bits[width - 1..].iter().all(|&bit| !bit)
        } else {
            let trailing_trues = bits.iter().rev().take_while(|&&bit| bit).count();
            let significant_bits = bits.len() - trailing_trues;
            significant_bits < width
        }
    }))
}

// All `Vec<bool>` that could be the bits, in descending order, of a value of type `T`, which is
// unsigned.
pub fn vecs_of_bool_var_4<T: PrimitiveUnsigned>(gm: GenerationMode) -> It<Vec<bool>> {
    Box::new(vecs_of_bool(gm).filter(|bits| {
        let width = usize::exact_from(T::WIDTH);
        bits.len() <= width || bits[..bits.len() - width].iter().all(|&bit| !bit)
    }))
}

// All `Vec<bool>` that could be the bits, in descending order, of a value of type `T`, which is
// signed.
pub fn vecs_of_bool_var_5<T: PrimitiveSigned>(gm: GenerationMode) -> It<Vec<bool>> {
    Box::new(vecs_of_bool(gm).filter(|bits| {
        if bits.is_empty() {
            return true;
        }
        let width = usize::exact_from(T::WIDTH);
        if !*bits.last().unwrap() {
            bits.len() < width || bits[..bits.len() - width + 1].iter().all(|&bit| !bit)
        } else {
            let trailing_trues = bits.iter().take_while(|&&bit| bit).count();
            let significant_bits = bits.len() - trailing_trues;
            significant_bits < width
        }
    }))
}

fn triples_of_unsigned_vec_unsigned_vec_and_unsigned<T: PrimitiveUnsigned + Rand>(
    gm: GenerationMode,
) -> It<(Vec<T>, Vec<T>, T)> {
    match gm {
        GenerationMode::Exhaustive => Box::new(exhaustive_triples(
            exhaustive_vecs(exhaustive_unsigneds()),
            exhaustive_vecs(exhaustive_unsigneds()),
            exhaustive_unsigneds(),
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
            exhaustive_vecs(exhaustive_unsigneds()),
            exhaustive_vecs(exhaustive_unsigneds()),
            exhaustive_positive_primitive_ints(),
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
            |&(ref out, ref in_limbs, _)| out.len() >= in_limbs.len() && !slice_test_zero(in_limbs),
        ),
    )
}

// All triples of `Vec<T>`, `Vec<T>`, and `u64` where `T` is unsigned and the `u64` is between 1 and
// 31, inclusive.
fn triples_of_unsigned_vec_unsigned_vec_and_u64_var_4<T: PrimitiveUnsigned + Rand>(
    gm: GenerationMode,
) -> It<(Vec<T>, Vec<T>, u64)> {
    match gm {
        GenerationMode::Exhaustive => Box::new(exhaustive_triples(
            exhaustive_vecs(exhaustive_unsigneds()),
            exhaustive_vecs(exhaustive_unsigneds()),
            primitive_int_increasing_range(1, u32::WIDTH),
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

// All triples of `Vec<T>`, `Vec<T>`, and `u64` where `T` is unsigned, the first `Vec` is at least
// as long as the second, and the `u64` is between 1 and 31, inclusive.
pub fn triples_of_unsigned_vec_unsigned_vec_and_u64_var_5<T: PrimitiveUnsigned + Rand>(
    gm: GenerationMode,
) -> It<(Vec<T>, Vec<T>, u64)> {
    Box::new(
        triples_of_unsigned_vec_unsigned_vec_and_u64_var_4(gm)
            .filter(|&(ref out, ref in_limbs, _)| out.len() >= in_limbs.len()),
    )
}

// All triples of `Vec<T>`, `Vec<T>`, and `u64` where `T` is unsigned, the first `Vec` is at least
// as long as the second, the second is nonempty, and the `u64` is between 1 and 31, inclusive.
pub fn triples_of_unsigned_vec_unsigned_vec_and_u64_var_6<T: PrimitiveUnsigned + Rand>(
    gm: GenerationMode,
) -> It<(Vec<T>, Vec<T>, u64)> {
    Box::new(
        triples_of_unsigned_vec_unsigned_vec_and_u64_var_4(gm).filter(
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
// `limbs_eq_mod_limb`.
pub fn triples_of_unsigned_vec_unsigned_vec_and_unsigned_var_8<T: PrimitiveUnsigned + Rand>(
    gm: GenerationMode,
) -> It<(Vec<T>, Vec<T>, T)> {
    let ps: It<((Vec<T>, Vec<T>), T)> = match gm {
        GenerationMode::Exhaustive => Box::new(sqrt_pairs(
            exhaustive_pairs_from_single(
                exhaustive_vecs_min_length(2, exhaustive_unsigneds())
                    .filter(|xs| *xs.last().unwrap() != T::ZERO),
            ),
            exhaustive_positive_primitive_ints(),
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

// All triples of `Vec<Limb>`, and `Vec<Limb>`, and `Limb` that meet the preconditions for
// `limbs_eq_mod_limb`, where the `Natural` represented by the first `Vec` is equal to the
// `Natural` represented by the second `Vec` mod the `Limb`.
pub fn triples_of_limb_vec_limb_vec_and_limb_var_9(
    gm: GenerationMode,
) -> It<(Vec<Limb>, Vec<Limb>, Limb)> {
    Box::new(
        triples_of_unsigned_vec_unsigned_vec_and_unsigned_var_8(gm).map(|(mut xs, ys, m)| {
            limbs_vec_mul_limb_in_place(&mut xs, m);
            if xs.last() == Some(&0) {
                xs.pop();
            }
            limbs_vec_add_in_place_left(&mut xs, &ys);
            (xs, ys, m)
        }),
    )
}

// All triples of `Vec<Limb>`, Vec<Limb>`, and `Limb` that meet the preconditions for
// `limbs_eq_mod_limb`, where the `Natural` represented by the first `Vec` is not equal to
// the `Natural` represented by the second `Vec` mod the `Limb`.
pub fn triples_of_limb_vec_limb_vec_and_limb_var_10(
    gm: GenerationMode,
) -> It<(Vec<Limb>, Vec<Limb>, Limb)> {
    Box::new(
        triples_of_unsigned_vec_unsigned_vec_and_unsigned_var_8::<Limb>(gm)
            .filter(|(xs, ys, m)| !limbs_eq_mod_limb_ref_ref(&*xs, &*ys, *m)),
    )
}

// All triples of `Vec<Limb>`, and `Vec<Limb>`, and `Limb` that meet the preconditions for
// `limbs_eq_mod_limb`, where the `Natural` represented by the first `Vec` is equal to the
// negative of the `Natural` represented by the second `Vec` mod the `Limb`.
pub fn triples_of_limb_vec_limb_vec_and_limb_var_11(
    gm: GenerationMode,
) -> It<(Vec<Limb>, Vec<Limb>, Limb)> {
    Box::new(
        triples_of_unsigned_vec_unsigned_vec_and_unsigned_var_8(gm).filter_map(|(xs, ys, m)| {
            let mut product_limbs = if xs.is_empty() {
                Vec::new()
            } else {
                limbs_mul_limb(&xs, m)
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
                Some((product_limbs, ys, m))
            }
        }),
    )
}

// All triples of `Vec<Limb>`, and `Vec<Limb>`, and `Limb` that meet the preconditions for
// `limbs_eq_mod_limb`, where the `Natural` represented by the first `Vec` is not equal to the
// negative of the `Natural` represented by the second `Vec` mod the `Limb`.
pub fn triples_of_limb_vec_limb_vec_and_limb_var_12(
    gm: GenerationMode,
) -> It<(Vec<Limb>, Vec<Limb>, Limb)> {
    Box::new(
        triples_of_unsigned_vec_unsigned_vec_and_unsigned_var_8::<Limb>(gm).filter(
            |&(ref xs, ref ys, m)| {
                !Integer::from(Natural::from_limbs_asc(xs))
                    .eq_mod(-Natural::from_limbs_asc(ys), Natural::from(m))
            },
        ),
    )
}

// All triples of `Vec<Limb>`, `Vec<Limb>`, and `u64` such that the number of significant bits of
// either `Vec` does not exceed the `u64`.
pub fn triples_of_limb_vec_limb_vec_and_u64_var_13(
    gm: GenerationMode,
) -> It<(Vec<Limb>, Vec<Limb>, u64)> {
    if gm == GenerationMode::Exhaustive {
        let ts = triples_of_unsigned_vec_unsigned_vec_and_small_unsigned::<Limb, u64>(gm).map(
            |(mut xs, mut ys, pow)| {
                limbs_slice_mod_power_of_two_in_place(&mut xs, pow);
                limbs_slice_mod_power_of_two_in_place(&mut ys, pow);
                (xs, ys, pow)
            },
        );
        Box::new(ts.unique())
    } else {
        let ts = triples_of_unsigned_vec_unsigned_vec_and_small_unsigned::<Limb, u64>(
            gm.with_scale(gm.get_scale().unwrap()),
        )
        .map(|(mut xs, mut ys, pow)| {
            limbs_slice_mod_power_of_two_in_place(&mut xs, pow);
            limbs_slice_mod_power_of_two_in_place(&mut ys, pow);
            (xs, ys, pow)
        });
        Box::new(ts)
    }
}

// All triples of `Vec<Limb>`, `Vec<Limb>`, and `u64` such that the first `Vec` is at least as long
// as the second and the number of significant bits of either `Vec` does not exceed the `u64`.
pub fn triples_of_limb_vec_limb_vec_and_u64_var_14(
    gm: GenerationMode,
) -> It<(Vec<Limb>, Vec<Limb>, u64)> {
    Box::new(
        triples_of_limb_vec_limb_vec_and_u64_var_13(gm).map(|(xs, ys, pow)| {
            if xs.len() >= ys.len() {
                (xs, ys, pow)
            } else {
                (ys, xs, pow)
            }
        }),
    )
}

// All triples of `Vec<Limb>`, `Vec<Limb>`, and `u64` such that the number of significant bits of
// either `Vec` does not exceed the `u64`, and the `Vec`s have no trailing zeros.
pub fn triples_of_limb_vec_limb_vec_and_u64_var_15(
    gm: GenerationMode,
) -> It<(Vec<Limb>, Vec<Limb>, u64)> {
    Box::new(
        triples_of_limb_vec_limb_vec_and_u64_var_13(gm).filter(|&(ref xs, ref ys, _)| {
            (xs.is_empty() || *xs.last().unwrap() != 0)
                && (ys.is_empty() || *ys.last().unwrap() != 0)
        }),
    )
}

// All triples of `Vec<Limb>`, `Vec<Limb>`, and `u64` such that the number of significant bits of
// either `Vec` does not exceed the `u64`, neither `Vec` is empty, and neither `Vec` has trailing
// zeros.
pub fn triples_of_limb_vec_limb_vec_and_u64_var_16(
    gm: GenerationMode,
) -> It<(Vec<Limb>, Vec<Limb>, u64)> {
    Box::new(
        triples_of_limb_vec_limb_vec_and_u64_var_13(gm).filter(|&(ref xs, ref ys, _)| {
            !xs.is_empty() && !ys.is_empty() && *xs.last().unwrap() != 0 && *ys.last().unwrap() != 0
        }),
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
            exhaustive_vecs(exhaustive_unsigneds()),
            exhaustive_vecs(exhaustive_unsigneds()),
            exhaustive_unsigneds(),
        )),
        GenerationMode::Random(scale) => Box::new(random_triples(
            &EXAMPLE_SEED,
            &(|seed| random_vecs(seed, scale, &(|seed_2| random(seed_2)))),
            &(|seed| random_vecs(seed, scale, &(|seed_2| random(seed_2)))),
            &(|seed| u32s_geometric(seed, scale << Limb::LOG_WIDTH).flat_map(U::checked_from)),
        )),
        GenerationMode::SpecialRandom(scale) => Box::new(random_triples(
            &EXAMPLE_SEED,
            &(|seed| special_random_unsigned_vecs(seed, scale)),
            &(|seed| special_random_unsigned_vecs(seed, scale)),
            &(|seed| u32s_geometric(seed, scale << Limb::LOG_WIDTH).flat_map(U::checked_from)),
        )),
    }
}

pub fn triples_of_unsigned_vec_unsigned_and_unsigned<T: PrimitiveUnsigned + Rand>(
    gm: GenerationMode,
) -> It<(Vec<T>, T, T)> {
    match gm {
        GenerationMode::Exhaustive => Box::new(exhaustive_triples(
            exhaustive_vecs(exhaustive_unsigneds()),
            exhaustive_unsigneds(),
            exhaustive_unsigneds(),
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

// All triples of `Vec<T>`, `T`, and `T` where `T` is unsigned, the `Vec` is nonempty, and the first
// `T` is odd.
pub fn triples_of_unsigned_vec_unsigned_and_unsigned_var_1<T: PrimitiveUnsigned + Rand>(
    gm: GenerationMode,
) -> It<(Vec<T>, T, T)> {
    Box::new(
        triples_of_unsigned_vec_unsigned_and_unsigned::<T>(gm)
            .filter(|&(ref ns, d, _)| !ns.is_empty() && d.odd()),
    )
}

// All triples of `Vec<Limb>`, `Limb`, and `u64` such that the number of significant bits of neither
// the `Vec` nor the `Limb` exceeds the `u64`.
pub fn triples_of_limb_vec_limb_and_u64_var_1(gm: GenerationMode) -> It<(Vec<Limb>, Limb, u64)> {
    let ts = triples_of_unsigned_vec_unsigned_and_small_unsigned::<Limb, u64>(gm).map(
        |(mut xs, y, pow)| {
            limbs_slice_mod_power_of_two_in_place(&mut xs, pow);
            (xs, y.mod_power_of_two(pow), pow)
        },
    );
    if gm == GenerationMode::Exhaustive {
        Box::new(ts.unique())
    } else {
        Box::new(ts)
    }
}

// All triples of nonempty `Vec<Limb>`, `Limb`, and `u64` such that the number of significant bits
// of neither the `Vec` nor the `Limb` exceeds the `u64`.
pub fn triples_of_limb_vec_limb_and_u64_var_2(gm: GenerationMode) -> It<(Vec<Limb>, Limb, u64)> {
    Box::new(triples_of_limb_vec_limb_and_u64_var_1(gm).filter(|&(ref xs, _, _)| !xs.is_empty()))
}

// All triples of `Limb`, `Vec<Limb>`, and positive `u64` such that the number of significant bits
// of neither the `Limb` nor the `Vec` exceeds the `u64`.
pub fn triples_of_limb_limb_vec_and_u64_var_1(gm: GenerationMode) -> It<(Limb, Vec<Limb>, u64)> {
    permute_2_1_3(Box::new(
        triples_of_limb_vec_limb_and_u64_var_1(gm).filter(|&(_, _, pow)| pow != 0),
    ))
}

pub fn triples_of_unsigned_unsigned_and_small_unsigned<
    T: PrimitiveUnsigned + Rand,
    U: PrimitiveUnsigned,
>(
    gm: GenerationMode,
) -> It<(T, T, U)> {
    match gm {
        GenerationMode::Exhaustive => Box::new(reshape_2_1_to_3(Box::new(sqrt_pairs(
            exhaustive_pairs_from_single(exhaustive_unsigneds()),
            exhaustive_unsigneds(),
        )))),
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

// All triples of `T`, `T` and `u64`, where `T` is unsigned and the `u64` is between n and
// `T::WIDTH`, inclusive, where n is the maximum number of significant bits of the two `T`s.
pub fn triples_of_unsigned_unsigned_and_small_u64_var_1<
    T: PrimitiveUnsigned + Rand + SampleRange,
>(
    gm: NoSpecialGenerationMode,
) -> It<(T, T, u64)> {
    match gm {
        NoSpecialGenerationMode::Exhaustive => reshape_2_1_to_3(Box::new(dependent_pairs(
            exhaustive_pairs_from_single(exhaustive_unsigneds()),
            |&(x, y): &(T, T)| {
                Box::new(primitive_int_increasing_inclusive_range(
                    max(x.significant_bits(), y.significant_bits()),
                    T::WIDTH,
                ))
            },
        ))),
        NoSpecialGenerationMode::Random(_) => {
            reshape_2_1_to_3(permute_2_1(Box::new(random_dependent_pairs(
                (),
                random_range(&scramble(&EXAMPLE_SEED, "pow"), 0, T::WIDTH),
                |_, &pow| {
                    random_pairs_from_single(random_range::<T>(
                        &scramble(&EXAMPLE_SEED, "u"),
                        T::ZERO,
                        T::low_mask(pow),
                    ))
                },
            ))))
        }
    }
}

// All triples of `T`, `u64` and `u64`, where `T` is unsigned and the second `u64` is between n and
// `T::WIDTH`, inclusive, where n is the number of significant bits of the `T`.
pub fn triples_of_unsigned_unsigned_and_small_u64_var_2<
    T: PrimitiveUnsigned + Rand + SampleRange,
>(
    gm: NoSpecialGenerationMode,
) -> It<(T, u64, u64)> {
    match gm {
        NoSpecialGenerationMode::Exhaustive => reshape_2_1_to_3(Box::new(dependent_pairs(
            exhaustive_pairs(exhaustive_unsigneds(), exhaustive_unsigneds()),
            |&(u, _): &(T, u64)| {
                Box::new(primitive_int_increasing_inclusive_range(
                    u.significant_bits(),
                    T::WIDTH,
                ))
            },
        ))),
        NoSpecialGenerationMode::Random(_) => {
            reshape_2_1_to_3(permute_2_1(Box::new(random_dependent_pairs(
                (),
                random_range(&scramble(&EXAMPLE_SEED, "pow"), 0, T::WIDTH),
                |_, &pow| {
                    random_pairs(
                        &scramble(&EXAMPLE_SEED, "ps"),
                        &(|seed| {
                            random_range::<T>(&scramble(&seed, "u"), T::ZERO, T::low_mask(pow))
                        }),
                        &(|seed| random(seed)),
                    )
                },
            ))))
        }
    }
}

// All triples of `T`, `T`, and `Vec<T>`, where `T` is unsigned, both `T`s are positive, the `Vec`
// has at least two elements, and the `Vec`'s last element is nonzero.
pub fn triples_of_unsigned_unsigned_and_unsigned_vec_var_1<T: PrimitiveUnsigned + Rand>(
    gm: GenerationMode,
) -> It<(T, T, Vec<T>)> {
    match gm {
        GenerationMode::Exhaustive => Box::new(exhaustive_triples(
            exhaustive_positive_primitive_ints(),
            exhaustive_positive_primitive_ints(),
            exhaustive_vecs_min_length(2, exhaustive_unsigneds())
                .filter(|limbs| *limbs.last().unwrap() != T::ZERO),
        )),
        GenerationMode::Random(scale) => Box::new(random_triples(
            &EXAMPLE_SEED,
            &(|seed| random_positive_unsigned(seed)),
            &(|seed| random_positive_unsigned(seed)),
            &(|seed| {
                random_vecs_min_length(seed, scale, 2, &(|seed_2| random(seed_2)))
                    .filter(|limbs| *limbs.last().unwrap() != T::ZERO)
            }),
        )),
        GenerationMode::SpecialRandom(scale) => Box::new(random_triples(
            &EXAMPLE_SEED,
            &(|seed| special_random_positive_unsigned(seed)),
            &(|seed| special_random_positive_unsigned(seed)),
            &(|seed| {
                special_random_unsigned_vecs_min_length(seed, scale, 2)
                    .filter(|limbs| *limbs.last().unwrap() != T::ZERO)
            }),
        )),
    }
}

// All triples of `T`, `T`, and `Vec<T>`, where `T` is unsigned, both `T`s are positive, the `Vec`
// has at least two elements, the `Vec`'s last element is nonzero and the first `Limb` is not equal
// to the second `Limb` mod the `Natural` represented by the `Vec`.
pub fn triples_of_limb_limb_and_limb_vec_var_2(gm: GenerationMode) -> It<(Limb, Limb, Vec<Limb>)> {
    Box::new(
        triples_of_unsigned_unsigned_and_unsigned_vec_var_1::<Limb>(gm).filter(|&(x, y, ref m)| {
            !Integer::from(Natural::from(x)).eq_mod(-Natural::from(y), Natural::from_limbs_asc(m))
        }),
    )
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

fn triples_of_unsigned_unsigned_and_rounding_mode<T: PrimitiveUnsigned + Rand>(
    gm: GenerationMode,
) -> It<(T, T, RoundingMode)> {
    match gm {
        GenerationMode::Exhaustive => reshape_2_1_to_3(Box::new(lex_pairs(
            exhaustive_pairs_from_single(exhaustive_unsigneds()),
            exhaustive_rounding_modes(),
        ))),
        GenerationMode::Random(_) => Box::new(random_triples(
            &EXAMPLE_SEED,
            &(|seed| random(seed)),
            &(|seed| random(seed)),
            &(|seed| random_rounding_modes(seed)),
        )),
        GenerationMode::SpecialRandom(_) => Box::new(random_triples(
            &EXAMPLE_SEED,
            &(|seed| special_random_unsigned(seed)),
            &(|seed| special_random_unsigned(seed)),
            &(|seed| random_rounding_modes(seed)),
        )),
    }
}

// All triples of `T`, `T`, and `RoundingMode`, where `T` is unsigned and the first `T` can be
// rounded to a multiple of the second, according to the rounding mode.
pub fn triples_of_unsigned_unsigned_and_rounding_mode_var_1<T: PrimitiveUnsigned + Rand>(
    gm: GenerationMode,
) -> It<(T, T, RoundingMode)> {
    Box::new(
        triples_of_unsigned_unsigned_and_rounding_mode::<T>(gm).filter_map(|(x, y, rm)| {
            if x == y {
                Some((x, y, rm))
            } else if y == T::ZERO {
                if rm == RoundingMode::Floor
                    || rm == RoundingMode::Down
                    || rm == RoundingMode::Nearest
                {
                    Some((x, y, rm))
                } else {
                    None
                }
            } else if rm != RoundingMode::Exact {
                x.div_round(y, rm).checked_mul(y).map(|_| (x, y, rm))
            } else {
                x.checked_mul(y).map(|product| (product, y, rm))
            }
        }),
    )
}

// Ignoring RoundingMode::Exact case
fn round_to_multiple_unsigned_valid<T: PrimitiveUnsigned>(x: T, y: T, rm: RoundingMode) -> bool {
    if x == y {
        true
    } else if y == T::ZERO {
        rm == RoundingMode::Down || rm == RoundingMode::Down || rm == RoundingMode::Nearest
    } else {
        x.div_round(y, rm).checked_mul(y).is_some()
    }
}

// Ignoring RoundingMode::Exact case
fn round_to_multiple_signed_valid<T: PrimitiveSigned>(x: T, y: T, rm: RoundingMode) -> bool
where
    T: ConvertibleFrom<<T as UnsignedAbs>::Output> + CheckedFrom<<T as UnsignedAbs>::Output>,
    <T as UnsignedAbs>::Output: PrimitiveUnsigned,
{
    let x_abs = x.unsigned_abs();
    let y_abs = y.unsigned_abs();
    if x >= T::ZERO {
        round_to_multiple_unsigned_valid(x_abs, y_abs, rm)
            && T::convertible_from(x_abs.round_to_multiple(y_abs, rm))
    } else if !round_to_multiple_unsigned_valid(x_abs, y_abs, -rm) {
        false
    } else {
        let abs_result = x_abs.round_to_multiple(y_abs, -rm);
        abs_result == T::MIN.unsigned_abs()
            || T::checked_from(abs_result)
                .and_then(CheckedNeg::checked_neg)
                .is_some()
    }
}

fn triples_of_signed_signed_and_rounding_mode<T: PrimitiveSigned + Rand>(
    gm: GenerationMode,
) -> It<(T, T, RoundingMode)>
where
    T::UnsignedOfEqualWidth: Rand,
    T: WrappingFrom<<T as PrimitiveSigned>::UnsignedOfEqualWidth>,
{
    match gm {
        GenerationMode::Exhaustive => reshape_2_1_to_3(Box::new(lex_pairs(
            exhaustive_pairs_from_single(exhaustive_signeds()),
            exhaustive_rounding_modes(),
        ))),
        GenerationMode::Random(_) => Box::new(random_triples(
            &EXAMPLE_SEED,
            &(|seed| random(seed)),
            &(|seed| random(seed)),
            &(|seed| random_rounding_modes(seed)),
        )),
        GenerationMode::SpecialRandom(_) => Box::new(random_triples(
            &EXAMPLE_SEED,
            &(|seed| special_random_signed(seed)),
            &(|seed| special_random_signed(seed)),
            &(|seed| random_rounding_modes(seed)),
        )),
    }
}

// All triples of `T`, `T`, and `RoundingMode`, where `T` is signed and the first `T` can be rounded
// to a multiple of the second, according to the rounding mode.
pub fn triples_of_signed_signed_and_rounding_mode_var_1<T: PrimitiveSigned + Rand>(
    gm: GenerationMode,
) -> It<(T, T, RoundingMode)>
where
    T: WrappingFrom<<T as PrimitiveSigned>::UnsignedOfEqualWidth>
        + ConvertibleFrom<<T as UnsignedAbs>::Output>
        + CheckedFrom<<T as UnsignedAbs>::Output>,
    <T as UnsignedAbs>::Output: PrimitiveUnsigned,
    T::UnsignedOfEqualWidth: Rand,
{
    Box::new(
        triples_of_signed_signed_and_rounding_mode::<T>(gm).filter_map(|(x, y, rm)| {
            if rm != RoundingMode::Exact {
                if round_to_multiple_signed_valid(x, y, rm) {
                    Some((x, y, rm))
                } else {
                    None
                }
            } else {
                x.checked_mul(y).map(|product| (product, y, rm))
            }
        }),
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
            exhaustive_unsigneds(),
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
            exhaustive_vecs_min_length(2, exhaustive_unsigneds())
                .filter(|limbs| *limbs.last().unwrap() != T::ZERO),
            exhaustive_unsigneds(),
            exhaustive_positive_primitive_ints(),
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
            exhaustive_vecs_min_length(1, exhaustive_unsigneds())
                .filter(|limbs| *limbs.last().unwrap() != T::ZERO),
            exhaustive_unsigneds(),
            exhaustive_positive_primitive_ints(),
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
            .map(|(mut limbs, limb, m)| {
                let carry = limbs_slice_mul_limb_in_place(&mut limbs, m);
                if carry != 0 {
                    limbs.push(carry);
                } else if *limbs.last().unwrap() == 0 {
                    limbs.pop();
                }
                limbs_vec_add_limb_in_place(&mut limbs, limb);
                (limbs, limb, m)
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
            .filter(|(limbs, limb, m)| !limbs_eq_limb_mod_limb(&*limbs, *limb, *m)),
    )
}

fn triples_of_unsigned_vec_unsigned_and_small_unsigned<
    T: PrimitiveUnsigned + Rand,
    U: PrimitiveUnsigned,
>(
    gm: GenerationMode,
) -> It<(Vec<T>, T, U)> {
    match gm {
        GenerationMode::Exhaustive => Box::new(exhaustive_triples(
            exhaustive_vecs(exhaustive_unsigneds()),
            exhaustive_unsigneds(),
            exhaustive_unsigneds(),
        )),
        GenerationMode::Random(scale) => Box::new(random_triples(
            &EXAMPLE_SEED,
            &(|seed| random_vecs(seed, scale, &(|seed_2| random(seed_2)))),
            &(|seed| random(seed)),
            &(|seed| u32s_geometric(seed, scale).flat_map(U::checked_from)),
        )),
        GenerationMode::SpecialRandom(scale) => Box::new(random_triples(
            &EXAMPLE_SEED,
            &(|seed| special_random_unsigned_vecs(seed, scale)),
            &(|seed| special_random_unsigned(seed)),
            &(|seed| u32s_geometric(seed, scale).flat_map(U::checked_from)),
        )),
    }
}

// All triples of `Vec<T>`, T, and small `U`, where `T` and `U` are unsigned, the `Vec` is
// non-empty, and its last element is nonzero.
pub fn triples_of_unsigned_vec_unsigned_and_small_unsigned_var_1<
    T: PrimitiveUnsigned + Rand,
    U: PrimitiveUnsigned,
>(
    gm: GenerationMode,
) -> It<(Vec<T>, T, U)> {
    match gm {
        GenerationMode::Exhaustive => Box::new(exhaustive_triples(
            exhaustive_vecs(exhaustive_unsigneds())
                .filter(|limbs| !limbs.is_empty() && *limbs.last().unwrap() != T::ZERO),
            exhaustive_unsigneds(),
            exhaustive_unsigneds(),
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
            exhaustive_vecs(exhaustive_unsigneds())
                .filter(|limbs| limbs.len() > 1 && *limbs.last().unwrap() != T::ZERO),
            exhaustive_unsigneds(),
            exhaustive_unsigneds(),
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
            exhaustive_vecs(exhaustive_unsigneds()),
            exhaustive_unsigneds(),
            exhaustive_vecs(exhaustive_unsigneds()),
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
// `limbs_eq_limb_mod`.
pub fn triples_of_unsigned_vec_unsigned_and_unsigned_vec_var_1<T: PrimitiveUnsigned + Rand>(
    gm: GenerationMode,
) -> It<(Vec<T>, T, Vec<T>)> {
    permute_1_3_2(triples_of_unsigned_vec_unsigned_vec_and_unsigned_var_8(gm))
}

// All triples of `Vec<Limb>`, `Limb`, and `Vec<Limb>` that meet the preconditions for
// `limbs_eq_limb_mod`, where the `Natural` represented by the first `Vec` is equal to the `Limb`
// mod the `Natural` represented by the second `Vec`.
pub fn triples_of_limb_vec_limb_and_limb_vec_var_2(
    gm: GenerationMode,
) -> It<(Vec<Limb>, Limb, Vec<Limb>)> {
    Box::new(
        triples_of_unsigned_vec_unsigned_and_unsigned_vec_var_1(gm).map(|(xs, y, m)| {
            let mut product_limbs = if xs.is_empty() {
                Vec::new()
            } else {
                limbs_mul(&xs, &m)
            };
            if product_limbs.last() == Some(&0) {
                product_limbs.pop();
            }
            limbs_vec_add_limb_in_place(&mut product_limbs, y);
            (product_limbs, y, m)
        }),
    )
}

// All triples of `Vec<Limb>`, `Limb`, and `Vec<T>` that meet the preconditions for
// `limbs_eq_limb_mod`, where the `Natural` represented by the first `Vec` is not equal to the
// `Limb` mod the `Natural` represented by the second `Vec`.
pub fn triples_of_limb_vec_limb_and_limb_vec_var_3(
    gm: GenerationMode,
) -> It<(Vec<Limb>, Limb, Vec<Limb>)> {
    Box::new(
        triples_of_unsigned_vec_unsigned_and_unsigned_vec_var_1::<Limb>(gm)
            .filter(|(xs, y, m)| !limbs_eq_limb_mod_ref_ref(&*xs, *y, &*m)),
    )
}

// All triples of `Vec<Limb>`, `Limb`, and `Vec<Limb>` that meet the preconditions for
// `limbs_eq_limb_mod`, where the `Natural` represented by the first `Vec` is equal to the negative
// of the `Limb` mod the `Natural` represented by the second `Vec`.
pub fn triples_of_limb_vec_limb_and_limb_vec_var_4(
    gm: GenerationMode,
) -> It<(Vec<Limb>, Limb, Vec<Limb>)> {
    Box::new(
        triples_of_unsigned_vec_unsigned_and_unsigned_vec_var_1(gm).filter_map(|(xs, y, m)| {
            let mut product_limbs = if xs.is_empty() {
                Vec::new()
            } else {
                limbs_mul(&xs, &m)
            };
            if product_limbs.last() == Some(&0) {
                product_limbs.pop();
            }
            if limbs_sub_limb_in_place(&mut product_limbs, y) || *product_limbs.last().unwrap() == 0
            {
                None
            } else {
                Some((product_limbs, y, m))
            }
        }),
    )
}

// All triples of `Vec<Limb>`, `Limb`, and `Vec<Limb>` that meet the preconditions for
// `limbs_eq_limb_mod`, where the `Natural` represented by the first `Vec` is not equal to the
// negative of the `Limb` mod the `Natural` represented by the second `Vec`.
pub fn triples_of_limb_vec_limb_and_limb_vec_var_5(
    gm: GenerationMode,
) -> It<(Vec<Limb>, Limb, Vec<Limb>)> {
    Box::new(
        triples_of_unsigned_vec_unsigned_and_unsigned_vec_var_1::<Limb>(gm).filter(
            |&(ref xs, y, ref m)| {
                !Integer::from(Natural::from_limbs_asc(xs))
                    .eq_mod(-Natural::from(y), Natural::from_limbs_asc(m))
            },
        ),
    )
}

// All quadruples of `T`, `T`, `T` and `u64`, where `T` is unsigned and the `u64` is between n and
// `T::WIDTH`, inclusive, where n is the maximum number of significant bits of the three `T`s.
pub fn quadruples_of_three_unsigneds_and_small_u64_var_1<
    T: PrimitiveUnsigned + Rand + SampleRange,
>(
    gm: NoSpecialGenerationMode,
) -> It<(T, T, T, u64)> {
    match gm {
        NoSpecialGenerationMode::Exhaustive => reshape_3_1_to_4(Box::new(dependent_pairs(
            exhaustive_triples_from_single(exhaustive_unsigneds()),
            |&(x, y, z): &(T, T, T)| {
                Box::new(primitive_int_increasing_inclusive_range(
                    max!(
                        x.significant_bits(),
                        y.significant_bits(),
                        z.significant_bits()
                    ),
                    T::WIDTH,
                ))
            },
        ))),
        NoSpecialGenerationMode::Random(_) => {
            reshape_3_1_to_4(permute_2_1(Box::new(random_dependent_pairs(
                (),
                random_range(&scramble(&EXAMPLE_SEED, "pow"), 0, T::WIDTH),
                |_, &pow| {
                    random_triples_from_single(random_range::<T>(
                        &scramble(&EXAMPLE_SEED, "u"),
                        T::ZERO,
                        T::low_mask(pow),
                    ))
                },
            ))))
        }
    }
}

fn quadruples_of_unsigned_vec_small_unsigned_small_unsigned_and_unsigned_vec<
    T: PrimitiveUnsigned + Rand,
    U: PrimitiveUnsigned + Rand,
>(
    gm: GenerationMode,
) -> It<(Vec<T>, U, U, Vec<T>)> {
    permute_1_3_4_2(reshape_2_2_to_4(match gm {
        GenerationMode::Exhaustive => Box::new(sqrt_pairs(
            exhaustive_pairs_from_single(exhaustive_vecs(exhaustive_unsigneds())),
            exhaustive_pairs_from_single(exhaustive_unsigneds()),
        )),
        GenerationMode::Random(scale) => Box::new(random_pairs(
            &EXAMPLE_SEED,
            &(|seed| {
                random_pairs_from_single(random_vecs(seed, scale, &(|seed_2| random(seed_2))))
            }),
            &(|seed| {
                random_pairs_from_single(u32s_geometric(seed, scale).flat_map(U::checked_from))
            }),
        )),
        GenerationMode::SpecialRandom(scale) => Box::new(random_pairs(
            &EXAMPLE_SEED,
            &(|seed| random_pairs_from_single(special_random_unsigned_vecs(seed, scale))),
            &(|seed| {
                random_pairs_from_single(u32s_geometric(seed, scale).flat_map(U::checked_from))
            }),
        )),
    }))
}

pub fn quadruples_of_unsigned_vec_small_unsigned_small_unsigned_and_unsigned_vec_var_1<
    T: PrimitiveUnsigned + Rand,
    U: PrimitiveUnsigned + Rand,
>(
    gm: GenerationMode,
) -> It<(Vec<T>, U, U, Vec<T>)> {
    Box::new(
        quadruples_of_unsigned_vec_small_unsigned_small_unsigned_and_unsigned_vec(gm)
            .filter(|&(_, start, end, _)| start < end),
    )
}

pub fn quadruples_of_unsigned_vec_small_unsigned_small_unsigned_and_unsigned_vec_var_2<
    T: PrimitiveUnsigned + Rand,
    U: PrimitiveUnsigned + Rand,
>(
    gm: GenerationMode,
) -> It<(Vec<T>, U, U, Vec<T>)> {
    Box::new(
        quadruples_of_unsigned_vec_small_unsigned_small_unsigned_and_unsigned_vec(gm)
            .filter(|&(ref limbs, start, end, _)| start < end && !slice_test_zero(limbs)),
    )
}

pub fn quadruples_of_three_unsigneds_and_small_unsigned<
    T: PrimitiveUnsigned + Rand,
    U: PrimitiveUnsigned,
>(
    gm: GenerationMode,
) -> It<(T, T, T, U)> {
    match gm {
        GenerationMode::Exhaustive => Box::new(reshape_3_1_to_4(Box::new(sqrt_pairs(
            exhaustive_triples_from_single(exhaustive_unsigneds()),
            exhaustive_unsigneds(),
        )))),
        GenerationMode::Random(scale) => Box::new(random_quadruples(
            &EXAMPLE_SEED,
            &(|seed| random(seed)),
            &(|seed| random(seed)),
            &(|seed| random(seed)),
            &(|seed| u32s_geometric(seed, scale).flat_map(U::checked_from)),
        )),
        GenerationMode::SpecialRandom(scale) => Box::new(random_quadruples(
            &EXAMPLE_SEED,
            &(|seed| special_random_unsigned(seed)),
            &(|seed| special_random_unsigned(seed)),
            &(|seed| special_random_unsigned(seed)),
            &(|seed| u32s_geometric(seed, scale).flat_map(U::checked_from)),
        )),
    }
}

pub fn quadruples_of_three_signeds_and_small_unsigned<
    T: PrimitiveSigned + Rand,
    U: PrimitiveUnsigned + Rand,
>(
    gm: GenerationMode,
) -> It<(T, T, T, U)>
where
    T::UnsignedOfEqualWidth: Rand,
    T: WrappingFrom<<T as PrimitiveSigned>::UnsignedOfEqualWidth>,
{
    match gm {
        GenerationMode::Exhaustive => Box::new(reshape_3_1_to_4(Box::new(sqrt_pairs(
            exhaustive_triples_from_single(exhaustive_signeds()),
            exhaustive_unsigneds(),
        )))),
        GenerationMode::Random(scale) => Box::new(random_quadruples(
            &EXAMPLE_SEED,
            &(|seed| random(seed)),
            &(|seed| random(seed)),
            &(|seed| random(seed)),
            &(|seed| u32s_geometric(seed, scale).flat_map(U::checked_from)),
        )),
        GenerationMode::SpecialRandom(scale) => Box::new(random_quadruples(
            &EXAMPLE_SEED,
            &(|seed| special_random_signed(seed)),
            &(|seed| special_random_signed(seed)),
            &(|seed| special_random_signed(seed)),
            &(|seed| u32s_geometric(seed, scale).flat_map(U::checked_from)),
        )),
    }
}

fn digits_valid<T: PrimitiveUnsigned, U: PrimitiveUnsigned>(log_base: u64, digits: &[U]) -> bool {
    let digits = &digits[..digits.len() - slice_trailing_zeros(&digits)];
    if digits.is_empty() {
        return true;
    }
    let significant_bits = ((u64::wrapping_from(digits.len()) - 1) * log_base)
        + digits.last().unwrap().significant_bits();
    significant_bits <= T::WIDTH
}

fn pairs_of_u64_and_unsigned_vec_var_1_helper<
    T: PrimitiveUnsigned,
    U: PrimitiveUnsigned + Rand + SampleRange,
>(
    &scale: &u32,
    &log_base: &u64,
) -> It<Vec<U>> {
    Box::new(
        random_vecs(
            &EXAMPLE_SEED,
            scale,
            &(|seed| random_range_down(seed, U::low_mask(log_base))),
        )
        .map(move |digits| (log_base, digits))
        .filter_map(|(log_base, digits)| {
            if digits_valid::<T, U>(log_base, &digits) {
                Some(digits)
            } else {
                None
            }
        }),
    )
}

fn pairs_of_u64_and_unsigned_vec_var_3_helper<T: PrimitiveUnsigned + Rand + SampleRange>(
    &scale: &u32,
    &log_base: &u64,
) -> It<Vec<T>> {
    Box::new(random_vecs(
        &EXAMPLE_SEED,
        scale,
        &(|seed| random_range_down(seed, T::low_mask(log_base))),
    ))
}

// All pairs of `u64` and `Vec<U>`, where each pair is a valid input to
// `from_power_of_two_digits_asc<T, U>`.
pub fn pairs_of_u64_and_unsigned_vec_var_1<
    T: PrimitiveUnsigned,
    U: PrimitiveUnsigned + Rand + SampleRange,
>(
    gm: NoSpecialGenerationMode,
) -> It<(u64, Vec<U>)> {
    match gm {
        NoSpecialGenerationMode::Exhaustive => {
            let f = |_: &(), &log_base: &u64| -> It<Option<Vec<U>>> {
                if log_base > U::WIDTH {
                    Box::new(repeat(None))
                } else {
                    let digit_vecs = if log_base == 1 {
                        exhaustive_vecs_shortlex(primitive_int_increasing_range(U::ZERO, U::TWO))
                    } else if log_base == U::WIDTH {
                        Box::new(exhaustive_vecs(exhaustive_unsigneds()))
                    } else {
                        Box::new(exhaustive_vecs(primitive_int_increasing_range(
                            U::ZERO,
                            U::power_of_two(log_base),
                        )))
                    };
                    Box::new(digit_vecs.filter_map(move |digits| {
                        Some(if digits_valid::<T, U>(log_base, &digits) {
                            Some(digits)
                        } else {
                            None
                        })
                    }))
                }
            };
            Box::new(
                exhaustive_dependent_pairs_infinite_sqrt(
                    (),
                    exhaustive_positive_primitive_ints(),
                    f,
                )
                .filter_map(|(log_base, digits)| {
                    if let Some(digits) = digits {
                        Some((log_base, digits))
                    } else {
                        None
                    }
                }),
            )
        }
        NoSpecialGenerationMode::Random(scale) => Box::new(random_dependent_pairs(
            scale,
            random_range(&EXAMPLE_SEED, 1, U::WIDTH),
            pairs_of_u64_and_unsigned_vec_var_1_helper::<T, U>,
        )),
    }
}

// All pairs of `u64` and `Vec<U>`, where each pair is a valid input to
// `from_power_of_two_digits_desc<T, U>`.
pub fn pairs_of_u64_and_unsigned_vec_var_2<
    T: PrimitiveUnsigned,
    U: PrimitiveUnsigned + Rand + SampleRange,
>(
    gm: NoSpecialGenerationMode,
) -> It<(u64, Vec<U>)> {
    Box::new(
        pairs_of_u64_and_unsigned_vec_var_1::<T, U>(gm)
            .map(|(log_base, digits)| (log_base, digits.iter().rev().cloned().collect())),
    )
}

// All pairs of `u64` and `Vec<T>`, where each pair is a valid input to
// `from_power_of_two_digits_asc<Natural, T>`.
pub fn pairs_of_u64_and_unsigned_vec_var_3<T: PrimitiveUnsigned + Rand + SampleRange>(
    gm: NoSpecialGenerationMode,
) -> It<(u64, Vec<T>)> {
    match gm {
        NoSpecialGenerationMode::Exhaustive => {
            let f = |_: &(), &log_base: &u64| -> It<Option<Vec<T>>> {
                if log_base > T::WIDTH {
                    Box::new(repeat(None))
                } else {
                    let digit_vecs = if log_base == 1 {
                        exhaustive_vecs_shortlex(primitive_int_increasing_range(T::ZERO, T::TWO))
                    } else if log_base == T::WIDTH {
                        Box::new(exhaustive_vecs(exhaustive_unsigneds()))
                    } else {
                        Box::new(exhaustive_vecs(primitive_int_increasing_range(
                            T::ZERO,
                            T::power_of_two(log_base),
                        )))
                    };
                    Box::new(digit_vecs.map(Option::Some))
                }
            };
            Box::new(
                exhaustive_dependent_pairs_infinite_sqrt(
                    (),
                    exhaustive_positive_primitive_ints(),
                    f,
                )
                .filter_map(|(log_base, digits)| {
                    if let Some(digits) = digits {
                        Some((log_base, digits))
                    } else {
                        None
                    }
                }),
            )
        }
        NoSpecialGenerationMode::Random(scale) => Box::new(random_dependent_pairs(
            scale,
            random_range(&EXAMPLE_SEED, 1, T::WIDTH),
            pairs_of_u64_and_unsigned_vec_var_3_helper::<T>,
        )),
    }
}

// All pairs of `u64` and `Vec<T>`, where each pair is a valid input to
// `from_power_of_two_digits_asc<Natural, T>`.
pub fn pairs_of_u64_and_unsigned_vec_var_4<T: PrimitiveUnsigned + Rand + SampleRange>(
    gm: NoSpecialGenerationMode,
) -> It<(u64, Vec<T>)> {
    Box::new(
        pairs_of_u64_and_unsigned_vec_var_3::<T>(gm)
            .map(|(log_base, digits)| (log_base, digits.iter().rev().cloned().collect())),
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
            exhaustive_vecs(exhaustive_unsigneds()),
            exhaustive_unsigneds(),
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
            .filter(|&(ref limbs, _, _)| !slice_test_zero(limbs)),
    )
}

fn triples_of_unsigned_unsigned_vec_and_rounding_mode<T: PrimitiveUnsigned + Rand>(
    gm: GenerationMode,
) -> It<(T, Vec<T>, RoundingMode)> {
    match gm {
        GenerationMode::Exhaustive => Box::new(exhaustive_triples(
            exhaustive_unsigneds(),
            exhaustive_vecs(exhaustive_unsigneds()),
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
pub fn triples_of_unsigned_unsigned_vec_and_rounding_mode_var_1<T: PrimitiveUnsigned + Rand>(
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
            log_pairs(exhaustive_unsigneds(), exhaustive_unsigneds()),
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
    T: ArithmeticCheckedShl<U, Output = T>,
{
    Box::new(
        triples_of_unsigned_small_unsigned_and_rounding_mode::<T, U>(gm).filter_map(
            |(n, u, rm)| {
                if n == T::ZERO || rm != RoundingMode::Exact {
                    Some((n, u, rm))
                } else {
                    n.arithmetic_checked_shl(u).map(|shifted| (shifted, u, rm))
                }
            },
        ),
    )
}

// All triples of `T`, small `u64`, and `RoundingMode`, where `T` is unsigned and the `T` can be
// rounded to a multiple of 2 to the power of the `u64`, according to the rounding mode.
pub fn triples_of_unsigned_small_u64_and_rounding_mode_var_2<T: PrimitiveUnsigned + Rand>(
    gm: GenerationMode,
) -> It<(T, u64, RoundingMode)> {
    Box::new(
        triples_of_unsigned_small_unsigned_and_rounding_mode::<T, u64>(gm).filter_map(
            |(n, u, rm)| {
                if n == T::ZERO || rm != RoundingMode::Exact {
                    n.shr_round(u, rm)
                        .arithmetic_checked_shl(u)
                        .map(|_| (n, u, rm))
                } else {
                    n.arithmetic_checked_shl(u).map(|shifted| (shifted, u, rm))
                }
            },
        ),
    )
}

// All triples of `T`, `U` and `u64`, where `T` is unsigned, `U` is signed, and the `u64` is between
// the number of significant_bits of the first `T` and `T::WIDTH`, inclusive.
pub fn triples_of_unsigned_small_signed_and_small_unsigned_var_1<
    T: PrimitiveUnsigned + Rand + SampleRange,
    U: PrimitiveSigned,
>(
    gm: NoSpecialGenerationMode,
) -> It<(T, U, u64)> {
    match gm {
        NoSpecialGenerationMode::Exhaustive => reshape_2_1_to_3(Box::new(dependent_pairs(
            sqrt_pairs_of_unsigned_and_signed(),
            |&(x, _): &(T, _)| {
                Box::new(primitive_int_increasing_inclusive_range(
                    x.significant_bits(),
                    T::WIDTH,
                ))
            },
        ))),
        NoSpecialGenerationMode::Random(scale) => {
            reshape_2_1_to_3(permute_2_1(Box::new(random_dependent_pairs(
                scale,
                random_range(&scramble(&EXAMPLE_SEED, "pow"), 0, T::WIDTH),
                |&scale, &pow| {
                    Box::new(random_pairs(
                        &scramble(&EXAMPLE_SEED, "u"),
                        &(|seed| random_range::<T>(seed, T::ZERO, T::low_mask(pow))),
                        &(|seed| i32s_geometric(seed, scale).flat_map(U::checked_from)),
                    ))
                },
            ))))
        }
    }
}

fn triples_of_unsigned_small_signed_and_rounding_mode<
    T: PrimitiveUnsigned + Rand,
    U: PrimitiveSigned,
>(
    gm: GenerationMode,
) -> It<(T, U, RoundingMode)> {
    match gm {
        GenerationMode::Exhaustive => reshape_2_1_to_3(Box::new(lex_pairs(
            sqrt_pairs_of_unsigned_and_signed(),
            exhaustive_rounding_modes(),
        ))),
        GenerationMode::Random(scale) => Box::new(random_triples(
            &EXAMPLE_SEED,
            &(|seed| random(seed)),
            &(|seed| i32s_geometric(seed, scale).flat_map(U::checked_from)),
            &(|seed| random_rounding_modes(seed)),
        )),
        GenerationMode::SpecialRandom(scale) => Box::new(random_triples(
            &EXAMPLE_SEED,
            &(|seed| special_random_unsigned(seed)),
            &(|seed| i32s_geometric(seed, scale).flat_map(U::checked_from)),
            &(|seed| random_rounding_modes(seed)),
        )),
    }
}

// All triples of `T`, small `U`, and `RoundingMode`, where `T` is unsigned, `U` is signed, and if
// the `RoundingMode` is `RoundingMode::Exact`, the `T` is divisible by 2 to the power of the `U`.
pub fn triples_of_unsigned_small_signed_and_rounding_mode_var_1<
    T: PrimitiveUnsigned + Rand,
    U: PrimitiveSigned,
>(
    gm: GenerationMode,
) -> It<(T, U, RoundingMode)>
where
    T: Shl<U, Output = T> + Shr<U, Output = T>,
{
    Box::new(
        triples_of_unsigned_small_signed_and_rounding_mode(gm).filter_map(|(n, i, rm)| {
            if n == T::ZERO || i < U::ZERO {
                Some((n, i, rm))
            } else if rm == RoundingMode::Exact {
                if i >= U::exact_from(T::WIDTH) {
                    None
                } else {
                    let shifted = n << i;
                    if shifted >> i == n {
                        Some((shifted, i, rm))
                    } else {
                        None
                    }
                }
            } else {
                Some((n, i, rm))
            }
        }),
    )
}

// All triples of `T`, small `U`, and `RoundingMode`, where `T` is unsigned, `U` is signed, and if
// the `RoundingMode` is `RoundingMode::Exact`, the `T` is divisible by 2 to the power of the
// negative of the `U`.
pub fn triples_of_unsigned_small_signed_and_rounding_mode_var_2<
    T: PrimitiveUnsigned + Rand,
    U: PrimitiveSigned,
>(
    gm: GenerationMode,
) -> It<(T, U, RoundingMode)>
where
    T: Shl<U, Output = T> + Shr<U, Output = T> + ArithmeticCheckedShr<U, Output = T>,
{
    Box::new(
        triples_of_unsigned_small_signed_and_rounding_mode::<T, U>(gm).filter_map(|(n, i, rm)| {
            if n == T::ZERO || i >= U::ZERO || rm != RoundingMode::Exact {
                Some((n, i, rm))
            } else {
                n.arithmetic_checked_shr(i).map(|shifted| (shifted, i, rm))
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
            log_pairs(exhaustive_signeds(), exhaustive_unsigneds()),
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
    T: ArithmeticCheckedShl<U, Output = T>
        + WrappingFrom<<T as PrimitiveSigned>::UnsignedOfEqualWidth>,
    T::UnsignedOfEqualWidth: Rand,
{
    Box::new(
        triples_of_signed_small_unsigned_and_rounding_mode::<T, U>(gm).filter_map(|(n, u, rm)| {
            if n == T::ZERO || rm != RoundingMode::Exact {
                Some((n, u, rm))
            } else {
                n.arithmetic_checked_shl(u).map(|shifted| (shifted, u, rm))
            }
        }),
    )
}

// All triples of `T`, small `u64`, and `RoundingMode`, where `T` is signed and the `T` can be
// rounded to a multiple of 2 to the power of the `u64`, according to the rounding mode.
pub fn triples_of_signed_small_u64_and_rounding_mode_var_2<T: PrimitiveSigned + Rand>(
    gm: GenerationMode,
) -> It<(T, u64, RoundingMode)>
where
    T: WrappingFrom<<T as PrimitiveSigned>::UnsignedOfEqualWidth>,
    T::UnsignedOfEqualWidth: Rand,
{
    Box::new(
        triples_of_signed_small_unsigned_and_rounding_mode::<T, u64>(gm).filter_map(
            |(n, u, rm)| {
                if n == T::ZERO || rm != RoundingMode::Exact {
                    n.shr_round(u, rm)
                        .arithmetic_checked_shl(u)
                        .map(|_| (n, u, rm))
                } else {
                    n.arithmetic_checked_shl(u).map(|shifted| (shifted, u, rm))
                }
            },
        ),
    )
}

fn triples_of_signed_small_signed_and_rounding_mode<
    T: PrimitiveSigned + Rand,
    U: PrimitiveSigned + Rand,
>(
    gm: GenerationMode,
) -> It<(T, U, RoundingMode)>
where
    T::UnsignedOfEqualWidth: Rand,
    T: WrappingFrom<<T as PrimitiveSigned>::UnsignedOfEqualWidth>,
{
    match gm {
        GenerationMode::Exhaustive => reshape_2_1_to_3(Box::new(lex_pairs(
            log_pairs(exhaustive_signeds(), exhaustive_signeds()),
            exhaustive_rounding_modes(),
        ))),
        GenerationMode::Random(scale) => Box::new(random_triples(
            &EXAMPLE_SEED,
            &(|seed| random(seed)),
            &(|seed| i32s_geometric(seed, scale).flat_map(U::checked_from)),
            &(|seed| random_rounding_modes(seed)),
        )),
        GenerationMode::SpecialRandom(scale) => Box::new(random_triples(
            &EXAMPLE_SEED,
            &(|seed| special_random_signed(seed)),
            &(|seed| i32s_geometric(seed, scale).flat_map(U::checked_from)),
            &(|seed| random_rounding_modes(seed)),
        )),
    }
}

// All triples of `T`, small `U`, and `RoundingMode`, where `T` and `U` are signed and if the
// `RoundingMode` is `RoundingMode::Exact`, the `T` is divisible by 2 to the power of the `U`.
pub fn triples_of_signed_small_signed_and_rounding_mode_var_1<
    T: PrimitiveSigned + Rand,
    U: PrimitiveSigned + Rand,
>(
    gm: GenerationMode,
) -> It<(T, U, RoundingMode)>
where
    T: Shl<U, Output = T>
        + Shr<U, Output = T>
        + WrappingFrom<<T as PrimitiveSigned>::UnsignedOfEqualWidth>
        + ArithmeticCheckedShl<U, Output = T>,
    T::UnsignedOfEqualWidth: Rand,
{
    Box::new(
        triples_of_signed_small_signed_and_rounding_mode::<T, U>(gm).filter_map(|(n, i, rm)| {
            if n == T::ZERO || i < U::ZERO || rm != RoundingMode::Exact {
                Some((n, i, rm))
            } else {
                n.arithmetic_checked_shl(i).map(|shifted| (shifted, i, rm))
            }
        }),
    )
}

// All triples of `T`, small `U`, and `RoundingMode`, where `T` and `U` are signed and if the
// `RoundingMode` is `RoundingMode::Exact`, the `T` is divisible by 2 to the power of the negative
// of the `U`.
pub fn triples_of_signed_small_signed_and_rounding_mode_var_2<
    T: PrimitiveSigned + Rand,
    U: PrimitiveSigned + Rand,
>(
    gm: GenerationMode,
) -> It<(T, U, RoundingMode)>
where
    T: Shl<U, Output = T>
        + Shr<U, Output = T>
        + WrappingFrom<<T as PrimitiveSigned>::UnsignedOfEqualWidth>,
    T::UnsignedOfEqualWidth: Rand,
{
    Box::new(
        triples_of_signed_small_signed_and_rounding_mode::<T, U>(gm).filter_map(|(n, i, rm)| {
            if n == T::ZERO || i >= U::ZERO {
                Some((n, i, rm))
            } else if rm == RoundingMode::Exact {
                if -i >= U::exact_from(T::WIDTH) {
                    None
                } else {
                    let shifted = n << -i;
                    if shifted >> -i == n {
                        Some((shifted, i, rm))
                    } else {
                        None
                    }
                }
            } else {
                Some((n, i, rm))
            }
        }),
    )
}

fn triples_of_unsigned_positive_unsigned_and_rounding_mode<T: PrimitiveUnsigned + Rand>(
    gm: GenerationMode,
) -> It<(T, T, RoundingMode)> {
    match gm {
        GenerationMode::Exhaustive => reshape_2_1_to_3(Box::new(lex_pairs(
            exhaustive_pairs(exhaustive_unsigneds(), exhaustive_positive_primitive_ints()),
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

// All triples of `T`, positive `T`, and `RoundingMode`, where `T` is unsigned and if the
// `RoundingMode` is `RoundingMode::Exact`, the first `T` is divisible by the second.
pub fn triples_of_unsigned_positive_unsigned_and_rounding_mode_var_1<
    T: PrimitiveUnsigned + Rand,
>(
    gm: GenerationMode,
) -> It<(T, T, RoundingMode)> {
    Box::new(
        triples_of_unsigned_positive_unsigned_and_rounding_mode::<T>(gm).filter_map(
            |(x, y, rm)| {
                if rm == RoundingMode::Exact {
                    x.checked_mul(y).map(|product| (product, y, rm))
                } else {
                    Some((x, y, rm))
                }
            },
        ),
    )
}

fn triples_of_signed_nonzero_signed_and_rounding_mode<T: PrimitiveSigned + Rand>(
    gm: GenerationMode,
) -> It<(T, T, RoundingMode)>
where
    T: WrappingFrom<<T as PrimitiveSigned>::UnsignedOfEqualWidth>,
    T::UnsignedOfEqualWidth: Rand,
{
    match gm {
        GenerationMode::Exhaustive => reshape_2_1_to_3(Box::new(lex_pairs(
            exhaustive_pairs(exhaustive_signeds(), exhaustive_nonzero_signeds()),
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

// All triples of `T`, nonzero `T`, and `RoundingMode`, where `T` is signed, the `T`s are not
// `(T::MIN, T::NEGATIVE_ONE)` and if the `RoundingMode` is `RoundingMode::Exact`, the first `T` is
// divisible by the second.
pub fn triples_of_signed_nonzero_signed_and_rounding_mode_var_1<T: PrimitiveSigned + Rand>(
    gm: GenerationMode,
) -> It<(T, T, RoundingMode)>
where
    T: WrappingFrom<<T as PrimitiveSigned>::UnsignedOfEqualWidth>,
    T::UnsignedOfEqualWidth: Rand,
{
    Box::new(
        triples_of_signed_nonzero_signed_and_rounding_mode::<T>(gm).filter_map(|(x, y, rm)| {
            if x == T::MIN && y == T::NEGATIVE_ONE {
                None
            } else if rm == RoundingMode::Exact {
                x.checked_mul(y).map(|product| (product, y, rm))
            } else {
                Some((x, y, rm))
            }
        }),
    )
}

pub(crate) struct RandomValueAndVecOfBool<T> {
    pub(crate) xs: It<T>,
    pub(crate) rng: Box<IsaacRng>,
}

impl<T: BitConvertible> Iterator for RandomValueAndVecOfBool<T> {
    type Item = (T, Vec<bool>);

    fn next(&mut self) -> Option<(T, Vec<bool>)> {
        let n = self.xs.next().unwrap();
        let mut bools = Vec::new();
        for _ in n.to_bits_asc() {
            bools.push(self.rng.gen::<bool>());
        }
        Some((n, bools))
    }
}

fn random_pairs_of_primitive_and_vec_of_bool_var_1<T: PrimitiveInt + Rand>(
    seed: &[u32],
) -> RandomValueAndVecOfBool<T> {
    RandomValueAndVecOfBool {
        xs: Box::new(random(&scramble(seed, "naturals"))),
        rng: Box::new(IsaacRng::from_seed(&scramble(seed, "bools"))),
    }
}

fn special_random_pairs_of_unsigned_and_vec_of_bool_var_1<T: PrimitiveUnsigned + Rand>(
    seed: &[u32],
) -> RandomValueAndVecOfBool<T> {
    RandomValueAndVecOfBool {
        xs: Box::new(special_random_unsigned(&scramble(seed, "naturals"))),
        rng: Box::new(IsaacRng::from_seed(&scramble(seed, "bools"))),
    }
}

// All pairs of `T` and `Vec<bool>`, where `T` is unsigned and the length of the `Vec` is equal to
// the significant bit count of the `T`.
pub fn pairs_of_unsigned_and_vec_of_bool_var_1<T: PrimitiveUnsigned + Rand>(
    gm: GenerationMode,
) -> It<(T, Vec<bool>)> {
    match gm {
        GenerationMode::Exhaustive => {
            let f = |n: &T| {
                exhaustive_fixed_length_vecs_from_single(
                    usize::wrapping_from(n.significant_bits()),
                    exhaustive_bools(),
                )
            };
            Box::new(dependent_pairs(exhaustive_unsigneds(), f))
        }
        GenerationMode::Random(_) => Box::new(random_pairs_of_primitive_and_vec_of_bool_var_1(
            &EXAMPLE_SEED,
        )),
        GenerationMode::SpecialRandom(_) => Box::new(
            special_random_pairs_of_unsigned_and_vec_of_bool_var_1(&EXAMPLE_SEED),
        ),
    }
}

fn special_random_pairs_of_signed_and_vec_of_bool_var_1<T: PrimitiveSigned + Rand>(
    seed: &[u32],
) -> RandomValueAndVecOfBool<T>
where
    T::UnsignedOfEqualWidth: Rand,
    T: WrappingFrom<<T as PrimitiveSigned>::UnsignedOfEqualWidth>,
{
    RandomValueAndVecOfBool {
        xs: Box::new(special_random_signed(&scramble(seed, "naturals"))),
        rng: Box::new(IsaacRng::from_seed(&scramble(seed, "bools"))),
    }
}

// All pairs of `T` and `Vec<bool>`, where `T` is signed and the length of the `Vec` is equal to the
// significant bit count of the `T`.
pub fn pairs_of_signed_and_vec_of_bool_var_1<T: PrimitiveSigned + Rand>(
    gm: GenerationMode,
) -> It<(T, Vec<bool>)>
where
    T::UnsignedOfEqualWidth: Rand,
    T: WrappingFrom<<T as PrimitiveSigned>::UnsignedOfEqualWidth>,
{
    match gm {
        GenerationMode::Exhaustive => {
            let f = |n: &T| {
                exhaustive_fixed_length_vecs_from_single(n.to_bits_asc().len(), exhaustive_bools())
            };
            Box::new(dependent_pairs(exhaustive_signeds(), f))
        }
        GenerationMode::Random(_) => Box::new(random_pairs_of_primitive_and_vec_of_bool_var_1(
            &EXAMPLE_SEED,
        )),
        GenerationMode::SpecialRandom(_) => Box::new(
            special_random_pairs_of_signed_and_vec_of_bool_var_1(&EXAMPLE_SEED),
        ),
    }
}

pub(crate) struct RandomValueSmallU64AndVecOfBool<T> {
    pub(crate) ps: It<(T, u64)>,
    pub(crate) rng: Box<IsaacRng>,
}

impl<T: Copy + SignificantBits> Iterator for RandomValueSmallU64AndVecOfBool<T> {
    type Item = (T, u64, Vec<bool>);

    fn next(&mut self) -> Option<(T, u64, Vec<bool>)> {
        let (u, log_base) = self.ps.next().unwrap();
        let mut bools = Vec::new();
        for _ in 0..u
            .significant_bits()
            .div_round(log_base, RoundingMode::Ceiling)
        {
            bools.push(self.rng.gen::<bool>());
        }
        Some((u, log_base, bools))
    }
}

fn random_triples_of_unsigned_small_u64_and_vec_of_bool_var_1<
    T: PrimitiveUnsigned + Rand,
    U: PrimitiveUnsigned,
>(
    seed: &[u32],
) -> RandomValueSmallU64AndVecOfBool<T> {
    RandomValueSmallU64AndVecOfBool {
        ps: Box::new(
            random_pairs_of_primitive_and_small_unsigned_var_1_with_seed::<T, U>(&scramble(
                seed, "pairs",
            )),
        ),
        rng: Box::new(IsaacRng::from_seed(&scramble(seed, "bools"))),
    }
}

fn special_random_triples_of_unsigned_small_u64_and_vec_of_bool_var_1<
    T: PrimitiveUnsigned + Rand,
    U: PrimitiveUnsigned,
>(
    seed: &[u32],
) -> RandomValueSmallU64AndVecOfBool<T> {
    RandomValueSmallU64AndVecOfBool {
        ps: Box::new(
            special_random_pairs_of_unsigned_and_small_unsigned_var_1_with_seed::<T, U>(&scramble(
                seed, "pairs",
            )),
        ),
        rng: Box::new(IsaacRng::from_seed(&scramble(seed, "bools"))),
    }
}

// All pairs of `T`, `u64` and `Vec<bool>`, where `T` and `U` are unsigned, `log_base` is no greater
// than `U::WIDTH`, and the length of the `Vec` is equal to the significant base-
// 2<sup>`log_base`</sup>-digit count of the `T`.
pub fn triples_of_unsigned_small_u64_and_vec_of_bool_var_1<
    T: PrimitiveUnsigned + Rand,
    U: PrimitiveUnsigned,
>(
    gm: GenerationMode,
) -> It<(T, u64, Vec<bool>)> {
    match gm {
        GenerationMode::Exhaustive => {
            let f = |&(u, log_base): &(T, u64)| {
                exhaustive_fixed_length_vecs_from_single(
                    usize::wrapping_from(u.significant_bits())
                        .div_round(usize::wrapping_from(log_base), RoundingMode::Ceiling),
                    exhaustive_bools(),
                )
            };
            reshape_2_1_to_3(Box::new(dependent_pairs(
                pairs_of_unsigned_and_small_u64_var_1::<T, U>(gm),
                f,
            )))
        }
        GenerationMode::Random(_) => Box::new(
            random_triples_of_unsigned_small_u64_and_vec_of_bool_var_1::<T, U>(&EXAMPLE_SEED),
        ),
        GenerationMode::SpecialRandom(_) => Box::new(
            special_random_triples_of_unsigned_small_u64_and_vec_of_bool_var_1::<T, U>(
                &EXAMPLE_SEED,
            ),
        ),
    }
}

pub fn strings(gm: NoSpecialGenerationMode) -> It<String> {
    match gm {
        NoSpecialGenerationMode::Exhaustive => Box::new(exhaustive_strings()),
        NoSpecialGenerationMode::Random(scale) => Box::new(random_strings(&EXAMPLE_SEED, scale)),
    }
}

pub fn ascii_strings(gm: NoSpecialGenerationMode) -> It<String> {
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

pub fn pairs_of_ascii_strings(gm: NoSpecialGenerationMode) -> It<(String, String)> {
    match gm {
        NoSpecialGenerationMode::Exhaustive => Box::new(exhaustive_pairs_from_single(
            exhaustive_strings_with_chars(exhaustive_ascii_chars()),
        )),
        NoSpecialGenerationMode::Random(scale) => Box::new(random_pairs_from_single(
            random_strings_with_chars(&EXAMPLE_SEED, scale, &|seed| random_ascii_chars(seed)),
        )),
    }
}

//TODO replace with something better please
fn naive_factors(x: usize) -> Vec<(usize, usize)> {
    let mut factors = Vec::new();
    for i in 0..=x {
        for j in 0..=x {
            if i * j <= x {
                factors.push((i, j));
            }
        }
    }
    factors
}

// All pairs of `Vec<T>`, `usize`, and `usize`, where `T` is unsigned and the length of the `Vec` is
// at least the product of the `usize`s.
pub fn triples_of_unsigned_vec_usize_usize_var_1<T: PrimitiveUnsigned + Rand>(
    gm: NoSpecialGenerationMode,
) -> It<(Vec<T>, usize, usize)> {
    let ps: It<(Vec<T>, (usize, usize))> = match gm {
        NoSpecialGenerationMode::Exhaustive => Box::new(dependent_pairs(
            exhaustive_vecs(exhaustive_unsigneds()),
            |xs| Box::new(naive_factors(xs.len()).into_iter()),
        )),
        NoSpecialGenerationMode::Random(scale) => Box::new(random_dependent_pairs(
            (),
            Box::new(random_vecs(
                &scramble(&EXAMPLE_SEED, "xs"),
                scale,
                &(|seed| random(seed)),
            )),
            |_, xs| random_from_vector(&scramble(&EXAMPLE_SEED, "p"), naive_factors(xs.len())),
        )),
    };
    reshape_1_2_to_3(ps)
}
