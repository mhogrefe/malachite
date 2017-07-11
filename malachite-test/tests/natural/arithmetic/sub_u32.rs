use common::LARGE_LIMIT;
use malachite_native::natural as native;
use malachite_gmp::natural as gmp;
use malachite_test::common::{gmp_natural_to_native, native_natural_to_gmp,
                             native_natural_to_num_biguint, native_natural_to_rugint_integer,
                             num_biguint_to_native_natural, rugint_integer_to_native_natural};
use malachite_test::natural::arithmetic::sub_u32::{num_sub_u32, rugint_sub_u32};
use num;
use rugint;
use rust_wheels::iterators::common::EXAMPLE_SEED;
use rust_wheels::iterators::general::random_x;
use rust_wheels::iterators::naturals::{exhaustive_naturals, random_naturals};
use rust_wheels::iterators::primitive_ints::exhaustive_u;
use rust_wheels::iterators::tuples::{exhaustive_pairs, random_pairs};
use std::str::FromStr;

#[test]
fn test_sub_assign_u32() {
    let test = |u, v: u32, out| {
        let mut n = native::Natural::from_str(u).unwrap();
        n -= v;
        assert_eq!(n.to_string(), out);
        assert!(n.is_valid());

        let mut n = gmp::Natural::from_str(u).unwrap();
        n -= v;
        assert_eq!(n.to_string(), out);
        assert!(n.is_valid());
    };
    test("0", 0, "0");
    test("123", 123, "0");
    test("123", 0, "123");
    test("456", 123, "333");
    test("1000000000000", 123, "999999999877");
    test("4294967296", 1, "4294967295");
    test("18446744073709551616", 1, "18446744073709551615");
}

#[test]
#[should_panic(expected = "Cannot subtract a u32 from a smaller Natural. self: 123, other: 456")]
fn sub_assign_u32_native() {
    let mut x = native::Natural::from_str("123").unwrap();
    x -= 456;
}

#[test]
#[should_panic(expected = "Cannot subtract a u32 from a smaller Natural. self: 123, other: 456")]
fn sub_assign_u32_gmp() {
    let mut x = gmp::Natural::from_str("123").unwrap();
    x -= 456;
}

#[test]
fn test_sub_u32() {
    let test = |u, v: u32, out| {
        let on = native::Natural::from_str(u).unwrap() - v;
        assert_eq!(format!("{:?}", on), out);
        assert!(on.map_or(true, |n| n.is_valid()));

        let on = gmp::Natural::from_str(u).unwrap() - v;
        assert_eq!(format!("{:?}", on), out);
        assert!(on.map_or(true, |n| n.is_valid()));

        let on = &native::Natural::from_str(u).unwrap() - v;
        assert_eq!(format!("{:?}", on), out);
        assert!(on.map_or(true, |n| n.is_valid()));

        let on = &gmp::Natural::from_str(u).unwrap() - v;
        assert_eq!(format!("{:?}", on), out);
        assert!(on.map_or(true, |n| n.is_valid()));

        let ux = num::BigUint::from_str(u).unwrap();
        let on = num_sub_u32(ux, v).map(|x| num_biguint_to_native_natural(&x));
        assert_eq!(format!("{:?}", on), out);

        let on = rugint_sub_u32(rugint::Integer::from_str(u).unwrap(), v);
        assert_eq!(format!("{:?}", on), out);
    };
    test("0", 0, "Some(0)");
    test("123", 123, "Some(0)");
    test("123", 0, "Some(123)");
    test("456", 123, "Some(333)");
    test("123", 456, "None");
    test("1000000000000", 123, "Some(999999999877)");
    test("4294967296", 1, "Some(4294967295)");
    test("18446744073709551616", 1, "Some(18446744073709551615)");
}

#[test]
fn test_u32_sub_natural() {
    let test = |u: u32, v, out| {
        let on = u - &native::Natural::from_str(v).unwrap();
        assert_eq!(format!("{:?}", on), out);
        assert!(on.map_or(true, |n| n.is_valid()));

        let on = u - &gmp::Natural::from_str(v).unwrap();
        assert_eq!(format!("{:?}", on), out);
        assert!(on.map_or(true, |n| n.is_valid()));
    };
    test(0, "0", "Some(0)");
    test(123, "123", "Some(0)");
    test(123, "0", "Some(123)");
    test(456, "123", "Some(333)");
    test(123, "456", "None");
    test(123, "1000000000000", "None");
    test(4294967295, "4294967295", "Some(0)");
}

#[test]
fn sub_u32_properties() {
    // n -= u is equivalent for malachite-gmp, malachite-native, and rugint.
    // n - u is equivalent for malachite-gmp, malachite-native, num, and rugint.
    // &n - u is equivalent for malachite-gmp, malachite-native, and num.
    // n -= u; n is valid.
    // n - u is valid.
    // &n - u and u - &n are valid.
    // n -= u, n - u, and &n - u give the same result.
    // n - u == n - from(u)
    // u - n == from(u) - n
    // if n >= u, n - u <= n
    // if n >= u, (n - u).unwrap() + u == n
    #[allow(cyclomatic_complexity)]
    let natural_and_u32 = |mut gmp_n: gmp::Natural, u: u32| {
        let mut n = gmp_natural_to_native(&gmp_n);
        let old_n = n.clone();

        if n >= u {
            gmp_n -= u;
            assert!(gmp_n.is_valid());

            n -= u;
            assert!(n.is_valid());
            assert_eq!(gmp_natural_to_native(&gmp_n), n);

            let mut rugint_n = native_natural_to_rugint_integer(&old_n);
            rugint_n -= u;
            assert_eq!(rugint_integer_to_native_natural(&rugint_n), n);
        }
        let on = if old_n >= u { Some(n) } else { None };

        let n2 = old_n.clone();
        let result = &n2 - u;
        assert_eq!(result, on);
        assert!(result.map_or(true, |n| n.is_valid()));
        let result = n2 - u;
        assert_eq!(result, on);
        assert!(result.map_or(true, |n| n.is_valid()));

        let n2 = old_n.clone();
        let result = u - &n2;
        assert_eq!(result.is_some(), u == n2 || on.is_none());
        if result.is_some() {
            assert_eq!(native::Natural::from(u) - n2.to_u32().unwrap(), result);
        }
        assert!(result.map_or(true, |n| n.is_valid()));

        let n2 = old_n.clone();
        let result = n2 - &native::Natural::from(u);
        assert_eq!(result, on);
        let n2 = old_n.clone();
        assert_eq!(u - &n2, native::Natural::from(u) - &n2);

        let gmp_n2 = native_natural_to_gmp(&old_n);
        let result = &gmp_n2 - u;
        assert_eq!(result.clone().map(|x| gmp_natural_to_native(&x)), on);
        assert!(result.map_or(true, |n| n.is_valid()));
        let result = gmp_n2 - u;
        assert_eq!(result.clone().map(|x| gmp_natural_to_native(&x)), on);
        assert!(result.map_or(true, |n| n.is_valid()));

        let gmp_n2 = native_natural_to_gmp(&old_n);
        let result = u - &gmp_n2;
        assert_eq!(result.is_some(), u == gmp_n2 || on.is_none());
        if result.is_some() {
            assert_eq!(gmp::Natural::from(u) - gmp_n2.to_u32().unwrap(), result);
        }
        assert!(result.map_or(true, |n| n.is_valid()));

        let num_n2 = native_natural_to_num_biguint(&old_n);
        assert_eq!(num_sub_u32(num_n2, u).map(|x| num_biguint_to_native_natural(&x)),
                   on);

        let rugint_n2 = native_natural_to_rugint_integer(&old_n);
        assert_eq!(rugint_sub_u32(rugint_n2, u).map(|x| rugint_integer_to_native_natural(&x)),
                   on);

        if on.is_some() {
            assert!(on.clone().unwrap() <= old_n);
            assert_eq!(on.unwrap() + u, old_n);
        }
    };


    // n - 0 == n
    // if n != 0, 0 - n == None
    #[allow(identity_op)]
    let one_natural = |gmp_n: gmp::Natural| {
        let n = gmp_natural_to_native(&gmp_n);
        assert_eq!((&n - 0).unwrap(), n);
        if n != 0 {
            assert!((0 - &n).is_none());
        }
    };

    // u - 0 == u
    // if u != 0, 0 - u == None
    let one_u32 = |u: u32| {
        assert_eq!(u - &native::Natural::from(0u32),
                   Some(native::Natural::from(u)));
        if u != 0 {
            assert!((native::Natural::from(0u32) - u).is_none());
        }
    };

    for (n, u) in exhaustive_pairs(exhaustive_naturals(), exhaustive_u::<u32>()).take(LARGE_LIMIT) {
        natural_and_u32(n, u);
    }

    for (n, u) in random_pairs(&EXAMPLE_SEED,
                               &(|seed| random_naturals(seed, 32)),
                               &(|seed| random_x(seed)))
                .take(LARGE_LIMIT) {
        natural_and_u32(n, u);
    }

    for n in exhaustive_naturals().take(LARGE_LIMIT) {
        one_natural(n);
    }

    for n in random_naturals(&EXAMPLE_SEED, 32).take(LARGE_LIMIT) {
        one_natural(n);
    }

    for n in exhaustive_u().take(LARGE_LIMIT) {
        one_u32(n);
    }

    for n in random_x(&EXAMPLE_SEED).take(LARGE_LIMIT) {
        one_u32(n);
    }
}
