use common::test_properties;
use malachite_base::num::{BitAccess, SignificantBits};
use malachite_nz::natural::Natural;
use malachite_test::common::{natural_to_biguint, natural_to_rug_integer};
use malachite_test::inputs::natural::{naturals, pairs_of_natural_and_small_u64};
use malachite_test::natural::logic::get_bit::num_get_bit;
use num::BigUint;
use rug;
use std::str::FromStr;

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
fn get_bit_properties() {
    test_properties(pairs_of_natural_and_small_u64, |&(ref n, index)| {
        let bit = n.get_bit(index);
        assert_eq!(num_get_bit(&natural_to_biguint(n), index), bit);
        assert_eq!(natural_to_rug_integer(n).get_bit(index as u32), bit);

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
