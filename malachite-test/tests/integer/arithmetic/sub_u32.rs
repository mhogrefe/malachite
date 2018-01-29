use common::LARGE_LIMIT;
use malachite_base::traits::Zero;
use malachite_nz::integer::Integer;
use malachite_test::common::{bigint_to_integer, integer_to_bigint, integer_to_rugint_integer,
                             rugint_integer_to_integer, GenerationMode};
use malachite_test::integer::arithmetic::sub_u32::num_sub_u32;
use malachite_test::inputs::base::unsigneds;
use malachite_test::inputs::integer::{integers, pairs_of_integer_and_unsigned};
use num::BigInt;
use rugint;
use std::str::FromStr;

#[test]
fn test_sub_assign_u32() {
    let test = |u, v: u32, out| {
        let mut n = Integer::from_str(u).unwrap();
        n -= v;
        assert_eq!(n.to_string(), out);
        assert!(n.is_valid());

        let mut n = rugint::Integer::from_str(u).unwrap();
        n -= v;
        assert_eq!(n.to_string(), out);

        let n = Integer::from_str(u).unwrap() - v;
        assert_eq!(n.to_string(), out);
        assert!(n.is_valid());

        let n = num_sub_u32(BigInt::from_str(u).unwrap(), v);
        assert_eq!(n.to_string(), out);

        let n = rugint::Integer::from_str(u).unwrap() - v;
        assert_eq!(n.to_string(), out);

        let n = &Integer::from_str(u).unwrap() - v;
        assert_eq!(n.to_string(), out);
        assert!(n.is_valid());

        let n = v - Integer::from_str(u).unwrap();
        assert!(n.is_valid());
        assert_eq!((-n).to_string(), out);

        let n = v - rugint::Integer::from_str(u).unwrap();
        assert_eq!((-n).to_string(), out);

        let n = v - &Integer::from_str(u).unwrap();
        assert!(n.is_valid());
        assert_eq!((-n).to_string(), out);

        let n = v - &rugint::Integer::from_str(u).unwrap();
        assert_eq!((-n).to_string(), out);
    };
    test("0", 0, "0");
    test("123", 123, "0");
    test("123", 0, "123");
    test("456", 123, "333");
    test("123", 456, "-333");
    test("-456", 123, "-579");
    test("1000000000000", 123, "999999999877");
    test("-1000000000000", 123, "-1000000000123");
    test("4294967296", 1, "4294967295");
    test("-4294967295", 1, "-4294967296");
    test("2147483648", 1, "2147483647");
    test("-2147483647", 1, "-2147483648");
    test("18446744073709551616", 1, "18446744073709551615");
    test("-18446744073709551615", 1, "-18446744073709551616");
}

#[test]
fn sub_u32_properties() {
    // n -= u is equivalent for malachite and rugint.
    // n - u is equivalent for malachite, num, and rugint.
    // &n - u is equivalent for malachite and num.
    // n -= u; n is valid.
    // n - u and u - n are valid.
    // &n - u and u - &n are valid.
    // n -= u, n - u, and &n - u give the same result.
    // u - n and u - &n give the same result.
    // u - n == -(n - u)
    // n - u == n - from(u)
    // n - u + u == n
    // n - (n - u) == u
    let integer_and_u32 = |mut n: Integer, u: u32| {
        let old_n = n.clone();
        n -= u;
        assert!(n.is_valid());

        let mut rugint_n = integer_to_rugint_integer(&old_n);
        rugint_n -= u;
        assert_eq!(rugint_integer_to_integer(&rugint_n), n);

        let n2 = old_n.clone();
        let result = &n2 - u;
        assert!(result.is_valid());
        assert_eq!(result, n);
        let result = n2 - u;
        assert!(result.is_valid());
        assert_eq!(result, n);

        let n2 = old_n.clone();
        let result = u - &n2;
        assert!(result.is_valid());
        assert_eq!(result, -&n);
        let result = u - n2;
        assert_eq!(result, -&n);
        assert!(result.is_valid());

        let n2 = old_n.clone();
        let result = n2 - Integer::from(u);
        assert_eq!(result, n);
        let n2 = old_n.clone();
        let result = Integer::from(u) - n2;
        assert_eq!(result, -&n);

        let num_n2 = integer_to_bigint(&old_n);
        assert_eq!(bigint_to_integer(&num_sub_u32(num_n2, u)), n);

        let rugint_n2 = integer_to_rugint_integer(&old_n);
        assert_eq!(rugint_integer_to_integer(&(rugint_n2 - u)), n);

        assert_eq!(&n + u, old_n);
        assert_eq!(old_n - n, u);
    };

    // n - 0 == n
    // 0 - n == -n
    #[allow(unknown_lints, identity_op)]
    let one_integer = |n: Integer| {
        assert_eq!(&n + 0u32, n);
        assert_eq!(0u32 - &n, -n);
    };

    // 0 - u == u
    // u - 0 == u
    let one_u32 = |u: u32| {
        assert_eq!(Integer::ZERO - u, -Integer::from(u));
        assert_eq!(u - Integer::ZERO, u);
    };

    for (n, u) in pairs_of_integer_and_unsigned(GenerationMode::Exhaustive).take(LARGE_LIMIT) {
        integer_and_u32(n, u);
    }

    for (n, u) in pairs_of_integer_and_unsigned(GenerationMode::Random(32)).take(LARGE_LIMIT) {
        integer_and_u32(n, u);
    }

    for n in integers(GenerationMode::Exhaustive).take(LARGE_LIMIT) {
        one_integer(n);
    }

    for n in integers(GenerationMode::Random(32)).take(LARGE_LIMIT) {
        one_integer(n);
    }

    for n in unsigneds(GenerationMode::Exhaustive).take(LARGE_LIMIT) {
        one_u32(n);
    }

    for n in unsigneds(GenerationMode::Random(32)).take(LARGE_LIMIT) {
        one_u32(n);
    }
}
