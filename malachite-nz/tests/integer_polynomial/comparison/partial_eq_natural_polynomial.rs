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
use malachite_nz::natural_polynomial::NaturalPolynomial;
use malachite_nz::test_util::generators::integer_polynomial_natural_polynomial_pair_gen;

#[test]
fn test_partial_eq_natural_polynomial() {
    let test = |s, t, out| {
        let p = IntegerPolynomial::from_str(s).unwrap();
        let q = NaturalPolynomial::from_str(t).unwrap();
        assert_eq!(p == q, out);
        assert_eq!(q == p, out);
    };
    test("0", "0", true);
    test("0", "1", false);
    test("1", "1", true);
    test("-1", "1", false);
    test("x", "x", true);
    test("x-1", "x+1", false);
    test("-x", "x", false);
    test("3*x^2+5", "3*x^2+5", true);
    test("-3*x^2+5", "3*x^2+5", false);
    test("2*x^2+3", "3*x^2+2", false);
    test(
        "1000000000000000000000000*x+1",
        "1000000000000000000000000*x+1",
        true,
    );
    test(
        "-1000000000000000000000000*x+1",
        "1000000000000000000000000*x+1",
        false,
    );
    // The same coefficients at different degrees: lengths differ.
    test("x^3+x", "x^2+x", false);
}

// Comparing with a converted polynomial is the reference the direct comparison is checked against.
#[allow(clippy::cmp_owned)]
#[test]
fn partial_eq_natural_polynomial_properties() {
    integer_polynomial_natural_polynomial_pair_gen().test_properties(|(p, q)| {
        let eq = p == q;
        assert_eq!(q == p, eq);
        assert_eq!(p == IntegerPolynomial::from(q.clone()), eq);
        if eq {
            assert_eq!(p.degree(), q.degree());
        }

        let q_p = IntegerPolynomial::from(q.clone());
        assert!(q_p == q);
        assert!(q == q_p);
    });

    assert!(IntegerPolynomial::ZERO == NaturalPolynomial::ZERO);
    assert!(IntegerPolynomial::one() == NaturalPolynomial::one());
    assert!(IntegerPolynomial::x() == NaturalPolynomial::x());
    assert!(IntegerPolynomial::negative_one() != NaturalPolynomial::one());
}
