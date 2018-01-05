use common::LARGE_LIMIT;
use malachite_base::traits::{One, Zero};
use malachite_native::integer as native;
use malachite_gmp::integer as gmp;
use malachite_test::common::{gmp_integer_to_native, native_integer_to_gmp,
                             native_integer_to_num_bigint, native_integer_to_rugint,
                             num_bigint_to_native_integer, rugint_integer_to_native,
                             GenerationMode};
use malachite_test::integer::arithmetic::mul_u32::{num_mul_u32, select_inputs_1};
use num;
use rugint;
use rust_wheels::iterators::common::EXAMPLE_SEED;
use rust_wheels::iterators::general::random_x;
use rust_wheels::iterators::integers::{exhaustive_integers, random_integers};
use rust_wheels::iterators::primitive_ints::exhaustive_u;
use std::str::FromStr;

#[test]
fn test_add_u32() {
    #[allow(unknown_lints, cyclomatic_complexity)]
    let test = |u, v: u32, out| {
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

        let n = num_mul_u32(num::BigInt::from_str(u).unwrap(), v);
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
}

#[test]
fn mul_u32_properties() {
    // n *= u is equivalent for malachite-gmp, malachite-native, and rugint.
    // n * u is equivalent for malachite-gmp, malachite-native, num, and rugint.
    // &n * u is equivalent for malachite-gmp, malachite-native, and num.
    // n *= u; n is valid.
    // n * u and u * n are valid.
    // &n * u and u * &n are valid.
    // n *= u, n * u, u * n, &n * u, and u * &n give the same result.
    // n * u == n * from(u)
    // (-n) * u == -(n * u)
    // TODO n * u / u == n
    #[allow(unknown_lints, cyclomatic_complexity)]
    let integer_and_u32 = |mut gmp_n: gmp::Integer, u: u32| {
        let mut n = gmp_integer_to_native(&gmp_n);
        let old_n = n.clone();
        gmp_n *= u;
        assert!(gmp_n.is_valid());

        n *= u;
        assert!(n.is_valid());
        assert_eq!(gmp_integer_to_native(&gmp_n), n);

        let mut rugint_n = native_integer_to_rugint(&old_n);
        rugint_n *= u;
        assert_eq!(rugint_integer_to_native(&rugint_n), n);

        let n2 = old_n.clone();
        let result = &n2 * u;
        assert!(result.is_valid());
        assert_eq!(result, n);
        let result = n2 * u;
        assert!(result.is_valid());
        assert_eq!(result, n);

        let n2 = old_n.clone();
        let result = u * &n2;
        assert!(result.is_valid());
        assert_eq!(result, n);
        let result = u * n2;
        assert_eq!(result, n);
        assert!(result.is_valid());

        let n2 = old_n.clone();
        let result = n2 * native::Integer::from(u);
        assert_eq!(result, n);
        let n2 = old_n.clone();
        let result = native::Integer::from(u) * n2;
        assert_eq!(result, n);

        let gmp_n2 = native_integer_to_gmp(&old_n);
        let result = &gmp_n2 * u;
        assert!(result.is_valid());
        assert_eq!(gmp_integer_to_native(&result), n);
        let result = gmp_n2 * u;
        assert!(result.is_valid());
        assert_eq!(gmp_integer_to_native(&result), n);

        let gmp_n2 = native_integer_to_gmp(&old_n);
        let result = u * &gmp_n2;
        assert!(result.is_valid());
        assert_eq!(gmp_integer_to_native(&result), n);
        let result = u * gmp_n2;
        assert!(result.is_valid());
        assert_eq!(gmp_integer_to_native(&result), n);

        let num_n2 = native_integer_to_num_bigint(&old_n);
        assert_eq!(num_bigint_to_native_integer(&num_mul_u32(num_n2, u)), n);

        let rugint_n2 = native_integer_to_rugint(&old_n);
        assert_eq!(rugint_integer_to_native(&(rugint_n2 * u)), n);

        assert_eq!((-&n) * u, -(n * u));
        //TODO assert_eq!(n / u, Some(old_n));
    };

    // n * 0 == 0
    // 0 * n == 0
    // n * 1 == n
    // 1 * n == n
    // n * 2 == n << 1
    // 2 * n == n << 1
    #[allow(unknown_lints, erasing_op, identity_op)]
    let one_integer = |gmp_n: gmp::Integer| {
        let n = gmp_integer_to_native(&gmp_n);
        assert_eq!(&n * 0u32, 0);
        assert_eq!(0u32 * &n, 0);
        assert_eq!(&n * 1u32, n);
        assert_eq!(1u32 * &n, n);
        assert_eq!(&n * 2u32, &n << 1);
        assert_eq!(2u32 * &n, &n << 1);
    };

    // 0 * u == 0
    // u * 0 == 0
    // 1 * u == u
    // u * 1 == u
    let one_u32 = |u: u32| {
        assert_eq!(native::Integer::ZERO * u, 0);
        assert_eq!(u * native::Integer::ZERO, 0);
        assert_eq!(native::Integer::ONE * u, u);
        assert_eq!(u * native::Integer::ONE, u);
    };

    for (n, u) in select_inputs_1(GenerationMode::Exhaustive).take(LARGE_LIMIT) {
        integer_and_u32(n, u);
    }

    for (n, u) in select_inputs_1(GenerationMode::Random(32)).take(LARGE_LIMIT) {
        integer_and_u32(n, u);
    }

    for n in exhaustive_integers().take(LARGE_LIMIT) {
        one_integer(n);
    }

    for n in random_integers(&EXAMPLE_SEED, 32).take(LARGE_LIMIT) {
        one_integer(n);
    }

    for u in exhaustive_u().take(LARGE_LIMIT) {
        one_u32(u);
    }

    for u in random_x(&EXAMPLE_SEED).take(LARGE_LIMIT) {
        one_u32(u);
    }
}
