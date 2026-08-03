// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::arithmetic::traits::{AddMul, AddMulAssign, Pow, SubMul};
use malachite_base::num::basic::traits::{One, Zero};
use malachite_nz::natural::Natural;
use malachite_nz::test_util::generators::integer_triple_gen;
use malachite_q::Rational;
use malachite_q::test_util::generators::{rational_pair_gen, rational_triple_gen};
use malachite_q::test_util::rational::arithmetic::add_mul::add_mul_split;
use std::str::FromStr;

#[test]
fn test_add_mul() {
    let test = |r, s, t, out| {
        let u = Rational::from_str(r).unwrap();
        let v = Rational::from_str(s).unwrap();
        let w = Rational::from_str(t).unwrap();

        assert_eq!(u.clone().add_mul(v.clone(), w.clone()).to_string(), out);
        assert_eq!(u.clone().add_mul(v.clone(), &w).to_string(), out);
        assert_eq!(u.clone().add_mul(&v, w.clone()).to_string(), out);
        assert_eq!(u.clone().add_mul(&v, &w).to_string(), out);
        assert_eq!((&u).add_mul(&v, &w).to_string(), out);

        let mut mut_u = u.clone();
        mut_u.add_mul_assign(v.clone(), w.clone());
        assert_eq!(mut_u.to_string(), out);

        let mut mut_u = u.clone();
        mut_u.add_mul_assign(v.clone(), &w);
        assert_eq!(mut_u.to_string(), out);

        let mut mut_u = u.clone();
        mut_u.add_mul_assign(&v, w.clone());
        assert_eq!(mut_u.to_string(), out);

        let mut mut_u = u;
        mut_u.add_mul_assign(&v, &w);
        assert_eq!(mut_u.to_string(), out);
    };
    test("0", "0", "0", "0");
    test("0", "2/3", "3/4", "1/2");
    test("1/2", "0", "3/4", "1/2");
    test("1/2", "2/3", "0", "1/2");
    // the product cancels the first operand exactly
    test("1/2", "2/3", "3/4", "1");
    test("22/7", "-1/2", "1/3", "125/42");
    // a negative product can cancel the whole thing
    test("1/6", "-1/2", "1/3", "0");
    // the denominators of the operands need not divide the denominator of the result
    test("1/2", "1/3", "1/5", "17/30");
    test("-3", "4", "5", "17");
    test(
        "1000000000000",
        "1000000000000",
        "1000000000000",
        "1000000000001000000000000",
    );
}

#[test]
fn add_mul_properties() {
    rational_triple_gen().test_properties(|(x, y, z)| {
        let sum = (&x).add_mul(&y, &z);
        assert!(sum.is_valid());

        // every spelling agrees
        assert_eq!(x.clone().add_mul(y.clone(), z.clone()), sum);
        assert_eq!(x.clone().add_mul(y.clone(), &z), sum);
        assert_eq!(x.clone().add_mul(&y, z.clone()), sum);
        assert_eq!(x.clone().add_mul(&y, &z), sum);

        let mut mut_x = x.clone();
        mut_x.add_mul_assign(y.clone(), z.clone());
        assert!(mut_x.is_valid());
        assert_eq!(mut_x, sum);
        let mut mut_x = x.clone();
        mut_x.add_mul_assign(y.clone(), &z);
        assert_eq!(mut_x, sum);
        let mut mut_x = x.clone();
        mut_x.add_mul_assign(&y, z.clone());
        assert_eq!(mut_x, sum);
        let mut mut_x = x.clone();
        mut_x.add_mul_assign(&y, &z);
        assert_eq!(mut_x, sum);

        // the defining identity, and the unfused spelling the lint steers away from
        assert_eq!(&x + &y * &z, sum);
        // the rejected split-cancellation variant agrees; see its documentation for why it is not
        // the implementation
        let split = add_mul_split(&x, &y, &z);
        assert!(split.is_valid());
        assert_eq!(split, sum);
        // the two factors commute
        assert_eq!((&x).add_mul(&z, &y), sum);
        // adding a product is subtracting its negation
        assert_eq!((&x).sub_mul(&y, &-&z), sum);
        assert_eq!(-((-&x).add_mul(-&y, &z)), sum);
    });

    rational_pair_gen().test_properties(|(x, y)| {
        // the degenerate factors
        assert_eq!((&x).add_mul(&y, &Rational::ZERO), x);
        assert_eq!((&x).add_mul(&Rational::ZERO, &y), x);
        assert_eq!((&x).add_mul(&y, &Rational::ONE), &x + &y);
        assert_eq!((&x).add_mul(&Rational::ONE, &y), &x + &y);
        assert_eq!(Rational::ZERO.add_mul(&x, &y), &x * &y);
        // adding a value's own negation through the fused operation
        assert_eq!((&x).add_mul(&x, &Rational::from(-1)), Rational::ZERO);
    });

    integer_triple_gen().test_properties(|(x, y, z)| {
        // agreement with the Integer implementation, which is exact for the same inputs
        assert_eq!(
            Rational::from(&x).add_mul(Rational::from(&y), Rational::from(&z)),
            Rational::from((&x).add_mul(&y, &z))
        );
    });
}

// `rational_triple_gen`'s denominators are essentially always pairwise coprime, so the branch
// `add_mul_split` exists to exploit -- the denominator of `x` sharing factors with both of the
// others -- has to be built by hand to be exercised at all.
#[test]
fn add_mul_split_shared_factors() {
    let p = Natural::from(1000003u32);
    let q = Natural::from(1000033u32);
    for (dp, dq, k) in [(1u64, 1u64, 1u32), (3, 2, 5), (7, 4, 11), (2, 9, 3), (13, 13, 2)] {
        let b = (&p).pow(dp) * (&q).pow(dq);
        let d = (&p).pow(dp) * Natural::from(k);
        let f = (&q).pow(dq) * Natural::from(k + 1);
        let x = Rational::from_naturals(Natural::from(k + 2), b);
        let y = Rational::from_naturals(Natural::from(k + 3), d);
        let z = Rational::from_naturals(Natural::from(k + 5), f);
        let split = add_mul_split(&x, &y, &z);
        assert!(split.is_valid());
        assert_eq!(split, (&x).add_mul(&y, &z));
    }
}
