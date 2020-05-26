use malachite_base::num::basic::traits::Zero;
use malachite_base::num::logic::traits::{BitBlockAccess, LowMask, SignificantBits};
use malachite_base_test_util::num::logic::bit_block_access::_get_bits_naive;
use malachite_nz::integer::logic::bit_block_access::{
    limbs_neg_limb_get_bits, limbs_slice_neg_get_bits, limbs_vec_neg_get_bits,
};
use malachite_nz::integer::Integer;
use malachite_nz::natural::Natural;
use malachite_nz::platform::{Limb, SignedLimb};

use malachite_test::common::test_properties;
use malachite_test::inputs::base::{
    pairs_of_unsigneds_var_3, triples_of_limb_vec_small_unsigned_and_small_unsigned_var_2,
    triples_of_positive_unsigned_small_unsigned_and_small_unsigned_var_1,
    triples_of_signed_small_unsigned_and_small_unsigned_var_1,
};
use malachite_test::inputs::integer::{
    pairs_of_integer_and_small_unsigned, triples_of_integer_small_unsigned_and_small_unsigned_var_1,
};
use malachite_test::inputs::natural::triples_of_natural_small_unsigned_and_small_unsigned_var_1;

fn verify_limbs_neg_limb_get_bits(limb: Limb, start: u64, end: u64, out: &[Limb]) {
    let n = -Natural::from(limb);
    let result = n.get_bits(start, end);
    assert_eq!(_get_bits_naive::<Integer, Natural>(&n, start, end), result);
    assert_eq!(Natural::from_limbs_asc(out), result);
}

fn verify_limbs_neg_get_bits(limbs: &[Limb], start: u64, end: u64, out: &[Limb]) {
    let n = -Natural::from_limbs_asc(limbs);
    let result = n.get_bits(start, end);
    assert_eq!(_get_bits_naive::<Integer, Natural>(&n, start, end), result);
    assert_eq!(Natural::from_limbs_asc(out), result);
}

#[test]
fn limbs_neg_limb_get_bits_properties() {
    test_properties(
        triples_of_positive_unsigned_small_unsigned_and_small_unsigned_var_1,
        |&(limb, start, end)| {
            let out = limbs_neg_limb_get_bits(limb, start, end);
            verify_limbs_neg_limb_get_bits(limb, start, end, &out);
        },
    );
}

#[test]
fn limbs_neg_get_bits_properties() {
    test_properties(
        triples_of_limb_vec_small_unsigned_and_small_unsigned_var_2,
        |&(ref limbs, start, end)| {
            let out = limbs_slice_neg_get_bits(limbs, start, end);
            verify_limbs_neg_get_bits(limbs, start, end, &out);
            let out = limbs_vec_neg_get_bits(limbs.to_vec(), start, end);
            verify_limbs_neg_get_bits(limbs, start, end, &out);
        },
    );
}

#[test]
fn get_bits_properties() {
    test_properties(
        triples_of_integer_small_unsigned_and_small_unsigned_var_1,
        |&(ref n, start, end)| {
            let bits = n.get_bits(start, end);
            assert_eq!(n.clone().get_bits_owned(start, end), bits);
            assert_eq!(_get_bits_naive::<Integer, Natural>(n, start, end), bits);
            let significant_bits = n.significant_bits();
            assert_eq!(
                n.get_bits(start + significant_bits, end + significant_bits),
                if *n >= 0 {
                    Natural::ZERO
                } else {
                    Natural::low_mask(end - start)
                }
            );
            assert_eq!(
                (!n).get_bits(start, end),
                Natural::low_mask(end - start) - &bits
            );
            let mut mut_n = n.clone();
            mut_n.assign_bits(start, end, &bits);
            assert_eq!(*n, mut_n);
        },
    );

    test_properties(pairs_of_integer_and_small_unsigned, |&(ref n, start)| {
        assert_eq!(n.get_bits(start, start), 0);
    });

    test_properties(pairs_of_unsigneds_var_3, |&(start, end)| {
        assert_eq!(Integer::ZERO.get_bits(start, end), 0);
    });

    test_properties(
        triples_of_natural_small_unsigned_and_small_unsigned_var_1,
        |&(ref n, start, end)| {
            assert_eq!(
                Integer::from(n).get_bits(start, end),
                n.get_bits(start, end)
            );
        },
    );

    test_properties(
        triples_of_signed_small_unsigned_and_small_unsigned_var_1::<SignedLimb, u64>,
        |&(n, start, end)| {
            assert_eq!(
                Integer::from(n).get_bits(start, end),
                n.get_bits(start, end)
            );
        },
    );
}
