use malachite_base::num::arithmetic::traits::{
    Abs, Parity, RoundToMultiple, RoundToMultipleAssign,
};
use malachite_base::num::basic::traits::{NegativeOne, One, Zero};
use malachite_base::num::comparison::traits::PartialOrdAbs;
use malachite_base::num::conversion::traits::{ExactFrom, IsInteger};
use malachite_base::rounding_modes::RoundingMode;
use malachite_nz::integer::Integer;
use malachite_nz::test_util::generators::{
    integer_integer_rounding_mode_triple_gen_var_2, integer_rounding_mode_pair_gen,
};
use malachite_q::test_util::generators::{
    rational_pair_gen_var_1, rational_pair_gen_var_2,
    rational_rational_rounding_mode_triple_gen_var_1,
};
use malachite_q::Rational;
use std::str::FromStr;

#[test]
fn test_round_to_multiple() {
    let test = |s, t, rm, quotient| {
        let u = Rational::from_str(s).unwrap();
        let v = Rational::from_str(t).unwrap();

        let mut n = u.clone();
        n.round_to_multiple_assign(v.clone(), rm);
        assert_eq!(n.to_string(), quotient);
        assert!(n.is_valid());

        let r = u.clone().round_to_multiple(v.clone(), rm);
        assert!(r.is_valid());
        assert_eq!(r.to_string(), quotient);

        let r = u.clone().round_to_multiple(&v, rm);
        assert!(r.is_valid());
        assert_eq!(r.to_string(), quotient);

        let r = (&u).round_to_multiple(v.clone(), rm);
        assert!(r.is_valid());
        assert_eq!(r.to_string(), quotient);

        let r = (&u).round_to_multiple(&v, rm);
        assert!(r.is_valid());
        assert_eq!(r.to_string(), quotient);
    };
    test("0", "1", RoundingMode::Down, "0");
    test("0", "1", RoundingMode::Floor, "0");
    test("0", "1", RoundingMode::Up, "0");
    test("0", "1", RoundingMode::Ceiling, "0");
    test("0", "1", RoundingMode::Nearest, "0");
    test("0", "1", RoundingMode::Exact, "0");

    test("0", "22/7", RoundingMode::Down, "0");
    test("0", "22/7", RoundingMode::Floor, "0");
    test("0", "22/7", RoundingMode::Up, "0");
    test("0", "22/7", RoundingMode::Ceiling, "0");
    test("0", "22/7", RoundingMode::Nearest, "0");
    test("0", "22/7", RoundingMode::Exact, "0");

    test("1/3", "1", RoundingMode::Down, "0");
    test("1/3", "1", RoundingMode::Floor, "0");
    test("1/3", "1", RoundingMode::Up, "1");
    test("1/3", "1", RoundingMode::Ceiling, "1");
    test("1/3", "1", RoundingMode::Nearest, "0");

    test("1/3", "1/3", RoundingMode::Down, "1/3");
    test("1/3", "1/3", RoundingMode::Floor, "1/3");
    test("1/3", "1/3", RoundingMode::Up, "1/3");
    test("1/3", "1/3", RoundingMode::Ceiling, "1/3");
    test("1/3", "1/3", RoundingMode::Nearest, "1/3");
    test("1/3", "1/3", RoundingMode::Exact, "1/3");

    test("1/3", "1/4", RoundingMode::Down, "1/4");
    test("1/3", "1/4", RoundingMode::Floor, "1/4");
    test("1/3", "1/4", RoundingMode::Up, "1/2");
    test("1/3", "1/4", RoundingMode::Ceiling, "1/2");
    test("1/3", "1/4", RoundingMode::Nearest, "1/4");

    test(
        "884279719003555/281474976710656",
        "1/1000000",
        RoundingMode::Down,
        "392699/125000",
    );
    test(
        "884279719003555/281474976710656",
        "1/1000000",
        RoundingMode::Floor,
        "392699/125000",
    );
    test(
        "884279719003555/281474976710656",
        "1/1000000",
        RoundingMode::Up,
        "3141593/1000000",
    );
    test(
        "884279719003555/281474976710656",
        "1/1000000",
        RoundingMode::Ceiling,
        "3141593/1000000",
    );
    test(
        "884279719003555/281474976710656",
        "1/1000000",
        RoundingMode::Nearest,
        "3141593/1000000",
    );

    test(
        "1000000",
        "884279719003555/281474976710656",
        RoundingMode::Down,
        "281474193076302588495/281474976710656",
    );
    test(
        "1000000",
        "884279719003555/281474976710656",
        RoundingMode::Floor,
        "281474193076302588495/281474976710656",
    );
    test(
        "1000000",
        "884279719003555/281474976710656",
        RoundingMode::Up,
        "140737538678010796025/140737488355328",
    );
    test(
        "1000000",
        "884279719003555/281474976710656",
        RoundingMode::Ceiling,
        "140737538678010796025/140737488355328",
    );
    test(
        "1000000",
        "884279719003555/281474976710656",
        RoundingMode::Nearest,
        "140737538678010796025/140737488355328",
    );

    test("-1/3", "1", RoundingMode::Down, "0");
    test("-1/3", "1", RoundingMode::Floor, "-1");
    test("-1/3", "1", RoundingMode::Up, "-1");
    test("-1/3", "1", RoundingMode::Ceiling, "0");
    test("-1/3", "1", RoundingMode::Nearest, "0");

    test("-1/3", "1/3", RoundingMode::Down, "-1/3");
    test("-1/3", "1/3", RoundingMode::Floor, "-1/3");
    test("-1/3", "1/3", RoundingMode::Up, "-1/3");
    test("-1/3", "1/3", RoundingMode::Ceiling, "-1/3");
    test("-1/3", "1/3", RoundingMode::Nearest, "-1/3");
    test("-1/3", "1/3", RoundingMode::Exact, "-1/3");

    test("-1/3", "1/4", RoundingMode::Down, "-1/4");
    test("-1/3", "1/4", RoundingMode::Floor, "-1/2");
    test("-1/3", "1/4", RoundingMode::Up, "-1/2");
    test("-1/3", "1/4", RoundingMode::Ceiling, "-1/4");
    test("-1/3", "1/4", RoundingMode::Nearest, "-1/4");

    test(
        "-884279719003555/281474976710656",
        "1/1000000",
        RoundingMode::Down,
        "-392699/125000",
    );
    test(
        "-884279719003555/281474976710656",
        "1/1000000",
        RoundingMode::Floor,
        "-3141593/1000000",
    );
    test(
        "-884279719003555/281474976710656",
        "1/1000000",
        RoundingMode::Up,
        "-3141593/1000000",
    );
    test(
        "-884279719003555/281474976710656",
        "1/1000000",
        RoundingMode::Ceiling,
        "-392699/125000",
    );
    test(
        "-884279719003555/281474976710656",
        "1/1000000",
        RoundingMode::Nearest,
        "-3141593/1000000",
    );

    test(
        "-1000000",
        "884279719003555/281474976710656",
        RoundingMode::Down,
        "-281474193076302588495/281474976710656",
    );
    test(
        "-1000000",
        "884279719003555/281474976710656",
        RoundingMode::Floor,
        "-140737538678010796025/140737488355328",
    );
    test(
        "-1000000",
        "884279719003555/281474976710656",
        RoundingMode::Up,
        "-140737538678010796025/140737488355328",
    );
    test(
        "-1000000",
        "884279719003555/281474976710656",
        RoundingMode::Ceiling,
        "-281474193076302588495/281474976710656",
    );
    test(
        "-1000000",
        "884279719003555/281474976710656",
        RoundingMode::Nearest,
        "-140737538678010796025/140737488355328",
    );

    test("0", "-1", RoundingMode::Down, "0");
    test("0", "-1", RoundingMode::Floor, "0");
    test("0", "-1", RoundingMode::Up, "0");
    test("0", "-1", RoundingMode::Ceiling, "0");
    test("0", "-1", RoundingMode::Nearest, "0");
    test("0", "-1", RoundingMode::Exact, "0");

    test("0", "-22/7", RoundingMode::Down, "0");
    test("0", "-22/7", RoundingMode::Floor, "0");
    test("0", "-22/7", RoundingMode::Up, "0");
    test("0", "-22/7", RoundingMode::Ceiling, "0");
    test("0", "-22/7", RoundingMode::Nearest, "0");
    test("0", "-22/7", RoundingMode::Exact, "0");

    test("1/3", "-1", RoundingMode::Down, "0");
    test("1/3", "-1", RoundingMode::Floor, "0");
    test("1/3", "-1", RoundingMode::Up, "1");
    test("1/3", "-1", RoundingMode::Ceiling, "1");
    test("1/3", "-1", RoundingMode::Nearest, "0");

    test("1/3", "-1/3", RoundingMode::Down, "1/3");
    test("1/3", "-1/3", RoundingMode::Floor, "1/3");
    test("1/3", "-1/3", RoundingMode::Up, "1/3");
    test("1/3", "-1/3", RoundingMode::Ceiling, "1/3");
    test("1/3", "-1/3", RoundingMode::Nearest, "1/3");
    test("1/3", "-1/3", RoundingMode::Exact, "1/3");

    test("1/3", "-1/4", RoundingMode::Down, "1/4");
    test("1/3", "-1/4", RoundingMode::Floor, "1/4");
    test("1/3", "-1/4", RoundingMode::Up, "1/2");
    test("1/3", "-1/4", RoundingMode::Ceiling, "1/2");
    test("1/3", "-1/4", RoundingMode::Nearest, "1/4");

    test(
        "884279719003555/281474976710656",
        "-1/1000000",
        RoundingMode::Down,
        "392699/125000",
    );
    test(
        "884279719003555/281474976710656",
        "-1/1000000",
        RoundingMode::Floor,
        "392699/125000",
    );
    test(
        "884279719003555/281474976710656",
        "-1/1000000",
        RoundingMode::Up,
        "3141593/1000000",
    );
    test(
        "884279719003555/281474976710656",
        "-1/1000000",
        RoundingMode::Ceiling,
        "3141593/1000000",
    );
    test(
        "884279719003555/281474976710656",
        "-1/1000000",
        RoundingMode::Nearest,
        "3141593/1000000",
    );

    test(
        "1000000",
        "-884279719003555/281474976710656",
        RoundingMode::Down,
        "281474193076302588495/281474976710656",
    );
    test(
        "1000000",
        "-884279719003555/281474976710656",
        RoundingMode::Floor,
        "281474193076302588495/281474976710656",
    );
    test(
        "1000000",
        "-884279719003555/281474976710656",
        RoundingMode::Up,
        "140737538678010796025/140737488355328",
    );
    test(
        "1000000",
        "-884279719003555/281474976710656",
        RoundingMode::Ceiling,
        "140737538678010796025/140737488355328",
    );
    test(
        "1000000",
        "-884279719003555/281474976710656",
        RoundingMode::Nearest,
        "140737538678010796025/140737488355328",
    );

    test("-1/3", "-1", RoundingMode::Down, "0");
    test("-1/3", "-1", RoundingMode::Floor, "-1");
    test("-1/3", "-1", RoundingMode::Up, "-1");
    test("-1/3", "-1", RoundingMode::Ceiling, "0");
    test("-1/3", "-1", RoundingMode::Nearest, "0");

    test("-1/3", "-1/3", RoundingMode::Down, "-1/3");
    test("-1/3", "-1/3", RoundingMode::Floor, "-1/3");
    test("-1/3", "-1/3", RoundingMode::Up, "-1/3");
    test("-1/3", "-1/3", RoundingMode::Ceiling, "-1/3");
    test("-1/3", "-1/3", RoundingMode::Nearest, "-1/3");
    test("-1/3", "-1/3", RoundingMode::Exact, "-1/3");

    test("-1/3", "-1/4", RoundingMode::Down, "-1/4");
    test("-1/3", "-1/4", RoundingMode::Floor, "-1/2");
    test("-1/3", "-1/4", RoundingMode::Up, "-1/2");
    test("-1/3", "-1/4", RoundingMode::Ceiling, "-1/4");
    test("-1/3", "-1/4", RoundingMode::Nearest, "-1/4");

    test(
        "-884279719003555/281474976710656",
        "-1/1000000",
        RoundingMode::Down,
        "-392699/125000",
    );
    test(
        "-884279719003555/281474976710656",
        "-1/1000000",
        RoundingMode::Floor,
        "-3141593/1000000",
    );
    test(
        "-884279719003555/281474976710656",
        "-1/1000000",
        RoundingMode::Up,
        "-3141593/1000000",
    );
    test(
        "-884279719003555/281474976710656",
        "-1/1000000",
        RoundingMode::Ceiling,
        "-392699/125000",
    );
    test(
        "-884279719003555/281474976710656",
        "-1/1000000",
        RoundingMode::Nearest,
        "-3141593/1000000",
    );

    test(
        "-1000000",
        "-884279719003555/281474976710656",
        RoundingMode::Down,
        "-281474193076302588495/281474976710656",
    );
    test(
        "-1000000",
        "-884279719003555/281474976710656",
        RoundingMode::Floor,
        "-140737538678010796025/140737488355328",
    );
    test(
        "-1000000",
        "-884279719003555/281474976710656",
        RoundingMode::Up,
        "-140737538678010796025/140737488355328",
    );
    test(
        "-1000000",
        "-884279719003555/281474976710656",
        RoundingMode::Ceiling,
        "-281474193076302588495/281474976710656",
    );
    test(
        "-1000000",
        "-884279719003555/281474976710656",
        RoundingMode::Nearest,
        "-140737538678010796025/140737488355328",
    );
}

#[test]
#[should_panic]
fn round_to_multiple_assign_fail_1() {
    let mut n = Rational::from(10);
    n.round_to_multiple_assign(Rational::from(3), RoundingMode::Exact);
}

#[test]
#[should_panic]
fn round_to_multiple_assign_fail_2() {
    let mut n = Rational::from(10);
    n.round_to_multiple_assign(Rational::ZERO, RoundingMode::Ceiling);
}

#[test]
#[should_panic]
fn round_to_multiple_assign_fail_3() {
    let mut n = Rational::from(10);
    n.round_to_multiple_assign(Rational::ZERO, RoundingMode::Up);
}

#[test]
#[should_panic]
fn round_to_multiple_assign_fail_4() {
    let mut n = Rational::from(10);
    n.round_to_multiple_assign(Rational::ZERO, RoundingMode::Exact);
}

#[test]
#[should_panic]
fn round_to_multiple_assign_ref_fail_1() {
    let mut n = Rational::from(10);
    n.round_to_multiple_assign(&Rational::from(3), RoundingMode::Exact);
}

#[test]
#[should_panic]
fn round_to_multiple_assign_ref_fail_2() {
    let mut n = Rational::from(10);
    n.round_to_multiple_assign(&Rational::ZERO, RoundingMode::Ceiling);
}

#[test]
#[should_panic]
fn round_to_multiple_assign_ref_fail_3() {
    let mut n = Rational::from(10);
    n.round_to_multiple_assign(&Rational::ZERO, RoundingMode::Up);
}

#[test]
#[should_panic]
fn round_to_multiple_assign_ref_fail_4() {
    let mut n = Rational::from(10);
    n.round_to_multiple_assign(&Rational::ZERO, RoundingMode::Exact);
}

#[test]
#[should_panic]
fn round_to_multiple_fail_1() {
    Rational::from(10).round_to_multiple(Rational::from(3), RoundingMode::Exact);
}

#[test]
#[should_panic]
fn round_to_multiple_fail_2() {
    Rational::from(10).round_to_multiple(Rational::ZERO, RoundingMode::Ceiling);
}

#[test]
#[should_panic]
fn round_to_multiple_fail_3() {
    Rational::from(10).round_to_multiple(Rational::ZERO, RoundingMode::Up);
}

#[test]
#[should_panic]
fn round_to_multiple_fail_4() {
    Rational::from(10).round_to_multiple(Rational::ZERO, RoundingMode::Exact);
}

#[test]
#[should_panic]
fn round_to_multiple_val_ref_fail_1() {
    Rational::from(10).round_to_multiple(&Rational::from(3), RoundingMode::Exact);
}

#[test]
#[should_panic]
fn round_to_multiple_val_ref_fail_2() {
    Rational::from(10).round_to_multiple(&Rational::ZERO, RoundingMode::Ceiling);
}

#[test]
#[should_panic]
fn round_to_multiple_val_ref_fail_3() {
    Rational::from(10).round_to_multiple(&Rational::ZERO, RoundingMode::Up);
}

#[test]
#[should_panic]
fn round_to_multiple_val_ref_fail_4() {
    Rational::from(10).round_to_multiple(&Rational::ZERO, RoundingMode::Exact);
}

#[test]
#[should_panic]
fn round_to_multiple_ref_val_fail_1() {
    (&Rational::from(10)).round_to_multiple(Rational::from(3), RoundingMode::Exact);
}

#[test]
#[should_panic]
fn round_to_multiple_ref_val_fail_2() {
    (&Rational::from(10)).round_to_multiple(Rational::ZERO, RoundingMode::Ceiling);
}

#[test]
#[should_panic]
fn round_to_multiple_ref_val_fail_3() {
    (&Rational::from(10)).round_to_multiple(Rational::ZERO, RoundingMode::Up);
}

#[test]
#[should_panic]
fn round_to_multiple_ref_val_fail_4() {
    (&Rational::from(10)).round_to_multiple(Rational::ZERO, RoundingMode::Exact);
}

#[test]
#[should_panic]
fn round_to_multiple_ref_ref_fail_1() {
    (&Rational::from(10)).round_to_multiple(&Rational::from(3), RoundingMode::Exact);
}

#[test]
#[should_panic]
fn round_to_multiple_ref_ref_fail_2() {
    (&Rational::from(10)).round_to_multiple(&Rational::ZERO, RoundingMode::Ceiling);
}

#[test]
#[should_panic]
fn round_to_multiple_ref_ref_fail_3() {
    (&Rational::from(10)).round_to_multiple(&Rational::ZERO, RoundingMode::Up);
}

#[test]
#[should_panic]
fn round_to_multiple_ref_ref_fail_4() {
    (&Rational::from(10)).round_to_multiple(&Rational::ZERO, RoundingMode::Exact);
}

#[test]
fn round_to_multiple_properties() {
    rational_rational_rounding_mode_triple_gen_var_1().test_properties(|(x, y, rm)| {
        let mut mut_n = x.clone();
        mut_n.round_to_multiple_assign(&y, rm);
        assert!(mut_n.is_valid());
        let r = mut_n;

        let mut mut_n = x.clone();
        mut_n.round_to_multiple_assign(y.clone(), rm);
        assert!(mut_n.is_valid());
        assert_eq!(mut_n, r);

        let r_alt = (&x).round_to_multiple(&y, rm);
        assert!(r_alt.is_valid());
        assert_eq!(r_alt, r);

        let r_alt = (&x).round_to_multiple(y.clone(), rm);
        assert!(r_alt.is_valid());
        assert_eq!(r_alt, r);

        let r_alt = x.clone().round_to_multiple(&y, rm);
        assert!(r_alt.is_valid());
        assert_eq!(r_alt, r);

        let r_alt = x.clone().round_to_multiple(y.clone(), rm);
        assert!(r_alt.is_valid());
        assert_eq!(r_alt, r);

        assert_eq!(-(-&x).round_to_multiple(&y, -rm), r);
        assert_eq!((&x).round_to_multiple(-&y, rm), r);
        if y == 0 {
            assert_eq!(r, 0);
        } else {
            assert!((&r / &y).is_integer());
            assert!((&r - &x).le_abs(&y));
            match rm {
                RoundingMode::Floor => assert!(r <= x),
                RoundingMode::Ceiling => assert!(r >= x),
                RoundingMode::Down => assert!(r.le_abs(&x)),
                RoundingMode::Up => assert!(r.ge_abs(&x)),
                RoundingMode::Exact => assert_eq!(r, x),
                RoundingMode::Nearest => {
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
        let rounded = x.round_to_multiple(&y, RoundingMode::Nearest);
        assert_eq!(
            (&rounded).round_to_multiple(&y, RoundingMode::Down),
            rounded
        );
        assert_eq!((&rounded).round_to_multiple(&y, RoundingMode::Up), rounded);
        assert_eq!(
            (&rounded).round_to_multiple(&y, RoundingMode::Floor),
            rounded
        );
        assert_eq!(
            (&rounded).round_to_multiple(&y, RoundingMode::Ceiling),
            rounded
        );
        assert_eq!(
            (&rounded).round_to_multiple(&y, RoundingMode::Nearest),
            rounded
        );
        assert_eq!(
            (&rounded).round_to_multiple(&y, RoundingMode::Exact),
            rounded
        );
    });

    rational_pair_gen_var_2().test_properties(|(x, y)| {
        let down = (&x).round_to_multiple(&y, RoundingMode::Down);
        let up = if x >= 0 {
            &down + (&y).abs()
        } else {
            &down - (&y).abs()
        };
        let floor = (&x).round_to_multiple(&y, RoundingMode::Floor);
        let ceiling = &floor + (&y).abs();
        assert_eq!((&x).round_to_multiple(&y, RoundingMode::Up), up);
        assert_eq!((&x).round_to_multiple(&y, RoundingMode::Ceiling), ceiling);
        let nearest = x.round_to_multiple(y, RoundingMode::Nearest);
        assert!(nearest == down || nearest == up);
    });

    integer_rounding_mode_pair_gen().test_properties(|(x, rm)| {
        let x = Rational::from(x);
        let x = &x;
        assert_eq!(x.round_to_multiple(Rational::ONE, rm), *x);
        assert_eq!(x.round_to_multiple(Rational::NEGATIVE_ONE, rm), *x);
        assert_eq!(Rational::ZERO.round_to_multiple(x, rm), 0);
        assert_eq!(x.round_to_multiple(x, rm), *x);
        assert_eq!(x.round_to_multiple(-x, rm), *x);
        assert_eq!((-x).round_to_multiple(x, rm), -x);
    });

    integer_integer_rounding_mode_triple_gen_var_2().test_properties(|(x, y, rm)| {
        assert_eq!(
            Rational::from(&x).round_to_multiple(Rational::from(&y), rm),
            x.round_to_multiple(y, rm)
        );
    });
}
