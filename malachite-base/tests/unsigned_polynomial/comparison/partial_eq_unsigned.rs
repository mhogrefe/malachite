// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use core::str::FromStr;
use malachite_base::num::basic::traits::Zero;
use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base::polynomial::Polynomial;
use malachite_base::test_util::generators::{
    unsigned_gen, unsigned_polynomial_gen, unsigned_polynomial_unsigned_pair_gen,
};
use malachite_base::unsigned_polynomial::UnsignedPolynomial;

#[test]
fn test_partial_eq_unsigned() {
    let test = |s, c: u64, out| {
        let p = UnsignedPolynomial::<u64>::from_str(s).unwrap();
        assert_eq!(p == c, out);
        assert_eq!(c == p, out);
    };
    // The zero polynomial equals 0 and nothing else.
    test("0", 0, true);
    test("0", 1, false);
    test("1", 1, true);
    test("1", 0, false);
    test("1", 2, false);
    test("123", 123, true);
    test("123", 5, false);
    test("18446744073709551615", u64::MAX, true);
    test("18446744073709551615", 0, false);
    // No polynomial of positive degree equals a value, whatever its constant term.
    test("x", 0, false);
    test("x", 1, false);
    test("x+1", 1, false);
    test("2*x^2+3", 3, false);
}

fn partial_eq_unsigned_properties_helper<T>()
where
    T: PrimitiveUnsigned + PartialEq<UnsignedPolynomial<T>>,
    UnsignedPolynomial<T>: From<T>,
{
    unsigned_gen::<T>().test_properties(|c| {
        let p = UnsignedPolynomial::<T>::from(c);
        assert!(p == c);
        assert!(c == p);
        assert_eq!(UnsignedPolynomial::<T>::ZERO == c, c == T::ZERO);
        assert_eq!(UnsignedPolynomial::<T>::one() == c, c == T::ONE);
        assert!(UnsignedPolynomial::<T>::x() != c);
    });
}

// Comparing with a converted value is the reference the direct comparison is checked against.
#[allow(clippy::cmp_owned)]
#[test]
fn partial_eq_unsigned_properties() {
    unsigned_polynomial_unsigned_pair_gen().test_properties(|(p, c)| {
        let eq = p == c;
        assert_eq!(c == p, eq);
        // Comparing with a value is comparing with the constant polynomial built from it.
        assert_eq!(p == UnsignedPolynomial::from(c), eq);
        if eq {
            assert!(p.degree().is_none_or(|d| d == 0));
        }
    });

    unsigned_polynomial_gen().test_properties(|p| {
        // The comparisons that stand in for FLINT's `is_zero` and `is_one`.
        assert_eq!(p == 0, p == UnsignedPolynomial::ZERO);
        assert_eq!(p == 1, p == UnsignedPolynomial::one());
        if let Some(d) = p.degree()
            && d > 0
        {
            assert!(p != p.coefficient(0));
        }
    });

    apply_fn_to_unsigneds!(partial_eq_unsigned_properties_helper);
}
