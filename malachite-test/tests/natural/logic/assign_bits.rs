use malachite_base::num::arithmetic::traits::{ModPowerOf2, NegModPowerOf2};
use malachite_base::num::basic::traits::{One, Zero};
use malachite_base::num::logic::traits::{BitBlockAccess, LowMask, SignificantBits};
use malachite_base_test_util::num::logic::bit_block_access::assign_bits_naive;
use malachite_nz::natural::logic::bit_block_access::limbs_assign_bits;
use malachite_nz::natural::Natural;
use malachite_nz::platform::Limb;

use malachite_test::common::test_properties;
use malachite_test::inputs::base::*;
use malachite_test::inputs::natural::{
    naturals, pairs_of_natural_and_small_unsigned,
    quadruples_of_natural_small_unsigned_small_unsigned_and_natural_var_1,
    triples_of_natural_unsigned_and_natural,
};

fn verify_limbs_assign_bits(limbs: &[Limb], start: u64, end: u64, bits: &[Limb], out: &[Limb]) {
    let old_n = Natural::from_limbs_asc(limbs);
    let mut n = old_n.clone();
    let bits = Natural::from_limbs_asc(bits);
    n.assign_bits(start, end, &bits);
    let result = n;
    assert_eq!(Natural::from_limbs_asc(out), result);
    let mut n = old_n.clone();
    assign_bits_naive::<Natural, Natural>(&mut n, start, end, &bits);
    assert_eq!(n, result);
}

#[test]
fn limbs_assign_bits_properties() {
    test_properties(
        quadruples_of_unsigned_vec_small_unsigned_small_unsigned_and_unsigned_vec_var_1,
        |&(ref limbs_in, start, end, ref bits)| {
            let mut limbs = limbs_in.to_vec();
            limbs_assign_bits(&mut limbs, start, end, bits);
            verify_limbs_assign_bits(limbs_in, start, end, bits, &limbs);
        },
    );
}

#[test]
fn assign_bits_properties() {
    test_properties(
        quadruples_of_natural_small_unsigned_small_unsigned_and_natural_var_1,
        |&(ref n, start, end, ref bits)| {
            let old_n = n;
            let mut n = old_n.clone();
            n.assign_bits(start, end, bits);
            let result = n;
            let mut n = old_n.clone();
            assign_bits_naive(&mut n, start, end, bits);
            assert_eq!(n, result);
            n.assign_bits(start, end, bits);
            assert_eq!(n, result);
            let bits_width = end - start;
            assert_eq!(n.get_bits(start, end), bits.mod_power_of_2(bits_width));
            let mut n = !old_n;
            //TODO use sub_mod_power_of_2
            let mut not_bits = bits.neg_mod_power_of_2(bits_width);
            if not_bits == 0 {
                not_bits = Natural::low_mask(bits_width);
            } else {
                not_bits -= Natural::ONE;
            }
            n.assign_bits(start, end, &not_bits);
            assert_eq!(!n, result);
        },
    );

    test_properties(
        triples_of_natural_unsigned_and_natural,
        |&(ref n, start, ref bits)| {
            let old_n = n;
            let mut n = old_n.clone();
            n.assign_bits(start, start, bits);
            assert_eq!(n, *old_n);
        },
    );

    test_properties(pairs_of_natural_and_small_unsigned, |&(ref bits, start)| {
        let mut n = Natural::ZERO;
        n.assign_bits(start, start + bits.significant_bits(), bits);
        assert_eq!(n, bits << start);
    });

    test_properties(naturals, |n| {
        let old_n = n;
        let mut n = old_n.clone();
        n.assign_bits(0, old_n.significant_bits(), &Natural::ZERO);
        assert_eq!(n, 0);

        let mut n = Natural::ZERO;
        n.assign_bits(0, old_n.significant_bits(), old_n);
        assert_eq!(n, *old_n);

        let mut n = old_n.clone();
        n.assign_bits(0, old_n.significant_bits(), old_n);
        assert_eq!(n, *old_n);
    });
}
