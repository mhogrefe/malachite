use common::LARGE_LIMIT;
use malachite_native::integer as native;
use malachite_gmp::integer as gmp;
use malachite_test::common::{gmp_integer_to_native, native_integer_to_gmp,
                             native_integer_to_num_bigint, native_integer_to_rugint,
                             num_bigint_to_native_integer, rugint_integer_to_native};
use malachite_test::integer::arithmetic::mul_i32::num_mul_i32;
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
        n *= v;
        assert_eq!(n.to_string(), out);
        assert!(n.is_valid());

        let mut n = gmp::Integer::from_str(u).unwrap();
        n *= v;
        assert_eq!(n.to_string(), out);
        assert!(n.is_valid());

        let mut n = rugint::Integer::from_str(u).unwrap();
        n *= v;
        assert_eq!(n.to_string(), out);

        let n = native::Integer::from_str(u).unwrap() * v;
        assert_eq!(n.to_string(), out);
        assert!(n.is_valid());

        let n = gmp::Integer::from_str(u).unwrap() * v;
        assert_eq!(n.to_string(), out);
        assert!(n.is_valid());

        let n = num_mul_i32(num::BigInt::from_str(u).unwrap(), v);
        assert_eq!(n.to_string(), out);

        let n = rugint::Integer::from_str(u).unwrap() * v;
        assert_eq!(n.to_string(), out);

        let n = &native::Integer::from_str(u).unwrap() * v;
        assert_eq!(n.to_string(), out);
        assert!(n.is_valid());

        let n = &gmp::Integer::from_str(u).unwrap() * v;
        assert_eq!(n.to_string(), out);
        assert!(n.is_valid());

        let n = v * native::Integer::from_str(u).unwrap();
        assert_eq!(n.to_string(), out);
        assert!(n.is_valid());

        let n = v * gmp::Integer::from_str(u).unwrap();
        assert_eq!(n.to_string(), out);
        assert!(n.is_valid());

        let n = v * rugint::Integer::from_str(u).unwrap();
        assert_eq!(n.to_string(), out);

        let n = v * &native::Integer::from_str(u).unwrap();
        assert_eq!(n.to_string(), out);
        assert!(n.is_valid());

        let n = v * &gmp::Integer::from_str(u).unwrap();
        assert_eq!(n.to_string(), out);
        assert!(n.is_valid());

        let n = v * &rugint::Integer::from_str(u).unwrap();
        assert_eq!(n.to_string(), out);
    };
    test("0", 0, "0");
    test("0", 123, "0");
    test("123", 0, "0");
    test("1", 123, "123");
    test("123", 1, "123");
    test("123", 456, "56088");
    test("1000000000000", 0, "0");
    test("1000000000000", 1, "1000000000000");
    test("1000000000000", 123, "123000000000000");
    test("4294967295", 2, "8589934590");
    test("18446744073709551615", 2, "36893488147419103230");
    test("-1", 123, "-123");
    test("-123", 1, "-123");
    test("-123", 456, "-56088");
    test("-1000000000000", 0, "0");
    test("-1000000000000", 1, "-1000000000000");
    test("-1000000000000", 123, "-123000000000000");
    test("-4294967295", 2, "-8589934590");
    test("-4294967296", 2, "-8589934592");
    test("-18446744073709551615", 2, "-36893488147419103230");
    test("0", -123, "0");
    test("123", 0, "0");
    test("1", -123, "-123");
    test("123", -1, "-123");
    test("123", -456, "-56088");
    test("1000000000000", -1, "-1000000000000");
    test("1000000000000", -123, "-123000000000000");
    test("4294967295", -2, "-8589934590");
    test("18446744073709551615", -2, "-36893488147419103230");
    test("-1", -123, "123");
    test("-123", -1, "123");
    test("-123", -456, "56088");
    test("-1000000000000", -1, "1000000000000");
    test("-1000000000000", -123, "123000000000000");
    test("-4294967295", -2, "8589934590");
    test("-4294967296", -2, "8589934592");
    test("-18446744073709551615", -2, "36893488147419103230");
    test("-4294967296", -1, "4294967296");
    test("4294967296", -1, "-4294967296");
}

#[test]
fn mul_i32_properties() {
    // n *= i is equivalent for malachite-gmp, malachite-native, and rugint.
    // n * i is equivalent for malachite-gmp, malachite-native, num, and rugint.
    // &n * i is equivalent for malachite-gmp, malachite-native, and num.
    // n *= i; n is valid.
    // n * i and i * n are valid.
    // &n * i and i * &n are valid.
    // n *= i, n * i, i * n, &n * i, and i * &n give the same result.
    // n * i == n * from(i)
    // (-n) * i == -(n * i)
    // if i != -2^31, n * (-i) == -(n * i)
    // TODO n * i / i == n
    #[allow(cyclomatic_complexity)]
    let integer_and_i32 = |mut gmp_n: gmp::Integer, i: i32| {
        let mut n = gmp_integer_to_native(&gmp_n);
        let old_n = n.clone();
        gmp_n *= i;
        assert!(gmp_n.is_valid());

        n *= i;
        assert!(n.is_valid());
        assert_eq!(gmp_integer_to_native(&gmp_n), n);

        let mut rugint_n = native_integer_to_rugint(&old_n);
        rugint_n *= i;
        assert_eq!(rugint_integer_to_native(&rugint_n), n);

        let n2 = old_n.clone();
        let result = &n2 * i;
        assert!(result.is_valid());
        assert_eq!(result, n);
        let result = n2 * i;
        assert!(result.is_valid());
        assert_eq!(result, n);

        let n2 = old_n.clone();
        let result = i * &n2;
        assert!(result.is_valid());
        assert_eq!(result, n);
        let result = i * n2;
        assert_eq!(result, n);
        assert!(result.is_valid());

        //TODO
        /*
        let n2 = old_n.clone();
        let result = n2 * native::Integer::from(u);
        assert_eq!(result, n);
        let n2 = old_n.clone();
        let result = native::Integer::from(u) * n2;
        assert_eq!(result, n);
        */

        let gmp_n2 = native_integer_to_gmp(&old_n);
        let result = &gmp_n2 * i;
        assert!(result.is_valid());
        assert_eq!(gmp_integer_to_native(&result), n);
        let result = gmp_n2 * i;
        assert!(result.is_valid());
        assert_eq!(gmp_integer_to_native(&result), n);

        let gmp_n2 = native_integer_to_gmp(&old_n);
        let result = i * &gmp_n2;
        assert!(result.is_valid());
        assert_eq!(gmp_integer_to_native(&result), n);
        let result = i * gmp_n2;
        assert!(result.is_valid());
        assert_eq!(gmp_integer_to_native(&result), n);

        let num_n2 = native_integer_to_num_bigint(&old_n);
        assert_eq!(num_bigint_to_native_integer(&num_mul_i32(num_n2, i)), n);

        let rugint_n2 = native_integer_to_rugint(&old_n);
        assert_eq!(rugint_integer_to_native(&(rugint_n2 * i)), n);

        assert_eq!((-&n) * i, -(&n * i));
        if i != i32::min_value() {
            assert_eq!(&n * (-i), -(n * i));
        }
        //TODO assert_eq!(n / u, Some(old_n));
    };

    // n * 0 == 0
    // 0 * n == 0
    // n * 1 == n
    // 1 * n == n
    // n * 2 == n << 1
    // 2 * n == n << 1
    // n * -1 == -n
    // -1 * n == -n
    #[allow(identity_op)]
    let one_integer = |gmp_n: gmp::Integer| {
        let n = gmp_integer_to_native(&gmp_n);
        assert_eq!(&n * 0i32, 0);
        assert_eq!(0i32 * &n, 0);
        assert_eq!(&n * 1i32, n);
        assert_eq!(1i32 * &n, n);
        assert_eq!(&n * 2i32, &n << 1);
        assert_eq!(2i32 * &n, &n << 1);
        assert_eq!(&n * -1i32, -&n);
        assert_eq!(-1i32 * &n, -n);
    };

    // 0 * i == 0
    // i * 0 == 0
    // 1 * i == i
    // i * 1 == i
    // if i != -2^31, i * -1 == -i and -1 * i == -i
    let one_i32 = |i: i32| {
        assert_eq!(native::Integer::from(0i32) * i, 0);
        assert_eq!(i * native::Integer::from(0i32), 0);
        assert_eq!(native::Integer::from(1i32) * i, i);
        assert_eq!(i * native::Integer::from(1i32), i);
        if i != i32::min_value() {
            assert_eq!(native::Integer::from(-1i32) * i, -i);
            assert_eq!(i * native::Integer::from(-1i32), -i);
        }
    };

    for (n, i) in exhaustive_pairs(exhaustive_integers(), exhaustive_i::<i32>()).take(LARGE_LIMIT) {
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

    for i in exhaustive_i().take(LARGE_LIMIT) {
        one_i32(i);
    }

    for i in random_x(&EXAMPLE_SEED).take(LARGE_LIMIT) {
        one_i32(i);
    }
}
