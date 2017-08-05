use common::LARGE_LIMIT;
use malachite_native::natural as native;
use malachite_gmp::natural as gmp;
use malachite_test::common::{gmp_natural_to_native, native_natural_to_num_biguint,
                             native_natural_to_rugint_integer};
use num;
use rugint;
use rust_wheels::iterators::common::EXAMPLE_SEED;
use rust_wheels::iterators::naturals::{exhaustive_naturals, random_naturals};
use std::str::FromStr;

#[test]
fn test_significant_bits() {
    let test = |n, out| {
        assert_eq!(
            native::Natural::from_str(n).unwrap().significant_bits(),
            out
        );
        assert_eq!(gmp::Natural::from_str(n).unwrap().significant_bits(), out);
        assert_eq!(num::BigUint::from_str(n).unwrap().bits() as u64, out);
        assert_eq!(
            rugint::Integer::from_str(n).unwrap().significant_bits() as u64,
            out
        );
    };
    test("0", 0);
    test("100", 7);
    test("1000000000000", 40);
    test("4294967295", 32);
    test("4294967296", 33);
    test("18446744073709551615", 64);
    test("18446744073709551616", 65);
}

#[test]
fn significant_bits_properties() {
    // x.significant_bits() is equivalent for malachite-gmp, malachite-native, num, and rugint.
    // (x < 2^32) == (x.significant_bits() <= 32)
    // if x != 0, (x.significant_bits() == n) == (2^(n-1) <= x < 2^n)
    let one_natural = |gmp_x: gmp::Natural| {
        let x = gmp_natural_to_native(&gmp_x);
        let significant_bits = x.significant_bits();
        assert_eq!(gmp_x.significant_bits(), significant_bits);
        assert_eq!(
            native_natural_to_num_biguint(&x).bits() as u64,
            significant_bits
        );
        assert_eq!(
            native_natural_to_rugint_integer(&x).significant_bits() as u64,
            significant_bits
        );
        assert_eq!(x <= u32::max_value(), significant_bits <= 32);
        if x != 0 {
            let n = significant_bits as u32;
            assert!(native::Natural::from(1u32) << (n - 1) <= x);
            assert!(x < native::Natural::from(1u32) << n);
        }
    };

    for n in exhaustive_naturals().take(LARGE_LIMIT) {
        one_natural(n);
    }

    for n in random_naturals(&EXAMPLE_SEED, 32).take(LARGE_LIMIT) {
        one_natural(n);
    }
}
