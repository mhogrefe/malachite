// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::arithmetic::traits::{AbsSquared, Conjugate, ConjugateAssign};
use malachite_base::num::conversion::traits::IsReal;
use malachite_nz::gaussian_integer::GaussianInteger;
use malachite_nz::test_util::generators::{gaussian_integer_gen, gaussian_integer_gen_var_1};
use std::str::FromStr;

#[test]
fn test_conjugate() {
    let test = |s, out| {
        let x = GaussianInteger::from_str(s).unwrap();

        let conjugate = x.clone().conjugate();
        assert!(conjugate.real.is_valid());
        assert!(conjugate.imaginary.is_valid());
        assert_eq!(conjugate.to_string(), out);

        let conjugate = (&x).conjugate();
        assert!(conjugate.real.is_valid());
        assert!(conjugate.imaginary.is_valid());
        assert_eq!(conjugate.to_string(), out);

        let mut conjugate = x;
        conjugate.conjugate_assign();
        assert_eq!(conjugate.to_string(), out);
    };
    test("0", "0");
    test("1", "1");
    test("-123", "-123");
    test("i", "-i");
    test("-i", "i");
    test("1+i", "1-i");
    test("2-3i", "2+3i");
    test("-2+3i", "-2-3i");
}

#[test]
fn conjugate_properties() {
    gaussian_integer_gen().test_properties(|x| {
        let conjugate = x.clone().conjugate();
        assert!(conjugate.real.is_valid());
        assert!(conjugate.imaginary.is_valid());
        assert_eq!((&x).conjugate(), conjugate);
        let mut x_alt = x.clone();
        x_alt.conjugate_assign();
        assert_eq!(x_alt, conjugate);

        assert_eq!(conjugate.real, x.real);
        assert_eq!(conjugate.imaginary, -&x.imaginary);
        assert_eq!((&conjugate).conjugate(), x);
        assert_eq!(conjugate == x, x.is_real());
        assert_eq!((&conjugate).abs_squared(), (&x).abs_squared());
        assert_eq!((-&x).conjugate(), -(&x).conjugate());
    });

    gaussian_integer_gen_var_1().test_properties(|x| {
        assert_eq!((&x).conjugate(), x);
    });
}
