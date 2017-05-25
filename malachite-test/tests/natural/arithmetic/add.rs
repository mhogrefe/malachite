use common::LARGE_LIMIT;
use malachite_native::natural as native;
use malachite_gmp::natural as gmp;
use malachite_test::common::{gmp_to_native, native_to_num, native_to_rugint, num_to_native,
                             rugint_to_native};
use num;
use rugint;
use rust_wheels::iterators::common::EXAMPLE_SEED;
use rust_wheels::iterators::general::random_x;
use rust_wheels::iterators::naturals::{exhaustive_naturals, random_naturals};
use rust_wheels::iterators::primitive_ints::exhaustive_u;
use rust_wheels::iterators::tuples::{exhaustive_pairs, exhaustive_pairs_from_single, random_pairs,
                                     random_pairs_from_single};
use std::str::FromStr;

#[test]
fn test_add_assign() {
    let test = |u, v, out| {
        let mut n = native::Natural::from_str(u).unwrap();
        n += native::Natural::from_str(v).unwrap();
        assert_eq!(n.to_string(), out);
        assert!(n.is_valid());

        let mut n = gmp::Natural::from_str(u).unwrap();
        n += gmp::Natural::from_str(v).unwrap();
        assert_eq!(n.to_string(), out);
        assert!(n.is_valid());
    };
    test("0", "0", "0");
    test("0", "123", "123");
    test("123", "0", "123");
    test("123", "456", "579");
    test("1000000000000", "123", "1000000000123");
    test("123", "1000000000000", "1000000000123");
    test("12345678987654321", "314159265358979", "12659838253013300");
}

#[test]
fn test_add() {
    let test = |u, v, out| {
        let n = native::Natural::from_str(u).unwrap() + native::Natural::from_str(v).unwrap();
        assert_eq!(n.to_string(), out);
        assert!(n.is_valid());

        let n = gmp::Natural::from_str(u).unwrap() + gmp::Natural::from_str(v).unwrap();
        assert_eq!(n.to_string(), out);
        assert!(n.is_valid());

        let n = num::BigUint::from_str(u).unwrap() + num::BigUint::from_str(v).unwrap();
        assert_eq!(n.to_string(), out);

        let n = rugint::Integer::from_str(u).unwrap() + rugint::Integer::from_str(v).unwrap();
        assert_eq!(n.to_string(), out);
    };
    test("0", "0", "0");
    test("0", "123", "123");
    test("123", "0", "123");
    test("123", "456", "579");
    test("1000000000000", "123", "1000000000123");
    test("123", "1000000000000", "1000000000123");
    test("12345678987654321", "314159265358979", "12659838253013300");
}

#[test]
fn add_properties() {
    // x + y is valid.
    // x + y is equivalent for malachite-gmp, malachite-native, num, and rugint.
    // x + y == y + x
    let two_naturals = |gmp_x: gmp::Natural, gmp_y: gmp::Natural| {
        let x = gmp_to_native(&gmp_x);
        let y = gmp_to_native(&gmp_y);
        let raw_gmp_sum = gmp_x + gmp_y;
        assert!(raw_gmp_sum.is_valid());
        let gmp_sum = gmp_to_native(&raw_gmp_sum);
        let num_sum = num_to_native(&(native_to_num(&x) + native_to_num(&y)));
        let rugint_sum = rugint_to_native(&(native_to_rugint(&x) + native_to_rugint(&y)));
        let reverse_sum = y.clone() + x.clone();
        let sum = x + y;
        assert!(sum.is_valid());
        assert_eq!(gmp_sum, sum);
        assert_eq!(num_sum, sum);
        assert_eq!(rugint_sum, sum);
        assert_eq!(reverse_sum, sum);
    };

    // x + (y: u32) == x + from(y)
    // (y: u32) + x == x + from(y)
    let natural_and_u32 = |gmp_x: gmp::Natural, y: u32| {
        let x = gmp_to_native(&gmp_x);
        let primitive_sum_1 = x.clone() + y;
        let primitive_sum_2 = y + x.clone();
        let sum = x + native::Natural::from(y);
        assert_eq!(primitive_sum_1, sum);
        assert_eq!(primitive_sum_2, sum);
    };

    // x + 0 == x
    // 0 + x == x
    // x + x == x << 1
    let one_natural = |gmp_x: gmp::Natural| {
        let x = gmp_to_native(&gmp_x);
        let x_old = x.clone();
        let id_1 = x.clone() + native::Natural::from(0);
        let id_2 = native::Natural::from(0) + x.clone();
        let double = x.clone() + x;
        assert_eq!(id_1, x_old);
        assert_eq!(id_2, x_old);
        assert_eq!(double, x_old << 1);
    };

    for (x, y) in exhaustive_pairs_from_single(exhaustive_naturals()).take(LARGE_LIMIT) {
        two_naturals(x, y);
    }

    for (x, y) in random_pairs_from_single(random_naturals(&EXAMPLE_SEED, 32)).take(LARGE_LIMIT) {
        two_naturals(x, y);
    }

    for (x, y) in exhaustive_pairs(exhaustive_naturals(), exhaustive_u::<u32>()).take(LARGE_LIMIT) {
        natural_and_u32(x, y);
    }

    for (x, y) in random_pairs(&EXAMPLE_SEED,
                               &(|seed| random_naturals(seed, 32)),
                               &(|seed| random_x(seed)))
                .take(LARGE_LIMIT) {
        natural_and_u32(x, y);
    }

    for n in exhaustive_naturals().take(LARGE_LIMIT) {
        one_natural(n);
    }

    for n in random_naturals(&EXAMPLE_SEED, 32).take(LARGE_LIMIT) {
        one_natural(n);
    }

    //TODO inverse of sub

    //TODO associativity
}
