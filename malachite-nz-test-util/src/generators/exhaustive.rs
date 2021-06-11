use crate::common::{
    integer_to_bigint, integer_to_rug_integer, natural_to_biguint, natural_to_rug_integer,
};
use malachite_base::bools::exhaustive::{exhaustive_bools, ExhaustiveBools};
use malachite_base::iterators::bit_distributor::BitDistributorOutputType;
use malachite_base::iterators::iter_windows;
use malachite_base::num::arithmetic::traits::{ArithmeticCheckedShl, DivRound, Parity, PowerOf2};
use malachite_base::num::basic::integers::PrimitiveInt;
use malachite_base::num::basic::traits::{One, Two, Zero};
use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base::num::conversion::traits::{
    ConvertibleFrom, ExactFrom, SaturatingFrom, WrappingFrom,
};
use malachite_base::num::exhaustive::{
    exhaustive_positive_primitive_ints, exhaustive_unsigneds,
    primitive_int_increasing_inclusive_range, primitive_int_increasing_range,
    PrimitiveIntIncreasingRange,
};
use malachite_base::num::float::PrimitiveFloat;
use malachite_base::num::iterators::{bit_distributor_sequence, ruler_sequence};
use malachite_base::num::logic::traits::SignificantBits;
use malachite_base::rounding_modes::exhaustive::exhaustive_rounding_modes;
use malachite_base::rounding_modes::RoundingMode;
use malachite_base::tuples::exhaustive::{
    exhaustive_dependent_pairs, exhaustive_pairs, exhaustive_pairs_from_single,
    exhaustive_triples_custom_output, exhaustive_triples_from_single, lex_pairs,
    ExhaustiveDependentPairsYsGenerator,
};
use malachite_base::vecs::exhaustive::{
    exhaustive_fixed_length_vecs_from_single, exhaustive_vecs, exhaustive_vecs_length_range,
    exhaustive_vecs_min_length, lex_fixed_length_vecs_from_single, ExhaustiveVecs,
    LexFixedLengthVecsFromSingle,
};
use malachite_base_test_util::generators::common::{
    permute_1_3_2, permute_2_1, reshape_2_1_to_3, It,
};
use malachite_base_test_util::generators::exhaustive::{
    UnsignedVecTripleLenGenerator, UnsignedVecTripleXYYLenGenerator,
};
use malachite_base_test_util::generators::exhaustive_pairs_big_tiny;
use malachite_nz::integer::exhaustive::exhaustive_integers;
use malachite_nz::integer::Integer;
use malachite_nz::natural::arithmetic::mul::fft::*;
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
use malachite_nz::natural::conversion::digits::general_digits::{
    limbs_digit_count, limbs_per_digit_in_base, GET_STR_PRECOMPUTE_THRESHOLD,
};
use malachite_nz::natural::exhaustive::{
    exhaustive_natural_range, exhaustive_natural_range_to_infinity, exhaustive_naturals,
    exhaustive_positive_naturals, ExhaustiveNaturalRange,
};
use malachite_nz::natural::Natural;
use malachite_nz::platform::Limb;
use std::iter::once;
use std::marker::PhantomData;

// -- Integer --

pub fn exhaustive_integer_gen() -> It<Integer> {
    Box::new(exhaustive_integers())
}

pub fn exhaustive_integer_gen_var_1<T: PrimitiveFloat>() -> It<Integer>
where
    Natural: From<T> + From<T::UnsignedOfEqualWidth>,
{
    Box::new(
        once(Integer::ZERO).chain(
            lex_pairs(
                exhaustive_positive_float_naturals::<T>(0),
                exhaustive_bools(),
            )
            .map(|(n, b)| Integer::from_sign_and_abs(b, n)),
        ),
    )
}

pub fn exhaustive_integer_gen_var_2<T: for<'a> ConvertibleFrom<&'a Natural> + PrimitiveFloat>(
) -> It<Integer> {
    Box::new(
        lex_pairs(exhaustive_natural_gen_var_4::<T>(), exhaustive_bools())
            .map(|(n, b)| Integer::from_sign_and_abs(b, n)),
    )
}

pub fn exhaustive_integer_gen_var_3<T: PrimitiveFloat>() -> It<Integer>
where
    Natural: From<T> + From<T::UnsignedOfEqualWidth>,
{
    Box::new(
        lex_pairs(exhaustive_natural_gen_var_5::<T>(), exhaustive_bools())
            .map(|(n, b)| Integer::from_sign_and_abs(b, n)),
    )
}

// -- (Integer, PrimitiveUnsigned) --

pub fn exhaustive_integer_unsigned_pair_gen_var_1<T: ExactFrom<u8> + PrimitiveUnsigned>(
) -> It<(Integer, T)> {
    Box::new(lex_pairs(
        exhaustive_integers(),
        primitive_int_increasing_inclusive_range(T::TWO, T::exact_from(36u8)),
    ))
}

pub fn exhaustive_integer_unsigned_pair_gen_var_2<T: PrimitiveUnsigned>() -> It<(Integer, T)> {
    Box::new(exhaustive_pairs_big_tiny(
        exhaustive_integers(),
        exhaustive_unsigneds(),
    ))
}

// -- (Integer, PrimitiveUnsigned, PrimitiveUnsigned) --

pub fn exhaustive_integer_unsigned_unsigned_triple_gen_var_1<
    T: ExactFrom<u8> + PrimitiveUnsigned,
    U: PrimitiveUnsigned,
>() -> It<(Integer, T, U)> {
    permute_1_3_2(reshape_2_1_to_3(Box::new(lex_pairs(
        exhaustive_pairs_big_tiny(exhaustive_integers(), exhaustive_unsigneds()),
        primitive_int_increasing_inclusive_range(T::TWO, T::exact_from(36u8)),
    ))))
}

// -- (Integer, RoundingMode) --

pub fn exhaustive_integer_rounding_mode_pair_gen_var_1<
    T: for<'a> ConvertibleFrom<&'a Integer> + PrimitiveFloat,
>() -> It<(Integer, RoundingMode)> {
    Box::new(
        lex_pairs(exhaustive_integers(), exhaustive_rounding_modes())
            .filter(|&(ref n, rm)| rm != RoundingMode::Exact || T::convertible_from(n)),
    )
}

// -- Natural --

pub fn exhaustive_natural_gen() -> It<Natural> {
    Box::new(exhaustive_naturals())
}

pub fn exhaustive_natural_gen_var_1() -> It<Natural> {
    Box::new(exhaustive_natural_range_to_infinity(Natural::TWO))
}

pub fn exhaustive_natural_gen_var_2() -> It<Natural> {
    Box::new(exhaustive_positive_naturals())
}

struct ExhaustivePositiveFloatNaturals<T: PrimitiveFloat>
where
    Natural: From<T> + From<T::UnsignedOfEqualWidth>,
{
    done: bool,
    exponent: i64,
    limit: T::UnsignedOfEqualWidth,
    mantissa: T::UnsignedOfEqualWidth,
    max_finite: Natural,
}

impl<T: PrimitiveFloat> Iterator for ExhaustivePositiveFloatNaturals<T>
where
    Natural: From<T> + From<T::UnsignedOfEqualWidth>,
{
    type Item = Natural;

    fn next(&mut self) -> Option<Natural> {
        if self.done {
            None
        } else {
            let n = Natural::from(self.mantissa) << self.exponent;
            if n == self.max_finite {
                self.done = true;
            } else {
                self.mantissa += T::UnsignedOfEqualWidth::ONE;
                if self.mantissa == self.limit {
                    self.mantissa >>= 1;
                    self.exponent += 1;
                    self.limit = T::UnsignedOfEqualWidth::power_of_2(T::MANTISSA_WIDTH + 1);
                }
            }
            Some(n)
        }
    }
}

fn exhaustive_positive_float_naturals<T: PrimitiveFloat>(
    start_exponent: i64,
) -> ExhaustivePositiveFloatNaturals<T>
where
    Natural: From<T> + From<T::UnsignedOfEqualWidth>,
{
    ExhaustivePositiveFloatNaturals {
        done: false,
        exponent: start_exponent,
        limit: T::UnsignedOfEqualWidth::power_of_2(T::MANTISSA_WIDTH + 1),
        mantissa: if start_exponent == 0 {
            T::UnsignedOfEqualWidth::ONE
        } else {
            T::UnsignedOfEqualWidth::power_of_2(T::MANTISSA_WIDTH)
        },
        max_finite: Natural::from(T::MAX_FINITE),
    }
}

pub fn exhaustive_natural_gen_var_3<T: PrimitiveFloat>() -> It<Natural>
where
    Natural: From<T> + From<T::UnsignedOfEqualWidth>,
{
    Box::new(once(Natural::ZERO).chain(exhaustive_positive_float_naturals::<T>(0)))
}

pub fn exhaustive_natural_gen_var_4<T: for<'a> ConvertibleFrom<&'a Natural> + PrimitiveFloat>(
) -> It<Natural> {
    Box::new(
        exhaustive_natural_range_to_infinity(
            Natural::power_of_2(T::MANTISSA_WIDTH + 1) | Natural::ONE,
        )
        .filter(|n| !T::convertible_from(n)),
    )
}

pub fn exhaustive_natural_gen_var_5<T: PrimitiveFloat>() -> It<Natural>
where
    Natural: From<T> + From<T::UnsignedOfEqualWidth>,
{
    Box::new(
        iter_windows(2, exhaustive_positive_float_naturals::<T>(1)).filter_map(|xs| {
            let mut xs = xs.into_iter();
            let a = xs.next().unwrap();
            let diff = xs.next().unwrap() - &a;
            if diff.even() {
                // This happens almost always
                Some(a + (diff >> 1))
            } else {
                None
            }
        }),
    )
}

// -- (Natural, Natural) --

pub fn exhaustive_natural_pair_gen() -> It<(Natural, Natural)> {
    Box::new(exhaustive_pairs_from_single(exhaustive_naturals()))
}

pub fn exhaustive_natural_pair_gen_var_1() -> It<(Natural, Natural)> {
    Box::new(exhaustive_pairs(
        exhaustive_natural_range_to_infinity(Natural::power_of_2(Limb::WIDTH)),
        exhaustive_natural_range_to_infinity(Natural::TWO),
    ))
}

pub fn exhaustive_natural_pair_gen_var_2() -> It<(Natural, Natural)> {
    Box::new(exhaustive_pairs(
        exhaustive_naturals(),
        exhaustive_natural_range_to_infinity(Natural::TWO),
    ))
}

// -- (Natural, Natural, Natural) --

pub fn exhaustive_natural_triple_gen() -> It<(Natural, Natural, Natural)> {
    Box::new(exhaustive_triples_from_single(exhaustive_naturals()))
}

// -- (Natural, PrimitiveInt) --

pub fn exhaustive_natural_primitive_int_pair_gen_var_1<
    T: PrimitiveInt + SaturatingFrom<U>,
    U: PrimitiveInt,
>() -> It<(Natural, T)> {
    Box::new(exhaustive_pairs_big_tiny(
        exhaustive_naturals(),
        primitive_int_increasing_inclusive_range(T::TWO, T::saturating_from(U::MAX)),
    ))
}

pub fn exhaustive_natural_primitive_int_pair_gen_var_2<T: PrimitiveInt>() -> It<(Natural, T)> {
    Box::new(exhaustive_pairs_big_tiny(
        exhaustive_naturals(),
        primitive_int_increasing_inclusive_range(T::TWO, T::MAX),
    ))
}

pub fn exhaustive_natural_primitive_int_pair_gen_var_3<T: PrimitiveInt>() -> It<(Natural, T)> {
    Box::new(exhaustive_pairs_big_tiny(
        exhaustive_naturals(),
        exhaustive_positive_primitive_ints(),
    ))
}

pub fn exhaustive_natural_primitive_int_pair_gen_var_4<T: PrimitiveInt>() -> It<(Natural, T)> {
    Box::new(exhaustive_pairs_big_tiny(
        exhaustive_positive_naturals(),
        exhaustive_positive_primitive_ints(),
    ))
}

// -- (Natural, PrimitiveInt, PrimitiveUnsigned) --

pub fn exhaustive_natural_primitive_int_unsigned_triple_gen_var_3<
    T: PrimitiveInt,
    U: PrimitiveUnsigned,
>() -> It<(Natural, T, U)> {
    Box::new(exhaustive_triples_custom_output(
        exhaustive_naturals(),
        exhaustive_positive_primitive_ints(),
        exhaustive_unsigneds(),
        BitDistributorOutputType::normal(1),
        BitDistributorOutputType::tiny(),
        BitDistributorOutputType::tiny(),
    ))
}

// -- (Natural, PrimitiveUnsigned) --

pub fn exhaustive_natural_unsigned_pair_gen_var_1<T: ExactFrom<u8> + PrimitiveUnsigned>(
) -> It<(Natural, T)> {
    Box::new(lex_pairs(
        exhaustive_naturals(),
        primitive_int_increasing_inclusive_range(T::TWO, T::exact_from(36u8)),
    ))
}

pub fn exhaustive_natural_unsigned_pair_gen_var_2<T: PrimitiveUnsigned>() -> It<(Natural, T)> {
    Box::new(exhaustive_pairs_big_tiny(
        exhaustive_naturals(),
        exhaustive_unsigneds(),
    ))
}

pub fn exhaustive_natural_unsigned_pair_gen_var_3<T: PrimitiveUnsigned>() -> It<(Natural, T)> {
    Box::new(exhaustive_pairs_big_tiny(
        exhaustive_natural_range_to_infinity(Natural::TWO),
        exhaustive_unsigneds(),
    ))
}

pub fn exhaustive_natural_unsigned_pair_gen_var_4<T: PrimitiveInt>() -> It<(Natural, u64)> {
    Box::new(exhaustive_pairs_big_tiny(
        exhaustive_naturals(),
        primitive_int_increasing_inclusive_range(1, T::WIDTH),
    ))
}

// -- (Natural, PrimitiveUnsigned, PrimitiveUnsigned) --

pub fn exhaustive_natural_unsigned_unsigned_triple_gen_var_1<
    T: ExactFrom<u8> + PrimitiveUnsigned,
    U: PrimitiveUnsigned,
>() -> It<(Natural, T, U)> {
    permute_1_3_2(reshape_2_1_to_3(Box::new(lex_pairs(
        exhaustive_pairs_big_tiny(exhaustive_naturals(), exhaustive_unsigneds()),
        primitive_int_increasing_inclusive_range(T::TWO, T::exact_from(36u8)),
    ))))
}

pub fn exhaustive_natural_unsigned_unsigned_triple_gen_var_2<
    T: PrimitiveUnsigned,
    U: PrimitiveInt,
>() -> It<(Natural, u64, T)> {
    permute_1_3_2(reshape_2_1_to_3(Box::new(lex_pairs(
        exhaustive_pairs_big_tiny(exhaustive_naturals(), exhaustive_unsigneds()),
        primitive_int_increasing_inclusive_range(1, U::WIDTH),
    ))))
}

// -- (Natural, PrimitiveUnsigned, Vec<bool>) --

struct NaturalUnsignedBoolVecPairGenerator;

impl
    ExhaustiveDependentPairsYsGenerator<
        (Natural, u64),
        Vec<bool>,
        LexFixedLengthVecsFromSingle<ExhaustiveBools>,
    > for NaturalUnsignedBoolVecPairGenerator
{
    #[inline]
    fn get_ys(&self, p: &(Natural, u64)) -> LexFixedLengthVecsFromSingle<ExhaustiveBools> {
        lex_fixed_length_vecs_from_single(
            p.0.significant_bits().div_round(p.1, RoundingMode::Up),
            exhaustive_bools(),
        )
    }
}

pub fn exhaustive_natural_unsigned_bool_vec_triple_gen_var_1() -> It<(Natural, u64, Vec<bool>)> {
    reshape_2_1_to_3(Box::new(exhaustive_dependent_pairs(
        bit_distributor_sequence(
            BitDistributorOutputType::normal(1),
            BitDistributorOutputType::normal(1),
        ),
        exhaustive_pairs_big_tiny(exhaustive_naturals(), exhaustive_positive_primitive_ints()),
        NaturalUnsignedBoolVecPairGenerator,
    )))
}

pub fn exhaustive_natural_unsigned_bool_vec_triple_gen_var_2<T: PrimitiveInt>(
) -> It<(Natural, u64, Vec<bool>)> {
    reshape_2_1_to_3(Box::new(exhaustive_dependent_pairs(
        bit_distributor_sequence(
            BitDistributorOutputType::normal(1),
            BitDistributorOutputType::normal(1),
        ),
        lex_pairs(
            exhaustive_naturals(),
            primitive_int_increasing_inclusive_range(1, T::WIDTH),
        ),
        NaturalUnsignedBoolVecPairGenerator,
    )))
}

// -- (Natural, RoundingMode) --

pub fn exhaustive_natural_rounding_mode_pair_gen_var_1<
    T: for<'a> ConvertibleFrom<&'a Natural> + PrimitiveFloat,
>() -> It<(Natural, RoundingMode)> {
    Box::new(
        lex_pairs(exhaustive_naturals(), exhaustive_rounding_modes())
            .filter(|&(ref n, rm)| rm != RoundingMode::Exact || T::convertible_from(n)),
    )
}

// -- (String, String, String) --

pub fn exhaustive_string_triple_gen_var_1() -> It<(String, String, String)> {
    Box::new(exhaustive_naturals().map(|x| {
        (
            serde_json::to_string(&natural_to_biguint(&x)).unwrap(),
            serde_json::to_string(&natural_to_rug_integer(&x)).unwrap(),
            serde_json::to_string(&x).unwrap(),
        )
    }))
}

pub fn exhaustive_string_triple_gen_var_2() -> It<(String, String, String)> {
    Box::new(exhaustive_integers().map(|x| {
        (
            serde_json::to_string(&integer_to_bigint(&x)).unwrap(),
            serde_json::to_string(&integer_to_rug_integer(&x)).unwrap(),
            serde_json::to_string(&x).unwrap(),
        )
    }))
}

// -- (Vec<Natural>, Natural)

struct ValidDigitsGenerator;

impl ExhaustiveDependentPairsYsGenerator<Natural, Vec<Natural>, It<Vec<Natural>>>
    for ValidDigitsGenerator
{
    #[inline]
    fn get_ys(&self, base: &Natural) -> It<Vec<Natural>> {
        Box::new(exhaustive_vecs(exhaustive_natural_range(
            Natural::ZERO,
            base.clone(),
        )))
    }
}

pub fn exhaustive_natural_vec_natural_pair_gen_var_1() -> It<(Vec<Natural>, Natural)> {
    permute_2_1(Box::new(exhaustive_dependent_pairs(
        bit_distributor_sequence(
            BitDistributorOutputType::normal(1),
            BitDistributorOutputType::normal(1),
        ),
        exhaustive_natural_range_to_infinity(Natural::power_of_2(Limb::WIDTH)),
        ValidDigitsGenerator,
    )))
}

pub fn exhaustive_natural_vec_natural_pair_gen_var_2() -> It<(Vec<Natural>, Natural)> {
    permute_2_1(Box::new(exhaustive_dependent_pairs(
        bit_distributor_sequence(
            BitDistributorOutputType::normal(1),
            BitDistributorOutputType::normal(1),
        ),
        exhaustive_natural_range_to_infinity(Natural::TWO),
        ValidDigitsGenerator,
    )))
}

pub fn exhaustive_natural_vec_natural_pair_gen_var_3() -> It<(Vec<Natural>, Natural)> {
    Box::new(exhaustive_pairs_big_tiny(
        exhaustive_vecs(exhaustive_naturals()),
        exhaustive_natural_range_to_infinity(Natural::power_of_2(Limb::WIDTH)),
    ))
}

pub fn exhaustive_natural_vec_natural_pair_gen_var_4() -> It<(Vec<Natural>, Natural)> {
    Box::new(exhaustive_pairs_big_tiny(
        exhaustive_vecs(exhaustive_naturals()),
        exhaustive_natural_range_to_infinity(Natural::TWO),
    ))
}

// -- (Vec<Natural>, PrimitiveInt) --

pub fn exhaustive_natural_vec_primitive_int_pair_gen_var_1<T: PrimitiveInt>(
) -> It<(Vec<Natural>, T)> {
    Box::new(exhaustive_pairs_big_tiny(
        exhaustive_vecs(exhaustive_naturals()),
        exhaustive_positive_primitive_ints(),
    ))
}

// -- (Vec<Natural>, u64) --

struct PowerOfTwoDigitsGenerator;

impl
    ExhaustiveDependentPairsYsGenerator<
        u64,
        Vec<Natural>,
        ExhaustiveVecs<Natural, PrimitiveIntIncreasingRange<u64>, ExhaustiveNaturalRange>,
    > for PowerOfTwoDigitsGenerator
{
    #[inline]
    fn get_ys(
        &self,
        &log_base: &u64,
    ) -> ExhaustiveVecs<Natural, PrimitiveIntIncreasingRange<u64>, ExhaustiveNaturalRange> {
        exhaustive_vecs(exhaustive_natural_range(
            Natural::ZERO,
            Natural::power_of_2(log_base),
        ))
    }
}

pub fn exhaustive_natural_vec_unsigned_pair_gen_var_1() -> It<(Vec<Natural>, u64)> {
    permute_2_1(Box::new(exhaustive_dependent_pairs(
        bit_distributor_sequence(
            BitDistributorOutputType::normal(1),
            BitDistributorOutputType::normal(1),
        ),
        primitive_int_increasing_inclusive_range(1, u64::MAX),
        PowerOfTwoDigitsGenerator,
    )))
}

// -- (Vec<PrimitiveUnsigned>, PrimitiveUnsigned)

// vars 1 through 3 are in malachite-base

pub fn exhaustive_unsigned_vec_unsigned_pair_gen_var_4<
    T: PrimitiveUnsigned + SaturatingFrom<U>,
    U: PrimitiveInt,
>() -> It<(Vec<T>, T)> {
    Box::new(exhaustive_pairs_big_tiny(
        exhaustive_vecs_min_length(2, exhaustive_unsigneds()),
        primitive_int_increasing_inclusive_range(T::TWO, T::saturating_from(U::MAX)),
    ))
}

// var 5 is in malachite-base

// -- (Vec<PrimitiveUnsigned>, PrimitiveUnsigned, Vec<PrimitiveUnsigned>)

struct ValidLengthsGenerator;

impl<T: PrimitiveUnsigned> ExhaustiveDependentPairsYsGenerator<(Vec<Limb>, u64), Vec<T>, It<Vec<T>>>
    for ValidLengthsGenerator
{
    #[inline]
    fn get_ys(&self, p: &(Vec<Limb>, u64)) -> It<Vec<T>> {
        Box::new(exhaustive_vecs_min_length(
            limbs_digit_count(&p.0, p.1),
            exhaustive_unsigneds(),
        ))
    }
}

pub fn exhaustive_unsigned_vec_unsigned_unsigned_vec_triple_gen_var_1<T: PrimitiveUnsigned>(
) -> It<(Vec<T>, u64, Vec<Limb>)> {
    Box::new(
        exhaustive_dependent_pairs(
            bit_distributor_sequence(
                BitDistributorOutputType::normal(1),
                BitDistributorOutputType::normal(1),
            ),
            exhaustive_pairs_big_tiny(
                exhaustive_vecs(exhaustive_unsigneds()),
                (3u64..256).filter(|&b| !b.is_power_of_two()),
            ),
            ValidLengthsGenerator,
        )
        .map(|((xs, base), out)| (out, base, xs)),
    )
}

// -- (Vec<PrimitiveUnsigned>, PrimitiveUnsigned, Vec<PrimitiveUnsigned>, PrimitiveUnsigned) --

struct ValidLengthsBasecaseGenerator {
    min_out_len: usize,
}

impl<T: PrimitiveUnsigned> ExhaustiveDependentPairsYsGenerator<usize, Vec<T>, It<Vec<T>>>
    for ValidLengthsBasecaseGenerator
{
    #[inline]
    fn get_ys(&self, &len: &usize) -> It<Vec<T>> {
        Box::new(exhaustive_vecs_min_length(
            u64::exact_from(if len == 0 { self.min_out_len } else { len }),
            exhaustive_unsigneds(),
        ))
    }
}

struct BasecaseDigitsInputGenerator;

impl<T: PrimitiveUnsigned>
    ExhaustiveDependentPairsYsGenerator<(Vec<Limb>, u64), (Vec<T>, usize), It<(Vec<T>, usize)>>
    for BasecaseDigitsInputGenerator
{
    #[inline]
    fn get_ys(&self, p: &(Vec<Limb>, u64)) -> It<(Vec<T>, usize)> {
        let min_out_len = usize::exact_from(limbs_digit_count(&p.0, p.1));
        permute_2_1(Box::new(exhaustive_dependent_pairs(
            ruler_sequence(),
            once(0).chain(primitive_int_increasing_inclusive_range(
                min_out_len,
                usize::MAX,
            )),
            ValidLengthsBasecaseGenerator { min_out_len },
        )))
    }
}

pub fn exhaustive_unsigned_vec_unsigned_unsigned_vec_unsigned_quadruple_gen_var_1<
    T: PrimitiveUnsigned,
>() -> It<(Vec<T>, usize, Vec<Limb>, u64)> {
    Box::new(
        exhaustive_dependent_pairs(
            bit_distributor_sequence(
                BitDistributorOutputType::normal(1),
                BitDistributorOutputType::normal(1),
            ),
            exhaustive_pairs_big_tiny(
                exhaustive_vecs_length_range(
                    0,
                    u64::wrapping_from(GET_STR_PRECOMPUTE_THRESHOLD),
                    exhaustive_unsigneds(),
                ),
                (3u64..256).filter(|&b| !b.is_power_of_two()),
            ),
            BasecaseDigitsInputGenerator,
        )
        .map(|((xs, base), (out, len))| (out, len, xs, base)),
    )
}

// -- (Vec<PrimitiveUnsigned>, Vec<PrimitiveUnsigned>, PrimitiveUnsigned) --

struct ValidDigitsGenerator1<T: PrimitiveUnsigned, U: PrimitiveUnsigned> {
    phantom_t: PhantomData<*const T>,
    phantom_u: PhantomData<*const U>,
}

impl<T: PrimitiveUnsigned, U: PrimitiveUnsigned>
    ExhaustiveDependentPairsYsGenerator<(u64, usize), (Vec<T>, Vec<U>), It<(Vec<T>, Vec<U>)>>
    for ValidDigitsGenerator1<T, U>
{
    #[inline]
    fn get_ys(&self, p: &(u64, usize)) -> It<(Vec<T>, Vec<U>)> {
        Box::new(exhaustive_pairs(
            exhaustive_fixed_length_vecs_from_single(
                u64::wrapping_from(p.1),
                primitive_int_increasing_range(T::ZERO, T::wrapping_from(p.0)),
            ),
            exhaustive_vecs_min_length(limbs_per_digit_in_base(p.1, p.0), exhaustive_unsigneds()),
        ))
    }
}

// var 1 is in malachite-base

pub fn exhaustive_unsigned_vec_unsigned_vec_unsigned_triple_gen_var_2<
    T: PrimitiveUnsigned,
    U: PrimitiveUnsigned,
>() -> It<(Vec<U>, Vec<T>, u64)> {
    Box::new(
        exhaustive_dependent_pairs(
            bit_distributor_sequence(
                BitDistributorOutputType::normal(1),
                BitDistributorOutputType::normal(1),
            ),
            exhaustive_pairs_big_tiny(
                (3u64..256).filter(|&b| !b.is_power_of_two()),
                exhaustive_positive_primitive_ints(),
            ),
            ValidDigitsGenerator1 {
                phantom_t: PhantomData,
                phantom_u: PhantomData,
            },
        )
        .map(|((base, _), (xs, out))| (out, xs, base)),
    )
}

struct ValidDigitsGenerator2<T: PrimitiveUnsigned, U: PrimitiveUnsigned> {
    phantom_t: PhantomData<*const T>,
    phantom_u: PhantomData<*const U>,
}

impl<T: PrimitiveUnsigned, U: PrimitiveUnsigned>
    ExhaustiveDependentPairsYsGenerator<(u64, usize), (Vec<T>, Vec<U>), It<(Vec<T>, Vec<U>)>>
    for ValidDigitsGenerator2<T, U>
{
    #[inline]
    fn get_ys(&self, p: &(u64, usize)) -> It<(Vec<T>, Vec<U>)> {
        Box::new(exhaustive_pairs(
            exhaustive_fixed_length_vecs_from_single(
                u64::wrapping_from(p.1),
                exhaustive_unsigneds(),
            ),
            exhaustive_vecs_min_length(limbs_per_digit_in_base(p.1, p.0), exhaustive_unsigneds()),
        ))
    }
}

pub fn exhaustive_unsigned_vec_unsigned_vec_unsigned_triple_gen_var_3<
    T: PrimitiveUnsigned,
    U: PrimitiveUnsigned,
>() -> It<(Vec<U>, Vec<T>, u64)> {
    Box::new(
        exhaustive_dependent_pairs(
            bit_distributor_sequence(
                BitDistributorOutputType::normal(1),
                BitDistributorOutputType::normal(1),
            ),
            exhaustive_pairs_big_tiny(
                (3u64..256).filter(|&b| !b.is_power_of_two()),
                exhaustive_positive_primitive_ints(),
            ),
            ValidDigitsGenerator2 {
                phantom_t: PhantomData,
                phantom_u: PhantomData,
            },
        )
        .map(|((base, _), (xs, out))| (out, xs, base)),
    )
}

// -- (Vec<PrimitiveUnsigned>, Vec<PrimitiveUnsigned>, Vec<PrimitiveUnsigned>) --

// vars 1 through 3 are in malachite-base

fn exhaustive_mul_helper<T: PrimitiveUnsigned, F: Fn(usize, usize) -> bool>(
    valid: &'static F,
    min_x: u64,
    min_y: u64,
) -> It<(Vec<T>, Vec<T>, Vec<T>)> {
    Box::new(
        exhaustive_dependent_pairs(
            bit_distributor_sequence(
                BitDistributorOutputType::tiny(),
                BitDistributorOutputType::normal(1),
            ),
            exhaustive_triples_from_single(exhaustive_unsigneds::<u64>()).flat_map(
                move |(o, x, y)| {
                    let x = x.checked_add(min_x)?;
                    let y = y.checked_add(min_y)?;
                    if valid(usize::exact_from(x), usize::exact_from(y)) {
                        let o = x.checked_add(y)?.checked_add(o)?;
                        Some((o, x, y))
                    } else {
                        None
                    }
                },
            ),
            UnsignedVecTripleLenGenerator,
        )
        .map(|p| p.1),
    )
}

pub fn exhaustive_unsigned_vec_triple_gen_var_4<T: PrimitiveUnsigned>(
) -> It<(Vec<T>, Vec<T>, Vec<T>)> {
    exhaustive_mul_helper(&_limbs_mul_greater_to_out_toom_22_input_sizes_valid, 2, 2)
}

pub fn exhaustive_unsigned_vec_triple_gen_var_5<T: PrimitiveUnsigned>(
) -> It<(Vec<T>, Vec<T>, Vec<T>)> {
    exhaustive_mul_helper(&_limbs_mul_greater_to_out_toom_32_input_sizes_valid, 6, 4)
}

pub fn exhaustive_unsigned_vec_triple_gen_var_6<T: PrimitiveUnsigned>(
) -> It<(Vec<T>, Vec<T>, Vec<T>)> {
    exhaustive_mul_helper(&_limbs_mul_greater_to_out_toom_33_input_sizes_valid, 3, 3)
}

pub fn exhaustive_unsigned_vec_triple_gen_var_7<T: PrimitiveUnsigned>(
) -> It<(Vec<T>, Vec<T>, Vec<T>)> {
    exhaustive_mul_helper(&_limbs_mul_greater_to_out_toom_42_input_sizes_valid, 4, 2)
}

pub fn exhaustive_unsigned_vec_triple_gen_var_8<T: PrimitiveUnsigned>(
) -> It<(Vec<T>, Vec<T>, Vec<T>)> {
    exhaustive_mul_helper(&_limbs_mul_greater_to_out_toom_43_input_sizes_valid, 11, 8)
}

pub fn exhaustive_unsigned_vec_triple_gen_var_9<T: PrimitiveUnsigned>(
) -> It<(Vec<T>, Vec<T>, Vec<T>)> {
    exhaustive_mul_helper(&_limbs_mul_greater_to_out_toom_44_input_sizes_valid, 4, 4)
}

pub fn exhaustive_unsigned_vec_triple_gen_var_10<T: PrimitiveUnsigned>(
) -> It<(Vec<T>, Vec<T>, Vec<T>)> {
    exhaustive_mul_helper(&_limbs_mul_greater_to_out_toom_52_input_sizes_valid, 14, 5)
}

pub fn exhaustive_unsigned_vec_triple_gen_var_11<T: PrimitiveUnsigned>(
) -> It<(Vec<T>, Vec<T>, Vec<T>)> {
    exhaustive_mul_helper(&_limbs_mul_greater_to_out_toom_53_input_sizes_valid, 5, 3)
}

pub fn exhaustive_unsigned_vec_triple_gen_var_12<T: PrimitiveUnsigned>(
) -> It<(Vec<T>, Vec<T>, Vec<T>)> {
    exhaustive_mul_helper(&_limbs_mul_greater_to_out_toom_54_input_sizes_valid, 14, 11)
}

pub fn exhaustive_unsigned_vec_triple_gen_var_13<T: PrimitiveUnsigned>(
) -> It<(Vec<T>, Vec<T>, Vec<T>)> {
    exhaustive_mul_helper(&_limbs_mul_greater_to_out_toom_62_input_sizes_valid, 6, 2)
}

pub fn exhaustive_unsigned_vec_triple_gen_var_14<T: PrimitiveUnsigned>(
) -> It<(Vec<T>, Vec<T>, Vec<T>)> {
    exhaustive_mul_helper(&_limbs_mul_greater_to_out_toom_63_input_sizes_valid, 17, 9)
}

pub fn exhaustive_unsigned_vec_triple_gen_var_15<T: PrimitiveUnsigned>(
) -> It<(Vec<T>, Vec<T>, Vec<T>)> {
    exhaustive_mul_helper(&_limbs_mul_greater_to_out_toom_6h_input_sizes_valid, 42, 42)
}

pub fn exhaustive_unsigned_vec_triple_gen_var_16<T: PrimitiveUnsigned>(
) -> It<(Vec<T>, Vec<T>, Vec<T>)> {
    exhaustive_mul_helper(&_limbs_mul_greater_to_out_toom_8h_input_sizes_valid, 86, 86)
}

pub fn exhaustive_unsigned_vec_triple_gen_var_17<T: PrimitiveUnsigned>(
) -> It<(Vec<T>, Vec<T>, Vec<T>)> {
    exhaustive_mul_helper(&_limbs_mul_greater_to_out_fft_input_sizes_threshold, 15, 15)
}

fn exhaustive_mul_same_length_helper<T: PrimitiveUnsigned, F: Fn(usize, usize) -> bool>(
    valid: &'static F,
    min_x: u64,
) -> It<(Vec<T>, Vec<T>, Vec<T>)> {
    Box::new(
        exhaustive_dependent_pairs(
            bit_distributor_sequence(
                BitDistributorOutputType::tiny(),
                BitDistributorOutputType::normal(1),
            ),
            exhaustive_pairs_from_single(exhaustive_unsigneds::<u64>()).flat_map(move |(o, x)| {
                let x = x.checked_add(min_x)?;
                let ux = usize::exact_from(x);
                if valid(ux, ux) {
                    let o = x.arithmetic_checked_shl(1u64)?.checked_add(o)?;
                    Some((o, x))
                } else {
                    None
                }
            }),
            UnsignedVecTripleXYYLenGenerator,
        )
        .map(|p| p.1),
    )
}

pub fn exhaustive_unsigned_vec_triple_gen_var_18<T: PrimitiveUnsigned>(
) -> It<(Vec<T>, Vec<T>, Vec<T>)> {
    exhaustive_mul_same_length_helper(&_limbs_mul_greater_to_out_toom_33_input_sizes_valid, 5)
}

pub fn exhaustive_unsigned_vec_triple_gen_var_19<T: PrimitiveUnsigned>(
) -> It<(Vec<T>, Vec<T>, Vec<T>)> {
    exhaustive_mul_same_length_helper(&_limbs_mul_greater_to_out_toom_6h_input_sizes_valid, 42)
}

pub fn exhaustive_unsigned_vec_triple_gen_var_20<T: PrimitiveUnsigned>(
) -> It<(Vec<T>, Vec<T>, Vec<T>)> {
    exhaustive_mul_same_length_helper(&_limbs_mul_greater_to_out_toom_8h_input_sizes_valid, 86)
}

pub fn exhaustive_unsigned_vec_triple_gen_var_21<T: PrimitiveUnsigned>(
) -> It<(Vec<T>, Vec<T>, Vec<T>)> {
    exhaustive_mul_same_length_helper(
        &|xs_len, ys_len| {
            _limbs_mul_greater_to_out_toom_8h_input_sizes_valid(xs_len, ys_len)
                && _limbs_mul_greater_to_out_fft_input_sizes_threshold(xs_len, ys_len)
        },
        86,
    )
}

pub fn exhaustive_unsigned_vec_triple_gen_var_22<T: PrimitiveUnsigned>(
) -> It<(Vec<T>, Vec<T>, Vec<T>)> {
    exhaustive_mul_helper(
        &|xs_len, ys_len| {
            _limbs_mul_greater_to_out_toom_32_input_sizes_valid(xs_len, ys_len)
                && _limbs_mul_greater_to_out_toom_43_input_sizes_valid(xs_len, ys_len)
        },
        11,
        8,
    )
}

pub fn exhaustive_unsigned_vec_triple_gen_var_23<T: PrimitiveUnsigned>(
) -> It<(Vec<T>, Vec<T>, Vec<T>)> {
    exhaustive_mul_helper(
        &|xs_len, ys_len| {
            _limbs_mul_greater_to_out_toom_42_input_sizes_valid(xs_len, ys_len)
                && _limbs_mul_greater_to_out_toom_53_input_sizes_valid(xs_len, ys_len)
        },
        5,
        3,
    )
}

// vars 24 through 27 are in malachite-base
