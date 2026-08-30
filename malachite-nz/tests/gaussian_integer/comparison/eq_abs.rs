// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::arithmetic::traits::{AbsSquared, Conjugate};
use malachite_base::num::comparison::traits::{EqAbs, OrdAbs};
use malachite_nz::gaussian_integer::GaussianInteger;
use malachite_nz::test_util::generators::{gaussian_integer_gen, gaussian_integer_pair_gen};
use std::cmp::Ordering::Equal;
use std::str::FromStr;

#[test]
fn test_eq_abs() {
    let test = |s, t, out| {
        let x = GaussianInteger::from_str(s).unwrap();
        let y = GaussianInteger::from_str(t).unwrap();
        assert_eq!(x.eq_abs(&y), out);
        assert_eq!(y.eq_abs(&x), out);
    };
    test("0", "0", true);
    test("0", "i", false);
    test("i", "1", true);
    test("i", "-1", true);
    test("1+2i", "-2+i", true);
    test("1+2i", "2+i", true);
    test("1+2i", "1-2i", true);
    test("3+4i", "5", true);
    test("3+4i", "-5i", true);
    test("3+4i", "4+3i", true);
    test("2+2i", "3i", false);
    test("1000000000000+i", "1000000000000", false);
}

#[test]
fn eq_abs_properties() {
    gaussian_integer_pair_gen().test_properties(|(x, y)| {
        let eq = x.eq_abs(&y);
        assert_eq!(y.eq_abs(&x), eq);
        assert_eq!(x.cmp_abs(&y) == Equal, eq);
        assert_eq!((&x).abs_squared() == (&y).abs_squared(), eq);
        assert_eq!((&x).conjugate().eq_abs(&y), eq);
        assert_eq!((-&x).eq_abs(&y), eq);
    });

    gaussian_integer_gen().test_properties(|x| {
        assert!(x.eq_abs(&x));
        assert!(x.eq_abs(&-&x));
        assert!(x.eq_abs(&(&x).conjugate()));
    });
}
