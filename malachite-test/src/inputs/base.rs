use common::{GenerationMode, NoSpecialGenerationMode};
use inputs::common::{
    permute_1_2_4_3, permute_1_3_2, permute_1_3_4_2, permute_2_1, permute_2_1_3, reshape_2_1_to_3,
    reshape_2_2_to_4, reshape_3_1_to_4, reshape_3_3_3_to_9, reshape_4_4_4_to_12,
};
use itertools::Itertools;
use malachite_base::bools::exhaustive::exhaustive_bools;
use malachite_base::num::arithmetic::traits::{
    ArithmeticCheckedShl, ArithmeticCheckedShr, CheckedNeg, EqMod, ModPowerOf2, Parity, PowerOf2,
    RoundToMultiple, UnsignedAbs,
};
use malachite_base::num::basic::integers::PrimitiveInt;
use malachite_base::num::basic::signeds::PrimitiveSigned;
use malachite_base::num::basic::traits::Zero;
use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base::num::conversion::traits::{
    CheckedFrom, ConvertibleFrom, ExactFrom, WrappingFrom,
};
use malachite_base::num::exhaustive::{
    exhaustive_natural_signeds, exhaustive_negative_signeds, exhaustive_nonzero_signeds,
    exhaustive_positive_primitive_ints, exhaustive_signeds, exhaustive_unsigneds,
};
use malachite_base::num::exhaustive::{
    exhaustive_signed_range, primitive_int_increasing_inclusive_range,
    primitive_int_increasing_range,
};
use malachite_base::num::logic::traits::{BitAccess, BitConvertible, BitIterable, LeadingZeros};
use malachite_base::rounding_modes::exhaustive::exhaustive_rounding_modes;
use malachite_base::rounding_modes::RoundingMode;
use malachite_base::slices::slice_test_zero;
use malachite_base::tuples::exhaustive::{
    exhaustive_octuples_from_single, exhaustive_pairs, exhaustive_pairs_from_single,
    exhaustive_quadruples, exhaustive_quadruples_from_single, exhaustive_quintuples,
    exhaustive_sextuples_from_single, exhaustive_triples, exhaustive_triples_from_single,
    lex_pairs,
};
use malachite_base::vecs::exhaustive::{
    exhaustive_vecs, exhaustive_vecs_min_length, shortlex_vecs,
};
use malachite_base_test_util::generators::common::{reshape_1_2_to_3, It};
use malachite_base_test_util::generators::{exhaustive_pairs_big_small, exhaustive_pairs_big_tiny};
use malachite_nz::integer::logic::bit_access::limbs_vec_clear_bit_neg;
use malachite_nz::integer::Integer;
use malachite_nz::natural::arithmetic::add::{
    limbs_vec_add_in_place_left, limbs_vec_add_limb_in_place,
};
use malachite_nz::natural::arithmetic::div_exact::{
    limbs_modular_invert_limb, limbs_modular_invert_scratch_len,
};
use malachite_nz::natural::arithmetic::div_mod::{
    _limbs_div_mod_barrett_is_len, _limbs_div_mod_barrett_scratch_len, limbs_invert_limb,
    limbs_two_limb_inverse_helper,
};
use malachite_nz::natural::arithmetic::eq_mod::{
    limbs_eq_limb_mod_limb, limbs_eq_limb_mod_ref_ref, limbs_eq_mod_limb_ref_ref,
    limbs_eq_mod_ref_ref_ref,
};
use malachite_nz::natural::arithmetic::mod_mul::_limbs_precompute_mod_mul_two_limbs;
use malachite_nz::natural::arithmetic::mod_power_of_2::limbs_slice_mod_power_of_2_in_place;
use malachite_nz::natural::arithmetic::mod_power_of_2_square::SQRLO_DC_THRESHOLD_LIMIT;
use malachite_nz::natural::arithmetic::mul::fft::*;
use malachite_nz::natural::arithmetic::mul::limb::{
    limbs_mul_limb, limbs_slice_mul_limb_in_place, limbs_vec_mul_limb_in_place,
};
use malachite_nz::natural::arithmetic::mul::limbs_mul;
use malachite_nz::natural::arithmetic::mul::mul_mod::*;
use malachite_nz::natural::arithmetic::square::{
    _limbs_square_to_out_toom_3_input_size_valid, _limbs_square_to_out_toom_4_input_size_valid,
    _limbs_square_to_out_toom_6_input_size_valid, _limbs_square_to_out_toom_8_input_size_valid,
};
use malachite_nz::natural::arithmetic::sub::{limbs_sub_in_place_left, limbs_sub_limb_in_place};
use malachite_nz::natural::Natural;
use malachite_nz::platform::{Limb, SQR_TOOM2_THRESHOLD};
use rand::distributions::range::SampleRange;
use rand::distributions::{IndependentSample, Range};
use rand::{IsaacRng, Rand, Rng, SeedableRng};
use rust_wheels::iterators::common::{scramble, EXAMPLE_SEED};
use rust_wheels::iterators::dependent_pairs::{dependent_pairs, random_dependent_pairs};
use rust_wheels::iterators::general::{random, random_from_vector};
use rust_wheels::iterators::integers_geometric::{
    i32s_geometric, range_up_geometric_u32, u32s_geometric,
};
use rust_wheels::iterators::primitive_ints::{
    random_natural_signed, random_negative_signed, random_nonzero_signed, random_positive_signed,
    random_positive_unsigned, random_range, random_range_down, random_range_up,
    special_random_natural_signed, special_random_negative_signed, special_random_nonzero_signed,
    special_random_positive_signed, special_random_positive_unsigned, special_random_signed,
    special_random_unsigned,
};
use rust_wheels::iterators::rounding_modes::random_rounding_modes;
use rust_wheels::iterators::tuples::{
    random_octuples_from_single, random_pairs, random_pairs_from_single, random_quadruples,
    random_quadruples_from_single, random_quintuples, random_sextuples_from_single, random_triples,
    random_triples_from_single,
};
use rust_wheels::iterators::vecs::{
    random_vecs, random_vecs_min_length, special_random_bool_vecs, special_random_unsigned_vecs,
    special_random_unsigned_vecs_min_length,
};
use std::cmp::max;
use std::ops::{Shl, Shr};

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

// All `T`s, where `T` is unsigned and its most-significant bit is set.
pub fn unsigneds_var_1<T: PrimitiveUnsigned + Rand>(gm: GenerationMode) -> It<T> {
    match gm {
        GenerationMode::Exhaustive => Box::new(primitive_int_increasing_inclusive_range(
            T::power_of_2(T::WIDTH - 1),
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

// All `T`s, where `T` is signed and the square of the `T` is representable.
pub fn signeds_var_2<T: PrimitiveSigned + Rand + SampleRange>(
    gm: NoSpecialGenerationMode,
) -> It<T> {
    let max = T::power_of_2(T::WIDTH >> 1) - T::ONE;
    let xs: It<T> = match gm {
        NoSpecialGenerationMode::Exhaustive => {
            Box::new(exhaustive_signed_range(-max, max + T::ONE))
        }
        NoSpecialGenerationMode::Random(_) => Box::new(random_range(&EXAMPLE_SEED, -max, max)),
    };
    Box::new(xs.filter(|&x| x.checked_square().is_some()))
}

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
            primitive_int_increasing_inclusive_range(T::power_of_2(T::WIDTH - 1), T::MAX),
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
    let max = T::power_of_2(T::WIDTH >> 1);
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

// All triples of unsigned `T` where the first `T` is smaller than the third.
pub fn triples_of_unsigneds_var_2<T: PrimitiveUnsigned + Rand>(
    gm: GenerationMode,
) -> It<(T, T, T)> {
    Box::new(triples_of_unsigneds(gm).filter(|&(x_1, _, y)| x_1 < y))
}

fn add_mul_inputs_valid<T: PrimitiveInt>(x: T, y: T, z: T) -> bool {
    x.checked_add_mul(y, z).is_some()
}

struct ValidAddMulInputs<T> {
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

struct ValidSubMulInputs<T> {
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

// All `Limb`s smaller than 2<sup>Limb::WIDTH - 1<sup>.
fn limbs_range_2(gm: GenerationMode) -> It<Limb> {
    let upper = Limb::power_of_2(Limb::WIDTH - 1);
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

fn triples_of_unsigned_unsigned_and_unsigned<
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
        GenerationMode::Random(_) => {
            Box::new(random_triples(&EXAMPLE_SEED, &random, &random, &random))
        }
        GenerationMode::SpecialRandom(_) => Box::new(random_triples(
            &EXAMPLE_SEED,
            &special_random_unsigned,
            &special_random_unsigned,
            &special_random_unsigned,
        )),
    }
}

fn triples_of_unsigned_signed_and_unsigned<
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
        GenerationMode::Random(_) => {
            Box::new(random_triples(&EXAMPLE_SEED, &random, &random, &random))
        }
        GenerationMode::SpecialRandom(_) => Box::new(random_triples(
            &EXAMPLE_SEED,
            &special_random_unsigned,
            &special_random_signed,
            &special_random_unsigned,
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
            &random,
            &random_positive_unsigned,
        )),
        GenerationMode::SpecialRandom(_) => Box::new(random_pairs(
            &EXAMPLE_SEED,
            &special_random_unsigned,
            &special_random_positive_unsigned,
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
        GenerationMode::Random(_) => {
            Box::new(random_pairs(&EXAMPLE_SEED, &random, &random_nonzero_signed))
        }
        GenerationMode::SpecialRandom(_) => Box::new(random_pairs(
            &EXAMPLE_SEED,
            &special_random_signed,
            &special_random_nonzero_signed,
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
    Box::new(exhaustive_pairs_big_small(
        exhaustive_unsigneds(),
        exhaustive_unsigneds(),
    ))
}

fn sqrt_pairs_of_unsigned_and_signed<T: PrimitiveUnsigned, U: PrimitiveSigned>() -> It<(T, U)> {
    Box::new(exhaustive_pairs_big_small(
        exhaustive_unsigneds(),
        exhaustive_signeds(),
    ))
}

fn sqrt_pairs_of_signed_and_unsigned<T: PrimitiveSigned, U: PrimitiveUnsigned>() -> It<(T, U)> {
    Box::new(exhaustive_pairs_big_small(
        exhaustive_signeds(),
        exhaustive_unsigneds(),
    ))
}

fn random_pairs_of_primitive_and_geometric<T: PrimitiveInt + Rand, U: PrimitiveInt>(
    scale: u32,
) -> It<(T, U)> {
    Box::new(random_pairs(
        &EXAMPLE_SEED,
        &random,
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
            &special_random_unsigned,
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
            .filter(|&(n, u)| !n.divisible_by_power_of_2(u.exact_into())),
    )
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
                &special_random_unsigned,
                &(|seed| u32s_geometric(seed, scale).map(u64::from)),
            )
            .filter(|&(x, y)| x == T::ZERO || y < T::WIDTH),
        ),
    }
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

fn sqrt_pairs_of_positive_primitive_and_unsigned<T: PrimitiveInt, U: PrimitiveUnsigned + Rand>(
) -> It<(T, U)> {
    Box::new(exhaustive_pairs_big_small(
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
            &random_positive_unsigned,
            &(|seed| u32s_geometric(seed, scale).flat_map(U::checked_from)),
        )),
        GenerationMode::SpecialRandom(scale) => Box::new(random_pairs(
            &EXAMPLE_SEED,
            &special_random_positive_unsigned,
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
            &random_positive_signed,
            &(|seed| u32s_geometric(seed, scale).flat_map(U::checked_from)),
        )),
        GenerationMode::SpecialRandom(scale) => Box::new(random_pairs(
            &EXAMPLE_SEED,
            &special_random_positive_signed,
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
            &special_random_signed,
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
            .filter(|&(ref n, u)| !n.divisible_by_power_of_2(u.exact_into())),
    )
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

fn triples_of_unsigned_small_unsigned_and_small_unsigned<
    T: PrimitiveUnsigned + Rand,
    U: PrimitiveUnsigned + Rand,
>(
    gm: GenerationMode,
) -> It<(T, U, U)> {
    match gm {
        GenerationMode::Exhaustive => reshape_1_2_to_3(Box::new(exhaustive_pairs_big_small(
            exhaustive_unsigneds(),
            exhaustive_pairs_from_single(exhaustive_unsigneds()),
        ))),
        GenerationMode::Random(scale) => Box::new(random_triples(
            &EXAMPLE_SEED,
            &random,
            &(|seed| u32s_geometric(seed, scale).flat_map(U::checked_from)),
            &(|seed| u32s_geometric(seed, scale).flat_map(U::checked_from)),
        )),
        GenerationMode::SpecialRandom(scale) => Box::new(random_triples(
            &EXAMPLE_SEED,
            &special_random_unsigned,
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
        GenerationMode::Exhaustive => reshape_1_2_to_3(Box::new(exhaustive_pairs_big_small(
            exhaustive_signeds(),
            exhaustive_pairs_from_single(exhaustive_unsigneds()),
        ))),
        GenerationMode::Random(scale) => Box::new(random_triples(
            &EXAMPLE_SEED,
            &random,
            &(|seed| u32s_geometric(seed, scale).flat_map(U::checked_from)),
            &(|seed| u32s_geometric(seed, scale).flat_map(U::checked_from)),
        )),
        GenerationMode::SpecialRandom(scale) => Box::new(random_triples(
            &EXAMPLE_SEED,
            &special_random_signed,
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
        GenerationMode::Exhaustive => reshape_1_2_to_3(Box::new(exhaustive_pairs_big_small(
            exhaustive_positive_primitive_ints(),
            exhaustive_pairs_from_single(exhaustive_unsigneds()),
        ))),
        GenerationMode::Random(scale) => Box::new(random_triples(
            &EXAMPLE_SEED,
            &random_positive_unsigned,
            &(|seed| u32s_geometric(seed, scale).flat_map(U::checked_from)),
            &(|seed| u32s_geometric(seed, scale).flat_map(U::checked_from)),
        )),
        GenerationMode::SpecialRandom(scale) => Box::new(random_triples(
            &EXAMPLE_SEED,
            &special_random_positive_unsigned,
            &(|seed| u32s_geometric(seed, scale).flat_map(U::checked_from)),
            &(|seed| u32s_geometric(seed, scale).flat_map(U::checked_from)),
        )),
    };
    Box::new(ts.filter(|&(_, start, end)| start <= end))
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
        GenerationMode::Exhaustive => Box::new(exhaustive_pairs_big_small(
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
        GenerationMode::Exhaustive => {
            Box::new(reshape_2_1_to_3(Box::new(exhaustive_pairs_big_small(
                exhaustive_pairs_from_single(exhaustive_signeds()),
                exhaustive_unsigneds(),
            ))))
        }
        GenerationMode::Random(scale) => Box::new(random_triples(
            &EXAMPLE_SEED,
            &random,
            &random,
            &(|seed| u32s_geometric(seed, scale).flat_map(U::checked_from)),
        )),
        GenerationMode::SpecialRandom(scale) => Box::new(random_triples(
            &EXAMPLE_SEED,
            &special_random_signed,
            &special_random_signed,
            &(|seed| u32s_geometric(seed, scale).flat_map(U::checked_from)),
        )),
    }
}

type ItR<T> = It<(T, RoundingMode)>;

fn random_pairs_of_primitive_and_rounding_mode<T: PrimitiveInt + Rand>() -> ItR<T> {
    Box::new(random_pairs(&EXAMPLE_SEED, &random, &random_rounding_modes))
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
            &special_random_unsigned,
            &random_rounding_modes,
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
            &special_random_signed,
            &random_rounding_modes,
        )),
    }
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
        GenerationMode::Exhaustive => Box::new(exhaustive_pairs_big_small(
            filtered_quads(&EXAMPLE_SEED),
            exhaustive_unsigneds(),
        )),
        GenerationMode::Random(_) => {
            Box::new(random_pairs(&EXAMPLE_SEED, &filtered_quads, &random))
        }
        GenerationMode::SpecialRandom(_) => Box::new(random_pairs(
            &EXAMPLE_SEED,
            &filtered_quads,
            &special_random_unsigned,
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
        GenerationMode::Random(scale) => Box::new(random_vecs(seed, scale, &random)),
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
            &random,
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

// All pairs of `Vec<T>`, where `T` is unsigned and the first `Vec` is at least as long as the
// second.
pub fn pairs_of_unsigned_vec_var_3<T: PrimitiveUnsigned + Rand>(
    gm: GenerationMode,
) -> It<(Vec<T>, Vec<T>)> {
    Box::new(pairs_of_unsigned_vec(gm).filter(|&(ref xs, ref ys)| xs.len() >= ys.len()))
}

// All pairs of `Vec<T>`, where `T` is unsigned, the first `Vec` is at least as long as the second,
// and neither `Vec` is empty.
pub fn pairs_of_unsigned_vec_var_4<T: PrimitiveUnsigned + Rand>(
    gm: GenerationMode,
) -> It<(Vec<T>, Vec<T>)> {
    Box::new(
        pairs_of_unsigned_vec(gm)
            .filter(|&(ref xs, ref ys)| !ys.is_empty() && xs.len() >= ys.len()),
    )
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
            random_vecs_min_length(&EXAMPLE_SEED, scale, 2, &random),
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
            &(|seed| random_vecs_min_length(seed, scale, 2, &random)),
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
            random_vecs_min_length(&EXAMPLE_SEED, scale, 2, &random),
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

// All pairs of `Vec<T>`, where `T` is unsigned and `out` and `xs` are valid inputs to
// `_limbs_square_low_basecase`.
pub fn pairs_of_unsigned_vec_var_26<T: PrimitiveUnsigned + Rand>(
    gm: GenerationMode,
) -> It<(Vec<T>, Vec<T>)> {
    Box::new(pairs_of_unsigned_vec(gm).filter(|&(ref xs, ref ys)| {
        !ys.is_empty() && ys.len() <= SQRLO_DC_THRESHOLD_LIMIT && xs.len() >= ys.len()
    }))
}

// All pairs of `Vec<T>`, where `T` is unsigned, the second `Vec` has at least 2 elements, and the
// first `Vec` is at least as long as the second.
pub fn pairs_of_unsigned_vec_var_27<T: PrimitiveUnsigned + Rand>(
    gm: GenerationMode,
) -> It<(Vec<T>, Vec<T>)> {
    Box::new(
        pairs_of_unsigned_vec_min_sizes(gm, 2, 2)
            .filter(|&(ref out, ref xs)| out.len() >= xs.len()),
    )
}

// All pairs of `Vec<T>` that meet the preconditions for `limbs_pow_low`.
pub fn pairs_of_unsigned_vec_var_28<T: PrimitiveUnsigned + Rand>(
    gm: GenerationMode,
) -> It<(Vec<T>, Vec<T>)> {
    Box::new(pairs_of_unsigned_vec(gm).filter(|&(ref xs, ref es)| {
        !xs.is_empty()
            && (es.len() > 1 || es.len() == 1 && es[0] > T::ONE)
            && *es.last().unwrap() != T::ZERO
    }))
}

fn pairs_of_unsigned_vec_and_bool<T: PrimitiveUnsigned + Rand>(
    gm: GenerationMode,
) -> It<(Vec<T>, bool)> {
    match gm {
        GenerationMode::Exhaustive => Box::new(exhaustive_pairs_big_tiny(
            exhaustive_vecs(exhaustive_unsigneds()),
            exhaustive_bools(),
        )),
        GenerationMode::Random(scale) => Box::new(random_pairs(
            &EXAMPLE_SEED,
            &(|seed| random_vecs(seed, scale, &random)),
            &random,
        )),
        GenerationMode::SpecialRandom(scale) => Box::new(random_pairs(
            &EXAMPLE_SEED,
            &(|seed| special_random_unsigned_vecs(seed, scale)),
            &random,
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
            primitive_int_increasing_inclusive_range(Limb::power_of_2(Limb::WIDTH - 1), Limb::MAX),
        )),
        GenerationMode::Random(scale) => Box::new(random_triples(
            &EXAMPLE_SEED,
            &(|seed| random_vecs_min_length(seed, scale, 3, &random)),
            &(|seed| random_vecs_min_length(seed, scale, 2, &random)),
            &(|seed| random_range_up(seed, Limb::power_of_2(Limb::WIDTH - 1))),
        )),
        GenerationMode::SpecialRandom(scale) => Box::new(random_triples(
            &EXAMPLE_SEED,
            &(|seed| special_random_unsigned_vecs_min_length(seed, scale, 3)),
            &(|seed| special_random_unsigned_vecs_min_length(seed, scale, 2)),
            &(|seed| random_range_up(seed, Limb::power_of_2(Limb::WIDTH - 1))),
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
            &random,
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
            &random,
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
    Box::new(quadruples_of_unsigned_vec_var_2(gm).filter(|&(_, _, _, ref ms)| ms[0].odd()))
}

// All quadruples of `Vec<Limb>` that are valid inputs to `limbs_mod_pow`.
pub fn quadruples_of_unsigned_vec_var_2(
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
        GenerationMode::Exhaustive => Box::new(exhaustive_pairs_big_small(
            pairs_of_unsigned_vec_var_1(gm),
            exhaustive_vecs(exhaustive_unsigneds()),
        )),
        GenerationMode::Random(scale) => Box::new(random_pairs(
            &EXAMPLE_SEED,
            &(|seed| pairs_of_unsigned_vec_var_1_with_seed(gm, seed)),
            &(|seed| random_vecs(seed, scale, &random)),
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
            &(|seed| random_vecs(seed, scale, &random)),
            &(|seed| random_vecs_min_length(seed, scale, 2, &random)),
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
            primitive_int_increasing_inclusive_range(Limb::power_of_2(Limb::WIDTH - 1), Limb::MAX),
        )),
        GenerationMode::Random(scale) => Box::new(random_quadruples(
            &EXAMPLE_SEED,
            &(|seed| random_vecs(seed, scale, &random)),
            &(|seed| random_vecs_min_length(seed, scale, 2, &random)),
            &(|seed| random_vecs(seed, scale, &random)),
            &(|seed| random_range_up(seed, Limb::power_of_2(Limb::WIDTH - 1))),
        )),
        GenerationMode::SpecialRandom(scale) => Box::new(random_quadruples(
            &EXAMPLE_SEED,
            &(|seed| special_random_unsigned_vecs(seed, scale)),
            &(|seed| special_random_unsigned_vecs_min_length(seed, scale, 2)),
            &(|seed| special_random_unsigned_vecs(seed, scale)),
            &(|seed| random_range_up(seed, Limb::power_of_2(Limb::WIDTH - 1))),
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
            primitive_int_increasing_inclusive_range(Limb::power_of_2(Limb::WIDTH - 1), Limb::MAX),
        )),
        GenerationMode::Random(scale) => Box::new(random_quadruples(
            &EXAMPLE_SEED,
            &(|seed| random_vecs(seed, scale, &random)),
            &(|seed| random_vecs_min_length(seed, scale, 10, &random)),
            &(|seed| random_vecs_min_length(seed, scale, 4, &random)),
            &(|seed| random_range_up(seed, Limb::power_of_2(Limb::WIDTH - 1))),
        )),
        GenerationMode::SpecialRandom(scale) => Box::new(random_quadruples(
            &EXAMPLE_SEED,
            &(|seed| special_random_unsigned_vecs(seed, scale)),
            &(|seed| special_random_unsigned_vecs_min_length(seed, scale, 10)),
            &(|seed| special_random_unsigned_vecs_min_length(seed, scale, 4)),
            &(|seed| random_range_up(seed, Limb::power_of_2(Limb::WIDTH - 1))),
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
            primitive_int_increasing_inclusive_range(Limb::power_of_2(Limb::WIDTH - 1), Limb::MAX),
        )),
        GenerationMode::Random(scale) => Box::new(random_quadruples(
            &EXAMPLE_SEED,
            &(|seed| random_vecs_min_length(seed, scale, 3, &random)),
            &(|seed| random_vecs_min_length(seed, scale, 9, &random)),
            &(|seed| random_vecs_min_length(seed, scale, 5, &random)),
            &(|seed| random_range_up(seed, Limb::power_of_2(Limb::WIDTH - 1))),
        )),
        GenerationMode::SpecialRandom(scale) => Box::new(random_quadruples(
            &EXAMPLE_SEED,
            &(|seed| special_random_unsigned_vecs_min_length(seed, scale, 3)),
            &(|seed| special_random_unsigned_vecs_min_length(seed, scale, 9)),
            &(|seed| special_random_unsigned_vecs_min_length(seed, scale, 5)),
            &(|seed| random_range_up(seed, Limb::power_of_2(Limb::WIDTH - 1))),
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
            primitive_int_increasing_inclusive_range(Limb::power_of_2(Limb::WIDTH - 1), Limb::MAX),
        )),
        GenerationMode::Random(scale) => Box::new(random_quadruples(
            &EXAMPLE_SEED,
            &(|seed| random_vecs(seed, scale, &random)),
            &(|seed| random_vecs_min_length(seed, scale, 2, &random)),
            &(|seed| random_vecs_min_length(seed, scale, 1, &random)),
            &(|seed| random_range_up(seed, Limb::power_of_2(Limb::WIDTH - 1))),
        )),
        GenerationMode::SpecialRandom(scale) => Box::new(random_quadruples(
            &EXAMPLE_SEED,
            &(|seed| special_random_unsigned_vecs(seed, scale)),
            &(|seed| special_random_unsigned_vecs_min_length(seed, scale, 2)),
            &(|seed| special_random_unsigned_vecs_min_length(seed, scale, 1)),
            &(|seed| random_range_up(seed, Limb::power_of_2(Limb::WIDTH - 1))),
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
            primitive_int_increasing_inclusive_range(Limb::power_of_2(Limb::WIDTH - 1), Limb::MAX),
        )),
        GenerationMode::Random(scale) => Box::new(random_quadruples(
            &EXAMPLE_SEED,
            &(|seed| random_vecs(seed, scale, &random)),
            &(|seed| random_vecs_min_length(seed, scale, 3, &random)),
            &(|seed| random_vecs_min_length(seed, scale, 1, &random)),
            &(|seed| random_range_up(seed, Limb::power_of_2(Limb::WIDTH - 1))),
        )),
        GenerationMode::SpecialRandom(scale) => Box::new(random_quadruples(
            &EXAMPLE_SEED,
            &(|seed| special_random_unsigned_vecs(seed, scale)),
            &(|seed| special_random_unsigned_vecs_min_length(seed, scale, 3)),
            &(|seed| special_random_unsigned_vecs_min_length(seed, scale, 1)),
            &(|seed| random_range_up(seed, Limb::power_of_2(Limb::WIDTH - 1))),
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
            &(|seed| random_vecs_min_length(seed, scale, 1, &random)),
            &(|seed| random_vecs_min_length(seed, scale, 2, &random)),
            &(|seed| random_vecs_min_length(seed, scale, 2, &random)),
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
            random_vecs_min_length(&EXAMPLE_SEED, scale, 2, &random),
        )),
        GenerationMode::SpecialRandom(scale) => Box::new(random_triples_from_single(
            special_random_unsigned_vecs_min_length(&EXAMPLE_SEED, scale, 2),
        )),
    };
    Box::new(
        ts.filter(|(r, n, d)| *d.last().unwrap() != 0 && n.len() >= d.len() && r.len() >= d.len()),
    )
}

// All triples of `Vec<Limb>`, where `qs`, `ns`, and `ds` meet the preconditions of
// `_limbs_modular_div_barrett`.
pub fn triples_of_limb_vec_var_50(gm: GenerationMode) -> It<(Vec<Limb>, Vec<Limb>, Vec<Limb>)> {
    let ts: It<(Vec<Limb>, Vec<Limb>, Vec<Limb>)> = match gm {
        GenerationMode::Exhaustive => Box::new(exhaustive_triples_from_single(
            exhaustive_vecs_min_length(2, exhaustive_unsigneds()),
        )),
        GenerationMode::Random(scale) => Box::new(random_triples_from_single(
            random_vecs_min_length(&EXAMPLE_SEED, scale, 2, &random),
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
            random_vecs_min_length(&EXAMPLE_SEED, scale, 1, &random),
        )),
        GenerationMode::SpecialRandom(scale) => Box::new(random_triples_from_single(
            special_random_unsigned_vecs_min_length(&EXAMPLE_SEED, scale, 1),
        )),
    };
    Box::new(ts.filter(|(q, n, d)| q.len() >= n.len() && n.len() >= d.len() && d[0].odd()))
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
            random_vecs_min_length(&EXAMPLE_SEED, scale, 2, &random)
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
            &(|seed| random_vecs_min_length(seed, scale, min_xs_len, &random)),
            &(|seed| random_vecs_min_length(seed, scale, min_ys_len, &random)),
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

fn pairs_of_unsigned_vec_min_sizes_2<T: PrimitiveUnsigned + Rand>(
    gm: GenerationMode,
    min_len: u64,
) -> It<(Vec<T>, Vec<T>)> {
    match gm {
        GenerationMode::Exhaustive => Box::new(exhaustive_pairs_from_single(
            exhaustive_vecs_min_length(min_len, exhaustive_unsigneds()),
        )),
        GenerationMode::Random(scale) => Box::new(random_pairs_from_single(
            random_vecs_min_length(&EXAMPLE_SEED, scale, min_len, &random),
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
            &(|seed| random_vecs_min_length(seed, scale, min_xs_len, &random)),
            &(|seed| random_vecs_min_length(seed, scale, min_ys_len, &random)),
            &(|seed| random_vecs_min_length(seed, scale, min_zs_len, &random)),
        )),
        GenerationMode::SpecialRandom(scale) => Box::new(random_triples(
            &EXAMPLE_SEED,
            &(|seed| special_random_unsigned_vecs_min_length(seed, scale, min_xs_len)),
            &(|seed| special_random_unsigned_vecs_min_length(seed, scale, min_ys_len)),
            &(|seed| special_random_unsigned_vecs_min_length(seed, scale, min_zs_len)),
        )),
    }
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
            random_vecs_min_length(&EXAMPLE_SEED, scale, min_len, &random),
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
            &(|seed| random_vecs(seed, scale, &random)),
            &(|seed| random_vecs(seed, scale, &random)),
            &(|seed| random_vecs(seed, scale, &random)),
            &random,
        )),
        GenerationMode::SpecialRandom(scale) => Box::new(random_quadruples(
            &EXAMPLE_SEED,
            &(|seed| special_random_unsigned_vecs(seed, scale)),
            &(|seed| special_random_unsigned_vecs(seed, scale)),
            &(|seed| special_random_unsigned_vecs(seed, scale)),
            &random,
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
            primitive_int_increasing_inclusive_range(Limb::power_of_2(Limb::WIDTH - 1), Limb::MAX),
        )),
        GenerationMode::Random(scale) => Box::new(random_quintuples(
            &EXAMPLE_SEED,
            &(|seed| random_vecs_min_length(seed, scale, 1, &random)),
            &(|seed| random_vecs_min_length(seed, scale, 2, &random)),
            &(|seed| random_vecs_min_length(seed, scale, 3, &random)),
            &(|seed| random_vecs_min_length(seed, scale, 1, &random)),
            &(|seed| random_range_up(seed, Limb::power_of_2(Limb::WIDTH - 1))),
        )),
        GenerationMode::SpecialRandom(scale) => Box::new(random_quintuples(
            &EXAMPLE_SEED,
            &(|seed| special_random_unsigned_vecs_min_length(seed, scale, 1)),
            &(|seed| special_random_unsigned_vecs_min_length(seed, scale, 2)),
            &(|seed| special_random_unsigned_vecs_min_length(seed, scale, 3)),
            &(|seed| special_random_unsigned_vecs_min_length(seed, scale, 1)),
            &(|seed| random_range_up(seed, Limb::power_of_2(Limb::WIDTH - 1))),
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
            &(|seed| random_vecs_min_length(seed, scale, 1, &random)),
            &(|seed| random_vecs_min_length(seed, scale, 2, &random)),
            &(|seed| random_vecs_min_length(seed, scale, 2, &random)),
            &(|seed| random_vecs_min_length(seed, scale, 2, &random)),
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
            &(|seed| random_vecs_min_length(seed, scale, 2, &random)),
            &(|seed| random_vecs_min_length(seed, scale, 2, &random)),
            &(|seed| random_vecs_min_length(seed, scale, 4, &random)),
            &(|seed| random_vecs_min_length(seed, scale, 2, &random)),
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
            &(|seed| random_vecs_min_length(seed, scale, 4, &random)),
            &(|seed| random_vecs_min_length(seed, scale, 2, &random)),
            &(|seed| random_vecs_min_length(seed, scale, 4, &random)),
            &(|seed| random_vecs_min_length(seed, scale, 2, &random)),
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
            &(|seed| random_vecs_min_length(seed, scale, 2, &random)),
            &(|seed| random_vecs(seed, scale, &random)),
            &(|seed| random_vecs(seed, scale, &random)),
            &(|seed| random_vecs(seed, scale, &random)),
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

// All quadruples of `Vec<Limb>`, `Vec<Limb>`, `Limb`, and `Limb` where the first limb is a divisor
// of `Limb::MAX`.
fn quadruples_of_limb_vec_limb_vec_limb_and_limb_var_2(
    gm: GenerationMode,
) -> It<(Vec<Limb>, Vec<Limb>, Limb, Limb)> {
    match gm {
        GenerationMode::Exhaustive => {
            permute_1_2_4_3(reshape_3_1_to_4(Box::new(exhaustive_pairs_big_small(
                exhaustive_triples(
                    exhaustive_vecs(exhaustive_unsigneds()),
                    exhaustive_vecs(exhaustive_unsigneds()),
                    exhaustive_unsigneds(),
                ),
                factors_of_limb_max().into_iter(),
            ))))
        }
        GenerationMode::Random(scale) => Box::new(random_quadruples(
            &EXAMPLE_SEED,
            &(|seed| random_vecs(seed, scale, &random)),
            &(|seed| random_vecs(seed, scale, &random)),
            &(|seed| random_from_vector(seed, factors_of_limb_max())),
            &random,
        )),
        GenerationMode::SpecialRandom(scale) => Box::new(random_quadruples(
            &EXAMPLE_SEED,
            &(|seed| special_random_unsigned_vecs(seed, scale)),
            &(|seed| special_random_unsigned_vecs(seed, scale)),
            &(|seed| random_from_vector(seed, factors_of_limb_max())),
            &special_random_unsigned,
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
            primitive_int_increasing_inclusive_range(Limb::power_of_2(Limb::WIDTH - 1), Limb::MAX),
        )),
        GenerationMode::Random(scale) => Box::new(random_quadruples(
            &EXAMPLE_SEED,
            &(|seed| random_vecs(seed, scale, &random)),
            &(|seed| random_vecs_min_length(seed, scale, 3, &random)),
            &(|seed| random_vecs_min_length(seed, scale, 2, &random)),
            &(|seed| random_range_up(seed, Limb::power_of_2(Limb::WIDTH - 1))),
        )),
        GenerationMode::SpecialRandom(scale) => Box::new(random_quadruples(
            &EXAMPLE_SEED,
            &(|seed| special_random_unsigned_vecs(seed, scale)),
            &(|seed| special_random_unsigned_vecs_min_length(seed, scale, 3)),
            &(|seed| special_random_unsigned_vecs_min_length(seed, scale, 2)),
            &(|seed| random_range_up(seed, Limb::power_of_2(Limb::WIDTH - 1))),
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
            primitive_int_increasing_inclusive_range(Limb::power_of_2(Limb::WIDTH - 1), Limb::MAX),
        )),
        GenerationMode::Random(scale) => Box::new(random_quadruples(
            &EXAMPLE_SEED,
            &(|seed| random_vecs_min_length(seed, scale, 3, &random)),
            &(|seed| random_vecs_min_length(seed, scale, 9, &random)),
            &(|seed| random_vecs_min_length(seed, scale, 5, &random)),
            &(|seed| random_range_up(seed, Limb::power_of_2(Limb::WIDTH - 1))),
        )),
        GenerationMode::SpecialRandom(scale) => Box::new(random_quadruples(
            &EXAMPLE_SEED,
            &(|seed| special_random_unsigned_vecs_min_length(seed, scale, 3)),
            &(|seed| special_random_unsigned_vecs_min_length(seed, scale, 9)),
            &(|seed| special_random_unsigned_vecs_min_length(seed, scale, 5)),
            &(|seed| random_range_up(seed, Limb::power_of_2(Limb::WIDTH - 1))),
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
            random_vecs_min_length(&EXAMPLE_SEED, scale, 1, &random),
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
            &(|seed| random_vecs_min_length(seed, scale, 2, &random)),
            &(|seed| random_vecs_min_length(seed, scale, 2, &random)),
            &(|seed| random_vecs_min_length(seed, scale, 1, &random)),
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
            &(|seed| random_vecs_min_length(seed, scale, 3, &random)),
            &(|seed| random_vecs_min_length(seed, scale, 3, &random)),
            &(|seed| random_vecs_min_length(seed, scale, 2, &random)),
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
            random_vecs_min_length(&EXAMPLE_SEED, scale, 2, &random),
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
            GenerationMode::Random(scale) => {
                Box::new(random_vecs_min_length(&EXAMPLE_SEED, scale, 1, &random))
            }
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
        GenerationMode::Exhaustive => {
            permute_1_3_2(reshape_2_1_to_3(Box::new(exhaustive_pairs_big_small(
                exhaustive_pairs(
                    exhaustive_vecs(exhaustive_unsigneds()),
                    exhaustive_unsigneds(),
                ),
                factors_of_limb_max().into_iter(),
            ))))
        }
        GenerationMode::Random(scale) => Box::new(random_triples(
            &EXAMPLE_SEED,
            &(|seed| random_vecs(seed, scale, &random)),
            &(|seed| random_from_vector(seed, factors_of_limb_max())),
            &random,
        )),
        GenerationMode::SpecialRandom(scale) => Box::new(random_triples(
            &EXAMPLE_SEED,
            &(|seed| special_random_unsigned_vecs(seed, scale)),
            &(|seed| random_from_vector(seed, factors_of_limb_max())),
            &special_random_unsigned,
        )),
    }
}

fn exhaustive_pairs_of_unsigned_vec_and_unsigned<T: PrimitiveUnsigned + Rand>() -> It<(Vec<T>, T)> {
    Box::new(exhaustive_pairs(
        exhaustive_vecs(exhaustive_unsigneds()),
        exhaustive_unsigneds(),
    ))
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
            &(|seed| random_vecs(seed, scale, &random)),
            &random,
        )),
        GenerationMode::SpecialRandom(scale) => Box::new(random_pairs(
            &EXAMPLE_SEED,
            &(|seed| special_random_unsigned_vecs(seed, scale)),
            &special_random_unsigned,
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
            primitive_int_increasing_inclusive_range(T::power_of_2(T::WIDTH - 1), T::MAX),
        )),
        GenerationMode::Random(scale) => Box::new(random_pairs(
            &EXAMPLE_SEED,
            &(|seed| random_vecs_min_length(seed, scale, 2, &random)),
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
            primitive_int_increasing_inclusive_range(T::power_of_2(T::WIDTH - 1), T::MAX),
        )),
        GenerationMode::Random(scale) => Box::new(random_pairs(
            &EXAMPLE_SEED,
            &(|seed| random_vecs_min_length(seed, scale, 1, &random)),
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
            &(|seed| random_vecs_min_length(seed, scale, 1, &random)),
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
            &(|seed| random_vecs_min_length(seed, scale, 1, &random)),
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
            &(|seed| random_vecs(seed, scale, &random)),
            &random_positive_unsigned,
        )),
        GenerationMode::SpecialRandom(scale) => Box::new(random_pairs(
            &EXAMPLE_SEED,
            &(|seed| special_random_unsigned_vecs(seed, scale)),
            &special_random_positive_unsigned,
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
            &(|seed| random_vecs_min_length(seed, scale, 2, &random)),
            &random_positive_unsigned,
        )),
        GenerationMode::SpecialRandom(scale) => Box::new(random_pairs(
            &EXAMPLE_SEED,
            &(|seed| special_random_unsigned_vecs_min_length(seed, scale, 2)),
            &special_random_positive_unsigned,
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
            &(|seed| random_vecs_min_length(seed, scale, 2, &random)),
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
            &(|seed| random_vecs(seed, scale, &random)),
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
        GenerationMode::Exhaustive => Box::new(exhaustive_pairs_big_tiny(
            exhaustive_vecs(exhaustive_unsigneds()),
            exhaustive_unsigneds(),
        )),
        GenerationMode::Random(scale) => Box::new(random_pairs(
            &EXAMPLE_SEED,
            &(|seed| random_vecs(seed, scale, &random)),
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

// All pairs of `Vec<Limb>`, and `u64` such that the `Vec` is nonempty and the number of significant
// bits of the `Vec` does not exceed the `u64`.
pub fn pairs_of_unsigned_vec_and_small_unsigned_var_4(gm: GenerationMode) -> It<(Vec<Limb>, u64)> {
    if gm == GenerationMode::Exhaustive {
        let ps = pairs_of_unsigned_vec_and_small_unsigned::<Limb, u64>(gm).filter_map(
            |(mut xs, pow)| {
                if xs.is_empty() {
                    None
                } else {
                    limbs_slice_mod_power_of_2_in_place(&mut xs, pow);
                    if *xs.last().unwrap() == 0 {
                        None
                    } else {
                        Some((xs, pow))
                    }
                }
            },
        );
        Box::new(ps.unique())
    } else {
        let ps = pairs_of_unsigned_vec_and_small_unsigned::<Limb, u64>(gm).filter_map(
            |(mut xs, pow)| {
                if xs.is_empty() {
                    None
                } else {
                    limbs_slice_mod_power_of_2_in_place(&mut xs, pow);
                    if *xs.last().unwrap() == 0 {
                        None
                    } else {
                        Some((xs, pow))
                    }
                }
            },
        );
        Box::new(ps)
    }
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
            &(|seed| random_vecs_min_length(seed, scale, 1, &random)),
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
        GenerationMode::Exhaustive => reshape_1_2_to_3(Box::new(exhaustive_pairs_big_tiny(
            exhaustive_vecs(exhaustive_unsigneds()),
            exhaustive_pairs_from_single(exhaustive_unsigneds()),
        ))),
        GenerationMode::Random(scale) => Box::new(random_triples(
            &EXAMPLE_SEED,
            &(|seed| random_vecs(seed, scale, &random)),
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
        GenerationMode::Exhaustive => Box::new(shortlex_vecs(exhaustive_bools())),
        GenerationMode::Random(scale) => Box::new(random_vecs(&EXAMPLE_SEED, scale, &random)),
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
            exhaustive_vecs(exhaustive_unsigneds()),
            exhaustive_vecs(exhaustive_unsigneds()),
            exhaustive_unsigneds(),
        )),
        GenerationMode::Random(scale) => Box::new(random_triples(
            &EXAMPLE_SEED,
            &(|seed| random_vecs(seed, scale, &random)),
            &(|seed| random_vecs(seed, scale, &random)),
            &random,
        )),
        GenerationMode::SpecialRandom(scale) => Box::new(random_triples(
            &EXAMPLE_SEED,
            &(|seed| special_random_unsigned_vecs(seed, scale)),
            &(|seed| special_random_unsigned_vecs(seed, scale)),
            &special_random_unsigned,
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
            &(|seed| random_vecs(seed, scale, &random)),
            &(|seed| random_vecs(seed, scale, &random)),
            &random_positive_unsigned,
        )),
        GenerationMode::SpecialRandom(scale) => Box::new(random_triples(
            &EXAMPLE_SEED,
            &(|seed| special_random_unsigned_vecs(seed, scale)),
            &(|seed| special_random_unsigned_vecs(seed, scale)),
            &special_random_positive_unsigned,
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
            &(|seed| random_vecs(seed, scale, &random)),
            &(|seed| random_vecs(seed, scale, &random)),
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
        GenerationMode::Exhaustive => Box::new(exhaustive_pairs_big_small(
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
                    random_vecs_min_length(seed, scale, 2, &random)
                        .filter(|xs| *xs.last().unwrap() != T::ZERO),
                )
            }),
            &random_positive_unsigned,
        )),
        GenerationMode::SpecialRandom(scale) => Box::new(random_pairs(
            &EXAMPLE_SEED,
            &(|seed| {
                random_pairs_from_single(
                    special_random_unsigned_vecs_min_length(seed, scale, 2)
                        .filter(|xs| *xs.last().unwrap() != T::ZERO),
                )
            }),
            &special_random_positive_unsigned,
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
                limbs_slice_mod_power_of_2_in_place(&mut xs, pow);
                limbs_slice_mod_power_of_2_in_place(&mut ys, pow);
                (xs, ys, pow)
            },
        );
        Box::new(ts.unique())
    } else {
        let ts = triples_of_unsigned_vec_unsigned_vec_and_small_unsigned::<Limb, u64>(
            gm.with_scale(gm.get_scale().unwrap()),
        )
        .map(|(mut xs, mut ys, pow)| {
            limbs_slice_mod_power_of_2_in_place(&mut xs, pow);
            limbs_slice_mod_power_of_2_in_place(&mut ys, pow);
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

// All triples of `Vec<T>`, `Vec<T>`, and `u64` which meet the preconditions of
// `limbs_mod_power_of_2_pow`.
pub fn triples_of_unsigned_vec_unsigned_vec_and_unsigned_var_17(
    gm: GenerationMode,
) -> It<(Vec<Limb>, Vec<Limb>, u64)> {
    if gm == GenerationMode::Exhaustive {
        let ts = triples_of_unsigned_vec_unsigned_vec_and_small_unsigned::<Limb, u64>(gm)
            .filter_map(|(mut xs, es, pow)| {
                if xs.is_empty()
                    || es.is_empty()
                    || *es.last().unwrap() == 0
                    || es.len() == 1 && es[0] == 1
                {
                    None
                } else {
                    limbs_slice_mod_power_of_2_in_place(&mut xs, pow);
                    if *xs.last().unwrap() == 0 {
                        None
                    } else {
                        Some((xs, es, pow))
                    }
                }
            });
        Box::new(ts.unique())
    } else {
        let ts = triples_of_unsigned_vec_unsigned_vec_and_small_unsigned::<Limb, u64>(gm)
            .filter_map(|(mut xs, es, pow)| {
                if xs.is_empty()
                    || es.is_empty()
                    || *es.last().unwrap() == 0
                    || es.len() == 1 && es[0] == 1
                {
                    None
                } else {
                    limbs_slice_mod_power_of_2_in_place(&mut xs, pow);
                    if *xs.last().unwrap() == 0 {
                        None
                    } else {
                        Some((xs, es, pow))
                    }
                }
            });
        Box::new(ts)
    }
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
            &(|seed| random_vecs(seed, scale, &random)),
            &(|seed| random_vecs(seed, scale, &random)),
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

fn triples_of_unsigned_vec_unsigned_and_unsigned<T: PrimitiveUnsigned + Rand>(
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
            &(|seed| random_vecs(seed, scale, &random)),
            &random,
            &random,
        )),
        GenerationMode::SpecialRandom(scale) => Box::new(random_triples(
            &EXAMPLE_SEED,
            &(|seed| special_random_unsigned_vecs(seed, scale)),
            &special_random_unsigned,
            &special_random_unsigned,
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
            limbs_slice_mod_power_of_2_in_place(&mut xs, pow);
            (xs, y.mod_power_of_2(pow), pow)
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
                        &random,
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
            &random_positive_unsigned,
            &random_positive_unsigned,
            &(|seed| {
                random_vecs_min_length(seed, scale, 2, &random)
                    .filter(|limbs| *limbs.last().unwrap() != T::ZERO)
            }),
        )),
        GenerationMode::SpecialRandom(scale) => Box::new(random_triples(
            &EXAMPLE_SEED,
            &special_random_positive_unsigned,
            &special_random_positive_unsigned,
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
            &random,
            &random,
            &random_rounding_modes,
        )),
        GenerationMode::SpecialRandom(_) => Box::new(random_triples(
            &EXAMPLE_SEED,
            &special_random_unsigned,
            &special_random_unsigned,
            &random_rounding_modes,
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
            &random,
            &random,
            &random_rounding_modes,
        )),
        GenerationMode::SpecialRandom(_) => Box::new(random_triples(
            &EXAMPLE_SEED,
            &special_random_signed,
            &special_random_signed,
            &random_rounding_modes,
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
        GenerationMode::Exhaustive => Box::new(exhaustive_pairs_big_small(
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
                random_vecs_min_length(seed, scale, 2, &random)
                    .filter(|limbs| *limbs.last().unwrap() != T::ZERO)
            }),
            &random,
            &random_positive_unsigned,
        )),
        GenerationMode::SpecialRandom(scale) => Box::new(random_triples(
            &EXAMPLE_SEED,
            &(|seed| {
                special_random_unsigned_vecs_min_length(seed, scale, 2)
                    .filter(|limbs| *limbs.last().unwrap() != T::ZERO)
            }),
            &special_random_unsigned,
            &special_random_positive_unsigned,
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
                random_vecs_min_length(seed, scale, 1, &random)
                    .filter(|limbs| *limbs.last().unwrap() != T::ZERO)
            }),
            &random,
            &random_positive_unsigned,
        )),
        GenerationMode::SpecialRandom(scale) => Box::new(random_triples(
            &EXAMPLE_SEED,
            &(|seed| {
                special_random_unsigned_vecs_min_length(seed, scale, 1)
                    .filter(|limbs| *limbs.last().unwrap() != T::ZERO)
            }),
            &special_random_unsigned,
            &special_random_positive_unsigned,
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
            &(|seed| random_vecs(seed, scale, &random)),
            &random,
            &(|seed| u32s_geometric(seed, scale).flat_map(U::checked_from)),
        )),
        GenerationMode::SpecialRandom(scale) => Box::new(random_triples(
            &EXAMPLE_SEED,
            &(|seed| special_random_unsigned_vecs(seed, scale)),
            &special_random_unsigned,
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
                random_vecs(seed, scale, &random)
                    .filter(|limbs| !limbs.is_empty() && *limbs.last().unwrap() != T::ZERO)
            }),
            &random,
            &(|seed| u32s_geometric(seed, scale).flat_map(U::checked_from)),
        )),
        GenerationMode::SpecialRandom(scale) => Box::new(random_triples(
            &EXAMPLE_SEED,
            &(|seed| {
                special_random_unsigned_vecs(seed, scale)
                    .filter(|limbs| !limbs.is_empty() && *limbs.last().unwrap() != T::ZERO)
            }),
            &special_random_unsigned,
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
                random_vecs(seed, scale, &random)
                    .filter(|limbs| limbs.len() > 1 && *limbs.last().unwrap() != T::ZERO)
            }),
            &random,
            &(|seed| u32s_geometric(seed, scale).flat_map(U::checked_from)),
        )),
        GenerationMode::SpecialRandom(scale) => Box::new(random_triples(
            &EXAMPLE_SEED,
            &(|seed| {
                special_random_unsigned_vecs(seed, scale)
                    .filter(|limbs| limbs.len() > 1 && *limbs.last().unwrap() != T::ZERO)
            }),
            &special_random_unsigned,
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
            &(|seed| random_vecs(seed, scale, &random)),
            &(|seed| u32s_geometric(seed, scale).map(usize::wrapping_from)),
            &(|seed| random_vecs(seed, scale, &random)),
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

fn quadruples_of_unsigned_vec_small_unsigned_small_unsigned_and_unsigned_vec<
    T: PrimitiveUnsigned + Rand,
    U: PrimitiveUnsigned + Rand,
>(
    gm: GenerationMode,
) -> It<(Vec<T>, U, U, Vec<T>)> {
    permute_1_3_4_2(reshape_2_2_to_4(match gm {
        GenerationMode::Exhaustive => Box::new(exhaustive_pairs_big_small(
            exhaustive_pairs_from_single(exhaustive_vecs(exhaustive_unsigneds())),
            exhaustive_pairs_from_single(exhaustive_unsigneds()),
        )),
        GenerationMode::Random(scale) => Box::new(random_pairs(
            &EXAMPLE_SEED,
            &(|seed| random_pairs_from_single(random_vecs(seed, scale, &random))),
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
            &(|seed| random_vecs(seed, scale, &random)),
            &(|seed| u32s_geometric(seed, scale).flat_map(U::checked_from)),
            &random_rounding_modes,
        )),
        GenerationMode::SpecialRandom(scale) => Box::new(random_triples(
            &EXAMPLE_SEED,
            &(|seed| special_random_unsigned_vecs(seed, scale)),
            &(|seed| u32s_geometric(seed, scale).flat_map(U::checked_from)),
            &random_rounding_modes,
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
            &random,
            &(|seed| random_vecs(seed, scale, &random)),
            &random_rounding_modes,
        )),
        GenerationMode::SpecialRandom(scale) => Box::new(random_triples(
            &EXAMPLE_SEED,
            &special_random_unsigned,
            &(|seed| special_random_unsigned_vecs(seed, scale)),
            &random_rounding_modes,
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
            exhaustive_pairs_big_tiny(exhaustive_unsigneds(), exhaustive_unsigneds()),
            exhaustive_rounding_modes(),
        ))),
        GenerationMode::Random(scale) => Box::new(random_triples(
            &EXAMPLE_SEED,
            &random,
            &(|seed| u32s_geometric(seed, scale).flat_map(U::checked_from)),
            &random_rounding_modes,
        )),
        GenerationMode::SpecialRandom(scale) => Box::new(random_triples(
            &EXAMPLE_SEED,
            &special_random_unsigned,
            &(|seed| u32s_geometric(seed, scale).flat_map(U::checked_from)),
            &random_rounding_modes,
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
            &random,
            &(|seed| i32s_geometric(seed, scale).flat_map(U::checked_from)),
            &random_rounding_modes,
        )),
        GenerationMode::SpecialRandom(scale) => Box::new(random_triples(
            &EXAMPLE_SEED,
            &special_random_unsigned,
            &(|seed| i32s_geometric(seed, scale).flat_map(U::checked_from)),
            &random_rounding_modes,
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
            exhaustive_pairs_big_tiny(exhaustive_signeds(), exhaustive_unsigneds()),
            exhaustive_rounding_modes(),
        ))),
        GenerationMode::Random(scale) => Box::new(random_triples(
            &EXAMPLE_SEED,
            &random,
            &(|seed| u32s_geometric(seed, scale).flat_map(U::checked_from)),
            &random_rounding_modes,
        )),
        GenerationMode::SpecialRandom(scale) => Box::new(random_triples(
            &EXAMPLE_SEED,
            &special_random_signed,
            &(|seed| u32s_geometric(seed, scale).flat_map(U::checked_from)),
            &random_rounding_modes,
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
            exhaustive_pairs_big_tiny(exhaustive_signeds(), exhaustive_signeds()),
            exhaustive_rounding_modes(),
        ))),
        GenerationMode::Random(scale) => Box::new(random_triples(
            &EXAMPLE_SEED,
            &random,
            &(|seed| i32s_geometric(seed, scale).flat_map(U::checked_from)),
            &random_rounding_modes,
        )),
        GenerationMode::SpecialRandom(scale) => Box::new(random_triples(
            &EXAMPLE_SEED,
            &special_random_signed,
            &(|seed| i32s_geometric(seed, scale).flat_map(U::checked_from)),
            &random_rounding_modes,
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
            &random,
            &random_positive_unsigned,
            &random_rounding_modes,
        )),
        GenerationMode::SpecialRandom(_) => Box::new(random_triples(
            &EXAMPLE_SEED,
            &special_random_unsigned,
            &special_random_positive_unsigned,
            &random_rounding_modes,
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
            &random,
            &random_nonzero_signed,
            &random_rounding_modes,
        )),
        GenerationMode::SpecialRandom(_) => Box::new(random_triples(
            &EXAMPLE_SEED,
            &special_random_signed,
            &special_random_nonzero_signed,
            &random_rounding_modes,
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

// All sextuples of `Vec<Limb>`, `usize`, `Vec<Limb>`, `Limb`, `Limb`, and `u64` that are valid
// inputs to `limbs_div_mod_extra`.
pub fn sextuples_var_1(gm: GenerationMode) -> It<(Vec<Limb>, usize, Vec<Limb>, Limb, Limb, u64)> {
    let qs: It<(Vec<Limb>, usize, Vec<Limb>, Limb)> = match gm {
        GenerationMode::Exhaustive => Box::new(exhaustive_quadruples(
            exhaustive_vecs(exhaustive_unsigneds()),
            exhaustive_unsigneds(),
            exhaustive_vecs(exhaustive_unsigneds()),
            exhaustive_positive_primitive_ints(),
        )),
        GenerationMode::Random(scale) => Box::new(random_quadruples(
            &EXAMPLE_SEED,
            &(|seed| random_vecs(seed, scale, &random)),
            &(|seed| u32s_geometric(seed, scale).flat_map(usize::checked_from)),
            &(|seed| random_vecs(seed, scale, &random)),
            &random_positive_unsigned,
        )),
        GenerationMode::SpecialRandom(scale) => Box::new(random_quadruples(
            &EXAMPLE_SEED,
            &(|seed| special_random_unsigned_vecs(seed, scale)),
            &(|seed| u32s_geometric(seed, scale).flat_map(usize::checked_from)),
            &(|seed| special_random_unsigned_vecs(seed, scale)),
            &special_random_positive_unsigned,
        )),
    };
    Box::new(qs.filter_map(|(out, fraction_len, ns, d)| {
        if ns.is_empty() || out.len() < ns.len() + fraction_len {
            None
        } else {
            let shift = LeadingZeros::leading_zeros(d);
            let d_inv = limbs_invert_limb(d << shift);
            Some((out, fraction_len, ns, d, d_inv, shift))
        }
    }))
}

// All quintuples of `Vec<Limb>`, `usize`, `Limb`, `Limb`, and `u64` that are valid inputs to
// `limbs_div_mod_extra_in_place`.
pub fn quintuples_var_1(gm: GenerationMode) -> It<(Vec<Limb>, usize, Limb, Limb, u64)> {
    let ts: It<(Vec<Limb>, usize, Limb)> = match gm {
        GenerationMode::Exhaustive => Box::new(exhaustive_triples(
            exhaustive_vecs(exhaustive_unsigneds()),
            exhaustive_unsigneds(),
            exhaustive_positive_primitive_ints(),
        )),
        GenerationMode::Random(scale) => Box::new(random_triples(
            &EXAMPLE_SEED,
            &(|seed| random_vecs(seed, scale, &random)),
            &(|seed| u32s_geometric(seed, scale).flat_map(usize::checked_from)),
            &random_positive_unsigned,
        )),
        GenerationMode::SpecialRandom(scale) => Box::new(random_triples(
            &EXAMPLE_SEED,
            &(|seed| special_random_unsigned_vecs(seed, scale)),
            &(|seed| u32s_geometric(seed, scale).flat_map(usize::checked_from)),
            &special_random_positive_unsigned,
        )),
    };
    Box::new(ts.filter_map(|(ns, fraction_len, d)| {
        if ns.len() <= fraction_len {
            None
        } else {
            let shift = LeadingZeros::leading_zeros(d);
            let d_inv = limbs_invert_limb(d << shift);
            Some((ns, fraction_len, d, d_inv, shift))
        }
    }))
}
