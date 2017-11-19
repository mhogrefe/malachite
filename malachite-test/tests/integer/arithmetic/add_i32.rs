use common::LARGE_LIMIT;
use malachite_base::traits::Zero;
use malachite_native::integer as native;
use malachite_gmp::integer as gmp;
use malachite_test::common::{gmp_integer_to_native, native_integer_to_gmp,
                             native_integer_to_num_bigint, native_integer_to_rugint,
                             num_bigint_to_native_integer, rugint_integer_to_native};
use malachite_test::integer::arithmetic::add_i32::num_add_i32;
use num;
use rugint;
use rust_wheels::iterators::common::EXAMPLE_SEED;
use rust_wheels::iterators::general::random_x;
use rust_wheels::iterators::integers::{exhaustive_integers, random_integers};
use rust_wheels::iterators::primitive_ints::exhaustive_i;
use rust_wheels::iterators::tuples::{exhaustive_pairs, random_pairs};
use std::str::FromStr;

#[test]
fn test_add_i32() {
    #[allow(cyclomatic_complexity)]
    let test = |u, v: i32, out| {
        let mut n = native::Integer::from_str(u).unwrap();
        n += v;
        assert_eq!(n.to_string(), out);
        assert!(n.is_valid());

        let mut n = gmp::Integer::from_str(u).unwrap();
        n += v;
        assert_eq!(n.to_string(), out);
        assert!(n.is_valid());

        let mut n = rugint::Integer::from_str(u).unwrap();
        n += v;
        assert_eq!(n.to_string(), out);

        let n = native::Integer::from_str(u).unwrap() + v;
        assert_eq!(n.to_string(), out);
        assert!(n.is_valid());

        let n = gmp::Integer::from_str(u).unwrap() + v;
        assert_eq!(n.to_string(), out);
        assert!(n.is_valid());

        let n = num_add_i32(num::BigInt::from_str(u).unwrap(), v);
        assert_eq!(n.to_string(), out);

        let n = rugint::Integer::from_str(u).unwrap() + v;
        assert_eq!(n.to_string(), out);

        let n = &native::Integer::from_str(u).unwrap() + v;
        assert_eq!(n.to_string(), out);
        assert!(n.is_valid());

        let n = &gmp::Integer::from_str(u).unwrap() + v;
        assert_eq!(n.to_string(), out);
        assert!(n.is_valid());

        let n = v + native::Integer::from_str(u).unwrap();
        assert_eq!(n.to_string(), out);
        assert!(n.is_valid());

        let n = v + gmp::Integer::from_str(u).unwrap();
        assert_eq!(n.to_string(), out);
        assert!(n.is_valid());

        let n = v + rugint::Integer::from_str(u).unwrap();
        assert_eq!(n.to_string(), out);

        let n = v + &native::Integer::from_str(u).unwrap();
        assert_eq!(n.to_string(), out);
        assert!(n.is_valid());

        let n = v + &gmp::Integer::from_str(u).unwrap();
        assert_eq!(n.to_string(), out);
        assert!(n.is_valid());

        let n = v + &rugint::Integer::from_str(u).unwrap();
        assert_eq!(n.to_string(), out);
    };
    test("0", 0, "0");
    test("0", 123, "123");
    test("123", 0, "123");
    test("123", 456, "579");
    test("-123", 456, "333");
    test("-500", 456, "-44");
    test("123", -123, "0");
    test("456", -123, "333");
    test("123", -456, "-333");
    test("-456", -123, "-579");
    test("1000000000000", 123, "1000000000123");
    test("-1000000000000", 123, "-999999999877");
    test("1000000000000", -123, "999999999877");
    test("-1000000000000", -123, "-1000000000123");
    test("4294967295", 1, "4294967296");
    test("-4294967296", 1, "-4294967295");
    test("2147483647", 1, "2147483648");
    test("-2147483648", 1, "-2147483647");
    test("18446744073709551615", 1, "18446744073709551616");
    test("-18446744073709551616", 1, "-18446744073709551615");
    test("4294967296", -1, "4294967295");
    test("-4294967295", -1, "-4294967296");
    test("2147483648", -1, "2147483647");
    test("-2147483647", -1, "-2147483648");
    test("18446744073709551616", -1, "18446744073709551615");
    test("-18446744073709551615", -1, "-18446744073709551616");
}

#[test]
fn add_i32_properties() {
    // n += i is equivalent for malachite-gmp, malachite-native, and rugint.
    // n + i is equivalent for malachite-gmp, malachite-native, num, and rugint.
    // &n + i is equivalent for malachite-gmp, malachite-native, and num.
    // n += i; n is valid.
    // n + i and i + n are valid.
    // &n + i and i + &n are valid.
    // n += i, n + i, i + n, &n + i, and i + &n give the same result.
    // n + i == n + from(u)
    // n + i - i == n
    // n + i - n == i
    #[allow(cyclomatic_complexity)]
    let integer_and_i32 = |mut gmp_n: gmp::Integer, i: i32| {
        let mut n = gmp_integer_to_native(&gmp_n);
        let old_n = n.clone();
        gmp_n += i;
        assert!(gmp_n.is_valid());

        n += i;
        assert!(n.is_valid());
        assert_eq!(gmp_integer_to_native(&gmp_n), n);

        let mut rugint_n = native_integer_to_rugint(&old_n);
        rugint_n += i;
        assert_eq!(rugint_integer_to_native(&rugint_n), n);

        let n2 = old_n.clone();
        let result = &n2 + i;
        assert!(result.is_valid());
        assert_eq!(result, n);
        let result = n2 + i;
        assert!(result.is_valid());
        assert_eq!(result, n);

        let n2 = old_n.clone();
        let result = i + &n2;
        assert!(result.is_valid());
        assert_eq!(result, n);
        let result = i + n2;
        assert_eq!(result, n);
        assert!(result.is_valid());

        let n2 = old_n.clone();
        let result = n2 + native::Integer::from(i);
        assert_eq!(result, n);
        let n2 = old_n.clone();
        let result = native::Integer::from(i) + n2;
        assert_eq!(result, n);

        let gmp_n2 = native_integer_to_gmp(&old_n);
        let result = &gmp_n2 + i;
        assert!(result.is_valid());
        assert_eq!(gmp_integer_to_native(&result), n);
        let result = gmp_n2 + i;
        assert!(result.is_valid());
        assert_eq!(gmp_integer_to_native(&result), n);

        let gmp_n2 = native_integer_to_gmp(&old_n);
        let result = i + &gmp_n2;
        assert!(result.is_valid());
        assert_eq!(gmp_integer_to_native(&result), n);
        let result = i + gmp_n2;
        assert!(result.is_valid());
        assert_eq!(gmp_integer_to_native(&result), n);

        let num_n2 = native_integer_to_num_bigint(&old_n);
        assert_eq!(num_bigint_to_native_integer(&num_add_i32(num_n2, i)), n);

        let rugint_n2 = native_integer_to_rugint(&old_n);
        assert_eq!(rugint_integer_to_native(&(rugint_n2 + i)), n);

        assert_eq!(&n - i, old_n);
        assert_eq!(n - old_n, i);
    };

    // n + 0 == n
    // 0 + n == n
    #[allow(identity_op)]
    let one_integer = |gmp_n: gmp::Integer| {
        let n = gmp_integer_to_native(&gmp_n);
        assert_eq!(&n + 0i32, n);
        assert_eq!(0i32 + &n, n);
    };

    // 0 + i == i
    // i + 0 == i
    let one_i32 = |i: i32| {
        assert_eq!(native::Integer::zero() + i, native::Integer::from(i));
        assert_eq!(i + native::Integer::zero(), native::Integer::from(i));
    };

    for (n, i) in exhaustive_pairs(exhaustive_integers(), exhaustive_i()).take(LARGE_LIMIT) {
        integer_and_i32(n, i);
    }

    for (n, i) in random_pairs(
        &EXAMPLE_SEED,
        &(|seed| random_integers(seed, 32)),
        &(|seed| random_x(seed)),
    ).take(LARGE_LIMIT)
    {
        integer_and_i32(n, i);
    }

    for n in exhaustive_integers().take(LARGE_LIMIT) {
        one_integer(n);
    }

    for n in random_integers(&EXAMPLE_SEED, 32).take(LARGE_LIMIT) {
        one_integer(n);
    }

    for n in exhaustive_i().take(LARGE_LIMIT) {
        one_i32(n);
    }

    for n in random_x(&EXAMPLE_SEED).take(LARGE_LIMIT) {
        one_i32(n);
    }
}
