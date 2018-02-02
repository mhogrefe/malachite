use common::LARGE_LIMIT;
use malachite_base::num::Zero;
use malachite_nz::natural::Natural;
use malachite_test::common::{biguint_to_natural, natural_to_biguint, natural_to_rug_integer,
                             rug_integer_to_natural, GenerationMode};
use malachite_test::inputs::base::unsigneds;
use malachite_test::inputs::natural::{naturals, pairs_of_natural_and_unsigned};
use malachite_test::natural::arithmetic::sub_u32::{num_sub_u32, rug_sub_u32};
use num::BigUint;
use rug;
use std::str::FromStr;
use std::u32;

#[test]
fn test_sub_assign_u32() {
    let test = |u, v: u32, out| {
        let mut n = Natural::from_str(u).unwrap();
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
fn sub_assign_u32_fail() {
    let mut x = Natural::from_str("123").unwrap();
    x -= 456;
}

#[test]
fn test_sub_u32() {
    let test = |u, v: u32, out| {
        let on = Natural::from_str(u).unwrap() - v;
        assert_eq!(format!("{:?}", on), out);
        assert!(on.map_or(true, |n| n.is_valid()));

        let on = &Natural::from_str(u).unwrap() - v;
        assert_eq!(format!("{:?}", on), out);
        assert!(on.map_or(true, |n| n.is_valid()));

        let ux = BigUint::from_str(u).unwrap();
        let on = num_sub_u32(ux, v).map(|x| biguint_to_natural(&x));
        assert_eq!(format!("{:?}", on), out);

        let on = rug_sub_u32(rug::Integer::from_str(u).unwrap(), v);
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
        let on = u - &Natural::from_str(v).unwrap();
        assert_eq!(format!("{:?}", on), out);
        assert!(on.map_or(true, |n| n.is_valid()));
    };
    test(0, "0", "Some(0)");
    test(123, "123", "Some(0)");
    test(123, "0", "Some(123)");
    test(456, "123", "Some(333)");
    test(123, "456", "None");
    test(123, "1000000000000", "None");
    test(u32::MAX, "4294967295", "Some(0)");
}

#[test]
fn sub_u32_properties() {
    // n -= u is equivalent for malachite and rug.
    // n - u is equivalent for malachite, num, and rug.
    // &n - u is equivalent for malachite and num.
    // n -= u; n is valid.
    // n - u is valid.
    // &n - u and u - &n are valid.
    // n -= u, n - u, and &n - u give the same result.
    // n - u == n - from(u)
    // u - n == from(u) - n
    // if n >= u, n - u <= n
    // if n >= u, (n - u).unwrap() + u == n
    let natural_and_u32 = |mut n: Natural, u: u32| {
        let old_n = n.clone();
        if n >= u {
            n -= u;
            assert!(n.is_valid());

            let mut rug_n = natural_to_rug_integer(&old_n);
            rug_n -= u;
            assert_eq!(rug_integer_to_natural(&rug_n), n);
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
            assert_eq!(Natural::from(u) - n2.to_u32().unwrap(), result);
        }
        assert!(result.map_or(true, |n| n.is_valid()));

        let n2 = old_n.clone();
        let result = n2 - &Natural::from(u);
        assert_eq!(result, on);
        let n2 = old_n.clone();
        assert_eq!(u - &n2, Natural::from(u) - &n2);

        let num_n2 = natural_to_biguint(&old_n);
        assert_eq!(num_sub_u32(num_n2, u).map(|x| biguint_to_natural(&x)), on);

        let rug_n2 = natural_to_rug_integer(&old_n);
        assert_eq!(
            rug_sub_u32(rug_n2, u).map(|x| rug_integer_to_natural(&x)),
            on
        );

        if on.is_some() {
            assert!(on.clone().unwrap() <= old_n);
            assert_eq!(on.unwrap() + u, old_n);
        }
    };

    // n - 0 == n
    // if n != 0, 0 - n == None
    #[allow(unknown_lints, identity_op)]
    let one_natural = |n: Natural| {
        assert_eq!((&n - 0).unwrap(), n);
        if n != 0 {
            assert!((0 - &n).is_none());
        }
    };

    // u - 0 == u
    // if u != 0, 0 - u == None
    let one_u32 = |u: u32| {
        assert_eq!(u - &Natural::ZERO, Some(Natural::from(u)));
        if u != 0 {
            assert!((Natural::ZERO - u).is_none());
        }
    };

    for (n, u) in pairs_of_natural_and_unsigned(GenerationMode::Exhaustive).take(LARGE_LIMIT) {
        natural_and_u32(n, u);
    }

    for (n, u) in pairs_of_natural_and_unsigned(GenerationMode::Random(32)).take(LARGE_LIMIT) {
        natural_and_u32(n, u);
    }

    for n in naturals(GenerationMode::Exhaustive).take(LARGE_LIMIT) {
        one_natural(n);
    }

    for n in naturals(GenerationMode::Random(32)).take(LARGE_LIMIT) {
        one_natural(n);
    }

    for n in unsigneds(GenerationMode::Exhaustive).take(LARGE_LIMIT) {
        one_u32(n);
    }

    for n in unsigneds(GenerationMode::Random(32)).take(LARGE_LIMIT) {
        one_u32(n);
    }
}
