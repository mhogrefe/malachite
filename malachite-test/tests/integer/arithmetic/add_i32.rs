use common::LARGE_LIMIT;
use malachite_base::num::Zero;
use malachite_nz::integer::Integer;
use malachite_test::common::{bigint_to_integer, integer_to_bigint, integer_to_rugint_integer,
                             rugint_integer_to_integer, GenerationMode};
use malachite_test::inputs::base::signeds;
use malachite_test::inputs::integer::{integers, pairs_of_integer_and_signed};
use malachite_test::integer::arithmetic::add_i32::num_add_i32;
use num::BigInt;
use rugint;
use std::str::FromStr;

#[test]
fn test_add_i32() {
    let test = |u, v: i32, out| {
        let mut n = Integer::from_str(u).unwrap();
        n += v;
        assert_eq!(n.to_string(), out);
        assert!(n.is_valid());

        let mut n = rugint::Integer::from_str(u).unwrap();
        n += v;
        assert_eq!(n.to_string(), out);

        let n = Integer::from_str(u).unwrap() + v;
        assert_eq!(n.to_string(), out);
        assert!(n.is_valid());

        let n = num_add_i32(BigInt::from_str(u).unwrap(), v);
        assert_eq!(n.to_string(), out);

        let n = rugint::Integer::from_str(u).unwrap() + v;
        assert_eq!(n.to_string(), out);

        let n = &Integer::from_str(u).unwrap() + v;
        assert_eq!(n.to_string(), out);
        assert!(n.is_valid());

        let n = v + Integer::from_str(u).unwrap();
        assert_eq!(n.to_string(), out);
        assert!(n.is_valid());

        let n = v + rugint::Integer::from_str(u).unwrap();
        assert_eq!(n.to_string(), out);

        let n = v + &Integer::from_str(u).unwrap();
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
    // n += i is equivalent for malachite and rugint.
    // n + i is equivalent for malachite, num, and rugint.
    // &n + i is equivalent for malachite and num.
    // n += i; n is valid.
    // n + i and i + n are valid.
    // &n + i and i + &n are valid.
    // n += i, n + i, i + n, &n + i, and i + &n give the same result.
    // n + i == n + from(u)
    // n + i - i == n
    // n + i - n == i
    let integer_and_i32 = |mut n: Integer, i: i32| {
        let old_n = n.clone();
        n += i;
        assert!(n.is_valid());

        let mut rugint_n = integer_to_rugint_integer(&old_n);
        rugint_n += i;
        assert_eq!(rugint_integer_to_integer(&rugint_n), n);

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
        let result = n2 + Integer::from(i);
        assert_eq!(result, n);
        let n2 = old_n.clone();
        let result = Integer::from(i) + n2;
        assert_eq!(result, n);

        let num_n2 = integer_to_bigint(&old_n);
        assert_eq!(bigint_to_integer(&num_add_i32(num_n2, i)), n);

        let rugint_n2 = integer_to_rugint_integer(&old_n);
        assert_eq!(rugint_integer_to_integer(&(rugint_n2 + i)), n);

        assert_eq!(&n - i, old_n);
        assert_eq!(n - old_n, i);
    };

    // n + 0 == n
    // 0 + n == n
    #[allow(unknown_lints, identity_op)]
    let one_integer = |n: Integer| {
        assert_eq!(&n + 0i32, n);
        assert_eq!(0i32 + &n, n);
    };

    // 0 + i == i
    // i + 0 == i
    let one_i32 = |i: i32| {
        assert_eq!(Integer::ZERO + i, Integer::from(i));
        assert_eq!(i + Integer::ZERO, Integer::from(i));
    };

    for (n, i) in pairs_of_integer_and_signed(GenerationMode::Exhaustive).take(LARGE_LIMIT) {
        integer_and_i32(n, i);
    }

    for (n, i) in pairs_of_integer_and_signed(GenerationMode::Random(32)).take(LARGE_LIMIT) {
        integer_and_i32(n, i);
    }

    for n in integers(GenerationMode::Exhaustive).take(LARGE_LIMIT) {
        one_integer(n);
    }

    for n in integers(GenerationMode::Random(32)).take(LARGE_LIMIT) {
        one_integer(n);
    }

    for n in signeds(GenerationMode::Exhaustive).take(LARGE_LIMIT) {
        one_i32(n);
    }

    for n in signeds(GenerationMode::Random(32)).take(LARGE_LIMIT) {
        one_i32(n);
    }
}
