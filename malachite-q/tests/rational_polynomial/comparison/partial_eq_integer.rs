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
use malachite_nz::test_util::generators::integer_gen;
use malachite_q::Rational;
use malachite_q::rational_polynomial::RationalPolynomial;
use malachite_q::test_util::generators::rational_polynomial_integer_pair_gen;

#[test]
fn test_partial_eq_integer() {
    let test = |s, t, out| {
        let p = RationalPolynomial::from_str(s).unwrap();
        let i = Integer::from_str(t).unwrap();
        assert_eq!(p == i, out);
        assert_eq!(i == p, out);
    };
    // The zero polynomial equals 0 and nothing else.
    test("0", "0", true);
    test("0", "-1", false);
    test("1", "1", true);
    test("-1", "-1", true);
    test("-1", "1", false);
    // A constant that is not an integer equals no Integer, neither its floor nor its ceiling.
    test("-1/2", "-1", false);
    test("-1/2", "0", false);
    test("-123", "-123", true);
    test("123", "-123", false);
    test(
        "-1000000000000000000000000",
        "-1000000000000000000000000",
        true,
    );
    // No polynomial of positive degree equals an Integer, whatever its constant term.
    test("x", "0", false);
    test("x-1", "-1", false);
}

// Comparing with a converted value is the reference the direct comparison is checked against.
#[allow(clippy::cmp_owned)]
#[test]
fn partial_eq_integer_properties() {
    rational_polynomial_integer_pair_gen().test_properties(|(p, i)| {
        let eq = p == i;
        assert_eq!(i == p, eq);
        assert_eq!(p == RationalPolynomial::from(Rational::from(&i)), eq);
        // Comparing with an Integer is comparing with the Rational of the same value.
        assert_eq!(p == Rational::from(&i), eq);
        assert_eq!(eq, *p.denominator_ref() == 1u32 && *p.numerator_ref() == i);
    });

    integer_gen().test_properties(|i| {
        let p = RationalPolynomial::from(Rational::from(&i));
        assert!(p == i);
        assert!(i == p);
        assert_eq!(RationalPolynomial::ZERO == i, i == 0u32);
        assert_eq!(RationalPolynomial::one() == i, i == 1u32);
        assert_eq!(RationalPolynomial::negative_one() == i, i == -1i32);
        assert!(RationalPolynomial::one_half() != i);
        assert!(RationalPolynomial::x() != i);
    });
}
