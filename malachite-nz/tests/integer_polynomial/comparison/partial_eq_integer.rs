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
use malachite_nz::test_util::generators::{
    integer_gen, integer_polynomial_gen, integer_polynomial_integer_pair_gen,
};

#[test]
fn test_partial_eq_integer() {
    let test = |s, t, out| {
        let p = IntegerPolynomial::from_str(s).unwrap();
        let i = Integer::from_str(t).unwrap();
        assert_eq!(p == i, out);
        assert_eq!(i == p, out);
    };
    // The zero polynomial equals 0 and nothing else.
    test("0", "0", true);
    test("0", "1", false);
    test("0", "-1", false);
    test("1", "1", true);
    test("-1", "-1", true);
    test("-1", "1", false);
    test("-123", "-123", true);
    test("123", "-123", false);
    test(
        "-1000000000000000000000000",
        "-1000000000000000000000000",
        true,
    );
    test(
        "-1000000000000000000000000",
        "1000000000000000000000000",
        false,
    );
    // No polynomial of positive degree equals an Integer, whatever its constant term.
    test("x", "0", false);
    test("x-1", "-1", false);
    test("-2*x^2+3", "3", false);
}

// Comparing with a converted value is the reference the direct comparison is checked against.
#[allow(clippy::cmp_owned)]
#[test]
fn partial_eq_integer_properties() {
    integer_polynomial_integer_pair_gen().test_properties(|(p, i)| {
        let eq = p == i;
        assert_eq!(i == p, eq);
        assert_eq!(p == IntegerPolynomial::from(i.clone()), eq);
        if eq {
            assert!(p.degree().is_none_or(|d| d == 0));
        }
    });

    integer_gen().test_properties(|i| {
        let p = IntegerPolynomial::from(i.clone());
        assert!(p == i);
        assert!(i == p);
        assert_eq!(IntegerPolynomial::ZERO == i, i == 0u32);
        assert_eq!(IntegerPolynomial::one() == i, i == 1u32);
        assert_eq!(IntegerPolynomial::negative_one() == i, i == -1i32);
        assert!(IntegerPolynomial::x() != i);
    });

    integer_polynomial_gen().test_properties(|p| {
        // Comparing with an Integer agrees with comparing with a primitive of the same value.
        assert_eq!(p == Integer::ZERO, p == 0u32);
        assert_eq!(p == Integer::from(-5), p == -5i32);
        if let Some(d) = p.degree()
            && d > 0
        {
            assert!(p != *p.coefficient(0));
        }
    });
}
