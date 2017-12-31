use common::LARGE_LIMIT;
use malachite_native::natural as native;
use malachite_gmp::natural as gmp;
use malachite_test::common::{gmp_natural_to_native, native_natural_to_num_biguint,
                             native_natural_to_rugint_integer, GenerationMode};
use malachite_test::natural::logic::get_bit::{num_get_bit, select_inputs};
use num;
use rugint;
use rust_wheels::iterators::common::EXAMPLE_SEED;
use rust_wheels::iterators::naturals::{exhaustive_naturals, random_naturals};
use std::str::FromStr;

#[test]
pub fn test_get_bit() {
    let test = |n, index, out| {
        assert_eq!(native::Natural::from_str(n).unwrap().get_bit(index), out);
        assert_eq!(gmp::Natural::from_str(n).unwrap().get_bit(index), out);
        assert_eq!(num_get_bit(&num::BigUint::from_str(n).unwrap(), index), out);
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
    test("1000000000000", 12, true);
    test("1000000000000", 100, false);
}

#[test]
fn get_bit_properties() {
    // n.get_bit(index) is equivalent for malachite-gmp, malachite-native, num, and rugint.
    let natural_and_u64 = |gmp_n: gmp::Natural, index: u64| {
        let n = gmp_natural_to_native(&gmp_n);
        let bit = n.get_bit(index);
        assert_eq!(gmp_n.get_bit(index), bit);
        assert_eq!(num_get_bit(&native_natural_to_num_biguint(&n), index), bit);
        assert_eq!(
            native_natural_to_rugint_integer(&n).get_bit(index as u32),
            bit
        );
    };

    // !n.get_bit(n.significant_bits())
    // if n != 0, n.get_bit(n.significant_bits() - 1)
    let one_natural = |gmp_n: gmp::Natural| {
        let n = gmp_natural_to_native(&gmp_n);
        let significant_bits = n.significant_bits();
        assert!(!n.get_bit(significant_bits));
        if n != 0 {
            assert!(n.get_bit(significant_bits - 1));
        }
    };

    for (n, index) in select_inputs(GenerationMode::Exhaustive).take(LARGE_LIMIT) {
        natural_and_u64(n, index);
    }

    for (n, index) in select_inputs(GenerationMode::Random(32)).take(LARGE_LIMIT) {
        natural_and_u64(n, index);
    }

    for n in exhaustive_naturals().take(LARGE_LIMIT) {
        one_natural(n);
    }

    for n in random_naturals(&EXAMPLE_SEED, 32).take(LARGE_LIMIT) {
        one_natural(n);
    }
}
