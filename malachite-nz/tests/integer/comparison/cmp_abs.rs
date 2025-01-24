// Copyright © 2025 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::arithmetic::traits::Abs;
use malachite_base::num::basic::traits::Zero;
use malachite_base::num::comparison::traits::{OrdAbs, PartialOrdAbs};
use malachite_base::test_util::common::test_custom_cmp_helper;
use malachite_base::test_util::generators::signed_pair_gen;
use malachite_nz::integer::Integer;
use malachite_nz::platform::SignedLimb;
use malachite_nz::test_util::generators::{
    integer_gen, integer_pair_gen, integer_triple_gen, natural_pair_gen,
};
use rug;
use std::cmp::Ordering::*;

#[test]
fn test_ord_abs() {
    let strings =
        &["0", "1", "-2", "123", "-124", "999999999999", "-1000000000000", "1000000000001"];
    test_custom_cmp_helper::<Integer, _>(strings, OrdAbs::cmp_abs);
    test_custom_cmp_helper::<rug::Integer, _>(strings, rug::Integer::cmp_abs);
}

#[test]
fn cmp_abs_properties() {
    integer_pair_gen().test_properties(|(x, y)| {
        let ord = x.cmp_abs(&y);
        assert_eq!(rug::Integer::from(&x).cmp_abs(&rug::Integer::from(&y)), ord);
        assert_eq!((&x).abs().cmp(&(&y).abs()), ord);
        assert_eq!((-x).cmp_abs(&(-y)), ord);
    });

    integer_gen().test_properties(|x| {
        assert_eq!(x.cmp_abs(&x), Equal);
        assert_eq!(x.cmp_abs(&-&x), Equal);
        assert!(x.ge_abs(&Integer::ZERO));
    });

    integer_triple_gen().test_properties(|(x, y, z)| {
        if x.lt_abs(&y) && y.lt_abs(&z) {
            assert!(x.lt_abs(&z));
        } else if x.gt_abs(&y) && y.gt_abs(&z) {
            assert!(x.gt_abs(&z));
        }
    });

    natural_pair_gen().test_properties(|(x, y)| {
        assert_eq!(Integer::from(&x).cmp_abs(&Integer::from(&y)), x.cmp(&y));
    });

    signed_pair_gen::<SignedLimb>().test_properties(|(x, y)| {
        assert_eq!(Integer::from(x).cmp_abs(&Integer::from(y)), x.cmp_abs(&y));
    });
}
