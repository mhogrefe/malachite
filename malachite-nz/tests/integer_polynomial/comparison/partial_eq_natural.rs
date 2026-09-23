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
use malachite_nz::integer_polynomial::IntegerPolynomial;
use malachite_nz::natural::Natural;
use malachite_nz::test_util::generators::{integer_polynomial_natural_pair_gen, natural_gen};

#[test]
fn test_partial_eq_natural() {
    let test = |s, t, out| {
        let p = IntegerPolynomial::from_str(s).unwrap();
        let n = Natural::from_str(t).unwrap();
        assert_eq!(p == n, out);
        assert_eq!(n == p, out);
    };
    // The zero polynomial equals 0 and nothing else.
    test("0", "0", true);
    test("0", "1", false);
    test("1", "1", true);
    test("-1", "1", false);
    test("123", "123", true);
    test("-123", "123", false);
    test(
        "1000000000000000000000000",
        "1000000000000000000000000",
        true,
    );
    test(
        "-1000000000000000000000000",
        "1000000000000000000000000",
        false,
    );
    // No polynomial of positive degree equals a Natural, whatever its constant term.
    test("x", "0", false);
    test("x+1", "1", false);
    test("-2*x^2+3", "3", false);
}

// Comparing with a converted value is the reference the direct comparison is checked against.
#[allow(clippy::cmp_owned)]
#[test]
fn partial_eq_natural_properties() {
    integer_polynomial_natural_pair_gen().test_properties(|(p, n)| {
        let eq = p == n;
        assert_eq!(n == p, eq);
        assert_eq!(p == IntegerPolynomial::from(Integer::from(&n)), eq);
        // Comparing with a Natural is comparing with the Integer of the same value.
        assert_eq!(p == Integer::from(&n), eq);
    });

    natural_gen().test_properties(|n| {
        let p = IntegerPolynomial::from(Integer::from(&n));
        assert!(p == n);
        assert!(n == p);
        assert_eq!(IntegerPolynomial::ZERO == n, n == 0u32);
        assert_eq!(IntegerPolynomial::one() == n, n == 1u32);
        assert!(IntegerPolynomial::negative_one() != n);
        assert!(IntegerPolynomial::x() != n);
    });
}
