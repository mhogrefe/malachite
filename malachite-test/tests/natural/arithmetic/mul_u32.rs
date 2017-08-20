use common::LARGE_LIMIT;
use malachite_native::natural as native;
use malachite_gmp::natural as gmp;
use malachite_test::common::{gmp_natural_to_native, native_natural_to_gmp,
                             native_natural_to_num_biguint, native_natural_to_rugint_integer,
                             num_biguint_to_native_natural, rugint_integer_to_native_natural};
use malachite_test::natural::arithmetic::mul_u32::num_mul_u32;
use num;
use rugint;
use rust_wheels::iterators::common::EXAMPLE_SEED;
use rust_wheels::iterators::general::random_x;
use rust_wheels::iterators::naturals::{exhaustive_naturals, random_naturals};
use rust_wheels::iterators::primitive_ints::exhaustive_u;
use rust_wheels::iterators::tuples::{exhaustive_pairs, random_pairs};
use std::str::FromStr;

#[test]
fn test_add_u32() {
    #[allow(cyclomatic_complexity)]
    let test = |u, v: u32, out| {
        let mut n = native::Natural::from_str(u).unwrap();
        n *= v;
        assert_eq!(n.to_string(), out);
        assert!(n.is_valid());

        let mut n = gmp::Natural::from_str(u).unwrap();
        n *= v;
        assert_eq!(n.to_string(), out);
        assert!(n.is_valid());

        let mut n = rugint::Integer::from_str(u).unwrap();
        n *= v;
        assert_eq!(n.to_string(), out);

        let n = native::Natural::from_str(u).unwrap() * v;
        assert_eq!(n.to_string(), out);
        assert!(n.is_valid());

        let n = gmp::Natural::from_str(u).unwrap() * v;
        assert_eq!(n.to_string(), out);
        assert!(n.is_valid());

        let n = num_mul_u32(num::BigUint::from_str(u).unwrap(), v);
        assert_eq!(n.to_string(), out);

        let n = rugint::Integer::from_str(u).unwrap() * v;
        assert_eq!(n.to_string(), out);

        let n = &native::Natural::from_str(u).unwrap() * v;
        assert_eq!(n.to_string(), out);
        assert!(n.is_valid());

        let n = &gmp::Natural::from_str(u).unwrap() * v;
        assert_eq!(n.to_string(), out);
        assert!(n.is_valid());

        let n = v * native::Natural::from_str(u).unwrap();
        assert_eq!(n.to_string(), out);
        assert!(n.is_valid());

        let n = v * gmp::Natural::from_str(u).unwrap();
        assert_eq!(n.to_string(), out);
        assert!(n.is_valid());

        let n = v * rugint::Integer::from_str(u).unwrap();
        assert_eq!(n.to_string(), out);

        let n = v * &native::Natural::from_str(u).unwrap();
        assert_eq!(n.to_string(), out);
        assert!(n.is_valid());

        let n = v * &gmp::Natural::from_str(u).unwrap();
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
    // if n != 0 and u != 0, n * u >= n and n * u >= u
    // TODO n * u / u == n
    #[allow(cyclomatic_complexity)]
    let natural_and_u32 = |mut gmp_n: gmp::Natural, u: u32| {
        let mut n = gmp_natural_to_native(&gmp_n);
        let old_n = n.clone();
        gmp_n *= u;
        assert!(gmp_n.is_valid());

        n *= u;
        assert!(n.is_valid());
        assert_eq!(gmp_natural_to_native(&gmp_n), n);

        let mut rugint_n = native_natural_to_rugint_integer(&old_n);
        rugint_n *= u;
        assert_eq!(rugint_integer_to_native_natural(&rugint_n), n);

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
        let result = n2 * native::Natural::from(u);
        assert_eq!(result, n);
        let n2 = old_n.clone();
        let result = native::Natural::from(u) * n2;
        assert_eq!(result, n);

        let gmp_n2 = native_natural_to_gmp(&old_n);
        let result = &gmp_n2 * u;
        assert!(result.is_valid());
        assert_eq!(gmp_natural_to_native(&result), n);
        let result = gmp_n2 * u;
        assert!(result.is_valid());
        assert_eq!(gmp_natural_to_native(&result), n);

        let gmp_n2 = native_natural_to_gmp(&old_n);
        let result = u * &gmp_n2;
        assert!(result.is_valid());
        assert_eq!(gmp_natural_to_native(&result), n);
        let result = u * gmp_n2;
        assert!(result.is_valid());
        assert_eq!(gmp_natural_to_native(&result), n);

        let num_n2 = native_natural_to_num_biguint(&old_n);
        assert_eq!(num_biguint_to_native_natural(&num_mul_u32(num_n2, u)), n);

        let rugint_n2 = native_natural_to_rugint_integer(&old_n);
        assert_eq!(rugint_integer_to_native_natural(&(rugint_n2 * u)), n);

        if n != 0 && u != 0 {
            assert!(n >= old_n);
            assert!(n >= u);
        }
        //TODO assert_eq!(n / u, Some(old_n));
    };

    // n * 0 == 0
    // 0 * n == 0
    // n * 1 == n
    // 1 * n == n
    // n * 2 == n << 1
    // 2 * n == n << 1
    #[allow(identity_op)]
    let one_natural = |gmp_n: gmp::Natural| {
        let n = gmp_natural_to_native(&gmp_n);
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
        assert_eq!(native::Natural::from(0u32) * u, 0);
        assert_eq!(u * native::Natural::from(0u32), 0);
        assert_eq!(native::Natural::from(1u32) * u, u);
        assert_eq!(u * native::Natural::from(1u32), u);
    };

    for (n, u) in exhaustive_pairs(exhaustive_naturals(), exhaustive_u::<u32>()).take(LARGE_LIMIT) {
        natural_and_u32(n, u);
    }

    for (n, u) in random_pairs(
        &EXAMPLE_SEED,
        &(|seed| random_naturals(seed, 32)),
        &(|seed| random_x(seed)),
    ).take(LARGE_LIMIT)
    {
        natural_and_u32(n, u);
    }

    for n in exhaustive_naturals().take(LARGE_LIMIT) {
        one_natural(n);
    }

    for n in random_naturals(&EXAMPLE_SEED, 32).take(LARGE_LIMIT) {
        one_natural(n);
    }

    for u in exhaustive_u().take(LARGE_LIMIT) {
        one_u32(u);
    }

    for u in random_x(&EXAMPLE_SEED).take(LARGE_LIMIT) {
        one_u32(u);
    }
}
