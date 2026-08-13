// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::arithmetic::traits::Abs;
use malachite_base::num::comparison::traits::{OrdAbs, OrdAbsDouble, PartialOrdAbs};
use malachite_base::test_util::common::test_custom_cmp_helper;
use malachite_nz::test_util::generators::integer_pair_gen;
use malachite_q::Rational;
use malachite_q::test_util::generators::{rational_gen, rational_pair_gen, rational_triple_gen};
use rug;
use std::cmp::Ordering::*;
use std::str::FromStr;

#[test]
fn test_ord_abs() {
    let strings = &[
        "0",
        "1237/1000000000000",
        "-123/1000000",
        "3/8",
        "-5/7",
        "1",
        "-7/5",
        "2",
        "-123",
        "999999999999",
        "-1000000000000",
        "1000000000001",
    ];
    test_custom_cmp_helper::<Rational, _>(strings, OrdAbs::cmp_abs);
    test_custom_cmp_helper::<rug::Rational, _>(strings, rug::Rational::cmp_abs);
}

#[test]
fn cmp_abs_properties() {
    rational_pair_gen().test_properties(|(x, y)| {
        let ord = x.cmp_abs(&y);
        assert_eq!(
            rug::Rational::from(&x).cmp_abs(&rug::Rational::from(&y)),
            ord
        );
        assert_eq!((&x).abs().cmp(&(&y).abs()), ord);
        assert_eq!((-x).cmp_abs(&(-y)), ord);
    });

    rational_gen().test_properties(|x| {
        assert_eq!(x.cmp_abs(&x), Equal);
        assert_eq!(x.cmp_abs(&-&x), Equal);
    });

    rational_triple_gen().test_properties(|(x, y, z)| {
        if x.lt_abs(&y) && y.lt_abs(&z) {
            assert!(x.lt_abs(&z));
        } else if x.gt_abs(&y) && y.gt_abs(&z) {
            assert!(x.gt_abs(&z));
        }
    });

    integer_pair_gen().test_properties(|(x, y)| {
        assert_eq!(
            Rational::from(&x).cmp_abs(&Rational::from(&y)),
            x.cmp_abs(&y)
        );
    });
}

#[test]
fn test_cmp_abs_double() {
    let test = |s, t, out| {
        let x = Rational::from_str(s).unwrap();
        let y = Rational::from_str(t).unwrap();
        assert_eq!(x.cmp_abs_double(&y), out);
        // the form it exists to avoid: doubling a Rational must renormalize its denominator
        assert_eq!(x.cmp_abs(&(&y << 1u64)), out);
    };
    test("0", "0", Equal);
    test("0", "1/2", Less);
    test("1/2", "0", Greater);
    test("1", "1/2", Equal);
    test("-1", "-1/2", Equal);
    test("1", "-1/2", Equal);
    test("2/3", "1/2", Less);
    test("3/2", "1/2", Greater);
    // - an even denominator, where doubling would halve it
    test("1", "1/4", Greater);
    test("1/2", "1/4", Equal);
    // - both sides above and below 1
    test("5", "1/2", Greater);
    test("1/5", "3", Less);
    test("100000000000000000000/3", "50000000000000000000/3", Equal);
}

#[test]
fn cmp_abs_double_properties() {
    rational_pair_gen().test_properties(|(x, y)| {
        let c = x.cmp_abs_double(&y);
        // the allocating form it replaces
        assert_eq!(c, x.cmp_abs(&(&y << 1u64)));
        // sign changes are invisible
        assert_eq!((-&x).cmp_abs_double(&y), c);
        assert_eq!(x.cmp_abs_double(&-&y), c);
        // a doubled value is at least as big in absolute value
        if c == Greater {
            assert_eq!(x.cmp_abs(&y), Greater);
        }
    });
}
