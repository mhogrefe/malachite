// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::arithmetic::traits::AbsSquared;
use malachite_base::num::comparison::traits::{OrdAbs, PartialOrdAbs};
use malachite_nz::gaussian_integer::GaussianInteger;
use malachite_nz::test_util::generators::{
    gaussian_integer_gen, gaussian_integer_gen_var_1, gaussian_integer_pair_gen,
    gaussian_integer_triple_gen,
};
use std::cmp::Ordering::*;
use std::str::FromStr;

#[test]
fn test_cmp_abs() {
    let test = |s, t, out| {
        let x = GaussianInteger::from_str(s).unwrap();
        let y = GaussianInteger::from_str(t).unwrap();
        assert_eq!(x.cmp_abs(&y), out);
        assert_eq!(x.partial_cmp_abs(&y), Some(out));
    };
    // Both componentwise comparisons agree.
    test("1+i", "2+3i", Less);
    test("2+3i", "1+i", Greater);
    test("2+3i", "-2-3i", Equal);
    test("0", "0", Equal);
    // One componentwise comparison is a tie and the other decides.
    test("2+i", "2+3i", Less);
    test("2+3i", "2+i", Greater);
    // The crosswise comparisons decide.
    test("1+2i", "-2+i", Equal);
    test("1+3i", "-4+i", Less);
    test("4+i", "1+3i", Greater);
    // Both pairings conflict, so the squared absolute values are compared.
    test("3", "2+2i", Greater);
    test("2+2i", "3", Less);
    test("1+8i", "4+7i", Equal);
    test("4+2i", "3+3i", Greater);
    test("3+3i", "4+2i", Less);
}

#[test]
// The antisymmetry assertion below is the property under test; swapping its operands would restate
// the line above it rather than assert anything.
#[cfg_attr(dylint_lib = "malachite_lints", expect(redundant_cmp_reverse))]
fn cmp_abs_properties() {
    gaussian_integer_pair_gen().test_properties(|(x, y)| {
        let ord = x.cmp_abs(&y);
        assert_eq!(x.partial_cmp_abs(&y), Some(ord));
        assert_eq!((&x).abs_squared().cmp(&(&y).abs_squared()), ord);
        assert_eq!(y.cmp_abs(&x).reverse(), ord);
        let swapped = GaussianInteger {
            real: x.imaginary.clone(),
            imaginary: x.real.clone(),
        };
        assert_eq!(swapped.cmp_abs(&y), ord);
        let negated = GaussianInteger {
            real: -&x.real,
            imaginary: -&x.imaginary,
        };
        assert_eq!(negated.cmp_abs(&y), ord);
        let conjugate = GaussianInteger {
            real: x.real.clone(),
            imaginary: -&x.imaginary,
        };
        assert_eq!(conjugate.cmp_abs(&y), ord);
    });

    gaussian_integer_gen().test_properties(|x| {
        assert_eq!(x.cmp_abs(&x), Equal);
    });

    gaussian_integer_gen_var_1().test_properties(|x| {
        let y = GaussianInteger {
            real: x.imaginary.clone(),
            imaginary: x.real.clone(),
        };
        assert_eq!(x.cmp_abs(&y), Equal);
    });

    gaussian_integer_triple_gen().test_properties(|(x, y, z)| {
        if x.lt_abs(&y) && y.lt_abs(&z) {
            assert!(x.lt_abs(&z));
        } else if x.gt_abs(&y) && y.gt_abs(&z) {
            assert!(x.gt_abs(&z));
        }
    });
}
