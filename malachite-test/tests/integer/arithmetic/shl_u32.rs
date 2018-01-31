use common::LARGE_LIMIT;
use malachite_base::num::{One, Zero};
use malachite_nz::integer::Integer;
use malachite_test::common::{bigint_to_integer, integer_to_bigint, integer_to_rugint_integer,
                             rugint_integer_to_integer, GenerationMode};
use malachite_test::inputs::base::small_u32s;
use malachite_test::inputs::integer::{integers, pairs_of_integer_and_small_u32};
use num::BigInt;
use rugint;
use std::i32;
use std::str::FromStr;

#[test]
fn test_shl_u32() {
    let test = |u, v: u32, out| {
        let mut n = Integer::from_str(u).unwrap();
        n <<= v;
        assert_eq!(n.to_string(), out);
        assert!(n.is_valid());

        let mut n = rugint::Integer::from_str(u).unwrap();
        n <<= v;
        assert_eq!(n.to_string(), out);

        let n = Integer::from_str(u).unwrap() << v;
        assert_eq!(n.to_string(), out);
        assert!(n.is_valid());

        let n = rugint::Integer::from_str(u).unwrap() << v;
        assert_eq!(n.to_string(), out);

        let n = BigInt::from_str(u).unwrap() << v as usize;
        assert_eq!(n.to_string(), out);

        let n = &Integer::from_str(u).unwrap() << v;
        assert_eq!(n.to_string(), out);
        assert!(n.is_valid());

        let n = &BigInt::from_str(u).unwrap() << v as usize;
        assert_eq!(n.to_string(), out);
    };
    test("0", 0, "0");
    test("0", 10, "0");

    test("123", 0, "123");
    test("123", 1, "246");
    test("123", 2, "492");
    test("123", 25, "4127195136");
    test("123", 26, "8254390272");
    test("123", 100, "155921023828072216384094494261248");
    test("2147483648", 1, "4294967296");
    test("1000000000000", 0, "1000000000000");
    test("1000000000000", 3, "8000000000000");
    test("1000000000000", 24, "16777216000000000000");
    test("1000000000000", 25, "33554432000000000000");
    test("1000000000000", 31, "2147483648000000000000");
    test("1000000000000", 32, "4294967296000000000000");
    test("1000000000000", 33, "8589934592000000000000");
    test(
        "1000000000000",
        100,
        "1267650600228229401496703205376000000000000",
    );

    test("-123", 0, "-123");
    test("-123", 1, "-246");
    test("-123", 2, "-492");
    test("-123", 25, "-4127195136");
    test("-123", 26, "-8254390272");
    test("-123", 100, "-155921023828072216384094494261248");
    test("-2147483648", 1, "-4294967296");
    test("-1000000000000", 0, "-1000000000000");
    test("-1000000000000", 3, "-8000000000000");
    test("-1000000000000", 24, "-16777216000000000000");
    test("-1000000000000", 25, "-33554432000000000000");
    test("-1000000000000", 31, "-2147483648000000000000");
    test("-1000000000000", 32, "-4294967296000000000000");
    test("-1000000000000", 33, "-8589934592000000000000");
    test(
        "-1000000000000",
        100,
        "-1267650600228229401496703205376000000000000",
    );
}

#[test]
fn shl_u32_properties() {
    // n <<= u is equivalent for malachite and rugint.
    // n << u is equivalent for malachite, num, and rugint.
    // &n << u is equivalent for malachite and num.
    // n <<= u; n is valid.
    // n << u is valid.
    // &n << u is valid.
    // n <<= u, n << u, and &n << u give the same result.
    // |n << u| >= |n|
    // n << u == n * (1 << u)
    // n << u >> u == n
    // if u < 2^31, n << u == n << (u as i32) == n >> -(u as i32)
    let integer_and_u32 = |mut n: Integer, u: u32| {
        let old_n = n.clone();
        n <<= u;
        assert!(n.is_valid());

        let mut rugint_n = integer_to_rugint_integer(&old_n);
        rugint_n <<= u;
        assert_eq!(rugint_integer_to_integer(&rugint_n), n);

        let n2 = old_n.clone();
        let result = &n2 << u;
        assert_eq!(result, n);
        assert!(result.is_valid());
        let result = n2 << u;
        assert!(result.is_valid());
        assert_eq!(result, n);

        let num_n2 = integer_to_bigint(&old_n);
        assert_eq!(bigint_to_integer(&(&num_n2 << u as usize)), n);
        assert_eq!(bigint_to_integer(&(num_n2 << u as usize)), n);

        let rugint_n2 = integer_to_rugint_integer(&old_n);
        assert_eq!(rugint_integer_to_integer(&(rugint_n2 << u)), n);

        assert!((&old_n << u).abs() >= old_n.abs_ref());
        assert_eq!(-&old_n << u, -(&old_n << u));

        assert_eq!(&old_n << u, &old_n * (Integer::ONE << u));
        assert_eq!(&old_n << u >> u, old_n);

        if u <= (i32::MAX as u32) {
            assert_eq!(&old_n << (u as i32), n);
            assert_eq!(&old_n >> -(u as i32), n);
        }
    };

    // n << 0 == n
    #[allow(unknown_lints, identity_op)]
    let one_integer = |n: Integer| {
        assert_eq!(&n << 0, n);
    };

    // 0 << n == 0
    // 1 << n is a power of 2
    let one_u32 = |u: u32| {
        assert_eq!(Integer::ZERO << u, 0);
        assert!((Integer::ONE << u).into_natural().unwrap().is_power_of_2());
    };

    for (n, u) in pairs_of_integer_and_small_u32(GenerationMode::Exhaustive).take(LARGE_LIMIT) {
        integer_and_u32(n, u);
    }

    for (n, u) in pairs_of_integer_and_small_u32(GenerationMode::Random(32)).take(LARGE_LIMIT) {
        integer_and_u32(n, u);
    }

    for n in integers(GenerationMode::Exhaustive).take(LARGE_LIMIT) {
        one_integer(n);
    }

    for n in integers(GenerationMode::Random(32)).take(LARGE_LIMIT) {
        one_integer(n);
    }

    for n in small_u32s(GenerationMode::Exhaustive).take(LARGE_LIMIT) {
        one_u32(n);
    }

    for n in small_u32s(GenerationMode::Random(32)).take(LARGE_LIMIT) {
        one_u32(n);
    }
}
