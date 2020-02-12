use std::str::FromStr;

use malachite_base::num::basic::traits::One;
use malachite_base::num::conversion::traits::ExactFrom;
use malachite_base::num::logic::traits::{BitAccess, SignificantBits};
use malachite_nz::integer::logic::bit_access::limbs_get_bit_neg;
use malachite_nz::integer::Integer;
use malachite_nz::natural::Natural;
#[cfg(feature = "32_bit_limbs")]
use malachite_nz::platform::Limb;
use malachite_nz::platform::SignedLimb;
use rug;

use malachite_test::common::integer_to_rug_integer;
use malachite_test::common::test_properties;
use malachite_test::inputs::base::{
    pairs_of_signed_and_small_unsigned, pairs_of_unsigned_vec_and_small_unsigned_var_1,
};
use malachite_test::inputs::integer::{natural_integers, pairs_of_integer_and_small_unsigned};

#[cfg(feature = "32_bit_limbs")]
#[test]
fn test_limbs_get_bit_neg() {
    let test = |limbs: &[Limb], index: u64, out: bool| {
        assert_eq!(limbs_get_bit_neg(limbs, index), out);
    };
    test(&[1], 0, true);
    test(&[1], 100, true);
    test(&[123], 2, true);
    test(&[123], 3, false);
    test(&[123], 100, true);
    test(&[0, 0b1011], 0, false);
    test(&[0, 0b1011], 32, true);
    test(&[0, 0b1011], 33, false);
    test(&[0, 0b1011], 34, true);
    test(&[0, 0b1011], 35, false);
    test(&[0, 0b1011], 100, true);
    test(&[1, 0b1011], 0, true);
    test(&[1, 0b1011], 32, false);
    test(&[1, 0b1011], 33, false);
    test(&[1, 0b1011], 34, true);
    test(&[1, 0b1011], 35, false);
    test(&[1, 0b1011], 100, true);
}

#[test]
fn test_get_bit() {
    let test = |n, index, out| {
        assert_eq!(Integer::from_str(n).unwrap().get_bit(index), out);
        assert_eq!(
            rug::Integer::from_str(n)
                .unwrap()
                .get_bit(u32::exact_from(index)),
            out
        );
    };

    test("0", 0, false);
    test("0", 100, false);
    test("123", 2, false);
    test("123", 3, true);
    test("123", 100, false);
    test("-123", 0, true);
    test("-123", 1, false);
    test("-123", 100, true);
    test("1000000000000", 12, true);
    test("1000000000000", 100, false);
    test("-1000000000000", 12, true);
    test("-1000000000000", 100, true);
    test("4294967295", 31, true);
    test("4294967295", 32, false);
    test("4294967296", 31, false);
    test("4294967296", 32, true);
    test("4294967296", 33, false);
    test("-4294967295", 0, true);
    test("-4294967295", 1, false);
    test("-4294967295", 31, false);
    test("-4294967295", 32, true);
    test("-4294967295", 33, true);
    test("-4294967296", 0, false);
    test("-4294967296", 31, false);
    test("-4294967296", 32, true);
    test("-4294967296", 33, true);
}

#[test]
fn limbs_get_bit_neg_properties() {
    test_properties(
        pairs_of_unsigned_vec_and_small_unsigned_var_1,
        |&(ref limbs, index)| {
            assert_eq!(
                (-Natural::from_limbs_asc(limbs)).get_bit(index),
                limbs_get_bit_neg(limbs, index)
            );
        },
    );
}

#[test]
fn get_bit_properties() {
    test_properties(pairs_of_integer_and_small_unsigned, |&(ref n, index)| {
        let bit = n.get_bit(index);
        assert_eq!(
            integer_to_rug_integer(n).get_bit(u32::exact_from(index)),
            bit
        );

        assert_eq!(n & (Integer::ONE << index) != 0, bit);
        assert_eq!(!(!n).get_bit(index), bit);
    });

    test_properties(natural_integers, |n| {
        let significant_bits = n.significant_bits();
        assert!(!n.get_bit(significant_bits));
        if *n != 0 {
            assert!(n.get_bit(significant_bits - 1));
        }
    });

    test_properties(
        pairs_of_signed_and_small_unsigned::<SignedLimb, u64>,
        |&(i, index)| {
            assert_eq!(Integer::from(i).get_bit(index), i.get_bit(index));
        },
    );
}
