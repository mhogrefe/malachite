// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use core::str::FromStr;
use malachite_base::num::basic::traits::Zero;
use malachite_nz::integer_polynomial::IntegerPolynomial;
use malachite_nz::integer_polynomial::conversion::natural_polynomial_from_integer_polynomial::*;
use malachite_nz::natural_polynomial::NaturalPolynomial;
use malachite_nz::test_util::generators::{integer_polynomial_gen, natural_polynomial_gen};

#[test]
fn test_natural_polynomial_from_integer_polynomial() {
    let test = |s, out: Option<&str>| {
        let p = IntegerPolynomial::from_str(s).unwrap();
        let q = NaturalPolynomial::try_from(&p);
        assert_eq!(q.as_ref().ok().map(ToString::to_string).as_deref(), out);
        if let Ok(q) = &q {
            assert!(q.is_valid());
        } else {
            assert_eq!(q, Err(NaturalPolynomialFromIntegerPolynomialError));
        }
        assert_eq!(NaturalPolynomial::try_from(p), q);
    };
    test("0", Some("0"));
    test("1", Some("1"));
    test("-1", None);
    test("x", Some("x"));
    test("-x", None);
    test("3*x^2+5", Some("3*x^2+5"));
    // One negative coefficient is enough, wherever it is.
    test("3*x^2-5", None);
    test("-3*x^2+5", None);
    test("x^3-x+1", None);
    test(
        "1000000000000000000000000*x+1",
        Some("1000000000000000000000000*x+1"),
    );
    test("-1000000000000000000000000*x+1", None);
}

#[test]
fn natural_polynomial_from_integer_polynomial_properties() {
    integer_polynomial_gen().test_properties(|p| {
        let q = NaturalPolynomial::try_from(&p);
        assert_eq!(NaturalPolynomial::try_from(p.clone()), q);
        // The conversion succeeds exactly when no coefficient is negative.
        assert_eq!(q.is_ok(), p.coefficients_asc().iter().all(|c| *c >= 0u32));
        if let Ok(q) = q {
            assert!(q.is_valid());
            assert!(p == q);
            assert_eq!(q.degree(), p.degree());
            assert_eq!(IntegerPolynomial::from(q), p);
        }
    });

    natural_polynomial_gen().test_properties(|q| {
        // Converting to an IntegerPolynomial and back is the identity.
        assert_eq!(
            NaturalPolynomial::try_from(IntegerPolynomial::from(q.clone())),
            Ok(q)
        );
    });

    assert_eq!(
        NaturalPolynomial::try_from(IntegerPolynomial::ZERO),
        Ok(NaturalPolynomial::ZERO)
    );
    assert!(NaturalPolynomial::try_from(IntegerPolynomial::negative_one()).is_err());
}
