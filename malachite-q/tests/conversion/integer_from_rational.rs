use malachite_base::num::arithmetic::traits::{Ceiling, Floor, Parity};
use malachite_base::num::basic::traits::{One, OneHalf, Two};
use malachite_base::num::comparison::traits::PartialOrdAbs;
use malachite_base::num::conversion::traits::{
    CheckedFrom, ConvertibleFrom, ExactFrom, IsInteger, RoundingFrom,
};
use malachite_base::rounding_modes::RoundingMode;
use malachite_base::strings::ToDebugString;
use malachite_nz::integer::Integer;
use malachite_nz_test_util::generators::integer_gen;
use malachite_q::Rational;
use malachite_q_test_util::generators::{rational_gen, rational_rounding_mode_pair_gen_var_2};
use std::str::FromStr;

#[test]
fn test_checked_from_rational() {
    let test = |s, out| {
        let u = Rational::from_str(s).unwrap();

        let on = Integer::checked_from(u.clone());
        assert_eq!(on.to_debug_string(), out);
        assert!(on.map_or(true, |n| n.is_valid()));

        let on = Integer::checked_from(&u);
        assert_eq!(on.to_debug_string(), out);
        assert!(on.map_or(true, |n| n.is_valid()));
    };
    test("0", "Some(0)");
    test("123", "Some(123)");
    test("-123", "Some(-123)");
    test("1000000000000", "Some(1000000000000)");
    test("-1000000000000", "Some(-1000000000000)");
    test("22/7", "None");
    test("-22/7", "None");
}

#[test]
fn test_exact_from_rational() {
    let test = |s, out| {
        let u = Rational::from_str(s).unwrap();

        let n = Integer::exact_from(u.clone());
        assert_eq!(n.to_string(), out);
        assert!(n.is_valid());

        let n = Integer::exact_from(&u);
        assert_eq!(n.to_string(), out);
        assert!(n.is_valid());
    };
    test("0", "0");
    test("123", "123");
    test("-123", "-123");
    test("1000000000000", "1000000000000");
    test("-1000000000000", "-1000000000000");
}

#[test]
#[should_panic]
fn integer_exact_from_rational_fail() {
    Integer::exact_from(Rational::from_str("22/7").unwrap());
}

#[test]
#[should_panic]
fn integer_exact_from_rational_ref_fail() {
    Integer::exact_from(&Rational::from_str("-22/7").unwrap());
}

#[test]
fn test_convertible_from_rational() {
    let test = |s, out| {
        let u = Rational::from_str(s).unwrap();
        assert_eq!(Integer::convertible_from(&u), out);
    };
    test("0", true);
    test("123", true);
    test("-123", true);
    test("1000000000000", true);
    test("-1000000000000", true);
    test("22/7", false);
    test("-22/7", false);
}

#[test]
fn test_rounding_from_rational() {
    let test = |s, rm, out| {
        let u = Rational::from_str(s).unwrap();

        let n = Integer::rounding_from(u.clone(), rm);
        assert_eq!(n.to_string(), out);
        assert!(n.is_valid());

        let n = Integer::rounding_from(&u, rm);
        assert_eq!(n.to_string(), out);
        assert!(n.is_valid());
    };
    test("123", RoundingMode::Floor, "123");
    test("123", RoundingMode::Down, "123");
    test("123", RoundingMode::Ceiling, "123");
    test("123", RoundingMode::Up, "123");
    test("123", RoundingMode::Nearest, "123");
    test("123", RoundingMode::Exact, "123");

    test("-123", RoundingMode::Floor, "-123");
    test("-123", RoundingMode::Down, "-123");
    test("-123", RoundingMode::Ceiling, "-123");
    test("-123", RoundingMode::Up, "-123");
    test("-123", RoundingMode::Nearest, "-123");
    test("-123", RoundingMode::Exact, "-123");

    test("22/7", RoundingMode::Floor, "3");
    test("22/7", RoundingMode::Down, "3");
    test("22/7", RoundingMode::Ceiling, "4");
    test("22/7", RoundingMode::Up, "4");
    test("22/7", RoundingMode::Nearest, "3");

    test("-22/7", RoundingMode::Floor, "-4");
    test("-22/7", RoundingMode::Down, "-3");
    test("-22/7", RoundingMode::Ceiling, "-3");
    test("-22/7", RoundingMode::Up, "-4");
    test("-22/7", RoundingMode::Nearest, "-3");

    test("7/2", RoundingMode::Floor, "3");
    test("7/2", RoundingMode::Down, "3");
    test("7/2", RoundingMode::Ceiling, "4");
    test("7/2", RoundingMode::Up, "4");
    test("7/2", RoundingMode::Nearest, "4");

    test("9/2", RoundingMode::Floor, "4");
    test("9/2", RoundingMode::Down, "4");
    test("9/2", RoundingMode::Ceiling, "5");
    test("9/2", RoundingMode::Up, "5");
    test("9/2", RoundingMode::Nearest, "4");
}

#[test]
#[should_panic]
fn integer_rounding_from_rational_fail() {
    Integer::rounding_from(Rational::from_str("22/7").unwrap(), RoundingMode::Exact);
}

#[test]
#[should_panic]
fn integer_rounding_from_rational_ref_fail() {
    Integer::rounding_from(&Rational::from_str("22/7").unwrap(), RoundingMode::Exact);
}

#[test]
fn checked_from_rational_properties() {
    rational_gen().test_properties(|x| {
        let integer_x = Integer::checked_from(x.clone());
        assert!(integer_x.as_ref().map_or(true, Integer::is_valid));

        let integer_x_alt = Integer::checked_from(&x);
        assert!(integer_x_alt.as_ref().map_or(true, Integer::is_valid));
        assert_eq!(integer_x, integer_x_alt);

        assert_eq!(integer_x.is_some(), x.is_integer());
        assert_eq!(integer_x.is_some(), Integer::convertible_from(&x));
        if let Some(n) = integer_x {
            assert_eq!(n.to_string(), x.to_string());
            assert_eq!(Integer::exact_from(&x), n);
            assert_eq!(Rational::from(&n), x);
            assert_eq!(Rational::from(n), x);
        }
    });
}

#[test]
fn convertible_from_rational_properties() {
    rational_gen().test_properties(|x| {
        let convertible = Integer::convertible_from(&x);
        assert_eq!(convertible, x.is_integer());
    });
}

#[test]
fn integer_from_rational_properties() {
    rational_rounding_mode_pair_gen_var_2().test_properties(|(x, rm)| {
        let n = Integer::rounding_from(&x, rm);
        assert_eq!(Integer::rounding_from(x.clone(), rm), n);
        assert!((Rational::from(n) - x).lt_abs(&1));
    });

    rational_gen().test_properties(|x| {
        let floor = Integer::rounding_from(&x, RoundingMode::Floor);
        assert_eq!(floor, (&x).floor());
        assert!(floor <= x);
        assert!(&floor + Integer::ONE > x);

        let ceiling = Integer::rounding_from(&x, RoundingMode::Ceiling);
        assert_eq!(ceiling, (&x).ceiling());
        assert!(ceiling >= x);
        assert!(&ceiling - Integer::ONE < x);

        if x >= 0 {
            assert_eq!(Integer::rounding_from(&x, RoundingMode::Down), floor);
            assert_eq!(Integer::rounding_from(&x, RoundingMode::Up), ceiling);
        } else {
            assert_eq!(Integer::rounding_from(&x, RoundingMode::Down), ceiling);
            assert_eq!(Integer::rounding_from(&x, RoundingMode::Up), floor);
        }

        let nearest = Integer::rounding_from(&x, RoundingMode::Nearest);
        assert!(nearest == floor || nearest == ceiling);
        assert!((Rational::from(nearest) - x).le_abs(&Rational::ONE_HALF));
    });

    integer_gen().test_properties(|n| {
        let x = Rational::from(&n);
        assert_eq!(Integer::rounding_from(&x, RoundingMode::Floor), n);
        assert_eq!(Integer::rounding_from(&x, RoundingMode::Down), n);
        assert_eq!(Integer::rounding_from(&x, RoundingMode::Ceiling), n);
        assert_eq!(Integer::rounding_from(&x, RoundingMode::Up), n);
        assert_eq!(Integer::rounding_from(&x, RoundingMode::Nearest), n);
        assert_eq!(Integer::rounding_from(&x, RoundingMode::Exact), n);

        let x = Rational::from_integers((n << 1) | Integer::ONE, Integer::TWO);
        assert!(Integer::rounding_from(x, RoundingMode::Nearest).even());
    });
}
