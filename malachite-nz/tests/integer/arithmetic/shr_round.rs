use malachite_base::num::arithmetic::traits::{
    ArithmeticCheckedShr, DivRound, ShrRound, ShrRoundAssign,
};
use malachite_base::num::basic::signeds::PrimitiveSigned;
use malachite_base::num::basic::traits::{One, Zero};
use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base::num::comparison::traits::PartialOrdAbs;
use malachite_base::num::conversion::traits::ExactFrom;
use malachite_base::rounding_modes::RoundingMode;
use malachite_base::test_util::generators::{
    signed_rounding_mode_pair_gen, signed_signed_rounding_mode_triple_gen_var_3,
    signed_unsigned_rounding_mode_triple_gen_var_2, unsigned_rounding_mode_pair_gen,
};
use malachite_nz::integer::Integer;
use malachite_nz::natural::Natural;
use malachite_nz::platform::SignedLimb;
use malachite_nz::test_util::generators::{
    integer_rounding_mode_pair_gen, integer_signed_rounding_mode_triple_gen_var_2,
    integer_unsigned_pair_gen_var_2, integer_unsigned_pair_gen_var_5,
    integer_unsigned_rounding_mode_triple_gen_var_2, natural_signed_rounding_mode_triple_gen_var_2,
    natural_unsigned_rounding_mode_triple_gen_var_1,
};
use std::ops::Shl;
use std::panic::catch_unwind;
use std::str::FromStr;

macro_rules! test_shr_round_unsigned_helper {
    ($t:ident) => {
        let test = |u, v: $t, rm: RoundingMode, out| {
            let u = Integer::from_str(u).unwrap();

            let mut n = u.clone();
            n.shr_round_assign(v, rm);
            assert_eq!(n.to_string(), out);
            assert!(n.is_valid());

            let n = u.clone().shr_round(v, rm);
            assert_eq!(n.to_string(), out);
            assert!(n.is_valid());

            let n = (&u).shr_round(v, rm);
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
    };
}

#[test]
fn test_shr_round_unsigned() {
    apply_to_unsigneds!(test_shr_round_unsigned_helper);
}

macro_rules! shr_round_unsigned_fail_helper {
    ($t:ident) => {
        assert_panic!(Integer::from(-123).shr_round_assign($t::ONE, RoundingMode::Exact));
        assert_panic!(
            Integer::from(-123).shr_round_assign($t::exact_from(100), RoundingMode::Exact)
        );
        assert_panic!(Integer::from_str("-1000000000001")
            .unwrap()
            .shr_round_assign($t::ONE, RoundingMode::Exact));
        assert_panic!(Integer::from_str("-1000000000001")
            .unwrap()
            .shr_round_assign($t::exact_from(100), RoundingMode::Exact));
        assert_panic!(Integer::from(-123).shr_round($t::ONE, RoundingMode::Exact));
        assert_panic!(Integer::from(-123).shr_round($t::exact_from(100), RoundingMode::Exact));
        assert_panic!(Integer::from_str("-1000000000001")
            .unwrap()
            .shr_round($t::ONE, RoundingMode::Exact));
        assert_panic!(Integer::from_str("-1000000000001")
            .unwrap()
            .shr_round($t::exact_from(100), RoundingMode::Exact));
        assert_panic!((&Integer::from(-123)).shr_round($t::ONE, RoundingMode::Exact));
        assert_panic!((&Integer::from(-123)).shr_round($t::exact_from(100), RoundingMode::Exact));
        assert_panic!(
            (&Integer::from_str("-1000000000001").unwrap()).shr_round($t::ONE, RoundingMode::Exact)
        );
        assert_panic!((&Integer::from_str("-1000000000001").unwrap())
            .shr_round($t::exact_from(100), RoundingMode::Exact));
    };
}

#[test]
fn shr_round_unsigned_fail() {
    apply_to_unsigneds!(shr_round_unsigned_fail_helper);
}

macro_rules! test_shr_round_signed_helper {
    ($t:ident) => {
        let test = |i, j: $t, rm: RoundingMode, out| {
            let u = Integer::from_str(i).unwrap();

            let mut n = u.clone();
            n.shr_round_assign(j, rm);
            assert_eq!(n.to_string(), out);
            assert!(n.is_valid());

            let n = u.clone().shr_round(j, rm);
            assert_eq!(n.to_string(), out);
            assert!(n.is_valid());

            let n = (&u).shr_round(j, rm);
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
    };
}

#[test]
fn test_shr_round_signed() {
    apply_to_signeds!(test_shr_round_signed_helper);
}

macro_rules! shr_round_signed_fail_helper {
    ($t:ident) => {
        assert_panic!(Integer::from(-123).shr_round_assign($t::ONE, RoundingMode::Exact));
        assert_panic!(
            Integer::from(-123).shr_round_assign($t::exact_from(100), RoundingMode::Exact)
        );
        assert_panic!(Integer::from_str("-1000000000001")
            .unwrap()
            .shr_round_assign($t::ONE, RoundingMode::Exact));
        assert_panic!(Integer::from_str("-1000000000001")
            .unwrap()
            .shr_round_assign($t::exact_from(100), RoundingMode::Exact));
        assert_panic!(Integer::from(-123).shr_round($t::ONE, RoundingMode::Exact));
        assert_panic!(Integer::from(-123).shr_round($t::exact_from(100), RoundingMode::Exact));
        assert_panic!(Integer::from_str("-1000000000001")
            .unwrap()
            .shr_round($t::ONE, RoundingMode::Exact));
        assert_panic!(Integer::from_str("-1000000000001")
            .unwrap()
            .shr_round($t::exact_from(100), RoundingMode::Exact));
        assert_panic!((&Integer::from(-123)).shr_round($t::ONE, RoundingMode::Exact));
        assert_panic!((&Integer::from(-123)).shr_round($t::exact_from(100), RoundingMode::Exact));
        assert_panic!(
            (&Integer::from_str("-1000000000001").unwrap()).shr_round($t::ONE, RoundingMode::Exact)
        );
        assert_panic!((&Integer::from_str("-1000000000001").unwrap())
            .shr_round($t::exact_from(100), RoundingMode::Exact));
    };
}

#[test]
fn shr_round_signed_fail() {
    apply_to_signeds!(shr_round_signed_fail_helper);
}

fn shr_round_properties_helper_unsigned<T: PrimitiveUnsigned>()
where
    Integer: Shl<T, Output = Integer> + ShrRound<T, Output = Integer> + ShrRoundAssign<T>,
    for<'a> &'a Integer: Shl<T, Output = Integer> + ShrRound<T, Output = Integer>,
    Natural: Shl<T, Output = Natural>,
    for<'a> &'a Natural: ShrRound<T, Output = Natural>,
    SignedLimb: ShrRound<T, Output = SignedLimb>,
{
    integer_unsigned_rounding_mode_triple_gen_var_2::<T>().test_properties(|(n, u, rm)| {
        let mut mut_n = n.clone();
        mut_n.shr_round_assign(u, rm);
        assert!(mut_n.is_valid());
        let shifted = mut_n;

        let shifted_alt = (&n).shr_round(u, rm);
        assert!(shifted_alt.is_valid());
        assert_eq!(shifted_alt, shifted);
        let shifted_alt = n.clone().shr_round(u, rm);
        assert!(shifted_alt.is_valid());
        assert_eq!(shifted_alt, shifted);

        assert!((&n).shr_round(u, rm).le_abs(&n));
        assert_eq!(-(-&n).shr_round(u, -rm), shifted);
        assert_eq!(n.div_round(Integer::ONE << u, rm), shifted);
    });

    integer_unsigned_pair_gen_var_2::<T>().test_properties(|(n, u)| {
        let left_shifted = &n << u;
        assert_eq!((&left_shifted).shr_round(u, RoundingMode::Down), n);
        assert_eq!((&left_shifted).shr_round(u, RoundingMode::Up), n);
        assert_eq!((&left_shifted).shr_round(u, RoundingMode::Floor), n);
        assert_eq!((&left_shifted).shr_round(u, RoundingMode::Ceiling), n);
        assert_eq!((&left_shifted).shr_round(u, RoundingMode::Nearest), n);
        assert_eq!((&left_shifted).shr_round(u, RoundingMode::Exact), n);
    });

    integer_unsigned_pair_gen_var_5::<T>().test_properties(|(n, u)| {
        let floor = (&n).shr_round(u, RoundingMode::Floor);
        let ceiling = &floor + Integer::ONE;
        assert_eq!((&n).shr_round(u, RoundingMode::Ceiling), ceiling);
        if n >= 0 {
            assert_eq!((&n).shr_round(u, RoundingMode::Up), ceiling);
            assert_eq!((&n).shr_round(u, RoundingMode::Down), floor);
        } else {
            assert_eq!((&n).shr_round(u, RoundingMode::Up), floor);
            assert_eq!((&n).shr_round(u, RoundingMode::Down), ceiling);
        }
        let nearest = n.shr_round(u, RoundingMode::Nearest);
        assert!(nearest == floor || nearest == ceiling);
    });

    integer_rounding_mode_pair_gen().test_properties(|(n, rm)| {
        assert_eq!((&n).shr_round(T::ZERO, rm), n);
    });

    unsigned_rounding_mode_pair_gen::<T>().test_properties(|(u, rm)| {
        assert_eq!(Integer::ZERO.shr_round(u, rm), 0);
    });

    natural_unsigned_rounding_mode_triple_gen_var_1::<T>().test_properties(|(n, u, rm)| {
        assert_eq!((&n).shr_round(u, rm), Integer::from(n).shr_round(u, rm));
    });

    signed_unsigned_rounding_mode_triple_gen_var_2::<SignedLimb, T>().test_properties(
        |(n, u, rm)| {
            assert_eq!(n.shr_round(u, rm), Integer::from(n).shr_round(u, rm));
        },
    );
}

fn shr_round_properties_helper_signed<T: PrimitiveSigned>()
where
    Integer: Shl<T, Output = Integer> + ShrRound<T, Output = Integer> + ShrRoundAssign<T>,
    for<'a> &'a Integer: ShrRound<T, Output = Integer>,
    Natural: Shl<T, Output = Natural>,
    for<'a> &'a Natural: ShrRound<T, Output = Natural>,
    SignedLimb: ArithmeticCheckedShr<T, Output = SignedLimb> + ShrRound<T, Output = SignedLimb>,
{
    integer_signed_rounding_mode_triple_gen_var_2::<T>().test_properties(|(n, i, rm)| {
        let mut mut_n = n.clone();
        mut_n.shr_round_assign(i, rm);
        assert!(mut_n.is_valid());
        let shifted = mut_n;

        let shifted_alt = (&n).shr_round(i, rm);
        assert!(shifted_alt.is_valid());
        assert_eq!(shifted_alt, shifted);
        let shifted_alt = n.clone().shr_round(i, rm);
        assert!(shifted_alt.is_valid());
        assert_eq!(shifted_alt, shifted);

        assert_eq!(-(-n).shr_round(i, -rm), shifted);
    });

    integer_rounding_mode_pair_gen().test_properties(|(n, rm)| {
        assert_eq!((&n).shr_round(T::ZERO, rm), n);
    });

    signed_rounding_mode_pair_gen::<T>().test_properties(|(i, rm)| {
        assert_eq!(Integer::ZERO.shr_round(i, rm), 0);
    });

    natural_signed_rounding_mode_triple_gen_var_2::<T>().test_properties(|(n, i, rm)| {
        assert_eq!((&n).shr_round(i, rm), Integer::from(n).shr_round(i, rm));
    });

    signed_signed_rounding_mode_triple_gen_var_3::<SignedLimb, T>().test_properties(
        |(n, i, rm)| {
            if n.arithmetic_checked_shr(i).is_some() {
                assert_eq!(n.shr_round(i, rm), Integer::from(n).shr_round(i, rm));
            }
        },
    );
}

#[test]
fn shr_round_properties() {
    apply_fn_to_unsigneds!(shr_round_properties_helper_unsigned);
    apply_fn_to_signeds!(shr_round_properties_helper_signed);
}
