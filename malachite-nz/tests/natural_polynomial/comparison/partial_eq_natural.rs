// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use core::str::FromStr;
use malachite_base::num::basic::traits::Zero;
use malachite_nz::natural::Natural;
use malachite_nz::natural_polynomial::NaturalPolynomial;
use malachite_nz::test_util::generators::{
    natural_gen, natural_polynomial_gen, natural_polynomial_natural_pair_gen,
};

#[test]
fn test_partial_eq_natural() {
    let test = |s, t, out| {
        let p = NaturalPolynomial::from_str(s).unwrap();
        let n = Natural::from_str(t).unwrap();
        assert_eq!(p == n, out);
        assert_eq!(n == p, out);
    };
    // The zero polynomial equals 0 and nothing else.
    test("0", "0", true);
    test("0", "1", false);
    test("1", "1", true);
    test("1", "0", false);
    test("123", "123", true);
    test("123", "5", false);
    test(
        "1000000000000000000000000",
        "1000000000000000000000000",
        true,
    );
    test(
        "1000000000000000000000000",
        "1000000000000000000000001",
        false,
    );
    test("1000000000000000000000000", "0", false);
    // No polynomial of positive degree equals a Natural, whatever its constant term.
    test("x", "0", false);
    test("x", "1", false);
    test("x+1", "1", false);
    test(
        "2*x^2+1000000000000000000000000",
        "1000000000000000000000000",
        false,
    );
}

// Comparing with a converted value is the reference the direct comparison is checked against.
#[allow(clippy::cmp_owned)]
#[test]
fn partial_eq_natural_properties() {
    natural_polynomial_natural_pair_gen().test_properties(|(p, n)| {
        let eq = p == n;
        assert_eq!(n == p, eq);
        assert_eq!(p == NaturalPolynomial::from(n.clone()), eq);
        if eq {
            assert!(p.degree().is_none_or(|d| d == 0));
        }
    });

    natural_gen().test_properties(|n| {
        let p = NaturalPolynomial::from(n.clone());
        assert!(p == n);
        assert!(n == p);
        assert_eq!(NaturalPolynomial::ZERO == n, n == 0u32);
        assert_eq!(NaturalPolynomial::one() == n, n == 1u32);
        assert!(NaturalPolynomial::x() != n);
    });

    natural_polynomial_gen().test_properties(|p| {
        // Comparing with a Natural agrees with comparing with a primitive of the same value.
        assert_eq!(p == Natural::ZERO, p == 0u32);
        if let Some(d) = p.degree()
            && d > 0
        {
            assert!(p != *p.coefficient(0));
        }
    });
}
