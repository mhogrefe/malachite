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
use malachite_base::rounding_modes::RoundingMode;
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
use std::cmp::Ordering;
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
    test("0", 0, RoundingMode::Down, "0", Ordering::Equal);
    test("0", 0, RoundingMode::Up, "0", Ordering::Equal);
    test("0", 0, RoundingMode::Floor, "0", Ordering::Equal);
    test("0", 0, RoundingMode::Ceiling, "0", Ordering::Equal);
    test("0", 0, RoundingMode::Nearest, "0", Ordering::Equal);
    test("0", 0, RoundingMode::Exact, "0", Ordering::Equal);

    test("0", 10, RoundingMode::Down, "0", Ordering::Equal);
    test("0", 10, RoundingMode::Up, "0", Ordering::Equal);
    test("0", 10, RoundingMode::Floor, "0", Ordering::Equal);
    test("0", 10, RoundingMode::Ceiling, "0", Ordering::Equal);
    test("0", 10, RoundingMode::Nearest, "0", Ordering::Equal);
    test("0", 10, RoundingMode::Exact, "0", Ordering::Equal);

    test("123", 0, RoundingMode::Down, "123", Ordering::Equal);
    test("123", 0, RoundingMode::Up, "123", Ordering::Equal);
    test("123", 0, RoundingMode::Floor, "123", Ordering::Equal);
    test("123", 0, RoundingMode::Ceiling, "123", Ordering::Equal);
    test("123", 0, RoundingMode::Nearest, "123", Ordering::Equal);
    test("123", 0, RoundingMode::Exact, "123", Ordering::Equal);

    test("123", 2, RoundingMode::Down, "120", Ordering::Less);
    test("123", 2, RoundingMode::Up, "124", Ordering::Greater);
    test("123", 2, RoundingMode::Floor, "120", Ordering::Less);
    test("123", 2, RoundingMode::Ceiling, "124", Ordering::Greater);
    test("123", 2, RoundingMode::Nearest, "124", Ordering::Greater);

    test("123", -2, RoundingMode::Down, "123", Ordering::Equal);
    test("123", -2, RoundingMode::Up, "123", Ordering::Equal);
    test("123", -2, RoundingMode::Floor, "123", Ordering::Equal);
    test("123", -2, RoundingMode::Ceiling, "123", Ordering::Equal);
    test("123", -2, RoundingMode::Nearest, "123", Ordering::Equal);
    test("123", -2, RoundingMode::Exact, "123", Ordering::Equal);

    test(
        "884279719003555/281474976710656",
        2,
        RoundingMode::Down,
        "0",
        Ordering::Less,
    );
    test(
        "884279719003555/281474976710656",
        2,
        RoundingMode::Up,
        "4",
        Ordering::Greater,
    );
    test(
        "884279719003555/281474976710656",
        2,
        RoundingMode::Floor,
        "0",
        Ordering::Less,
    );
    test(
        "884279719003555/281474976710656",
        2,
        RoundingMode::Ceiling,
        "4",
        Ordering::Greater,
    );
    test(
        "884279719003555/281474976710656",
        2,
        RoundingMode::Nearest,
        "4",
        Ordering::Greater,
    );

    test(
        "884279719003555/281474976710656",
        -4,
        RoundingMode::Down,
        "25/8",
        Ordering::Less,
    );
    test(
        "884279719003555/281474976710656",
        -4,
        RoundingMode::Up,
        "51/16",
        Ordering::Greater,
    );
    test(
        "884279719003555/281474976710656",
        -4,
        RoundingMode::Floor,
        "25/8",
        Ordering::Less,
    );
    test(
        "884279719003555/281474976710656",
        -4,
        RoundingMode::Ceiling,
        "51/16",
        Ordering::Greater,
    );
    test(
        "884279719003555/281474976710656",
        -4,
        RoundingMode::Nearest,
        "25/8",
        Ordering::Less,
    );

    test(
        "884279719003555/281474976710656",
        -10,
        RoundingMode::Down,
        "201/64",
        Ordering::Less,
    );
    test(
        "884279719003555/281474976710656",
        -10,
        RoundingMode::Up,
        "3217/1024",
        Ordering::Greater,
    );
    test(
        "884279719003555/281474976710656",
        -10,
        RoundingMode::Floor,
        "201/64",
        Ordering::Less,
    );
    test(
        "884279719003555/281474976710656",
        -10,
        RoundingMode::Ceiling,
        "3217/1024",
        Ordering::Greater,
    );
    test(
        "884279719003555/281474976710656",
        -10,
        RoundingMode::Nearest,
        "3217/1024",
        Ordering::Greater,
    );

    test("-123", 0, RoundingMode::Down, "-123", Ordering::Equal);
    test("-123", 0, RoundingMode::Up, "-123", Ordering::Equal);
    test("-123", 0, RoundingMode::Floor, "-123", Ordering::Equal);
    test("-123", 0, RoundingMode::Ceiling, "-123", Ordering::Equal);
    test("-123", 0, RoundingMode::Nearest, "-123", Ordering::Equal);
    test("-123", 0, RoundingMode::Exact, "-123", Ordering::Equal);

    test("-123", 2, RoundingMode::Down, "-120", Ordering::Greater);
    test("-123", 2, RoundingMode::Up, "-124", Ordering::Less);
    test("-123", 2, RoundingMode::Floor, "-124", Ordering::Less);
    test("-123", 2, RoundingMode::Ceiling, "-120", Ordering::Greater);
    test("-123", 2, RoundingMode::Nearest, "-124", Ordering::Less);

    test("-123", -2, RoundingMode::Down, "-123", Ordering::Equal);
    test("-123", -2, RoundingMode::Up, "-123", Ordering::Equal);
    test("-123", -2, RoundingMode::Floor, "-123", Ordering::Equal);
    test("-123", -2, RoundingMode::Ceiling, "-123", Ordering::Equal);
    test("-123", -2, RoundingMode::Nearest, "-123", Ordering::Equal);
    test("-123", -2, RoundingMode::Exact, "-123", Ordering::Equal);

    test(
        "-884279719003555/281474976710656",
        2,
        RoundingMode::Down,
        "0",
        Ordering::Greater,
    );
    test(
        "-884279719003555/281474976710656",
        2,
        RoundingMode::Up,
        "-4",
        Ordering::Less,
    );
    test(
        "-884279719003555/281474976710656",
        2,
        RoundingMode::Floor,
        "-4",
        Ordering::Less,
    );
    test(
        "-884279719003555/281474976710656",
        2,
        RoundingMode::Ceiling,
        "0",
        Ordering::Greater,
    );
    test(
        "-884279719003555/281474976710656",
        2,
        RoundingMode::Nearest,
        "-4",
        Ordering::Less,
    );

    test(
        "-884279719003555/281474976710656",
        -4,
        RoundingMode::Down,
        "-25/8",
        Ordering::Greater,
    );
    test(
        "-884279719003555/281474976710656",
        -4,
        RoundingMode::Up,
        "-51/16",
        Ordering::Less,
    );
    test(
        "-884279719003555/281474976710656",
        -4,
        RoundingMode::Floor,
        "-51/16",
        Ordering::Less,
    );
    test(
        "-884279719003555/281474976710656",
        -4,
        RoundingMode::Ceiling,
        "-25/8",
        Ordering::Greater,
    );
    test(
        "-884279719003555/281474976710656",
        -4,
        RoundingMode::Nearest,
        "-25/8",
        Ordering::Greater,
    );

    test(
        "-884279719003555/281474976710656",
        -10,
        RoundingMode::Down,
        "-201/64",
        Ordering::Greater,
    );
    test(
        "-884279719003555/281474976710656",
        -10,
        RoundingMode::Up,
        "-3217/1024",
        Ordering::Less,
    );
    test(
        "-884279719003555/281474976710656",
        -10,
        RoundingMode::Floor,
        "-3217/1024",
        Ordering::Less,
    );
    test(
        "-884279719003555/281474976710656",
        -10,
        RoundingMode::Ceiling,
        "-201/64",
        Ordering::Greater,
    );
    test(
        "-884279719003555/281474976710656",
        -10,
        RoundingMode::Nearest,
        "-3217/1024",
        Ordering::Less,
    );
}

#[test]
#[should_panic]
fn round_to_multiple_of_power_of_2_assign_fail_1() {
    Rational::from(-123).round_to_multiple_of_power_of_2_assign(1, RoundingMode::Exact);
}

#[test]
#[should_panic]
fn round_to_multiple_of_power_of_2_assign_fail_2() {
    Rational::from(-123).round_to_multiple_of_power_of_2_assign(100, RoundingMode::Exact);
}

#[test]
#[should_panic]
fn round_to_multiple_of_power_of_2_assign_fail_3() {
    Rational::from_str("-1000000000001")
        .unwrap()
        .round_to_multiple_of_power_of_2_assign(1, RoundingMode::Exact);
}

#[test]
#[should_panic]
fn round_to_multiple_of_power_of_2_assign_fail_4() {
    Rational::from_str("-1000000000001")
        .unwrap()
        .round_to_multiple_of_power_of_2_assign(100, RoundingMode::Exact);
}

#[test]
#[should_panic]
fn round_to_multiple_of_power_of_2_fail_1() {
    Rational::from(-123).round_to_multiple_of_power_of_2(1, RoundingMode::Exact);
}

#[test]
#[should_panic]
fn round_to_multiple_of_power_of_2_fail_2() {
    Rational::from(-123).round_to_multiple_of_power_of_2(100, RoundingMode::Exact);
}

#[test]
#[should_panic]
fn round_to_multiple_of_power_of_2_fail_3() {
    Rational::from_str("-1000000000001")
        .unwrap()
        .round_to_multiple_of_power_of_2(1, RoundingMode::Exact);
}

#[test]
#[should_panic]
fn round_to_multiple_of_power_of_2_fail_4() {
    Rational::from_str("-1000000000001")
        .unwrap()
        .round_to_multiple_of_power_of_2(100, RoundingMode::Exact);
}

#[test]
#[should_panic]
fn round_to_multiple_of_power_of_2_ref_fail_1() {
    (&Rational::from(-123)).round_to_multiple_of_power_of_2(1, RoundingMode::Exact);
}

#[test]
#[should_panic]
fn round_to_multiple_of_power_of_2_ref_fail_2() {
    (&Rational::from(-123)).round_to_multiple_of_power_of_2(100, RoundingMode::Exact);
}

#[test]
#[should_panic]
fn round_to_multiple_of_power_of_2_ref_fail_3() {
    (&Rational::from_str("-1000000000001").unwrap())
        .round_to_multiple_of_power_of_2(1, RoundingMode::Exact);
}

#[test]
#[should_panic]
fn round_to_multiple_of_power_of_2_ref_fail_4() {
    (&Rational::from_str("-1000000000001").unwrap())
        .round_to_multiple_of_power_of_2(100, RoundingMode::Exact);
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
            (_, RoundingMode::Floor) | (true, RoundingMode::Down) | (false, RoundingMode::Up) => {
                assert_ne!(o, Ordering::Greater)
            }
            (_, RoundingMode::Ceiling) | (true, RoundingMode::Up) | (false, RoundingMode::Down) => {
                assert_ne!(o, Ordering::Less)
            }
            (_, RoundingMode::Exact) => assert_eq!(o, Ordering::Equal),
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
            RoundingMode::Floor => assert!(r <= n),
            RoundingMode::Ceiling => assert!(r >= n),
            RoundingMode::Down => assert!(r.le_abs(&n)),
            RoundingMode::Up => assert!(r.ge_abs(&n)),
            RoundingMode::Exact => assert_eq!(r, n),
            RoundingMode::Nearest => {
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
        let rounded = (&n)
            .round_to_multiple_of_power_of_2(pow, RoundingMode::Nearest)
            .0;
        let ro = (rounded.clone(), Ordering::Equal);
        assert_eq!(
            (&rounded).round_to_multiple_of_power_of_2(pow, RoundingMode::Down),
            ro
        );
        assert_eq!(
            (&rounded).round_to_multiple_of_power_of_2(pow, RoundingMode::Up),
            ro
        );
        assert_eq!(
            (&rounded).round_to_multiple_of_power_of_2(pow, RoundingMode::Floor),
            ro
        );
        assert_eq!(
            (&rounded).round_to_multiple_of_power_of_2(pow, RoundingMode::Ceiling),
            ro
        );
        assert_eq!(
            (&rounded).round_to_multiple_of_power_of_2(pow, RoundingMode::Nearest),
            ro
        );
        assert_eq!(
            (&rounded).round_to_multiple_of_power_of_2(pow, RoundingMode::Exact),
            ro
        );
    });

    rational_signed_pair_gen_var_3().test_properties(|(n, pow)| {
        let floor = (&n).round_to_multiple_of_power_of_2(pow, RoundingMode::Floor);
        assert_eq!(floor.1, Ordering::Less);
        let ceiling = (&floor.0 + Rational::power_of_2(pow), Ordering::Greater);
        assert_eq!(
            (&n).round_to_multiple_of_power_of_2(pow, RoundingMode::Ceiling),
            ceiling
        );
        if n >= 0 {
            assert_eq!(
                (&n).round_to_multiple_of_power_of_2(pow, RoundingMode::Up),
                ceiling
            );
            assert_eq!(
                (&n).round_to_multiple_of_power_of_2(pow, RoundingMode::Down),
                floor
            );
        } else {
            assert_eq!(
                (&n).round_to_multiple_of_power_of_2(pow, RoundingMode::Up),
                floor
            );
            assert_eq!(
                (&n).round_to_multiple_of_power_of_2(pow, RoundingMode::Down),
                ceiling
            );
        }
        let nearest = n.round_to_multiple_of_power_of_2(pow, RoundingMode::Nearest);
        assert!(nearest == ceiling || nearest == floor);
    });

    integer_rounding_mode_pair_gen().test_properties(|(n, rm)| {
        let rn = Rational::from(n);
        assert_eq!(
            (&rn).round_to_multiple_of_power_of_2(0, rm),
            (rn, Ordering::Equal)
        );
    });

    signed_rounding_mode_pair_gen().test_properties(|(pow, rm)| {
        assert_eq!(
            Rational::ZERO.round_to_multiple_of_power_of_2(pow, rm),
            (Rational::ZERO, Ordering::Equal)
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
