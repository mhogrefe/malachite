// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use core::str::FromStr;
use malachite_base::num::arithmetic::traits::ModPowerOf2IsReduced;
use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base::polynomial::*;
use malachite_base::test_util::generators::*;
use malachite_base::test_util::unsigned_polynomial::arithmetic::mod_power_of_2_compositional_inverse_truncated::*;
use malachite_base::unsigned_polynomial::UnsignedPolynomial;

fn verify<T: PrimitiveUnsigned>(
    p: &UnsignedPolynomial<T>,
    len: u64,
    pow: u64,
) -> Option<UnsignedPolynomial<T>> {
    let r = p.mod_power_of_2_compositional_inverse_truncated(len, pow);
    if let Some(r) = &r {
        assert!(r.is_valid());
        assert!(r.mod_power_of_2_is_reduced(pow));
    }
    assert_eq!(
        p.clone()
            .mod_power_of_2_compositional_inverse_truncated(len, pow),
        r
    );
    assert_eq!(
        mod_power_of_2_compositional_inverse_truncated_naive(p, len, pow),
        r
    );
    assert_eq!(
        mod_power_of_2_compositional_inverse_truncated_newton(p, len, pow),
        r
    );
    if let Some(r_alt) = mod_power_of_2_compositional_inverse_truncated_lagrange(p, len, pow) {
        assert_eq!(Some(r_alt), r);
    }
    if let Some(r_alt) = mod_power_of_2_compositional_inverse_truncated_lagrange_fast(p, len, pow) {
        assert_eq!(Some(r_alt), r);
    }
    r
}

fn show<T: PrimitiveUnsigned>(p: Option<UnsignedPolynomial<T>>) -> String {
    p.map_or_else(|| "None".to_string(), |p| p.to_string())
}

fn test_mod_power_of_2_compositional_inverse_truncated_helper<T: PrimitiveUnsigned>() {
    let test = |p, len, pow, out| {
        let p = UnsignedPolynomial::<T>::from_str(p).unwrap();
        assert_eq!(show(verify(&p, len, pow)), out);
    };
    // - the length is 0
    test("x", 0, 3, "0");
    // - the length is 1
    test("3*x", 1, 3, "0");
    // - the length is 2: the inverse of the coefficient of x
    test("x^2+3*x", 2, 3, "3*x");
    // - the power is 0
    test("0", 5, 0, "0");
    // - a division by an even index is not possible, so Newton's iteration is used
    test("x^2+x", 6, 3, "6*x^5+3*x^4+2*x^3+7*x^2+x");
    // - a longer result
    test(
        "5*x^3+2*x^2+3*x",
        10,
        8,
        "53*x^9+82*x^8+84*x^7+56*x^6+189*x^5+10*x^4+99*x^3+218*x^2+171*x",
    );
    // - several Newton steps
    test(
        "5*x^3+3*x^2+x",
        12,
        5,
        "9*x^11+x^10+11*x^9+3*x^8+10*x^7+26*x^6+8*x^5+4*x^4+13*x^3+29*x^2+x",
    );
    // - a linear polynomial
    test("5*x", 6, 3, "5*x");
    // - the constant coefficient is not 0
    test("x+1", 5, 3, "None");
    // - the coefficient of x is 0
    test("x^2", 5, 3, "None");
    // - the coefficient of x is even
    test("x^2+2*x", 5, 3, "None");
    // - the polynomial is 0
    test("0", 5, 3, "None");
}

#[test]
fn test_mod_power_of_2_compositional_inverse_truncated() {
    apply_fn_to_unsigneds!(test_mod_power_of_2_compositional_inverse_truncated_helper);
}

fn mod_power_of_2_compositional_inverse_truncated_properties_helper<T: PrimitiveUnsigned>() {
    unsigned_polynomial_unsigned_unsigned_triple_gen_var_17::<T>().test_properties(
        |(p, len, pow)| {
            let q = (&p)
                .mod_power_of_2_compositional_inverse_truncated(len, pow)
                .unwrap();
            assert!(q.is_valid());
            assert_eq!(
                p.clone()
                    .mod_power_of_2_compositional_inverse_truncated(len, pow),
                Some(q.clone())
            );
            assert_eq!(
                mod_power_of_2_compositional_inverse_truncated_naive(&p, len, pow),
                Some(q.clone())
            );
            assert_eq!(
                mod_power_of_2_compositional_inverse_truncated_newton(&p, len, pow),
                Some(q.clone())
            );
            if let Some(q_alt) =
                mod_power_of_2_compositional_inverse_truncated_lagrange(&p, len, pow)
            {
                assert_eq!(q_alt, q);
            }
            if let Some(q_alt) =
                mod_power_of_2_compositional_inverse_truncated_lagrange_fast(&p, len, pow)
            {
                assert_eq!(q_alt, q);
            }
            if len >= 2 {
                let x = UnsignedPolynomial::<T>::x();
                assert_eq!((&p).mod_power_of_2_compose_truncated(&q, len, pow), x);
                assert_eq!((&q).mod_power_of_2_compose_truncated(&p, len, pow), x);
            }
        },
    );

    unsigned_polynomial_unsigned_unsigned_triple_gen_var_7::<T>().test_properties(
        |(p, len, pow)| {
            let q = (&p).mod_power_of_2_compositional_inverse_truncated(len, pow);
            let xs = p.coefficients_asc();
            let reversible = pow == 0 || xs.len() >= 2 && xs[0] == T::ZERO && xs[1].odd();
            assert_eq!(q.is_some(), reversible);
            assert_eq!(
                mod_power_of_2_compositional_inverse_truncated_naive(&p, len, pow),
                q
            );
        },
    );
}

#[test]
fn mod_power_of_2_compositional_inverse_truncated_properties() {
    apply_fn_to_unsigneds!(mod_power_of_2_compositional_inverse_truncated_properties_helper);
}
