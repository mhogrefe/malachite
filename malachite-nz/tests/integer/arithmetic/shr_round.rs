use std::str::FromStr;

use malachite_base::num::arithmetic::traits::{ShrRound, ShrRoundAssign};
use malachite_base::num::basic::traits::One;
use malachite_base::num::conversion::traits::ExactFrom;
use malachite_base::rounding_modes::RoundingMode;

use malachite_nz::integer::Integer;

//TODO clean

macro_rules! tests_unsigned {
    (
        $t:ident,
        $test_shr_round_u:ident,
        $shr_round_assign_u_fail_1:ident,
        $shr_round_assign_u_fail_2:ident,
        $shr_round_assign_u_fail_3:ident,
        $shr_round_assign_u_fail_4:ident,
        $shr_round_u_fail_1:ident,
        $shr_round_u_fail_2:ident,
        $shr_round_u_fail_3:ident,
        $shr_round_u_fail_4:ident,
        $shr_round_u_ref_fail_1:ident,
        $shr_round_u_ref_fail_2:ident,
        $shr_round_u_ref_fail_3:ident,
        $shr_round_u_ref_fail_4:ident
    ) => {
        #[test]
        fn $test_shr_round_u() {
            let test = |u, v: $t, rm: RoundingMode, out| {
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
        #[should_panic]
        fn $shr_round_assign_u_fail_1() {
            Integer::from(-123).shr_round_assign($t::ONE, RoundingMode::Exact);
        }

        #[test]
        #[should_panic]
        fn $shr_round_assign_u_fail_2() {
            Integer::from(-123).shr_round_assign($t::exact_from(100), RoundingMode::Exact);
        }

        #[test]
        #[should_panic]
        fn $shr_round_assign_u_fail_3() {
            Integer::from_str("-1000000000001")
                .unwrap()
                .shr_round_assign($t::ONE, RoundingMode::Exact);
        }

        #[test]
        #[should_panic]
        fn $shr_round_assign_u_fail_4() {
            Integer::from_str("-1000000000001")
                .unwrap()
                .shr_round_assign($t::exact_from(100), RoundingMode::Exact);
        }

        #[test]
        #[should_panic]
        fn $shr_round_u_fail_1() {
            Integer::from(-123).shr_round($t::ONE, RoundingMode::Exact);
        }

        #[test]
        #[should_panic]
        fn $shr_round_u_fail_2() {
            Integer::from(-123).shr_round($t::exact_from(100), RoundingMode::Exact);
        }

        #[test]
        #[should_panic]
        fn $shr_round_u_fail_3() {
            Integer::from_str("-1000000000001")
                .unwrap()
                .shr_round($t::ONE, RoundingMode::Exact);
        }

        #[test]
        #[should_panic]
        fn $shr_round_u_fail_4() {
            Integer::from_str("-1000000000001")
                .unwrap()
                .shr_round($t::exact_from(100), RoundingMode::Exact);
        }

        #[test]
        #[should_panic]
        fn $shr_round_u_ref_fail_1() {
            (&Integer::from(-123)).shr_round($t::ONE, RoundingMode::Exact);
        }

        #[test]
        #[should_panic]
        fn $shr_round_u_ref_fail_2() {
            (&Integer::from(-123)).shr_round($t::exact_from(100), RoundingMode::Exact);
        }

        #[test]
        #[should_panic]
        fn $shr_round_u_ref_fail_3() {
            (&Integer::from_str("-1000000000001").unwrap()).shr_round($t::ONE, RoundingMode::Exact);
        }

        #[test]
        #[should_panic]
        fn $shr_round_u_ref_fail_4() {
            (&Integer::from_str("-1000000000001").unwrap())
                .shr_round($t::exact_from(100), RoundingMode::Exact);
        }
    };
}
tests_unsigned!(
    u8,
    test_shr_round_u8,
    shr_round_assign_u8_fail_1,
    shr_round_assign_u8_fail_2,
    shr_round_assign_u8_fail_3,
    shr_round_assign_u8_fail_4,
    shr_round_u8_fail_1,
    shr_round_u8_fail_2,
    shr_round_u8_fail_3,
    shr_round_u8_fail_4,
    shr_round_u8_ref_fail_1,
    shr_round_u8_ref_fail_2,
    shr_round_u8_ref_fail_3,
    shr_round_u8_ref_fail_4
);
tests_unsigned!(
    u16,
    test_shr_round_u16,
    shr_round_assign_u16_fail_1,
    shr_round_assign_u16_fail_2,
    shr_round_assign_u16_fail_3,
    shr_round_assign_u16_fail_4,
    shr_round_u16_fail_1,
    shr_round_u16_fail_2,
    shr_round_u16_fail_3,
    shr_round_u16_fail_4,
    shr_round_u16_ref_fail_1,
    shr_round_u16_ref_fail_2,
    shr_round_u16_ref_fail_3,
    shr_round_u16_ref_fail_4
);
tests_unsigned!(
    u32,
    shr_u32_properties,
    shr_round_assign_u32_fail_1,
    shr_round_assign_u32_fail_2,
    shr_round_assign_u32_fail_3,
    shr_round_assign_u32_fail_4,
    shr_round_u32_fail_1,
    shr_round_u32_fail_2,
    shr_round_u32_fail_3,
    shr_round_u32_fail_4,
    shr_round_u32_ref_fail_1,
    shr_round_u32_ref_fail_2,
    shr_round_u32_ref_fail_3,
    shr_round_u32_ref_fail_4
);
tests_unsigned!(
    u64,
    test_shr_round_u64,
    shr_round_assign_u64_fail_1,
    shr_round_assign_u64_fail_2,
    shr_round_assign_u64_fail_3,
    shr_round_assign_u64_fail_4,
    shr_round_u64_fail_1,
    shr_round_u64_fail_2,
    shr_round_u64_fail_3,
    shr_round_u64_fail_4,
    shr_round_u64_ref_fail_1,
    shr_round_u64_ref_fail_2,
    shr_round_u64_ref_fail_3,
    shr_round_u64_ref_fail_4
);
tests_unsigned!(
    u128,
    test_shr_round_u128,
    shr_round_assign_u128_fail_1,
    shr_round_assign_u128_fail_2,
    shr_round_assign_u128_fail_3,
    shr_round_assign_u128_fail_4,
    shr_round_u128_fail_1,
    shr_round_u128_fail_2,
    shr_round_u128_fail_3,
    shr_round_u128_fail_4,
    shr_round_u128_ref_fail_1,
    shr_round_u128_ref_fail_2,
    shr_round_u128_ref_fail_3,
    shr_round_u128_ref_fail_4
);
tests_unsigned!(
    usize,
    test_shr_round_usize,
    shr_round_assign_usize_fail_1,
    shr_round_assign_usize_fail_2,
    shr_round_assign_usize_fail_3,
    shr_round_assign_usize_fail_4,
    shr_round_usize_fail_1,
    shr_round_usize_fail_2,
    shr_round_usize_fail_3,
    shr_round_usize_fail_4,
    shr_round_usize_ref_fail_1,
    shr_round_usize_ref_fail_2,
    shr_round_usize_ref_fail_3,
    shr_round_usize_ref_fail_4
);

macro_rules! tests_signed {
    (
        $t:ident,
        $test_shr_round_i:ident,
        $shr_round_assign_i_fail_1:ident,
        $shr_round_assign_i_fail_2:ident,
        $shr_round_assign_i_fail_3:ident,
        $shr_round_assign_i_fail_4:ident,
        $shr_round_i_fail_1:ident,
        $shr_round_i_fail_2:ident,
        $shr_round_i_fail_3:ident,
        $shr_round_i_fail_4:ident,
        $shr_round_i_ref_fail_1:ident,
        $shr_round_i_ref_fail_2:ident,
        $shr_round_i_ref_fail_3:ident,
        $shr_round_i_ref_fail_4:ident
    ) => {
        #[test]
        fn $test_shr_round_i() {
            let test = |i, j: $t, rm: RoundingMode, out| {
                let mut n = Integer::from_str(i).unwrap();
                n.shr_round_assign(j, rm);
                assert_eq!(n.to_string(), out);
                assert!(n.is_valid());

                let n = Integer::from_str(i).unwrap().shr_round(j, rm);
                assert_eq!(n.to_string(), out);
                assert!(n.is_valid());

                let n = &Integer::from_str(i).unwrap().shr_round(j, rm);
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

            test("0", -10, RoundingMode::Exact, "0");
            test("123", -1, RoundingMode::Exact, "246");
            test("123", -2, RoundingMode::Exact, "492");
            test("123", -25, RoundingMode::Exact, "4127195136");
            test("123", -26, RoundingMode::Exact, "8254390272");
            test(
                "123",
                -100,
                RoundingMode::Exact,
                "155921023828072216384094494261248",
            );
            test("2147483648", -1, RoundingMode::Exact, "4294967296");
            test("1000000000000", -3, RoundingMode::Exact, "8000000000000");
            test(
                "1000000000000",
                -24,
                RoundingMode::Exact,
                "16777216000000000000",
            );
            test(
                "1000000000000",
                -25,
                RoundingMode::Exact,
                "33554432000000000000",
            );
            test(
                "1000000000000",
                -31,
                RoundingMode::Exact,
                "2147483648000000000000",
            );
            test(
                "1000000000000",
                -32,
                RoundingMode::Exact,
                "4294967296000000000000",
            );
            test(
                "1000000000000",
                -33,
                RoundingMode::Exact,
                "8589934592000000000000",
            );
            test(
                "1000000000000",
                -100,
                RoundingMode::Exact,
                "1267650600228229401496703205376000000000000",
            );

            test("-123", -1, RoundingMode::Exact, "-246");
            test("-123", -2, RoundingMode::Exact, "-492");
            test("-123", -25, RoundingMode::Exact, "-4127195136");
            test("-123", -26, RoundingMode::Exact, "-8254390272");
            test(
                "-123",
                -100,
                RoundingMode::Exact,
                "-155921023828072216384094494261248",
            );
            test("-2147483648", -1, RoundingMode::Exact, "-4294967296");
            test("-1000000000000", -3, RoundingMode::Exact, "-8000000000000");
            test(
                "-1000000000000",
                -24,
                RoundingMode::Exact,
                "-16777216000000000000",
            );
            test(
                "-1000000000000",
                -25,
                RoundingMode::Exact,
                "-33554432000000000000",
            );
            test(
                "-1000000000000",
                -31,
                RoundingMode::Exact,
                "-2147483648000000000000",
            );
            test(
                "-1000000000000",
                -32,
                RoundingMode::Exact,
                "-4294967296000000000000",
            );
            test(
                "-1000000000000",
                -33,
                RoundingMode::Exact,
                "-8589934592000000000000",
            );
            test(
                "-1000000000000",
                -100,
                RoundingMode::Exact,
                "-1267650600228229401496703205376000000000000",
            );
        }

        #[test]
        #[should_panic]
        fn $shr_round_assign_i_fail_1() {
            Integer::from(-123).shr_round_assign($t::ONE, RoundingMode::Exact);
        }

        #[test]
        #[should_panic]
        fn $shr_round_assign_i_fail_2() {
            Integer::from(-123).shr_round_assign($t::exact_from(100), RoundingMode::Exact);
        }

        #[test]
        #[should_panic]
        fn $shr_round_assign_i_fail_3() {
            Integer::from_str("-1000000000001")
                .unwrap()
                .shr_round_assign($t::ONE, RoundingMode::Exact);
        }

        #[test]
        #[should_panic]
        fn $shr_round_assign_i_fail_4() {
            Integer::from_str("-1000000000001")
                .unwrap()
                .shr_round_assign($t::exact_from(100), RoundingMode::Exact);
        }

        #[test]
        #[should_panic]
        fn $shr_round_i_fail_1() {
            Integer::from(-123).shr_round($t::ONE, RoundingMode::Exact);
        }

        #[test]
        #[should_panic]
        fn $shr_round_i_fail_2() {
            Integer::from(-123).shr_round($t::exact_from(100), RoundingMode::Exact);
        }

        #[test]
        #[should_panic]
        fn $shr_round_i_fail_3() {
            Integer::from_str("-1000000000001")
                .unwrap()
                .shr_round($t::ONE, RoundingMode::Exact);
        }

        #[test]
        #[should_panic]
        fn $shr_round_i_fail_4() {
            Integer::from_str("-1000000000001")
                .unwrap()
                .shr_round($t::exact_from(100), RoundingMode::Exact);
        }

        #[test]
        #[should_panic]
        fn $shr_round_i_ref_fail_1() {
            (&Integer::from(-123)).shr_round($t::ONE, RoundingMode::Exact);
        }

        #[test]
        #[should_panic]
        fn $shr_round_i_ref_fail_2() {
            (&Integer::from(-123)).shr_round($t::exact_from(100), RoundingMode::Exact);
        }

        #[test]
        #[should_panic]
        fn $shr_round_i_ref_fail_3() {
            (&Integer::from_str("-1000000000001").unwrap()).shr_round($t::ONE, RoundingMode::Exact);
        }

        #[test]
        #[should_panic]
        fn $shr_round_i_ref_fail_4() {
            (&Integer::from_str("-1000000000001").unwrap())
                .shr_round($t::exact_from(100), RoundingMode::Exact);
        }
    };
}
tests_signed!(
    i8,
    test_shr_round_i8,
    shr_round_assign_i8_fail_1,
    shr_round_assign_i8_fail_2,
    shr_round_assign_i8_fail_3,
    shr_round_assign_i8_fail_4,
    shr_round_i8_fail_1,
    shr_round_i8_fail_2,
    shr_round_i8_fail_3,
    shr_round_i8_fail_4,
    shr_round_i8_ref_fail_1,
    shr_round_i8_ref_fail_2,
    shr_round_i8_ref_fail_3,
    shr_round_i8_ref_fail_4
);
tests_signed!(
    i16,
    test_shr_round_i16,
    shr_round_assign_i16_fail_1,
    shr_round_assign_i16_fail_2,
    shr_round_assign_i16_fail_3,
    shr_round_assign_i16_fail_4,
    shr_round_i16_fail_1,
    shr_round_i16_fail_2,
    shr_round_i16_fail_3,
    shr_round_i16_fail_4,
    shr_round_i16_ref_fail_1,
    shr_round_i16_ref_fail_2,
    shr_round_i16_ref_fail_3,
    shr_round_i16_ref_fail_4
);
tests_signed!(
    i32,
    test_shr_round_i32,
    shr_round_assign_i32_fail_1,
    shr_round_assign_i32_fail_2,
    shr_round_assign_i32_fail_3,
    shr_round_assign_i32_fail_4,
    shr_round_i32_fail_1,
    shr_round_i32_fail_2,
    shr_round_i32_fail_3,
    shr_round_i32_fail_4,
    shr_round_i32_ref_fail_1,
    shr_round_i32_ref_fail_2,
    shr_round_i32_ref_fail_3,
    shr_round_i32_ref_fail_4
);
tests_signed!(
    i64,
    test_shr_round_i64,
    shr_round_assign_i64_fail_1,
    shr_round_assign_i64_fail_2,
    shr_round_assign_i64_fail_3,
    shr_round_assign_i64_fail_4,
    shr_round_i64_fail_1,
    shr_round_i64_fail_2,
    shr_round_i64_fail_3,
    shr_round_i64_fail_4,
    shr_round_i64_ref_fail_1,
    shr_round_i64_ref_fail_2,
    shr_round_i64_ref_fail_3,
    shr_round_i64_ref_fail_4
);
tests_signed!(
    i128,
    test_shr_round_i128,
    shr_round_assign_i128_fail_1,
    shr_round_assign_i128_fail_2,
    shr_round_assign_i128_fail_3,
    shr_round_assign_i128_fail_4,
    shr_round_i128_fail_1,
    shr_round_i128_fail_2,
    shr_round_i128_fail_3,
    shr_round_i128_fail_4,
    shr_round_i128_ref_fail_1,
    shr_round_i128_ref_fail_2,
    shr_round_i128_ref_fail_3,
    shr_round_i128_ref_fail_4
);
tests_signed!(
    isize,
    test_shr_round_isize,
    shr_round_assign_isize_fail_1,
    shr_round_assign_isize_fail_2,
    shr_round_assign_isize_fail_3,
    shr_round_assign_isize_fail_4,
    shr_round_isize_fail_1,
    shr_round_isize_fail_2,
    shr_round_isize_fail_3,
    shr_round_isize_fail_4,
    shr_round_isize_ref_fail_1,
    shr_round_isize_ref_fail_2,
    shr_round_isize_ref_fail_3,
    shr_round_isize_ref_fail_4
);
