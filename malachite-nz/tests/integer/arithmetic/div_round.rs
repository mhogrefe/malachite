// Copyright © 2025 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::arithmetic::traits::{DivRound, DivRoundAssign};
use malachite_base::num::basic::integers::PrimitiveInt;
use malachite_base::num::basic::traits::{NegativeOne, One, Zero};
use malachite_base::num::comparison::traits::PartialOrdAbs;
use malachite_base::rounding_modes::RoundingMode::*;
use malachite_base::test_util::generators::common::GenConfig;
use malachite_base::test_util::generators::signed_signed_rounding_mode_triple_gen_var_1;
use malachite_nz::integer::Integer;
use malachite_nz::platform::{Limb, SignedLimb};
use malachite_nz::test_util::generators::{
    integer_integer_rounding_mode_triple_gen_var_1, integer_pair_gen_var_1, integer_pair_gen_var_3,
    integer_rounding_mode_pair_gen, integer_rounding_mode_pair_gen_var_2,
    natural_natural_rounding_mode_triple_gen_var_1,
};
use num::{BigInt, Integer as NumInteger};
use rug::ops::DivRounding;
use std::cmp::Ordering::*;
use std::str::FromStr;

#[test]
fn test_div_round() {
    let test = |s, t, rm, quotient, o| {
        let u = Integer::from_str(s).unwrap();
        let v = Integer::from_str(t).unwrap();

        let mut x = u.clone();
        assert_eq!(x.div_round_assign(v.clone(), rm), o);
        assert_eq!(x.to_string(), quotient);
        assert!(x.is_valid());

        let mut x = u.clone();
        assert_eq!(x.div_round_assign(&v, rm), o);
        assert_eq!(x.to_string(), quotient);
        assert!(x.is_valid());

        let (q, o_alt) = u.clone().div_round(v.clone(), rm);
        assert!(q.is_valid());
        assert_eq!(q.to_string(), quotient);
        assert_eq!(o_alt, o);

        let (q, o_alt) = u.clone().div_round(&v, rm);
        assert!(q.is_valid());
        assert_eq!(q.to_string(), quotient);
        assert_eq!(o_alt, o);

        let (q, o_alt) = (&u).div_round(v.clone(), rm);
        assert!(q.is_valid());
        assert_eq!(q.to_string(), quotient);
        assert_eq!(o_alt, o);

        let (q, o_alt) = (&u).div_round(&v, rm);
        assert!(q.is_valid());
        assert_eq!(q.to_string(), quotient);
        assert_eq!(o_alt, o);
        match rm {
            Down => {
                assert_eq!(
                    rug::Integer::from_str(s)
                        .unwrap()
                        .div_trunc(rug::Integer::from_str(t).unwrap())
                        .to_string(),
                    quotient
                );
            }
            Floor => {
                assert_eq!(
                    BigInt::from_str(s)
                        .unwrap()
                        .div_floor(&BigInt::from_str(t).unwrap())
                        .to_string(),
                    quotient
                );
                assert_eq!(
                    rug::Integer::from_str(s)
                        .unwrap()
                        .div_floor(rug::Integer::from_str(t).unwrap())
                        .to_string(),
                    quotient
                );
            }
            Ceiling => {
                assert_eq!(
                    rug::Integer::from_str(s)
                        .unwrap()
                        .div_ceil(rug::Integer::from_str(t).unwrap())
                        .to_string(),
                    quotient
                );
            }
            _ => {}
        }
    };
    test("0", "1", Down, "0", Equal);
    test("0", "1", Floor, "0", Equal);
    test("0", "1", Up, "0", Equal);
    test("0", "1", Ceiling, "0", Equal);
    test("0", "1", Nearest, "0", Equal);
    test("0", "1", Exact, "0", Equal);

    test("0", "123", Down, "0", Equal);
    test("0", "123", Floor, "0", Equal);
    test("0", "123", Up, "0", Equal);
    test("0", "123", Ceiling, "0", Equal);
    test("0", "123", Nearest, "0", Equal);
    test("0", "123", Exact, "0", Equal);

    test("1", "1", Down, "1", Equal);
    test("1", "1", Floor, "1", Equal);
    test("1", "1", Up, "1", Equal);
    test("1", "1", Ceiling, "1", Equal);
    test("1", "1", Nearest, "1", Equal);
    test("1", "1", Exact, "1", Equal);

    test("123", "1", Down, "123", Equal);
    test("123", "1", Floor, "123", Equal);
    test("123", "1", Up, "123", Equal);
    test("123", "1", Ceiling, "123", Equal);
    test("123", "1", Nearest, "123", Equal);
    test("123", "1", Exact, "123", Equal);

    test("123", "2", Down, "61", Less);
    test("123", "2", Floor, "61", Less);
    test("123", "2", Up, "62", Greater);
    test("123", "2", Ceiling, "62", Greater);
    test("123", "2", Nearest, "62", Greater);

    test("125", "2", Down, "62", Less);
    test("125", "2", Floor, "62", Less);
    test("125", "2", Up, "63", Greater);
    test("125", "2", Ceiling, "63", Greater);
    test("125", "2", Nearest, "62", Less);

    test("123", "123", Down, "1", Equal);
    test("123", "123", Floor, "1", Equal);
    test("123", "123", Up, "1", Equal);
    test("123", "123", Ceiling, "1", Equal);
    test("123", "123", Nearest, "1", Equal);
    test("123", "123", Exact, "1", Equal);

    test("123", "456", Down, "0", Less);
    test("123", "456", Floor, "0", Less);
    test("123", "456", Up, "1", Greater);
    test("123", "456", Ceiling, "1", Greater);
    test("123", "456", Nearest, "0", Less);

    test("1000000000000", "1", Down, "1000000000000", Equal);
    test("1000000000000", "1", Floor, "1000000000000", Equal);
    test("1000000000000", "1", Up, "1000000000000", Equal);
    test("1000000000000", "1", Ceiling, "1000000000000", Equal);
    test("1000000000000", "1", Nearest, "1000000000000", Equal);
    test("1000000000000", "1", Exact, "1000000000000", Equal);

    test("1000000000000", "3", Down, "333333333333", Less);
    test("1000000000000", "3", Floor, "333333333333", Less);
    test("1000000000000", "3", Up, "333333333334", Greater);
    test("1000000000000", "3", Ceiling, "333333333334", Greater);
    test("1000000000000", "3", Nearest, "333333333333", Less);

    test("999999999999", "2", Down, "499999999999", Less);
    test("999999999999", "2", Floor, "499999999999", Less);
    test("999999999999", "2", Up, "500000000000", Greater);
    test("999999999999", "2", Ceiling, "500000000000", Greater);
    test("999999999999", "2", Nearest, "500000000000", Greater);

    test("1000000000001", "2", Down, "500000000000", Less);
    test("1000000000001", "2", Floor, "500000000000", Less);
    test("1000000000001", "2", Up, "500000000001", Greater);
    test("1000000000001", "2", Ceiling, "500000000001", Greater);
    test("1000000000001", "2", Nearest, "500000000000", Less);

    test(
        "1000000000000000000000000",
        "4294967295",
        Down,
        "232830643708079",
        Less,
    );
    test(
        "1000000000000000000000000",
        "4294967295",
        Floor,
        "232830643708079",
        Less,
    );
    test(
        "1000000000000000000000000",
        "4294967295",
        Up,
        "232830643708080",
        Greater,
    );
    test(
        "1000000000000000000000000",
        "4294967295",
        Ceiling,
        "232830643708080",
        Greater,
    );
    test(
        "1000000000000000000000000",
        "4294967295",
        Nearest,
        "232830643708080",
        Greater,
    );

    test(
        "1000000000000000000000000",
        "1000000000000",
        Down,
        "1000000000000",
        Equal,
    );
    test(
        "1000000000000000000000000",
        "1000000000000",
        Floor,
        "1000000000000",
        Equal,
    );
    test(
        "1000000000000000000000000",
        "1000000000000",
        Up,
        "1000000000000",
        Equal,
    );
    test(
        "1000000000000000000000000",
        "1000000000000",
        Ceiling,
        "1000000000000",
        Equal,
    );
    test(
        "1000000000000000000000000",
        "1000000000000",
        Nearest,
        "1000000000000",
        Equal,
    );
    test(
        "1000000000000000000000000",
        "1000000000000",
        Exact,
        "1000000000000",
        Equal,
    );

    test(
        "1000000000000000000000000",
        "1000000000001",
        Down,
        "999999999999",
        Less,
    );
    test(
        "1000000000000000000000000",
        "1000000000001",
        Floor,
        "999999999999",
        Less,
    );
    test(
        "1000000000000000000000000",
        "1000000000001",
        Up,
        "1000000000000",
        Greater,
    );
    test(
        "1000000000000000000000000",
        "1000000000001",
        Ceiling,
        "1000000000000",
        Greater,
    );
    test(
        "1000000000000000000000000",
        "1000000000001",
        Nearest,
        "999999999999",
        Less,
    );

    test(
        "2999999999999999999999999",
        "2000000000000000000000000",
        Nearest,
        "1",
        Less,
    );
    test(
        "3000000000000000000000000",
        "2000000000000000000000000",
        Nearest,
        "2",
        Greater,
    );
    test(
        "3000000000000000000000001",
        "2000000000000000000000000",
        Nearest,
        "2",
        Greater,
    );

    test("0", "-1", Down, "0", Equal);
    test("0", "-1", Floor, "0", Equal);
    test("0", "-1", Up, "0", Equal);
    test("0", "-1", Ceiling, "0", Equal);
    test("0", "-1", Nearest, "0", Equal);
    test("0", "-1", Exact, "0", Equal);

    test("0", "-123", Down, "0", Equal);
    test("0", "-123", Floor, "0", Equal);
    test("0", "-123", Up, "0", Equal);
    test("0", "-123", Ceiling, "0", Equal);
    test("0", "-123", Nearest, "0", Equal);
    test("0", "-123", Exact, "0", Equal);

    test("1", "-1", Down, "-1", Equal);
    test("1", "-1", Floor, "-1", Equal);
    test("1", "-1", Up, "-1", Equal);
    test("1", "-1", Ceiling, "-1", Equal);
    test("1", "-1", Nearest, "-1", Equal);
    test("1", "-1", Exact, "-1", Equal);

    test("123", "-1", Down, "-123", Equal);
    test("123", "-1", Floor, "-123", Equal);
    test("123", "-1", Up, "-123", Equal);
    test("123", "-1", Ceiling, "-123", Equal);
    test("123", "-1", Nearest, "-123", Equal);
    test("123", "-1", Exact, "-123", Equal);

    test("123", "-2", Down, "-61", Greater);
    test("123", "-2", Floor, "-62", Less);
    test("123", "-2", Up, "-62", Less);
    test("123", "-2", Ceiling, "-61", Greater);
    test("123", "-2", Nearest, "-62", Less);

    test("125", "-2", Down, "-62", Greater);
    test("125", "-2", Floor, "-63", Less);
    test("125", "-2", Up, "-63", Less);
    test("125", "-2", Ceiling, "-62", Greater);
    test("125", "-2", Nearest, "-62", Greater);

    test("123", "-123", Down, "-1", Equal);
    test("123", "-123", Floor, "-1", Equal);
    test("123", "-123", Up, "-1", Equal);
    test("123", "-123", Ceiling, "-1", Equal);
    test("123", "-123", Nearest, "-1", Equal);
    test("123", "-123", Exact, "-1", Equal);

    test("123", "-456", Down, "0", Greater);
    test("123", "-456", Floor, "-1", Less);
    test("123", "-456", Up, "-1", Less);
    test("123", "-456", Ceiling, "0", Greater);
    test("123", "-456", Nearest, "0", Greater);

    test("1000000000000", "-1", Down, "-1000000000000", Equal);
    test("1000000000000", "-1", Floor, "-1000000000000", Equal);
    test("1000000000000", "-1", Up, "-1000000000000", Equal);
    test("1000000000000", "-1", Ceiling, "-1000000000000", Equal);
    test("1000000000000", "-1", Nearest, "-1000000000000", Equal);
    test("1000000000000", "-1", Exact, "-1000000000000", Equal);

    test("1000000000000", "-3", Down, "-333333333333", Greater);
    test("1000000000000", "-3", Floor, "-333333333334", Less);
    test("1000000000000", "-3", Up, "-333333333334", Less);
    test("1000000000000", "-3", Ceiling, "-333333333333", Greater);
    test("1000000000000", "-3", Nearest, "-333333333333", Greater);

    test("999999999999", "-2", Down, "-499999999999", Greater);
    test("999999999999", "-2", Floor, "-500000000000", Less);
    test("999999999999", "-2", Up, "-500000000000", Less);
    test("999999999999", "-2", Ceiling, "-499999999999", Greater);
    test("999999999999", "-2", Nearest, "-500000000000", Less);

    test("1000000000001", "-2", Down, "-500000000000", Greater);
    test("1000000000001", "-2", Floor, "-500000000001", Less);
    test("1000000000001", "-2", Up, "-500000000001", Less);
    test("1000000000001", "-2", Ceiling, "-500000000000", Greater);
    test("1000000000001", "-2", Nearest, "-500000000000", Greater);

    test(
        "1000000000000000000000000",
        "-4294967295",
        Down,
        "-232830643708079",
        Greater,
    );
    test(
        "1000000000000000000000000",
        "-4294967295",
        Floor,
        "-232830643708080",
        Less,
    );
    test(
        "1000000000000000000000000",
        "-4294967295",
        Up,
        "-232830643708080",
        Less,
    );
    test(
        "1000000000000000000000000",
        "-4294967295",
        Ceiling,
        "-232830643708079",
        Greater,
    );
    test(
        "1000000000000000000000000",
        "-4294967295",
        Nearest,
        "-232830643708080",
        Less,
    );

    test(
        "1000000000000000000000000",
        "-1000000000000",
        Down,
        "-1000000000000",
        Equal,
    );
    test(
        "1000000000000000000000000",
        "-1000000000000",
        Floor,
        "-1000000000000",
        Equal,
    );
    test(
        "1000000000000000000000000",
        "-1000000000000",
        Up,
        "-1000000000000",
        Equal,
    );
    test(
        "1000000000000000000000000",
        "-1000000000000",
        Ceiling,
        "-1000000000000",
        Equal,
    );
    test(
        "1000000000000000000000000",
        "-1000000000000",
        Nearest,
        "-1000000000000",
        Equal,
    );
    test(
        "1000000000000000000000000",
        "-1000000000000",
        Exact,
        "-1000000000000",
        Equal,
    );

    test(
        "1000000000000000000000000",
        "-1000000000001",
        Down,
        "-999999999999",
        Greater,
    );
    test(
        "1000000000000000000000000",
        "-1000000000001",
        Floor,
        "-1000000000000",
        Less,
    );
    test(
        "1000000000000000000000000",
        "-1000000000001",
        Up,
        "-1000000000000",
        Less,
    );
    test(
        "1000000000000000000000000",
        "-1000000000001",
        Ceiling,
        "-999999999999",
        Greater,
    );
    test(
        "1000000000000000000000000",
        "-1000000000001",
        Nearest,
        "-999999999999",
        Greater,
    );

    test(
        "2999999999999999999999999",
        "-2000000000000000000000000",
        Nearest,
        "-1",
        Greater,
    );
    test(
        "3000000000000000000000000",
        "-2000000000000000000000000",
        Nearest,
        "-2",
        Less,
    );
    test(
        "3000000000000000000000001",
        "-2000000000000000000000000",
        Nearest,
        "-2",
        Less,
    );

    test("-1", "1", Down, "-1", Equal);
    test("-1", "1", Floor, "-1", Equal);
    test("-1", "1", Up, "-1", Equal);
    test("-1", "1", Ceiling, "-1", Equal);
    test("-1", "1", Nearest, "-1", Equal);
    test("-1", "1", Exact, "-1", Equal);

    test("-123", "1", Down, "-123", Equal);
    test("-123", "1", Floor, "-123", Equal);
    test("-123", "1", Up, "-123", Equal);
    test("-123", "1", Ceiling, "-123", Equal);
    test("-123", "1", Nearest, "-123", Equal);
    test("-123", "1", Exact, "-123", Equal);

    test("-123", "2", Down, "-61", Greater);
    test("-123", "2", Floor, "-62", Less);
    test("-123", "2", Up, "-62", Less);
    test("-123", "2", Ceiling, "-61", Greater);
    test("-123", "2", Nearest, "-62", Less);

    test("-125", "2", Down, "-62", Greater);
    test("-125", "2", Floor, "-63", Less);
    test("-125", "2", Up, "-63", Less);
    test("-125", "2", Ceiling, "-62", Greater);
    test("-125", "2", Nearest, "-62", Greater);

    test("-123", "123", Down, "-1", Equal);
    test("-123", "123", Floor, "-1", Equal);
    test("-123", "123", Up, "-1", Equal);
    test("-123", "123", Ceiling, "-1", Equal);
    test("-123", "123", Nearest, "-1", Equal);
    test("-123", "123", Exact, "-1", Equal);

    test("-123", "456", Down, "0", Greater);
    test("-123", "456", Floor, "-1", Less);
    test("-123", "456", Up, "-1", Less);
    test("-123", "456", Ceiling, "0", Greater);
    test("-123", "456", Nearest, "0", Greater);

    test("-1000000000000", "1", Down, "-1000000000000", Equal);
    test("-1000000000000", "1", Floor, "-1000000000000", Equal);
    test("-1000000000000", "1", Up, "-1000000000000", Equal);
    test("-1000000000000", "1", Ceiling, "-1000000000000", Equal);
    test("-1000000000000", "1", Nearest, "-1000000000000", Equal);
    test("-1000000000000", "1", Exact, "-1000000000000", Equal);

    test("-1000000000000", "3", Down, "-333333333333", Greater);
    test("-1000000000000", "3", Floor, "-333333333334", Less);
    test("-1000000000000", "3", Up, "-333333333334", Less);
    test("-1000000000000", "3", Ceiling, "-333333333333", Greater);
    test("-1000000000000", "3", Nearest, "-333333333333", Greater);

    test("-999999999999", "2", Down, "-499999999999", Greater);
    test("-999999999999", "2", Floor, "-500000000000", Less);
    test("-999999999999", "2", Up, "-500000000000", Less);
    test("-999999999999", "2", Ceiling, "-499999999999", Greater);
    test("-999999999999", "2", Nearest, "-500000000000", Less);

    test("-1000000000001", "2", Down, "-500000000000", Greater);
    test("-1000000000001", "2", Floor, "-500000000001", Less);
    test("-1000000000001", "2", Up, "-500000000001", Less);
    test("-1000000000001", "2", Ceiling, "-500000000000", Greater);
    test("-1000000000001", "2", Nearest, "-500000000000", Greater);

    test(
        "-1000000000000000000000000",
        "4294967295",
        Down,
        "-232830643708079",
        Greater,
    );
    test(
        "-1000000000000000000000000",
        "4294967295",
        Floor,
        "-232830643708080",
        Less,
    );
    test(
        "-1000000000000000000000000",
        "4294967295",
        Up,
        "-232830643708080",
        Less,
    );
    test(
        "-1000000000000000000000000",
        "4294967295",
        Ceiling,
        "-232830643708079",
        Greater,
    );
    test(
        "-1000000000000000000000000",
        "4294967295",
        Nearest,
        "-232830643708080",
        Less,
    );

    test(
        "-1000000000000000000000000",
        "1000000000000",
        Down,
        "-1000000000000",
        Equal,
    );
    test(
        "-1000000000000000000000000",
        "1000000000000",
        Floor,
        "-1000000000000",
        Equal,
    );
    test(
        "-1000000000000000000000000",
        "1000000000000",
        Up,
        "-1000000000000",
        Equal,
    );
    test(
        "-1000000000000000000000000",
        "1000000000000",
        Ceiling,
        "-1000000000000",
        Equal,
    );
    test(
        "-1000000000000000000000000",
        "1000000000000",
        Nearest,
        "-1000000000000",
        Equal,
    );
    test(
        "-1000000000000000000000000",
        "1000000000000",
        Exact,
        "-1000000000000",
        Equal,
    );

    test(
        "-1000000000000000000000000",
        "1000000000001",
        Down,
        "-999999999999",
        Greater,
    );
    test(
        "-1000000000000000000000000",
        "1000000000001",
        Floor,
        "-1000000000000",
        Less,
    );
    test(
        "-1000000000000000000000000",
        "1000000000001",
        Up,
        "-1000000000000",
        Less,
    );
    test(
        "-1000000000000000000000000",
        "1000000000001",
        Ceiling,
        "-999999999999",
        Greater,
    );
    test(
        "-1000000000000000000000000",
        "1000000000001",
        Nearest,
        "-999999999999",
        Greater,
    );

    test(
        "-2999999999999999999999999",
        "2000000000000000000000000",
        Nearest,
        "-1",
        Greater,
    );
    test(
        "-3000000000000000000000000",
        "2000000000000000000000000",
        Nearest,
        "-2",
        Less,
    );
    test(
        "-3000000000000000000000001",
        "2000000000000000000000000",
        Nearest,
        "-2",
        Less,
    );

    test("-1", "-1", Down, "1", Equal);
    test("-1", "-1", Floor, "1", Equal);
    test("-1", "-1", Up, "1", Equal);
    test("-1", "-1", Ceiling, "1", Equal);
    test("-1", "-1", Nearest, "1", Equal);
    test("-1", "-1", Exact, "1", Equal);

    test("-123", "-1", Down, "123", Equal);
    test("-123", "-1", Floor, "123", Equal);
    test("-123", "-1", Up, "123", Equal);
    test("-123", "-1", Ceiling, "123", Equal);
    test("-123", "-1", Nearest, "123", Equal);
    test("-123", "-1", Exact, "123", Equal);

    test("-123", "-2", Down, "61", Less);
    test("-123", "-2", Floor, "61", Less);
    test("-123", "-2", Up, "62", Greater);
    test("-123", "-2", Ceiling, "62", Greater);
    test("-123", "-2", Nearest, "62", Greater);

    test("-125", "-2", Down, "62", Less);
    test("-125", "-2", Floor, "62", Less);
    test("-125", "-2", Up, "63", Greater);
    test("-125", "-2", Ceiling, "63", Greater);
    test("-125", "-2", Nearest, "62", Less);

    test("-123", "-123", Down, "1", Equal);
    test("-123", "-123", Floor, "1", Equal);
    test("-123", "-123", Up, "1", Equal);
    test("-123", "-123", Ceiling, "1", Equal);
    test("-123", "-123", Nearest, "1", Equal);
    test("-123", "-123", Exact, "1", Equal);

    test("-123", "-456", Down, "0", Less);
    test("-123", "-456", Floor, "0", Less);
    test("-123", "-456", Up, "1", Greater);
    test("-123", "-456", Ceiling, "1", Greater);
    test("-123", "-456", Nearest, "0", Less);

    test("-1000000000000", "-1", Down, "1000000000000", Equal);
    test("-1000000000000", "-1", Floor, "1000000000000", Equal);
    test("-1000000000000", "-1", Up, "1000000000000", Equal);
    test("-1000000000000", "-1", Ceiling, "1000000000000", Equal);
    test("-1000000000000", "-1", Nearest, "1000000000000", Equal);
    test("-1000000000000", "-1", Exact, "1000000000000", Equal);

    test("-1000000000000", "-3", Down, "333333333333", Less);
    test("-1000000000000", "-3", Floor, "333333333333", Less);
    test("-1000000000000", "-3", Up, "333333333334", Greater);
    test("-1000000000000", "-3", Ceiling, "333333333334", Greater);
    test("-1000000000000", "-3", Nearest, "333333333333", Less);

    test("-999999999999", "-2", Down, "499999999999", Less);
    test("-999999999999", "-2", Floor, "499999999999", Less);
    test("-999999999999", "-2", Up, "500000000000", Greater);
    test("-999999999999", "-2", Ceiling, "500000000000", Greater);
    test("-999999999999", "-2", Nearest, "500000000000", Greater);

    test("-1000000000001", "-2", Down, "500000000000", Less);
    test("-1000000000001", "-2", Floor, "500000000000", Less);
    test("-1000000000001", "-2", Up, "500000000001", Greater);
    test("-1000000000001", "-2", Ceiling, "500000000001", Greater);
    test("-1000000000001", "-2", Nearest, "500000000000", Less);

    test(
        "-1000000000000000000000000",
        "-4294967295",
        Down,
        "232830643708079",
        Less,
    );
    test(
        "-1000000000000000000000000",
        "-4294967295",
        Floor,
        "232830643708079",
        Less,
    );
    test(
        "-1000000000000000000000000",
        "-4294967295",
        Up,
        "232830643708080",
        Greater,
    );
    test(
        "-1000000000000000000000000",
        "-4294967295",
        Ceiling,
        "232830643708080",
        Greater,
    );
    test(
        "-1000000000000000000000000",
        "-4294967295",
        Nearest,
        "232830643708080",
        Greater,
    );

    test(
        "-1000000000000000000000000",
        "-1000000000000",
        Down,
        "1000000000000",
        Equal,
    );
    test(
        "-1000000000000000000000000",
        "-1000000000000",
        Floor,
        "1000000000000",
        Equal,
    );
    test(
        "-1000000000000000000000000",
        "-1000000000000",
        Up,
        "1000000000000",
        Equal,
    );
    test(
        "-1000000000000000000000000",
        "-1000000000000",
        Ceiling,
        "1000000000000",
        Equal,
    );
    test(
        "-1000000000000000000000000",
        "-1000000000000",
        Nearest,
        "1000000000000",
        Equal,
    );
    test(
        "-1000000000000000000000000",
        "-1000000000000",
        Exact,
        "1000000000000",
        Equal,
    );

    test(
        "-1000000000000000000000000",
        "-1000000000001",
        Down,
        "999999999999",
        Less,
    );
    test(
        "-1000000000000000000000000",
        "-1000000000001",
        Floor,
        "999999999999",
        Less,
    );
    test(
        "-1000000000000000000000000",
        "-1000000000001",
        Up,
        "1000000000000",
        Greater,
    );
    test(
        "-1000000000000000000000000",
        "-1000000000001",
        Ceiling,
        "1000000000000",
        Greater,
    );
    test(
        "-1000000000000000000000000",
        "-1000000000001",
        Nearest,
        "999999999999",
        Less,
    );

    test(
        "-2999999999999999999999999",
        "-2000000000000000000000000",
        Nearest,
        "1",
        Less,
    );
    test(
        "-3000000000000000000000000",
        "-2000000000000000000000000",
        Nearest,
        "2",
        Greater,
    );
    test(
        "-3000000000000000000000001",
        "-2000000000000000000000000",
        Nearest,
        "2",
        Greater,
    );
}

#[test]
#[should_panic]
fn div_round_assign_fail_1() {
    let mut n = Integer::from(10);
    n.div_round_assign(Integer::ZERO, Floor);
}

#[test]
#[should_panic]
fn div_round_assign_fail_2() {
    let mut n = Integer::from(10);
    n.div_round_assign(Integer::from(3), Exact);
}

#[test]
#[should_panic]
fn div_round_assign_ref_fail_1() {
    let mut n = Integer::from(10);
    n.div_round_assign(&Integer::ZERO, Floor);
}

#[test]
#[should_panic]
fn div_round_assign_ref_fail_2() {
    let mut n = Integer::from(10);
    n.div_round_assign(&Integer::from(3), Exact);
}

#[test]
#[should_panic]
fn div_round_fail_1() {
    Integer::from(10).div_round(Integer::ZERO, Floor);
}

#[test]
#[should_panic]
fn div_round_fail_2() {
    Integer::from(10).div_round(Integer::from(3), Exact);
}

#[test]
#[should_panic]
fn div_round_val_ref_fail_1() {
    Integer::from(10).div_round(&Integer::ZERO, Floor);
}

#[test]
#[should_panic]
fn div_round_val_ref_fail_2() {
    Integer::from(10).div_round(&Integer::from(3), Exact);
}

#[test]
#[should_panic]
fn div_round_ref_val_fail_1() {
    (&Integer::from(10)).div_round(Integer::ZERO, Floor);
}

#[test]
#[should_panic]
fn div_round_ref_val_fail_2() {
    (&Integer::from(10)).div_round(Integer::from(3), Exact);
}

#[test]
#[should_panic]
fn div_round_ref_ref_fail_1() {
    (&Integer::from(10)).div_round(&Integer::ZERO, Floor);
}

#[test]
#[should_panic]
fn div_round_ref_ref_fail_2() {
    (&Integer::from(10)).div_round(&Integer::from(3), Exact);
}

#[test]
fn div_round_properties() {
    let mut config = GenConfig::new();
    config.insert("mean_bits_n", 2048);
    config.insert("mean_stripe_n", 16 << Limb::LOG_WIDTH);
    integer_integer_rounding_mode_triple_gen_var_1().test_properties_with_config(
        &config,
        |(x, y, rm)| {
            let mut mut_n = x.clone();
            let o = mut_n.div_round_assign(&y, rm);
            assert!(mut_n.is_valid());
            let q = mut_n;

            let mut mut_n = x.clone();
            assert_eq!(mut_n.div_round_assign(y.clone(), rm), o);
            assert!(mut_n.is_valid());
            assert_eq!(mut_n, q);

            let (q_alt, o_alt) = (&x).div_round(&y, rm);
            assert!(q_alt.is_valid());
            assert_eq!(q_alt, q);
            assert_eq!(o_alt, o);

            let (q_alt, o_alt) = (&x).div_round(y.clone(), rm);
            assert!(q_alt.is_valid());
            assert_eq!(q_alt, q);
            assert_eq!(o_alt, o);

            let (q_alt, o_alt) = x.clone().div_round(&y, rm);
            assert!(q_alt.is_valid());
            assert_eq!(q_alt, q);
            assert_eq!(o_alt, o);

            let (q_alt, o_alt) = x.clone().div_round(y.clone(), rm);
            assert!(q_alt.is_valid());
            assert_eq!(q_alt, q);
            assert_eq!(o_alt, o);

            assert!(q.le_abs(&x));
            let (q_alt, o_alt) = (-&x).div_round(&y, -rm);
            assert_eq!(-q_alt, q);
            assert_eq!(o_alt, o.reverse());
            let (q_alt, o_alt) = (&x).div_round(-&y, -rm);
            assert_eq!(-q_alt, q);
            assert_eq!(o_alt, o.reverse());

            assert_eq!((q * &y).cmp(&x), if y >= 0 { o } else { o.reverse() });

            match ((x >= 0) == (y >= 0), rm) {
                (_, Floor) | (true, Down) | (false, Up) => {
                    assert_ne!(o, Greater);
                }
                (_, Ceiling) | (true, Up) | (false, Down) => {
                    assert_ne!(o, Less);
                }
                (_, Exact) => assert_eq!(o, Equal),
                _ => {}
            }
        },
    );

    integer_pair_gen_var_1().test_properties(|(x, y)| {
        let left_multiplied = &x * &y;
        let xo = (x.clone(), Equal);
        assert_eq!((&left_multiplied).div_round(&y, Down), xo);
        assert_eq!((&left_multiplied).div_round(&y, Up), xo);
        assert_eq!((&left_multiplied).div_round(&y, Floor), xo);
        assert_eq!((&left_multiplied).div_round(&y, Ceiling), xo);
        assert_eq!((&left_multiplied).div_round(&y, Nearest), xo);
        assert_eq!((&left_multiplied).div_round(&y, Exact), xo);

        assert_eq!(
            Integer::from(&rug::Integer::from(&x).div_trunc(rug::Integer::from(&y))),
            (&x).div_round(&y, Down).0
        );
        assert_eq!(
            Integer::from(&BigInt::from(&x).div_floor(&BigInt::from(&y))),
            (&x).div_round(&y, Floor).0
        );
        assert_eq!(
            Integer::from(&rug::Integer::from(&x).div_floor(rug::Integer::from(&y))),
            (&x).div_round(&y, Floor).0
        );
        assert_eq!(
            Integer::from(&rug::Integer::from(&x).div_ceil(rug::Integer::from(&y))),
            x.div_round(y, Ceiling).0
        );
    });

    integer_pair_gen_var_3().test_properties(|(x, y)| {
        let down = (&x).div_round(&y, Down);
        let up = if (x >= 0) == (y >= 0) {
            (&down.0 + Integer::ONE, Greater)
        } else {
            (&down.0 - Integer::ONE, Less)
        };
        let floor = (&x).div_round(&y, Floor);
        let ceiling = (&floor.0 + Integer::ONE, Greater);
        assert_eq!((&x).div_round(&y, Up), up);
        assert_eq!((&x).div_round(&y, Ceiling), ceiling);
        let nearest = x.div_round(y, Nearest);
        assert!(nearest == down || nearest == up);
    });

    integer_rounding_mode_pair_gen().test_properties(|(x, rm)| {
        assert_eq!((&x).div_round(Integer::ONE, rm), (x.clone(), Equal));
        assert_eq!((&x).div_round(Integer::NEGATIVE_ONE, rm), (-x, Equal));
    });

    integer_rounding_mode_pair_gen_var_2().test_properties(|(ref x, rm)| {
        assert_eq!(Integer::ZERO.div_round(x, rm), (Integer::ZERO, Equal));
        assert_eq!(x.div_round(x, rm), (Integer::ONE, Equal));
        assert_eq!(x.div_round(-x, rm), (Integer::NEGATIVE_ONE, Equal));
        assert_eq!((-x).div_round(x, rm), (Integer::NEGATIVE_ONE, Equal));
    });

    natural_natural_rounding_mode_triple_gen_var_1().test_properties(|(x, y, rm)| {
        let (q, o) = (&x).div_round(&y, rm);
        assert_eq!(
            Integer::from(x).div_round(Integer::from(y), rm),
            (Integer::from(q), o)
        );
    });

    signed_signed_rounding_mode_triple_gen_var_1::<SignedLimb>().test_properties(|(x, y, rm)| {
        let (q, o) = x.div_round(y, rm);
        assert_eq!(
            Integer::from(x).div_round(Integer::from(y), rm),
            (Integer::from(q), o)
        );
    });
}
