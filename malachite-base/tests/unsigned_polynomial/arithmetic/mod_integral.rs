// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use core::str::FromStr;
use malachite_base::num::arithmetic::traits::ModIsReduced;
use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base::polynomial::{ModDerivative, ModIntegral, ModIntegralAssign, Polynomial};
use malachite_base::test_util::generators::*;
use malachite_base::test_util::unsigned_polynomial::arithmetic::mod_integral::*;
use malachite_base::unsigned_polynomial::UnsignedPolynomial;

#[test]
fn test_mod_integral() {
    fn test<T: PrimitiveUnsigned>(s: &str, m: T, out: &str) {
        let p = UnsignedPolynomial::<T>::from_str(s).unwrap();
        let r = (&p).mod_integral(m);
        assert!(r.is_valid());
        assert_eq!(r.to_string(), out);
        assert_eq!(p.clone().mod_integral(m), r);
        let mut q = p.clone();
        q.mod_integral_assign(m);
        assert_eq!(q, r);
        assert_eq!(mod_integral_naive(&p, m).unwrap(), r);
    }
    // - p.coefficients.is_empty(), for every m, even when no k is a unit
    test::<u8>("0", 1, "0");
    test::<u8>("0", 2, "0");
    // - n == 1: a constant moves to x without a division
    test::<u8>("5", 7, "5*x");
    test::<u8>("1", 2, "x");
    // - n >= 2, n < 3: one division, by 2
    test::<u8>("4*x+5", 7, "2*x^2+5*x");
    // - n >= 3
    test::<u8>("3*x^2+4*x+5", 7, "x^3+2*x^2+5*x");
    // Dividing by 3 is multiplying by 5 modulo 7.
    test::<u8>("x^2", 7, "5*x^3");
    // The loop past x^3.
    test::<u8>("x^4+x^3+x^2+x+1", 7, "3*x^5+2*x^4+5*x^3+4*x^2+x");
    // A composite modulus whose prime factors exceed the degree plus 1.
    test::<u8>("x^3+x^2+x+1", 35, "9*x^4+12*x^3+18*x^2+x");
    // A 64-bit prime.
    test::<u64>("x+1", 18446744073709551557, "9223372036854775779*x^2+x");
    // A modulus too large for a usize, so indices are reduced without one.
    test::<u128>(
        "2*x^2+3",
        170141183460469231731687303715884105727,
        "56713727820156410577229101238628035243*x^3+3*x",
    );
}

#[test]
#[should_panic]
fn mod_integral_fail_1() {
    // 2 is not a unit modulo 2.
    let _ = (&UnsignedPolynomial::<u8>::from_str("x+1").unwrap()).mod_integral(2);
}

#[test]
#[should_panic]
fn mod_integral_fail_2() {
    // 3 is 0 modulo 3, so the product of the indices is 0.
    let _ = (&UnsignedPolynomial::<u8>::from_str("x^2+1").unwrap()).mod_integral(3);
}

#[test]
#[should_panic]
fn mod_integral_fail_3() {
    // 2 is not a unit modulo 4, although 3 is.
    let _ = (&UnsignedPolynomial::<u8>::from_str("x^2").unwrap()).mod_integral(4);
}

#[test]
#[should_panic]
fn mod_integral_fail_4() {
    // A coefficient is not reduced.
    let _ = (&UnsignedPolynomial::<u8>::from_str("7").unwrap()).mod_integral(7);
}

#[test]
#[should_panic]
fn mod_integral_fail_5() {
    // m is 0.
    let _ = (&UnsignedPolynomial::<u8>::from_str("0").unwrap()).mod_integral(0);
}

#[test]
#[should_panic]
fn mod_integral_assign_fail() {
    let mut p = UnsignedPolynomial::<u8>::from_str("x+1").unwrap();
    p.mod_integral_assign(2);
}

// A long polynomial, whose indices, up to 250, are all units modulo the prime 251.
fn mod_integral_long_helper<T: PrimitiveUnsigned>() {
    let m = T::exact_from(251);
    let p = UnsignedPolynomial::from_coefficients_asc(
        (0..250u32)
            .map(|i| T::exact_from(i % 251))
            .collect::<Vec<T>>(),
    );
    assert_eq!((&p).mod_integral(m), mod_integral_naive(&p, m).unwrap());
}

#[test]
fn test_mod_integral_long() {
    mod_integral_long_helper::<u8>();
    mod_integral_long_helper::<u16>();
    mod_integral_long_helper::<u64>();
    mod_integral_long_helper::<u128>();
}

fn mod_integral_properties_helper<T: PrimitiveUnsigned>() {
    unsigned_polynomial_unsigned_unsigned_triple_gen_var_2::<T>().test_properties(|(p, _, m)| {
        if !mod_integral_is_defined(&p, m) {
            assert!(mod_integral_naive(&p, m).is_none());
            return;
        }
        let r = (&p).mod_integral(m);
        assert!(r.is_valid());
        // The forms agree.
        assert_eq!(p.clone().mod_integral(m), r);
        let mut q = p.clone();
        q.mod_integral_assign(m);
        assert_eq!(q, r);

        // It is reduced, and is the coefficient-wise integral.
        assert!(r.mod_is_reduced(&m));
        assert_eq!(mod_integral_naive(&p, m).unwrap(), r);
        // Its constant term is zero, and differentiating it gives back the polynomial.
        assert_eq!(r.coefficient(0), T::ZERO);
        assert_eq!((&r).mod_derivative(m), p);
    });
}

#[test]
fn mod_integral_properties() {
    apply_fn_to_unsigneds!(mod_integral_properties_helper);
}
