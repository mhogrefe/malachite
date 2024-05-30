// Copyright © 2024 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::arithmetic::traits::{
    Abs, Parity, PowerOf2, RoundToMultiple, RoundToMultipleOfPowerOf2,
    RoundToMultipleOfPowerOf2Assign,
};
use malachite_base::num::basic::traits::Zero;
use malachite_base::num::comparison::traits::PartialOrdAbs;
use malachite_base::num::conversion::traits::{ExactFrom, IsInteger};
use malachite_base::rounding_modes::RoundingMode::{self, *};
use malachite_base::test_util::generators::signed_rounding_mode_pair_gen;
use malachite_nz::integer::Integer;
use malachite_nz::test_util::generators::{
    integer_rounding_mode_pair_gen, integer_unsigned_rounding_mode_triple_gen_var_1,
};
use malachite_q::test_util::generators::{
    rational_signed_pair_gen_var_1, rational_signed_pair_gen_var_3,
    rational_signed_rounding_mode_triple_gen_var_1,
};
use malachite_q::Rational;
use std::cmp::Ordering::*;
use std::str::FromStr;

#[test]
fn test_round_to_multiple_of_power_of_2() {
    let test = |s, v: i64, rm: RoundingMode, out, o| {
        let u = Rational::from_str(s).unwrap();

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

    test("123", 2, Down, "120", Less);
    test("123", 2, Up, "124", Greater);
    test("123", 2, Floor, "120", Less);
    test("123", 2, Ceiling, "124", Greater);
    test("123", 2, Nearest, "124", Greater);

    test("123", -2, Down, "123", Equal);
    test("123", -2, Up, "123", Equal);
    test("123", -2, Floor, "123", Equal);
    test("123", -2, Ceiling, "123", Equal);
    test("123", -2, Nearest, "123", Equal);
    test("123", -2, Exact, "123", Equal);

    test("884279719003555/281474976710656", 2, Down, "0", Less);
    test("884279719003555/281474976710656", 2, Up, "4", Greater);
    test("884279719003555/281474976710656", 2, Floor, "0", Less);
    test("884279719003555/281474976710656", 2, Ceiling, "4", Greater);
    test("884279719003555/281474976710656", 2, Nearest, "4", Greater);

    test("884279719003555/281474976710656", -4, Down, "25/8", Less);
    test("884279719003555/281474976710656", -4, Up, "51/16", Greater);
    test("884279719003555/281474976710656", -4, Floor, "25/8", Less);
    test(
        "884279719003555/281474976710656",
        -4,
        Ceiling,
        "51/16",
        Greater,
    );
    test("884279719003555/281474976710656", -4, Nearest, "25/8", Less);

    test("884279719003555/281474976710656", -10, Down, "201/64", Less);
    test(
        "884279719003555/281474976710656",
        -10,
        Up,
        "3217/1024",
        Greater,
    );
    test(
        "884279719003555/281474976710656",
        -10,
        Floor,
        "201/64",
        Less,
    );
    test(
        "884279719003555/281474976710656",
        -10,
        Ceiling,
        "3217/1024",
        Greater,
    );
    test(
        "884279719003555/281474976710656",
        -10,
        Nearest,
        "3217/1024",
        Greater,
    );

    test("-123", 0, Down, "-123", Equal);
    test("-123", 0, Up, "-123", Equal);
    test("-123", 0, Floor, "-123", Equal);
    test("-123", 0, Ceiling, "-123", Equal);
    test("-123", 0, Nearest, "-123", Equal);
    test("-123", 0, Exact, "-123", Equal);

    test("-123", 2, Down, "-120", Greater);
    test("-123", 2, Up, "-124", Less);
    test("-123", 2, Floor, "-124", Less);
    test("-123", 2, Ceiling, "-120", Greater);
    test("-123", 2, Nearest, "-124", Less);

    test("-123", -2, Down, "-123", Equal);
    test("-123", -2, Up, "-123", Equal);
    test("-123", -2, Floor, "-123", Equal);
    test("-123", -2, Ceiling, "-123", Equal);
    test("-123", -2, Nearest, "-123", Equal);
    test("-123", -2, Exact, "-123", Equal);

    test("-884279719003555/281474976710656", 2, Down, "0", Greater);
    test("-884279719003555/281474976710656", 2, Up, "-4", Less);
    test("-884279719003555/281474976710656", 2, Floor, "-4", Less);
    test("-884279719003555/281474976710656", 2, Ceiling, "0", Greater);
    test("-884279719003555/281474976710656", 2, Nearest, "-4", Less);

    test(
        "-884279719003555/281474976710656",
        -4,
        Down,
        "-25/8",
        Greater,
    );
    test("-884279719003555/281474976710656", -4, Up, "-51/16", Less);
    test(
        "-884279719003555/281474976710656",
        -4,
        Floor,
        "-51/16",
        Less,
    );
    test(
        "-884279719003555/281474976710656",
        -4,
        Ceiling,
        "-25/8",
        Greater,
    );
    test(
        "-884279719003555/281474976710656",
        -4,
        Nearest,
        "-25/8",
        Greater,
    );

    test(
        "-884279719003555/281474976710656",
        -10,
        Down,
        "-201/64",
        Greater,
    );
    test(
        "-884279719003555/281474976710656",
        -10,
        Up,
        "-3217/1024",
        Less,
    );
    test(
        "-884279719003555/281474976710656",
        -10,
        Floor,
        "-3217/1024",
        Less,
    );
    test(
        "-884279719003555/281474976710656",
        -10,
        Ceiling,
        "-201/64",
        Greater,
    );
    test(
        "-884279719003555/281474976710656",
        -10,
        Nearest,
        "-3217/1024",
        Less,
    );
}

#[test]
#[should_panic]
fn round_to_multiple_of_power_of_2_assign_fail_1() {
    Rational::from(-123).round_to_multiple_of_power_of_2_assign(1, Exact);
}

#[test]
#[should_panic]
fn round_to_multiple_of_power_of_2_assign_fail_2() {
    Rational::from(-123).round_to_multiple_of_power_of_2_assign(100, Exact);
}

#[test]
#[should_panic]
fn round_to_multiple_of_power_of_2_assign_fail_3() {
    Rational::from_str("-1000000000001")
        .unwrap()
        .round_to_multiple_of_power_of_2_assign(1, Exact);
}

#[test]
#[should_panic]
fn round_to_multiple_of_power_of_2_assign_fail_4() {
    Rational::from_str("-1000000000001")
        .unwrap()
        .round_to_multiple_of_power_of_2_assign(100, Exact);
}

#[test]
#[should_panic]
fn round_to_multiple_of_power_of_2_fail_1() {
    Rational::from(-123).round_to_multiple_of_power_of_2(1, Exact);
}

#[test]
#[should_panic]
fn round_to_multiple_of_power_of_2_fail_2() {
    Rational::from(-123).round_to_multiple_of_power_of_2(100, Exact);
}

#[test]
#[should_panic]
fn round_to_multiple_of_power_of_2_fail_3() {
    Rational::from_str("-1000000000001")
        .unwrap()
        .round_to_multiple_of_power_of_2(1, Exact);
}

#[test]
#[should_panic]
fn round_to_multiple_of_power_of_2_fail_4() {
    Rational::from_str("-1000000000001")
        .unwrap()
        .round_to_multiple_of_power_of_2(100, Exact);
}

#[test]
#[should_panic]
fn round_to_multiple_of_power_of_2_ref_fail_1() {
    (&Rational::from(-123)).round_to_multiple_of_power_of_2(1, Exact);
}

#[test]
#[should_panic]
fn round_to_multiple_of_power_of_2_ref_fail_2() {
    (&Rational::from(-123)).round_to_multiple_of_power_of_2(100, Exact);
}

#[test]
#[should_panic]
fn round_to_multiple_of_power_of_2_ref_fail_3() {
    (&Rational::from_str("-1000000000001").unwrap()).round_to_multiple_of_power_of_2(1, Exact);
}

#[test]
#[should_panic]
fn round_to_multiple_of_power_of_2_ref_fail_4() {
    (&Rational::from_str("-1000000000001").unwrap()).round_to_multiple_of_power_of_2(100, Exact);
}

#[test]
fn round_to_multiple_of_power_of_2_properties() {
    rational_signed_rounding_mode_triple_gen_var_1().test_properties(|(n, pow, rm)| {
        let (r, o) = (&n).round_to_multiple_of_power_of_2(pow, rm);
        assert!(r.is_valid());

        let (r_alt, o_alt) = n.clone().round_to_multiple_of_power_of_2(pow, rm);
        assert!(r_alt.is_valid());
        assert_eq!(r_alt, r);
        assert_eq!(o_alt, o);

        let mut mut_n = n.clone();
        let o_alt = mut_n.round_to_multiple_of_power_of_2_assign(pow, rm);
        assert!(mut_n.is_valid());
        assert_eq!(mut_n, r);
        assert_eq!(o_alt, o);

        assert!((&r >> pow).is_integer());
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

        let (r_alt, o_alt) = (-&n).round_to_multiple_of_power_of_2(pow, -rm);
        assert_eq!(-r_alt, r);
        assert_eq!(o_alt.reverse(), o);
        assert!((&r - &n).abs() <= Rational::power_of_2(pow));
        let (r_alt, o_alt) = (&n).round_to_multiple(Rational::power_of_2(pow), rm);
        assert_eq!(r_alt, r);
        assert_eq!(o_alt, o);
        match rm {
            Floor => assert!(r <= n),
            Ceiling => assert!(r >= n),
            Down => assert!(r.le_abs(&n)),
            Up => assert!(r.ge_abs(&n)),
            Exact => assert_eq!(r, n),
            Nearest => {
                let k = Rational::power_of_2(pow);
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
                    assert!(Integer::exact_from(r >> pow).even());
                }
            }
        }
    });

    rational_signed_pair_gen_var_1().test_properties(|(n, pow)| {
        let rounded = (&n).round_to_multiple_of_power_of_2(pow, Nearest).0;
        let ro = (rounded.clone(), Equal);
        assert_eq!((&rounded).round_to_multiple_of_power_of_2(pow, Down), ro);
        assert_eq!((&rounded).round_to_multiple_of_power_of_2(pow, Up), ro);
        assert_eq!((&rounded).round_to_multiple_of_power_of_2(pow, Floor), ro);
        assert_eq!((&rounded).round_to_multiple_of_power_of_2(pow, Ceiling), ro);
        assert_eq!((&rounded).round_to_multiple_of_power_of_2(pow, Nearest), ro);
        assert_eq!((&rounded).round_to_multiple_of_power_of_2(pow, Exact), ro);
    });

    rational_signed_pair_gen_var_3().test_properties(|(n, pow)| {
        let floor = (&n).round_to_multiple_of_power_of_2(pow, Floor);
        assert_eq!(floor.1, Less);
        let ceiling = (&floor.0 + Rational::power_of_2(pow), Greater);
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
        let rn = Rational::from(n);
        assert_eq!((&rn).round_to_multiple_of_power_of_2(0, rm), (rn, Equal));
    });

    signed_rounding_mode_pair_gen().test_properties(|(pow, rm)| {
        assert_eq!(
            Rational::ZERO.round_to_multiple_of_power_of_2(pow, rm),
            (Rational::ZERO, Equal)
        );
    });

    integer_unsigned_rounding_mode_triple_gen_var_1().test_properties(|(n, pow, rm)| {
        let (r, o) = (&n).round_to_multiple_of_power_of_2(pow, rm);
        let (r_alt, o_alt) =
            Rational::from(n).round_to_multiple_of_power_of_2(i64::exact_from(pow), rm);
        assert_eq!(r_alt, r);
        assert_eq!(o_alt, o);
    });
}
