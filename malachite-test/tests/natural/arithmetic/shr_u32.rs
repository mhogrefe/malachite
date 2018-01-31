use common::LARGE_LIMIT;
use malachite_base::round::RoundingMode;
use malachite_base::num::{ShrRound, ShrRoundAssign, Zero};
use malachite_nz::natural::Natural;
use malachite_test::common::{biguint_to_natural, natural_to_biguint, natural_to_rugint_integer,
                             rugint_integer_to_natural, GenerationMode};
use malachite_test::inputs::base::{pairs_of_unsigned_and_rounding_mode, unsigneds,
                                   pairs_of_positive_unsigned_and_small_u32,
                                   pairs_of_unsigned_and_small_u32};
use malachite_test::inputs::natural::{naturals, pairs_of_natural_and_rounding_mode,
                                      pairs_of_natural_and_small_u32,
                                      pairs_of_natural_and_small_u32_var_2,
                                      triples_of_natural_small_u32_and_rounding_mode_var_1,
                                      triples_of_natural_small_u32_and_small_u32};
use num::BigUint;
use rugint;
use std::i32;
use std::str::FromStr;

#[test]
fn test_shr_u32() {
    let test = |u, v: u32, out| {
        let mut n = Natural::from_str(u).unwrap();
        n >>= v;
        assert_eq!(n.to_string(), out);
        assert!(n.is_valid());

        let mut n = rugint::Integer::from_str(u).unwrap();
        n >>= v;
        assert_eq!(n.to_string(), out);

        let n = Natural::from_str(u).unwrap() >> v;
        assert_eq!(n.to_string(), out);
        assert!(n.is_valid());

        let n = rugint::Integer::from_str(u).unwrap() >> v;
        assert_eq!(n.to_string(), out);

        let n = BigUint::from_str(u).unwrap() >> v as usize;
        assert_eq!(n.to_string(), out);

        let n = &Natural::from_str(u).unwrap() >> v;
        assert_eq!(n.to_string(), out);
        assert!(n.is_valid());

        let n = &BigUint::from_str(u).unwrap() >> v as usize;
        assert_eq!(n.to_string(), out);
    };
    test("0", 0, "0");
    test("0", 10, "0");
    test("123", 0, "123");
    test("245", 1, "122");
    test("246", 1, "123");
    test("247", 1, "123");
    test("491", 2, "122");
    test("492", 2, "123");
    test("493", 2, "123");
    test("4127195135", 25, "122");
    test("4127195136", 25, "123");
    test("4127195137", 25, "123");
    test("8254390271", 26, "122");
    test("8254390272", 26, "123");
    test("8254390273", 26, "123");
    test("155921023828072216384094494261247", 100, "122");
    test("155921023828072216384094494261248", 100, "123");
    test("155921023828072216384094494261249", 100, "123");
    test("4294967295", 1, "2147483647");
    test("4294967296", 1, "2147483648");
    test("4294967297", 1, "2147483648");
    test("1000000000000", 0, "1000000000000");
    test("7999999999999", 3, "999999999999");
    test("8000000000000", 3, "1000000000000");
    test("8000000000001", 3, "1000000000000");
    test("16777216000000000000", 24, "1000000000000");
    test("33554432000000000000", 25, "1000000000000");
    test("2147483648000000000000", 31, "1000000000000");
    test("4294967296000000000000", 32, "1000000000000");
    test("8589934592000000000000", 33, "1000000000000");
    test(
        "1267650600228229401496703205376000000000000",
        100,
        "1000000000000",
    );
    test("1000000000000", 10, "976562500");
    test("980657949", 72, "0");
    test("4294967295", 31, "1");
    test("4294967295", 32, "0");
    test("4294967296", 32, "1");
    test("4294967296", 33, "0");
}

#[test]
fn shr_u32_properties() {
    // n >>= u is equivalent for malachite and rugint.
    // n >> u is equivalent for malachite, num, and rugint.
    // &n >> u is equivalent for malachite and num.
    // n >>= u; n is valid.
    // n >> u is valid.
    // &n >> u is valid.
    // n >>= u, n >> u, and &n >> u give the same result.
    // n >> u <= n
    // TODO n >> u == n / (1 << u)
    // n >> u == n.shr_round(u, Floor)
    // if u < 2^31, n >> u == n >> (u as i32) == n << -(u as i32)
    let natural_and_u32 = |mut n: Natural, u: u32| {
        let old_n = n.clone();
        n >>= u;
        assert!(n.is_valid());

        let mut rugint_n = natural_to_rugint_integer(&old_n);
        rugint_n >>= u;
        assert_eq!(rugint_integer_to_natural(&rugint_n), n);

        let n2 = old_n.clone();
        let result = &n2 >> u;
        assert_eq!(result, n);
        assert!(result.is_valid());
        let result = n2 >> u;
        assert!(result.is_valid());
        assert_eq!(result, n);

        let num_n2 = natural_to_biguint(&old_n);
        assert_eq!(biguint_to_natural(&(&num_n2 >> u as usize)), n);
        assert_eq!(biguint_to_natural(&(num_n2 >> u as usize)), n);

        let rugint_n2 = natural_to_rugint_integer(&old_n);
        assert_eq!(rugint_integer_to_natural(&(rugint_n2 >> u)), n);

        assert!(&old_n >> u <= old_n);
        assert_eq!(&old_n >> u, (&old_n).shr_round(u, RoundingMode::Floor));

        if u <= (i32::MAX as u32) {
            assert_eq!(&old_n >> (u as i32), n);
            assert_eq!(&old_n << -(u as i32), n);
        }
    };

    // if u >= 32, n >> u == 0
    let two_u32s = |u: u32, v: u32| {
        assert_eq!(Natural::from(u) >> (v + 32), 0);
    };

    // n >> u >> v == n >> (u + v)
    let natural_and_two_u32s = |n: Natural, u: u32, v: u32| {
        assert_eq!(&n >> u >> v, &n >> (u + v));
    };

    // n >> 0 == n
    #[allow(unknown_lints, identity_op)]
    let one_natural = |n: Natural| {
        assert_eq!(&n >> 0, n);
    };

    // 0 >> n == 0
    let one_u32 = |u: u32| {
        assert_eq!(Natural::ZERO >> u, 0);
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

    for (n, u) in pairs_of_unsigned_and_small_u32(GenerationMode::Exhaustive).take(LARGE_LIMIT) {
        two_u32s(n, u);
    }

    for (n, u) in pairs_of_unsigned_and_small_u32(GenerationMode::Random(32)).take(LARGE_LIMIT) {
        two_u32s(n, u);
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

#[test]
fn test_shr_round_u32() {
    let test = |u, v: u32, rm: RoundingMode, out| {
        let mut n = Natural::from_str(u).unwrap();
        n.shr_round_assign(v, rm);
        assert_eq!(n.to_string(), out);
        assert!(n.is_valid());

        let n = Natural::from_str(u).unwrap().shr_round(v, rm);
        assert_eq!(n.to_string(), out);
        assert!(n.is_valid());

        let n = &Natural::from_str(u).unwrap().shr_round(v, rm);
        assert_eq!(n.to_string(), out);
        assert!(n.is_valid());
    };
    test("0", 0, RoundingMode::Down, "0");
    test("0", 0, RoundingMode::Up, "0");
    test("0", 0, RoundingMode::Floor, "0");
    test("0", 0, RoundingMode::Ceiling, "0");
    test("0", 0, RoundingMode::Nearest, "0");
    test("0", 0, RoundingMode::Exact, "0");

    test("0", 10, RoundingMode::Down, "0");
    test("0", 10, RoundingMode::Up, "0");
    test("0", 10, RoundingMode::Floor, "0");
    test("0", 10, RoundingMode::Ceiling, "0");
    test("0", 10, RoundingMode::Nearest, "0");
    test("0", 10, RoundingMode::Exact, "0");

    test("123", 0, RoundingMode::Down, "123");
    test("123", 0, RoundingMode::Up, "123");
    test("123", 0, RoundingMode::Floor, "123");
    test("123", 0, RoundingMode::Ceiling, "123");
    test("123", 0, RoundingMode::Nearest, "123");
    test("123", 0, RoundingMode::Exact, "123");

    test("245", 1, RoundingMode::Down, "122");
    test("245", 1, RoundingMode::Up, "123");
    test("245", 1, RoundingMode::Floor, "122");
    test("245", 1, RoundingMode::Ceiling, "123");
    test("245", 1, RoundingMode::Nearest, "122");

    test("246", 1, RoundingMode::Down, "123");
    test("246", 1, RoundingMode::Up, "123");
    test("246", 1, RoundingMode::Floor, "123");
    test("246", 1, RoundingMode::Ceiling, "123");
    test("246", 1, RoundingMode::Nearest, "123");
    test("246", 1, RoundingMode::Exact, "123");

    test("247", 1, RoundingMode::Down, "123");
    test("247", 1, RoundingMode::Up, "124");
    test("247", 1, RoundingMode::Floor, "123");
    test("247", 1, RoundingMode::Ceiling, "124");
    test("247", 1, RoundingMode::Nearest, "124");

    test("491", 2, RoundingMode::Down, "122");
    test("491", 2, RoundingMode::Up, "123");
    test("491", 2, RoundingMode::Floor, "122");
    test("491", 2, RoundingMode::Ceiling, "123");
    test("491", 2, RoundingMode::Nearest, "123");

    test("492", 2, RoundingMode::Down, "123");
    test("492", 2, RoundingMode::Up, "123");
    test("492", 2, RoundingMode::Floor, "123");
    test("492", 2, RoundingMode::Ceiling, "123");
    test("492", 2, RoundingMode::Nearest, "123");
    test("492", 2, RoundingMode::Exact, "123");

    test("493", 2, RoundingMode::Down, "123");
    test("493", 2, RoundingMode::Up, "124");
    test("493", 2, RoundingMode::Floor, "123");
    test("493", 2, RoundingMode::Ceiling, "124");
    test("493", 2, RoundingMode::Nearest, "123");

    test("4127195135", 25, RoundingMode::Down, "122");
    test("4127195135", 25, RoundingMode::Up, "123");
    test("4127195135", 25, RoundingMode::Floor, "122");
    test("4127195135", 25, RoundingMode::Ceiling, "123");
    test("4127195135", 25, RoundingMode::Nearest, "123");

    test("4127195136", 25, RoundingMode::Down, "123");
    test("4127195136", 25, RoundingMode::Up, "123");
    test("4127195136", 25, RoundingMode::Floor, "123");
    test("4127195136", 25, RoundingMode::Ceiling, "123");
    test("4127195136", 25, RoundingMode::Nearest, "123");
    test("4127195136", 25, RoundingMode::Exact, "123");

    test("4127195137", 25, RoundingMode::Down, "123");
    test("4127195137", 25, RoundingMode::Up, "124");
    test("4127195137", 25, RoundingMode::Floor, "123");
    test("4127195137", 25, RoundingMode::Ceiling, "124");
    test("4127195137", 25, RoundingMode::Nearest, "123");

    test("8254390271", 26, RoundingMode::Down, "122");
    test("8254390271", 26, RoundingMode::Up, "123");
    test("8254390271", 26, RoundingMode::Floor, "122");
    test("8254390271", 26, RoundingMode::Ceiling, "123");
    test("8254390271", 26, RoundingMode::Nearest, "123");

    test("8254390272", 26, RoundingMode::Down, "123");
    test("8254390272", 26, RoundingMode::Up, "123");
    test("8254390272", 26, RoundingMode::Floor, "123");
    test("8254390272", 26, RoundingMode::Ceiling, "123");
    test("8254390272", 26, RoundingMode::Nearest, "123");
    test("8254390272", 26, RoundingMode::Exact, "123");

    test("8254390273", 26, RoundingMode::Down, "123");
    test("8254390273", 26, RoundingMode::Up, "124");
    test("8254390273", 26, RoundingMode::Floor, "123");
    test("8254390273", 26, RoundingMode::Ceiling, "124");
    test("8254390273", 26, RoundingMode::Nearest, "123");

    test(
        "155921023828072216384094494261247",
        100,
        RoundingMode::Down,
        "122",
    );
    test(
        "155921023828072216384094494261247",
        100,
        RoundingMode::Up,
        "123",
    );
    test(
        "155921023828072216384094494261247",
        100,
        RoundingMode::Floor,
        "122",
    );
    test(
        "155921023828072216384094494261247",
        100,
        RoundingMode::Ceiling,
        "123",
    );
    test(
        "155921023828072216384094494261247",
        100,
        RoundingMode::Nearest,
        "123",
    );

    test(
        "155921023828072216384094494261248",
        100,
        RoundingMode::Down,
        "123",
    );
    test(
        "155921023828072216384094494261248",
        100,
        RoundingMode::Up,
        "123",
    );
    test(
        "155921023828072216384094494261248",
        100,
        RoundingMode::Floor,
        "123",
    );
    test(
        "155921023828072216384094494261248",
        100,
        RoundingMode::Ceiling,
        "123",
    );
    test(
        "155921023828072216384094494261248",
        100,
        RoundingMode::Nearest,
        "123",
    );
    test(
        "155921023828072216384094494261248",
        100,
        RoundingMode::Exact,
        "123",
    );

    test(
        "155921023828072216384094494261249",
        100,
        RoundingMode::Down,
        "123",
    );
    test(
        "155921023828072216384094494261249",
        100,
        RoundingMode::Up,
        "124",
    );
    test(
        "155921023828072216384094494261249",
        100,
        RoundingMode::Floor,
        "123",
    );
    test(
        "155921023828072216384094494261249",
        100,
        RoundingMode::Ceiling,
        "124",
    );
    test(
        "155921023828072216384094494261249",
        100,
        RoundingMode::Nearest,
        "123",
    );

    test("4294967295", 1, RoundingMode::Down, "2147483647");
    test("4294967295", 1, RoundingMode::Up, "2147483648");
    test("4294967295", 1, RoundingMode::Floor, "2147483647");
    test("4294967295", 1, RoundingMode::Ceiling, "2147483648");
    test("4294967295", 1, RoundingMode::Nearest, "2147483648");

    test("4294967296", 1, RoundingMode::Down, "2147483648");
    test("4294967296", 1, RoundingMode::Up, "2147483648");
    test("4294967296", 1, RoundingMode::Floor, "2147483648");
    test("4294967296", 1, RoundingMode::Ceiling, "2147483648");
    test("4294967296", 1, RoundingMode::Nearest, "2147483648");
    test("4294967296", 1, RoundingMode::Exact, "2147483648");

    test("4294967297", 1, RoundingMode::Down, "2147483648");
    test("4294967297", 1, RoundingMode::Up, "2147483649");
    test("4294967297", 1, RoundingMode::Floor, "2147483648");
    test("4294967297", 1, RoundingMode::Ceiling, "2147483649");
    test("4294967297", 1, RoundingMode::Nearest, "2147483648");

    test("1000000000000", 0, RoundingMode::Down, "1000000000000");
    test("1000000000000", 0, RoundingMode::Up, "1000000000000");
    test("1000000000000", 0, RoundingMode::Floor, "1000000000000");
    test("1000000000000", 0, RoundingMode::Ceiling, "1000000000000");
    test("1000000000000", 0, RoundingMode::Nearest, "1000000000000");
    test("1000000000000", 0, RoundingMode::Exact, "1000000000000");

    test("7999999999999", 3, RoundingMode::Down, "999999999999");
    test("7999999999999", 3, RoundingMode::Up, "1000000000000");
    test("7999999999999", 3, RoundingMode::Floor, "999999999999");
    test("7999999999999", 3, RoundingMode::Ceiling, "1000000000000");
    test("7999999999999", 3, RoundingMode::Nearest, "1000000000000");

    test("8000000000000", 3, RoundingMode::Down, "1000000000000");
    test("8000000000000", 3, RoundingMode::Up, "1000000000000");
    test("8000000000000", 3, RoundingMode::Floor, "1000000000000");
    test("8000000000000", 3, RoundingMode::Ceiling, "1000000000000");
    test("8000000000000", 3, RoundingMode::Nearest, "1000000000000");
    test("8000000000000", 3, RoundingMode::Exact, "1000000000000");

    test("8000000000001", 3, RoundingMode::Down, "1000000000000");
    test("8000000000001", 3, RoundingMode::Up, "1000000000001");
    test("8000000000001", 3, RoundingMode::Floor, "1000000000000");
    test("8000000000001", 3, RoundingMode::Ceiling, "1000000000001");
    test("8000000000001", 3, RoundingMode::Nearest, "1000000000000");

    test(
        "16777216000000000000",
        24,
        RoundingMode::Down,
        "1000000000000",
    );
    test(
        "16777216000000000000",
        24,
        RoundingMode::Up,
        "1000000000000",
    );
    test(
        "16777216000000000000",
        24,
        RoundingMode::Floor,
        "1000000000000",
    );
    test(
        "16777216000000000000",
        24,
        RoundingMode::Ceiling,
        "1000000000000",
    );
    test(
        "16777216000000000000",
        24,
        RoundingMode::Nearest,
        "1000000000000",
    );
    test(
        "16777216000000000000",
        24,
        RoundingMode::Exact,
        "1000000000000",
    );

    test(
        "33554432000000000000",
        25,
        RoundingMode::Down,
        "1000000000000",
    );
    test(
        "33554432000000000000",
        25,
        RoundingMode::Up,
        "1000000000000",
    );
    test(
        "33554432000000000000",
        25,
        RoundingMode::Floor,
        "1000000000000",
    );
    test(
        "33554432000000000000",
        25,
        RoundingMode::Ceiling,
        "1000000000000",
    );
    test(
        "33554432000000000000",
        25,
        RoundingMode::Nearest,
        "1000000000000",
    );
    test(
        "33554432000000000000",
        25,
        RoundingMode::Exact,
        "1000000000000",
    );

    test(
        "2147483648000000000000",
        31,
        RoundingMode::Down,
        "1000000000000",
    );
    test(
        "2147483648000000000000",
        31,
        RoundingMode::Up,
        "1000000000000",
    );
    test(
        "2147483648000000000000",
        31,
        RoundingMode::Floor,
        "1000000000000",
    );
    test(
        "2147483648000000000000",
        31,
        RoundingMode::Ceiling,
        "1000000000000",
    );
    test(
        "2147483648000000000000",
        31,
        RoundingMode::Nearest,
        "1000000000000",
    );
    test(
        "2147483648000000000000",
        31,
        RoundingMode::Exact,
        "1000000000000",
    );

    test(
        "4294967296000000000000",
        32,
        RoundingMode::Down,
        "1000000000000",
    );
    test(
        "4294967296000000000000",
        32,
        RoundingMode::Up,
        "1000000000000",
    );
    test(
        "4294967296000000000000",
        32,
        RoundingMode::Floor,
        "1000000000000",
    );
    test(
        "4294967296000000000000",
        32,
        RoundingMode::Ceiling,
        "1000000000000",
    );
    test(
        "4294967296000000000000",
        32,
        RoundingMode::Nearest,
        "1000000000000",
    );
    test(
        "4294967296000000000000",
        32,
        RoundingMode::Exact,
        "1000000000000",
    );

    test(
        "8589934592000000000000",
        33,
        RoundingMode::Down,
        "1000000000000",
    );
    test(
        "8589934592000000000000",
        33,
        RoundingMode::Up,
        "1000000000000",
    );
    test(
        "8589934592000000000000",
        33,
        RoundingMode::Floor,
        "1000000000000",
    );
    test(
        "8589934592000000000000",
        33,
        RoundingMode::Ceiling,
        "1000000000000",
    );
    test(
        "8589934592000000000000",
        33,
        RoundingMode::Nearest,
        "1000000000000",
    );
    test(
        "8589934592000000000000",
        33,
        RoundingMode::Exact,
        "1000000000000",
    );

    test(
        "1267650600228229401496703205376000000000000",
        100,
        RoundingMode::Down,
        "1000000000000",
    );
    test(
        "1267650600228229401496703205376000000000000",
        100,
        RoundingMode::Up,
        "1000000000000",
    );
    test(
        "1267650600228229401496703205376000000000000",
        100,
        RoundingMode::Floor,
        "1000000000000",
    );
    test(
        "1267650600228229401496703205376000000000000",
        100,
        RoundingMode::Ceiling,
        "1000000000000",
    );
    test(
        "1267650600228229401496703205376000000000000",
        100,
        RoundingMode::Nearest,
        "1000000000000",
    );
    test(
        "1267650600228229401496703205376000000000000",
        100,
        RoundingMode::Exact,
        "1000000000000",
    );

    test("1000000000000", 10, RoundingMode::Down, "976562500");
    test("1000000000000", 10, RoundingMode::Up, "976562500");
    test("1000000000000", 10, RoundingMode::Floor, "976562500");
    test("1000000000000", 10, RoundingMode::Ceiling, "976562500");
    test("1000000000000", 10, RoundingMode::Nearest, "976562500");
    test("1000000000000", 10, RoundingMode::Exact, "976562500");

    test("980657949", 72, RoundingMode::Down, "0");
    test("980657949", 72, RoundingMode::Up, "1");
    test("980657949", 72, RoundingMode::Floor, "0");
    test("980657949", 72, RoundingMode::Ceiling, "1");
    test("980657949", 72, RoundingMode::Nearest, "0");

    test("4294967295", 31, RoundingMode::Down, "1");
    test("4294967295", 31, RoundingMode::Up, "2");
    test("4294967295", 31, RoundingMode::Floor, "1");
    test("4294967295", 31, RoundingMode::Ceiling, "2");
    test("4294967295", 31, RoundingMode::Nearest, "2");

    test("4294967295", 32, RoundingMode::Down, "0");
    test("4294967295", 32, RoundingMode::Up, "1");
    test("4294967295", 32, RoundingMode::Floor, "0");
    test("4294967295", 32, RoundingMode::Ceiling, "1");
    test("4294967295", 32, RoundingMode::Nearest, "1");

    test("4294967296", 32, RoundingMode::Down, "1");
    test("4294967296", 32, RoundingMode::Up, "1");
    test("4294967296", 32, RoundingMode::Floor, "1");
    test("4294967296", 32, RoundingMode::Ceiling, "1");
    test("4294967296", 32, RoundingMode::Nearest, "1");
    test("4294967296", 32, RoundingMode::Exact, "1");

    test("4294967296", 33, RoundingMode::Down, "0");
    test("4294967296", 33, RoundingMode::Up, "1");
    test("4294967296", 33, RoundingMode::Floor, "0");
    test("4294967296", 33, RoundingMode::Ceiling, "1");
    test("4294967296", 33, RoundingMode::Nearest, "0");
}

#[test]
#[should_panic(expected = "Right shift is not exact: 123 >>= 1")]
fn shr_round_assign_u32_fail_1() {
    Natural::from(123u32).shr_round_assign(1, RoundingMode::Exact);
}

#[test]
#[should_panic(expected = "Right shift is not exact: 123 >>= 100")]
fn shr_round_assign_u32_fail_2() {
    Natural::from(123u32).shr_round_assign(100, RoundingMode::Exact);
}

#[test]
#[should_panic(expected = "Right shift is not exact: 1000000000001 >>= 1")]
fn shr_round_assign_u32_fail_3() {
    Natural::from_str("1000000000001")
        .unwrap()
        .shr_round_assign(1, RoundingMode::Exact);
}

#[test]
#[should_panic(expected = "Right shift is not exact: 1000000000001 >>= 100")]
fn shr_round_assign_u32_fail_4() {
    Natural::from_str("1000000000001")
        .unwrap()
        .shr_round_assign(100, RoundingMode::Exact);
}

#[test]
#[should_panic(expected = "Right shift is not exact: 123 >>= 1")]
fn shr_round_u32_fail_1() {
    Natural::from(123u32).shr_round(1, RoundingMode::Exact);
}

#[test]
#[should_panic(expected = "Right shift is not exact: 123 >>= 100")]
fn shr_round_u32_fail_2() {
    Natural::from(123u32).shr_round(100, RoundingMode::Exact);
}

#[test]
#[should_panic(expected = "Right shift is not exact: 1000000000001 >>= 1")]
fn shr_round_u32_fail_3() {
    Natural::from_str("1000000000001")
        .unwrap()
        .shr_round(1, RoundingMode::Exact);
}

#[test]
#[should_panic(expected = "Right shift is not exact: 1000000000001 >>= 100")]
fn shr_round_u32_fail_4() {
    Natural::from_str("1000000000001")
        .unwrap()
        .shr_round(100, RoundingMode::Exact);
}

#[test]
#[should_panic(expected = "Right shift is not exact: 123 >> 1")]
fn shr_round_u32_ref_fail_1() {
    (&Natural::from(123u32)).shr_round(1, RoundingMode::Exact);
}

#[test]
#[should_panic(expected = "Right shift is not exact: 123 >> 100")]
fn shr_round_u32_ref_fail_2() {
    (&Natural::from(123u32)).shr_round(100, RoundingMode::Exact);
}

#[test]
#[should_panic(expected = "Right shift is not exact: 1000000000001 >> 1")]
fn shr_round_u32_ref_fail_3() {
    (&Natural::from_str("1000000000001").unwrap()).shr_round(1, RoundingMode::Exact);
}

#[test]
#[should_panic(expected = "Right shift is not exact: 1000000000001 >> 100")]
fn shr_round_u32_ref_fail_4() {
    (&Natural::from_str("1000000000001").unwrap()).shr_round(100, RoundingMode::Exact);
}

#[test]
fn shr_round_u32_properties() {
    // n.shr_round_assign(u, rm) is equivalent for malachite and rugint.
    // n.shr_round(u, rm) is equivalent for malachite and rugint.
    // (&n).shr_round(u, rm) is equivalent for malachite and num.
    // n.shr_round_assign(u, rm); n is valid.
    // n.shr_round(u, rm) is valid.
    // (&n).shr_round(u, rm) is valid.
    // n.shr_round_assign(u, rm), n.shr_round(u, rm), and (&n).shr_round(u, rm) give the same
    //      result.
    // n.shr_round(u, rm) <= n
    // TODO n.shr_round(u, rm) == n.div_round(1 << u)
    let natural_u32_and_rounding_mode = |mut n: Natural, u: u32, rm: RoundingMode| {
        let old_n = n.clone();
        n.shr_round_assign(u, rm);
        assert!(n.is_valid());

        let n2 = old_n.clone();
        let result = (&n2).shr_round(u, rm);
        assert_eq!(result, n);
        assert!(result.is_valid());
        let result = n2.shr_round(u, rm);
        assert!(result.is_valid());
        assert_eq!(result, n);

        assert!((&old_n).shr_round(u, rm) <= old_n);
    };

    // If n is divisible by 2^u, n.shr_round(u, rm) are equal for all rm.
    let natural_and_u32 = |n: Natural, u: u32| {
        let x = &n << u;
        assert_eq!((&x).shr_round(u, RoundingMode::Down), n);
        assert_eq!((&x).shr_round(u, RoundingMode::Up), n);
        assert_eq!((&x).shr_round(u, RoundingMode::Floor), n);
        assert_eq!((&x).shr_round(u, RoundingMode::Ceiling), n);
        assert_eq!((&x).shr_round(u, RoundingMode::Nearest), n);
        assert_eq!((&x).shr_round(u, RoundingMode::Exact), n);
    };

    // Rounding using Down or Floor is equivalent, as is using Up or Ceiling. When the shift is
    // inexact, rounding using Up yields a value 1 larger than using Down. Using Nearest gives a
    // value that is equal to either that produced by rounding Down or that produced by rounding Up.
    // TODO test using Rationals
    let natural_and_u32_inexact = |n: Natural, u: u32| {
        let down = (&n).shr_round(u, RoundingMode::Down);
        let up = &down + 1;
        assert_eq!((&n).shr_round(u, RoundingMode::Up), up);
        assert_eq!((&n).shr_round(u, RoundingMode::Floor), down);
        assert_eq!((&n).shr_round(u, RoundingMode::Ceiling), up);
        let nearest = (&n).shr_round(u, RoundingMode::Nearest);
        assert!(nearest == down || nearest == up);
    };

    // if u > 0 and v >= 32, u.shr_round(v, Down) == 0
    // if u > 0 and v >= 32, u.shr_round(v, Floor) == 0
    // if u > 0 and v >= 32, u.shr_round(v, Up) == 1
    // if u > 0 and v >= 32, u.shr_round(v, Ceiling) == 1
    // if u > 0 and v >= 33, u.shr_round(v, Nearest) == 0
    let positive_u32_and_u32 = |u: u32, v: u32| {
        assert_eq!(Natural::from(u).shr_round(v + 32, RoundingMode::Down), 0);
        assert_eq!(Natural::from(u).shr_round(v + 32, RoundingMode::Floor), 0);
        assert_eq!(Natural::from(u).shr_round(v + 32, RoundingMode::Up), 1);
        assert_eq!(Natural::from(u).shr_round(v + 32, RoundingMode::Ceiling), 1);
        assert_eq!(Natural::from(u).shr_round(v + 33, RoundingMode::Nearest), 0);
    };

    // n.shr_round(0, rm) == n
    #[allow(unknown_lints, identity_op)]
    let natural_and_rounding_mode = |n: Natural, rm: RoundingMode| {
        assert_eq!((&n).shr_round(0, rm), n);
    };

    // 0.shr_round(u, rm) == 0
    let u32_and_rounding_mode = |u: u32, rm: RoundingMode| {
        assert_eq!(Natural::ZERO.shr_round(u, rm), 0);
    };

    for (n, u) in pairs_of_natural_and_small_u32(GenerationMode::Exhaustive).take(LARGE_LIMIT) {
        natural_and_u32(n, u);
    }

    for (n, u) in pairs_of_natural_and_small_u32(GenerationMode::Random(32)).take(LARGE_LIMIT) {
        natural_and_u32(n, u);
    }

    for (n, u) in pairs_of_natural_and_small_u32_var_2(GenerationMode::Exhaustive).take(LARGE_LIMIT)
    {
        natural_and_u32_inexact(n, u);
    }

    for (n, u) in pairs_of_natural_and_small_u32_var_2(GenerationMode::Random(32)).take(LARGE_LIMIT)
    {
        natural_and_u32_inexact(n, u);
    }

    for (n, u, rm) in triples_of_natural_small_u32_and_rounding_mode_var_1(
        GenerationMode::Exhaustive,
    ).take(LARGE_LIMIT)
    {
        natural_u32_and_rounding_mode(n, u, rm);
    }

    for (n, u, rm) in triples_of_natural_small_u32_and_rounding_mode_var_1(GenerationMode::Random(
        32,
    )).take(LARGE_LIMIT)
    {
        natural_u32_and_rounding_mode(n, u, rm);
    }

    for (n, u) in
        pairs_of_positive_unsigned_and_small_u32(GenerationMode::Exhaustive).take(LARGE_LIMIT)
    {
        positive_u32_and_u32(n, u);
    }

    for (n, u) in
        pairs_of_positive_unsigned_and_small_u32(GenerationMode::Random(32)).take(LARGE_LIMIT)
    {
        positive_u32_and_u32(n, u);
    }

    for (n, rm) in pairs_of_natural_and_rounding_mode(GenerationMode::Exhaustive).take(LARGE_LIMIT)
    {
        natural_and_rounding_mode(n, rm);
    }

    for (n, rm) in pairs_of_natural_and_rounding_mode(GenerationMode::Random(32)).take(LARGE_LIMIT)
    {
        natural_and_rounding_mode(n, rm);
    }

    for (u, rm) in pairs_of_unsigned_and_rounding_mode(GenerationMode::Exhaustive).take(LARGE_LIMIT)
    {
        u32_and_rounding_mode(u, rm);
    }

    for (u, rm) in pairs_of_unsigned_and_rounding_mode(GenerationMode::Random(32)).take(LARGE_LIMIT)
    {
        u32_and_rounding_mode(u, rm);
    }
}
