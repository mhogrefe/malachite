// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use core::str::FromStr;
use malachite_base::num::arithmetic::traits::{ModIsReduced, ModNeg, ModNegAssign, ModPowerOf2Neg};
use malachite_base::num::basic::traits::Zero;
use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base::polynomial::Polynomial;
use malachite_base::test_util::generators::{
    unsigned_polynomial_unsigned_unsigned_triple_gen_var_1,
    unsigned_polynomial_unsigned_unsigned_triple_gen_var_2,
};
use malachite_base::unsigned_polynomial::UnsignedPolynomial;

#[test]
fn test_mod_neg() {
    fn test<T: PrimitiveUnsigned>(s: &str, m: T, out: &str) {
        let p = UnsignedPolynomial::<T>::from_str(s).unwrap();
        let q = (&p).mod_neg(m);
        assert!(q.is_valid());
        assert_eq!(q.to_string(), out);
        assert_eq!(p.clone().mod_neg(m), q);
        let mut r = p;
        r.mod_neg_assign(m);
        assert_eq!(r, q);
    }
    test::<u8>("0", 1, "0");
    test::<u8>("0", 7, "0");
    test::<u8>("1", 2, "1");
    test::<u8>("5*x^2+x+3", 7, "2*x^2+6*x+4");
    test::<u8>("x", 255, "254*x");
    test::<u8>("x^2+2", 3, "2*x^2+1");
    test::<u8>("254*x+1", 255, "x+254");
    test::<u8>("6*x^3+6", 7, "x^3+1");
    test::<u64>(
        "3*x^2+5",
        18446744073709551557,
        "18446744073709551554*x^2+18446744073709551552",
    );
    test::<u64>(
        "18446744073709551556*x+1",
        18446744073709551557,
        "x+18446744073709551556",
    );
}

#[test]
#[should_panic]
fn mod_neg_fail_1() {
    // m is 0.
    (&UnsignedPolynomial::<u8>::ZERO).mod_neg(0);
}

#[test]
#[should_panic]
fn mod_neg_fail_2() {
    // A coefficient is not reduced.
    (&UnsignedPolynomial::<u8>::from_str("7*x+1").unwrap()).mod_neg(7);
}

#[test]
#[should_panic]
fn mod_neg_assign_fail() {
    // A coefficient is not reduced.
    let mut p = UnsignedPolynomial::<u8>::from_str("7*x+1").unwrap();
    p.mod_neg_assign(7);
}

fn mod_neg_properties_helper<T: PrimitiveUnsigned>() {
    unsigned_polynomial_unsigned_unsigned_triple_gen_var_2::<T>().test_properties(|(p, _, m)| {
        let q = (&p).mod_neg(m);
        assert!(q.is_valid());
        assert_eq!(p.clone().mod_neg(m), q);
        let mut r = p.clone();
        r.mod_neg_assign(m);
        assert_eq!(r, q);

        // The result is reduced, has the same degree, and is the coefficient-wise negation.
        assert!(q.mod_is_reduced(&m));
        assert_eq!(q.degree(), p.degree());
        for i in 0..p.len() {
            let c = p.coefficient(i);
            let d = q.coefficient(i);
            assert_eq!(d, c.mod_neg(m));
            assert_eq!(c.mod_add(d, m), T::ZERO);
        }
        // Negating twice gives the polynomial back.
        assert_eq!((&q).mod_neg(m), p);
    });

    unsigned_polynomial_unsigned_unsigned_triple_gen_var_1::<T>().test_properties(|(p, _, pow)| {
        // Modulo a power of 2 that fits in T, this agrees with mod_power_of_2_neg.
        if pow != 0 && pow < T::WIDTH {
            assert_eq!(
                (&p).mod_neg(T::power_of_2(pow)),
                (&p).mod_power_of_2_neg(pow)
            );
        }
    });
}

#[test]
fn mod_neg_properties() {
    apply_fn_to_unsigneds!(mod_neg_properties_helper);
}
