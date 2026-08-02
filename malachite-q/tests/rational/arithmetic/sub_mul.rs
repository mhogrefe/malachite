// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::arithmetic::traits::{AddMul, SubMul, SubMulAssign};
use malachite_base::num::basic::traits::{One, Zero};
use malachite_nz::test_util::generators::integer_triple_gen;
use malachite_q::Rational;
use malachite_q::test_util::generators::{rational_pair_gen, rational_triple_gen};
use std::str::FromStr;

#[test]
fn test_sub_mul() {
    let test = |r, s, t, out| {
        let u = Rational::from_str(r).unwrap();
        let v = Rational::from_str(s).unwrap();
        let w = Rational::from_str(t).unwrap();

        assert_eq!(u.clone().sub_mul(v.clone(), w.clone()).to_string(), out);
        assert_eq!(u.clone().sub_mul(v.clone(), &w).to_string(), out);
        assert_eq!(u.clone().sub_mul(&v, w.clone()).to_string(), out);
        assert_eq!(u.clone().sub_mul(&v, &w).to_string(), out);
        assert_eq!((&u).sub_mul(&v, &w).to_string(), out);

        let mut mut_u = u.clone();
        mut_u.sub_mul_assign(v.clone(), w.clone());
        assert_eq!(mut_u.to_string(), out);

        let mut mut_u = u.clone();
        mut_u.sub_mul_assign(v.clone(), &w);
        assert_eq!(mut_u.to_string(), out);

        let mut mut_u = u.clone();
        mut_u.sub_mul_assign(&v, w.clone());
        assert_eq!(mut_u.to_string(), out);

        let mut mut_u = u;
        mut_u.sub_mul_assign(&v, &w);
        assert_eq!(mut_u.to_string(), out);
    };
    test("0", "0", "0", "0");
    test("0", "2/3", "3/4", "-1/2");
    test("1/2", "0", "3/4", "1/2");
    test("1/2", "2/3", "0", "1/2");
    // the product cancels the first operand exactly
    test("1/2", "2/3", "3/4", "0");
    test("22/7", "-1/2", "1/3", "139/42");
    // subtracting a negative product adds
    test("1/6", "-1/2", "1/3", "1/3");
    // the denominators of the operands need not divide the denominator of the result
    test("1/2", "1/3", "1/5", "13/30");
    test("-3", "4", "5", "-23");
    test(
        "1000000000000",
        "1000000000000",
        "1000000000000",
        "-999999999999000000000000",
    );
}

#[test]
fn sub_mul_properties() {
    rational_triple_gen().test_properties(|(x, y, z)| {
        let diff = (&x).sub_mul(&y, &z);
        assert!(diff.is_valid());

        // every spelling agrees
        assert_eq!(x.clone().sub_mul(y.clone(), z.clone()), diff);
        assert_eq!(x.clone().sub_mul(y.clone(), &z), diff);
        assert_eq!(x.clone().sub_mul(&y, z.clone()), diff);
        assert_eq!(x.clone().sub_mul(&y, &z), diff);

        let mut mut_x = x.clone();
        mut_x.sub_mul_assign(y.clone(), z.clone());
        assert!(mut_x.is_valid());
        assert_eq!(mut_x, diff);
        let mut mut_x = x.clone();
        mut_x.sub_mul_assign(y.clone(), &z);
        assert_eq!(mut_x, diff);
        let mut mut_x = x.clone();
        mut_x.sub_mul_assign(&y, z.clone());
        assert_eq!(mut_x, diff);
        let mut mut_x = x.clone();
        mut_x.sub_mul_assign(&y, &z);
        assert_eq!(mut_x, diff);

        // the defining identity, and the unfused spelling the lint steers away from
        assert_eq!(&x - &y * &z, diff);
        // the two factors commute
        assert_eq!((&x).sub_mul(&z, &y), diff);
        // subtracting a product is adding its negation
        assert_eq!((&x).add_mul(&y, &-&z), diff);
        assert_eq!(-((-&x).sub_mul(-&y, &z)), diff);
    });

    rational_pair_gen().test_properties(|(x, y)| {
        // the degenerate factors
        assert_eq!((&x).sub_mul(&y, &Rational::ZERO), x);
        assert_eq!((&x).sub_mul(&Rational::ZERO, &y), x);
        assert_eq!((&x).sub_mul(&y, &Rational::ONE), &x - &y);
        assert_eq!((&x).sub_mul(&Rational::ONE, &y), &x - &y);
        assert_eq!(Rational::ZERO.sub_mul(&x, &y), -(&x * &y));
        // subtracting a value from itself through the fused operation
        assert_eq!((&x).sub_mul(&x, &Rational::ONE), Rational::ZERO);
    });

    integer_triple_gen().test_properties(|(x, y, z)| {
        // agreement with the Integer implementation, which is exact for the same inputs
        assert_eq!(
            Rational::from(&x).sub_mul(Rational::from(&y), Rational::from(&z)),
            Rational::from((&x).sub_mul(&y, &z))
        );
    });
}
