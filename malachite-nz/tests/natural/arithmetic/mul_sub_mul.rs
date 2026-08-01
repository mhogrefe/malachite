// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::arithmetic::traits::{CheckedMulSubMul, MulSubMul, MulSubMulAssign};
use malachite_nz::natural::Natural;
use malachite_nz::test_util::generators::natural_quadruple_gen;
use std::panic::catch_unwind;
use std::str::FromStr;

#[test]
fn test_mul_sub_mul() {
    fn test(s: &str, t: &str, u: &str, v: &str, out: &str) {
        let x = Natural::from_str(s).unwrap();
        let y = Natural::from_str(t).unwrap();
        let z = Natural::from_str(u).unwrap();
        let w = Natural::from_str(v).unwrap();
        let out = Natural::from_str(out).unwrap();

        let result = x.clone().mul_sub_mul(y.clone(), z.clone(), w.clone());
        assert_eq!(result, out);
        // Every by-value/by-reference spelling must give the same answer.
        assert_eq!(x.clone().mul_sub_mul(y.clone(), z.clone(), &w.clone()), out);
        assert_eq!(x.clone().mul_sub_mul(y.clone(), &z.clone(), w.clone()), out);
        assert_eq!(
            x.clone().mul_sub_mul(y.clone(), &z.clone(), &w.clone()),
            out
        );
        assert_eq!(x.clone().mul_sub_mul(&y.clone(), z.clone(), w.clone()), out);
        assert_eq!(
            x.clone().mul_sub_mul(&y.clone(), z.clone(), &w.clone()),
            out
        );
        assert_eq!(
            x.clone().mul_sub_mul(&y.clone(), &z.clone(), w.clone()),
            out
        );
        assert_eq!(
            x.clone().mul_sub_mul(&y.clone(), &z.clone(), &w.clone()),
            out
        );
        assert_eq!((&x).mul_sub_mul(&y, &z, &w), out);

        let mut x_alt = x.clone();
        x_alt.mul_sub_mul_assign(y.clone(), z.clone(), w.clone());
        assert_eq!(x_alt, out);

        let mut x_alt = x.clone();
        x_alt.mul_sub_mul_assign(&y, &z, &w);
        assert_eq!(x_alt, out);
    }
    test("0", "0", "0", "0", "0");
    test("10", "3", "4", "5", "10");
    test("123", "456", "789", "12", "46620");
    // The two products are equal.
    test("6", "7", "21", "2", "0");
}

#[test]
fn mul_sub_mul_fail() {
    // 1 * 1 - 2 * 2 is negative.
    assert!(
        catch_unwind(|| {
            Natural::from(1u32).mul_sub_mul(
                Natural::from(1u32),
                Natural::from(2u32),
                Natural::from(2u32),
            )
        })
        .is_err()
    );
}

#[test]
fn mul_sub_mul_properties() {
    natural_quadruple_gen().test_properties(|(x, y, z, w)| {
        // `mul_sub_mul` panics where the exact result would be negative, which is exactly where
        // `checked_mul_sub_mul` declines.
        let Some(result) = x.clone().checked_mul_sub_mul(&y, &z, &w) else {
            return;
        };
        assert!(result.is_valid());

        assert_eq!(
            x.clone().mul_sub_mul(y.clone(), z.clone(), w.clone()),
            result
        );
        assert_eq!((&x).mul_sub_mul(&y, &z, &w), result);
        assert_eq!(x.clone().mul_sub_mul(&y, &z, &w), result);

        let mut x_alt = x.clone();
        x_alt.mul_sub_mul_assign(y.clone(), z.clone(), w.clone());
        assert_eq!(x_alt, result);

        let mut x_alt = x.clone();
        x_alt.mul_sub_mul_assign(&y, &z, &w);
        assert_eq!(x_alt, result);

        // Each product is symmetric in its own factors.
        assert_eq!(y.clone().mul_sub_mul(&x, &z, &w), result);
        assert_eq!(x.clone().mul_sub_mul(&y, &w, &z), result);
        // Cross-check against the unfused expression.
        assert_eq!(result, &x * &y - &z * &w);
    });
}
