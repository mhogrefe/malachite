// Copyright © 2024 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::assert_panic;
use malachite_base::num::arithmetic::traits::{Ceiling, Floor, Parity};
use malachite_base::num::basic::traits::{One, OneHalf, Two};
use malachite_base::num::comparison::traits::PartialOrdAbs;
use malachite_base::num::conversion::traits::{
    ConvertibleFrom, ExactFrom, IsInteger, RoundingFrom,
};
use malachite_base::rounding_modes::RoundingMode;
use malachite_base::strings::ToDebugString;
use malachite_nz::integer::Integer;
use malachite_nz::natural::Natural;
use malachite_nz::test_util::generators::natural_gen;
use malachite_q::test_util::generators::{
    rational_gen, rational_gen_var_3, rational_rounding_mode_pair_gen_var_1,
};
use malachite_q::Rational;
use std::cmp::Ordering;
use std::panic::catch_unwind;
use std::str::FromStr;

#[test]
fn test_try_from_rational() {
    let test = |s, out| {
        let u = Rational::from_str(s).unwrap();

        let on = Natural::try_from(u.clone());
        assert_eq!(on.to_debug_string(), out);
        assert!(on.map_or(true, |n| n.is_valid()));

        let on = Natural::try_from(&u);
        assert_eq!(on.to_debug_string(), out);
        assert!(on.map_or(true, |n| n.is_valid()));
    };
    test("0", "Ok(0)");
    test("123", "Ok(123)");
    test("-123", "Err(NaturalFromRationalError)");
    test("1000000000000", "Ok(1000000000000)");
    test("-1000000000000", "Err(NaturalFromRationalError)");
    test("22/7", "Err(NaturalFromRationalError)");
    test("-22/7", "Err(NaturalFromRationalError)");
}

#[test]
fn test_exact_from_rational() {
    let test = |s, out| {
        let u = Rational::from_str(s).unwrap();

        let n = Natural::exact_from(u.clone());
        assert_eq!(n.to_string(), out);
        assert!(n.is_valid());

        let n = Natural::exact_from(&u);
        assert_eq!(n.to_string(), out);
        assert!(n.is_valid());
    };
    test("0", "0");
    test("123", "123");
    test("1000000000000", "1000000000000");
}

#[test]
#[should_panic]
fn natural_exact_from_rational_fail_1() {
    Natural::exact_from(Rational::from_str("-123").unwrap());
}

#[test]
#[should_panic]
fn natural_exact_from_rational_fail_2() {
    Natural::exact_from(Rational::from_str("22/7").unwrap());
}

#[test]
#[should_panic]
fn natural_exact_from_rational_ref_fail_1() {
    Natural::exact_from(&Rational::from_str("-123").unwrap());
}

#[test]
#[should_panic]
fn natural_exact_from_rational_ref_fail_2() {
    Natural::exact_from(&Rational::from_str("22/7").unwrap());
}

#[test]
fn test_convertible_from_rational() {
    let test = |s, out| {
        let u = Rational::from_str(s).unwrap();
        assert_eq!(Natural::convertible_from(&u), out);
    };
    test("0", true);
    test("123", true);
    test("-123", false);
    test("1000000000000", true);
    test("-1000000000000", false);
    test("22/7", false);
    test("-22/7", false);
}

#[test]
fn test_rounding_from_rational() {
    let test = |s, rm, out, o_out| {
        let u = Rational::from_str(s).unwrap();

        let (n, o) = Natural::rounding_from(u.clone(), rm);
        assert_eq!(n.to_string(), out);
        assert!(n.is_valid());
        assert_eq!(o, o_out);

        let (n, o) = Natural::rounding_from(&u, rm);
        assert_eq!(n.to_string(), out);
        assert!(n.is_valid());
        assert_eq!(o, o_out);
    };
    test("123", RoundingMode::Floor, "123", Ordering::Equal);
    test("123", RoundingMode::Down, "123", Ordering::Equal);
    test("123", RoundingMode::Ceiling, "123", Ordering::Equal);
    test("123", RoundingMode::Up, "123", Ordering::Equal);
    test("123", RoundingMode::Nearest, "123", Ordering::Equal);
    test("123", RoundingMode::Exact, "123", Ordering::Equal);

    test("-123", RoundingMode::Down, "0", Ordering::Greater);
    test("-123", RoundingMode::Ceiling, "0", Ordering::Greater);
    test("-123", RoundingMode::Nearest, "0", Ordering::Greater);

    test("22/7", RoundingMode::Floor, "3", Ordering::Less);
    test("22/7", RoundingMode::Down, "3", Ordering::Less);
    test("22/7", RoundingMode::Ceiling, "4", Ordering::Greater);
    test("22/7", RoundingMode::Up, "4", Ordering::Greater);
    test("22/7", RoundingMode::Nearest, "3", Ordering::Less);

    test("7/2", RoundingMode::Floor, "3", Ordering::Less);
    test("7/2", RoundingMode::Down, "3", Ordering::Less);
    test("7/2", RoundingMode::Ceiling, "4", Ordering::Greater);
    test("7/2", RoundingMode::Up, "4", Ordering::Greater);
    test("7/2", RoundingMode::Nearest, "4", Ordering::Greater);

    test("9/2", RoundingMode::Floor, "4", Ordering::Less);
    test("9/2", RoundingMode::Down, "4", Ordering::Less);
    test("9/2", RoundingMode::Ceiling, "5", Ordering::Greater);
    test("9/2", RoundingMode::Up, "5", Ordering::Greater);
    test("9/2", RoundingMode::Nearest, "4", Ordering::Less);
}

#[test]
fn natural_rounding_from_rational_fail() {
    let x = Rational::from_str("-123").unwrap();
    assert_panic!(Natural::rounding_from(x.clone(), RoundingMode::Floor));
    assert_panic!(Natural::rounding_from(x.clone(), RoundingMode::Up));
    assert_panic!(Natural::rounding_from(x, RoundingMode::Exact));

    let x = Rational::from_str("22/7").unwrap();
    assert_panic!(Natural::rounding_from(x, RoundingMode::Exact));
}

#[test]
fn natural_rounding_from_rational_ref_fail() {
    let x = Rational::from_str("-123").unwrap();
    assert_panic!(Natural::rounding_from(&x, RoundingMode::Floor));
    assert_panic!(Natural::rounding_from(&x, RoundingMode::Up));
    assert_panic!(Natural::rounding_from(&x, RoundingMode::Exact));

    let x = Rational::from_str("22/7").unwrap();
    assert_panic!(Natural::rounding_from(&x, RoundingMode::Exact));
}

#[test]
fn try_from_rational_properties() {
    rational_gen().test_properties(|x| {
        let natural_x = Natural::try_from(x.clone());
        assert!(natural_x.as_ref().map_or(true, Natural::is_valid));

        let natural_x_alt = Natural::try_from(&x);
        assert!(natural_x_alt.as_ref().map_or(true, Natural::is_valid));
        assert_eq!(natural_x, natural_x_alt);

        assert_eq!(natural_x.is_ok(), x >= 0 && x.is_integer());
        assert_eq!(natural_x.is_ok(), Natural::convertible_from(&x));
        if let Ok(n) = natural_x {
            assert_eq!(n.to_string(), x.to_string());
            assert_eq!(Natural::exact_from(&x), n);
            assert_eq!(Rational::from(&n), x);
            assert_eq!(Rational::from(n), x);
        }
    });
}

#[test]
fn convertible_from_rational_properties() {
    rational_gen().test_properties(|x| {
        let convertible = Natural::convertible_from(&x);
        assert_eq!(convertible, x >= 0 && x.is_integer());
    });
}

#[test]
fn rounding_from_rational_properties() {
    rational_rounding_mode_pair_gen_var_1().test_properties(|(x, rm)| {
        let no = Natural::rounding_from(&x, rm);
        assert_eq!(Natural::rounding_from(x.clone(), rm), no);
        let (n, o) = no;
        if x >= 0 {
            assert_eq!(Integer::rounding_from(&x, rm), (Integer::from(&n), o));
            assert!((Rational::from(&n) - &x).lt_abs(&1));
        }

        assert_eq!(n.partial_cmp(&x), Some(o));
        match (x >= 0, rm) {
            (_, RoundingMode::Floor) | (true, RoundingMode::Down) | (false, RoundingMode::Up) => {
                assert_ne!(o, Ordering::Greater)
            }
            (_, RoundingMode::Ceiling) | (true, RoundingMode::Up) | (false, RoundingMode::Down) => {
                assert_ne!(o, Ordering::Less)
            }
            (_, RoundingMode::Exact) => assert_eq!(o, Ordering::Equal),
            _ => {}
        }
    });

    rational_gen_var_3().test_properties(|x| {
        let floor = Natural::rounding_from(&x, RoundingMode::Floor);
        assert_eq!(floor.0, (&x).floor());
        assert!(floor.0 <= x);
        assert!(&floor.0 + Natural::ONE > x);
        assert_eq!(Natural::rounding_from(&x, RoundingMode::Down), floor);

        let ceiling = Natural::rounding_from(&x, RoundingMode::Ceiling);
        assert_eq!(ceiling.0, (&x).ceiling());
        assert!(ceiling.0 >= x);
        if x > 0 {
            assert!(&ceiling.0 - Natural::ONE < x);
        }
        assert_eq!(Natural::rounding_from(&x, RoundingMode::Up), ceiling);

        let nearest = Natural::rounding_from(&x, RoundingMode::Nearest);
        assert!(nearest == floor || nearest == ceiling);
        assert!((Rational::from(nearest.0) - x).le_abs(&Rational::ONE_HALF));
    });

    natural_gen().test_properties(|n| {
        let x = Rational::from(&n);
        let no = (n, Ordering::Equal);
        assert_eq!(Natural::rounding_from(&x, RoundingMode::Floor), no);
        assert_eq!(Natural::rounding_from(&x, RoundingMode::Down), no);
        assert_eq!(Natural::rounding_from(&x, RoundingMode::Ceiling), no);
        assert_eq!(Natural::rounding_from(&x, RoundingMode::Up), no);
        assert_eq!(Natural::rounding_from(&x, RoundingMode::Nearest), no);
        assert_eq!(Natural::rounding_from(&x, RoundingMode::Exact), no);

        let x = Rational::from_naturals((no.0 << 1) | Natural::ONE, Natural::TWO);
        assert!(Natural::rounding_from(x, RoundingMode::Nearest).0.even());
    });
}
