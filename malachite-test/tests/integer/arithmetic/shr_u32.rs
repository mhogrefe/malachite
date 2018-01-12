use common::LARGE_LIMIT;
use malachite_base::round::RoundingMode;
use malachite_base::traits::{PartialOrdAbs, ShrRound, ShrRoundAssign, Zero};
use malachite_nz::integer::Integer;
use malachite_test::common::{integer_to_rugint_integer, rugint_integer_to_integer, GenerationMode};
use rugint;
use malachite_test::integer::arithmetic::shr_u32::{select_inputs_1, select_inputs_2};
use rust_wheels::iterators::common::EXAMPLE_SEED;
use rust_wheels::iterators::general::random_x;
use rust_wheels::iterators::integers_geometric::natural_u32s_geometric;
use rust_wheels::iterators::integers::{exhaustive_integers, random_integers};
use rust_wheels::iterators::primitive_ints::{exhaustive_negative_i, exhaustive_positive_x,
                                             exhaustive_u, random_negative_i, random_positive_i};
use rust_wheels::iterators::rounding_modes::{exhaustive_rounding_modes, random_rounding_modes};
use rust_wheels::iterators::tuples::{exhaustive_pairs_from_single, log_pairs,
                                     log_pairs_from_single, random_pairs, random_triples};
use std::str::FromStr;

#[test]
fn test_shr_u32() {
    let test = |u, v: u32, out| {
        let mut n = Integer::from_str(u).unwrap();
        n >>= v;
        assert_eq!(n.to_string(), out);
        assert!(n.is_valid());

        let mut n = rugint::Integer::from_str(u).unwrap();
        n >>= v;
        assert_eq!(n.to_string(), out);

        let n = Integer::from_str(u).unwrap() >> v;
        assert_eq!(n.to_string(), out);
        assert!(n.is_valid());

        let n = rugint::Integer::from_str(u).unwrap() >> v;
        assert_eq!(n.to_string(), out);

        let n = &Integer::from_str(u).unwrap() >> v;
        assert_eq!(n.to_string(), out);
        assert!(n.is_valid());
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

    test("-123", 0, "-123");
    test("-245", 1, "-123");
    test("-246", 1, "-123");
    test("-247", 1, "-124");
    test("-491", 2, "-123");
    test("-492", 2, "-123");
    test("-493", 2, "-124");
    test("-4127195135", 25, "-123");
    test("-4127195136", 25, "-123");
    test("-4127195137", 25, "-124");
    test("-8254390271", 26, "-123");
    test("-8254390272", 26, "-123");
    test("-8254390273", 26, "-124");
    test("-155921023828072216384094494261247", 100, "-123");
    test("-155921023828072216384094494261248", 100, "-123");
    test("-155921023828072216384094494261249", 100, "-124");
    test("-4294967295", 1, "-2147483648");
    test("-4294967296", 1, "-2147483648");
    test("-4294967297", 1, "-2147483649");
    test("-1000000000000", 0, "-1000000000000");
    test("-7999999999999", 3, "-1000000000000");
    test("-8000000000000", 3, "-1000000000000");
    test("-8000000000001", 3, "-1000000000001");
    test("-16777216000000000000", 24, "-1000000000000");
    test("-33554432000000000000", 25, "-1000000000000");
    test("-2147483648000000000000", 31, "-1000000000000");
    test("-4294967296000000000000", 32, "-1000000000000");
    test("-8589934592000000000000", 33, "-1000000000000");
    test(
        "-1267650600228229401496703205376000000000000",
        100,
        "-1000000000000",
    );
    test("-1000000000000", 10, "-976562500");
    test("-980657949", 72, "-1");
    test("-4294967295", 31, "-2");
    test("-4294967295", 32, "-1");
    test("-4294967296", 32, "-1");
    test("-4294967296", 33, "-1");
}

#[test]
fn shr_u32_properties() {
    // n >>= u is equivalent for malachite and rugint.
    // n >> u is equivalent for malachite and rugint.
    // ******* bug in num for n >> i when n < 0, n not divisible by 2^i
    // n >>= u; n is valid.
    // n >> u is valid.
    // &n >> u is valid.
    // n >>= u, n >> u, and &n >> u give the same result.
    // |n >> u| <= |n|
    // TODO n >> u == n / (1 << u)
    // n >> u == n.shr_round(u, Floor)
    // if u < 2^31, n >> u == n >> (u as i32) == n << -(u as i32)
    let integer_and_u32 = |mut n: Integer, u: u32| {
        let old_n = n.clone();
        n >>= u;
        assert!(n.is_valid());

        let mut rugint_n = integer_to_rugint_integer(&old_n);
        rugint_n >>= u;
        assert_eq!(rugint_integer_to_integer(&rugint_n), n);

        let n2 = old_n.clone();
        let result = &n2 >> u;
        assert_eq!(result, n);
        assert!(result.is_valid());
        let result = n2 >> u;
        assert!(result.is_valid());
        assert_eq!(result, n);

        let rugint_n2 = integer_to_rugint_integer(&old_n);
        assert_eq!(rugint_integer_to_integer(&(rugint_n2 >> u)), n);

        assert!((&old_n >> u).le_abs(&old_n));
        assert_eq!(&old_n >> u, (&old_n).shr_round(u, RoundingMode::Floor));

        if u <= (i32::max_value() as u32) {
            assert_eq!(&old_n >> (u as i32), n);
            assert_eq!(&old_n << -(u as i32), n);
        }
    };

    // if u >= 32, n >> u == 0
    let two_u32s = |u: u32, v: u32| {
        assert_eq!(Integer::from(u) >> (v + 32), 0);
    };

    // n >> u >> v == n >> (u + v)
    let integer_and_two_u32s = |n: Integer, u: u32, v: u32| {
        assert_eq!(&n >> u >> v, &n >> (u + v));
    };

    // n >> 0 == n
    #[allow(unknown_lints, identity_op)]
    let one_integer = |n: Integer| {
        assert_eq!(&n >> 0u32, n);
    };

    // 0 >> n == 0
    let one_u32 = |u: u32| {
        assert_eq!(Integer::ZERO >> u, 0);
    };

    for (n, u) in select_inputs_1(GenerationMode::Exhaustive).take(LARGE_LIMIT) {
        integer_and_u32(n, u);
    }

    for (n, u) in select_inputs_1(GenerationMode::Random(32)).take(LARGE_LIMIT) {
        integer_and_u32(n, u);
    }

    for (n, (u, v)) in log_pairs(
        exhaustive_integers(),
        exhaustive_pairs_from_single(exhaustive_u::<u32>()),
    ).take(LARGE_LIMIT)
    {
        integer_and_two_u32s(n, u, v);
    }

    for (n, u, v) in random_triples(
        &EXAMPLE_SEED,
        &(|seed| random_integers(seed, 32)),
        &(|seed| natural_u32s_geometric(seed, 32)),
        &(|seed| natural_u32s_geometric(seed, 32)),
    ).take(LARGE_LIMIT)
    {
        integer_and_two_u32s(n, u, v);
    }

    for (n, u) in log_pairs_from_single(exhaustive_u::<u32>()).take(LARGE_LIMIT) {
        two_u32s(n, u);
    }

    for (n, u) in random_pairs(
        &EXAMPLE_SEED,
        &(|seed| random_x(seed)),
        &(|seed| natural_u32s_geometric(seed, 32)),
    ).take(LARGE_LIMIT)
    {
        two_u32s(n, u);
    }

    for n in exhaustive_integers().take(LARGE_LIMIT) {
        one_integer(n);
    }

    for n in random_integers(&EXAMPLE_SEED, 32).take(LARGE_LIMIT) {
        one_integer(n);
    }

    for n in exhaustive_u().take(LARGE_LIMIT) {
        one_u32(n);
    }

    for n in natural_u32s_geometric(&EXAMPLE_SEED, 32).take(LARGE_LIMIT) {
        one_u32(n);
    }
}

#[test]
fn test_shr_round_u32() {
    let test = |u, v: u32, rm: RoundingMode, out| {
        let mut n = Integer::from_str(u).unwrap();
        n.shr_round_assign(v, rm);
        assert_eq!(n.to_string(), out);
        assert!(n.is_valid());

        let n = Integer::from_str(u).unwrap().shr_round(v, rm);
        assert_eq!(n.to_string(), out);
        assert!(n.is_valid());

        let n = &Integer::from_str(u).unwrap().shr_round(v, rm);
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

    test("-123", 0, RoundingMode::Down, "-123");
    test("-123", 0, RoundingMode::Up, "-123");
    test("-123", 0, RoundingMode::Floor, "-123");
    test("-123", 0, RoundingMode::Ceiling, "-123");
    test("-123", 0, RoundingMode::Nearest, "-123");
    test("-123", 0, RoundingMode::Exact, "-123");

    test("-245", 1, RoundingMode::Down, "-122");
    test("-245", 1, RoundingMode::Up, "-123");
    test("-245", 1, RoundingMode::Floor, "-123");
    test("-245", 1, RoundingMode::Ceiling, "-122");
    test("-245", 1, RoundingMode::Nearest, "-122");

    test("-246", 1, RoundingMode::Down, "-123");
    test("-246", 1, RoundingMode::Up, "-123");
    test("-246", 1, RoundingMode::Floor, "-123");
    test("-246", 1, RoundingMode::Ceiling, "-123");
    test("-246", 1, RoundingMode::Nearest, "-123");
    test("-246", 1, RoundingMode::Exact, "-123");

    test("-247", 1, RoundingMode::Down, "-123");
    test("-247", 1, RoundingMode::Up, "-124");
    test("-247", 1, RoundingMode::Floor, "-124");
    test("-247", 1, RoundingMode::Ceiling, "-123");
    test("-247", 1, RoundingMode::Nearest, "-124");

    test("-491", 2, RoundingMode::Down, "-122");
    test("-491", 2, RoundingMode::Up, "-123");
    test("-491", 2, RoundingMode::Floor, "-123");
    test("-491", 2, RoundingMode::Ceiling, "-122");
    test("-491", 2, RoundingMode::Nearest, "-123");

    test("-492", 2, RoundingMode::Down, "-123");
    test("-492", 2, RoundingMode::Up, "-123");
    test("-492", 2, RoundingMode::Floor, "-123");
    test("-492", 2, RoundingMode::Ceiling, "-123");
    test("-492", 2, RoundingMode::Nearest, "-123");
    test("-492", 2, RoundingMode::Exact, "-123");

    test("-493", 2, RoundingMode::Down, "-123");
    test("-493", 2, RoundingMode::Up, "-124");
    test("-493", 2, RoundingMode::Floor, "-124");
    test("-493", 2, RoundingMode::Ceiling, "-123");
    test("-493", 2, RoundingMode::Nearest, "-123");

    test("-4127195135", 25, RoundingMode::Down, "-122");
    test("-4127195135", 25, RoundingMode::Up, "-123");
    test("-4127195135", 25, RoundingMode::Floor, "-123");
    test("-4127195135", 25, RoundingMode::Ceiling, "-122");
    test("-4127195135", 25, RoundingMode::Nearest, "-123");

    test("-4127195136", 25, RoundingMode::Down, "-123");
    test("-4127195136", 25, RoundingMode::Up, "-123");
    test("-4127195136", 25, RoundingMode::Floor, "-123");
    test("-4127195136", 25, RoundingMode::Ceiling, "-123");
    test("-4127195136", 25, RoundingMode::Nearest, "-123");
    test("-4127195136", 25, RoundingMode::Exact, "-123");

    test("-4127195137", 25, RoundingMode::Down, "-123");
    test("-4127195137", 25, RoundingMode::Up, "-124");
    test("-4127195137", 25, RoundingMode::Floor, "-124");
    test("-4127195137", 25, RoundingMode::Ceiling, "-123");
    test("-4127195137", 25, RoundingMode::Nearest, "-123");

    test("-8254390271", 26, RoundingMode::Down, "-122");
    test("-8254390271", 26, RoundingMode::Up, "-123");
    test("-8254390271", 26, RoundingMode::Floor, "-123");
    test("-8254390271", 26, RoundingMode::Ceiling, "-122");
    test("-8254390271", 26, RoundingMode::Nearest, "-123");

    test("-8254390272", 26, RoundingMode::Down, "-123");
    test("-8254390272", 26, RoundingMode::Up, "-123");
    test("-8254390272", 26, RoundingMode::Floor, "-123");
    test("-8254390272", 26, RoundingMode::Ceiling, "-123");
    test("-8254390272", 26, RoundingMode::Nearest, "-123");
    test("-8254390272", 26, RoundingMode::Exact, "-123");

    test("-8254390273", 26, RoundingMode::Down, "-123");
    test("-8254390273", 26, RoundingMode::Up, "-124");
    test("-8254390273", 26, RoundingMode::Floor, "-124");
    test("-8254390273", 26, RoundingMode::Ceiling, "-123");
    test("-8254390273", 26, RoundingMode::Nearest, "-123");

    test(
        "-155921023828072216384094494261247",
        100,
        RoundingMode::Down,
        "-122",
    );
    test(
        "-155921023828072216384094494261247",
        100,
        RoundingMode::Up,
        "-123",
    );
    test(
        "-155921023828072216384094494261247",
        100,
        RoundingMode::Floor,
        "-123",
    );
    test(
        "-155921023828072216384094494261247",
        100,
        RoundingMode::Ceiling,
        "-122",
    );
    test(
        "-155921023828072216384094494261247",
        100,
        RoundingMode::Nearest,
        "-123",
    );

    test(
        "-155921023828072216384094494261248",
        100,
        RoundingMode::Down,
        "-123",
    );
    test(
        "-155921023828072216384094494261248",
        100,
        RoundingMode::Up,
        "-123",
    );
    test(
        "-155921023828072216384094494261248",
        100,
        RoundingMode::Floor,
        "-123",
    );
    test(
        "-155921023828072216384094494261248",
        100,
        RoundingMode::Ceiling,
        "-123",
    );
    test(
        "-155921023828072216384094494261248",
        100,
        RoundingMode::Nearest,
        "-123",
    );
    test(
        "-155921023828072216384094494261248",
        100,
        RoundingMode::Exact,
        "-123",
    );

    test(
        "-155921023828072216384094494261249",
        100,
        RoundingMode::Down,
        "-123",
    );
    test(
        "-155921023828072216384094494261249",
        100,
        RoundingMode::Up,
        "-124",
    );
    test(
        "-155921023828072216384094494261249",
        100,
        RoundingMode::Floor,
        "-124",
    );
    test(
        "-155921023828072216384094494261249",
        100,
        RoundingMode::Ceiling,
        "-123",
    );
    test(
        "-155921023828072216384094494261249",
        100,
        RoundingMode::Nearest,
        "-123",
    );

    test("-4294967295", 1, RoundingMode::Down, "-2147483647");
    test("-4294967295", 1, RoundingMode::Up, "-2147483648");
    test("-4294967295", 1, RoundingMode::Floor, "-2147483648");
    test("-4294967295", 1, RoundingMode::Ceiling, "-2147483647");
    test("-4294967295", 1, RoundingMode::Nearest, "-2147483648");

    test("-4294967296", 1, RoundingMode::Down, "-2147483648");
    test("-4294967296", 1, RoundingMode::Up, "-2147483648");
    test("-4294967296", 1, RoundingMode::Floor, "-2147483648");
    test("-4294967296", 1, RoundingMode::Ceiling, "-2147483648");
    test("-4294967296", 1, RoundingMode::Nearest, "-2147483648");
    test("-4294967296", 1, RoundingMode::Exact, "-2147483648");

    test("-4294967297", 1, RoundingMode::Down, "-2147483648");
    test("-4294967297", 1, RoundingMode::Up, "-2147483649");
    test("-4294967297", 1, RoundingMode::Floor, "-2147483649");
    test("-4294967297", 1, RoundingMode::Ceiling, "-2147483648");
    test("-4294967297", 1, RoundingMode::Nearest, "-2147483648");

    test("-1000000000000", 0, RoundingMode::Down, "-1000000000000");
    test("-1000000000000", 0, RoundingMode::Up, "-1000000000000");
    test("-1000000000000", 0, RoundingMode::Floor, "-1000000000000");
    test("-1000000000000", 0, RoundingMode::Ceiling, "-1000000000000");
    test("-1000000000000", 0, RoundingMode::Nearest, "-1000000000000");
    test("-1000000000000", 0, RoundingMode::Exact, "-1000000000000");

    test("-7999999999999", 3, RoundingMode::Down, "-999999999999");
    test("-7999999999999", 3, RoundingMode::Up, "-1000000000000");
    test("-7999999999999", 3, RoundingMode::Floor, "-1000000000000");
    test("-7999999999999", 3, RoundingMode::Ceiling, "-999999999999");
    test("-7999999999999", 3, RoundingMode::Nearest, "-1000000000000");

    test("-8000000000000", 3, RoundingMode::Down, "-1000000000000");
    test("-8000000000000", 3, RoundingMode::Up, "-1000000000000");
    test("-8000000000000", 3, RoundingMode::Floor, "-1000000000000");
    test("-8000000000000", 3, RoundingMode::Ceiling, "-1000000000000");
    test("-8000000000000", 3, RoundingMode::Nearest, "-1000000000000");
    test("-8000000000000", 3, RoundingMode::Exact, "-1000000000000");

    test("-8000000000001", 3, RoundingMode::Down, "-1000000000000");
    test("-8000000000001", 3, RoundingMode::Up, "-1000000000001");
    test("-8000000000001", 3, RoundingMode::Floor, "-1000000000001");
    test("-8000000000001", 3, RoundingMode::Ceiling, "-1000000000000");
    test("-8000000000001", 3, RoundingMode::Nearest, "-1000000000000");

    test(
        "-16777216000000000000",
        24,
        RoundingMode::Down,
        "-1000000000000",
    );
    test(
        "-16777216000000000000",
        24,
        RoundingMode::Up,
        "-1000000000000",
    );
    test(
        "-16777216000000000000",
        24,
        RoundingMode::Floor,
        "-1000000000000",
    );
    test(
        "-16777216000000000000",
        24,
        RoundingMode::Ceiling,
        "-1000000000000",
    );
    test(
        "-16777216000000000000",
        24,
        RoundingMode::Nearest,
        "-1000000000000",
    );
    test(
        "-16777216000000000000",
        24,
        RoundingMode::Exact,
        "-1000000000000",
    );

    test(
        "-33554432000000000000",
        25,
        RoundingMode::Down,
        "-1000000000000",
    );
    test(
        "-33554432000000000000",
        25,
        RoundingMode::Up,
        "-1000000000000",
    );
    test(
        "-33554432000000000000",
        25,
        RoundingMode::Floor,
        "-1000000000000",
    );
    test(
        "-33554432000000000000",
        25,
        RoundingMode::Ceiling,
        "-1000000000000",
    );
    test(
        "-33554432000000000000",
        25,
        RoundingMode::Nearest,
        "-1000000000000",
    );
    test(
        "-33554432000000000000",
        25,
        RoundingMode::Exact,
        "-1000000000000",
    );

    test(
        "-2147483648000000000000",
        31,
        RoundingMode::Down,
        "-1000000000000",
    );
    test(
        "-2147483648000000000000",
        31,
        RoundingMode::Up,
        "-1000000000000",
    );
    test(
        "-2147483648000000000000",
        31,
        RoundingMode::Floor,
        "-1000000000000",
    );
    test(
        "-2147483648000000000000",
        31,
        RoundingMode::Ceiling,
        "-1000000000000",
    );
    test(
        "-2147483648000000000000",
        31,
        RoundingMode::Nearest,
        "-1000000000000",
    );
    test(
        "-2147483648000000000000",
        31,
        RoundingMode::Exact,
        "-1000000000000",
    );

    test(
        "-4294967296000000000000",
        32,
        RoundingMode::Down,
        "-1000000000000",
    );
    test(
        "-4294967296000000000000",
        32,
        RoundingMode::Up,
        "-1000000000000",
    );
    test(
        "-4294967296000000000000",
        32,
        RoundingMode::Floor,
        "-1000000000000",
    );
    test(
        "-4294967296000000000000",
        32,
        RoundingMode::Ceiling,
        "-1000000000000",
    );
    test(
        "-4294967296000000000000",
        32,
        RoundingMode::Nearest,
        "-1000000000000",
    );
    test(
        "-4294967296000000000000",
        32,
        RoundingMode::Exact,
        "-1000000000000",
    );

    test(
        "-8589934592000000000000",
        33,
        RoundingMode::Down,
        "-1000000000000",
    );
    test(
        "-8589934592000000000000",
        33,
        RoundingMode::Up,
        "-1000000000000",
    );
    test(
        "-8589934592000000000000",
        33,
        RoundingMode::Floor,
        "-1000000000000",
    );
    test(
        "-8589934592000000000000",
        33,
        RoundingMode::Ceiling,
        "-1000000000000",
    );
    test(
        "-8589934592000000000000",
        33,
        RoundingMode::Nearest,
        "-1000000000000",
    );
    test(
        "-8589934592000000000000",
        33,
        RoundingMode::Exact,
        "-1000000000000",
    );

    test(
        "-1267650600228229401496703205376000000000000",
        100,
        RoundingMode::Down,
        "-1000000000000",
    );
    test(
        "-1267650600228229401496703205376000000000000",
        100,
        RoundingMode::Up,
        "-1000000000000",
    );
    test(
        "-1267650600228229401496703205376000000000000",
        100,
        RoundingMode::Floor,
        "-1000000000000",
    );
    test(
        "-1267650600228229401496703205376000000000000",
        100,
        RoundingMode::Ceiling,
        "-1000000000000",
    );
    test(
        "-1267650600228229401496703205376000000000000",
        100,
        RoundingMode::Nearest,
        "-1000000000000",
    );
    test(
        "-1267650600228229401496703205376000000000000",
        100,
        RoundingMode::Exact,
        "-1000000000000",
    );

    test("-1000000000000", 10, RoundingMode::Down, "-976562500");
    test("-1000000000000", 10, RoundingMode::Up, "-976562500");
    test("-1000000000000", 10, RoundingMode::Floor, "-976562500");
    test("-1000000000000", 10, RoundingMode::Ceiling, "-976562500");
    test("-1000000000000", 10, RoundingMode::Nearest, "-976562500");
    test("-1000000000000", 10, RoundingMode::Exact, "-976562500");

    test("-980657949", 72, RoundingMode::Down, "0");
    test("-980657949", 72, RoundingMode::Up, "-1");
    test("-980657949", 72, RoundingMode::Floor, "-1");
    test("-980657949", 72, RoundingMode::Ceiling, "0");
    test("-980657949", 72, RoundingMode::Nearest, "0");

    test("-4294967295", 31, RoundingMode::Down, "-1");
    test("-4294967295", 31, RoundingMode::Up, "-2");
    test("-4294967295", 31, RoundingMode::Floor, "-2");
    test("-4294967295", 31, RoundingMode::Ceiling, "-1");
    test("-4294967295", 31, RoundingMode::Nearest, "-2");

    test("-4294967295", 32, RoundingMode::Down, "0");
    test("-4294967295", 32, RoundingMode::Up, "-1");
    test("-4294967295", 32, RoundingMode::Floor, "-1");
    test("-4294967295", 32, RoundingMode::Ceiling, "0");
    test("-4294967295", 32, RoundingMode::Nearest, "-1");

    test("-4294967296", 32, RoundingMode::Down, "-1");
    test("-4294967296", 32, RoundingMode::Up, "-1");
    test("-4294967296", 32, RoundingMode::Floor, "-1");
    test("-4294967296", 32, RoundingMode::Ceiling, "-1");
    test("-4294967296", 32, RoundingMode::Nearest, "-1");
    test("-4294967296", 32, RoundingMode::Exact, "-1");

    test("-4294967296", 33, RoundingMode::Down, "0");
    test("-4294967296", 33, RoundingMode::Up, "-1");
    test("-4294967296", 33, RoundingMode::Floor, "-1");
    test("-4294967296", 33, RoundingMode::Ceiling, "0");
    test("-4294967296", 33, RoundingMode::Nearest, "0");
}

#[test]
#[should_panic(expected = "Right shift is not exact: 123 >>= 1")]
fn shr_round_assign_u32_fail_1() {
    Integer::from(123u32).shr_round_assign(1u32, RoundingMode::Exact);
}

#[test]
#[should_panic(expected = "Right shift is not exact: 123 >>= 100")]
fn shr_round_assign_u32_fail_2() {
    Integer::from(123u32).shr_round_assign(100u32, RoundingMode::Exact);
}

#[test]
#[should_panic(expected = "Right shift is not exact: 1000000000001 >>= 1")]
fn shr_round_assign_u32_fail_3() {
    Integer::from_str("1000000000001")
        .unwrap()
        .shr_round_assign(1u32, RoundingMode::Exact);
}

#[test]
#[should_panic(expected = "Right shift is not exact: 1000000000001 >>= 100")]
fn shr_round_assign_u32_fail_4() {
    Integer::from_str("1000000000001")
        .unwrap()
        .shr_round_assign(100u32, RoundingMode::Exact);
}

#[test]
#[should_panic(expected = "Right shift is not exact: 123 >>= 1")]
fn shr_round_u32_fail_1() {
    Integer::from(123u32).shr_round(1u32, RoundingMode::Exact);
}

#[test]
#[should_panic(expected = "Right shift is not exact: 123 >>= 100")]
fn shr_round_u32_fail_2() {
    Integer::from(123u32).shr_round(100u32, RoundingMode::Exact);
}

#[test]
#[should_panic(expected = "Right shift is not exact: 1000000000001 >>= 1")]
fn shr_round_u32_fail_3() {
    Integer::from_str("1000000000001")
        .unwrap()
        .shr_round(1u32, RoundingMode::Exact);
}

#[test]
#[should_panic(expected = "Right shift is not exact: 1000000000001 >>= 100")]
fn shr_round_u32_fail_4() {
    Integer::from_str("1000000000001")
        .unwrap()
        .shr_round(100u32, RoundingMode::Exact);
}

#[test]
#[should_panic(expected = "Right shift is not exact: 123 >> 1")]
fn shr_round_ref_u32_fail_1() {
    (&Integer::from(123u32)).shr_round(1u32, RoundingMode::Exact);
}

#[test]
#[should_panic(expected = "Right shift is not exact: 123 >> 100")]
fn shr_round_ref_u32_fail_2() {
    (&Integer::from(123u32)).shr_round(100u32, RoundingMode::Exact);
}

#[test]
#[should_panic(expected = "Right shift is not exact: 1000000000001 >> 1")]
fn shr_round_ref_u32_fail_3() {
    (&Integer::from_str("1000000000001").unwrap()).shr_round(1u32, RoundingMode::Exact);
}

#[test]
#[should_panic(expected = "Right shift is not exact: 1000000000001 >> 100")]
fn shr_round_ref_u32_fail_4() {
    (&Integer::from_str("1000000000001").unwrap()).shr_round(100u32, RoundingMode::Exact);
}

#[test]
fn shr_round_u32_properties() {
    // n.shr_round_assign(u, rm); n is valid.
    // n.shr_round(u, rm) is valid.
    // (&n).shr_round(u, rm) is valid.
    // n.shr_round_assign(u, rm), n.shr_round(u, rm), and (&n).shr_round(u, rm) give the same
    //      result.
    // |n.shr_round(u, rm)| <= |n|
    // TODO n.shr_round(u, rm) == n.div_round(1 << u)
    // -(-n).shr_round(u, -rm) == n.shr_round(u, rm)
    let integer_u32_and_rounding_mode = |mut n: Integer, u: u32, rm: RoundingMode| {
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

        assert!((&old_n).shr_round(u, rm).le_abs(&old_n));
        assert_eq!(-(-&old_n).shr_round(u, -rm), n);
    };

    // If n is divisible by 2^u, n.shr_round(u, rm) are equal for all rm.
    let integer_and_u32 = |n: Integer, u: u32| {
        let x = &n << u;
        assert_eq!((&x).shr_round(u, RoundingMode::Down), n);
        assert_eq!((&x).shr_round(u, RoundingMode::Up), n);
        assert_eq!((&x).shr_round(u, RoundingMode::Floor), n);
        assert_eq!((&x).shr_round(u, RoundingMode::Ceiling), n);
        assert_eq!((&x).shr_round(u, RoundingMode::Nearest), n);
        assert_eq!((&x).shr_round(u, RoundingMode::Exact), n);
    };

    // Rounding a non-negative number using Down or Floor is equivalent, as is using Up or Ceiling.
    // Rounding a non-positive number using Down or Ceiling is equivalent, as is using Up or Floor.
    // When the shift is inexact, rounding using Ceiling yields a value 1 larger than using Floor.
    // Using Nearest gives a value that is equal to either that produced by rounding Floor or that
    // produced by rounding Ceiling.
    // TODO test using Rationals
    let integer_and_u32_inexact = |n: Integer, u: u32| {
        let floor = (&n).shr_round(u, RoundingMode::Floor);
        let ceiling = &floor + 1;
        assert_eq!((&n).shr_round(u, RoundingMode::Ceiling), ceiling);
        if n > 0 {
            assert_eq!((&n).shr_round(u, RoundingMode::Up), ceiling);
            assert_eq!((&n).shr_round(u, RoundingMode::Down), floor);
        } else {
            assert_eq!((&n).shr_round(u, RoundingMode::Up), floor);
            assert_eq!((&n).shr_round(u, RoundingMode::Down), ceiling);
        }
        let nearest = (&n).shr_round(u, RoundingMode::Nearest);
        assert!(nearest == floor || nearest == ceiling);
    };

    // if i > 0 and j >= 31, i.shr_round(j, Down) == 0
    // if i > 0 and j >= 31, i.shr_round(j, Floor) == 0
    // if i > 0 and j >= 31, i.shr_round(j, Up) == 1
    // if i > 0 and j >= 31, i.shr_round(j, Ceiling) == 1
    // if i > 0 and j >= 32, i.shr_round(j, Nearest) == 0
    let positive_i32_and_u32 = |i: i32, j: u32| {
        assert_eq!(Integer::from(i).shr_round(j + 31, RoundingMode::Down), 0);
        assert_eq!(Integer::from(i).shr_round(j + 31, RoundingMode::Floor), 0);
        assert_eq!(Integer::from(i).shr_round(j + 31, RoundingMode::Up), 1);
        assert_eq!(Integer::from(i).shr_round(j + 31, RoundingMode::Ceiling), 1);
        assert_eq!(Integer::from(i).shr_round(j + 32, RoundingMode::Nearest), 0);
    };

    // if -2^31 < i < 0 and j >= 31, i.shr_round(j, Down) == 0
    // if -2^31 < i < 0 and j >= 31, i.shr_round(j, Floor) == -1
    // if -2^31 < i < 0 and j >= 31, i.shr_round(j, Up) == -1
    // if -2^31 < i < 0 and j >= 31, i.shr_round(j, Ceiling) == 0
    // if -2^31 < i < 0 and j >= 32, i.shr_round(j, Nearest) == 0
    let negative_not_min_i32_and_u32 = |i: i32, j: u32| {
        assert_eq!(Integer::from(i).shr_round(j + 31, RoundingMode::Down), 0);
        assert_eq!(Integer::from(i).shr_round(j + 31, RoundingMode::Floor), -1);
        assert_eq!(Integer::from(i).shr_round(j + 31, RoundingMode::Up), -1);
        assert_eq!(Integer::from(i).shr_round(j + 31, RoundingMode::Ceiling), 0);
        assert_eq!(Integer::from(i).shr_round(j + 32, RoundingMode::Nearest), 0);
    };

    // n.shr_round(0, rm) == n
    #[allow(unknown_lints, identity_op)]
    let integer_and_rounding_mode = |n: Integer, rm: RoundingMode| {
        assert_eq!((&n).shr_round(0u32, rm), n);
    };

    // 0.shr_round(u, rm) == 0
    let u32_and_rounding_mode = |u: u32, rm: RoundingMode| {
        assert_eq!(Integer::ZERO.shr_round(u, rm), 0);
    };

    for (n, u) in log_pairs(exhaustive_integers(), exhaustive_u::<u32>()).take(LARGE_LIMIT) {
        integer_and_u32(n, u);
    }

    for (n, u) in random_pairs(
        &EXAMPLE_SEED,
        &(|seed| random_integers(seed, 32)),
        &(|seed| natural_u32s_geometric(seed, 32)),
    ).take(LARGE_LIMIT)
    {
        integer_and_u32(n, u);
    }

    for (n, u) in log_pairs(exhaustive_integers(), exhaustive_u::<u32>())
        .filter(|&(ref n, u)| !n.divisible_by_power_of_2(u))
        .take(LARGE_LIMIT)
    {
        integer_and_u32_inexact(n, u);
    }

    for (n, u) in random_pairs(
        &EXAMPLE_SEED,
        &(|seed| random_integers(seed, 32)),
        &(|seed| natural_u32s_geometric(seed, 32)),
    ).filter(|&(ref n, u)| !n.divisible_by_power_of_2(u))
        .take(LARGE_LIMIT)
    {
        integer_and_u32_inexact(n, u);
    }

    for (n, u, rm) in select_inputs_2(GenerationMode::Exhaustive).take(LARGE_LIMIT) {
        integer_u32_and_rounding_mode(n, u, rm);
    }

    for (n, u, rm) in select_inputs_2(GenerationMode::Random(32)).take(LARGE_LIMIT) {
        integer_u32_and_rounding_mode(n, u, rm);
    }

    for (n, u) in log_pairs(exhaustive_positive_x::<i32>(), exhaustive_u::<u32>()).take(LARGE_LIMIT)
    {
        positive_i32_and_u32(n, u);
    }

    for (n, u) in random_pairs(
        &EXAMPLE_SEED,
        &(|seed| random_positive_i(seed)),
        &(|seed| natural_u32s_geometric(seed, 32)),
    ).take(LARGE_LIMIT)
    {
        positive_i32_and_u32(n, u);
    }

    for (n, u) in log_pairs(
        exhaustive_negative_i::<i32>().filter(|&i| i != i32::min_value()),
        exhaustive_u::<u32>(),
    ).take(LARGE_LIMIT)
    {
        negative_not_min_i32_and_u32(n, u);
    }

    for (n, u) in random_pairs(
        &EXAMPLE_SEED,
        &(|seed| random_negative_i(seed).filter(|&i| i != i32::min_value())),
        &(|seed| natural_u32s_geometric(seed, 32)),
    ).take(LARGE_LIMIT)
    {
        negative_not_min_i32_and_u32(n, u);
    }

    for (n, rm) in log_pairs(exhaustive_integers(), exhaustive_rounding_modes()).take(LARGE_LIMIT) {
        integer_and_rounding_mode(n, rm);
    }

    for (n, rm) in random_pairs(
        &EXAMPLE_SEED,
        &(|seed| random_integers(seed, 32)),
        &(|seed| random_rounding_modes(seed)),
    ).take(LARGE_LIMIT)
    {
        integer_and_rounding_mode(n, rm);
    }

    for (u, rm) in log_pairs(exhaustive_u(), exhaustive_rounding_modes()).take(LARGE_LIMIT) {
        u32_and_rounding_mode(u, rm);
    }

    for (u, rm) in random_pairs(
        &EXAMPLE_SEED,
        &(|seed| natural_u32s_geometric(seed, 32)),
        &(|seed| random_rounding_modes(seed)),
    ).take(LARGE_LIMIT)
    {
        u32_and_rounding_mode(u, rm);
    }
}
