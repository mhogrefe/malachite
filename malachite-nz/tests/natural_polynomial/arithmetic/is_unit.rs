// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use core::str::FromStr;
use malachite_base::num::arithmetic::traits::IsUnit;
use malachite_nz::integer_polynomial::IntegerPolynomial;
use malachite_nz::natural_polynomial::NaturalPolynomial;
use malachite_nz::test_util::generators::natural_polynomial_gen;

#[test]
fn test_is_unit() {
    let test = |s, out| {
        let p = NaturalPolynomial::from_str(s).unwrap();
        assert_eq!(p.is_unit(), out);
    };
    test("0", false);
    test("1", true);
    test("2", false);
    test("1000000000000000000000000", false);
    test("x", false);
    test("x+1", false);
}

#[test]
fn is_unit_properties() {
    natural_polynomial_gen().test_properties(|p| {
        let is_unit = p.is_unit();
        assert_eq!(is_unit, p == 1u32);
        // A polynomial with natural coefficients is a unit exactly when it is one among the
        // polynomials with integer coefficients.
        assert_eq!(IntegerPolynomial::from(p).is_unit(), is_unit);
    });
}
