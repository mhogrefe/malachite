use common::LARGE_LIMIT;
use malachite_native::integer as native;
use malachite_native::traits::Assign as native_assign;
use malachite_gmp::integer as gmp;
use malachite_gmp::traits::Assign as gmp_assign;
use malachite_test::common::{gmp_integer_to_native, native_integer_to_gmp,
                             native_integer_to_num_bigint, native_integer_to_rugint,
                             num_bigint_to_native_integer, rugint_integer_to_native};
use num;
use rugint;
use rugint::Assign as rugint_assign;
use rust_wheels::iterators::common::EXAMPLE_SEED;
use rust_wheels::iterators::integers::{exhaustive_integers, random_integers};
use rust_wheels::iterators::tuples::{exhaustive_pairs_from_single, random_pairs_from_single};
use std::str::FromStr;

#[test]
fn test_assign() {
    let test = |u, v, out| {
        let mut x = native::Integer::from_str(u).unwrap();
        x.assign(&native::Integer::from_str(v).unwrap());
        assert_eq!(x.to_string(), out);
        assert!(x.is_valid());

        let mut x = gmp::Integer::from_str(u).unwrap();
        x.assign(&gmp::Integer::from_str(v).unwrap());
        assert_eq!(x.to_string(), out);
        assert!(x.is_valid());
    };
    test("-123", "456", "456");
    test("-123", "1000000000000", "1000000000000");
    test("1000000000000", "-123", "-123");
    test("1000000000000", "2000000000000", "2000000000000");
}

#[test]
fn test_clone() {
    let test = |u| {
        let x = native::Integer::from_str(u).unwrap().clone();
        assert_eq!(x.to_string(), u);
        assert!(x.is_valid());

        let x = gmp::Integer::from_str(u).unwrap().clone();
        assert_eq!(x.to_string(), u);
        assert!(x.is_valid());

        let x = num::BigInt::from_str(u).unwrap().clone();
        assert_eq!(x.to_string(), u);

        let x = rugint::Integer::from_str(u).unwrap().clone();
        assert_eq!(x.to_string(), u);
    };
    test("123");
    test("1000000000000");
}

#[test]
fn test_clone_from() {
    let test = |u, v, out| {
        let mut x = native::Integer::from_str(u).unwrap();
        x.clone_from(&native::Integer::from_str(v).unwrap());
        assert_eq!(x.to_string(), out);
        assert!(x.is_valid());

        let mut x = gmp::Integer::from_str(u).unwrap();
        x.clone_from(&gmp::Integer::from_str(v).unwrap());
        assert_eq!(x.to_string(), out);
        assert!(x.is_valid());

        let mut x = num::BigInt::from_str(u).unwrap();
        x.clone_from(&num::BigInt::from_str(v).unwrap());
        assert_eq!(x.to_string(), out);

        let mut x = rugint::Integer::from_str(u).unwrap();
        x.clone_from(&rugint::Integer::from_str(v).unwrap());
        assert_eq!(x.to_string(), out);
    };
    test("-123", "456", "456");
    test("-123", "1000000000000", "1000000000000");
    test("1000000000000", "-123", "-123");
    test("1000000000000", "2000000000000", "2000000000000");
}


#[test]
fn clone_and_assign_properties() {
    // x.clone() is equivalent for malachite-gmp, malachite-native, num, and rugint.
    // x.clone() is valid.
    // x.clone() == x
    let one_integer = |gmp_x: gmp::Integer| {
        let x = gmp_integer_to_native(&gmp_x);
        let x_cloned = x.clone();
        assert!(x_cloned.is_valid());
        let gmp_x_cloned = gmp_x.clone();
        assert!(gmp_x_cloned.is_valid());
        assert_eq!(gmp_integer_to_native(&gmp_x_cloned), x_cloned);
        assert_eq!(num_bigint_to_native_integer(&native_integer_to_num_bigint(&x).clone()),
                   x);
        assert_eq!(rugint_integer_to_native(&native_integer_to_rugint(&x).clone()),
                   x);
        assert_eq!(x_cloned, x);
    };

    // x.clone_from(y) is equivalent for malachite-gmp, malachite-native, num, and rugint.
    // x.clone_from(y) is valid.
    // x.clone_from(y); x == y
    // x.assign(y) is equivalent for malachite-gmp, malachite-native, and rugint.
    // x.assign(y) is valid.
    // x.assign(y); x == y
    // x.assign(&y) is equivalent for malachite-gmp, malachite-native, and rugint.
    // x.assign(&y) is valid.
    // x.assign(&y); x == y
    let two_integers = |mut gmp_x: gmp::Integer, gmp_y: gmp::Integer| {
        let mut x = gmp_integer_to_native(&gmp_x);
        let y = gmp_integer_to_native(&gmp_y);
        let old_x = x.clone();
        gmp_x.clone_from(&gmp_y);
        assert!(gmp_x.is_valid());
        assert_eq!(gmp_x, gmp_y);
        x.clone_from(&y);
        assert!(x.is_valid());
        assert_eq!(x, y);
        let mut num_x = native_integer_to_num_bigint(&old_x);
        let num_y = native_integer_to_num_bigint(&y);
        num_x.clone_from(&num_y);
        assert_eq!(num_bigint_to_native_integer(&num_x), y);
        let mut rugint_x = native_integer_to_rugint(&old_x);
        let rugint_y = native_integer_to_rugint(&y);
        rugint_x.clone_from(&rugint_y);
        assert_eq!(rugint_integer_to_native(&rugint_x), y);

        x = old_x.clone();
        gmp_x = native_integer_to_gmp(&old_x);
        gmp_x.assign(gmp_y.clone());
        assert!(gmp_x.is_valid());
        assert_eq!(gmp_x, gmp_y);
        x.assign(y.clone());
        assert!(x.is_valid());
        assert_eq!(x, y);
        let mut rugint_x = native_integer_to_rugint(&old_x);
        let rugint_y = native_integer_to_rugint(&y);
        rugint_x.assign(rugint_y);
        assert_eq!(rugint_integer_to_native(&rugint_x), y);

        x = old_x.clone();
        gmp_x = native_integer_to_gmp(&old_x);
        gmp_x.assign(&gmp_y);
        assert!(gmp_x.is_valid());
        assert_eq!(gmp_x, gmp_y);
        x.assign(&y);
        assert!(x.is_valid());
        assert_eq!(x, y);
        let mut rugint_x = native_integer_to_rugint(&old_x);
        let rugint_y = native_integer_to_rugint(&y);
        rugint_x.assign(&rugint_y);
        assert_eq!(rugint_integer_to_native(&rugint_x), y);
    };

    for n in exhaustive_integers().take(LARGE_LIMIT) {
        one_integer(n);
    }

    for n in random_integers(&EXAMPLE_SEED, 32).take(LARGE_LIMIT) {
        one_integer(n);
    }

    for (x, y) in exhaustive_pairs_from_single(exhaustive_integers()).take(LARGE_LIMIT) {
        two_integers(x, y);
    }

    for (x, y) in random_pairs_from_single(random_integers(&EXAMPLE_SEED, 32)).take(LARGE_LIMIT) {
        two_integers(x, y);
    }
}
