// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use core::str::FromStr;
use malachite_base::num::arithmetic::traits::{ModPowerOf2Mul, ModPowerOf2Pow};
use malachite_base::num::basic::traits::Zero;
use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base::polynomial::*;
use malachite_base::test_util::generators::*;
use malachite_base::test_util::unsigned_polynomial::arithmetic::mod_power_of_2_remove_power::*;
use malachite_base::unsigned_polynomial::UnsignedPolynomial;

// Checks the removal of the largest power of `q` from `p` in every way of calling it, against the
// naive version; checks that `q` to the power times the result is `p`, and that `q` does not divide
// the result unless it is 0; and returns it.
fn verify<T: PrimitiveUnsigned>(
    p: &UnsignedPolynomial<T>,
    q: &UnsignedPolynomial<T>,
    pow: u64,
) -> (UnsignedPolynomial<T>, u64) {
    let (s, e) = p.mod_power_of_2_remove_power(q, pow);
    assert!(s.is_valid());
    assert_eq!(
        p.clone().mod_power_of_2_remove_power(q.clone(), pow),
        (s.clone(), e)
    );
    assert_eq!(
        p.clone().mod_power_of_2_remove_power(q, pow),
        (s.clone(), e)
    );
    assert_eq!(
        p.mod_power_of_2_remove_power(q.clone(), pow),
        (s.clone(), e)
    );
    let mut t = p.clone();
    assert_eq!(t.mod_power_of_2_remove_power_assign(q, pow), e);
    assert_eq!(t, s);
    let mut t = p.clone();
    assert_eq!(t.mod_power_of_2_remove_power_assign(q.clone(), pow), e);
    assert_eq!(t, s);
    assert_eq!(mod_power_of_2_remove_power_naive(p, q, pow), (s.clone(), e));
    assert_eq!(q.mod_power_of_2_pow(e, pow).mod_power_of_2_mul(&s, pow), *p);
    if s != UnsignedPolynomial::ZERO {
        assert!(!(&s).mod_power_of_2_divisible_by(q, pow));
    }
    (s, e)
}

fn test_mod_power_of_2_remove_power_helper<T: PrimitiveUnsigned>() {
    let test = |p, q, pow: u64, out: &str, out_e| {
        let p = UnsignedPolynomial::<T>::from_str(p).unwrap();
        let q = UnsignedPolynomial::<T>::from_str(q).unwrap();
        let (s, e) = verify(&p, &q, pow);
        assert_eq!(s.to_string(), out);
        assert_eq!(e, out_e);
    };
    // - modulo 1
    test("0", "0", 0, "0", 0);
    // - the dividend is 0
    test("0", "x+1", 3, "0", 0);
    // - not divisible
    test("x^2+2", "x+1", 3, "x^2+2", 0);
    // - divisible several times
    test("x^3+4*x^2+5*x+2", "x+1", 3, "x+2", 2);
    // - a non-monic factor
    test("5*x^2+6*x+5", "3*x+1", 3, "5", 2);
    // - the full width
    test("3*x^2+2*x+255", "3*x+255", 8, "x+1", 1);
}

#[test]
fn test_mod_power_of_2_remove_power() {
    apply_fn_to_unsigneds!(test_mod_power_of_2_remove_power_helper);
}

#[test]
#[should_panic]
fn mod_power_of_2_remove_power_fail_1() {
    // The factor is 0.
    UnsignedPolynomial::<u8>::from_str("x+1")
        .unwrap()
        .mod_power_of_2_remove_power(UnsignedPolynomial::ZERO, 3);
}

#[test]
#[should_panic]
fn mod_power_of_2_remove_power_fail_2() {
    // The factor is a constant.
    UnsignedPolynomial::<u8>::from_str("x+1")
        .unwrap()
        .mod_power_of_2_remove_power(UnsignedPolynomial::from_str("3").unwrap(), 3);
}

#[test]
#[should_panic]
fn mod_power_of_2_remove_power_fail_3() {
    // The factor's leading coefficient is not invertible.
    UnsignedPolynomial::<u8>::from_str("x^2+1")
        .unwrap()
        .mod_power_of_2_remove_power(UnsignedPolynomial::from_str("2*x+1").unwrap(), 3);
}

#[test]
#[should_panic]
fn mod_power_of_2_remove_power_assign_fail() {
    let mut p = UnsignedPolynomial::<u8>::from_str("x+1").unwrap();
    p.mod_power_of_2_remove_power_assign(UnsignedPolynomial::from_str("3").unwrap(), 3);
}

fn mod_power_of_2_remove_power_properties_helper<T: PrimitiveUnsigned>() {
    unsigned_polynomial_unsigned_polynomial_unsigned_triple_gen_var_9::<T>().test_properties(
        |(p, q, pow)| {
            let e = verify(&p, &q, pow).1;
            assert!(p == UnsignedPolynomial::ZERO || e != 0);
        },
    );

    unsigned_polynomial_unsigned_polynomial_unsigned_triple_gen_var_5::<T>().test_properties(
        |(p, q, pow)| {
            if q.len() < 2 {
                return;
            }
            let (s, e) = verify(&p, &q, pow);
            // Multiplying by a power of the factor adds to the exponent.
            if p != UnsignedPolynomial::ZERO {
                let pq2 = (&q).mod_power_of_2_mul(&q, pow).mod_power_of_2_mul(&p, pow);
                assert_eq!(verify(&pq2, &q, pow), (s, e + 2));
            }
        },
    );
}

#[test]
fn mod_power_of_2_remove_power_properties() {
    apply_fn_to_unsigneds!(mod_power_of_2_remove_power_properties_helper);
}
