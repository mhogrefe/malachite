use common::LARGE_LIMIT;
use malachite_base::num::Zero;
use malachite_nz::natural::Natural;
use malachite_test::common::{biguint_to_natural, natural_to_biguint, natural_to_rugint_integer,
                             rugint_integer_to_natural, GenerationMode};
use malachite_test::inputs::base::unsigneds;
use malachite_test::inputs::natural::{naturals, pairs_of_natural_and_unsigned};
use malachite_test::natural::arithmetic::add_u32::num_add_u32;
use num::BigUint;
use rugint;
use std::str::FromStr;

#[test]
fn test_add_u32() {
    let test = |u, v: u32, out| {
        let mut n = Natural::from_str(u).unwrap();
        n += v;
        assert_eq!(n.to_string(), out);
        assert!(n.is_valid());

        let mut n = rugint::Integer::from_str(u).unwrap();
        n += v;
        assert_eq!(n.to_string(), out);

        let n = Natural::from_str(u).unwrap() + v;
        assert_eq!(n.to_string(), out);
        assert!(n.is_valid());

        let n = num_add_u32(BigUint::from_str(u).unwrap(), v);
        assert_eq!(n.to_string(), out);

        let n = rugint::Integer::from_str(u).unwrap() + v;
        assert_eq!(n.to_string(), out);

        let n = &Natural::from_str(u).unwrap() + v;
        assert_eq!(n.to_string(), out);
        assert!(n.is_valid());

        let n = v + Natural::from_str(u).unwrap();
        assert_eq!(n.to_string(), out);
        assert!(n.is_valid());

        let n = v + rugint::Integer::from_str(u).unwrap();
        assert_eq!(n.to_string(), out);

        let n = v + &Natural::from_str(u).unwrap();
        assert_eq!(n.to_string(), out);
        assert!(n.is_valid());

        let n = v + &rugint::Integer::from_str(u).unwrap();
        assert_eq!(n.to_string(), out);
    };
    test("0", 0, "0");
    test("0", 123, "123");
    test("123", 0, "123");
    test("123", 456, "579");
    test("1000000000000", 123, "1000000000123");
    test("4294967295", 1, "4294967296");
    test("18446744073709551615", 1, "18446744073709551616");
}

#[test]
fn add_u32_properties() {
    // n += u is equivalent for malachite and rugint.
    // n + u is equivalent for malachite, num, and rugint.
    // &n + u is equivalent for malachite and num.
    // n += u; n is valid.
    // n + u and u + n are valid.
    // &n + u and u + &n are valid.
    // n += u, n + u, u + n, &n + u, and u + &n give the same result.
    // n + u == n + from(u)
    // n + u >= n and n + u >= u
    // n + u - u == n
    let natural_and_u32 = |mut n: Natural, u: u32| {
        let old_n = n.clone();
        n += u;
        assert!(n.is_valid());

        let mut rugint_n = natural_to_rugint_integer(&old_n);
        rugint_n += u;
        assert_eq!(rugint_integer_to_natural(&rugint_n), n);

        let n2 = old_n.clone();
        let result = &n2 + u;
        assert!(result.is_valid());
        assert_eq!(result, n);
        let result = n2 + u;
        assert!(result.is_valid());
        assert_eq!(result, n);

        let n2 = old_n.clone();
        let result = u + &n2;
        assert!(result.is_valid());
        assert_eq!(result, n);
        let result = u + n2;
        assert_eq!(result, n);
        assert!(result.is_valid());

        let n2 = old_n.clone();
        let result = n2 + Natural::from(u);
        assert_eq!(result, n);
        let n2 = old_n.clone();
        let result = Natural::from(u) + n2;
        assert_eq!(result, n);

        let num_n2 = natural_to_biguint(&old_n);
        assert_eq!(biguint_to_natural(&num_add_u32(num_n2, u)), n);

        let rugint_n2 = natural_to_rugint_integer(&old_n);
        assert_eq!(rugint_integer_to_natural(&(rugint_n2 + u)), n);

        assert!(n >= old_n);
        assert!(n >= u);
        assert_eq!(n - u, Some(old_n));
    };

    // n + 0 == n
    // 0 + n == n
    #[allow(unknown_lints, identity_op)]
    let one_natural = |n: Natural| {
        assert_eq!(&n + 0u32, n);
        assert_eq!(0u32 + &n, n);
    };

    // 0 + u == u
    // u + 0 == u
    let one_u32 = |u: u32| {
        assert_eq!(Natural::ZERO + u, u);
        assert_eq!(u + Natural::ZERO, u);
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
