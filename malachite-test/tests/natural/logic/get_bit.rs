use std::str::FromStr;

use malachite_base::num::basic::traits::One;
use malachite_base::num::conversion::traits::ExactFrom;
use malachite_base::num::logic::traits::{BitAccess, SignificantBits};
use malachite_nz::natural::logic::bit_access::limbs_get_bit;
use malachite_nz::natural::Natural;
use malachite_nz::platform::Limb;
use num::BigUint;
use rug;

use malachite_test::common::test_properties;
use malachite_test::common::{natural_to_biguint, natural_to_rug_integer};
use malachite_test::inputs::base::{
    pairs_of_unsigned_and_small_unsigned, pairs_of_unsigned_vec_and_small_unsigned,
};
use malachite_test::inputs::natural::{naturals, pairs_of_natural_and_small_unsigned};
use malachite_test::natural::logic::get_bit::num_get_bit;

#[cfg(feature = "32_bit_limbs")]
#[test]
fn test_limbs_get_bit() {
    let test = |limbs: &[Limb], index: u64, out: bool| {
        assert_eq!(limbs_get_bit(limbs, index), out);
    };
    test(&[1], 0, true);
    test(&[1], 100, false);
    test(&[123], 2, false);
    test(&[123], 3, true);
    test(&[123], 100, false);
    test(&[0, 0b1011], 0, false);
    test(&[0, 0b1011], 32, true);
    test(&[0, 0b1011], 33, true);
    test(&[0, 0b1011], 34, false);
    test(&[0, 0b1011], 35, true);
    test(&[0, 0b1011], 100, false);
}

#[test]
fn test_get_bit() {
    let test = |n, index, out| {
        assert_eq!(Natural::from_str(n).unwrap().get_bit(index), out);
        assert_eq!(num_get_bit(&BigUint::from_str(n).unwrap(), index), out);
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
    test("1000000000000", 12, true);
    test("1000000000000", 100, false);
}

#[test]
fn limbs_get_bit_properties() {
    test_properties(
        pairs_of_unsigned_vec_and_small_unsigned,
        |&(ref limbs, index)| {
            assert_eq!(
                Natural::from_limbs_asc(limbs).get_bit(index),
                limbs_get_bit(limbs, index)
            );
        },
    );
}

#[test]
fn get_bit_properties() {
    test_properties(pairs_of_natural_and_small_unsigned, |&(ref n, index)| {
        let bit = n.get_bit(index);
        assert_eq!(num_get_bit(&natural_to_biguint(n), index), bit);
        assert_eq!(
            natural_to_rug_integer(n).get_bit(u32::exact_from(index)),
            bit
        );

        assert_eq!(n & (Natural::ONE << index) != 0, bit);
        assert_ne!((!n).get_bit(index), bit);
    });

    test_properties(naturals, |n| {
        let significant_bits = n.significant_bits();
        assert!(!n.get_bit(significant_bits));
        if *n != 0 {
            assert!(n.get_bit(significant_bits - 1));
        }
    });

    test_properties(
        pairs_of_unsigned_and_small_unsigned::<Limb, u64>,
        |&(u, index)| {
            assert_eq!(Natural::from(u).get_bit(index), u.get_bit(index));
        },
    );
}
