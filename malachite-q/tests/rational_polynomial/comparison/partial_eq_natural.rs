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
use malachite_nz::integer::Integer;
use malachite_nz::natural::Natural;
use malachite_nz::test_util::generators::natural_gen;
use malachite_q::Rational;
use malachite_q::rational_polynomial::RationalPolynomial;
use malachite_q::test_util::generators::rational_polynomial_natural_pair_gen;

#[test]
fn test_partial_eq_natural() {
    let test = |s, t, out| {
        let p = RationalPolynomial::from_str(s).unwrap();
        let n = Natural::from_str(t).unwrap();
        assert_eq!(p == n, out);
        assert_eq!(n == p, out);
    };
    // The zero polynomial equals 0 and nothing else.
    test("0", "0", true);
    test("0", "1", false);
    test("1", "1", true);
    test("-1", "1", false);
    test("1/2", "0", false);
    test("1/2", "1", false);
    test("123", "123", true);
    test("-123", "123", false);
    test(
        "1000000000000000000000000",
        "1000000000000000000000000",
        true,
    );
    // No polynomial of positive degree equals a Natural, whatever its constant term.
    test("x", "0", false);
    test("1/2*x+1", "1", false);
}

// Comparing with a converted value is the reference the direct comparison is checked against.
#[allow(clippy::cmp_owned)]
#[test]
fn partial_eq_natural_properties() {
    rational_polynomial_natural_pair_gen().test_properties(|(p, n)| {
        let eq = p == n;
        assert_eq!(n == p, eq);
        assert_eq!(p == RationalPolynomial::from(Rational::from(&n)), eq);
        // Comparing with a Natural is comparing with the Integer, or the Rational, of the same
        // value.
        assert_eq!(p == Integer::from(&n), eq);
        assert_eq!(p == Rational::from(&n), eq);
    });

    natural_gen().test_properties(|n| {
        let p = RationalPolynomial::from(Rational::from(&n));
        assert!(p == n);
        assert!(n == p);
        assert_eq!(RationalPolynomial::ZERO == n, n == 0u32);
        assert_eq!(RationalPolynomial::one() == n, n == 1u32);
        assert!(RationalPolynomial::negative_one() != n);
        assert!(RationalPolynomial::one_half() != n);
        assert!(RationalPolynomial::x() != n);
    });
}
