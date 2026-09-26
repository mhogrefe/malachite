// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use core::str::FromStr;
use malachite_base::num::arithmetic::traits::{CanonicalizeUnit, CanonicalizeUnitAssign};
use malachite_base::num::basic::traits::Zero;
use malachite_base::polynomial::{MakeMonic, Polynomial};
use malachite_q::rational_polynomial::RationalPolynomial;
use malachite_q::test_util::generators::rational_polynomial_gen;

#[test]
fn test_canonicalize_unit() {
    let test = |s, out| {
        let p = RationalPolynomial::from_str(s).unwrap();
        let q = p.clone().canonicalize_unit();
        assert!(q.is_valid());
        assert_eq!(q.to_string(), out);
        assert_eq!((&p).canonicalize_unit(), q);
        let mut r = p;
        r.canonicalize_unit_assign();
        assert_eq!(r, q);
    };
    test("0", "0");
    test("-1/2", "1");
    test("3", "1");
    test("-1/2*x^2+3", "x^2-6");
    test("1/2*x^2-3", "x^2-6");
    test("-2/3*x-4/3", "x+2");
    test("6*x+4", "x+2/3");
    test("x^2-1/3", "x^2-1/3");
}

#[test]
fn canonicalize_unit_properties() {
    rational_polynomial_gen().test_properties(|p| {
        let q = (&p).canonicalize_unit();
        assert!(q.is_valid());
        assert_eq!(p.clone().canonicalize_unit(), q);
        let mut r = p.clone();
        r.canonicalize_unit_assign();
        assert_eq!(r, q);

        // It is the monic multiple, and zero only for the zero polynomial.
        assert_eq!((&p).make_monic(), q);
        assert_eq!(q == RationalPolynomial::ZERO, p == RationalPolynomial::ZERO);
        if p != RationalPolynomial::ZERO {
            assert!(q.is_monic());
        }
        // Canonicalizing again changes nothing, and p and -p have the same canonical form.
        assert_eq!((&q).canonicalize_unit(), q);
        assert_eq!((-&p).canonicalize_unit(), q);
        // Every nonzero constant multiple has the same canonical form.
        for c in ["2", "-1/3", "7/5"] {
            let c = RationalPolynomial::from_str(c).unwrap();
            if p != RationalPolynomial::ZERO {
                let scaled = RationalPolynomial::from_coefficients_asc(
                    p.to_coefficients_asc()
                        .into_iter()
                        .map(|x| x * c.coefficient(0))
                        .collect(),
                );
                assert_eq!(scaled.canonicalize_unit(), q);
            }
        }
    });
}
