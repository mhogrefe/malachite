// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use core::str::FromStr;
use malachite_base::num::arithmetic::traits::{
    ModPowerOf2, ModPowerOf2IsReduced, ModPowerOf2Mul, ModPowerOf2Pow, ModPowerOf2PowAssign,
};
use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base::polynomial::Polynomial;
use malachite_base::test_util::generators::unsigned_polynomial_unsigned_unsigned_triple_gen_var_7;
use malachite_base::test_util::unsigned_polynomial::arithmetic::mod_power_of_2_mul::*;
use malachite_base::test_util::unsigned_polynomial::arithmetic::mod_power_of_2_pow::*;
use malachite_base::unsigned_polynomial::UnsignedPolynomial;
use malachite_base::unsigned_polynomial::arithmetic::mod_power_of_2_pow::*;
use std::panic::catch_unwind;

fn verify<T: PrimitiveUnsigned>(
    p: &UnsignedPolynomial<T>,
    e: u64,
    pow: u64,
    power: &UnsignedPolynomial<T>,
) {
    assert!(power.is_valid());
    assert_eq!(p.clone().mod_power_of_2_pow(e, pow), *power);
    assert_eq!(p.mod_power_of_2_pow(e, pow), *power);
    let mut q = p.clone();
    q.mod_power_of_2_pow_assign(e, pow);
    assert_eq!(q, *power);
    assert_eq!(mod_power_of_2_pow_naive(p, e, pow), *power);
    let xs = p.coefficients_asc();
    if xs.len() >= 2 && xs[0] != T::ZERO && e >= 3 && pow != 0 {
        assert_eq!(
            mod_power_of_2_pow_binexp(xs, e, pow),
            power.coefficients_asc()
        );
    }
}

#[test]
fn test_mod_power_of_2_pow() {
    fn test<T: PrimitiveUnsigned>(s: &str, e: u64, pow: u64, out: &str) {
        let p = UnsignedPolynomial::<T>::from_str(s).unwrap();
        let power = (&p).mod_power_of_2_pow(e, pow);
        assert_eq!(power.to_string(), out);
        verify(&p, e, pow, &power);
    }
    // - pow == 0
    test::<u8>("0", 3, 0, "0");
    test::<u8>("0", 0, 0, "0");
    // - e == 0
    test::<u8>("x+1", 0, 3, "1");
    test::<u8>("0", 0, 3, "1");
    // - the polynomial is zero
    test::<u8>("0", 3, 3, "0");
    // - the polynomial is a constant
    test::<u8>("3", 5, 4, "3");
    test::<u8>("2", 3, 3, "0");
    // - the polynomial is a monomial
    test::<u8>("3*x^2", 3, 4, "11*x^6");
    test::<u8>("2*x^2", 3, 3, "0");
    // - e == 1
    test::<u8>("x^2+x", 1, 2, "x^2+x");
    // - e == 2
    test::<u8>("2*x+1", 2, 2, "1");
    // - binary exponentiation modulo 2^k
    test::<u8>("x+1", 5, 3, "x^5+5*x^4+2*x^3+2*x^2+5*x+1");
    test::<u8>(
        "3*x^2+7*x+5",
        9,
        4,
        "3*x^18+15*x^17+9*x^16+4*x^15+2*x^14+14*x^13+10*x^12+x^10+5*x^9+7*x^8+6*x^6+14*x^5+14*x^4+\
        4*x^3+15*x^2+15*x+5",
    );
    test::<u64>(
        "18446744073709551615*x+1",
        3,
        64,
        "18446744073709551615*x^3+3*x^2+18446744073709551613*x+1",
    );
    // - leading coefficients vanish modulo 2^k
    test::<u8>(
        "2*x^2+x+1",
        7,
        3,
        "4*x^9+2*x^8+5*x^7+x^6+x^5+x^4+7*x^3+3*x^2+7*x+1",
    );
    // - the power vanishes after a square
    test::<u8>("2*x+2", 3, 2, "0");
    // - the power vanishes after a multiplication
    test::<u8>("2*x+2", 3, 3, "0");
    // - a factor of x^2 removed first
    test::<u8>("x^3+x^2", 5, 2, "x^15+x^14+2*x^13+2*x^12+x^11+x^10");
}

fn mod_power_of_2_pow_generated_helper<T: PrimitiveUnsigned>() {
    for len in [2, 3, 8, 30] {
        for pow in [1, T::WIDTH >> 1, T::WIDTH - 1, T::WIDTH] {
            for e in [3, 5, 9] {
                let p = UnsignedPolynomial::from_coefficients_asc(
                    mod_power_of_2_generated_coefficients::<T>(len, pow, 3),
                );
                let power = (&p).mod_power_of_2_pow(e, pow);
                verify(&p, e, pow, &power);
            }
        }
    }
}

#[test]
fn test_mod_power_of_2_pow_generated() {
    apply_fn_to_unsigneds!(mod_power_of_2_pow_generated_helper);
}

#[test]
fn mod_power_of_2_pow_fail() {
    let p = UnsignedPolynomial::<u8>::from_str("x+4").unwrap();
    assert_panic!((&p).mod_power_of_2_pow(3, 2));
    assert_panic!((&p).mod_power_of_2_pow(3, 9));
    assert_panic!({
        let mut q = p.clone();
        q.mod_power_of_2_pow_assign(3, 2);
    });
}

fn mod_power_of_2_pow_properties_helper<T: PrimitiveUnsigned>() {
    unsigned_polynomial_unsigned_unsigned_triple_gen_var_7::<T>().test_properties(|(p, e, pow)| {
        let power = (&p).mod_power_of_2_pow(e, pow);
        assert!(power.mod_power_of_2_is_reduced(pow));
        verify(&p, e, pow, &power);
        let half = e >> 1;
        assert_eq!(
            (&p).mod_power_of_2_pow(half, pow)
                .mod_power_of_2_mul((&p).mod_power_of_2_pow(e - half, pow), pow),
            power
        );
        // reducing to a smaller power of 2 commutes with powering
        let smaller = pow >> 1;
        assert_eq!(
            (&p).mod_power_of_2(smaller).mod_power_of_2_pow(e, smaller),
            (&power).mod_power_of_2(smaller)
        );
    });
}

#[test]
fn mod_power_of_2_pow_properties() {
    apply_fn_to_unsigneds!(mod_power_of_2_pow_properties_helper);
}
