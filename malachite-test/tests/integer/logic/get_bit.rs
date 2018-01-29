use common::LARGE_LIMIT;
use malachite_base::num::{BitAccess, SignificantBits};
use malachite_nz::integer::Integer;
use malachite_test::common::{integer_to_rugint_integer, GenerationMode};
use malachite_test::inputs::integer::{natural_integers, pairs_of_integer_and_small_u64};
use rugint;
use std::str::FromStr;

#[test]
pub fn test_get_bit() {
    let test = |n, index, out| {
        assert_eq!(Integer::from_str(n).unwrap().get_bit(index), out);
        assert_eq!(
            rugint::Integer::from_str(n).unwrap().get_bit(index as u32),
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
fn get_bit_properties() {
    // n.get_bit(index) is equivalent for malachite, num, and rugint.
    // !(!n).get_bit(index) == n.get_bit_index()
    let integer_and_u64 = |n: Integer, index: u64| {
        let bit = n.get_bit(index);
        assert_eq!(integer_to_rugint_integer(&n).get_bit(index as u32), bit);

        assert_eq!(!(!n).get_bit(index), bit);
    };

    // if n >= 0, !n.get_bit(n.significant_bits())
    // if n > 0, n.get_bit(n.significant_bits() - 1)
    let one_natural_integer = |n: Integer| {
        let significant_bits = n.significant_bits();
        assert!(!n.get_bit(significant_bits));
        if n != 0 {
            assert!(n.get_bit(significant_bits - 1));
        }
    };

    for (n, index) in pairs_of_integer_and_small_u64(GenerationMode::Exhaustive).take(LARGE_LIMIT) {
        integer_and_u64(n, index);
    }

    for (n, index) in pairs_of_integer_and_small_u64(GenerationMode::Random(32)).take(LARGE_LIMIT) {
        integer_and_u64(n, index);
    }

    for n in natural_integers(GenerationMode::Exhaustive).take(LARGE_LIMIT) {
        one_natural_integer(n);
    }

    for n in natural_integers(GenerationMode::Random(32)).take(LARGE_LIMIT) {
        one_natural_integer(n);
    }
}
