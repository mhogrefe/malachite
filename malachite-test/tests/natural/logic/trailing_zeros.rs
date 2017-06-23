use common::LARGE_LIMIT;
use malachite_gmp::natural as gmp;
use malachite_native::natural as native;
use malachite_test::common::gmp_natural_to_native;
use rust_wheels::iterators::common::EXAMPLE_SEED;
use rust_wheels::iterators::naturals::{exhaustive_naturals, random_naturals};
use std::str::FromStr;

#[test]
fn test_trailing_zeros() {
    let test = |n, out| {
        assert_eq!(native::Natural::from_str(n).unwrap().trailing_zeros(), out);
        assert_eq!(gmp::Natural::from_str(n).unwrap().trailing_zeros(), out);
    };
    test("0", None);
    test("123", Some(0));
    test("1000000000000", Some(12));
    test("4294967295", Some(0));
    test("4294967296", Some(32));
    test("18446744073709551615", Some(0));
    test("18446744073709551616", Some(64));
}

#[test]
fn trailing_zeros_properties() {
    // x.trailing_zeros() is equivalent for malachite-gmp and malachite-native.
    // x.trailing_zeros().is_none() == (x == 0)
    // TODO >> <<
    let one_natural = |gmp_x: gmp::Natural| {
        let x = gmp_natural_to_native(&gmp_x);
        let trailing_zeros = x.trailing_zeros();
        assert_eq!(gmp_x.trailing_zeros(), trailing_zeros);
        assert_eq!(trailing_zeros.is_none(), x == 0);
    };

    for n in exhaustive_naturals().take(LARGE_LIMIT) {
        one_natural(n);
    }

    for n in random_naturals(&EXAMPLE_SEED, 32).take(LARGE_LIMIT) {
        one_natural(n);
    }
}
