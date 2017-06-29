use common::LARGE_LIMIT;
use malachite_gmp::natural as gmp;
use malachite_native::natural as native;
use malachite_test::common::gmp_natural_to_native;
use rust_wheels::iterators::common::EXAMPLE_SEED;
use rust_wheels::iterators::naturals::{exhaustive_naturals, random_naturals};
use std::str::FromStr;

#[test]
fn test_is_power_of_two() {
    let test = |n, out| {
        assert_eq!(native::Natural::from_str(n).unwrap().is_power_of_two(), out);
        assert_eq!(gmp::Natural::from_str(n).unwrap().is_power_of_two(), out);
    };
    test("0", false);
    test("1", true);
    test("2", true);
    test("3", false);
    test("4", true);
    test("5", false);
    test("6", false);
    test("7", false);
    test("8", true);
    test("1024", true);
    test("1025", false);
    test("1000000000000", false);
    test("1099511627776", true);
}

#[test]
fn is_power_of_two_properties() {
    // x.is_power_of_two() is equivalent for malachite-gmp and malachite-native.
    // if x != 0, x.is_power_of_two() == (x.trailing_zeros().unwrap() == x.significant_bits() - 1)
    // TODO >> <<
    let one_natural = |gmp_x: gmp::Natural| {
        let x = gmp_natural_to_native(&gmp_x);
        let is_power_of_two = x.is_power_of_two();
        assert_eq!(gmp_x.is_power_of_two(), is_power_of_two);
        if x != 0 {
            assert_eq!(x.trailing_zeros().unwrap() == x.significant_bits() - 1,
                       is_power_of_two);
        }
    };

    for n in exhaustive_naturals().take(LARGE_LIMIT) {
        one_natural(n);
    }

    for n in random_naturals(&EXAMPLE_SEED, 32).take(LARGE_LIMIT) {
        one_natural(n);
    }
}
