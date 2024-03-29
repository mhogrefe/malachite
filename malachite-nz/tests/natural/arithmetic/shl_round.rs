use malachite_base::num::arithmetic::traits::{ArithmeticCheckedShl, ShlRound, ShlRoundAssign};
use malachite_base::num::basic::signeds::PrimitiveSigned;
use malachite_base::num::basic::traits::{NegativeOne, Zero};
use malachite_base::num::conversion::traits::ExactFrom;
use malachite_base::rounding_modes::RoundingMode;
use malachite_base::test_util::generators::{
    signed_rounding_mode_pair_gen, unsigned_signed_rounding_mode_triple_gen_var_2,
};
use malachite_nz::natural::Natural;
use malachite_nz::platform::Limb;
use malachite_nz::test_util::generators::{
    natural_rounding_mode_pair_gen, natural_signed_rounding_mode_triple_gen_var_1,
};
use std::cmp::Ordering;
use std::ops::Shr;
use std::panic::catch_unwind;
use std::str::FromStr;

macro_rules! test_shl_round_signed_helper {
    ($t:ident) => {
        let test = |s, j: $t, rm: RoundingMode, out, o| {
            let u = Natural::from_str(s).unwrap();

            let mut n = u.clone();
            assert_eq!(n.shl_round_assign(j, rm), o);
            assert_eq!(n.to_string(), out);
            assert!(n.is_valid());

            let (n, o_alt) = u.clone().shl_round(j, rm);
            assert_eq!(n.to_string(), out);
            assert!(n.is_valid());
            assert_eq!(o_alt, o);

            let (n, o_alt) = (&u).shl_round(j, rm);
            assert_eq!(n.to_string(), out);
            assert!(n.is_valid());
            assert_eq!(o_alt, o);
        };
        test("0", 0, RoundingMode::Down, "0", Ordering::Equal);
        test("0", 0, RoundingMode::Up, "0", Ordering::Equal);
        test("0", 0, RoundingMode::Floor, "0", Ordering::Equal);
        test("0", 0, RoundingMode::Ceiling, "0", Ordering::Equal);
        test("0", 0, RoundingMode::Nearest, "0", Ordering::Equal);
        test("0", 0, RoundingMode::Exact, "0", Ordering::Equal);

        test("0", -10, RoundingMode::Down, "0", Ordering::Equal);
        test("0", -10, RoundingMode::Up, "0", Ordering::Equal);
        test("0", -10, RoundingMode::Floor, "0", Ordering::Equal);
        test("0", -10, RoundingMode::Ceiling, "0", Ordering::Equal);
        test("0", -10, RoundingMode::Nearest, "0", Ordering::Equal);
        test("0", -10, RoundingMode::Exact, "0", Ordering::Equal);

        test("123", 0, RoundingMode::Down, "123", Ordering::Equal);
        test("123", 0, RoundingMode::Up, "123", Ordering::Equal);
        test("123", 0, RoundingMode::Floor, "123", Ordering::Equal);
        test("123", 0, RoundingMode::Ceiling, "123", Ordering::Equal);
        test("123", 0, RoundingMode::Nearest, "123", Ordering::Equal);
        test("123", 0, RoundingMode::Exact, "123", Ordering::Equal);

        test("245", -1, RoundingMode::Down, "122", Ordering::Less);
        test("245", -1, RoundingMode::Up, "123", Ordering::Greater);
        test("245", -1, RoundingMode::Floor, "122", Ordering::Less);
        test("245", -1, RoundingMode::Ceiling, "123", Ordering::Greater);
        test("245", -1, RoundingMode::Nearest, "122", Ordering::Less);

        test("246", -1, RoundingMode::Down, "123", Ordering::Equal);
        test("246", -1, RoundingMode::Up, "123", Ordering::Equal);
        test("246", -1, RoundingMode::Floor, "123", Ordering::Equal);
        test("246", -1, RoundingMode::Ceiling, "123", Ordering::Equal);
        test("246", -1, RoundingMode::Nearest, "123", Ordering::Equal);
        test("246", -1, RoundingMode::Exact, "123", Ordering::Equal);

        test("247", -1, RoundingMode::Down, "123", Ordering::Less);
        test("247", -1, RoundingMode::Up, "124", Ordering::Greater);
        test("247", -1, RoundingMode::Floor, "123", Ordering::Less);
        test("247", -1, RoundingMode::Ceiling, "124", Ordering::Greater);
        test("247", -1, RoundingMode::Nearest, "124", Ordering::Greater);

        test("491", -2, RoundingMode::Down, "122", Ordering::Less);
        test("491", -2, RoundingMode::Up, "123", Ordering::Greater);
        test("491", -2, RoundingMode::Floor, "122", Ordering::Less);
        test("491", -2, RoundingMode::Ceiling, "123", Ordering::Greater);
        test("491", -2, RoundingMode::Nearest, "123", Ordering::Greater);

        test("492", -2, RoundingMode::Down, "123", Ordering::Equal);
        test("492", -2, RoundingMode::Up, "123", Ordering::Equal);
        test("492", -2, RoundingMode::Floor, "123", Ordering::Equal);
        test("492", -2, RoundingMode::Ceiling, "123", Ordering::Equal);
        test("492", -2, RoundingMode::Nearest, "123", Ordering::Equal);
        test("492", -2, RoundingMode::Exact, "123", Ordering::Equal);

        test("493", -2, RoundingMode::Down, "123", Ordering::Less);
        test("493", -2, RoundingMode::Up, "124", Ordering::Greater);
        test("493", -2, RoundingMode::Floor, "123", Ordering::Less);
        test("493", -2, RoundingMode::Ceiling, "124", Ordering::Greater);
        test("493", -2, RoundingMode::Nearest, "123", Ordering::Less);

        test("4127195135", -25, RoundingMode::Down, "122", Ordering::Less);
        test(
            "4127195135",
            -25,
            RoundingMode::Up,
            "123",
            Ordering::Greater,
        );
        test(
            "4127195135",
            -25,
            RoundingMode::Floor,
            "122",
            Ordering::Less,
        );
        test(
            "4127195135",
            -25,
            RoundingMode::Ceiling,
            "123",
            Ordering::Greater,
        );
        test(
            "4127195135",
            -25,
            RoundingMode::Nearest,
            "123",
            Ordering::Greater,
        );

        test(
            "4127195136",
            -25,
            RoundingMode::Down,
            "123",
            Ordering::Equal,
        );
        test("4127195136", -25, RoundingMode::Up, "123", Ordering::Equal);
        test(
            "4127195136",
            -25,
            RoundingMode::Floor,
            "123",
            Ordering::Equal,
        );
        test(
            "4127195136",
            -25,
            RoundingMode::Ceiling,
            "123",
            Ordering::Equal,
        );
        test(
            "4127195136",
            -25,
            RoundingMode::Nearest,
            "123",
            Ordering::Equal,
        );
        test(
            "4127195136",
            -25,
            RoundingMode::Exact,
            "123",
            Ordering::Equal,
        );

        test("4127195137", -25, RoundingMode::Down, "123", Ordering::Less);
        test(
            "4127195137",
            -25,
            RoundingMode::Up,
            "124",
            Ordering::Greater,
        );
        test(
            "4127195137",
            -25,
            RoundingMode::Floor,
            "123",
            Ordering::Less,
        );
        test(
            "4127195137",
            -25,
            RoundingMode::Ceiling,
            "124",
            Ordering::Greater,
        );
        test(
            "4127195137",
            -25,
            RoundingMode::Nearest,
            "123",
            Ordering::Less,
        );

        test("8254390271", -26, RoundingMode::Down, "122", Ordering::Less);
        test(
            "8254390271",
            -26,
            RoundingMode::Up,
            "123",
            Ordering::Greater,
        );
        test(
            "8254390271",
            -26,
            RoundingMode::Floor,
            "122",
            Ordering::Less,
        );
        test(
            "8254390271",
            -26,
            RoundingMode::Ceiling,
            "123",
            Ordering::Greater,
        );
        test(
            "8254390271",
            -26,
            RoundingMode::Nearest,
            "123",
            Ordering::Greater,
        );

        test(
            "8254390272",
            -26,
            RoundingMode::Down,
            "123",
            Ordering::Equal,
        );
        test("8254390272", -26, RoundingMode::Up, "123", Ordering::Equal);
        test(
            "8254390272",
            -26,
            RoundingMode::Floor,
            "123",
            Ordering::Equal,
        );
        test(
            "8254390272",
            -26,
            RoundingMode::Ceiling,
            "123",
            Ordering::Equal,
        );
        test(
            "8254390272",
            -26,
            RoundingMode::Nearest,
            "123",
            Ordering::Equal,
        );
        test(
            "8254390272",
            -26,
            RoundingMode::Exact,
            "123",
            Ordering::Equal,
        );

        test("8254390273", -26, RoundingMode::Down, "123", Ordering::Less);
        test(
            "8254390273",
            -26,
            RoundingMode::Up,
            "124",
            Ordering::Greater,
        );
        test(
            "8254390273",
            -26,
            RoundingMode::Floor,
            "123",
            Ordering::Less,
        );
        test(
            "8254390273",
            -26,
            RoundingMode::Ceiling,
            "124",
            Ordering::Greater,
        );
        test(
            "8254390273",
            -26,
            RoundingMode::Nearest,
            "123",
            Ordering::Less,
        );

        test(
            "155921023828072216384094494261247",
            -100,
            RoundingMode::Down,
            "122",
            Ordering::Less,
        );
        test(
            "155921023828072216384094494261247",
            -100,
            RoundingMode::Up,
            "123",
            Ordering::Greater,
        );
        test(
            "155921023828072216384094494261247",
            -100,
            RoundingMode::Floor,
            "122",
            Ordering::Less,
        );
        test(
            "155921023828072216384094494261247",
            -100,
            RoundingMode::Ceiling,
            "123",
            Ordering::Greater,
        );
        test(
            "155921023828072216384094494261247",
            -100,
            RoundingMode::Nearest,
            "123",
            Ordering::Greater,
        );

        test(
            "155921023828072216384094494261248",
            -100,
            RoundingMode::Down,
            "123",
            Ordering::Equal,
        );
        test(
            "155921023828072216384094494261248",
            -100,
            RoundingMode::Up,
            "123",
            Ordering::Equal,
        );
        test(
            "155921023828072216384094494261248",
            -100,
            RoundingMode::Floor,
            "123",
            Ordering::Equal,
        );
        test(
            "155921023828072216384094494261248",
            -100,
            RoundingMode::Ceiling,
            "123",
            Ordering::Equal,
        );
        test(
            "155921023828072216384094494261248",
            -100,
            RoundingMode::Nearest,
            "123",
            Ordering::Equal,
        );
        test(
            "155921023828072216384094494261248",
            -100,
            RoundingMode::Exact,
            "123",
            Ordering::Equal,
        );

        test(
            "155921023828072216384094494261249",
            -100,
            RoundingMode::Down,
            "123",
            Ordering::Less,
        );
        test(
            "155921023828072216384094494261249",
            -100,
            RoundingMode::Up,
            "124",
            Ordering::Greater,
        );
        test(
            "155921023828072216384094494261249",
            -100,
            RoundingMode::Floor,
            "123",
            Ordering::Less,
        );
        test(
            "155921023828072216384094494261249",
            -100,
            RoundingMode::Ceiling,
            "124",
            Ordering::Greater,
        );
        test(
            "155921023828072216384094494261249",
            -100,
            RoundingMode::Nearest,
            "123",
            Ordering::Less,
        );

        test(
            "4294967295",
            -1,
            RoundingMode::Down,
            "2147483647",
            Ordering::Less,
        );
        test(
            "4294967295",
            -1,
            RoundingMode::Up,
            "2147483648",
            Ordering::Greater,
        );
        test(
            "4294967295",
            -1,
            RoundingMode::Floor,
            "2147483647",
            Ordering::Less,
        );
        test(
            "4294967295",
            -1,
            RoundingMode::Ceiling,
            "2147483648",
            Ordering::Greater,
        );
        test(
            "4294967295",
            -1,
            RoundingMode::Nearest,
            "2147483648",
            Ordering::Greater,
        );

        test(
            "4294967296",
            -1,
            RoundingMode::Down,
            "2147483648",
            Ordering::Equal,
        );
        test(
            "4294967296",
            -1,
            RoundingMode::Up,
            "2147483648",
            Ordering::Equal,
        );
        test(
            "4294967296",
            -1,
            RoundingMode::Floor,
            "2147483648",
            Ordering::Equal,
        );
        test(
            "4294967296",
            -1,
            RoundingMode::Ceiling,
            "2147483648",
            Ordering::Equal,
        );
        test(
            "4294967296",
            -1,
            RoundingMode::Nearest,
            "2147483648",
            Ordering::Equal,
        );
        test(
            "4294967296",
            -1,
            RoundingMode::Exact,
            "2147483648",
            Ordering::Equal,
        );

        test(
            "4294967297",
            -1,
            RoundingMode::Down,
            "2147483648",
            Ordering::Less,
        );
        test(
            "4294967297",
            -1,
            RoundingMode::Up,
            "2147483649",
            Ordering::Greater,
        );
        test(
            "4294967297",
            -1,
            RoundingMode::Floor,
            "2147483648",
            Ordering::Less,
        );
        test(
            "4294967297",
            -1,
            RoundingMode::Ceiling,
            "2147483649",
            Ordering::Greater,
        );
        test(
            "4294967297",
            -1,
            RoundingMode::Nearest,
            "2147483648",
            Ordering::Less,
        );

        test(
            "1000000000000",
            0,
            RoundingMode::Down,
            "1000000000000",
            Ordering::Equal,
        );
        test(
            "1000000000000",
            0,
            RoundingMode::Up,
            "1000000000000",
            Ordering::Equal,
        );
        test(
            "1000000000000",
            0,
            RoundingMode::Floor,
            "1000000000000",
            Ordering::Equal,
        );
        test(
            "1000000000000",
            0,
            RoundingMode::Ceiling,
            "1000000000000",
            Ordering::Equal,
        );
        test(
            "1000000000000",
            0,
            RoundingMode::Nearest,
            "1000000000000",
            Ordering::Equal,
        );
        test(
            "1000000000000",
            0,
            RoundingMode::Exact,
            "1000000000000",
            Ordering::Equal,
        );

        test(
            "7999999999999",
            -3,
            RoundingMode::Down,
            "999999999999",
            Ordering::Less,
        );
        test(
            "7999999999999",
            -3,
            RoundingMode::Up,
            "1000000000000",
            Ordering::Greater,
        );
        test(
            "7999999999999",
            -3,
            RoundingMode::Floor,
            "999999999999",
            Ordering::Less,
        );
        test(
            "7999999999999",
            -3,
            RoundingMode::Ceiling,
            "1000000000000",
            Ordering::Greater,
        );
        test(
            "7999999999999",
            -3,
            RoundingMode::Nearest,
            "1000000000000",
            Ordering::Greater,
        );

        test(
            "8000000000000",
            -3,
            RoundingMode::Down,
            "1000000000000",
            Ordering::Equal,
        );
        test(
            "8000000000000",
            -3,
            RoundingMode::Up,
            "1000000000000",
            Ordering::Equal,
        );
        test(
            "8000000000000",
            -3,
            RoundingMode::Floor,
            "1000000000000",
            Ordering::Equal,
        );
        test(
            "8000000000000",
            -3,
            RoundingMode::Ceiling,
            "1000000000000",
            Ordering::Equal,
        );
        test(
            "8000000000000",
            -3,
            RoundingMode::Nearest,
            "1000000000000",
            Ordering::Equal,
        );
        test(
            "8000000000000",
            -3,
            RoundingMode::Exact,
            "1000000000000",
            Ordering::Equal,
        );

        test(
            "8000000000001",
            -3,
            RoundingMode::Down,
            "1000000000000",
            Ordering::Less,
        );
        test(
            "8000000000001",
            -3,
            RoundingMode::Up,
            "1000000000001",
            Ordering::Greater,
        );
        test(
            "8000000000001",
            -3,
            RoundingMode::Floor,
            "1000000000000",
            Ordering::Less,
        );
        test(
            "8000000000001",
            -3,
            RoundingMode::Ceiling,
            "1000000000001",
            Ordering::Greater,
        );
        test(
            "8000000000001",
            -3,
            RoundingMode::Nearest,
            "1000000000000",
            Ordering::Less,
        );

        test(
            "16777216000000000000",
            -24,
            RoundingMode::Down,
            "1000000000000",
            Ordering::Equal,
        );
        test(
            "16777216000000000000",
            -24,
            RoundingMode::Up,
            "1000000000000",
            Ordering::Equal,
        );
        test(
            "16777216000000000000",
            -24,
            RoundingMode::Floor,
            "1000000000000",
            Ordering::Equal,
        );
        test(
            "16777216000000000000",
            -24,
            RoundingMode::Ceiling,
            "1000000000000",
            Ordering::Equal,
        );
        test(
            "16777216000000000000",
            -24,
            RoundingMode::Nearest,
            "1000000000000",
            Ordering::Equal,
        );
        test(
            "16777216000000000000",
            -24,
            RoundingMode::Exact,
            "1000000000000",
            Ordering::Equal,
        );

        test(
            "33554432000000000000",
            -25,
            RoundingMode::Down,
            "1000000000000",
            Ordering::Equal,
        );
        test(
            "33554432000000000000",
            -25,
            RoundingMode::Up,
            "1000000000000",
            Ordering::Equal,
        );
        test(
            "33554432000000000000",
            -25,
            RoundingMode::Floor,
            "1000000000000",
            Ordering::Equal,
        );
        test(
            "33554432000000000000",
            -25,
            RoundingMode::Ceiling,
            "1000000000000",
            Ordering::Equal,
        );
        test(
            "33554432000000000000",
            -25,
            RoundingMode::Nearest,
            "1000000000000",
            Ordering::Equal,
        );
        test(
            "33554432000000000000",
            -25,
            RoundingMode::Exact,
            "1000000000000",
            Ordering::Equal,
        );

        test(
            "2147483648000000000000",
            -31,
            RoundingMode::Down,
            "1000000000000",
            Ordering::Equal,
        );
        test(
            "2147483648000000000000",
            -31,
            RoundingMode::Up,
            "1000000000000",
            Ordering::Equal,
        );
        test(
            "2147483648000000000000",
            -31,
            RoundingMode::Floor,
            "1000000000000",
            Ordering::Equal,
        );
        test(
            "2147483648000000000000",
            -31,
            RoundingMode::Ceiling,
            "1000000000000",
            Ordering::Equal,
        );
        test(
            "2147483648000000000000",
            -31,
            RoundingMode::Nearest,
            "1000000000000",
            Ordering::Equal,
        );
        test(
            "2147483648000000000000",
            -31,
            RoundingMode::Exact,
            "1000000000000",
            Ordering::Equal,
        );

        test(
            "4294967296000000000000",
            -32,
            RoundingMode::Down,
            "1000000000000",
            Ordering::Equal,
        );
        test(
            "4294967296000000000000",
            -32,
            RoundingMode::Up,
            "1000000000000",
            Ordering::Equal,
        );
        test(
            "4294967296000000000000",
            -32,
            RoundingMode::Floor,
            "1000000000000",
            Ordering::Equal,
        );
        test(
            "4294967296000000000000",
            -32,
            RoundingMode::Ceiling,
            "1000000000000",
            Ordering::Equal,
        );
        test(
            "4294967296000000000000",
            -32,
            RoundingMode::Nearest,
            "1000000000000",
            Ordering::Equal,
        );
        test(
            "4294967296000000000000",
            -32,
            RoundingMode::Exact,
            "1000000000000",
            Ordering::Equal,
        );

        test(
            "8589934592000000000000",
            -33,
            RoundingMode::Down,
            "1000000000000",
            Ordering::Equal,
        );
        test(
            "8589934592000000000000",
            -33,
            RoundingMode::Up,
            "1000000000000",
            Ordering::Equal,
        );
        test(
            "8589934592000000000000",
            -33,
            RoundingMode::Floor,
            "1000000000000",
            Ordering::Equal,
        );
        test(
            "8589934592000000000000",
            -33,
            RoundingMode::Ceiling,
            "1000000000000",
            Ordering::Equal,
        );
        test(
            "8589934592000000000000",
            -33,
            RoundingMode::Nearest,
            "1000000000000",
            Ordering::Equal,
        );
        test(
            "8589934592000000000000",
            -33,
            RoundingMode::Exact,
            "1000000000000",
            Ordering::Equal,
        );

        test(
            "1267650600228229401496703205376000000000000",
            -100,
            RoundingMode::Down,
            "1000000000000",
            Ordering::Equal,
        );
        test(
            "1267650600228229401496703205376000000000000",
            -100,
            RoundingMode::Up,
            "1000000000000",
            Ordering::Equal,
        );
        test(
            "1267650600228229401496703205376000000000000",
            -100,
            RoundingMode::Floor,
            "1000000000000",
            Ordering::Equal,
        );
        test(
            "1267650600228229401496703205376000000000000",
            -100,
            RoundingMode::Ceiling,
            "1000000000000",
            Ordering::Equal,
        );
        test(
            "1267650600228229401496703205376000000000000",
            -100,
            RoundingMode::Nearest,
            "1000000000000",
            Ordering::Equal,
        );
        test(
            "1267650600228229401496703205376000000000000",
            -100,
            RoundingMode::Exact,
            "1000000000000",
            Ordering::Equal,
        );

        test(
            "1000000000000",
            -10,
            RoundingMode::Down,
            "976562500",
            Ordering::Equal,
        );
        test(
            "1000000000000",
            -10,
            RoundingMode::Up,
            "976562500",
            Ordering::Equal,
        );
        test(
            "1000000000000",
            -10,
            RoundingMode::Floor,
            "976562500",
            Ordering::Equal,
        );
        test(
            "1000000000000",
            -10,
            RoundingMode::Ceiling,
            "976562500",
            Ordering::Equal,
        );
        test(
            "1000000000000",
            -10,
            RoundingMode::Nearest,
            "976562500",
            Ordering::Equal,
        );
        test(
            "1000000000000",
            -10,
            RoundingMode::Exact,
            "976562500",
            Ordering::Equal,
        );

        test("980657949", -72, RoundingMode::Down, "0", Ordering::Less);
        test("980657949", -72, RoundingMode::Up, "1", Ordering::Greater);
        test("980657949", -72, RoundingMode::Floor, "0", Ordering::Less);
        test(
            "980657949",
            -72,
            RoundingMode::Ceiling,
            "1",
            Ordering::Greater,
        );
        test("980657949", -72, RoundingMode::Nearest, "0", Ordering::Less);

        test("4294967295", -31, RoundingMode::Down, "1", Ordering::Less);
        test("4294967295", -31, RoundingMode::Up, "2", Ordering::Greater);
        test("4294967295", -31, RoundingMode::Floor, "1", Ordering::Less);
        test(
            "4294967295",
            -31,
            RoundingMode::Ceiling,
            "2",
            Ordering::Greater,
        );
        test(
            "4294967295",
            -31,
            RoundingMode::Nearest,
            "2",
            Ordering::Greater,
        );

        test("4294967295", -32, RoundingMode::Down, "0", Ordering::Less);
        test("4294967295", -32, RoundingMode::Up, "1", Ordering::Greater);
        test("4294967295", -32, RoundingMode::Floor, "0", Ordering::Less);
        test(
            "4294967295",
            -32,
            RoundingMode::Ceiling,
            "1",
            Ordering::Greater,
        );
        test(
            "4294967295",
            -32,
            RoundingMode::Nearest,
            "1",
            Ordering::Greater,
        );

        test("4294967296", -32, RoundingMode::Down, "1", Ordering::Equal);
        test("4294967296", -32, RoundingMode::Up, "1", Ordering::Equal);
        test("4294967296", -32, RoundingMode::Floor, "1", Ordering::Equal);
        test(
            "4294967296",
            -32,
            RoundingMode::Ceiling,
            "1",
            Ordering::Equal,
        );
        test(
            "4294967296",
            -32,
            RoundingMode::Nearest,
            "1",
            Ordering::Equal,
        );
        test("4294967296", -32, RoundingMode::Exact, "1", Ordering::Equal);

        test("4294967296", -33, RoundingMode::Down, "0", Ordering::Less);
        test("4294967296", -33, RoundingMode::Up, "1", Ordering::Greater);
        test("4294967296", -33, RoundingMode::Floor, "0", Ordering::Less);
        test(
            "4294967296",
            -33,
            RoundingMode::Ceiling,
            "1",
            Ordering::Greater,
        );
        test(
            "4294967296",
            -33,
            RoundingMode::Nearest,
            "0",
            Ordering::Less,
        );

        test("0", 10, RoundingMode::Exact, "0", Ordering::Equal);
        test("123", 1, RoundingMode::Exact, "246", Ordering::Equal);
        test("123", 2, RoundingMode::Exact, "492", Ordering::Equal);
        test(
            "123",
            25,
            RoundingMode::Exact,
            "4127195136",
            Ordering::Equal,
        );
        test(
            "123",
            26,
            RoundingMode::Exact,
            "8254390272",
            Ordering::Equal,
        );
        test(
            "123",
            100,
            RoundingMode::Exact,
            "155921023828072216384094494261248",
            Ordering::Equal,
        );
        test(
            "2147483648",
            1,
            RoundingMode::Exact,
            "4294967296",
            Ordering::Equal,
        );
        test(
            "1000000000000",
            3,
            RoundingMode::Exact,
            "8000000000000",
            Ordering::Equal,
        );
        test(
            "1000000000000",
            24,
            RoundingMode::Exact,
            "16777216000000000000",
            Ordering::Equal,
        );
        test(
            "1000000000000",
            25,
            RoundingMode::Exact,
            "33554432000000000000",
            Ordering::Equal,
        );
        test(
            "1000000000000",
            31,
            RoundingMode::Exact,
            "2147483648000000000000",
            Ordering::Equal,
        );
        test(
            "1000000000000",
            32,
            RoundingMode::Exact,
            "4294967296000000000000",
            Ordering::Equal,
        );
        test(
            "1000000000000",
            33,
            RoundingMode::Exact,
            "8589934592000000000000",
            Ordering::Equal,
        );
        test(
            "1000000000000",
            100,
            RoundingMode::Exact,
            "1267650600228229401496703205376000000000000",
            Ordering::Equal,
        );
    };
}

#[test]
fn test_shl_round_signed() {
    apply_to_signeds!(test_shl_round_signed_helper);
}

macro_rules! shl_round_signed_fail_helper {
    ($t:ident) => {
        assert_panic!(
            Natural::from(123u32).shl_round_assign($t::NEGATIVE_ONE, RoundingMode::Exact)
        );
        assert_panic!(
            Natural::from(123u32).shl_round_assign($t::exact_from(-100), RoundingMode::Exact)
        );
        assert_panic!(Natural::from_str("1000000000001")
            .unwrap()
            .shl_round_assign($t::NEGATIVE_ONE, RoundingMode::Exact));
        assert_panic!(Natural::from_str("1000000000001")
            .unwrap()
            .shl_round_assign($t::exact_from(-100), RoundingMode::Exact));
        assert_panic!(Natural::from(123u32).shl_round($t::NEGATIVE_ONE, RoundingMode::Exact));
        assert_panic!(Natural::from(123u32).shl_round($t::exact_from(-100), RoundingMode::Exact));
        assert_panic!(Natural::from_str("1000000000001")
            .unwrap()
            .shl_round($t::NEGATIVE_ONE, RoundingMode::Exact));
        assert_panic!(Natural::from_str("1000000000001")
            .unwrap()
            .shl_round($t::exact_from(-100), RoundingMode::Exact));
        assert_panic!((&Natural::from(123u32)).shl_round($t::NEGATIVE_ONE, RoundingMode::Exact));
        assert_panic!(
            (&Natural::from(123u32)).shl_round($t::exact_from(-100), RoundingMode::Exact)
        );
        assert_panic!((&Natural::from_str("1000000000001").unwrap())
            .shl_round($t::NEGATIVE_ONE, RoundingMode::Exact));
        assert_panic!((&Natural::from_str("1000000000001").unwrap())
            .shl_round($t::exact_from(-100), RoundingMode::Exact));
    };
}

#[test]
fn shl_round_signed_fail() {
    apply_to_signeds!(shl_round_signed_fail_helper);
}

fn properties_helper<T: PrimitiveSigned>()
where
    Natural: Shr<T, Output = Natural> + ShlRound<T, Output = Natural> + ShlRoundAssign<T>,
    for<'a> &'a Natural: ShlRound<T, Output = Natural>,
    Limb: ArithmeticCheckedShl<T, Output = Limb> + ShlRound<T, Output = Limb>,
{
    natural_signed_rounding_mode_triple_gen_var_1::<T>().test_properties(|(n, i, rm)| {
        let mut mut_n = n.clone();
        let o = mut_n.shl_round_assign(i, rm);
        assert!(mut_n.is_valid());
        let shifted = mut_n;

        let (shifted_alt, o_alt) = (&n).shl_round(i, rm);
        assert!(shifted_alt.is_valid());
        assert_eq!(shifted_alt, shifted);
        assert_eq!(o_alt, o);

        let (shifted_alt, o_alt) = n.clone().shl_round(i, rm);
        assert!(shifted_alt.is_valid());
        assert_eq!(shifted_alt, shifted);
        assert_eq!(o_alt, o);

        if i < T::ZERO {
            assert_eq!((shifted >> i).cmp(&n), o);
        }
        match rm {
            RoundingMode::Floor | RoundingMode::Down => assert_ne!(o, Ordering::Greater),
            RoundingMode::Ceiling | RoundingMode::Up => assert_ne!(o, Ordering::Less),
            RoundingMode::Exact => assert_eq!(o, Ordering::Equal),
            _ => {}
        }
    });

    natural_rounding_mode_pair_gen().test_properties(|(n, rm)| {
        assert_eq!((&n).shl_round(T::ZERO, rm), (n, Ordering::Equal));
    });

    signed_rounding_mode_pair_gen::<T>().test_properties(|(i, rm)| {
        assert_eq!(
            Natural::ZERO.shl_round(i, rm),
            (Natural::ZERO, Ordering::Equal)
        );
    });

    unsigned_signed_rounding_mode_triple_gen_var_2::<Limb, T>().test_properties(|(n, i, rm)| {
        if n.arithmetic_checked_shl(i).is_some() {
            let (s, o) = n.shl_round(i, rm);
            assert_eq!((Natural::from(s), o), Natural::from(n).shl_round(i, rm));
        }
    });
}

#[test]
fn shl_round_properties() {
    apply_fn_to_signeds!(properties_helper);
}
