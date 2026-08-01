// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::arithmetic::traits::{
    CheckedMulSubMul, SaturatingMulSubMul, SaturatingMulSubMulAssign,
};
use malachite_base::num::basic::traits::Zero;
use malachite_nz::natural::Natural;
use malachite_nz::test_util::generators::natural_quadruple_gen;
use std::str::FromStr;

#[test]
fn test_saturating_mul_sub_mul() {
    fn test(s: &str, t: &str, u: &str, v: &str, out: &str) {
        let x = Natural::from_str(s).unwrap();
        let y = Natural::from_str(t).unwrap();
        let z = Natural::from_str(u).unwrap();
        let w = Natural::from_str(v).unwrap();
        let out = Natural::from_str(out).unwrap();

        assert_eq!(
            x.clone()
                .saturating_mul_sub_mul(y.clone(), z.clone(), &w.clone()),
            out
        );
        assert_eq!(
            x.clone()
                .saturating_mul_sub_mul(y.clone(), &z.clone(), w.clone()),
            out
        );
        assert_eq!(
            x.clone()
                .saturating_mul_sub_mul(y.clone(), &z.clone(), &w.clone()),
            out
        );
        assert_eq!(
            x.clone()
                .saturating_mul_sub_mul(&y.clone(), z.clone(), w.clone()),
            out
        );
        assert_eq!(
            x.clone()
                .saturating_mul_sub_mul(&y.clone(), z.clone(), &w.clone()),
            out
        );
        assert_eq!(
            x.clone()
                .saturating_mul_sub_mul(&y.clone(), &z.clone(), w.clone()),
            out
        );
        assert_eq!(
            x.clone()
                .saturating_mul_sub_mul(&y.clone(), &z.clone(), &w.clone()),
            out
        );
        assert_eq!((&x).saturating_mul_sub_mul(&y, &z, &w), out);

        let mut x_alt = x.clone();
        x_alt.saturating_mul_sub_mul_assign(y.clone(), z.clone(), w.clone());
        assert_eq!(x_alt, out);

        let mut x_alt = x.clone();
        x_alt.saturating_mul_sub_mul_assign(&y, &z, &w);
        assert_eq!(x_alt, out);
    }
    test("0", "0", "0", "0", "0");
    test("10", "3", "4", "5", "10");
    test("123", "456", "789", "12", "46620");
    // The second product is larger, so the result saturates to zero.
    test("1", "1", "2", "2", "0");
    test("123", "12", "456", "789", "0");
}

#[test]
fn saturating_mul_sub_mul_properties() {
    natural_quadruple_gen().test_properties(|(x, y, z, w)| {
        let result = x
            .clone()
            .saturating_mul_sub_mul(y.clone(), z.clone(), w.clone());
        assert!(result.is_valid());

        assert_eq!((&x).saturating_mul_sub_mul(&y, &z, &w), result);
        assert_eq!(x.clone().saturating_mul_sub_mul(&y, &z, &w), result);

        let mut x_alt = x.clone();
        x_alt.saturating_mul_sub_mul_assign(y.clone(), z.clone(), w.clone());
        assert_eq!(x_alt, result);

        let mut x_alt = x.clone();
        x_alt.saturating_mul_sub_mul_assign(&y, &z, &w);
        assert_eq!(x_alt, result);

        // It agrees with the checked version where that succeeds, and is zero where it does not.
        match x.clone().checked_mul_sub_mul(&y, &z, &w) {
            Some(exact) => assert_eq!(result, exact),
            None => assert_eq!(result, Natural::ZERO),
        }
        // Each product is symmetric in its own factors.
        assert_eq!(y.clone().saturating_mul_sub_mul(&x, &z, &w), result);
        assert_eq!(x.clone().saturating_mul_sub_mul(&y, &w, &z), result);
    });
}
