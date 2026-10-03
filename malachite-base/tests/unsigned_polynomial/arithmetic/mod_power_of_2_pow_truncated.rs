// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use core::str::FromStr;
use malachite_base::num::arithmetic::traits::{ModPowerOf2IsReduced, ModPowerOf2Pow};
use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base::polynomial::{
    ModPowerOf2MulTruncated, ModPowerOf2PowTruncated, ModPowerOf2PowTruncatedAssign,
    ModPowerOf2SquareTruncated, Polynomial,
};
use malachite_base::test_util::generators::*;
use malachite_base::test_util::unsigned_polynomial::arithmetic::mod_power_of_2_mul::*;
use malachite_base::test_util::unsigned_polynomial::arithmetic::mod_power_of_2_pow_truncated::*;
use malachite_base::unsigned_polynomial::UnsignedPolynomial;
use malachite_base::unsigned_polynomial::arithmetic::mod_power_of_2_pow_truncated::*;
use std::panic::catch_unwind;

fn verify<T: PrimitiveUnsigned>(
    p: &UnsignedPolynomial<T>,
    e: u64,
    len: u64,
    pow: u64,
    power: &UnsignedPolynomial<T>,
) {
    assert!(power.is_valid());
    assert_eq!(p.clone().mod_power_of_2_pow_truncated(e, len, pow), *power);
    assert_eq!(p.mod_power_of_2_pow_truncated(e, len, pow), *power);
    let mut q = p.clone();
    q.mod_power_of_2_pow_truncated_assign(e, len, pow);
    assert_eq!(q, *power);
    assert_eq!(mod_power_of_2_pow_truncated_naive(p, e, len, pow), *power);
    let xs = p.coefficients_asc();
    if xs.len() >= 2 && xs[0] != T::ZERO && e >= 3 && len >= 2 && pow != 0 {
        assert_eq!(
            mod_power_of_2_pow_truncated_binexp(xs, e, len, pow),
            power.coefficients_asc()
        );
    }
}

#[test]
fn test_mod_power_of_2_pow_truncated() {
    fn test<T: PrimitiveUnsigned>(s: &str, e: u64, len: u64, pow: u64, out: &str) {
        let p = UnsignedPolynomial::<T>::from_str(s).unwrap();
        let power = (&p).mod_power_of_2_pow_truncated(e, len, pow);
        assert_eq!(power.to_string(), out);
        verify(&p, e, len, pow, &power);
    }
    // - len == 0
    test::<u8>("x+1", 3, 0, 4, "0");
    // - pow == 0
    test::<u8>("0", 0, 4, 0, "0");
    // - e == 0
    test::<u8>("x+1", 0, 5, 4, "1");
    test::<u8>("0", 0, 3, 4, "1");
    // - the polynomial is zero
    test::<u8>("0", 3, 2, 4, "0");
    // - the polynomial is zero modulo x^n
    test::<u8>("x^2+x", 3, 1, 4, "0");
    // - the polynomial is a constant
    test::<u8>("2", 3, 2, 3, "0");
    // - only the constant term of q^e is kept
    test::<u8>("3*x^2+x+2", 3, 1, 4, "8");
    // - e == 1
    test::<u8>("x^2+x+1", 1, 2, 3, "x+1");
    // - e == 2
    test::<u8>("x^2+x+1", 2, 3, 3, "3*x^2+2*x+1");
    // - binary exponentiation modulo 2^k
    test::<u8>("x+1", 5, 3, 3, "2*x^2+5*x+1");
    test::<u8>(
        "3*x^2+7*x+5",
        9,
        10,
        4,
        "5*x^9+7*x^8+6*x^6+14*x^5+14*x^4+4*x^3+15*x^2+15*x+5",
    );
    test::<u64>(
        "18446744073709551615*x+1",
        3,
        3,
        64,
        "3*x^2+18446744073709551613*x+1",
    );
    // - leading coefficients vanish modulo 2^k
    test::<u8>("2*x^2+x+1", 7, 6, 3, "x^5+x^4+7*x^3+3*x^2+7*x+1");
    // - the power vanishes after a square
    test::<u8>("2*x+2", 3, 5, 2, "0");
    // - the power vanishes after a multiplication
    test::<u8>("2*x+2", 3, 5, 3, "0");
    // - low != 0, e * low < n
    test::<u8>("x^2+x", 4, 5, 3, "x^4");
    test::<u8>("x^3+x^2", 3, 8, 2, "3*x^7+x^6");
    // - e * low >= n
    test::<u8>("x^2+x", 4, 4, 3, "0");
}

fn mod_power_of_2_pow_truncated_generated_helper<T: PrimitiveUnsigned>() {
    for (len, n) in [(2, 5), (3, 20), (8, 30), (30, 40)] {
        for pow in [1, T::WIDTH >> 1, T::WIDTH - 1, T::WIDTH] {
            for e in [3, 5, 9] {
                let p = UnsignedPolynomial::from_coefficients_asc(
                    mod_power_of_2_generated_coefficients::<T>(len, pow, 3),
                );
                let power = (&p).mod_power_of_2_pow_truncated(e, n, pow);
                verify(&p, e, n, pow, &power);
            }
        }
    }
}

#[test]
fn test_mod_power_of_2_pow_truncated_generated() {
    apply_fn_to_unsigneds!(mod_power_of_2_pow_truncated_generated_helper);
}

#[test]
fn mod_power_of_2_pow_truncated_fail() {
    let p = UnsignedPolynomial::<u8>::from_str("x+4").unwrap();
    assert_panic!((&p).mod_power_of_2_pow_truncated(3, 2, 2));
    assert_panic!((&p).mod_power_of_2_pow_truncated(3, 2, 9));
    assert_panic!({
        let mut q = p.clone();
        q.mod_power_of_2_pow_truncated_assign(3, 2, 2);
    });
}

fn mod_power_of_2_pow_truncated_properties_helper<T: PrimitiveUnsigned>() {
    unsigned_polynomial_unsigned_unsigned_unsigned_quadruple_gen_var_2::<T>().test_properties(
        |(p, e, len, pow)| {
            let power = (&p).mod_power_of_2_pow_truncated(e, len, pow);
            assert!(power.mod_power_of_2_is_reduced(pow));
            assert!(power.len() <= len);
            verify(&p, e, len, pow, &power);
            assert_eq!((&p).mod_power_of_2_pow(e, pow).truncate(len), power);
            if e != 0 {
                assert_eq!(
                    (&p).mod_power_of_2_pow_truncated(e - 1, len, pow)
                        .mod_power_of_2_mul_truncated(&p, len, pow),
                    power
                );
            }
            if e == 2 {
                assert_eq!((&p).mod_power_of_2_square_truncated(len, pow), power);
            }
        },
    );
}

#[test]
fn mod_power_of_2_pow_truncated_properties() {
    apply_fn_to_unsigneds!(mod_power_of_2_pow_truncated_properties_helper);
}
