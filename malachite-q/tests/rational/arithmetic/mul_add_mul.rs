// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::arithmetic::traits::{AddMul, MulAddMul, MulAddMulAssign};
use malachite_base::num::basic::traits::{One, Zero};
use malachite_nz::test_util::generators::integer_quadruple_gen;
use malachite_q::Rational;
use malachite_q::test_util::generators::{rational_pair_gen, rational_quadruple_gen};
use std::str::FromStr;

#[test]
fn test_mul_add_mul() {
    let test = |q, r, s, t, out| {
        let a = Rational::from_str(q).unwrap();
        let b = Rational::from_str(r).unwrap();
        let c = Rational::from_str(s).unwrap();
        let d = Rational::from_str(t).unwrap();

        assert_eq!(
            a.clone()
                .mul_add_mul(b.clone(), c.clone(), d.clone())
                .to_string(),
            out
        );
        assert_eq!(a.clone().mul_add_mul(&b, &c, &d).to_string(), out);
        assert_eq!((&a).mul_add_mul(&b, &c, &d).to_string(), out);

        let mut mut_a = a.clone();
        mut_a.mul_add_mul_assign(b.clone(), c.clone(), d.clone());
        assert_eq!(mut_a.to_string(), out);

        let mut mut_a = a;
        mut_a.mul_add_mul_assign(&b, &c, &d);
        assert_eq!(mut_a.to_string(), out);
    };
    test("0", "0", "0", "0", "0");
    test("1/2", "2/3", "3/4", "4/5", "14/15");
    test("22/7", "1/3", "-1/2", "5/6", "53/84");
    test("1/2", "0", "3/4", "4/5", "3/5");
    test("1/2", "2/3", "0", "4/5", "1/3");
    // the two products are equal, so they cancel or double exactly
    test("1/2", "2/3", "1/3", "1", "2/3");
    test("-3", "4", "5", "6", "18");
}

#[test]
fn mul_add_mul_properties() {
    rational_quadruple_gen().test_properties(|(x, y, z, w)| {
        let r = (&x).mul_add_mul(&y, &z, &w);
        assert!(r.is_valid());

        assert_eq!(x.clone().mul_add_mul(y.clone(), z.clone(), w.clone()), r);
        assert_eq!(x.clone().mul_add_mul(&y, &z, &w), r);

        let mut mut_x = x.clone();
        mut_x.mul_add_mul_assign(y.clone(), z.clone(), w.clone());
        assert!(mut_x.is_valid());
        assert_eq!(mut_x, r);
        let mut mut_x = x.clone();
        mut_x.mul_add_mul_assign(&y, &z, &w);
        assert_eq!(mut_x, r);

        // the defining identity, and the unfused spelling the lint steers away from
        assert_eq!(&x * &y + &z * &w, r);
        // the factors within each product commute
        assert_eq!((&y).mul_add_mul(&x, &w, &z), r);
        // it agrees with composing the three-operand fused operation
        assert_eq!((&x * &y).add_mul(&z, &w), r);
    });

    rational_pair_gen().test_properties(|(x, y)| {
        // degenerate operands
        assert_eq!(
            (&x).mul_add_mul(&y, &Rational::ZERO, &Rational::ONE),
            &x * &y
        );
        assert_eq!((&x).mul_add_mul(&Rational::ONE, &Rational::ZERO, &y), x);
        assert_eq!(Rational::ONE.mul_add_mul(&x, &Rational::ONE, &y), &x + &y);
    });

    integer_quadruple_gen().test_properties(|(x, y, z, w)| {
        // agreement with the Integer implementation, which is exact for the same inputs
        assert_eq!(
            Rational::from(&x).mul_add_mul(
                Rational::from(&y),
                Rational::from(&z),
                Rational::from(&w)
            ),
            Rational::from((&x).mul_add_mul(&y, &z, &w))
        );
    });
}
