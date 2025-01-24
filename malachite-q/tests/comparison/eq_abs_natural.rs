// Copyright © 2025 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::comparison::traits::EqAbs;
use malachite_nz::natural::Natural;
use malachite_nz::test_util::generators::natural_pair_gen;
use malachite_q::test_util::generators::{
    rational_natural_natural_triple_gen, rational_natural_pair_gen,
    rational_rational_natural_triple_gen,
};
use malachite_q::Rational;
use std::str::FromStr;

#[test]
fn test_eq_abs_rational_natural() {
    let test = |s, t, eq| {
        let u = Rational::from_str(s).unwrap();
        let v = Natural::from_str(t).unwrap();
        assert_eq!(u.eq_abs(&v), eq);
        assert_eq!(v.eq_abs(&u), eq);
    };
    test("0", "0", true);
    test("0", "5", false);
    test("123", "123", true);
    test("123", "124", false);
    test("123", "122", false);
    test("1000000000000", "123", false);
    test("123", "1000000000000", false);
    test("1000000000000", "1000000000000", true);
    test("1000000000000", "0", false);
    test("22/7", "0", false);
    test("22/7", "123", false);
    test("-22/7", "123", false);

    test("-123", "123", true);
    test("-123", "124", false);
    test("-123", "122", false);
    test("-1000000000000", "123", false);
    test("-123", "1000000000000", false);
    test("-1000000000000", "1000000000000", true);
    test("-1000000000000", "0", false);
}

#[test]
fn eq_abs_natural_properties() {
    rational_natural_pair_gen().test_properties(|(x, y)| {
        let eq = x.eq_abs(&y);
        assert_eq!(x.eq_abs(&Rational::from(&y)), eq);
        assert_eq!(y.eq_abs(&x), eq);
        assert_eq!((-x).eq_abs(&y), eq);
    });

    rational_rational_natural_triple_gen().test_properties(|(x, z, y)| {
        if x.eq_abs(&y) && y.eq_abs(&z) {
            assert!(x.eq_abs(&z));
        }
    });

    rational_natural_natural_triple_gen().test_properties(|(y, x, z)| {
        if x.eq_abs(&y) && y.eq_abs(&z) {
            assert_eq!(x, z);
        }
    });

    natural_pair_gen().test_properties(|(x, y)| {
        assert_eq!(Rational::from(&x).eq_abs(&y), x == y);
        assert_eq!(x.eq_abs(&Rational::from(&y)), x == y);
    });
}
