// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use core::str::FromStr;
use malachite_base::num::arithmetic::traits::{ModIsReduced, ModPow};
use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base::polynomial::{
    ModMulTruncated, ModPowTruncated, ModPowTruncatedAssign, ModSquareTruncated, Polynomial,
};
use malachite_base::test_util::generators::*;
use malachite_base::test_util::unsigned_polynomial::arithmetic::mod_mul::*;
use malachite_base::test_util::unsigned_polynomial::arithmetic::mod_pow_truncated::*;
use malachite_base::unsigned_polynomial::UnsignedPolynomial;
use malachite_base::unsigned_polynomial::arithmetic::mod_pow_truncated::*;
use std::panic::catch_unwind;

fn verify<T: PrimitiveUnsigned>(
    p: &UnsignedPolynomial<T>,
    e: u64,
    len: u64,
    m: T,
    power: &UnsignedPolynomial<T>,
) {
    assert!(power.is_valid());
    assert_eq!(p.clone().mod_pow_truncated(e, len, m), *power);
    assert_eq!(p.mod_pow_truncated(e, len, m), *power);
    let mut q = p.clone();
    q.mod_pow_truncated_assign(e, len, m);
    assert_eq!(q, *power);
    assert_eq!(mod_pow_truncated_naive(p, e, len, m), *power);
    let xs = p.coefficients_asc();
    if xs.len() >= 2 && xs[0] != T::ZERO && e >= 3 && len >= 2 && m > T::ONE {
        assert_eq!(
            mod_pow_truncated_binexp(xs, e, len, m),
            power.coefficients_asc()
        );
    }
}

#[test]
fn test_mod_pow_truncated() {
    fn test<T: PrimitiveUnsigned>(s: &str, e: u64, len: u64, m: T, out: &str) {
        let p = UnsignedPolynomial::<T>::from_str(s).unwrap();
        let power = (&p).mod_pow_truncated(e, len, m);
        assert_eq!(power.to_string(), out);
        verify(&p, e, len, m, &power);
    }
    // - len == 0
    test::<u8>("x+1", 3, 0, 7, "0");
    // - m == 1
    test::<u8>("0", 3, 4, 1, "0");
    // - e == 0
    test::<u8>("x+1", 0, 5, 7, "1");
    test::<u8>("0", 0, 3, 7, "1");
    // - the polynomial is zero
    test::<u8>("0", 3, 2, 7, "0");
    // - the polynomial is zero modulo x^n
    test::<u8>("x^2+x", 3, 1, 7, "0");
    // - the polynomial is a constant
    test::<u8>("2", 3, 2, 8, "0");
    // - only the constant term of q^e is kept
    test::<u8>("3*x^2+x+2", 3, 1, 7, "1");
    // - e == 1
    test::<u8>("x^2+x+1", 1, 2, 7, "x+1");
    // - e == 2
    test::<u8>("x^2+x+1", 2, 3, 7, "3*x^2+2*x+1");
    // - binary exponentiation modulo m
    test::<u8>("x+1", 5, 3, 7, "3*x^2+5*x+1");
    test::<u8>("3*x^2+5*x+4", 9, 10, 7, "4*x^8+3*x^7+x^4+x^3+6*x+1");
    test::<u64>(
        "18446744073709551556*x+1",
        3,
        3,
        18446744073709551557,
        "3*x^2+18446744073709551554*x+1",
    );
    // - leading coefficients vanish modulo m
    test::<u8>("2*x^2+x+1", 7, 6, 6, "x^5+5*x^4+5*x^3+5*x^2+x+1");
    // - the power vanishes after a square
    test::<u8>("2*x+2", 3, 5, 4, "0");
    // - the power vanishes after a multiplication
    test::<u8>("2*x+2", 3, 5, 8, "0");
    // - low != 0, e * low < n
    test::<u8>("x^2+x", 4, 5, 7, "x^4");
    test::<u8>("x^3+x^2", 3, 8, 6, "3*x^7+x^6");
    // - e * low >= n
    test::<u8>("x^2+x", 4, 4, 7, "0");
}

fn mod_pow_truncated_generated_helper<T: PrimitiveUnsigned>() {
    for m in test_moduli::<T>() {
        for (len, n) in [(2, 5), (3, 20), (8, 30), (30, 40)] {
            for e in [3, 5, 9] {
                let p = UnsignedPolynomial::from_coefficients_asc(mod_generated_coefficients::<T>(
                    len, m, 3,
                ));
                let power = (&p).mod_pow_truncated(e, n, m);
                verify(&p, e, n, m, &power);
            }
        }
    }
}

#[test]
fn test_mod_pow_truncated_generated() {
    apply_fn_to_unsigneds!(mod_pow_truncated_generated_helper);
}

#[test]
fn mod_pow_truncated_fail() {
    let p = UnsignedPolynomial::<u8>::from_str("x+4").unwrap();
    assert_panic!((&p).mod_pow_truncated(3, 2, 3));
    assert_panic!((&p).mod_pow_truncated(3, 2, 0));
    assert_panic!({
        let mut q = p.clone();
        q.mod_pow_truncated_assign(3, 2, 3);
    });
}

fn mod_pow_truncated_properties_helper<T: PrimitiveUnsigned>() {
    unsigned_polynomial_unsigned_unsigned_unsigned_quadruple_gen_var_3::<T>().test_properties(
        |(p, e, len, m)| {
            let power = (&p).mod_pow_truncated(e, len, m);
            assert!(power.mod_is_reduced(&m));
            assert!(power.len() <= len);
            verify(&p, e, len, m, &power);
            assert_eq!((&p).mod_pow(e, m).truncate(len), power);
            if e != 0 {
                assert_eq!(
                    (&p).mod_pow_truncated(e - 1, len, m)
                        .mod_mul_truncated(&p, len, m),
                    power
                );
            }
            if e == 2 {
                assert_eq!((&p).mod_square_truncated(len, m), power);
            }
        },
    );
}

#[test]
fn mod_pow_truncated_properties() {
    apply_fn_to_unsigneds!(mod_pow_truncated_properties_helper);
}
