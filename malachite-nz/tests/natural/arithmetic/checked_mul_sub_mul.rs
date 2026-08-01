// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::arithmetic::traits::CheckedMulSubMul;
use malachite_nz::natural::Natural;
use malachite_nz::test_util::generators::natural_quadruple_gen;
use std::str::FromStr;

#[test]
fn test_checked_mul_sub_mul() {
    fn test(s: &str, t: &str, u: &str, v: &str, out: Option<&str>) {
        let x = Natural::from_str(s).unwrap();
        let y = Natural::from_str(t).unwrap();
        let z = Natural::from_str(u).unwrap();
        let w = Natural::from_str(v).unwrap();
        let out = out.map(|out| Natural::from_str(out).unwrap());

        assert_eq!(
            x.clone()
                .checked_mul_sub_mul(y.clone(), z.clone(), w.clone()),
            out
        );
        assert_eq!(x.clone().checked_mul_sub_mul(&y, &z, &w), out);
        assert_eq!((&x).checked_mul_sub_mul(&y, &z, &w), out);
    }
    test("0", "0", "0", "0", Some("0"));
    test("10", "3", "4", "5", Some("10"));
    test("123", "456", "789", "12", Some("46620"));
    // The two products are equal, so the result is exactly zero rather than `None`.
    test("6", "7", "21", "2", Some("0"));
    // The second product is larger.
    test("1", "1", "2", "2", None);
    test("123", "12", "456", "789", None);
}

#[test]
fn checked_mul_sub_mul_properties() {
    natural_quadruple_gen().test_properties(|(x, y, z, w)| {
        let result = x
            .clone()
            .checked_mul_sub_mul(y.clone(), z.clone(), w.clone());
        if let Some(ref result) = result {
            assert!(result.is_valid());
        }
        assert_eq!((&x).checked_mul_sub_mul(&y, &z, &w), result);
        assert_eq!(x.clone().checked_mul_sub_mul(&y, &z, &w), result);

        // It declines exactly when the second product is the larger one.
        assert_eq!(result.is_some(), &x * &y >= &z * &w);
        // Each product is symmetric in its own factors.
        assert_eq!(y.clone().checked_mul_sub_mul(&x, &z, &w), result);
        assert_eq!(x.clone().checked_mul_sub_mul(&y, &w, &z), result);
    });
}
