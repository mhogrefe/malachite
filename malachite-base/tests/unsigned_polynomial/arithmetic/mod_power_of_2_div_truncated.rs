// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use core::str::FromStr;
use malachite_base::num::arithmetic::traits::{ModPowerOf2, ModPowerOf2IsReduced};
use malachite_base::num::basic::traits::Zero;
use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base::num::conversion::traits::ExactFrom;
use malachite_base::polynomial::{
    CheckedModDivTruncated, CheckedModPowerOf2DivTruncated, ModPowerOf2DivTruncated,
    ModPowerOf2DivTruncatedAssign, ModPowerOf2InverseTruncated, ModPowerOf2MulTruncated,
    Polynomial,
};
use malachite_base::test_util::generators::{
    unsigned_polynomial_pair_unsigned_unsigned_quadruple_gen_var_1,
    unsigned_polynomial_pair_unsigned_unsigned_quadruple_gen_var_8,
    unsigned_polynomial_pair_unsigned_unsigned_quadruple_gen_var_10,
};
use malachite_base::test_util::unsigned_polynomial::arithmetic::mod_power_of_2_div_truncated::*;
use malachite_base::test_util::unsigned_polynomial::arithmetic::mod_power_of_2_mul::*;
use malachite_base::unsigned_polynomial::UnsignedPolynomial;

// Checks the quotient as power series of `p` by `q`, truncated to `len` coefficients, in every way
// of calling it, against the naive version and each algorithm; checks that it exists exactly when
// the divisor's constant coefficient is invertible, that its product with `q` is `p`, and that it
// is `p` times the inverse of `q`; and returns it.
fn verify<T: PrimitiveUnsigned>(
    p: &UnsignedPolynomial<T>,
    q: &UnsignedPolynomial<T>,
    len: u64,
    pow: u64,
) -> Option<UnsignedPolynomial<T>> {
    let r = p.checked_mod_power_of_2_div_truncated(q, len, pow);
    assert_eq!(
        p.clone()
            .checked_mod_power_of_2_div_truncated(q.clone(), len, pow),
        r
    );
    assert_eq!(
        p.clone().checked_mod_power_of_2_div_truncated(q, len, pow),
        r
    );
    assert_eq!(
        p.checked_mod_power_of_2_div_truncated(q.clone(), len, pow),
        r
    );
    assert_eq!(mod_power_of_2_div_truncated_naive(p, q, len, pow), r);
    assert_eq!(
        mod_power_of_2_div_truncated_basecase_wrapper(p, q, len, pow),
        r
    );
    assert_eq!(
        mod_power_of_2_div_truncated_newton_wrapper(p, q, len, pow),
        r
    );
    if pow < T::WIDTH {
        assert_eq!(p.checked_mod_div_truncated(q, len, T::power_of_2(pow)), r);
    }
    let invertible = len == 0 || pow == 0 || q.coefficient(0).odd();
    assert_eq!(r.is_some(), invertible);
    if let Some(s) = &r {
        assert!(s.is_valid());
        assert!(s.mod_power_of_2_is_reduced(pow));
        assert!(s.len() <= len);
        assert_eq!(p.mod_power_of_2_div_truncated(q, len, pow), *s);
        assert_eq!(
            p.clone().mod_power_of_2_div_truncated(q.clone(), len, pow),
            *s
        );
        assert_eq!(p.clone().mod_power_of_2_div_truncated(q, len, pow), *s);
        assert_eq!(p.mod_power_of_2_div_truncated(q.clone(), len, pow), *s);
        let mut t = p.clone();
        t.mod_power_of_2_div_truncated_assign(q, len, pow);
        assert_eq!(t, *s);
        let mut t = p.clone();
        t.mod_power_of_2_div_truncated_assign(q.clone(), len, pow);
        assert_eq!(t, *s);
        assert_eq!(q.mod_power_of_2_mul_truncated(s, len, pow), p.truncate(len));
        assert_eq!(
            p.mod_power_of_2_mul_truncated(
                q.mod_power_of_2_inverse_truncated(len, pow).unwrap(),
                len,
                pow
            ),
            *s
        );
    }
    r
}

fn test_mod_power_of_2_div_truncated_helper<T: PrimitiveUnsigned>() {
    let test = |p, q, len, pow, out: Option<&str>| {
        let p = UnsignedPolynomial::<T>::from_str(p).unwrap();
        let q = UnsignedPolynomial::<T>::from_str(q).unwrap();
        assert_eq!(
            verify(&p, &q, len, pow).map(|r| r.to_string()),
            out.map(ToString::to_string)
        );
    };
    // - the length is 0
    test("3*x+5", "x+1", 0, 3, Some("0"));
    // - modulo 1
    test("0", "0", 5, 0, Some("0"));
    // - the divisor is 0
    test("3*x+5", "0", 3, 3, None);
    // - the divisor's constant coefficient is even
    test("3*x+5", "x+2", 3, 3, None);
    // - the divisor's constant coefficient is 0
    test("3*x+5", "3*x", 3, 3, None);
    // - the dividend is 0
    test("0", "x+1", 4, 3, Some("0"));
    // - a constant divisor
    test("x^2+3*x+2", "3", 4, 3, Some("3*x^2+x+6"));
    // - a constant dividend
    test("5", "x+1", 4, 3, Some("3*x^3+5*x^2+3*x+5"));
    // - the divisor's constant coefficient is 1
    test("x^2+3*x+2", "x+1", 4, 3, Some("x+2"));
    // - the divisor's constant coefficient is not 1
    test(
        "x^2+3*x+2",
        "x^2+2*x+3",
        5,
        3,
        Some("5*x^4+7*x^3+3*x^2+5*x+6"),
    );
    // - only the first coefficients are read
    test("5*x^4+x+3", "2*x^3+x+1", 2, 3, Some("6*x+3"));
    // - the quotient has zeros at the end
    test("x^2+2*x+1", "x+1", 4, 3, Some("x+1"));
    // - the full width of a byte
    test(
        "255*x^3+254*x^2+253*x",
        "253*x^2+254*x+255",
        5,
        8,
        Some("12*x^4+252*x^2+3*x"),
    );
}

#[test]
fn test_mod_power_of_2_div_truncated() {
    apply_fn_to_unsigneds!(test_mod_power_of_2_div_truncated_helper);
}

// Every algorithm, at lengths that reach the method of Karp and Markstein from the dispatcher,
// against the naive version.
fn mod_power_of_2_div_truncated_algorithms_helper<T: PrimitiveUnsigned>() {
    for &(p_len, q_len, len) in
        &[(5, 5, 5), (30, 90, 90), (90, 90, 90), (200, 700, 700), (700, 5, 700)]
    {
        for pow in [1, 3, T::WIDTH >> 1, T::WIDTH] {
            let n = p_len;
            let p = UnsignedPolynomial::<T>::from_coefficients_asc(
                mod_power_of_2_generated_coefficients(n, pow, 1),
            );
            let n = q_len;
            let mut ys = mod_power_of_2_generated_coefficients(n, pow, 2);
            ys[0] = T::ONE;
            let q = UnsignedPolynomial::<T>::from_coefficients_asc(ys);
            assert!(verify(&p, &q, u64::exact_from(len), pow).is_some());
        }
    }
}

#[test]
fn test_mod_power_of_2_div_truncated_algorithms() {
    apply_fn_to_unsigneds!(mod_power_of_2_div_truncated_algorithms_helper);
}

#[test]
#[should_panic]
fn mod_power_of_2_div_truncated_fail_1() {
    // The divisor's constant coefficient is even.
    UnsignedPolynomial::<u8>::from_str("x+1")
        .unwrap()
        .mod_power_of_2_div_truncated(UnsignedPolynomial::from_str("x+2").unwrap(), 3, 3);
}

#[test]
#[should_panic]
fn mod_power_of_2_div_truncated_fail_2() {
    // A coefficient is not reduced.
    UnsignedPolynomial::<u8>::from_str("8*x+1")
        .unwrap()
        .mod_power_of_2_div_truncated(UnsignedPolynomial::from_str("x+1").unwrap(), 3, 3);
}

#[test]
#[should_panic]
fn mod_power_of_2_div_truncated_fail_3() {
    // The exponent is greater than the width of the type.
    UnsignedPolynomial::<u8>::ZERO.mod_power_of_2_div_truncated(UnsignedPolynomial::ZERO, 3, 9);
}

#[test]
#[should_panic]
fn mod_power_of_2_div_truncated_assign_fail() {
    let mut p = UnsignedPolynomial::<u8>::from_str("x+1").unwrap();
    p.mod_power_of_2_div_truncated_assign(UnsignedPolynomial::from_str("x+2").unwrap(), 3, 3);
}

#[test]
#[should_panic]
fn checked_mod_power_of_2_div_truncated_fail() {
    // A coefficient is not reduced.
    UnsignedPolynomial::<u8>::from_str("8*x+1")
        .unwrap()
        .checked_mod_power_of_2_div_truncated(UnsignedPolynomial::from_str("x+1").unwrap(), 3, 3);
}

fn mod_power_of_2_div_truncated_properties_helper<T: PrimitiveUnsigned>() {
    unsigned_polynomial_pair_unsigned_unsigned_quadruple_gen_var_1::<T>().test_properties(
        |(p, q, len, pow)| {
            verify(&p, &q, len, pow);
        },
    );

    unsigned_polynomial_pair_unsigned_unsigned_quadruple_gen_var_8::<T>().test_properties(
        |(p, q, len, pow)| {
            let r = verify(&p, &q, len, pow).unwrap();
            // Dividing the product by the divisor gives the dividend back.
            assert_eq!(
                (&r).mod_power_of_2_mul_truncated(&q, len, pow)
                    .mod_power_of_2_div_truncated(&q, len, pow),
                r
            );
            // Dividing by 1 truncates.
            assert_eq!(
                (&p).mod_power_of_2_div_truncated(
                    UnsignedPolynomial::one().mod_power_of_2(pow),
                    len,
                    pow
                ),
                p.truncate(len)
            );
        },
    );

    unsigned_polynomial_pair_unsigned_unsigned_quadruple_gen_var_10::<T>().test_properties(
        |(p, q, len, pow)| {
            verify(&p, &q, len, pow);
        },
    );
}

#[test]
fn mod_power_of_2_div_truncated_properties() {
    apply_fn_to_unsigneds!(mod_power_of_2_div_truncated_properties_helper);
}
