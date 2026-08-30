// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::arithmetic::traits::{AbsSquared, Conjugate};
use malachite_base::num::comparison::traits::EqAbs;
use malachite_q::Rational;
use malachite_q::gaussian_rational::GaussianRational;
use malachite_q::test_util::generators::{gaussian_rational_rational_pair_gen, rational_pair_gen};
use std::str::FromStr;

#[test]
fn test_eq_abs_rational() {
    let test = |s, t, out| {
        let x = GaussianRational::from_str(s).unwrap();
        let y = Rational::from_str(t).unwrap();
        assert_eq!(x.eq_abs(&y), out);
        assert_eq!(y.eq_abs(&x), out);
    };
    test("0", "0", true);
    test("i", "1", true);
    test("i/2", "-1/2", true);
    test("3/5+4i/5", "1", true);
    test("3/5+4i/5", "-1", true);
    test("3/5+4i/5", "4/5", false);
    test("3/2+2i", "5/2", true);
    test("1/2+i/2", "1", false);
    test("-22/7", "22/7", true);
}

#[test]
fn eq_abs_rational_properties() {
    gaussian_rational_rational_pair_gen().test_properties(|(x, y)| {
        let eq = x.eq_abs(&y);
        assert_eq!(y.eq_abs(&x), eq);
        assert_eq!((&x).abs_squared() == (&y).abs_squared(), eq);
        assert_eq!((&x).conjugate().eq_abs(&y), eq);
        assert_eq!((-&x).eq_abs(&y), eq);
        assert_eq!(x.eq_abs(&GaussianRational::from(y.clone())), eq);
    });

    rational_pair_gen().test_properties(|(x, y)| {
        assert_eq!(GaussianRational::from(x.clone()).eq_abs(&y), x.eq_abs(&y));
        assert_eq!(x.eq_abs(&GaussianRational::from(y.clone())), x.eq_abs(&y));
    });
}
