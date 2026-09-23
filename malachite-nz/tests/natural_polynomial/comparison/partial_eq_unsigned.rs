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
use malachite_base::test_util::generators::unsigned_gen;
use malachite_nz::natural::Natural;
use malachite_nz::natural_polynomial::NaturalPolynomial;
use malachite_nz::test_util::generators::{
    natural_polynomial_gen, natural_polynomial_unsigned_pair_gen,
};

#[test]
fn test_partial_eq_unsigned() {
    fn test<T: PrimitiveUnsigned + PartialEq<NaturalPolynomial>>(s: &str, c: T, out: bool)
    where
        NaturalPolynomial: PartialEq<T>,
    {
        let p = NaturalPolynomial::from_str(s).unwrap();
        assert_eq!(p == c, out);
        assert_eq!(c == p, out);
    }
    // The zero polynomial equals 0 and nothing else.
    test("0", 0u8, true);
    test("0", 0u64, true);
    test("0", 1u32, false);
    test("1", 1u16, true);
    test("1", 0usize, false);
    test("1", 2u128, false);
    test("123", 123u8, true);
    test("123", 5u64, false);
    test("255", u8::MAX, true);
    // A constant too large for the type equals none of its values, including the one it would wrap
    // to.
    test("256", 0u8, false);
    test("256", u8::MAX, false);
    test("18446744073709551615", u64::MAX, true);
    test("18446744073709551616", 0u64, false);
    test("18446744073709551616", 18446744073709551616u128, true);
    test("340282366920938463463374607431768211455", u128::MAX, true);
    // No polynomial of positive degree equals a value, whatever its constant term.
    test("x", 0u32, false);
    test("x", 1u32, false);
    test("x+1", 1u8, false);
    test("2*x^2+3", 3u64, false);
}

// Comparing with a converted value is the reference the direct comparison is checked against.
#[allow(clippy::cmp_owned, clippy::op_ref)]
fn partial_eq_unsigned_properties_helper<T>()
where
    T: PrimitiveUnsigned + PartialEq<NaturalPolynomial>,
    NaturalPolynomial: PartialEq<T>,
    Natural: From<T>,
{
    natural_polynomial_unsigned_pair_gen::<T>().test_properties(|(p, c)| {
        let eq = p == c;
        assert_eq!(c == p, eq);
        // Extra refs for type inference: with `NaturalPolynomial: PartialEq<T>` in scope, `p == q`
        // would look for that impl.
        assert_eq!(&p == &NaturalPolynomial::from(Natural::from(c)), eq);
        if eq {
            assert!(p.degree().is_none_or(|d| d == 0));
        }
    });

    unsigned_gen::<T>().test_properties(|c| {
        let p = NaturalPolynomial::from(Natural::from(c));
        assert!(p == c);
        assert!(c == p);
        assert_eq!(NaturalPolynomial::ZERO == c, c == T::ZERO);
        assert_eq!(NaturalPolynomial::one() == c, c == T::ONE);
        assert!(NaturalPolynomial::x() != c);
    });
}

#[test]
fn partial_eq_unsigned_properties() {
    apply_fn_to_unsigneds!(partial_eq_unsigned_properties_helper);

    natural_polynomial_gen().test_properties(|p| {
        // The comparisons that stand in for FLINT's `is_zero` and `is_one`.
        assert_eq!(p == 0u32, p == NaturalPolynomial::ZERO);
        assert_eq!(p == 1u32, p == NaturalPolynomial::one());
        if let Some(d) = p.degree()
            && d > 0
        {
            assert!(p != 0u64);
        }
    });
}
