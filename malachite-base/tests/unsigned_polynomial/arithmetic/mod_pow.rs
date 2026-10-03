// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use core::str::FromStr;
use malachite_base::num::arithmetic::traits::{
    ModIsReduced, ModMul, ModPow, ModPowAssign, ModPowerOf2Pow,
};
use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base::polynomial::Polynomial;
use malachite_base::test_util::generators::unsigned_polynomial_unsigned_unsigned_triple_gen_var_8;
use malachite_base::test_util::unsigned_polynomial::arithmetic::mod_mul::*;
use malachite_base::test_util::unsigned_polynomial::arithmetic::mod_pow::*;
use malachite_base::unsigned_polynomial::UnsignedPolynomial;
use malachite_base::unsigned_polynomial::arithmetic::mod_pow::*;
use std::panic::catch_unwind;

fn verify<T: PrimitiveUnsigned>(
    p: &UnsignedPolynomial<T>,
    e: u64,
    m: T,
    power: &UnsignedPolynomial<T>,
) {
    assert!(power.is_valid());
    assert_eq!(p.clone().mod_pow(e, m), *power);
    assert_eq!(p.mod_pow(e, m), *power);
    let mut q = p.clone();
    q.mod_pow_assign(e, m);
    assert_eq!(q, *power);
    assert_eq!(mod_pow_naive(p, e, m), *power);
    let xs = p.coefficients_asc();
    if xs.len() >= 2 && xs[0] != T::ZERO && e >= 3 && m > T::ONE {
        assert_eq!(mod_pow_binexp(xs, e, m), power.coefficients_asc());
    }
}

#[test]
fn test_mod_pow() {
    fn test<T: PrimitiveUnsigned>(s: &str, e: u64, m: T, out: &str) {
        let p = UnsignedPolynomial::<T>::from_str(s).unwrap();
        let power = (&p).mod_pow(e, m);
        assert_eq!(power.to_string(), out);
        verify(&p, e, m, &power);
    }
    // - m == 1
    test::<u8>("0", 3, 1, "0");
    test::<u8>("0", 0, 1, "0");
    // - e == 0
    test::<u8>("x+1", 0, 7, "1");
    test::<u8>("0", 0, 7, "1");
    // - the polynomial is zero
    test::<u8>("0", 3, 7, "0");
    // - the polynomial is a constant
    test::<u8>("3", 5, 7, "5");
    test::<u8>("2", 3, 8, "0");
    // - the polynomial is a monomial
    test::<u8>("3*x^2", 3, 7, "6*x^6");
    test::<u8>("2*x^2", 3, 8, "0");
    // - e == 1
    test::<u8>("x^2+x", 1, 6, "x^2+x");
    // - e == 2
    test::<u8>("2*x+1", 2, 4, "1");
    // - binary exponentiation modulo m
    test::<u8>("x+1", 5, 7, "x^5+5*x^4+3*x^3+3*x^2+5*x+1");
    test::<u8>(
        "3*x^2+5*x+4",
        9,
        7,
        "6*x^18+6*x^17+x^15+6*x^14+3*x^11+3*x^10+4*x^8+3*x^7+x^4+x^3+6*x+1",
    );
    test::<u64>("x+1", 3, u64::MAX, "x^3+3*x^2+3*x+1");
    test::<u64>(
        "18446744073709551556*x+1",
        3,
        18446744073709551557,
        "18446744073709551556*x^3+3*x^2+18446744073709551554*x+1",
    );
    // - leading coefficients vanish modulo m
    test::<u8>(
        "2*x^2+x+1",
        7,
        6,
        "2*x^14+4*x^13+4*x^12+2*x^11+4*x^10+4*x^9+4*x^8+5*x^7+5*x^6+x^5+5*x^4+5*x^3+5*x^2+x+1",
    );
    // - the power vanishes after a square
    test::<u8>("2*x+2", 3, 4, "0");
    // - the power vanishes after a multiplication
    test::<u8>("2*x+2", 3, 8, "0");
    // - a factor of x^2 removed first
    test::<u8>("x^3+x^2", 5, 6, "x^15+5*x^14+4*x^13+4*x^12+5*x^11+x^10");
}

fn mod_pow_generated_helper<T: PrimitiveUnsigned>() {
    for m in test_moduli::<T>() {
        for len in [2, 3, 8, 30] {
            for e in [3, 5, 9] {
                let p = UnsignedPolynomial::from_coefficients_asc(mod_generated_coefficients::<T>(
                    len, m, 3,
                ));
                let power = (&p).mod_pow(e, m);
                verify(&p, e, m, &power);
            }
        }
    }
}

#[test]
fn test_mod_pow_generated() {
    apply_fn_to_unsigneds!(mod_pow_generated_helper);
}

#[test]
fn mod_pow_fail() {
    let p = UnsignedPolynomial::<u8>::from_str("x+4").unwrap();
    assert_panic!((&p).mod_pow(3, 3));
    assert_panic!((&p).mod_pow(3, 0));
    assert_panic!({
        let mut q = p.clone();
        q.mod_pow_assign(3, 3);
    });
}

fn mod_pow_properties_helper<T: PrimitiveUnsigned>() {
    unsigned_polynomial_unsigned_unsigned_triple_gen_var_8::<T>().test_properties(|(p, e, m)| {
        let power = (&p).mod_pow(e, m);
        assert!(power.mod_is_reduced(&m));
        verify(&p, e, m, &power);
        let half = e >> 1;
        assert_eq!(
            (&p).mod_pow(half, m).mod_mul((&p).mod_pow(e - half, m), m),
            power
        );
        if m.is_power_of_2() {
            assert_eq!((&p).mod_power_of_2_pow(e, m.floor_log_base_2()), power);
        }
    });
}

#[test]
fn mod_pow_properties() {
    apply_fn_to_unsigneds!(mod_pow_properties_helper);
}
