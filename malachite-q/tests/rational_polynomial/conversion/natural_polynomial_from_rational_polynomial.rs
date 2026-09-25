// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use core::str::FromStr;
use malachite_base::num::basic::traits::Zero;
use malachite_base::num::conversion::traits::ConvertibleFrom;
use malachite_base::polynomial::Polynomial;
use malachite_nz::integer_polynomial::IntegerPolynomial;
use malachite_nz::natural_polynomial::NaturalPolynomial;
use malachite_nz::test_util::generators::natural_polynomial_gen;
use malachite_q::rational_polynomial::RationalPolynomial;
use malachite_q::rational_polynomial::conversion::natural_polynomial_from_rational_polynomial::*;
use malachite_q::test_util::generators::rational_polynomial_gen;

#[test]
fn test_natural_polynomial_from_rational_polynomial() {
    let test = |s, out: Option<&str>| {
        let p = RationalPolynomial::from_str(s).unwrap();
        let q = NaturalPolynomial::try_from(&p);
        assert_eq!(q.as_ref().ok().map(ToString::to_string).as_deref(), out);
        if let Ok(q) = &q {
            assert!(q.is_valid());
        } else {
            assert_eq!(q, Err(NaturalPolynomialFromRationalPolynomialError));
        }
        assert_eq!(NaturalPolynomial::try_from(p), q);
    };
    test("0", Some("0"));
    test("1", Some("1"));
    test("4/2", Some("2"));
    test("x", Some("x"));
    test("3*x^2+5", Some("3*x^2+5"));
    // A negative coefficient, or one that is not an integer, is enough.
    test("-1", None);
    test("3*x^2-5", None);
    test("1/2", None);
    test("1/2*x+1", None);
    test("-1/2*x+1", None);
}

#[test]
fn natural_polynomial_from_rational_polynomial_properties() {
    rational_polynomial_gen().test_properties(|p| {
        let q = NaturalPolynomial::try_from(&p);
        assert_eq!(NaturalPolynomial::try_from(p.clone()), q);
        // Converting through an IntegerPolynomial gives the same answer.
        assert_eq!(
            IntegerPolynomial::try_from(&p)
                .ok()
                .and_then(|i| NaturalPolynomial::try_from(i).ok()),
            q.clone().ok()
        );
        if let Ok(q) = q {
            assert!(q.is_valid());
            assert!(p == q);
            assert_eq!(q.degree(), p.degree());
            assert_eq!(RationalPolynomial::from(q), p);
        }
    });

    natural_polynomial_gen().test_properties(|q| {
        // Converting to a RationalPolynomial and back is the identity.
        assert_eq!(
            NaturalPolynomial::try_from(RationalPolynomial::from(q.clone())),
            Ok(q)
        );
    });

    assert_eq!(
        NaturalPolynomial::try_from(RationalPolynomial::ZERO),
        Ok(NaturalPolynomial::ZERO)
    );
    assert!(NaturalPolynomial::try_from(RationalPolynomial::negative_one()).is_err());
    assert!(NaturalPolynomial::try_from(RationalPolynomial::one_half()).is_err());
}

#[test]
fn test_natural_polynomial_convertible_from_rational_polynomial() {
    let test = |s, out| {
        let p = RationalPolynomial::from_str(s).unwrap();
        assert_eq!(NaturalPolynomial::convertible_from(&p), out);
    };
    test("3*x^2+1", true);
    test("3*x^2-1", false);
    test("3*x^2+1/2", false);
    test("0", true);
}

#[test]
fn natural_polynomial_convertible_from_rational_polynomial_properties() {
    rational_polynomial_gen().test_properties(|p| {
        assert_eq!(
            NaturalPolynomial::convertible_from(&p),
            NaturalPolynomial::try_from(&p).is_ok()
        );
    });
}
