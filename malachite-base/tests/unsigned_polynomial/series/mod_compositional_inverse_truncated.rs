// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use core::str::FromStr;
use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base::num::modular::traits::ModIsReduced;
use malachite_base::polynomial::Polynomial;
use malachite_base::polynomial::series::traits::{
    ModComposeTruncated, ModCompositionalInverseTruncated,
};
use malachite_base::test_util::generators::*;
use malachite_base::test_util::unsigned_polynomial::series::mod_compositional_inverse_truncated::*;
use malachite_base::unsigned_polynomial::UnsignedPolynomial;

fn verify<T: PrimitiveUnsigned>(
    p: &UnsignedPolynomial<T>,
    len: u64,
    m: T,
) -> Option<UnsignedPolynomial<T>> {
    let r = p.mod_compositional_inverse_truncated(len, m);
    if let Some(r) = &r {
        assert!(r.is_valid());
        assert!(r.mod_is_reduced(&m));
    }
    assert_eq!(p.clone().mod_compositional_inverse_truncated(len, m), r);
    assert_eq!(mod_compositional_inverse_truncated_naive(p, len, m), r);
    assert_eq!(mod_compositional_inverse_truncated_newton(p, len, m), r);
    if let Some(r_alt) = mod_compositional_inverse_truncated_lagrange(p, len, m) {
        assert_eq!(Some(r_alt), r);
    }
    if let Some(r_alt) = mod_compositional_inverse_truncated_lagrange_fast(p, len, m) {
        assert_eq!(Some(r_alt), r);
    }
    r
}

fn show<T: PrimitiveUnsigned>(p: Option<UnsignedPolynomial<T>>) -> String {
    p.map_or_else(|| "None".to_string(), |p| p.to_string())
}

fn test_mod_compositional_inverse_truncated_helper<T: PrimitiveUnsigned>() {
    let test = |p, len, m: u8, out| {
        let p = UnsignedPolynomial::<T>::from_str(p).unwrap();
        assert_eq!(show(verify(&p, len, T::from(m))), out);
    };
    // - the length is 0
    test("x", 0, 7, "0");
    // - the length is 1
    test("3*x", 1, 7, "0");
    // - the length is 2: the inverse of the coefficient of x
    test("x^2+3*x", 2, 7, "5*x");
    // - modulo 1
    test("0", 5, 1, "0");
    // - the fast Lagrange algorithm
    test("x^2+x", 6, 7, "2*x^4+2*x^3+6*x^2+x");
    // - the fast Lagrange algorithm, with several giant steps
    test("5*x^3+2*x^2+3*x", 8, 7, "4*x^7+4*x^5+4*x^4+2*x^2+5*x");
    // - the polynomial is longer than the result
    test("x^9+x^8+x^7+x^6+x^5+x^4+x^3+x^2+x", 4, 7, "x^3+6*x^2+x");
    // - a linear polynomial
    test("4*x", 6, 7, "2*x");
    // - a division by an index is not possible, so Newton's iteration is used
    test("7*x^3+x^2+3*x", 6, 10, "2*x^5+7*x^3+7*x^2+7*x");
    // - a power of 2 as the modulus
    test(
        "5*x^3+3*x^2+x",
        10,
        128,
        "43*x^9+35*x^8+42*x^7+58*x^6+8*x^5+68*x^4+13*x^3+125*x^2+x",
    );
    // - a prime modulus less than the length, so Newton's iteration is used
    test("x^2+x", 8, 5, "2*x^7+3*x^6+4*x^5+2*x^3+4*x^2+x");
    // - a composite modulus whose prime factors are all at least the length
    test("3*x^3+x^2+2*x", 6, 49, "31*x^5+9*x^4+6*x^3+6*x^2+25*x");
    // - a composite modulus with a prime factor less than the length
    test(
        "x^2+x",
        10,
        49,
        "9*x^9+12*x^8+34*x^7+7*x^6+14*x^5+44*x^4+2*x^3+48*x^2+x",
    );
    // - the constant coefficient is not 0
    test("x+1", 5, 7, "None");
    // - the coefficient of x is 0
    test("x^2", 5, 7, "None");
    // - the coefficient of x is not a unit
    test("x^2+2*x", 5, 10, "None");
    // - the polynomial is 0
    test("0", 5, 7, "None");
    // - the polynomial is a nonzero constant
    test("3", 5, 7, "None");
}

#[test]
fn test_mod_compositional_inverse_truncated() {
    apply_fn_to_unsigneds!(test_mod_compositional_inverse_truncated_helper);
}

#[test]
fn test_mod_compositional_inverse_truncated_long() {
    // - the length reaches the Newton cutoff
    let p = UnsignedPolynomial::<u64>::from_str("x^2+x").unwrap();
    let m = 65521;
    let q = (&p).mod_compositional_inverse_truncated(2500, m).unwrap();
    assert_eq!(
        mod_compositional_inverse_truncated_lagrange_fast(&p, 2500, m),
        Some(q.clone())
    );
    assert_eq!(
        (&p).mod_compose_truncated(&q, 2500, m),
        UnsignedPolynomial::x()
    );
}

fn mod_compositional_inverse_truncated_properties_helper<T: PrimitiveUnsigned>() {
    unsigned_polynomial_unsigned_unsigned_triple_gen_var_16::<T>().test_properties(
        |(p, len, m)| {
            let q = (&p).mod_compositional_inverse_truncated(len, m).unwrap();
            assert!(q.is_valid());
            assert_eq!(
                p.clone().mod_compositional_inverse_truncated(len, m),
                Some(q.clone())
            );
            assert_eq!(
                mod_compositional_inverse_truncated_naive(&p, len, m),
                Some(q.clone())
            );
            assert_eq!(
                mod_compositional_inverse_truncated_newton(&p, len, m),
                Some(q.clone())
            );
            if let Some(q_alt) = mod_compositional_inverse_truncated_lagrange(&p, len, m) {
                assert_eq!(q_alt, q);
            }
            if let Some(q_alt) = mod_compositional_inverse_truncated_lagrange_fast(&p, len, m) {
                assert_eq!(q_alt, q);
            }
            if m != T::ONE && len >= 2 {
                let x = UnsignedPolynomial::<T>::x();
                assert_eq!((&p).mod_compose_truncated(&q, len, m), x);
                assert_eq!((&q).mod_compose_truncated(&p, len, m), x);
            }
        },
    );

    unsigned_polynomial_unsigned_unsigned_triple_gen_var_8::<T>().test_properties(|(p, len, m)| {
        let q = (&p).mod_compositional_inverse_truncated(len, m);
        let xs = p.coefficients_asc();
        let reversible = m == T::ONE
            || xs.len() >= 2
                && xs[0] == T::ZERO
                && xs[1] != T::ZERO
                && xs[1].mod_inverse(m).is_some();
        assert_eq!(q.is_some(), reversible);
        assert_eq!(mod_compositional_inverse_truncated_naive(&p, len, m), q);
    });
}

#[test]
fn mod_compositional_inverse_truncated_properties() {
    apply_fn_to_unsigneds!(mod_compositional_inverse_truncated_properties_helper);
}
