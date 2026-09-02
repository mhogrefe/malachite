// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::arithmetic::traits::{AbsSquared, Conjugate};
use malachite_base::num::comparison::traits::{EqAbs, OrdAbs, PartialOrdAbs};
use malachite_nz::gaussian_integer::GaussianInteger;
use malachite_nz::natural::Natural;
use malachite_nz::test_util::generators::{gaussian_integer_natural_pair_gen, natural_pair_gen};
use std::cmp::Ordering::{self, *};
use std::str::FromStr;

#[test]
fn test_partial_cmp_abs_natural() {
    let test = |s, t, cmp: Option<Ordering>| {
        let x = GaussianInteger::from_str(s).unwrap();
        let y = Natural::from_str(t).unwrap();
        assert_eq!(x.partial_cmp_abs(&y), cmp);
        assert_eq!(y.partial_cmp_abs(&x), cmp.map(Ordering::reverse));
        assert_eq!(x.lt_abs(&y), cmp == Some(Less));
        assert_eq!(x.gt_abs(&y), cmp == Some(Greater));
        assert_eq!(x.eq_abs(&y), cmp == Some(Equal));
        assert_eq!(y.lt_abs(&x), cmp == Some(Greater));
        assert_eq!(y.gt_abs(&x), cmp == Some(Less));
    };
    test("0", "0", Some(Equal));
    test("0", "5", Some(Less));
    test("i", "1", Some(Equal));
    test("-123", "123", Some(Equal));
    test("3+4i", "5", Some(Equal));
    test("3+4i", "4", Some(Greater));
    test("3+4i", "6", Some(Less));
    test("-5-12i", "13", Some(Equal));
    test("2+2i", "3", Some(Less));
    test("1000000000000+i", "1000000000000", Some(Greater));
}

#[test]
fn partial_cmp_abs_natural_properties() {
    gaussian_integer_natural_pair_gen().test_properties(|(x, y)| {
        let cmp = x.partial_cmp_abs(&y);
        assert_eq!(y.partial_cmp_abs(&x), cmp.map(Ordering::reverse));
        assert_eq!(x.eq_abs(&y), cmp == Some(Equal));
        assert_eq!((&x).abs_squared().partial_cmp(&(&y).abs_squared()), cmp);
        assert_eq!((&x).conjugate().partial_cmp_abs(&y), cmp);
        assert_eq!((-&x).partial_cmp_abs(&y), cmp);
        assert_eq!(Some(x.cmp_abs(&GaussianInteger::from(&y))), cmp);
    });

    natural_pair_gen().test_properties(|(x, y)| {
        assert_eq!(
            GaussianInteger::from(x.clone()).partial_cmp_abs(&y),
            Some(x.cmp(&y))
        );
        assert_eq!(
            x.partial_cmp_abs(&GaussianInteger::from(y.clone())),
            Some(x.cmp(&y))
        );
    });
}
