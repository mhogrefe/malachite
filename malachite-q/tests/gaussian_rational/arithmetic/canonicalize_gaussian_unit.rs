// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::arithmetic::traits::{
    CanonicalPrimitivePart, CanonicalUnitIPow, CanonicalizeGaussianUnit,
    CanonicalizeGaussianUnitAssign, DivI, MulI, MulIPow,
};
use malachite_nz::test_util::generators::gaussian_integer_gen;
use malachite_q::gaussian_rational::GaussianRational;
use malachite_q::test_util::generators::gaussian_rational_gen;
use std::str::FromStr;

#[test]
fn test_canonicalize_gaussian_unit() {
    let test = |s, out| {
        let x = GaussianRational::from_str(s).unwrap();

        let y = x.clone().canonicalize_gaussian_unit();
        assert!(y.real.is_valid());
        assert!(y.imaginary.is_valid());
        assert_eq!(y.to_string(), out);

        let y = (&x).canonicalize_gaussian_unit();
        assert!(y.real.is_valid());
        assert!(y.imaginary.is_valid());
        assert_eq!(y.to_string(), out);

        let mut y = x;
        y.canonicalize_gaussian_unit_assign();
        assert_eq!(y.to_string(), out);
    };
    test("0", "0");
    test("1", "1");
    test("i", "1");
    test("-1", "1");
    test("-i", "1");
    test("2+i", "2+i");
    test("-1+2i", "2+i");
    test("3+4i", "4-3i");
    test("-3", "3");
    test("-3i", "3");
    test("-1/2+3i/4", "3/4+i/2");
    // The boundary: argument pi/4 is canonical, -pi/4 is not.
    test("1+i", "1+i");
    test("1-i", "1+i");
}

#[test]
fn canonicalize_gaussian_unit_properties() {
    gaussian_rational_gen().test_properties(|x| {
        let y = x.clone().canonicalize_gaussian_unit();
        assert!(y.real.is_valid());
        assert!(y.imaginary.is_valid());
        assert_eq!((&x).canonicalize_gaussian_unit(), y);
        let mut x_alt = x.clone();
        x_alt.canonicalize_gaussian_unit_assign();
        assert_eq!(x_alt, y);

        // It is x times the power of i that `canonical_unit_i_pow` finds, and is itself canonical.
        assert_eq!((&x).mul_i_pow(x.canonical_unit_i_pow()), y);
        assert_eq!(y.canonical_unit_i_pow(), 0);
        assert_eq!((&y).canonicalize_gaussian_unit(), y);
        if x != 0u32 {
            // The canonical associate: positive real part, imaginary part in (-a, a].
            assert!(y.real > 0u32);
            assert!(-&y.real < y.imaginary && y.imaginary <= y.real);
        }
        // All four Gaussian associates canonicalize to the same value.
        assert_eq!((&x).mul_i().canonicalize_gaussian_unit(), y);
        assert_eq!((-&x).canonicalize_gaussian_unit(), y);
        assert_eq!((&x).div_i().canonicalize_gaussian_unit(), y);
        // The canonical primitive part is unchanged by it.
        assert_eq!(
            (&y).canonical_primitive_part(),
            (&x).canonical_primitive_part()
        );
    });

    gaussian_integer_gen().test_properties(|x| {
        // A Gaussian integer canonicalizes the same way either way.
        assert_eq!(
            GaussianRational::from(x.clone()).canonicalize_gaussian_unit(),
            GaussianRational::from(x.canonicalize_gaussian_unit())
        );
    });
}
