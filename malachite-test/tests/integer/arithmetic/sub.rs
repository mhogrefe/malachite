use common::LARGE_LIMIT;
use malachite_base::traits::Zero;
use malachite_native::integer as native;
use malachite_gmp::integer as gmp;
use malachite_test::common::{gmp_integer_to_native, native_integer_to_num_bigint,
                             native_integer_to_rugint, num_bigint_to_native_integer,
                             rugint_integer_to_native};
use num;
use rugint;
use rust_wheels::iterators::common::EXAMPLE_SEED;
use rust_wheels::iterators::general::random_x;
use rust_wheels::iterators::integers::{exhaustive_integers, random_integers};
use rust_wheels::iterators::primitive_ints::exhaustive_i;
use rust_wheels::iterators::tuples::{exhaustive_pairs, exhaustive_pairs_from_single, random_pairs,
                                     random_pairs_from_single};
use std::str::FromStr;

#[test]
fn test_sub() {
    let test = |u, v, out| {
        let mut n = native::Integer::from_str(u).unwrap();
        n -= native::Integer::from_str(v).unwrap();
        assert_eq!(n.to_string(), out);
        assert!(n.is_valid());

        let mut n = native::Integer::from_str(u).unwrap();
        n -= &native::Integer::from_str(v).unwrap();
        assert_eq!(n.to_string(), out);
        assert!(n.is_valid());

        let mut n = gmp::Integer::from_str(u).unwrap();
        n -= gmp::Integer::from_str(v).unwrap();
        assert_eq!(n.to_string(), out);
        assert!(n.is_valid());

        let mut n = gmp::Integer::from_str(u).unwrap();
        n -= &gmp::Integer::from_str(v).unwrap();
        assert_eq!(n.to_string(), out);
        assert!(n.is_valid());

        let n = native::Integer::from_str(u).unwrap() - native::Integer::from_str(v).unwrap();
        assert_eq!(n.to_string(), out);
        assert!(n.is_valid());

        let n = &native::Integer::from_str(u).unwrap() - native::Integer::from_str(v).unwrap();
        assert_eq!(n.to_string(), out);
        assert!(n.is_valid());

        let n = native::Integer::from_str(u).unwrap() - &native::Integer::from_str(v).unwrap();
        assert_eq!(n.to_string(), out);
        assert!(n.is_valid());

        let n = &native::Integer::from_str(u).unwrap() - &native::Integer::from_str(v).unwrap();
        assert_eq!(n.to_string(), out);
        assert!(n.is_valid());

        let n = gmp::Integer::from_str(u).unwrap() - gmp::Integer::from_str(v).unwrap();
        assert_eq!(n.to_string(), out);
        assert!(n.is_valid());

        let n = gmp::Integer::from_str(u).unwrap() - &gmp::Integer::from_str(v).unwrap();
        assert_eq!(n.to_string(), out);
        assert!(n.is_valid());

        let n = &gmp::Integer::from_str(u).unwrap() - gmp::Integer::from_str(v).unwrap();
        assert_eq!(n.to_string(), out);
        assert!(n.is_valid());

        let n = &gmp::Integer::from_str(u).unwrap() - &gmp::Integer::from_str(v).unwrap();
        assert_eq!(n.to_string(), out);
        assert!(n.is_valid());

        let n = num::BigInt::from_str(u).unwrap() - num::BigInt::from_str(v).unwrap();
        assert_eq!(n.to_string(), out);

        let n = rugint::Integer::from_str(u).unwrap() - rugint::Integer::from_str(v).unwrap();
        assert_eq!(n.to_string(), out);
    };
    test("0", "0", "0");
    test("0", "-123", "123");
    test("123", "0", "123");
    test("123", "-456", "579");
    test("1000000000000", "-123", "1000000000123");
    test("123", "-1000000000000", "1000000000123");
    test("12345678987654321", "-314159265358979", "12659838253013300");
    test("0", "123", "-123");
    test("123", "123", "0");
    test("123", "456", "-333");
    test("1000000000000", "123", "999999999877");
    test("123", "1000000000000", "-999999999877");
    test("12345678987654321", "314159265358979", "12031519722295342");
}

#[test]
fn sub_properties() {
    // x - y is valid.
    // x - &y is valid.
    // &x - y is valid.
    // &x - &y is valid.
    // x - y is equivalent for malachite-gmp, malachite-native, num, and rugint.
    // x -= y, x -= &y, x - y, x - &y, &x - y, and &x - &y give the same result.
    // x - y == -(y - x)
    #[allow(unknown_lints, cyclomatic_complexity)]
    let two_integers = |gmp_x: gmp::Integer, gmp_y: gmp::Integer| {
        let x = gmp_integer_to_native(&gmp_x);
        let y = gmp_integer_to_native(&gmp_y);
        let raw_gmp_difference = gmp_x.clone() - gmp_y.clone();
        assert!(raw_gmp_difference.is_valid());
        let gmp_difference = gmp_integer_to_native(&raw_gmp_difference);
        let num_difference = num_bigint_to_native_integer(
            &(native_integer_to_num_bigint(&x) - native_integer_to_num_bigint(&y)),
        );
        let rugint_difference = rugint_integer_to_native(
            &(native_integer_to_rugint(&x) - native_integer_to_rugint(&y)),
        );

        let difference_val_val = gmp_x.clone() - gmp_y.clone();
        let difference_val_ref = gmp_x.clone() - &gmp_y;
        let difference_ref_val = &gmp_x - gmp_y.clone();
        assert!(difference_val_val.is_valid());
        assert!(difference_val_ref.is_valid());
        assert!(difference_ref_val.is_valid());
        assert_eq!(difference_val_val, raw_gmp_difference);
        assert_eq!(difference_val_ref, raw_gmp_difference);
        assert_eq!(difference_ref_val, raw_gmp_difference);

        let difference_val_val = x.clone() - y.clone();
        let difference_val_ref = x.clone() - &y;
        let difference_ref_val = &x - y.clone();
        let difference = &x - &y;
        assert!(difference_val_val.is_valid());
        assert!(difference_val_ref.is_valid());
        assert!(difference_ref_val.is_valid());
        assert!(difference.is_valid());
        assert_eq!(difference_val_val, difference);
        assert_eq!(difference_val_ref, difference);
        assert_eq!(difference_ref_val, difference);

        let mut mut_x = x.clone();
        mut_x -= y.clone();
        assert!(mut_x.is_valid());
        assert_eq!(mut_x, difference);
        let mut mut_x = x.clone();
        mut_x -= &y;
        assert_eq!(mut_x, difference);
        assert!(mut_x.is_valid());

        let mut mut_x = gmp_x.clone();
        mut_x -= gmp_y.clone();
        assert!(mut_x.is_valid());
        assert_eq!(mut_x, raw_gmp_difference);
        let mut mut_x = gmp_x.clone();
        mut_x -= &gmp_y;
        assert_eq!(mut_x, raw_gmp_difference);
        assert!(mut_x.is_valid());

        let mut mut_x = native_integer_to_rugint(&x);
        mut_x -= native_integer_to_rugint(&y);
        assert_eq!(rugint_integer_to_native(&mut_x), difference);

        let reverse_difference = &y - &x;
        let inv_1 = &difference + &y;
        let inv_2 = &x - &difference;
        assert_eq!(gmp_difference, difference);
        assert_eq!(num_difference, difference);
        assert_eq!(rugint_difference, difference);
        assert_eq!(reverse_difference, -difference);
        assert_eq!(inv_1, x);
        assert_eq!(inv_2, y);
    };

    // x - (y: i32) == x - from(y)
    // (y: i32) - x == x - from(y)
    let integer_and_i32 = |gmp_x: gmp::Integer, y: i32| {
        let x = gmp_integer_to_native(&gmp_x);
        let primitive_difference_1 = &x - y;
        let primitive_difference_2 = y - &x;
        let difference = x - native::Integer::from(y);
        assert_eq!(primitive_difference_1, difference);
        assert_eq!(primitive_difference_2, -difference);
    };

    // x - 0 == x
    // 0 - x == -x
    // x - x == 0
    // x - -x == x << 1
    let one_integer = |gmp_x: gmp::Integer| {
        let x = gmp_integer_to_native(&gmp_x);
        let id_1 = &x - native::Integer::ZERO;
        let id_2 = native::Integer::ZERO - &x;
        let double = &x - -&x;
        assert_eq!(id_1, x);
        assert_eq!(id_2, -&x);
        assert_eq!(double, &x << 1);
        assert_eq!(&x - &x, 0)
    };

    for (x, y) in exhaustive_pairs_from_single(exhaustive_integers()).take(LARGE_LIMIT) {
        two_integers(x, y);
    }

    for (x, y) in random_pairs_from_single(random_integers(&EXAMPLE_SEED, 32)).take(LARGE_LIMIT) {
        two_integers(x, y);
    }

    for (x, y) in exhaustive_pairs(exhaustive_integers(), exhaustive_i()).take(LARGE_LIMIT) {
        integer_and_i32(x, y);
    }

    for (x, y) in random_pairs(
        &EXAMPLE_SEED,
        &(|seed| random_integers(seed, 32)),
        &(|seed| random_x(seed)),
    ).take(LARGE_LIMIT)
    {
        integer_and_i32(x, y);
    }

    for n in exhaustive_integers().take(LARGE_LIMIT) {
        one_integer(n);
    }

    for n in random_integers(&EXAMPLE_SEED, 32).take(LARGE_LIMIT) {
        one_integer(n);
    }
}
