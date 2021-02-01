use malachite_base::iterators::bit_distributor::BitDistributorOutputType;
use malachite_base::num::arithmetic::traits::PowerOfTwo;
use malachite_base::num::basic::integers::PrimitiveInt;
use malachite_base::num::basic::traits::Two;
use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base::num::conversion::traits::{ExactFrom, SaturatingFrom, WrappingFrom};
use malachite_base::num::exhaustive::{
    exhaustive_unsigneds, primitive_int_increasing_inclusive_range,
};
use malachite_base::num::iterators::{bit_distributor_sequence, ruler_sequence};
use malachite_base::tuples::exhaustive::{
    exhaustive_dependent_pairs, exhaustive_pairs, ExhaustiveDependentPairsYsGenerator,
};
use malachite_base::vecs::exhaustive::{
    exhaustive_vecs, exhaustive_vecs_length_range, exhaustive_vecs_min_length,
};
use malachite_base_test_util::generators::common::permute_2_1;
use malachite_base_test_util::generators::common::It;
use malachite_base_test_util::generators::exhaustive_pairs_big_tiny;
use malachite_nz::integer::exhaustive::exhaustive_integers;
use malachite_nz::integer::Integer;
use malachite_nz::natural::conversion::digits::general_digits::{
    limbs_digit_count, GET_STR_PRECOMPUTE_THRESHOLD,
};
use malachite_nz::natural::exhaustive::{
    exhaustive_natural_range_to_infinity, exhaustive_naturals,
};
use malachite_nz::natural::Natural;
use malachite_nz::platform::Limb;
use std::iter::once;
use std::marker::PhantomData;

// -- Integer --

pub fn exhaustive_integer_gen() -> It<Integer> {
    Box::new(exhaustive_integers())
}

// -- Natural --

pub fn exhaustive_natural_gen() -> It<Natural> {
    Box::new(exhaustive_naturals())
}

pub fn exhaustive_natural_gen_var_1() -> It<Natural> {
    Box::new(exhaustive_natural_range_to_infinity(Natural::TWO))
}

// -- (Natural, Natural) --

pub fn exhaustive_natural_pair_gen_var_1() -> It<(Natural, Natural)> {
    Box::new(exhaustive_pairs(
        exhaustive_natural_range_to_infinity(Natural::power_of_two(Limb::WIDTH)),
        exhaustive_natural_range_to_infinity(Natural::TWO),
    ))
}

pub fn exhaustive_natural_pair_gen_var_2() -> It<(Natural, Natural)> {
    Box::new(exhaustive_pairs(
        exhaustive_naturals(),
        exhaustive_natural_range_to_infinity(Natural::TWO),
    ))
}

// -- (Natural, PrimitiveInt) --

pub fn natural_primitive_int_pair_gen_var_1<
    T: PrimitiveInt + SaturatingFrom<U>,
    U: PrimitiveInt,
>() -> It<(Natural, T)> {
    Box::new(exhaustive_pairs_big_tiny(
        exhaustive_naturals(),
        primitive_int_increasing_inclusive_range(T::TWO, T::saturating_from(U::MAX)),
    ))
}

pub fn natural_primitive_int_pair_gen_var_2<T: PrimitiveInt>() -> It<(Natural, T)> {
    Box::new(exhaustive_pairs_big_tiny(
        exhaustive_naturals(),
        primitive_int_increasing_inclusive_range(T::TWO, T::MAX),
    ))
}

// -- (Vec<PrimitiveUnsigned>, PrimitiveUnsigned)

pub fn exhaustive_unsigned_vec_unsigned_pair_gen_var_1<
    T: PrimitiveUnsigned + SaturatingFrom<U>,
    U: PrimitiveInt,
>() -> It<(Vec<T>, T)> {
    Box::new(exhaustive_pairs_big_tiny(
        exhaustive_vecs_min_length(2, exhaustive_unsigneds()),
        primitive_int_increasing_inclusive_range(T::TWO, T::saturating_from(U::MAX)),
    ))
}

// -- (Vec<PrimitiveUnsigned>, PrimitiveUnsigned, Vec<PrimitiveUnsigned>)

struct ValidLengthsGenerator<T: PrimitiveUnsigned> {
    phantom: PhantomData<T>,
}

impl<T: PrimitiveUnsigned> ExhaustiveDependentPairsYsGenerator<(Vec<Limb>, u64), Vec<T>, It<Vec<T>>>
    for ValidLengthsGenerator<T>
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
                primitive_int_increasing_inclusive_range::<u64>(3, 256)
                    .filter(|&b| !b.is_power_of_two()),
            ),
            ValidLengthsGenerator {
                phantom: PhantomData,
            },
        )
        .map(|((xs, base), out)| (out, base, xs)),
    )
}

// -- (Vec<PrimitiveUnsigned>, PrimitiveUnsigned, Vec<PrimitiveUnsigned>, PrimitiveUnsigned) --

struct ValidLengthsBasecaseGenerator<T: PrimitiveUnsigned> {
    min_out_len: usize,
    phantom: PhantomData<T>,
}

impl<T: PrimitiveUnsigned> ExhaustiveDependentPairsYsGenerator<usize, Vec<T>, It<Vec<T>>>
    for ValidLengthsBasecaseGenerator<T>
{
    #[inline]
    fn get_ys(&self, &len: &usize) -> It<Vec<T>> {
        Box::new(exhaustive_vecs_min_length(
            u64::exact_from(if len == 0 { self.min_out_len } else { len }),
            exhaustive_unsigneds(),
        ))
    }
}

struct BasecaseDigitsInputGenerator<T: PrimitiveUnsigned> {
    phantom: PhantomData<T>,
}

impl<T: PrimitiveUnsigned>
    ExhaustiveDependentPairsYsGenerator<(Vec<Limb>, u64), (Vec<T>, usize), It<(Vec<T>, usize)>>
    for BasecaseDigitsInputGenerator<T>
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
            ValidLengthsBasecaseGenerator {
                min_out_len,
                phantom: PhantomData,
            },
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
                primitive_int_increasing_inclusive_range::<u64>(3, 256)
                    .filter(|&b| !b.is_power_of_two()),
            ),
            BasecaseDigitsInputGenerator {
                phantom: PhantomData,
            },
        )
        .map(|((xs, base), (out, len))| (out, len, xs, base)),
    )
}
