// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::arithmetic::traits::{MulAddMul, MulAddMulAssign};
use malachite_nz::integer::Integer;
use malachite_nz::test_util::generators::integer_quadruple_gen;
use std::str::FromStr;

#[test]
fn test_mul_add_mul() {
    fn test(s: &str, t: &str, u: &str, v: &str, out: &str) {
        let x = Integer::from_str(s).unwrap();
        let y = Integer::from_str(t).unwrap();
        let z = Integer::from_str(u).unwrap();
        let w = Integer::from_str(v).unwrap();
        let out = Integer::from_str(out).unwrap();

        // Every by-value/by-reference spelling must give the same answer.
        assert_eq!(x.clone().mul_add_mul(y.clone(), z.clone(), w.clone()), out);
        assert_eq!(x.clone().mul_add_mul(y.clone(), z.clone(), &w.clone()), out);
        assert_eq!(x.clone().mul_add_mul(y.clone(), &z.clone(), w.clone()), out);
        assert_eq!(
            x.clone().mul_add_mul(y.clone(), &z.clone(), &w.clone()),
            out
        );
        assert_eq!(x.clone().mul_add_mul(&y.clone(), z.clone(), w.clone()), out);
        assert_eq!(
            x.clone().mul_add_mul(&y.clone(), z.clone(), &w.clone()),
            out
        );
        assert_eq!(
            x.clone().mul_add_mul(&y.clone(), &z.clone(), w.clone()),
            out
        );
        assert_eq!(
            x.clone().mul_add_mul(&y.clone(), &z.clone(), &w.clone()),
            out
        );
        assert_eq!((&x).mul_add_mul(&y, &z, &w), out);

        for (b, c, d) in [(false, false, false), (true, true, true)] {
            let mut x_alt = x.clone();
            if b && c && d {
                x_alt.mul_add_mul_assign(&y, &z, &w);
            } else {
                x_alt.mul_add_mul_assign(y.clone(), z.clone(), w.clone());
            }
            assert_eq!(x_alt, out);
        }
    }
    test("0", "0", "0", "0", "0");
    test("-10", "3", "4", "5", "-10");
    test("123", "-456", "789", "12", "-46620");
}

#[test]
fn mul_add_mul_properties() {
    integer_quadruple_gen().test_properties(|(x, y, z, w)| {
        let result = x.clone().mul_add_mul(y.clone(), z.clone(), w.clone());
        assert!(result.is_valid());

        // All nine spellings agree.
        assert_eq!((&x).mul_add_mul(&y, &z, &w), result);
        assert_eq!(x.clone().mul_add_mul(&y, &z, &w), result);
        assert_eq!(x.clone().mul_add_mul(y.clone(), &z, &w), result);
        assert_eq!(x.clone().mul_add_mul(&y, z.clone(), &w), result);
        assert_eq!(x.clone().mul_add_mul(&y, &z, w.clone()), result);

        let mut x_alt = x.clone();
        x_alt.mul_add_mul_assign(y.clone(), z.clone(), w.clone());
        assert_eq!(x_alt, result);

        let mut x_alt = x.clone();
        x_alt.mul_add_mul_assign(&y, &z, &w);
        assert_eq!(x_alt, result);

        // Each product is symmetric in its own factors.
        assert_eq!(y.clone().mul_add_mul(&x, &z, &w), result);
        assert_eq!(x.clone().mul_add_mul(&y, &w, &z), result);
        // Cross-check against the unfused expression.
        assert_eq!(result, &x * &y + &z * &w);
    });
}
