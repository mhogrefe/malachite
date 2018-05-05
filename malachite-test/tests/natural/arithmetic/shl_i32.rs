use common::test_properties;
use malachite_base::num::{ShlRound, ShlRoundAssign, Zero};
use malachite_base::round::RoundingMode;
use malachite_nz::natural::Natural;
use malachite_test::common::{natural_to_rug_integer, rug_integer_to_natural};
use malachite_test::inputs::base::{pairs_of_signed_and_rounding_mode, signeds};
use malachite_test::inputs::natural::{
    naturals, pairs_of_natural_and_rounding_mode, pairs_of_natural_and_small_i32,
    triples_of_natural_small_i32_and_rounding_mode_var_1,
};
use rug;
use std::str::FromStr;

#[test]
fn test_shl_i32() {
    let test = |u, v: i32, out| {
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

        let n = &Natural::from_str(u).unwrap() << v;
        assert_eq!(n.to_string(), out);
        assert!(n.is_valid());
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

    test("0", -10, "0");
    test("123", 0, "123");
    test("245", -1, "122");
    test("246", -1, "123");
    test("247", -1, "123");
    test("491", -2, "122");
    test("492", -2, "123");
    test("493", -2, "123");
    test("4127195135", -25, "122");
    test("4127195136", -25, "123");
    test("4127195137", -25, "123");
    test("8254390271", -26, "122");
    test("8254390272", -26, "123");
    test("8254390273", -26, "123");
    test("155921023828072216384094494261247", -100, "122");
    test("155921023828072216384094494261248", -100, "123");
    test("155921023828072216384094494261249", -100, "123");
    test("4294967295", -1, "2147483647");
    test("4294967296", -1, "2147483648");
    test("4294967297", -1, "2147483648");
    test("7999999999999", -3, "999999999999");
    test("8000000000000", -3, "1000000000000");
    test("8000000000001", -3, "1000000000000");
    test("16777216000000000000", -24, "1000000000000");
    test("33554432000000000000", -25, "1000000000000");
    test("2147483648000000000000", -31, "1000000000000");
    test("4294967296000000000000", -32, "1000000000000");
    test("8589934592000000000000", -33, "1000000000000");
    test(
        "1267650600228229401496703205376000000000000",
        -100,
        "1000000000000",
    );
    test("1000000000000", -10, "976562500");
    test("980657949", -72, "0");
    test("4294967295", -31, "1");
    test("4294967295", -32, "0");
    test("4294967296", -32, "1");
    test("4294967296", -33, "0");
}

#[test]
fn shl_i32_properties() {
    test_properties(pairs_of_natural_and_small_i32, |&(ref n, i)| {
        let mut mut_n = n.clone();
        mut_n <<= i;
        assert!(mut_n.is_valid());
        let shifted = mut_n;

        let mut rug_n = natural_to_rug_integer(n);
        rug_n <<= i;
        assert_eq!(rug_integer_to_natural(&rug_n), shifted);

        let shifted_alt = n << i;
        assert!(shifted_alt.is_valid());
        assert_eq!(shifted_alt, shifted);

        let shifted_alt = n.clone() << i;
        assert!(shifted_alt.is_valid());
        assert_eq!(shifted_alt, shifted);

        assert_eq!(
            rug_integer_to_natural(&(natural_to_rug_integer(n) << i)),
            shifted
        );

        assert_eq!(n.shl_round(i, RoundingMode::Floor), shifted);
    });

    #[allow(unknown_lints, identity_op)]
    test_properties(naturals, |n| {
        assert_eq!(n << 0i32, *n);
    });

    test_properties(signeds, |&i: &i32| {
        assert_eq!(Natural::ZERO << i, 0);
    });
}

#[test]
fn test_shl_round_i32() {
    let test = |i, j: i32, rm: RoundingMode, out| {
        let mut n = Natural::from_str(i).unwrap();
        n.shl_round_assign(j, rm);
        assert_eq!(n.to_string(), out);
        assert!(n.is_valid());

        let n = Natural::from_str(i).unwrap().shl_round(j, rm);
        assert_eq!(n.to_string(), out);
        assert!(n.is_valid());

        let n = &Natural::from_str(i).unwrap().shl_round(j, rm);
        assert_eq!(n.to_string(), out);
        assert!(n.is_valid());
    };
    test("0", 0, RoundingMode::Down, "0");
    test("0", 0, RoundingMode::Up, "0");
    test("0", 0, RoundingMode::Floor, "0");
    test("0", 0, RoundingMode::Ceiling, "0");
    test("0", 0, RoundingMode::Nearest, "0");
    test("0", 0, RoundingMode::Exact, "0");

    test("0", -10, RoundingMode::Down, "0");
    test("0", -10, RoundingMode::Up, "0");
    test("0", -10, RoundingMode::Floor, "0");
    test("0", -10, RoundingMode::Ceiling, "0");
    test("0", -10, RoundingMode::Nearest, "0");
    test("0", -10, RoundingMode::Exact, "0");

    test("123", 0, RoundingMode::Down, "123");
    test("123", 0, RoundingMode::Up, "123");
    test("123", 0, RoundingMode::Floor, "123");
    test("123", 0, RoundingMode::Ceiling, "123");
    test("123", 0, RoundingMode::Nearest, "123");
    test("123", 0, RoundingMode::Exact, "123");

    test("245", -1, RoundingMode::Down, "122");
    test("245", -1, RoundingMode::Up, "123");
    test("245", -1, RoundingMode::Floor, "122");
    test("245", -1, RoundingMode::Ceiling, "123");
    test("245", -1, RoundingMode::Nearest, "122");

    test("246", -1, RoundingMode::Down, "123");
    test("246", -1, RoundingMode::Up, "123");
    test("246", -1, RoundingMode::Floor, "123");
    test("246", -1, RoundingMode::Ceiling, "123");
    test("246", -1, RoundingMode::Nearest, "123");
    test("246", -1, RoundingMode::Exact, "123");

    test("247", -1, RoundingMode::Down, "123");
    test("247", -1, RoundingMode::Up, "124");
    test("247", -1, RoundingMode::Floor, "123");
    test("247", -1, RoundingMode::Ceiling, "124");
    test("247", -1, RoundingMode::Nearest, "124");

    test("491", -2, RoundingMode::Down, "122");
    test("491", -2, RoundingMode::Up, "123");
    test("491", -2, RoundingMode::Floor, "122");
    test("491", -2, RoundingMode::Ceiling, "123");
    test("491", -2, RoundingMode::Nearest, "123");

    test("492", -2, RoundingMode::Down, "123");
    test("492", -2, RoundingMode::Up, "123");
    test("492", -2, RoundingMode::Floor, "123");
    test("492", -2, RoundingMode::Ceiling, "123");
    test("492", -2, RoundingMode::Nearest, "123");
    test("492", -2, RoundingMode::Exact, "123");

    test("493", -2, RoundingMode::Down, "123");
    test("493", -2, RoundingMode::Up, "124");
    test("493", -2, RoundingMode::Floor, "123");
    test("493", -2, RoundingMode::Ceiling, "124");
    test("493", -2, RoundingMode::Nearest, "123");

    test("4127195135", -25, RoundingMode::Down, "122");
    test("4127195135", -25, RoundingMode::Up, "123");
    test("4127195135", -25, RoundingMode::Floor, "122");
    test("4127195135", -25, RoundingMode::Ceiling, "123");
    test("4127195135", -25, RoundingMode::Nearest, "123");

    test("4127195136", -25, RoundingMode::Down, "123");
    test("4127195136", -25, RoundingMode::Up, "123");
    test("4127195136", -25, RoundingMode::Floor, "123");
    test("4127195136", -25, RoundingMode::Ceiling, "123");
    test("4127195136", -25, RoundingMode::Nearest, "123");
    test("4127195136", -25, RoundingMode::Exact, "123");

    test("4127195137", -25, RoundingMode::Down, "123");
    test("4127195137", -25, RoundingMode::Up, "124");
    test("4127195137", -25, RoundingMode::Floor, "123");
    test("4127195137", -25, RoundingMode::Ceiling, "124");
    test("4127195137", -25, RoundingMode::Nearest, "123");

    test("8254390271", -26, RoundingMode::Down, "122");
    test("8254390271", -26, RoundingMode::Up, "123");
    test("8254390271", -26, RoundingMode::Floor, "122");
    test("8254390271", -26, RoundingMode::Ceiling, "123");
    test("8254390271", -26, RoundingMode::Nearest, "123");

    test("8254390272", -26, RoundingMode::Down, "123");
    test("8254390272", -26, RoundingMode::Up, "123");
    test("8254390272", -26, RoundingMode::Floor, "123");
    test("8254390272", -26, RoundingMode::Ceiling, "123");
    test("8254390272", -26, RoundingMode::Nearest, "123");
    test("8254390272", -26, RoundingMode::Exact, "123");

    test("8254390273", -26, RoundingMode::Down, "123");
    test("8254390273", -26, RoundingMode::Up, "124");
    test("8254390273", -26, RoundingMode::Floor, "123");
    test("8254390273", -26, RoundingMode::Ceiling, "124");
    test("8254390273", -26, RoundingMode::Nearest, "123");

    test(
        "155921023828072216384094494261247",
        -100,
        RoundingMode::Down,
        "122",
    );
    test(
        "155921023828072216384094494261247",
        -100,
        RoundingMode::Up,
        "123",
    );
    test(
        "155921023828072216384094494261247",
        -100,
        RoundingMode::Floor,
        "122",
    );
    test(
        "155921023828072216384094494261247",
        -100,
        RoundingMode::Ceiling,
        "123",
    );
    test(
        "155921023828072216384094494261247",
        -100,
        RoundingMode::Nearest,
        "123",
    );

    test(
        "155921023828072216384094494261248",
        -100,
        RoundingMode::Down,
        "123",
    );
    test(
        "155921023828072216384094494261248",
        -100,
        RoundingMode::Up,
        "123",
    );
    test(
        "155921023828072216384094494261248",
        -100,
        RoundingMode::Floor,
        "123",
    );
    test(
        "155921023828072216384094494261248",
        -100,
        RoundingMode::Ceiling,
        "123",
    );
    test(
        "155921023828072216384094494261248",
        -100,
        RoundingMode::Nearest,
        "123",
    );
    test(
        "155921023828072216384094494261248",
        -100,
        RoundingMode::Exact,
        "123",
    );

    test(
        "155921023828072216384094494261249",
        -100,
        RoundingMode::Down,
        "123",
    );
    test(
        "155921023828072216384094494261249",
        -100,
        RoundingMode::Up,
        "124",
    );
    test(
        "155921023828072216384094494261249",
        -100,
        RoundingMode::Floor,
        "123",
    );
    test(
        "155921023828072216384094494261249",
        -100,
        RoundingMode::Ceiling,
        "124",
    );
    test(
        "155921023828072216384094494261249",
        -100,
        RoundingMode::Nearest,
        "123",
    );

    test("4294967295", -1, RoundingMode::Down, "2147483647");
    test("4294967295", -1, RoundingMode::Up, "2147483648");
    test("4294967295", -1, RoundingMode::Floor, "2147483647");
    test("4294967295", -1, RoundingMode::Ceiling, "2147483648");
    test("4294967295", -1, RoundingMode::Nearest, "2147483648");

    test("4294967296", -1, RoundingMode::Down, "2147483648");
    test("4294967296", -1, RoundingMode::Up, "2147483648");
    test("4294967296", -1, RoundingMode::Floor, "2147483648");
    test("4294967296", -1, RoundingMode::Ceiling, "2147483648");
    test("4294967296", -1, RoundingMode::Nearest, "2147483648");
    test("4294967296", -1, RoundingMode::Exact, "2147483648");

    test("4294967297", -1, RoundingMode::Down, "2147483648");
    test("4294967297", -1, RoundingMode::Up, "2147483649");
    test("4294967297", -1, RoundingMode::Floor, "2147483648");
    test("4294967297", -1, RoundingMode::Ceiling, "2147483649");
    test("4294967297", -1, RoundingMode::Nearest, "2147483648");

    test("1000000000000", 0, RoundingMode::Down, "1000000000000");
    test("1000000000000", 0, RoundingMode::Up, "1000000000000");
    test("1000000000000", 0, RoundingMode::Floor, "1000000000000");
    test("1000000000000", 0, RoundingMode::Ceiling, "1000000000000");
    test("1000000000000", 0, RoundingMode::Nearest, "1000000000000");
    test("1000000000000", 0, RoundingMode::Exact, "1000000000000");

    test("7999999999999", -3, RoundingMode::Down, "999999999999");
    test("7999999999999", -3, RoundingMode::Up, "1000000000000");
    test("7999999999999", -3, RoundingMode::Floor, "999999999999");
    test("7999999999999", -3, RoundingMode::Ceiling, "1000000000000");
    test("7999999999999", -3, RoundingMode::Nearest, "1000000000000");

    test("8000000000000", -3, RoundingMode::Down, "1000000000000");
    test("8000000000000", -3, RoundingMode::Up, "1000000000000");
    test("8000000000000", -3, RoundingMode::Floor, "1000000000000");
    test("8000000000000", -3, RoundingMode::Ceiling, "1000000000000");
    test("8000000000000", -3, RoundingMode::Nearest, "1000000000000");
    test("8000000000000", -3, RoundingMode::Exact, "1000000000000");

    test("8000000000001", -3, RoundingMode::Down, "1000000000000");
    test("8000000000001", -3, RoundingMode::Up, "1000000000001");
    test("8000000000001", -3, RoundingMode::Floor, "1000000000000");
    test("8000000000001", -3, RoundingMode::Ceiling, "1000000000001");
    test("8000000000001", -3, RoundingMode::Nearest, "1000000000000");

    test(
        "16777216000000000000",
        -24,
        RoundingMode::Down,
        "1000000000000",
    );
    test(
        "16777216000000000000",
        -24,
        RoundingMode::Up,
        "1000000000000",
    );
    test(
        "16777216000000000000",
        -24,
        RoundingMode::Floor,
        "1000000000000",
    );
    test(
        "16777216000000000000",
        -24,
        RoundingMode::Ceiling,
        "1000000000000",
    );
    test(
        "16777216000000000000",
        -24,
        RoundingMode::Nearest,
        "1000000000000",
    );
    test(
        "16777216000000000000",
        -24,
        RoundingMode::Exact,
        "1000000000000",
    );

    test(
        "33554432000000000000",
        -25,
        RoundingMode::Down,
        "1000000000000",
    );
    test(
        "33554432000000000000",
        -25,
        RoundingMode::Up,
        "1000000000000",
    );
    test(
        "33554432000000000000",
        -25,
        RoundingMode::Floor,
        "1000000000000",
    );
    test(
        "33554432000000000000",
        -25,
        RoundingMode::Ceiling,
        "1000000000000",
    );
    test(
        "33554432000000000000",
        -25,
        RoundingMode::Nearest,
        "1000000000000",
    );
    test(
        "33554432000000000000",
        -25,
        RoundingMode::Exact,
        "1000000000000",
    );

    test(
        "2147483648000000000000",
        -31,
        RoundingMode::Down,
        "1000000000000",
    );
    test(
        "2147483648000000000000",
        -31,
        RoundingMode::Up,
        "1000000000000",
    );
    test(
        "2147483648000000000000",
        -31,
        RoundingMode::Floor,
        "1000000000000",
    );
    test(
        "2147483648000000000000",
        -31,
        RoundingMode::Ceiling,
        "1000000000000",
    );
    test(
        "2147483648000000000000",
        -31,
        RoundingMode::Nearest,
        "1000000000000",
    );
    test(
        "2147483648000000000000",
        -31,
        RoundingMode::Exact,
        "1000000000000",
    );

    test(
        "4294967296000000000000",
        -32,
        RoundingMode::Down,
        "1000000000000",
    );
    test(
        "4294967296000000000000",
        -32,
        RoundingMode::Up,
        "1000000000000",
    );
    test(
        "4294967296000000000000",
        -32,
        RoundingMode::Floor,
        "1000000000000",
    );
    test(
        "4294967296000000000000",
        -32,
        RoundingMode::Ceiling,
        "1000000000000",
    );
    test(
        "4294967296000000000000",
        -32,
        RoundingMode::Nearest,
        "1000000000000",
    );
    test(
        "4294967296000000000000",
        -32,
        RoundingMode::Exact,
        "1000000000000",
    );

    test(
        "8589934592000000000000",
        -33,
        RoundingMode::Down,
        "1000000000000",
    );
    test(
        "8589934592000000000000",
        -33,
        RoundingMode::Up,
        "1000000000000",
    );
    test(
        "8589934592000000000000",
        -33,
        RoundingMode::Floor,
        "1000000000000",
    );
    test(
        "8589934592000000000000",
        -33,
        RoundingMode::Ceiling,
        "1000000000000",
    );
    test(
        "8589934592000000000000",
        -33,
        RoundingMode::Nearest,
        "1000000000000",
    );
    test(
        "8589934592000000000000",
        -33,
        RoundingMode::Exact,
        "1000000000000",
    );

    test(
        "1267650600228229401496703205376000000000000",
        -100,
        RoundingMode::Down,
        "1000000000000",
    );
    test(
        "1267650600228229401496703205376000000000000",
        -100,
        RoundingMode::Up,
        "1000000000000",
    );
    test(
        "1267650600228229401496703205376000000000000",
        -100,
        RoundingMode::Floor,
        "1000000000000",
    );
    test(
        "1267650600228229401496703205376000000000000",
        -100,
        RoundingMode::Ceiling,
        "1000000000000",
    );
    test(
        "1267650600228229401496703205376000000000000",
        -100,
        RoundingMode::Nearest,
        "1000000000000",
    );
    test(
        "1267650600228229401496703205376000000000000",
        -100,
        RoundingMode::Exact,
        "1000000000000",
    );

    test("1000000000000", -10, RoundingMode::Down, "976562500");
    test("1000000000000", -10, RoundingMode::Up, "976562500");
    test("1000000000000", -10, RoundingMode::Floor, "976562500");
    test("1000000000000", -10, RoundingMode::Ceiling, "976562500");
    test("1000000000000", -10, RoundingMode::Nearest, "976562500");
    test("1000000000000", -10, RoundingMode::Exact, "976562500");

    test("980657949", -72, RoundingMode::Down, "0");
    test("980657949", -72, RoundingMode::Up, "1");
    test("980657949", -72, RoundingMode::Floor, "0");
    test("980657949", -72, RoundingMode::Ceiling, "1");
    test("980657949", -72, RoundingMode::Nearest, "0");

    test("4294967295", -31, RoundingMode::Down, "1");
    test("4294967295", -31, RoundingMode::Up, "2");
    test("4294967295", -31, RoundingMode::Floor, "1");
    test("4294967295", -31, RoundingMode::Ceiling, "2");
    test("4294967295", -31, RoundingMode::Nearest, "2");

    test("4294967295", -32, RoundingMode::Down, "0");
    test("4294967295", -32, RoundingMode::Up, "1");
    test("4294967295", -32, RoundingMode::Floor, "0");
    test("4294967295", -32, RoundingMode::Ceiling, "1");
    test("4294967295", -32, RoundingMode::Nearest, "1");

    test("4294967296", -32, RoundingMode::Down, "1");
    test("4294967296", -32, RoundingMode::Up, "1");
    test("4294967296", -32, RoundingMode::Floor, "1");
    test("4294967296", -32, RoundingMode::Ceiling, "1");
    test("4294967296", -32, RoundingMode::Nearest, "1");
    test("4294967296", -32, RoundingMode::Exact, "1");

    test("4294967296", -33, RoundingMode::Down, "0");
    test("4294967296", -33, RoundingMode::Up, "1");
    test("4294967296", -33, RoundingMode::Floor, "0");
    test("4294967296", -33, RoundingMode::Ceiling, "1");
    test("4294967296", -33, RoundingMode::Nearest, "0");

    test("0", 10, RoundingMode::Exact, "0");
    test("123", 1, RoundingMode::Exact, "246");
    test("123", 2, RoundingMode::Exact, "492");
    test("123", 25, RoundingMode::Exact, "4127195136");
    test("123", 26, RoundingMode::Exact, "8254390272");
    test(
        "123",
        100,
        RoundingMode::Exact,
        "155921023828072216384094494261248",
    );
    test("2147483648", 1, RoundingMode::Exact, "4294967296");
    test("1000000000000", 3, RoundingMode::Exact, "8000000000000");
    test(
        "1000000000000",
        24,
        RoundingMode::Exact,
        "16777216000000000000",
    );
    test(
        "1000000000000",
        25,
        RoundingMode::Exact,
        "33554432000000000000",
    );
    test(
        "1000000000000",
        31,
        RoundingMode::Exact,
        "2147483648000000000000",
    );
    test(
        "1000000000000",
        32,
        RoundingMode::Exact,
        "4294967296000000000000",
    );
    test(
        "1000000000000",
        33,
        RoundingMode::Exact,
        "8589934592000000000000",
    );
    test(
        "1000000000000",
        100,
        RoundingMode::Exact,
        "1267650600228229401496703205376000000000000",
    );
}

#[test]
#[should_panic(expected = "Right shift is not exact: 123 >>= 1")]
fn shl_round_assign_i32_fail_1() {
    Natural::from(123u32).shl_round_assign(-1i32, RoundingMode::Exact);
}

#[test]
#[should_panic(expected = "Right shift is not exact: 123 >>= 100")]
fn shl_round_assign_i32_fail_2() {
    Natural::from(123u32).shl_round_assign(-100i32, RoundingMode::Exact);
}

#[test]
#[should_panic(expected = "Right shift is not exact: 1000000000001 >>= 1")]
fn shl_round_assign_i32_fail_3() {
    Natural::from_str("1000000000001")
        .unwrap()
        .shl_round_assign(-1i32, RoundingMode::Exact);
}

#[test]
#[should_panic(expected = "Right shift is not exact: 1000000000001 >>= 100")]
fn shl_round_assign_i32_fail_4() {
    Natural::from_str("1000000000001")
        .unwrap()
        .shl_round_assign(-100i32, RoundingMode::Exact);
}

#[test]
#[should_panic(expected = "Right shift is not exact: 123 >>= 1")]
fn shl_round_i32_fail_1() {
    Natural::from(123u32).shl_round(-1i32, RoundingMode::Exact);
}

#[test]
#[should_panic(expected = "Right shift is not exact: 123 >>= 100")]
fn shl_round_i32_fail_2() {
    Natural::from(123u32).shl_round(-100i32, RoundingMode::Exact);
}

#[test]
#[should_panic(expected = "Right shift is not exact: 1000000000001 >>= 1")]
fn shl_round_i32_fail_3() {
    Natural::from_str("1000000000001")
        .unwrap()
        .shl_round(-1i32, RoundingMode::Exact);
}

#[test]
#[should_panic(expected = "Right shift is not exact: 1000000000001 >>= 100")]
fn shl_round_i32_fail_4() {
    Natural::from_str("1000000000001")
        .unwrap()
        .shl_round(-100i32, RoundingMode::Exact);
}

#[test]
#[should_panic(expected = "Right shift is not exact: 123 >> 1")]
fn shl_round_ref_i32_fail_1() {
    (&Natural::from(123u32)).shl_round(-1i32, RoundingMode::Exact);
}

#[test]
#[should_panic(expected = "Right shift is not exact: 123 >> 100")]
fn shl_round_ref_i32_fail_2() {
    (&Natural::from(123u32)).shl_round(-100i32, RoundingMode::Exact);
}

#[test]
#[should_panic(expected = "Right shift is not exact: 1000000000001 >> 1")]
fn shl_round_ref_i32_fail_3() {
    (&Natural::from_str("1000000000001").unwrap()).shl_round(-1i32, RoundingMode::Exact);
}

#[test]
#[should_panic(expected = "Right shift is not exact: 1000000000001 >> 100")]
fn shl_round_ref_i32_fail_4() {
    (&Natural::from_str("1000000000001").unwrap()).shl_round(-100i32, RoundingMode::Exact);
}

#[test]
fn shl_round_u32_properties() {
    test_properties(
        triples_of_natural_small_i32_and_rounding_mode_var_1,
        |&(ref n, i, rm)| {
            let mut mut_n = n.clone();
            mut_n.shl_round_assign(i, rm);
            assert!(mut_n.is_valid());
            let shifted = mut_n;

            let shifted_alt = n.shl_round(i, rm);
            assert!(shifted_alt.is_valid());
            assert_eq!(shifted_alt, shifted);

            let shifted_alt = n.clone().shl_round(i, rm);
            assert!(shifted_alt.is_valid());
            assert_eq!(shifted_alt, shifted);
        },
    );

    #[allow(unknown_lints, identity_op)]
    test_properties(pairs_of_natural_and_rounding_mode, |&(ref n, rm)| {
        assert_eq!(n.shl_round(0i32, rm), *n);
    });

    test_properties(
        pairs_of_signed_and_rounding_mode,
        |&(i, rm): &(i32, RoundingMode)| {
            assert_eq!(Natural::ZERO.shl_round(i, rm), 0);
        },
    );
}
