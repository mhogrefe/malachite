use common::test_properties;
use malachite_base::misc::CheckedFrom;
use malachite_base::num::{BitAccess, One, SignificantBits};
use malachite_nz::natural::logic::bit_access::limbs_get_bit;
use malachite_nz::natural::Natural;
use malachite_test::common::{natural_to_biguint, natural_to_rug_integer};
use malachite_test::inputs::base::pairs_of_unsigned_vec_and_small_u64;
use malachite_test::inputs::natural::{naturals, pairs_of_natural_and_small_u64};
use malachite_test::natural::logic::get_bit::num_get_bit;
use num::BigUint;
use rug;
use std::str::FromStr;

#[test]
pub fn test_limbs_get_bit() {
    let test = |limbs: &[u32], index: u64, out: bool| {
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
pub fn test_get_bit() {
    let test = |n, index, out| {
        assert_eq!(Natural::from_str(n).unwrap().get_bit(index), out);
        assert_eq!(num_get_bit(&BigUint::from_str(n).unwrap(), index), out);
        assert_eq!(
            rug::Integer::from_str(n).unwrap().get_bit(index as u32),
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
        pairs_of_unsigned_vec_and_small_u64,
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
    test_properties(pairs_of_natural_and_small_u64, |&(ref n, index)| {
        let bit = n.get_bit(index);
        assert_eq!(num_get_bit(&natural_to_biguint(n), index), bit);
        assert_eq!(natural_to_rug_integer(n).get_bit(index as u32), bit);

        assert_eq!(
            n & (Natural::ONE << u32::checked_from(index).unwrap()) != 0,
            bit
        );
        assert_ne!((!n).get_bit(index), bit);
    });

    test_properties(naturals, |n| {
        let significant_bits = n.significant_bits();
        assert!(!n.get_bit(significant_bits));
        if *n != 0 {
            assert!(n.get_bit(significant_bits - 1));
        }
    });
}
