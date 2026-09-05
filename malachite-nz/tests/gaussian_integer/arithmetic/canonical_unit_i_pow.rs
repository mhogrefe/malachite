// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::arithmetic::traits::{CanonicalUnitIPow, CanonicalizeUnit, DivI, MulI};
use malachite_nz::gaussian_integer::GaussianInteger;
use malachite_nz::test_util::generators::gaussian_integer_gen;
use std::str::FromStr;

#[test]
fn test_canonical_unit_i_pow() {
    let test = |s, out| {
        assert_eq!(
            GaussianInteger::from_str(s).unwrap().canonical_unit_i_pow(),
            out
        );
    };
    test("0", 0);
    test("1", 0);
    test("i", 3);
    test("-1", 2);
    test("-i", 1);
    test("2+i", 0);
    test("2-i", 0);
    test("1+2i", 3);
    test("-1+2i", 3);
    test("-2+i", 2);
    test("-2-i", 2);
    test("-1-2i", 1);
    test("1-2i", 1);
    test("1+i", 0);
    test("1-i", 1);
    test("-1+i", 3);
    test("-1-i", 2);
    test("3+4i", 3);
    test("4+3i", 0);
}

#[test]
fn canonical_unit_i_pow_properties() {
    gaussian_integer_gen().test_properties(|x| {
        let k = x.canonical_unit_i_pow();
        assert!(k < 4);
        let mut y = x.clone();
        for _ in 0..k {
            y = y.mul_i();
        }
        assert_eq!((&x).canonicalize_unit(), y);
        if x == 0u32 {
            assert_eq!(k, 0);
        } else {
            // The canonical associate has a positive real part a and an imaginary part b with -a <
            // b <= a.
            assert!(y.real > 0u32);
            assert!(-&y.real < y.imaginary);
            assert!(y.imaginary <= y.real);
            assert_eq!(y.canonical_unit_i_pow(), 0);
            // Rotating x by i^j shifts the required correction by -j.
            assert_eq!((&x).mul_i().canonical_unit_i_pow(), (k + 3) % 4);
            assert_eq!((-&x).canonical_unit_i_pow(), (k + 2) % 4);
            assert_eq!((&x).div_i().canonical_unit_i_pow(), (k + 1) % 4);
        }
    });
}
