// Copyright © 2025 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::arithmetic::traits::{
    Abs, Parity, RoundToMultiple, RoundToMultipleAssign,
};
use malachite_base::num::basic::traits::{NegativeOne, One, Zero};
use malachite_base::num::comparison::traits::PartialOrdAbs;
use malachite_base::num::conversion::traits::{ExactFrom, IsInteger};
use malachite_base::rounding_modes::RoundingMode::*;
use malachite_nz::integer::Integer;
use malachite_nz::test_util::generators::{
    integer_integer_rounding_mode_triple_gen_var_2, integer_rounding_mode_pair_gen,
};
use malachite_q::test_util::generators::{
    rational_pair_gen_var_1, rational_pair_gen_var_2,
    rational_rational_rounding_mode_triple_gen_var_1,
};
use malachite_q::Rational;
use std::cmp::Ordering::*;
use std::str::FromStr;

#[test]
fn test_round_to_multiple() {
    let test = |s, t, rm, quotient, o| {
        let u = Rational::from_str(s).unwrap();
        let v = Rational::from_str(t).unwrap();

        let mut n = u.clone();
        assert_eq!(n.round_to_multiple_assign(v.clone(), rm), o);
        assert_eq!(n.to_string(), quotient);
        assert!(n.is_valid());

        let mut n = u.clone();
        assert_eq!(n.round_to_multiple_assign(&v, rm), o);
        assert_eq!(n.to_string(), quotient);
        assert!(n.is_valid());

        let (r, o_alt) = u.clone().round_to_multiple(v.clone(), rm);
        assert!(r.is_valid());
        assert_eq!(r.to_string(), quotient);
        assert_eq!(o_alt, o);

        let (r, o_alt) = u.clone().round_to_multiple(&v, rm);
        assert!(r.is_valid());
        assert_eq!(r.to_string(), quotient);
        assert_eq!(o_alt, o);

        let (r, o_alt) = (&u).round_to_multiple(v.clone(), rm);
        assert!(r.is_valid());
        assert_eq!(r.to_string(), quotient);
        assert_eq!(o_alt, o);

        let (r, o_alt) = (&u).round_to_multiple(&v, rm);
        assert!(r.is_valid());
        assert_eq!(r.to_string(), quotient);
        assert_eq!(o_alt, o);
    };
    test("0", "1", Down, "0", Equal);
    test("0", "1", Floor, "0", Equal);
    test("0", "1", Up, "0", Equal);
    test("0", "1", Ceiling, "0", Equal);
    test("0", "1", Nearest, "0", Equal);
    test("0", "1", Exact, "0", Equal);

    test("0", "22/7", Down, "0", Equal);
    test("0", "22/7", Floor, "0", Equal);
    test("0", "22/7", Up, "0", Equal);
    test("0", "22/7", Ceiling, "0", Equal);
    test("0", "22/7", Nearest, "0", Equal);
    test("0", "22/7", Exact, "0", Equal);

    test("1/3", "1", Down, "0", Less);
    test("1/3", "1", Floor, "0", Less);
    test("1/3", "1", Up, "1", Greater);
    test("1/3", "1", Ceiling, "1", Greater);
    test("1/3", "1", Nearest, "0", Less);

    test("1/3", "1/3", Down, "1/3", Equal);
    test("1/3", "1/3", Floor, "1/3", Equal);
    test("1/3", "1/3", Up, "1/3", Equal);
    test("1/3", "1/3", Ceiling, "1/3", Equal);
    test("1/3", "1/3", Nearest, "1/3", Equal);
    test("1/3", "1/3", Exact, "1/3", Equal);

    test("1/3", "1/4", Down, "1/4", Less);
    test("1/3", "1/4", Floor, "1/4", Less);
    test("1/3", "1/4", Up, "1/2", Greater);
    test("1/3", "1/4", Ceiling, "1/2", Greater);
    test("1/3", "1/4", Nearest, "1/4", Less);

    test(
        "884279719003555/281474976710656",
        "1/1000000",
        Down,
        "392699/125000",
        Less,
    );
    test(
        "884279719003555/281474976710656",
        "1/1000000",
        Floor,
        "392699/125000",
        Less,
    );
    test(
        "884279719003555/281474976710656",
        "1/1000000",
        Up,
        "3141593/1000000",
        Greater,
    );
    test(
        "884279719003555/281474976710656",
        "1/1000000",
        Ceiling,
        "3141593/1000000",
        Greater,
    );
    test(
        "884279719003555/281474976710656",
        "1/1000000",
        Nearest,
        "3141593/1000000",
        Greater,
    );

    test(
        "1000000",
        "884279719003555/281474976710656",
        Down,
        "281474193076302588495/281474976710656",
        Less,
    );
    test(
        "1000000",
        "884279719003555/281474976710656",
        Floor,
        "281474193076302588495/281474976710656",
        Less,
    );
    test(
        "1000000",
        "884279719003555/281474976710656",
        Up,
        "140737538678010796025/140737488355328",
        Greater,
    );
    test(
        "1000000",
        "884279719003555/281474976710656",
        Ceiling,
        "140737538678010796025/140737488355328",
        Greater,
    );
    test(
        "1000000",
        "884279719003555/281474976710656",
        Nearest,
        "140737538678010796025/140737488355328",
        Greater,
    );

    test("-1/3", "1", Down, "0", Greater);
    test("-1/3", "1", Floor, "-1", Less);
    test("-1/3", "1", Up, "-1", Less);
    test("-1/3", "1", Ceiling, "0", Greater);
    test("-1/3", "1", Nearest, "0", Greater);

    test("-1/3", "1/3", Down, "-1/3", Equal);
    test("-1/3", "1/3", Floor, "-1/3", Equal);
    test("-1/3", "1/3", Up, "-1/3", Equal);
    test("-1/3", "1/3", Ceiling, "-1/3", Equal);
    test("-1/3", "1/3", Nearest, "-1/3", Equal);
    test("-1/3", "1/3", Exact, "-1/3", Equal);

    test("-1/3", "1/4", Down, "-1/4", Greater);
    test("-1/3", "1/4", Floor, "-1/2", Less);
    test("-1/3", "1/4", Up, "-1/2", Less);
    test("-1/3", "1/4", Ceiling, "-1/4", Greater);
    test("-1/3", "1/4", Nearest, "-1/4", Greater);

    test(
        "-884279719003555/281474976710656",
        "1/1000000",
        Down,
        "-392699/125000",
        Greater,
    );
    test(
        "-884279719003555/281474976710656",
        "1/1000000",
        Floor,
        "-3141593/1000000",
        Less,
    );
    test(
        "-884279719003555/281474976710656",
        "1/1000000",
        Up,
        "-3141593/1000000",
        Less,
    );
    test(
        "-884279719003555/281474976710656",
        "1/1000000",
        Ceiling,
        "-392699/125000",
        Greater,
    );
    test(
        "-884279719003555/281474976710656",
        "1/1000000",
        Nearest,
        "-3141593/1000000",
        Less,
    );

    test(
        "-1000000",
        "884279719003555/281474976710656",
        Down,
        "-281474193076302588495/281474976710656",
        Greater,
    );
    test(
        "-1000000",
        "884279719003555/281474976710656",
        Floor,
        "-140737538678010796025/140737488355328",
        Less,
    );
    test(
        "-1000000",
        "884279719003555/281474976710656",
        Up,
        "-140737538678010796025/140737488355328",
        Less,
    );
    test(
        "-1000000",
        "884279719003555/281474976710656",
        Ceiling,
        "-281474193076302588495/281474976710656",
        Greater,
    );
    test(
        "-1000000",
        "884279719003555/281474976710656",
        Nearest,
        "-140737538678010796025/140737488355328",
        Less,
    );

    test("0", "-1", Down, "0", Equal);
    test("0", "-1", Floor, "0", Equal);
    test("0", "-1", Up, "0", Equal);
    test("0", "-1", Ceiling, "0", Equal);
    test("0", "-1", Nearest, "0", Equal);
    test("0", "-1", Exact, "0", Equal);

    test("0", "-22/7", Down, "0", Equal);
    test("0", "-22/7", Floor, "0", Equal);
    test("0", "-22/7", Up, "0", Equal);
    test("0", "-22/7", Ceiling, "0", Equal);
    test("0", "-22/7", Nearest, "0", Equal);
    test("0", "-22/7", Exact, "0", Equal);

    test("1/3", "-1", Down, "0", Less);
    test("1/3", "-1", Floor, "0", Less);
    test("1/3", "-1", Up, "1", Greater);
    test("1/3", "-1", Ceiling, "1", Greater);
    test("1/3", "-1", Nearest, "0", Less);

    test("1/3", "-1/3", Down, "1/3", Equal);
    test("1/3", "-1/3", Floor, "1/3", Equal);
    test("1/3", "-1/3", Up, "1/3", Equal);
    test("1/3", "-1/3", Ceiling, "1/3", Equal);
    test("1/3", "-1/3", Nearest, "1/3", Equal);
    test("1/3", "-1/3", Exact, "1/3", Equal);

    test("1/3", "-1/4", Down, "1/4", Less);
    test("1/3", "-1/4", Floor, "1/4", Less);
    test("1/3", "-1/4", Up, "1/2", Greater);
    test("1/3", "-1/4", Ceiling, "1/2", Greater);
    test("1/3", "-1/4", Nearest, "1/4", Less);

    test(
        "884279719003555/281474976710656",
        "-1/1000000",
        Down,
        "392699/125000",
        Less,
    );
    test(
        "884279719003555/281474976710656",
        "-1/1000000",
        Floor,
        "392699/125000",
        Less,
    );
    test(
        "884279719003555/281474976710656",
        "-1/1000000",
        Up,
        "3141593/1000000",
        Greater,
    );
    test(
        "884279719003555/281474976710656",
        "-1/1000000",
        Ceiling,
        "3141593/1000000",
        Greater,
    );
    test(
        "884279719003555/281474976710656",
        "-1/1000000",
        Nearest,
        "3141593/1000000",
        Greater,
    );

    test(
        "1000000",
        "-884279719003555/281474976710656",
        Down,
        "281474193076302588495/281474976710656",
        Less,
    );
    test(
        "1000000",
        "-884279719003555/281474976710656",
        Floor,
        "281474193076302588495/281474976710656",
        Less,
    );
    test(
        "1000000",
        "-884279719003555/281474976710656",
        Up,
        "140737538678010796025/140737488355328",
        Greater,
    );
    test(
        "1000000",
        "-884279719003555/281474976710656",
        Ceiling,
        "140737538678010796025/140737488355328",
        Greater,
    );
    test(
        "1000000",
        "-884279719003555/281474976710656",
        Nearest,
        "140737538678010796025/140737488355328",
        Greater,
    );

    test("-1/3", "-1", Down, "0", Greater);
    test("-1/3", "-1", Floor, "-1", Less);
    test("-1/3", "-1", Up, "-1", Less);
    test("-1/3", "-1", Ceiling, "0", Greater);
    test("-1/3", "-1", Nearest, "0", Greater);

    test("-1/3", "-1/3", Down, "-1/3", Equal);
    test("-1/3", "-1/3", Floor, "-1/3", Equal);
    test("-1/3", "-1/3", Up, "-1/3", Equal);
    test("-1/3", "-1/3", Ceiling, "-1/3", Equal);
    test("-1/3", "-1/3", Nearest, "-1/3", Equal);
    test("-1/3", "-1/3", Exact, "-1/3", Equal);

    test("-1/3", "-1/4", Down, "-1/4", Greater);
    test("-1/3", "-1/4", Floor, "-1/2", Less);
    test("-1/3", "-1/4", Up, "-1/2", Less);
    test("-1/3", "-1/4", Ceiling, "-1/4", Greater);
    test("-1/3", "-1/4", Nearest, "-1/4", Greater);

    test(
        "-884279719003555/281474976710656",
        "-1/1000000",
        Down,
        "-392699/125000",
        Greater,
    );
    test(
        "-884279719003555/281474976710656",
        "-1/1000000",
        Floor,
        "-3141593/1000000",
        Less,
    );
    test(
        "-884279719003555/281474976710656",
        "-1/1000000",
        Up,
        "-3141593/1000000",
        Less,
    );
    test(
        "-884279719003555/281474976710656",
        "-1/1000000",
        Ceiling,
        "-392699/125000",
        Greater,
    );
    test(
        "-884279719003555/281474976710656",
        "-1/1000000",
        Nearest,
        "-3141593/1000000",
        Less,
    );

    test(
        "-1000000",
        "-884279719003555/281474976710656",
        Down,
        "-281474193076302588495/281474976710656",
        Greater,
    );
    test(
        "-1000000",
        "-884279719003555/281474976710656",
        Floor,
        "-140737538678010796025/140737488355328",
        Less,
    );
    test(
        "-1000000",
        "-884279719003555/281474976710656",
        Up,
        "-140737538678010796025/140737488355328",
        Less,
    );
    test(
        "-1000000",
        "-884279719003555/281474976710656",
        Ceiling,
        "-281474193076302588495/281474976710656",
        Greater,
    );
    test(
        "-1000000",
        "-884279719003555/281474976710656",
        Nearest,
        "-140737538678010796025/140737488355328",
        Less,
    );

    test("1/3", "0", Down, "0", Less);
    test("1/3", "0", Floor, "0", Less);
    test("1/3", "0", Nearest, "0", Less);
    test("-1/3", "0", Down, "0", Greater);
    test("-1/3", "0", Ceiling, "0", Greater);
    test("-1/3", "0", Nearest, "0", Greater);
}

#[test]
#[should_panic]
fn round_to_multiple_assign_fail_1() {
    let mut n = Rational::from(10);
    n.round_to_multiple_assign(Rational::from(3), Exact);
}

#[test]
#[should_panic]
fn round_to_multiple_assign_fail_2() {
    let mut n = Rational::from(10);
    n.round_to_multiple_assign(Rational::ZERO, Ceiling);
}

#[test]
#[should_panic]
fn round_to_multiple_assign_fail_3() {
    let mut n = Rational::from(10);
    n.round_to_multiple_assign(Rational::ZERO, Up);
}

#[test]
#[should_panic]
fn round_to_multiple_assign_fail_4() {
    let mut n = Rational::from(10);
    n.round_to_multiple_assign(Rational::ZERO, Exact);
}

#[test]
#[should_panic]
fn round_to_multiple_assign_ref_fail_1() {
    let mut n = Rational::from(10);
    n.round_to_multiple_assign(&Rational::from(3), Exact);
}

#[test]
#[should_panic]
fn round_to_multiple_assign_ref_fail_2() {
    let mut n = Rational::from(10);
    n.round_to_multiple_assign(&Rational::ZERO, Ceiling);
}

#[test]
#[should_panic]
fn round_to_multiple_assign_ref_fail_3() {
    let mut n = Rational::from(10);
    n.round_to_multiple_assign(&Rational::ZERO, Up);
}

#[test]
#[should_panic]
fn round_to_multiple_assign_ref_fail_4() {
    let mut n = Rational::from(10);
    n.round_to_multiple_assign(&Rational::ZERO, Exact);
}

#[test]
#[should_panic]
fn round_to_multiple_fail_1() {
    Rational::from(10).round_to_multiple(Rational::from(3), Exact);
}

#[test]
#[should_panic]
fn round_to_multiple_fail_2() {
    Rational::from(10).round_to_multiple(Rational::ZERO, Ceiling);
}

#[test]
#[should_panic]
fn round_to_multiple_fail_3() {
    Rational::from(10).round_to_multiple(Rational::ZERO, Up);
}

#[test]
#[should_panic]
fn round_to_multiple_fail_4() {
    Rational::from(10).round_to_multiple(Rational::ZERO, Exact);
}

#[test]
#[should_panic]
fn round_to_multiple_val_ref_fail_1() {
    Rational::from(10).round_to_multiple(&Rational::from(3), Exact);
}

#[test]
#[should_panic]
fn round_to_multiple_val_ref_fail_2() {
    Rational::from(10).round_to_multiple(&Rational::ZERO, Ceiling);
}

#[test]
#[should_panic]
fn round_to_multiple_val_ref_fail_3() {
    Rational::from(10).round_to_multiple(&Rational::ZERO, Up);
}

#[test]
#[should_panic]
fn round_to_multiple_val_ref_fail_4() {
    Rational::from(10).round_to_multiple(&Rational::ZERO, Exact);
}

#[test]
#[should_panic]
fn round_to_multiple_ref_val_fail_1() {
    (&Rational::from(10)).round_to_multiple(Rational::from(3), Exact);
}

#[test]
#[should_panic]
fn round_to_multiple_ref_val_fail_2() {
    (&Rational::from(10)).round_to_multiple(Rational::ZERO, Ceiling);
}

#[test]
#[should_panic]
fn round_to_multiple_ref_val_fail_3() {
    (&Rational::from(10)).round_to_multiple(Rational::ZERO, Up);
}

#[test]
#[should_panic]
fn round_to_multiple_ref_val_fail_4() {
    (&Rational::from(10)).round_to_multiple(Rational::ZERO, Exact);
}

#[test]
#[should_panic]
fn round_to_multiple_ref_ref_fail_1() {
    (&Rational::from(10)).round_to_multiple(&Rational::from(3), Exact);
}

#[test]
#[should_panic]
fn round_to_multiple_ref_ref_fail_2() {
    (&Rational::from(10)).round_to_multiple(&Rational::ZERO, Ceiling);
}

#[test]
#[should_panic]
fn round_to_multiple_ref_ref_fail_3() {
    (&Rational::from(10)).round_to_multiple(&Rational::ZERO, Up);
}

#[test]
#[should_panic]
fn round_to_multiple_ref_ref_fail_4() {
    (&Rational::from(10)).round_to_multiple(&Rational::ZERO, Exact);
}

#[test]
fn round_to_multiple_properties() {
    rational_rational_rounding_mode_triple_gen_var_1().test_properties(|(x, y, rm)| {
        let mut mut_n = x.clone();
        let o = mut_n.round_to_multiple_assign(&y, rm);
        assert!(mut_n.is_valid());
        let r = mut_n;

        let mut mut_n = x.clone();
        assert_eq!(mut_n.round_to_multiple_assign(y.clone(), rm), o);
        assert!(mut_n.is_valid());
        assert_eq!(mut_n, r);

        let (r_alt, o_alt) = (&x).round_to_multiple(&y, rm);
        assert!(r_alt.is_valid());
        assert_eq!(r_alt, r);
        assert_eq!(o_alt, o);

        let (r_alt, o_alt) = (&x).round_to_multiple(y.clone(), rm);
        assert!(r_alt.is_valid());
        assert_eq!(r_alt, r);
        assert_eq!(o_alt, o);

        let (r_alt, o_alt) = x.clone().round_to_multiple(&y, rm);
        assert!(r_alt.is_valid());
        assert_eq!(r_alt, r);
        assert_eq!(o_alt, o);

        let (r_alt, o_alt) = x.clone().round_to_multiple(y.clone(), rm);
        assert!(r_alt.is_valid());
        assert_eq!(r_alt, r);
        assert_eq!(o_alt, o);

        let (r_alt, o_alt) = (-&x).round_to_multiple(&y, -rm);
        assert_eq!(-&r_alt, r);
        assert_eq!(o_alt.reverse(), o);

        let (r_alt, o_alt) = (&x).round_to_multiple(-&y, rm);
        assert_eq!(r_alt, r);
        assert_eq!(o_alt, o);

        assert_eq!(r.cmp(&x), o);
        match (x >= 0, rm) {
            (_, Floor) | (true, Down) | (false, Up) => {
                assert_ne!(o, Greater);
            }
            (_, Ceiling) | (true, Up) | (false, Down) => {
                assert_ne!(o, Less);
            }
            (_, Exact) => assert_eq!(o, Equal),
            _ => {}
        }
        if y == 0 {
            assert_eq!(r, 0);
        } else {
            assert!((&r / &y).is_integer());
            assert!((&r - &x).le_abs(&y));
            match rm {
                Floor => assert!(r <= x),
                Ceiling => assert!(r >= x),
                Down => assert!(r.le_abs(&x)),
                Up => assert!(r.ge_abs(&x)),
                Exact => assert_eq!(r, x),
                Nearest => {
                    let closest;
                    let second_closest;
                    if r <= x {
                        closest = &x - &r;
                        second_closest = &r + (&y).abs() - &x;
                    } else {
                        closest = &r - &x;
                        second_closest = x + (&y).abs() - &r;
                    }
                    assert!(closest <= second_closest);
                    if closest == second_closest {
                        assert!(Integer::exact_from(r / y).even());
                    }
                }
            }
        }
    });

    rational_pair_gen_var_1().test_properties(|(x, y)| {
        let rounded = x.round_to_multiple(&y, Nearest).0;
        let ro = (rounded.clone(), Equal);
        assert_eq!((&rounded).round_to_multiple(&y, Down), ro);
        assert_eq!((&rounded).round_to_multiple(&y, Up), ro);
        assert_eq!((&rounded).round_to_multiple(&y, Floor), ro);
        assert_eq!((&rounded).round_to_multiple(&y, Ceiling), ro);
        assert_eq!((&rounded).round_to_multiple(&y, Nearest), ro);
        assert_eq!((&rounded).round_to_multiple(&y, Exact), ro);
    });

    rational_pair_gen_var_2().test_properties(|(x, y)| {
        let down = (&x).round_to_multiple(&y, Down);
        assert_eq!(down.1, if x >= 0 { Less } else { Greater });
        let up = if x >= 0 {
            (&down.0 + (&y).abs(), Greater)
        } else {
            (&down.0 - (&y).abs(), Less)
        };
        let floor = (&x).round_to_multiple(&y, Floor);
        let ceiling = (&floor.0 + (&y).abs(), Greater);
        assert_eq!((&x).round_to_multiple(&y, Up), up);
        assert_eq!((&x).round_to_multiple(&y, Ceiling), ceiling);
        let nearest = x.round_to_multiple(y, Nearest);
        assert!(nearest == down || nearest == up);
    });

    integer_rounding_mode_pair_gen().test_properties(|(x, rm)| {
        let x = Rational::from(x);
        let x = &x;
        let xo = (x.clone(), Equal);
        assert_eq!(x.round_to_multiple(Rational::ONE, rm), xo);
        assert_eq!(x.round_to_multiple(Rational::NEGATIVE_ONE, rm), xo);
        assert_eq!(
            Rational::ZERO.round_to_multiple(x, rm),
            (Rational::ZERO, Equal)
        );
        assert_eq!(x.round_to_multiple(x, rm), xo);
        assert_eq!(x.round_to_multiple(-x, rm), xo);
        assert_eq!((-x).round_to_multiple(x, rm), (-x, Equal));
    });

    integer_integer_rounding_mode_triple_gen_var_2().test_properties(|(x, y, rm)| {
        let (n, no) = (&x).round_to_multiple(&y, rm);
        let (r, ro) = Rational::from(x).round_to_multiple(Rational::from(y), rm);
        assert_eq!(n, r);
        assert_eq!(no, ro);
    });
}
