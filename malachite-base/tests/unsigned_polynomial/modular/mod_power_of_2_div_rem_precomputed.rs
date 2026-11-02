// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use core::str::FromStr;
use malachite_base::num::basic::traits::Zero;
use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base::polynomial::Polynomial;
use malachite_base::polynomial::modular::traits::{
    ModPowerOf2DivAssignRemPrecomputed, ModPowerOf2DivRem, ModPowerOf2DivRemPrecomputed,
    ModPowerOf2QuotientPrecomputed, ModPowerOf2RemPrecomputed,
};
use malachite_base::test_util::generators::*;
use malachite_base::test_util::unsigned_polynomial::modular::mod_power_of_2_mul::*;
use malachite_base::unsigned_polynomial::UnsignedPolynomial;

// Checks the quotient and remainder of `p` by `q` modulo $2^k$ with precomputed data in every way
// of calling them, against `mod_power_of_2_div_rem`, and returns them.
fn verify<T: PrimitiveUnsigned>(
    p: &UnsignedPolynomial<T>,
    q: &UnsignedPolynomial<T>,
    pow: u64,
) -> (UnsignedPolynomial<T>, UnsignedPolynomial<T>) {
    let data = UnsignedPolynomial::precompute_mod_power_of_2_div_rem_data(q, pow);
    let r = p.mod_power_of_2_div_rem_precomputed(q, pow, &data);
    assert!(r.0.is_valid());
    assert!(r.1.is_valid());
    assert_eq!(
        p.clone()
            .mod_power_of_2_div_rem_precomputed(q.clone(), pow, &data),
        r
    );
    assert_eq!(
        p.clone().mod_power_of_2_div_rem_precomputed(q, pow, &data),
        r
    );
    assert_eq!(
        p.mod_power_of_2_div_rem_precomputed(q.clone(), pow, &data),
        r
    );
    let mut s = p.clone();
    assert_eq!(
        s.mod_power_of_2_div_assign_rem_precomputed(q, pow, &data),
        r.1
    );
    assert_eq!(s, r.0);
    let mut s = p.clone();
    assert_eq!(
        s.mod_power_of_2_div_assign_rem_precomputed(q.clone(), pow, &data),
        r.1
    );
    assert_eq!(s, r.0);
    assert_eq!(p.mod_power_of_2_div_rem(q, pow), r);
    assert_eq!(p.mod_power_of_2_quotient_precomputed(q, pow, &data), r.0);
    assert_eq!(p.mod_power_of_2_rem_precomputed(q, pow, &data), r.1);
    r
}

fn test_mod_power_of_2_div_rem_precomputed_helper<T: PrimitiveUnsigned>() {
    let test = |p, q, pow, out: (&str, &str)| {
        let p = UnsignedPolynomial::<T>::from_str(p).unwrap();
        let q = UnsignedPolynomial::<T>::from_str(q).unwrap();
        let (s, t) = verify(&p, &q, pow);
        assert_eq!((s.to_string().as_str(), t.to_string().as_str()), out);
    };
    // - modulo 1
    test("0", "0", 0, ("0", "0"));
    // - the dividend is shorter than the divisor
    test("5", "3*x+2", 3, ("0", "5"));
    // - a constant divisor
    test("6*x^2+5", "3", 3, ("2*x^2+7", "0"));
    // - a quotient of one coefficient, found with the precomputed inverse
    test("3*x+5", "x+1", 3, ("3", "2"));
    // - a quotient of two coefficients, found one term at a time
    test("x^3+2*x+1", "x^2+3", 3, ("x", "7*x+1"));
    // - the quotient is found with the precomputed inverse
    test(
        "x^5+3*x^4+2*x+1",
        "3*x^3+x+1",
        3,
        ("3*x^2+x+7", "4*x^2+2*x+2"),
    );
    // - the dividend is longer than twice the divisor, so it is divided a block at a time
    test("x^5+3*x^4+2*x+1", "3*x^2+x+1", 3, ("3*x^3+7*x+3", "6"));
    // - a long, sparse divisor, modulo 2
    test("x^30+x^7+1", "x^21+x^3+1", 1, ("x^9", "x^12+x^9+x^7+1"));
    // - a long, sparse, monic divisor
    test(
        "x^30+x^7+1",
        "x^21+2*x^3+1",
        3,
        ("x^9", "6*x^12+7*x^9+x^7+1"),
    );
    // - a long, sparse divisor that is not monic
    test(
        "x^30+x^7+1",
        "3*x^21+x^3+1",
        3,
        ("3*x^9", "5*x^12+5*x^9+x^7+1"),
    );
    // - a long divisor with no lower terms
    test("x^30+x^7+1", "3*x^21", 3, ("3*x^9", "x^7+1"));
    // - a long divisor with too many nonzero coefficients for the sparse method
    test(
        "x^30+x^7+1",
        "x^21+x^5+x^4+x^3+x^2+x+1",
        3,
        ("x^9", "7*x^14+7*x^13+7*x^12+7*x^11+7*x^10+7*x^9+x^7+1"),
    );
}

#[test]
fn test_mod_power_of_2_div_rem_precomputed() {
    apply_fn_to_unsigneds!(test_mod_power_of_2_div_rem_precomputed_helper);
}

// The kernels, on polynomials of chosen lengths, including long sparse divisors.
fn mod_power_of_2_div_rem_precomputed_algorithms_helper<T: PrimitiveUnsigned>() {
    let check = |xs: &[T], ys: &[T], pow: u64| {
        let p = UnsignedPolynomial::<T>::from_coefficients_asc(xs.to_vec());
        let q = UnsignedPolynomial::<T>::from_coefficients_asc(ys.to_vec());
        verify(&p, &q, pow);
    };
    for pow in [1, 3, T::WIDTH - 1, T::WIDTH] {
        let max = T::low_mask(pow);
        for &(p_len, q_len) in &[(10, 5), (9, 5), (6, 5), (200, 100), (500, 30), (3000, 7)] {
            let xs = mod_power_of_2_generated_coefficients(p_len, pow, 1);
            let mut ys = mod_power_of_2_generated_coefficients(q_len, pow, 2);
            *ys.last_mut().unwrap() = max;
            check(&xs, &ys, pow);
        }
        // - sparse divisors: monic with lower coefficients -1, monic with others, and non-monic
        for (lead, low) in [(T::ONE, max), (T::ONE, T::ONE), (max, T::ONE)] {
            let mut ys = vec![T::ZERO; 30];
            ys[0] = low;
            ys[3] = low;
            ys[17] = low;
            ys[29] = lead;
            for p_len in [50, 58, 200] {
                check(
                    &mod_power_of_2_generated_coefficients(p_len, pow, 1),
                    &ys,
                    pow,
                );
            }
        }
        // - a sparse divisor with too many nonzero coefficients for the sparse method
        let mut ys = vec![T::ZERO; 30];
        for i in [0, 2, 4, 6, 8, 10] {
            ys[i] = T::ONE;
        }
        ys[29] = T::ONE;
        check(&mod_power_of_2_generated_coefficients(50, pow, 1), &ys, pow);
    }
}

#[test]
fn test_mod_power_of_2_div_rem_precomputed_algorithms() {
    apply_fn_to_unsigneds!(mod_power_of_2_div_rem_precomputed_algorithms_helper);
}

#[test]
#[should_panic]
fn precompute_mod_power_of_2_div_rem_data_fail_1() {
    let _ = UnsignedPolynomial::<u8>::precompute_mod_power_of_2_div_rem_data(
        &UnsignedPolynomial::ZERO,
        3,
    );
}

#[test]
#[should_panic]
fn precompute_mod_power_of_2_div_rem_data_fail_2() {
    let _ = UnsignedPolynomial::<u8>::precompute_mod_power_of_2_div_rem_data(
        &UnsignedPolynomial::from_str("2*x+1").unwrap(),
        3,
    );
}

#[test]
#[should_panic]
fn precompute_mod_power_of_2_div_rem_data_fail_3() {
    let _ = UnsignedPolynomial::<u8>::precompute_mod_power_of_2_div_rem_data(
        &UnsignedPolynomial::from_str("x+1").unwrap(),
        9,
    );
}

#[test]
#[should_panic]
fn mod_power_of_2_div_rem_precomputed_fail() {
    let q = UnsignedPolynomial::<u8>::from_str("x+1").unwrap();
    let data = UnsignedPolynomial::precompute_mod_power_of_2_div_rem_data(&q, 3);
    let _ = UnsignedPolynomial::<u8>::from_str("8*x")
        .unwrap()
        .mod_power_of_2_div_rem_precomputed(q, 3, &data);
}

fn mod_power_of_2_div_rem_precomputed_properties_helper<T: PrimitiveUnsigned>() {
    unsigned_polynomial_unsigned_polynomial_unsigned_triple_gen_var_5::<T>().test_properties(
        |(p, q, pow)| {
            verify(&p, &q, pow);
        },
    );
}

#[test]
fn mod_power_of_2_div_rem_precomputed_properties() {
    apply_fn_to_unsigneds!(mod_power_of_2_div_rem_precomputed_properties_helper);
}
