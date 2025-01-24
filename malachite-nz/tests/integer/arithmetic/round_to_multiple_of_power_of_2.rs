// Copyright © 2025 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::arithmetic::traits::{
    Abs, DivisibleByPowerOf2, PowerOf2, RoundToMultiple, RoundToMultipleOfPowerOf2,
    RoundToMultipleOfPowerOf2Assign, ShrRound,
};
use malachite_base::num::basic::traits::Zero;
use malachite_base::num::comparison::traits::PartialOrdAbs;
use malachite_base::num::logic::traits::BitAccess;
use malachite_base::rounding_modes::RoundingMode::{self, *};
use malachite_base::test_util::generators::signed_unsigned_rounding_mode_triple_gen_var_1;
use malachite_base::test_util::generators::unsigned_rounding_mode_pair_gen;
use malachite_nz::integer::Integer;
use malachite_nz::platform::SignedLimb;
use malachite_nz::test_util::generators::{
    integer_rounding_mode_pair_gen, integer_unsigned_pair_gen_var_2,
    integer_unsigned_pair_gen_var_5, integer_unsigned_rounding_mode_triple_gen_var_1,
    natural_unsigned_rounding_mode_triple_gen_var_1,
};
use std::cmp::Ordering::*;
use std::str::FromStr;

#[test]
fn test_round_to_multiple_of_power_of_2() {
    let test = |s, v: u64, rm: RoundingMode, out, o| {
        let u = Integer::from_str(s).unwrap();

        let mut n = u.clone();
        assert_eq!(n.round_to_multiple_of_power_of_2_assign(v, rm), o);
        assert_eq!(n.to_string(), out);
        assert!(n.is_valid());

        let (n, o_alt) = u.clone().round_to_multiple_of_power_of_2(v, rm);
        assert_eq!(n.to_string(), out);
        assert!(n.is_valid());
        assert_eq!(o_alt, o);

        let (n, o_alt) = (&u).round_to_multiple_of_power_of_2(v, rm);
        assert_eq!(n.to_string(), out);
        assert!(n.is_valid());
        assert_eq!(o_alt, o);

        let (n, o_alt) = u.shr_round(v, rm);
        assert_eq!((n << v).to_string(), out);
        assert_eq!(o_alt, o);
    };
    test("0", 0, Down, "0", Equal);
    test("0", 0, Up, "0", Equal);
    test("0", 0, Floor, "0", Equal);
    test("0", 0, Ceiling, "0", Equal);
    test("0", 0, Nearest, "0", Equal);
    test("0", 0, Exact, "0", Equal);

    test("0", 10, Down, "0", Equal);
    test("0", 10, Up, "0", Equal);
    test("0", 10, Floor, "0", Equal);
    test("0", 10, Ceiling, "0", Equal);
    test("0", 10, Nearest, "0", Equal);
    test("0", 10, Exact, "0", Equal);

    test("123", 0, Down, "123", Equal);
    test("123", 0, Up, "123", Equal);
    test("123", 0, Floor, "123", Equal);
    test("123", 0, Ceiling, "123", Equal);
    test("123", 0, Nearest, "123", Equal);
    test("123", 0, Exact, "123", Equal);

    test("245", 1, Down, "244", Less);
    test("245", 1, Up, "246", Greater);
    test("245", 1, Floor, "244", Less);
    test("245", 1, Ceiling, "246", Greater);
    test("245", 1, Nearest, "244", Less);

    test("246", 1, Down, "246", Equal);
    test("246", 1, Up, "246", Equal);
    test("246", 1, Floor, "246", Equal);
    test("246", 1, Ceiling, "246", Equal);
    test("246", 1, Nearest, "246", Equal);
    test("246", 1, Exact, "246", Equal);

    test("247", 1, Down, "246", Less);
    test("247", 1, Up, "248", Greater);
    test("247", 1, Floor, "246", Less);
    test("247", 1, Ceiling, "248", Greater);
    test("247", 1, Nearest, "248", Greater);

    test("491", 2, Down, "488", Less);
    test("491", 2, Up, "492", Greater);
    test("491", 2, Floor, "488", Less);
    test("491", 2, Ceiling, "492", Greater);
    test("491", 2, Nearest, "492", Greater);

    test("492", 2, Down, "492", Equal);
    test("492", 2, Up, "492", Equal);
    test("492", 2, Floor, "492", Equal);
    test("492", 2, Ceiling, "492", Equal);
    test("492", 2, Nearest, "492", Equal);
    test("492", 2, Exact, "492", Equal);

    test("493", 2, Down, "492", Less);
    test("493", 2, Up, "496", Greater);
    test("493", 2, Floor, "492", Less);
    test("493", 2, Ceiling, "496", Greater);
    test("493", 2, Nearest, "492", Less);

    test("4127195135", 25, Down, "4093640704", Less);
    test("4127195135", 25, Up, "4127195136", Greater);
    test("4127195135", 25, Floor, "4093640704", Less);
    test("4127195135", 25, Ceiling, "4127195136", Greater);
    test("4127195135", 25, Nearest, "4127195136", Greater);

    test("4127195136", 25, Down, "4127195136", Equal);
    test("4127195136", 25, Up, "4127195136", Equal);
    test("4127195136", 25, Floor, "4127195136", Equal);
    test("4127195136", 25, Ceiling, "4127195136", Equal);
    test("4127195136", 25, Nearest, "4127195136", Equal);
    test("4127195136", 25, Exact, "4127195136", Equal);

    test("4127195137", 25, Down, "4127195136", Less);
    test("4127195137", 25, Up, "4160749568", Greater);
    test("4127195137", 25, Floor, "4127195136", Less);
    test("4127195137", 25, Ceiling, "4160749568", Greater);
    test("4127195137", 25, Nearest, "4127195136", Less);

    test("8254390271", 26, Down, "8187281408", Less);
    test("8254390271", 26, Up, "8254390272", Greater);
    test("8254390271", 26, Floor, "8187281408", Less);
    test("8254390271", 26, Ceiling, "8254390272", Greater);
    test("8254390271", 26, Nearest, "8254390272", Greater);

    test("8254390272", 26, Down, "8254390272", Equal);
    test("8254390272", 26, Up, "8254390272", Equal);
    test("8254390272", 26, Floor, "8254390272", Equal);
    test("8254390272", 26, Ceiling, "8254390272", Equal);
    test("8254390272", 26, Nearest, "8254390272", Equal);
    test("8254390272", 26, Exact, "8254390272", Equal);

    test("8254390273", 26, Down, "8254390272", Less);
    test("8254390273", 26, Up, "8321499136", Greater);
    test("8254390273", 26, Floor, "8254390272", Less);
    test("8254390273", 26, Ceiling, "8321499136", Greater);
    test("8254390273", 26, Nearest, "8254390272", Less);
    test(
        "155921023828072216384094494261247",
        100,
        Down,
        "154653373227843986982597791055872",
        Less,
    );
    test(
        "155921023828072216384094494261247",
        100,
        Up,
        "155921023828072216384094494261248",
        Greater,
    );
    test(
        "155921023828072216384094494261247",
        100,
        Floor,
        "154653373227843986982597791055872",
        Less,
    );
    test(
        "155921023828072216384094494261247",
        100,
        Ceiling,
        "155921023828072216384094494261248",
        Greater,
    );
    test(
        "155921023828072216384094494261247",
        100,
        Nearest,
        "155921023828072216384094494261248",
        Greater,
    );

    test(
        "155921023828072216384094494261248",
        100,
        Down,
        "155921023828072216384094494261248",
        Equal,
    );
    test(
        "155921023828072216384094494261248",
        100,
        Up,
        "155921023828072216384094494261248",
        Equal,
    );
    test(
        "155921023828072216384094494261248",
        100,
        Floor,
        "155921023828072216384094494261248",
        Equal,
    );
    test(
        "155921023828072216384094494261248",
        100,
        Ceiling,
        "155921023828072216384094494261248",
        Equal,
    );
    test(
        "155921023828072216384094494261248",
        100,
        Nearest,
        "155921023828072216384094494261248",
        Equal,
    );
    test(
        "155921023828072216384094494261248",
        100,
        Exact,
        "155921023828072216384094494261248",
        Equal,
    );

    test(
        "155921023828072216384094494261249",
        100,
        Down,
        "155921023828072216384094494261248",
        Less,
    );
    test(
        "155921023828072216384094494261249",
        100,
        Up,
        "157188674428300445785591197466624",
        Greater,
    );
    test(
        "155921023828072216384094494261249",
        100,
        Floor,
        "155921023828072216384094494261248",
        Less,
    );
    test(
        "155921023828072216384094494261249",
        100,
        Ceiling,
        "157188674428300445785591197466624",
        Greater,
    );
    test(
        "155921023828072216384094494261249",
        100,
        Nearest,
        "155921023828072216384094494261248",
        Less,
    );

    test("4294967295", 1, Down, "4294967294", Less);
    test("4294967295", 1, Up, "4294967296", Greater);
    test("4294967295", 1, Floor, "4294967294", Less);
    test("4294967295", 1, Ceiling, "4294967296", Greater);
    test("4294967295", 1, Nearest, "4294967296", Greater);

    test("4294967296", 1, Down, "4294967296", Equal);
    test("4294967296", 1, Up, "4294967296", Equal);
    test("4294967296", 1, Floor, "4294967296", Equal);
    test("4294967296", 1, Ceiling, "4294967296", Equal);
    test("4294967296", 1, Nearest, "4294967296", Equal);
    test("4294967296", 1, Exact, "4294967296", Equal);

    test("4294967297", 1, Down, "4294967296", Less);
    test("4294967297", 1, Up, "4294967298", Greater);
    test("4294967297", 1, Floor, "4294967296", Less);
    test("4294967297", 1, Ceiling, "4294967298", Greater);
    test("4294967297", 1, Nearest, "4294967296", Less);

    test("1000000000000", 0, Down, "1000000000000", Equal);
    test("1000000000000", 0, Up, "1000000000000", Equal);
    test("1000000000000", 0, Floor, "1000000000000", Equal);
    test("1000000000000", 0, Ceiling, "1000000000000", Equal);
    test("1000000000000", 0, Nearest, "1000000000000", Equal);
    test("1000000000000", 0, Exact, "1000000000000", Equal);

    test("7999999999999", 3, Down, "7999999999992", Less);
    test("7999999999999", 3, Up, "8000000000000", Greater);
    test("7999999999999", 3, Floor, "7999999999992", Less);
    test("7999999999999", 3, Ceiling, "8000000000000", Greater);
    test("7999999999999", 3, Nearest, "8000000000000", Greater);

    test("8000000000000", 3, Down, "8000000000000", Equal);
    test("8000000000000", 3, Up, "8000000000000", Equal);
    test("8000000000000", 3, Floor, "8000000000000", Equal);
    test("8000000000000", 3, Ceiling, "8000000000000", Equal);
    test("8000000000000", 3, Nearest, "8000000000000", Equal);
    test("8000000000000", 3, Exact, "8000000000000", Equal);

    test("8000000000001", 3, Down, "8000000000000", Less);
    test("8000000000001", 3, Up, "8000000000008", Greater);
    test("8000000000001", 3, Floor, "8000000000000", Less);
    test("8000000000001", 3, Ceiling, "8000000000008", Greater);
    test("8000000000001", 3, Nearest, "8000000000000", Less);

    test(
        "16777216000000000000",
        24,
        Down,
        "16777216000000000000",
        Equal,
    );
    test(
        "16777216000000000000",
        24,
        Up,
        "16777216000000000000",
        Equal,
    );
    test(
        "16777216000000000000",
        24,
        Floor,
        "16777216000000000000",
        Equal,
    );
    test(
        "16777216000000000000",
        24,
        Ceiling,
        "16777216000000000000",
        Equal,
    );
    test(
        "16777216000000000000",
        24,
        Nearest,
        "16777216000000000000",
        Equal,
    );
    test(
        "16777216000000000000",
        24,
        Exact,
        "16777216000000000000",
        Equal,
    );

    test(
        "33554432000000000000",
        25,
        Down,
        "33554432000000000000",
        Equal,
    );
    test(
        "33554432000000000000",
        25,
        Up,
        "33554432000000000000",
        Equal,
    );
    test(
        "33554432000000000000",
        25,
        Floor,
        "33554432000000000000",
        Equal,
    );
    test(
        "33554432000000000000",
        25,
        Ceiling,
        "33554432000000000000",
        Equal,
    );
    test(
        "33554432000000000000",
        25,
        Nearest,
        "33554432000000000000",
        Equal,
    );
    test(
        "33554432000000000000",
        25,
        Exact,
        "33554432000000000000",
        Equal,
    );

    test(
        "2147483648000000000000",
        31,
        Down,
        "2147483648000000000000",
        Equal,
    );
    test(
        "2147483648000000000000",
        31,
        Up,
        "2147483648000000000000",
        Equal,
    );
    test(
        "2147483648000000000000",
        31,
        Floor,
        "2147483648000000000000",
        Equal,
    );
    test(
        "2147483648000000000000",
        31,
        Ceiling,
        "2147483648000000000000",
        Equal,
    );
    test(
        "2147483648000000000000",
        31,
        Nearest,
        "2147483648000000000000",
        Equal,
    );
    test(
        "2147483648000000000000",
        31,
        Exact,
        "2147483648000000000000",
        Equal,
    );

    test(
        "4294967296000000000000",
        32,
        Down,
        "4294967296000000000000",
        Equal,
    );
    test(
        "4294967296000000000000",
        32,
        Up,
        "4294967296000000000000",
        Equal,
    );
    test(
        "4294967296000000000000",
        32,
        Floor,
        "4294967296000000000000",
        Equal,
    );
    test(
        "4294967296000000000000",
        32,
        Ceiling,
        "4294967296000000000000",
        Equal,
    );
    test(
        "4294967296000000000000",
        32,
        Nearest,
        "4294967296000000000000",
        Equal,
    );
    test(
        "4294967296000000000000",
        32,
        Exact,
        "4294967296000000000000",
        Equal,
    );

    test(
        "8589934592000000000000",
        33,
        Down,
        "8589934592000000000000",
        Equal,
    );
    test(
        "8589934592000000000000",
        33,
        Up,
        "8589934592000000000000",
        Equal,
    );
    test(
        "8589934592000000000000",
        33,
        Floor,
        "8589934592000000000000",
        Equal,
    );
    test(
        "8589934592000000000000",
        33,
        Ceiling,
        "8589934592000000000000",
        Equal,
    );
    test(
        "8589934592000000000000",
        33,
        Nearest,
        "8589934592000000000000",
        Equal,
    );
    test(
        "8589934592000000000000",
        33,
        Exact,
        "8589934592000000000000",
        Equal,
    );

    test(
        "1267650600228229401496703205376000000000000",
        100,
        Down,
        "1267650600228229401496703205376000000000000",
        Equal,
    );
    test(
        "1267650600228229401496703205376000000000000",
        100,
        Up,
        "1267650600228229401496703205376000000000000",
        Equal,
    );
    test(
        "1267650600228229401496703205376000000000000",
        100,
        Floor,
        "1267650600228229401496703205376000000000000",
        Equal,
    );
    test(
        "1267650600228229401496703205376000000000000",
        100,
        Ceiling,
        "1267650600228229401496703205376000000000000",
        Equal,
    );
    test(
        "1267650600228229401496703205376000000000000",
        100,
        Nearest,
        "1267650600228229401496703205376000000000000",
        Equal,
    );
    test(
        "1267650600228229401496703205376000000000000",
        100,
        Exact,
        "1267650600228229401496703205376000000000000",
        Equal,
    );

    test("1000000000000", 10, Down, "1000000000000", Equal);
    test("1000000000000", 10, Up, "1000000000000", Equal);
    test("1000000000000", 10, Floor, "1000000000000", Equal);
    test("1000000000000", 10, Ceiling, "1000000000000", Equal);
    test("1000000000000", 10, Nearest, "1000000000000", Equal);
    test("1000000000000", 10, Exact, "1000000000000", Equal);

    test("980657949", 72, Down, "0", Less);
    test("980657949", 72, Up, "4722366482869645213696", Greater);
    test("980657949", 72, Floor, "0", Less);
    test("980657949", 72, Ceiling, "4722366482869645213696", Greater);
    test("980657949", 72, Nearest, "0", Less);

    test("4294967295", 31, Down, "2147483648", Less);
    test("4294967295", 31, Up, "4294967296", Greater);
    test("4294967295", 31, Floor, "2147483648", Less);
    test("4294967295", 31, Ceiling, "4294967296", Greater);
    test("4294967295", 31, Nearest, "4294967296", Greater);

    test("4294967295", 32, Down, "0", Less);
    test("4294967295", 32, Up, "4294967296", Greater);
    test("4294967295", 32, Floor, "0", Less);
    test("4294967295", 32, Ceiling, "4294967296", Greater);
    test("4294967295", 32, Nearest, "4294967296", Greater);

    test("4294967296", 32, Down, "4294967296", Equal);
    test("4294967296", 32, Up, "4294967296", Equal);
    test("4294967296", 32, Floor, "4294967296", Equal);
    test("4294967296", 32, Ceiling, "4294967296", Equal);
    test("4294967296", 32, Nearest, "4294967296", Equal);
    test("4294967296", 32, Exact, "4294967296", Equal);

    test("4294967296", 33, Down, "0", Less);
    test("4294967296", 33, Up, "8589934592", Greater);
    test("4294967296", 33, Floor, "0", Less);
    test("4294967296", 33, Ceiling, "8589934592", Greater);
    test("4294967296", 33, Nearest, "0", Less);

    test("-123", 0, Down, "-123", Equal);
    test("-123", 0, Up, "-123", Equal);
    test("-123", 0, Floor, "-123", Equal);
    test("-123", 0, Ceiling, "-123", Equal);
    test("-123", 0, Nearest, "-123", Equal);
    test("-123", 0, Exact, "-123", Equal);

    test("-245", 1, Down, "-244", Greater);
    test("-245", 1, Up, "-246", Less);
    test("-245", 1, Floor, "-246", Less);
    test("-245", 1, Ceiling, "-244", Greater);
    test("-245", 1, Nearest, "-244", Greater);

    test("-246", 1, Down, "-246", Equal);
    test("-246", 1, Up, "-246", Equal);
    test("-246", 1, Floor, "-246", Equal);
    test("-246", 1, Ceiling, "-246", Equal);
    test("-246", 1, Nearest, "-246", Equal);
    test("-246", 1, Exact, "-246", Equal);

    test("-247", 1, Down, "-246", Greater);
    test("-247", 1, Up, "-248", Less);
    test("-247", 1, Floor, "-248", Less);
    test("-247", 1, Ceiling, "-246", Greater);
    test("-247", 1, Nearest, "-248", Less);

    test("-491", 2, Down, "-488", Greater);
    test("-491", 2, Up, "-492", Less);
    test("-491", 2, Floor, "-492", Less);
    test("-491", 2, Ceiling, "-488", Greater);
    test("-491", 2, Nearest, "-492", Less);

    test("-492", 2, Down, "-492", Equal);
    test("-492", 2, Up, "-492", Equal);
    test("-492", 2, Floor, "-492", Equal);
    test("-492", 2, Ceiling, "-492", Equal);
    test("-492", 2, Nearest, "-492", Equal);
    test("-492", 2, Exact, "-492", Equal);

    test("-493", 2, Down, "-492", Greater);
    test("-493", 2, Up, "-496", Less);
    test("-493", 2, Floor, "-496", Less);
    test("-493", 2, Ceiling, "-492", Greater);
    test("-493", 2, Nearest, "-492", Greater);

    test("-4127195135", 25, Down, "-4093640704", Greater);
    test("-4127195135", 25, Up, "-4127195136", Less);
    test("-4127195135", 25, Floor, "-4127195136", Less);
    test("-4127195135", 25, Ceiling, "-4093640704", Greater);
    test("-4127195135", 25, Nearest, "-4127195136", Less);

    test("-4127195136", 25, Down, "-4127195136", Equal);
    test("-4127195136", 25, Up, "-4127195136", Equal);
    test("-4127195136", 25, Floor, "-4127195136", Equal);
    test("-4127195136", 25, Ceiling, "-4127195136", Equal);
    test("-4127195136", 25, Nearest, "-4127195136", Equal);
    test("-4127195136", 25, Exact, "-4127195136", Equal);

    test("-4127195137", 25, Down, "-4127195136", Greater);
    test("-4127195137", 25, Up, "-4160749568", Less);
    test("-4127195137", 25, Floor, "-4160749568", Less);
    test("-4127195137", 25, Ceiling, "-4127195136", Greater);
    test("-4127195137", 25, Nearest, "-4127195136", Greater);

    test("-8254390271", 26, Down, "-8187281408", Greater);
    test("-8254390271", 26, Up, "-8254390272", Less);
    test("-8254390271", 26, Floor, "-8254390272", Less);
    test("-8254390271", 26, Ceiling, "-8187281408", Greater);
    test("-8254390271", 26, Nearest, "-8254390272", Less);

    test("-8254390272", 26, Down, "-8254390272", Equal);
    test("-8254390272", 26, Up, "-8254390272", Equal);
    test("-8254390272", 26, Floor, "-8254390272", Equal);
    test("-8254390272", 26, Ceiling, "-8254390272", Equal);
    test("-8254390272", 26, Nearest, "-8254390272", Equal);
    test("-8254390272", 26, Exact, "-8254390272", Equal);

    test("-8254390273", 26, Down, "-8254390272", Greater);
    test("-8254390273", 26, Up, "-8321499136", Less);
    test("-8254390273", 26, Floor, "-8321499136", Less);
    test("-8254390273", 26, Ceiling, "-8254390272", Greater);
    test("-8254390273", 26, Nearest, "-8254390272", Greater);

    test(
        "-155921023828072216384094494261247",
        100,
        Down,
        "-154653373227843986982597791055872",
        Greater,
    );
    test(
        "-155921023828072216384094494261247",
        100,
        Up,
        "-155921023828072216384094494261248",
        Less,
    );
    test(
        "-155921023828072216384094494261247",
        100,
        Floor,
        "-155921023828072216384094494261248",
        Less,
    );
    test(
        "-155921023828072216384094494261247",
        100,
        Ceiling,
        "-154653373227843986982597791055872",
        Greater,
    );
    test(
        "-155921023828072216384094494261247",
        100,
        Nearest,
        "-155921023828072216384094494261248",
        Less,
    );

    test(
        "-155921023828072216384094494261248",
        100,
        Down,
        "-155921023828072216384094494261248",
        Equal,
    );
    test(
        "-155921023828072216384094494261248",
        100,
        Up,
        "-155921023828072216384094494261248",
        Equal,
    );
    test(
        "-155921023828072216384094494261248",
        100,
        Floor,
        "-155921023828072216384094494261248",
        Equal,
    );
    test(
        "-155921023828072216384094494261248",
        100,
        Ceiling,
        "-155921023828072216384094494261248",
        Equal,
    );
    test(
        "-155921023828072216384094494261248",
        100,
        Nearest,
        "-155921023828072216384094494261248",
        Equal,
    );
    test(
        "-155921023828072216384094494261248",
        100,
        Exact,
        "-155921023828072216384094494261248",
        Equal,
    );

    test(
        "-155921023828072216384094494261249",
        100,
        Down,
        "-155921023828072216384094494261248",
        Greater,
    );
    test(
        "-155921023828072216384094494261249",
        100,
        Up,
        "-157188674428300445785591197466624",
        Less,
    );
    test(
        "-155921023828072216384094494261249",
        100,
        Floor,
        "-157188674428300445785591197466624",
        Less,
    );
    test(
        "-155921023828072216384094494261249",
        100,
        Ceiling,
        "-155921023828072216384094494261248",
        Greater,
    );
    test(
        "-155921023828072216384094494261249",
        100,
        Nearest,
        "-155921023828072216384094494261248",
        Greater,
    );

    test("-4294967295", 1, Down, "-4294967294", Greater);
    test("-4294967295", 1, Up, "-4294967296", Less);
    test("-4294967295", 1, Floor, "-4294967296", Less);
    test("-4294967295", 1, Ceiling, "-4294967294", Greater);
    test("-4294967295", 1, Nearest, "-4294967296", Less);

    test("-4294967296", 1, Down, "-4294967296", Equal);
    test("-4294967296", 1, Up, "-4294967296", Equal);
    test("-4294967296", 1, Floor, "-4294967296", Equal);
    test("-4294967296", 1, Ceiling, "-4294967296", Equal);
    test("-4294967296", 1, Nearest, "-4294967296", Equal);
    test("-4294967296", 1, Exact, "-4294967296", Equal);

    test("-4294967297", 1, Down, "-4294967296", Greater);
    test("-4294967297", 1, Up, "-4294967298", Less);
    test("-4294967297", 1, Floor, "-4294967298", Less);
    test("-4294967297", 1, Ceiling, "-4294967296", Greater);
    test("-4294967297", 1, Nearest, "-4294967296", Greater);

    test("-1000000000000", 0, Down, "-1000000000000", Equal);
    test("-1000000000000", 0, Up, "-1000000000000", Equal);
    test("-1000000000000", 0, Floor, "-1000000000000", Equal);
    test("-1000000000000", 0, Ceiling, "-1000000000000", Equal);
    test("-1000000000000", 0, Nearest, "-1000000000000", Equal);
    test("-1000000000000", 0, Exact, "-1000000000000", Equal);

    test("-7999999999999", 3, Down, "-7999999999992", Greater);
    test("-7999999999999", 3, Up, "-8000000000000", Less);
    test("-7999999999999", 3, Floor, "-8000000000000", Less);
    test("-7999999999999", 3, Ceiling, "-7999999999992", Greater);
    test("-7999999999999", 3, Nearest, "-8000000000000", Less);

    test("-8000000000000", 3, Down, "-8000000000000", Equal);
    test("-8000000000000", 3, Up, "-8000000000000", Equal);
    test("-8000000000000", 3, Floor, "-8000000000000", Equal);
    test("-8000000000000", 3, Ceiling, "-8000000000000", Equal);
    test("-8000000000000", 3, Nearest, "-8000000000000", Equal);
    test("-8000000000000", 3, Exact, "-8000000000000", Equal);

    test("-8000000000001", 3, Down, "-8000000000000", Greater);
    test("-8000000000001", 3, Up, "-8000000000008", Less);
    test("-8000000000001", 3, Floor, "-8000000000008", Less);
    test("-8000000000001", 3, Ceiling, "-8000000000000", Greater);
    test("-8000000000001", 3, Nearest, "-8000000000000", Greater);

    test(
        "-16777216000000000000",
        24,
        Down,
        "-16777216000000000000",
        Equal,
    );
    test(
        "-16777216000000000000",
        24,
        Up,
        "-16777216000000000000",
        Equal,
    );
    test(
        "-16777216000000000000",
        24,
        Floor,
        "-16777216000000000000",
        Equal,
    );
    test(
        "-16777216000000000000",
        24,
        Ceiling,
        "-16777216000000000000",
        Equal,
    );
    test(
        "-16777216000000000000",
        24,
        Nearest,
        "-16777216000000000000",
        Equal,
    );
    test(
        "-16777216000000000000",
        24,
        Exact,
        "-16777216000000000000",
        Equal,
    );

    test(
        "-33554432000000000000",
        25,
        Down,
        "-33554432000000000000",
        Equal,
    );
    test(
        "-33554432000000000000",
        25,
        Up,
        "-33554432000000000000",
        Equal,
    );
    test(
        "-33554432000000000000",
        25,
        Floor,
        "-33554432000000000000",
        Equal,
    );
    test(
        "-33554432000000000000",
        25,
        Ceiling,
        "-33554432000000000000",
        Equal,
    );
    test(
        "-33554432000000000000",
        25,
        Nearest,
        "-33554432000000000000",
        Equal,
    );
    test(
        "-33554432000000000000",
        25,
        Exact,
        "-33554432000000000000",
        Equal,
    );

    test(
        "-2147483648000000000000",
        31,
        Down,
        "-2147483648000000000000",
        Equal,
    );
    test(
        "-2147483648000000000000",
        31,
        Up,
        "-2147483648000000000000",
        Equal,
    );
    test(
        "-2147483648000000000000",
        31,
        Floor,
        "-2147483648000000000000",
        Equal,
    );
    test(
        "-2147483648000000000000",
        31,
        Ceiling,
        "-2147483648000000000000",
        Equal,
    );
    test(
        "-2147483648000000000000",
        31,
        Nearest,
        "-2147483648000000000000",
        Equal,
    );
    test(
        "-2147483648000000000000",
        31,
        Exact,
        "-2147483648000000000000",
        Equal,
    );

    test(
        "-4294967296000000000000",
        32,
        Down,
        "-4294967296000000000000",
        Equal,
    );
    test(
        "-4294967296000000000000",
        32,
        Up,
        "-4294967296000000000000",
        Equal,
    );
    test(
        "-4294967296000000000000",
        32,
        Floor,
        "-4294967296000000000000",
        Equal,
    );
    test(
        "-4294967296000000000000",
        32,
        Ceiling,
        "-4294967296000000000000",
        Equal,
    );
    test(
        "-4294967296000000000000",
        32,
        Nearest,
        "-4294967296000000000000",
        Equal,
    );
    test(
        "-4294967296000000000000",
        32,
        Exact,
        "-4294967296000000000000",
        Equal,
    );

    test(
        "-8589934592000000000000",
        33,
        Down,
        "-8589934592000000000000",
        Equal,
    );
    test(
        "-8589934592000000000000",
        33,
        Up,
        "-8589934592000000000000",
        Equal,
    );
    test(
        "-8589934592000000000000",
        33,
        Floor,
        "-8589934592000000000000",
        Equal,
    );
    test(
        "-8589934592000000000000",
        33,
        Ceiling,
        "-8589934592000000000000",
        Equal,
    );
    test(
        "-8589934592000000000000",
        33,
        Nearest,
        "-8589934592000000000000",
        Equal,
    );
    test(
        "-8589934592000000000000",
        33,
        Exact,
        "-8589934592000000000000",
        Equal,
    );

    test(
        "-1267650600228229401496703205376000000000000",
        100,
        Down,
        "-1267650600228229401496703205376000000000000",
        Equal,
    );
    test(
        "-1267650600228229401496703205376000000000000",
        100,
        Up,
        "-1267650600228229401496703205376000000000000",
        Equal,
    );
    test(
        "-1267650600228229401496703205376000000000000",
        100,
        Floor,
        "-1267650600228229401496703205376000000000000",
        Equal,
    );
    test(
        "-1267650600228229401496703205376000000000000",
        100,
        Ceiling,
        "-1267650600228229401496703205376000000000000",
        Equal,
    );
    test(
        "-1267650600228229401496703205376000000000000",
        100,
        Nearest,
        "-1267650600228229401496703205376000000000000",
        Equal,
    );
    test(
        "-1267650600228229401496703205376000000000000",
        100,
        Exact,
        "-1267650600228229401496703205376000000000000",
        Equal,
    );

    test("-1000000000000", 10, Down, "-1000000000000", Equal);
    test("-1000000000000", 10, Up, "-1000000000000", Equal);
    test("-1000000000000", 10, Floor, "-1000000000000", Equal);
    test("-1000000000000", 10, Ceiling, "-1000000000000", Equal);
    test("-1000000000000", 10, Nearest, "-1000000000000", Equal);
    test("-1000000000000", 10, Exact, "-1000000000000", Equal);

    test("-980657949", 72, Down, "0", Greater);
    test("-980657949", 72, Up, "-4722366482869645213696", Less);
    test("-980657949", 72, Floor, "-4722366482869645213696", Less);
    test("-980657949", 72, Ceiling, "0", Greater);
    test("-980657949", 72, Nearest, "0", Greater);

    test("-4294967295", 31, Down, "-2147483648", Greater);
    test("-4294967295", 31, Up, "-4294967296", Less);
    test("-4294967295", 31, Floor, "-4294967296", Less);
    test("-4294967295", 31, Ceiling, "-2147483648", Greater);
    test("-4294967295", 31, Nearest, "-4294967296", Less);

    test("-4294967295", 32, Down, "0", Greater);
    test("-4294967295", 32, Up, "-4294967296", Less);
    test("-4294967295", 32, Floor, "-4294967296", Less);
    test("-4294967295", 32, Ceiling, "0", Greater);
    test("-4294967295", 32, Nearest, "-4294967296", Less);

    test("-4294967296", 32, Down, "-4294967296", Equal);
    test("-4294967296", 32, Up, "-4294967296", Equal);
    test("-4294967296", 32, Floor, "-4294967296", Equal);
    test("-4294967296", 32, Ceiling, "-4294967296", Equal);
    test("-4294967296", 32, Nearest, "-4294967296", Equal);
    test("-4294967296", 32, Exact, "-4294967296", Equal);

    test("-4294967296", 33, Down, "0", Greater);
    test("-4294967296", 33, Up, "-8589934592", Less);
    test("-4294967296", 33, Floor, "-8589934592", Less);
    test("-4294967296", 33, Ceiling, "0", Greater);
    test("-4294967296", 33, Nearest, "0", Greater);
}

#[test]
#[should_panic]
fn round_to_multiple_of_power_of_2_assign_fail_1() {
    Integer::from(-123).round_to_multiple_of_power_of_2_assign(1, Exact);
}

#[test]
#[should_panic]
fn round_to_multiple_of_power_of_2_assign_fail_2() {
    Integer::from(-123).round_to_multiple_of_power_of_2_assign(100, Exact);
}

#[test]
#[should_panic]
fn round_to_multiple_of_power_of_2_assign_fail_3() {
    Integer::from_str("-1000000000001")
        .unwrap()
        .round_to_multiple_of_power_of_2_assign(1, Exact);
}

#[test]
#[should_panic]
fn round_to_multiple_of_power_of_2_assign_fail_4() {
    Integer::from_str("-1000000000001")
        .unwrap()
        .round_to_multiple_of_power_of_2_assign(100, Exact);
}

#[test]
#[should_panic]
fn round_to_multiple_of_power_of_2_fail_1() {
    Integer::from(-123).round_to_multiple_of_power_of_2(1, Exact);
}

#[test]
#[should_panic]
fn round_to_multiple_of_power_of_2_fail_2() {
    Integer::from(-123).round_to_multiple_of_power_of_2(100, Exact);
}

#[test]
#[should_panic]
fn round_to_multiple_of_power_of_2_fail_3() {
    Integer::from_str("-1000000000001")
        .unwrap()
        .round_to_multiple_of_power_of_2(1, Exact);
}

#[test]
#[should_panic]
fn round_to_multiple_of_power_of_2_fail_4() {
    Integer::from_str("-1000000000001")
        .unwrap()
        .round_to_multiple_of_power_of_2(100, Exact);
}

#[test]
#[should_panic]
fn round_to_multiple_of_power_of_2_ref_fail_1() {
    (&Integer::from(-123)).round_to_multiple_of_power_of_2(1, Exact);
}

#[test]
#[should_panic]
fn round_to_multiple_of_power_of_2_ref_fail_2() {
    (&Integer::from(-123)).round_to_multiple_of_power_of_2(100, Exact);
}

#[test]
#[should_panic]
fn round_to_multiple_of_power_of_2_ref_fail_3() {
    (&Integer::from_str("-1000000000001").unwrap()).round_to_multiple_of_power_of_2(1, Exact);
}

#[test]
#[should_panic]
fn round_to_multiple_of_power_of_2_ref_fail_4() {
    (&Integer::from_str("-1000000000001").unwrap()).round_to_multiple_of_power_of_2(100, Exact);
}

#[test]
fn round_to_multiple_of_power_of_2_properties() {
    integer_unsigned_rounding_mode_triple_gen_var_1().test_properties(|(n, pow, rm)| {
        let (r, o) = (&n).round_to_multiple_of_power_of_2(pow, rm);
        assert!(r.is_valid());

        let (r_alt, o_alt) = n.clone().round_to_multiple_of_power_of_2(pow, rm);
        assert!(r_alt.is_valid());
        assert_eq!(r_alt, r);
        assert_eq!(o_alt, o);

        let mut mut_n = n.clone();
        assert_eq!(mut_n.round_to_multiple_of_power_of_2_assign(pow, rm), o);
        assert!(mut_n.is_valid());
        assert_eq!(mut_n, r);

        assert!(r.divisible_by_power_of_2(pow));
        assert_eq!(r.cmp(&n), o);
        match (n >= 0, rm) {
            (_, Floor) | (true, Down) | (false, Up) => {
                assert_ne!(o, Greater);
            }
            (_, Ceiling) | (true, Up) | (false, Down) => {
                assert_ne!(o, Less);
            }
            (_, Exact) => assert_eq!(o, Equal),
            _ => {}
        }

        let (r_alt, o_alt) = (&n).shr_round(pow, rm);
        assert_eq!(r_alt << pow, r);
        assert_eq!(o_alt, o);

        let (r_alt, o_alt) = (-&n).round_to_multiple_of_power_of_2(pow, -rm);
        assert_eq!(-r_alt, r);
        assert_eq!(o_alt.reverse(), o);

        assert!((&r - &n).abs() <= Integer::power_of_2(pow));

        let (r_alt, o_alt) = (&n).round_to_multiple(Integer::power_of_2(pow), rm);
        assert_eq!(r_alt, r);
        assert_eq!(o_alt, o);
        match rm {
            Floor => assert!(r <= n),
            Ceiling => assert!(r >= n),
            Down => assert!(r.le_abs(&n)),
            Up => assert!(r.ge_abs(&n)),
            Exact => assert_eq!(r, n),
            Nearest => {
                let k = Integer::power_of_2(pow);
                let closest;
                let second_closest;
                if r <= n {
                    closest = &n - &r;
                    second_closest = &r + k - n;
                } else {
                    closest = &r - &n;
                    second_closest = n + k - &r;
                }
                assert!(closest <= second_closest);
                if closest == second_closest {
                    assert!(!r.get_bit(pow));
                }
            }
        }
    });

    integer_unsigned_pair_gen_var_2().test_properties(|(n, pow)| {
        let shifted: Integer = n << pow;
        let so = (shifted.clone(), Equal);
        assert_eq!((&shifted).round_to_multiple_of_power_of_2(pow, Down), so);
        assert_eq!((&shifted).round_to_multiple_of_power_of_2(pow, Up), so);
        assert_eq!((&shifted).round_to_multiple_of_power_of_2(pow, Floor), so);
        assert_eq!((&shifted).round_to_multiple_of_power_of_2(pow, Ceiling), so);
        assert_eq!((&shifted).round_to_multiple_of_power_of_2(pow, Nearest), so);
        assert_eq!((&shifted).round_to_multiple_of_power_of_2(pow, Exact), so);
    });

    integer_unsigned_pair_gen_var_5().test_properties(|(n, pow)| {
        let floor = (&n).round_to_multiple_of_power_of_2(pow, Floor);
        assert_eq!(floor.1, Less);
        let ceiling = (&floor.0 + Integer::power_of_2(pow), Greater);
        assert_eq!((&n).round_to_multiple_of_power_of_2(pow, Ceiling), ceiling);
        if n >= 0 {
            assert_eq!((&n).round_to_multiple_of_power_of_2(pow, Up), ceiling);
            assert_eq!((&n).round_to_multiple_of_power_of_2(pow, Down), floor);
        } else {
            assert_eq!((&n).round_to_multiple_of_power_of_2(pow, Up), floor);
            assert_eq!((&n).round_to_multiple_of_power_of_2(pow, Down), ceiling);
        }
        let nearest = n.round_to_multiple_of_power_of_2(pow, Nearest);
        assert!(nearest == ceiling || nearest == floor);
    });

    integer_rounding_mode_pair_gen().test_properties(|(n, rm)| {
        assert_eq!((&n).round_to_multiple_of_power_of_2(0, rm), (n, Equal));
    });

    unsigned_rounding_mode_pair_gen().test_properties(|(pow, rm)| {
        assert_eq!(
            Integer::ZERO.round_to_multiple_of_power_of_2(pow, rm),
            (Integer::ZERO, Equal)
        );
    });

    natural_unsigned_rounding_mode_triple_gen_var_1().test_properties(|(n, pow, rm)| {
        let (r, o) = Integer::from(&n).round_to_multiple_of_power_of_2(pow, rm);
        let (r_alt, o_alt) = (&n).round_to_multiple_of_power_of_2(pow, rm);
        assert_eq!(r_alt, r);
        assert_eq!(o_alt, o);
    });

    signed_unsigned_rounding_mode_triple_gen_var_1::<SignedLimb>().test_properties(
        |(n, pow, rm)| {
            let (r, o) = Integer::from(n).round_to_multiple_of_power_of_2(pow, rm);
            let (r_alt, o_alt) = n.round_to_multiple_of_power_of_2(pow, rm);
            assert_eq!(r_alt, r);
            assert_eq!(o_alt, o);
        },
    );
}
