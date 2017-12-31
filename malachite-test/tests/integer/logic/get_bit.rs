use common::LARGE_LIMIT;
use malachite_native::integer as native;
use malachite_gmp::integer as gmp;
use malachite_test::common::{gmp_integer_to_native, native_integer_to_rugint, GenerationMode};
use malachite_test::integer::logic::get_bit::select_inputs;
use rugint;
use rust_wheels::iterators::common::EXAMPLE_SEED;
use rust_wheels::iterators::integers::{exhaustive_natural_integers, random_natural_integers};
use std::str::FromStr;

#[test]
pub fn test_get_bit() {
    let test = |n, index, out| {
        assert_eq!(native::Integer::from_str(n).unwrap().get_bit(index), out);
        assert_eq!(gmp::Integer::from_str(n).unwrap().get_bit(index), out);
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
    // n.get_bit(index) is equivalent for malachite-gmp, malachite-native, num, and rugint.
    // !(!n).get_bit(index) == n.get_bit_index()
    let integer_and_u64 = |gmp_n: gmp::Integer, index: u64| {
        let n = gmp_integer_to_native(&gmp_n);
        let bit = n.get_bit(index);
        assert_eq!(gmp_n.get_bit(index), bit);
        assert_eq!(native_integer_to_rugint(&n).get_bit(index as u32), bit);

        assert_eq!(!(!n).get_bit(index), bit);
    };

    // if n >= 0, !n.get_bit(n.significant_bits())
    // if n > 0, n.get_bit(n.significant_bits() - 1)
    let one_natural_integer = |gmp_n: gmp::Integer| {
        let n = gmp_integer_to_native(&gmp_n);
        let significant_bits = n.significant_bits();
        assert!(!n.get_bit(significant_bits));
        if n != 0 {
            assert!(n.get_bit(significant_bits - 1));
        }
    };

    for (n, index) in select_inputs(GenerationMode::Exhaustive).take(LARGE_LIMIT) {
        integer_and_u64(n, index);
    }

    for (n, index) in select_inputs(GenerationMode::Random(32)).take(LARGE_LIMIT) {
        integer_and_u64(n, index);
    }

    for n in exhaustive_natural_integers().take(LARGE_LIMIT) {
        one_natural_integer(n);
    }

    for n in random_natural_integers(&EXAMPLE_SEED, 32).take(LARGE_LIMIT) {
        one_natural_integer(n);
    }
}
