// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use core::str::FromStr;
use malachite_base::matrix::Matrix;
use malachite_base::num::arithmetic::traits::{DivExact, DivExactAssign};
use malachite_base::num::basic::traits::{One, Zero};
use malachite_nz::natural::Natural;
use malachite_nz::natural_matrix::NaturalMatrix;
use malachite_nz::test_util::generators::natural_matrix_natural_pair_gen_var_7;
use std::panic::catch_unwind;

#[test]
fn test_div_exact() {
    let test = |s, c, out| {
        let a = NaturalMatrix::from_str(s).unwrap();
        let c = Natural::from_str(c).unwrap();
        let b = (&a).div_exact(&c);
        assert!(b.is_valid());
        assert_eq!(b.to_string(), out);
        assert_eq!((&a).div_exact(c.clone()), b);
        assert_eq!(a.clone().div_exact(&c), b);
        assert_eq!(a.clone().div_exact(c.clone()), b);
        let mut x = a.clone();
        x.div_exact_assign(&c);
        assert_eq!(x, b);
        let mut x = a;
        x.div_exact_assign(c);
        assert_eq!(x, b);
    };
    test("[]_{0,0}", "5", "[]_{0,0}");
    test("[]_{2,0}", "5", "[]_{2,0}");
    test("[0, 6; 255, 3]", "1", "[0, 6; 255, 3]");
    test("[0, 6; 255, 3]", "3", "[0, 2; 85, 1]");
    test(
        "[3802951800684688204490109616128, 6; 0, 3]",
        "3",
        "[1267650600228229401496703205376, 2; 0, 1]",
    );
    test(
        "[1267650600228229401496703205376, 0]",
        "1267650600228229401496703205376",
        "[1, 0]",
    );
}

#[test]
fn div_exact_fail() {
    let a = NaturalMatrix::from_str("[1, 2]").unwrap();
    assert_panic!((&a).div_exact(Natural::ZERO));
    assert_panic!((&a).div_exact(&Natural::ZERO));
    assert_panic!(a.clone().div_exact(Natural::ZERO));
    assert_panic!(a.clone().div_exact(&Natural::ZERO));
    assert_panic!(
        NaturalMatrix::from_str("[]_{0,0}")
            .unwrap()
            .div_exact(Natural::ZERO)
    );
    assert_panic!({
        let mut b = a.clone();
        b.div_exact_assign(Natural::ZERO);
    });
    assert_panic!({
        let mut b = a.clone();
        b.div_exact_assign(&Natural::ZERO);
    });
}

#[test]
fn div_exact_properties() {
    natural_matrix_natural_pair_gen_var_7().test_properties(|(a, c)| {
        let b = (&a).div_exact(&c);
        assert!(b.is_valid());
        // The forms agree.
        assert_eq!((&a).div_exact(c.clone()), b);
        assert_eq!(a.clone().div_exact(&c), b);
        assert_eq!(a.clone().div_exact(c.clone()), b);
        let mut x = a.clone();
        x.div_exact_assign(&c);
        assert_eq!(x, b);
        let mut x = a.clone();
        x.div_exact_assign(c.clone());
        assert_eq!(x, b);

        // Element by element, this is the scalar exact quotient, which is also the ordinary
        // quotient. The shape is unchanged, and multiplying by the scalar recovers the matrix.
        assert_eq!(b.height(), a.height());
        assert_eq!(b.width(), a.width());
        for (x, y) in a.elements.iter().zip(&b.elements) {
            assert_eq!(*y, x.div_exact(&c));
            assert_eq!(*y, x / &c);
        }
        assert_eq!(&b * &c, a);
        // Dividing by 1 changes nothing.
        assert_eq!((&a).div_exact(Natural::ONE), a);
    });
}
