// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use core::str::FromStr;
use malachite_base::num::arithmetic::traits::ModIsReduced;
use malachite_base::num::basic::traits::Zero;
use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base::num::conversion::traits::ExactFrom;
use malachite_base::polynomial::{
    CheckedModDivTruncated, ModDivTruncated, ModDivTruncatedAssign, ModInverseTruncated,
    ModMulTruncated, Polynomial,
};
use malachite_base::test_util::generators::{
    unsigned_polynomial_pair_unsigned_unsigned_quadruple_gen_var_2,
    unsigned_polynomial_pair_unsigned_unsigned_quadruple_gen_var_7,
    unsigned_polynomial_pair_unsigned_unsigned_quadruple_gen_var_9,
};
use malachite_base::test_util::unsigned_polynomial::arithmetic::mod_div_truncated::*;
use malachite_base::test_util::unsigned_polynomial::arithmetic::mod_mul::*;
use malachite_base::unsigned_polynomial::UnsignedPolynomial;

// Checks the quotient as power series of `p` by `q`, truncated to `len` coefficients, in every way
// of calling it, against the naive version and each algorithm; checks that it exists exactly when
// the divisor's constant coefficient is invertible, that its product with `q` is `p`, and that it
// is `p` times the inverse of `q`; and returns it.
fn verify<T: PrimitiveUnsigned>(
    p: &UnsignedPolynomial<T>,
    q: &UnsignedPolynomial<T>,
    len: u64,
    m: T,
) -> Option<UnsignedPolynomial<T>> {
    let r = p.checked_mod_div_truncated(q, len, m);
    assert_eq!(p.clone().checked_mod_div_truncated(q.clone(), len, m), r);
    assert_eq!(p.clone().checked_mod_div_truncated(q, len, m), r);
    assert_eq!(p.checked_mod_div_truncated(q.clone(), len, m), r);
    assert_eq!(mod_div_truncated_naive(p, q, len, m), r);
    assert_eq!(mod_div_truncated_basecase_wrapper(p, q, len, m), r);
    assert_eq!(mod_div_truncated_newton_wrapper(p, q, len, m), r);
    let invertible = len == 0 || m == T::ONE || q.coefficient(0).coprime_with(m);
    assert_eq!(r.is_some(), invertible);
    if let Some(s) = &r {
        assert!(s.is_valid());
        assert!(s.mod_is_reduced(&m));
        assert!(s.len() <= len);
        assert_eq!(p.mod_div_truncated(q, len, m), *s);
        assert_eq!(p.clone().mod_div_truncated(q.clone(), len, m), *s);
        assert_eq!(p.clone().mod_div_truncated(q, len, m), *s);
        assert_eq!(p.mod_div_truncated(q.clone(), len, m), *s);
        let mut t = p.clone();
        t.mod_div_truncated_assign(q, len, m);
        assert_eq!(t, *s);
        let mut t = p.clone();
        t.mod_div_truncated_assign(q.clone(), len, m);
        assert_eq!(t, *s);
        assert_eq!(q.mod_mul_truncated(s, len, m), p.truncate(len));
        assert_eq!(
            p.mod_mul_truncated(q.mod_inverse_truncated(len, m).unwrap(), len, m),
            *s
        );
    }
    r
}

fn test_mod_div_truncated_helper<T: PrimitiveUnsigned>() {
    let test = |p, q, len, m: u8, out: Option<&str>| {
        let p = UnsignedPolynomial::<T>::from_str(p).unwrap();
        let q = UnsignedPolynomial::<T>::from_str(q).unwrap();
        assert_eq!(
            verify(&p, &q, len, T::from(m)).map(|r| r.to_string()),
            out.map(ToString::to_string)
        );
    };
    // - the length is 0
    test("3*x+5", "x+1", 0, 7, Some("0"));
    // - modulo 1
    test("0", "0", 5, 1, Some("0"));
    // - the divisor is 0
    test("3*x+5", "0", 3, 7, None);
    // - the divisor's constant coefficient is not invertible
    test("3*x+5", "x+2", 3, 6, None);
    // - the divisor's constant coefficient is 0
    test("3*x+5", "3*x", 3, 7, None);
    // - the dividend is 0
    test("0", "x+1", 4, 7, Some("0"));
    // - a constant divisor
    test("x^2+3*x+2", "3", 4, 7, Some("5*x^2+x+3"));
    // - a constant dividend
    test("5", "x+1", 4, 7, Some("2*x^3+5*x^2+2*x+5"));
    // - the divisor's constant coefficient is 1
    test("x^2+3*x+2", "x+1", 4, 7, Some("x+2"));
    // - the divisor's constant coefficient is not 1
    test("x^2+3*x+2", "x^2+2*x+3", 5, 7, Some("6*x^4+5*x^3+6*x+3"));
    // - only the first coefficients are read
    test("5*x^4+x+3", "2*x^3+x+1", 2, 7, Some("5*x+3"));
    // - the quotient has zeros at the end
    test("x^2+2*x+1", "x+1", 4, 7, Some("x+1"));
    // - a composite modulus
    test(
        "3*x^2+5*x+1",
        "2*x+3",
        5,
        10,
        Some("8*x^4+8*x^3+3*x^2+7*x+7"),
    );
    // - a modulus near the top of the word
    test(
        "250*x^3+249*x^2+248*x",
        "250*x^2+249*x+248",
        5,
        251,
        Some("x"),
    );
}

#[test]
fn test_mod_div_truncated() {
    apply_fn_to_unsigneds!(test_mod_div_truncated_helper);
}

// Every algorithm, at lengths that reach the method of Karp and Markstein from the dispatcher,
// against the naive version.
fn mod_div_truncated_algorithms_helper<T: PrimitiveUnsigned>() {
    for &(p_len, q_len, len) in
        &[(5, 5, 5), (30, 90, 90), (90, 90, 90), (200, 1500, 1500), (700, 5, 700)]
    {
        for m in [T::from(7u8), T::MAX >> 1 | T::ONE, T::MAX] {
            let n = p_len;
            let p =
                UnsignedPolynomial::<T>::from_coefficients_asc(mod_generated_coefficients(n, m, 1));
            let n = q_len;
            let mut ys = mod_generated_coefficients(n, m, 2);
            ys[0] = T::ONE;
            let q = UnsignedPolynomial::<T>::from_coefficients_asc(ys);
            assert!(verify(&p, &q, u64::exact_from(len), m).is_some());
        }
    }
}

#[test]
fn test_mod_div_truncated_algorithms() {
    apply_fn_to_unsigneds!(mod_div_truncated_algorithms_helper);
}

#[test]
#[should_panic]
fn mod_div_truncated_fail_1() {
    // The divisor's constant coefficient is not invertible.
    UnsignedPolynomial::<u8>::from_str("x+1")
        .unwrap()
        .mod_div_truncated(UnsignedPolynomial::from_str("x+2").unwrap(), 3, 6);
}

#[test]
#[should_panic]
fn mod_div_truncated_fail_2() {
    // A coefficient is not reduced.
    UnsignedPolynomial::<u8>::from_str("7*x+1")
        .unwrap()
        .mod_div_truncated(UnsignedPolynomial::from_str("x+1").unwrap(), 3, 7);
}

#[test]
#[should_panic]
fn mod_div_truncated_fail_3() {
    // A coefficient of the divisor is not reduced.
    UnsignedPolynomial::<u8>::from_str("x+1")
        .unwrap()
        .mod_div_truncated(UnsignedPolynomial::from_str("7*x+1").unwrap(), 3, 7);
}

#[test]
#[should_panic]
fn mod_div_truncated_fail_4() {
    // The modulus is zero.
    UnsignedPolynomial::<u8>::ZERO.mod_div_truncated(UnsignedPolynomial::ZERO, 3, 0);
}

#[test]
#[should_panic]
fn mod_div_truncated_assign_fail() {
    let mut p = UnsignedPolynomial::<u8>::from_str("x+1").unwrap();
    p.mod_div_truncated_assign(UnsignedPolynomial::from_str("x+2").unwrap(), 3, 6);
}

#[test]
#[should_panic]
fn checked_mod_div_truncated_fail() {
    // A coefficient is not reduced.
    UnsignedPolynomial::<u8>::from_str("7*x+1")
        .unwrap()
        .checked_mod_div_truncated(UnsignedPolynomial::from_str("x+1").unwrap(), 3, 7);
}

fn mod_div_truncated_properties_helper<T: PrimitiveUnsigned>() {
    unsigned_polynomial_pair_unsigned_unsigned_quadruple_gen_var_2::<T>().test_properties(
        |(p, q, len, m)| {
            verify(&p, &q, len, m);
        },
    );

    unsigned_polynomial_pair_unsigned_unsigned_quadruple_gen_var_7::<T>().test_properties(
        |(p, q, len, m)| {
            let r = verify(&p, &q, len, m).unwrap();
            // Dividing the product by the divisor gives the dividend back.
            assert_eq!(
                (&r).mod_mul_truncated(&q, len, m)
                    .mod_div_truncated(&q, len, m),
                r
            );
            // Dividing by 1 truncates.
            assert_eq!(
                (&p).mod_div_truncated(UnsignedPolynomial::one() % m, len, m),
                p.truncate(len)
            );
        },
    );

    unsigned_polynomial_pair_unsigned_unsigned_quadruple_gen_var_9::<T>().test_properties(
        |(p, q, len, m)| {
            verify(&p, &q, len, m);
        },
    );
}

#[test]
fn mod_div_truncated_properties() {
    apply_fn_to_unsigneds!(mod_div_truncated_properties_helper);
}
