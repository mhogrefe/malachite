// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::arithmetic::traits::{AbsSquared, Conjugate};
use malachite_base::num::comparison::traits::EqAbs;
use malachite_nz::gaussian_integer::GaussianInteger;
use malachite_nz::natural::Natural;
use malachite_nz::test_util::generators::{gaussian_integer_natural_pair_gen, natural_pair_gen};
use std::str::FromStr;

#[test]
fn test_eq_abs_natural() {
    let test = |s, t, out| {
        let x = GaussianInteger::from_str(s).unwrap();
        let y = Natural::from_str(t).unwrap();
        assert_eq!(x.eq_abs(&y), out);
        assert_eq!(y.eq_abs(&x), out);
    };
    test("0", "0", true);
    test("i", "1", true);
    test("-123", "123", true);
    test("3+4i", "5", true);
    test("-3-4i", "5", true);
    test("3+4i", "4", false);
    test("5+12i", "13", true);
    test("2+2i", "3", false);
}

#[test]
fn eq_abs_natural_properties() {
    gaussian_integer_natural_pair_gen().test_properties(|(x, y)| {
        let eq = x.eq_abs(&y);
        assert_eq!(y.eq_abs(&x), eq);
        assert_eq!((&x).abs_squared() == (&y).abs_squared(), eq);
        assert_eq!((&x).conjugate().eq_abs(&y), eq);
        assert_eq!((-&x).eq_abs(&y), eq);
        assert_eq!(x.eq_abs(&GaussianInteger::from(y)), eq);
    });

    natural_pair_gen().test_properties(|(x, y)| {
        assert_eq!(GaussianInteger::from(x.clone()).eq_abs(&y), x == y);
        assert_eq!(x.eq_abs(&GaussianInteger::from(y.clone())), x == y);
    });
}
