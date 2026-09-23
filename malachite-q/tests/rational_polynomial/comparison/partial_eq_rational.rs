// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use core::str::FromStr;
use malachite_base::num::basic::traits::Zero;
use malachite_base::polynomial::Polynomial;
use malachite_q::Rational;
use malachite_q::rational_polynomial::RationalPolynomial;
use malachite_q::test_util::generators::{
    rational_gen, rational_polynomial_gen, rational_polynomial_rational_pair_gen,
};

#[test]
fn test_partial_eq_rational() {
    let test = |s, t, out| {
        let p = RationalPolynomial::from_str(s).unwrap();
        let r = Rational::from_str(t).unwrap();
        assert_eq!(p == r, out);
        assert_eq!(r == p, out);
    };
    // The zero polynomial equals 0 and nothing else.
    test("0", "0", true);
    test("0", "1/2", false);
    test("1", "1", true);
    test("-1", "1", false);
    test("1/2", "1/2", true);
    // A sign, numerator or denominator differing is enough.
    test("1/2", "-1/2", false);
    test("-1/2", "1/2", false);
    test("1/2", "1/3", false);
    test("1/2", "3/2", false);
    test("-22/7", "-22/7", true);
    test("-22/7", "-22/9", false);
    test(
        "1/1000000000000000000000000",
        "1/1000000000000000000000000",
        true,
    );
    test(
        "1/1000000000000000000000000",
        "1/1000000000000000000000001",
        false,
    );
    // No polynomial of positive degree equals a Rational, whatever its constant term.
    test("x", "0", false);
    test("1/2*x+1/2", "1/2", false);
}

// Comparing with a converted value is the reference the direct comparison is checked against.
#[allow(clippy::cmp_owned)]
#[test]
fn partial_eq_rational_properties() {
    rational_polynomial_rational_pair_gen().test_properties(|(p, r)| {
        let eq = p == r;
        assert_eq!(r == p, eq);
        assert_eq!(p == RationalPolynomial::from(r.clone()), eq);
        if eq {
            assert!(p.degree().is_none_or(|d| d == 0));
        }
    });

    rational_gen().test_properties(|r| {
        let p = RationalPolynomial::from(r.clone());
        assert!(p == r);
        assert!(r == p);
        assert_eq!(RationalPolynomial::ZERO == r, r == 0u32);
        assert_eq!(RationalPolynomial::one() == r, r == 1u32);
        assert_eq!(RationalPolynomial::negative_one() == r, r == -1i32);
        assert_eq!(
            RationalPolynomial::one_half() == r,
            r == Rational::from_unsigneds(1u32, 2)
        );
        assert!(RationalPolynomial::x() != r);
    });

    rational_polynomial_gen().test_properties(|p| {
        // Comparing with a Rational agrees with comparing with a primitive of the same value.
        assert_eq!(p == Rational::ZERO, p == 0u32);
        assert_eq!(p == Rational::from(-5), p == -5i32);
        if let Some(d) = p.degree()
            && d > 0
        {
            assert!(p != p.coefficient(0));
        }
    });
}
