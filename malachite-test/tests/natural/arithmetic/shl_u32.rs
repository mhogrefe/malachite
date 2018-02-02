use common::LARGE_LIMIT;
use malachite_base::num::{One, Zero};
use malachite_nz::natural::Natural;
use malachite_test::common::{biguint_to_natural, natural_to_biguint, natural_to_rug_integer,
                             rug_integer_to_natural, GenerationMode};
use malachite_test::inputs::base::small_u32s;
use malachite_test::inputs::natural::{naturals, pairs_of_natural_and_small_u32,
                                      triples_of_natural_small_u32_and_small_u32};
use num::BigUint;
use rug;
use std::i32;
use std::str::FromStr;

#[test]
fn test_shl_u32() {
    let test = |u, v: u32, out| {
        let mut n = Natural::from_str(u).unwrap();
        n <<= v;
        assert_eq!(n.to_string(), out);
        assert!(n.is_valid());

        let mut n = rug::Integer::from_str(u).unwrap();
        n <<= v;
        assert_eq!(n.to_string(), out);

        let n = Natural::from_str(u).unwrap() << v;
        assert_eq!(n.to_string(), out);
        assert!(n.is_valid());

        let n = rug::Integer::from_str(u).unwrap() << v;
        assert_eq!(n.to_string(), out);

        let n = BigUint::from_str(u).unwrap() << v as usize;
        assert_eq!(n.to_string(), out);

        let n = &Natural::from_str(u).unwrap() << v;
        assert_eq!(n.to_string(), out);
        assert!(n.is_valid());

        let n = &BigUint::from_str(u).unwrap() << v as usize;
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
}

#[test]
fn shl_u32_properties() {
    // n <<= u is equivalent for malachite and rug.
    // n << u is equivalent for malachite, num, and rug.
    // &n << u is equivalent for malachite and num.
    // n <<= u; n is valid.
    // n << u is valid.
    // &n << u is valid.
    // n <<= u, n << u, and &n << u give the same result.
    // n << u >= n
    // n << u == n * (1 << u)
    // n << u >> u == n
    // if u < 2^31, n << u == n << (u as i32) == n >> -(u as i32)
    let natural_and_u32 = |mut n: Natural, u: u32| {
        let old_n = n.clone();
        n <<= u;
        assert!(n.is_valid());

        let mut rug_n = natural_to_rug_integer(&old_n);
        rug_n <<= u;
        assert_eq!(rug_integer_to_natural(&rug_n), n);

        let n2 = old_n.clone();
        let result = &n2 << u;
        assert_eq!(result, n);
        assert!(result.is_valid());
        let result = n2 << u;
        assert!(result.is_valid());
        assert_eq!(result, n);

        let num_n2 = natural_to_biguint(&old_n);
        assert_eq!(biguint_to_natural(&(&num_n2 << u as usize)), n);
        assert_eq!(biguint_to_natural(&(num_n2 << u as usize)), n);

        let rug_n2 = natural_to_rug_integer(&old_n);
        assert_eq!(rug_integer_to_natural(&(rug_n2 << u)), n);

        assert!(&old_n << u >= old_n);
        assert_eq!(&old_n << u, &old_n * (Natural::ONE << u));
        assert_eq!(&old_n << u >> u, old_n);

        if u <= (i32::MAX as u32) {
            assert_eq!(&old_n << (u as i32), n);
            assert_eq!(&old_n >> -(u as i32), n);
        }
    };

    // n << u << v == n << (u + v)
    let natural_and_two_u32s = |n: Natural, u: u32, v: u32| {
        assert_eq!(&n << u << v, &n << (u + v));
    };

    // n << 0 == n
    #[allow(unknown_lints, identity_op)]
    let one_natural = |n: Natural| {
        assert_eq!(&n << 0, n);
    };

    // 0 << n == 0
    // 1 << n is a power of 2
    let one_u32 = |u: u32| {
        assert_eq!(Natural::ZERO << u, 0);
        assert!((Natural::ONE << u).is_power_of_2());
    };

    for (n, u) in pairs_of_natural_and_small_u32(GenerationMode::Exhaustive).take(LARGE_LIMIT) {
        natural_and_u32(n, u);
    }

    for (n, u) in pairs_of_natural_and_small_u32(GenerationMode::Random(32)).take(LARGE_LIMIT) {
        natural_and_u32(n, u);
    }

    for (n, u, v) in
        triples_of_natural_small_u32_and_small_u32(GenerationMode::Exhaustive).take(LARGE_LIMIT)
    {
        natural_and_two_u32s(n, u, v);
    }

    for (n, u, v) in
        triples_of_natural_small_u32_and_small_u32(GenerationMode::Random(32)).take(LARGE_LIMIT)
    {
        natural_and_two_u32s(n, u, v);
    }

    for n in naturals(GenerationMode::Exhaustive).take(LARGE_LIMIT) {
        one_natural(n);
    }

    for n in naturals(GenerationMode::Random(32)).take(LARGE_LIMIT) {
        one_natural(n);
    }

    for n in small_u32s(GenerationMode::Exhaustive).take(LARGE_LIMIT) {
        one_u32(n);
    }

    for n in small_u32s(GenerationMode::Random(32)).take(LARGE_LIMIT) {
        one_u32(n);
    }
}
