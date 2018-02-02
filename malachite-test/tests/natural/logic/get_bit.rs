use common::LARGE_LIMIT;
use malachite_base::num::{BitAccess, SignificantBits};
use malachite_nz::natural::Natural;
use malachite_test::common::{natural_to_biguint, natural_to_rug_integer, GenerationMode};
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
    // n.get_bit(index) is equivalent for malachite, num, and rug.
    // n.get_bit(index) = !(!n).get_bit(index)
    let natural_and_u64 = |n: Natural, index: u64| {
        let bit = n.get_bit(index);
        assert_eq!(num_get_bit(&natural_to_biguint(&n), index), bit);
        assert_eq!(natural_to_rug_integer(&n).get_bit(index as u32), bit);

        assert_ne!((!n).get_bit(index), bit);
    };

    // !n.get_bit(n.significant_bits())
    // if n != 0, n.get_bit(n.significant_bits() - 1)
    let one_natural = |n: Natural| {
        let significant_bits = n.significant_bits();
        assert!(!n.get_bit(significant_bits));
        if n != 0 {
            assert!(n.get_bit(significant_bits - 1));
        }
    };

    for (n, index) in pairs_of_natural_and_small_u64(GenerationMode::Exhaustive).take(LARGE_LIMIT) {
        natural_and_u64(n, index);
    }

    for (n, index) in pairs_of_natural_and_small_u64(GenerationMode::Random(32)).take(LARGE_LIMIT) {
        natural_and_u64(n, index);
    }

    for n in naturals(GenerationMode::Exhaustive).take(LARGE_LIMIT) {
        one_natural(n);
    }

    for n in naturals(GenerationMode::Random(32)).take(LARGE_LIMIT) {
        one_natural(n);
    }
}
