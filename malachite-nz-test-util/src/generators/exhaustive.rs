use malachite_base::iterators::bit_distributor::BitDistributorOutputType;
use malachite_base::num::conversion::traits::{ExactFrom, WrappingFrom};
use malachite_base::num::exhaustive::{
    exhaustive_unsigneds, primitive_int_increasing_inclusive_range,
};
use malachite_base::num::iterators::{bit_distributor_sequence, ruler_sequence};
use malachite_base::tuples::exhaustive::{
    exhaustive_dependent_pairs, ExhaustiveDependentPairsYsGenerator,
};
use malachite_base::vecs::exhaustive::{
    exhaustive_vecs, exhaustive_vecs_length_range, exhaustive_vecs_min_length,
};
use malachite_base_test_util::generators::common::permute_2_1;
use malachite_base_test_util::generators::common::It;
use malachite_base_test_util::generators::exhaustive_pairs_big_tiny;
use malachite_nz::natural::conversion::digits::general_digits::{
    limbs_digit_count, GET_STR_PRECOMPUTE_THRESHOLD,
};
use malachite_nz::natural::exhaustive::exhaustive_naturals;
use malachite_nz::natural::Natural;
use malachite_nz::platform::Limb;
use std::iter::once;

// -- Natural --

pub fn exhaustive_natural_gen() -> It<Natural> {
    Box::new(exhaustive_naturals())
}

// -- (Vec<PrimitiveUnsigned>, PrimitiveUnsigned, Vec<PrimitiveUnsigned>)

struct ValidLengthsGenerator;

impl ExhaustiveDependentPairsYsGenerator<(Vec<Limb>, u64), Vec<u8>, It<Vec<u8>>>
    for ValidLengthsGenerator
{
    #[inline]
    fn get_ys(&self, p: &(Vec<Limb>, u64)) -> It<Vec<u8>> {
        Box::new(exhaustive_vecs_min_length(
            limbs_digit_count(&p.0, p.1),
            exhaustive_unsigneds(),
        ))
    }
}

pub fn exhaustive_unsigned_vec_unsigned_unsigned_vec_triple_gen_var_1(
) -> It<(Vec<u8>, u64, Vec<Limb>)> {
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
            ValidLengthsGenerator,
        )
        .map(|((xs, base), out)| (out, base, xs)),
    )
}

// -- (Vec<PrimitiveUnsigned>, PrimitiveUnsigned, Vec<PrimitiveUnsigned>, PrimitiveUnsigned) --

struct ValidLengthsBasecaseGenerator {
    min_out_len: usize,
}

impl ExhaustiveDependentPairsYsGenerator<usize, Vec<u8>, It<Vec<u8>>>
    for ValidLengthsBasecaseGenerator
{
    #[inline]
    fn get_ys(&self, &len: &usize) -> It<Vec<u8>> {
        Box::new(exhaustive_vecs_min_length(
            u64::exact_from(if len == 0 { self.min_out_len } else { len }),
            exhaustive_unsigneds(),
        ))
    }
}

struct BasecaseDigitsInputGenerator;

impl ExhaustiveDependentPairsYsGenerator<(Vec<Limb>, u64), (Vec<u8>, usize), It<(Vec<u8>, usize)>>
    for BasecaseDigitsInputGenerator
{
    #[inline]
    fn get_ys(&self, p: &(Vec<Limb>, u64)) -> It<(Vec<u8>, usize)> {
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

pub fn exhaustive_unsigned_vec_unsigned_unsigned_vec_unsigned_quadruple_gen_var_1(
) -> It<(Vec<u8>, usize, Vec<Limb>, u64)> {
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
            BasecaseDigitsInputGenerator,
        )
        .map(|((xs, base), (out, len))| (out, len, xs, base)),
    )
}
