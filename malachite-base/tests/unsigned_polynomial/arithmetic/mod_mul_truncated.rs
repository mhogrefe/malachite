// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.
use core::str::FromStr;
use malachite_base::num::arithmetic::traits::{ModIsReduced, ModMul};
use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base::polynomial::{ModMulTruncated, ModMulTruncatedAssign, Polynomial};
use malachite_base::test_util::generators::*;
use malachite_base::test_util::unsigned_polynomial::arithmetic::mod_mul::*;
use malachite_base::test_util::unsigned_polynomial::arithmetic::mod_mul_truncated::*;
use malachite_base::unsigned_polynomial::UnsignedPolynomial;
use malachite_base::unsigned_polynomial::arithmetic::mod_mul_truncated::*;

#[test]
fn test_mod_mul_truncated_to_out() {
    fn test<T: PrimitiveUnsigned>(xs: &[T], ys: &[T], len: usize, m: T, out: &[T]) {
        let mut result = vec![T::ZERO; len];
        mod_mul_truncated_to_out(&mut result, xs, ys, m);
        assert_eq!(result, out);
        let mut result = vec![T::ZERO; len];
        mod_mul_truncated_to_out_classical(&mut result, xs, ys, m);
        assert_eq!(result, out);
        let mut result = vec![T::ZERO; len];
        mod_mul_truncated_to_out_karatsuba(&mut result, xs, ys, m);
        assert_eq!(result, out);
        assert_eq!(mod_mul_truncated_naive(xs, ys, len, m), out);
    }
    // (x^2+3x+2)(2x+5) = 2x^3+11x^2+19x+10, truncated and reduced modulo 7.
    test::<u8>(&[2, 3, 1], &[5, 2], 1, 7, &[3]);
    test::<u8>(&[2, 3, 1], &[5, 2], 2, 7, &[3, 5]);
    // A length past the end of the product pads with zeros.
    test::<u8>(&[2, 3, 1], &[5, 2], 6, 7, &[3, 5, 4, 2, 0, 0]);
    // (x - 1)(-x - 1) = 1 - x^2 modulo 2^64 - 1.
    test::<u64>(
        &[u64::MAX - 1, 1],
        &[u64::MAX - 1, u64::MAX - 1],
        2,
        u64::MAX,
        &[1, 0],
    );
}

fn mod_mul_truncated_algorithms_helper<T: PrimitiveUnsigned>() {
    for &(n, m, len) in &[
        (1, 1, 1),
        (3, 2, 2),
        (31, 31, 40),
        (32, 32, 32),
        (33, 32, 64),
        (65, 33, 80),
        (80, 80, 80),
        (80, 80, 159),
        (100, 37, 70),
        (129, 128, 129),
        (129, 128, 300),
        (300, 260, 400),
    ] {
        for modulus in test_moduli::<T>() {
            let xs = mod_generated_coefficients::<T>(n, modulus, 1);
            let ys = mod_generated_coefficients::<T>(m, modulus, 2);
            let expected = mod_mul_truncated_naive(&xs, &ys, len, modulus);
            let mut out = vec![T::ZERO; len];
            mod_mul_truncated_to_out(&mut out, &xs, &ys, modulus);
            assert_eq!(out, expected);
            mod_mul_truncated_to_out(&mut out, &ys, &xs, modulus);
            assert_eq!(out, expected);
            mod_mul_truncated_to_out_classical(&mut out, &xs, &ys, modulus);
            assert_eq!(out, expected);
            mod_mul_truncated_to_out_karatsuba(&mut out, &xs, &ys, modulus);
            assert_eq!(out, expected);
        }
    }
}

#[test]
fn test_mod_mul_truncated_algorithms() {
    apply_fn_to_unsigneds!(mod_mul_truncated_algorithms_helper);
}

#[test]
#[should_panic]
fn mod_mul_truncated_to_out_fail_1() {
    mod_mul_truncated_to_out::<u8>(&mut [], &[1, 2], &[3], 7);
}

#[test]
#[should_panic]
fn mod_mul_truncated_to_out_fail_2() {
    mod_mul_truncated_to_out::<u8>(&mut [0; 2], &[1, 2], &[3], 0);
}

fn mod_mul_truncated_properties_helper<T: PrimitiveUnsigned>() {
    unsigned_polynomial_pair_unsigned_unsigned_quadruple_gen_var_2::<T>().test_properties(
        |(p, q, len, m)| {
            let xs = p.coefficients_asc();
            let ys = q.coefficients_asc();
            let len = usize::try_from(len).unwrap();
            if xs.is_empty() || ys.is_empty() || len == 0 {
                return;
            }
            let expected = mod_mul_truncated_naive(xs, ys, len, m);
            let mut out = vec![T::ZERO; len];
            mod_mul_truncated_to_out(&mut out, xs, ys, m);
            assert_eq!(out, expected);
            mod_mul_truncated_to_out_classical(&mut out, xs, ys, m);
            assert_eq!(out, expected);
            mod_mul_truncated_to_out_karatsuba(&mut out, xs, ys, m);
            assert_eq!(out, expected);
            // It is the full product, truncated.
            let mut full = mod_mul_naive(xs, ys, m);
            full.resize(len, T::ZERO);
            assert_eq!(out, full);
        },
    );
}

#[test]
fn mod_mul_truncated_properties() {
    apply_fn_to_unsigneds!(mod_mul_truncated_properties_helper);
}

#[test]
fn test_mod_mul_truncated() {
    fn test<T: PrimitiveUnsigned>(s: &str, t: &str, len: u64, m: T, out: &str) {
        let p = UnsignedPolynomial::<T>::from_str(s).unwrap();
        let q = UnsignedPolynomial::<T>::from_str(t).unwrap();
        // All four combinations of value and reference, and in place with both.
        let r = (&p).mod_mul_truncated(&q, len, m);
        assert!(r.is_valid());
        assert_eq!(r.to_string(), out);
        assert_eq!((&p).mod_mul_truncated(q.clone(), len, m), r);
        assert_eq!(p.clone().mod_mul_truncated(&q, len, m), r);
        assert_eq!(p.clone().mod_mul_truncated(q.clone(), len, m), r);
        let mut s = p.clone();
        s.mod_mul_truncated_assign(&q, len, m);
        assert!(s.is_valid());
        assert_eq!(s, r);
        let mut s = p.clone();
        s.mod_mul_truncated_assign(q.clone(), len, m);
        assert!(s.is_valid());
        assert_eq!(s, r);
        assert_eq!(mod_mul_truncated_polynomial_naive(&p, &q, len, m), r);
    }
    test::<u8>("0", "x", 3, 7, "0");
    test::<u8>("x^2+3*x+2", "2*x+5", 0, 7, "0");
    // The product is 2*x^3+11*x^2+19*x+10; its low two coefficients, modulo 7.
    test::<u8>("x^2+3*x+2", "2*x+5", 2, 7, "5*x+3");
    // A length past the end of the product keeps all of it.
    test::<u8>("x^2+3*x+2", "2*x+5", 10, 7, "2*x^3+4*x^2+5*x+3");
    // The linear coefficient of the product, 7, vanishes modulo 7.
    test::<u8>("x+6", "x+1", 2, 7, "6");
    // Near the top of the type: (x - 1)(-x - 1) = 1 - x^2 modulo 2^64 - 1.
    test::<u64>(
        "x+18446744073709551614",
        "18446744073709551614*x+18446744073709551614",
        2,
        u64::MAX,
        "1",
    );
}

#[test]
#[should_panic]
fn mod_mul_truncated_val_val_self_fail() {
    // A coefficient of self is not reduced.
    let p = UnsignedPolynomial::<u8>::from_str("7*x+1").unwrap();
    let q = UnsignedPolynomial::<u8>::from_str("x").unwrap();
    let _ = p.mod_mul_truncated(q, 2, 7);
}

#[test]
#[should_panic]
fn mod_mul_truncated_val_val_other_fail() {
    // A coefficient of other is not reduced.
    let p = UnsignedPolynomial::<u8>::from_str("x").unwrap();
    let q = UnsignedPolynomial::<u8>::from_str("7*x+1").unwrap();
    let _ = p.mod_mul_truncated(q, 2, 7);
}

#[test]
#[should_panic]
fn mod_mul_truncated_val_val_zero_fail() {
    // The modulus is 0.
    let p = UnsignedPolynomial::<u8>::from_str("0").unwrap();
    let q = UnsignedPolynomial::<u8>::from_str("0").unwrap();
    let _ = p.mod_mul_truncated(q, 2, 0);
}

#[test]
#[should_panic]
fn mod_mul_truncated_val_ref_self_fail() {
    // A coefficient of self is not reduced.
    let p = UnsignedPolynomial::<u8>::from_str("7*x+1").unwrap();
    let q = UnsignedPolynomial::<u8>::from_str("x").unwrap();
    let _ = p.mod_mul_truncated(&q, 2, 7);
}

#[test]
#[should_panic]
fn mod_mul_truncated_val_ref_other_fail() {
    // A coefficient of other is not reduced.
    let p = UnsignedPolynomial::<u8>::from_str("x").unwrap();
    let q = UnsignedPolynomial::<u8>::from_str("7*x+1").unwrap();
    let _ = p.mod_mul_truncated(&q, 2, 7);
}

#[test]
#[should_panic]
fn mod_mul_truncated_val_ref_zero_fail() {
    // The modulus is 0.
    let p = UnsignedPolynomial::<u8>::from_str("0").unwrap();
    let q = UnsignedPolynomial::<u8>::from_str("0").unwrap();
    let _ = p.mod_mul_truncated(&q, 2, 0);
}

#[test]
#[should_panic]
fn mod_mul_truncated_ref_val_self_fail() {
    // A coefficient of self is not reduced.
    let p = UnsignedPolynomial::<u8>::from_str("7*x+1").unwrap();
    let q = UnsignedPolynomial::<u8>::from_str("x").unwrap();
    let _ = (&p).mod_mul_truncated(q, 2, 7);
}

#[test]
#[should_panic]
fn mod_mul_truncated_ref_val_other_fail() {
    // A coefficient of other is not reduced.
    let p = UnsignedPolynomial::<u8>::from_str("x").unwrap();
    let q = UnsignedPolynomial::<u8>::from_str("7*x+1").unwrap();
    let _ = (&p).mod_mul_truncated(q, 2, 7);
}

#[test]
#[should_panic]
fn mod_mul_truncated_ref_val_zero_fail() {
    // The modulus is 0.
    let p = UnsignedPolynomial::<u8>::from_str("0").unwrap();
    let q = UnsignedPolynomial::<u8>::from_str("0").unwrap();
    let _ = (&p).mod_mul_truncated(q, 2, 0);
}

#[test]
#[should_panic]
fn mod_mul_truncated_ref_ref_self_fail() {
    // A coefficient of self is not reduced.
    let p = UnsignedPolynomial::<u8>::from_str("7*x+1").unwrap();
    let q = UnsignedPolynomial::<u8>::from_str("x").unwrap();
    let _ = (&p).mod_mul_truncated(&q, 2, 7);
}

#[test]
#[should_panic]
fn mod_mul_truncated_ref_ref_other_fail() {
    // A coefficient of other is not reduced.
    let p = UnsignedPolynomial::<u8>::from_str("x").unwrap();
    let q = UnsignedPolynomial::<u8>::from_str("7*x+1").unwrap();
    let _ = (&p).mod_mul_truncated(&q, 2, 7);
}

#[test]
#[should_panic]
fn mod_mul_truncated_ref_ref_zero_fail() {
    // The modulus is 0.
    let p = UnsignedPolynomial::<u8>::from_str("0").unwrap();
    let q = UnsignedPolynomial::<u8>::from_str("0").unwrap();
    let _ = (&p).mod_mul_truncated(&q, 2, 0);
}

#[test]
#[should_panic]
fn mod_mul_truncated_assign_self_fail() {
    // A coefficient of self is not reduced.
    let mut p = UnsignedPolynomial::<u8>::from_str("7*x+1").unwrap();
    let q = UnsignedPolynomial::<u8>::from_str("x").unwrap();
    p.mod_mul_truncated_assign(q, 2, 7);
}

#[test]
#[should_panic]
fn mod_mul_truncated_assign_other_fail() {
    // A coefficient of other is not reduced.
    let mut p = UnsignedPolynomial::<u8>::from_str("x").unwrap();
    let q = UnsignedPolynomial::<u8>::from_str("7*x+1").unwrap();
    p.mod_mul_truncated_assign(q, 2, 7);
}

#[test]
#[should_panic]
fn mod_mul_truncated_assign_zero_fail() {
    // The modulus is 0.
    let mut p = UnsignedPolynomial::<u8>::from_str("0").unwrap();
    let q = UnsignedPolynomial::<u8>::from_str("0").unwrap();
    p.mod_mul_truncated_assign(q, 2, 0);
}

#[test]
#[should_panic]
fn mod_mul_truncated_assign_ref_self_fail() {
    // A coefficient of self is not reduced.
    let mut p = UnsignedPolynomial::<u8>::from_str("7*x+1").unwrap();
    let q = UnsignedPolynomial::<u8>::from_str("x").unwrap();
    p.mod_mul_truncated_assign(&q, 2, 7);
}

#[test]
#[should_panic]
fn mod_mul_truncated_assign_ref_other_fail() {
    // A coefficient of other is not reduced.
    let mut p = UnsignedPolynomial::<u8>::from_str("x").unwrap();
    let q = UnsignedPolynomial::<u8>::from_str("7*x+1").unwrap();
    p.mod_mul_truncated_assign(&q, 2, 7);
}

#[test]
#[should_panic]
fn mod_mul_truncated_assign_ref_zero_fail() {
    // The modulus is 0.
    let mut p = UnsignedPolynomial::<u8>::from_str("0").unwrap();
    let q = UnsignedPolynomial::<u8>::from_str("0").unwrap();
    p.mod_mul_truncated_assign(&q, 2, 0);
}

fn mod_mul_truncated_public_properties_helper<T: PrimitiveUnsigned>() {
    unsigned_polynomial_pair_unsigned_unsigned_quadruple_gen_var_2::<T>().test_properties(
        |(p, q, len, m)| {
            let r = (&p).mod_mul_truncated(&q, len, m);
            assert!(r.is_valid());
            // The forms agree.
            assert_eq!((&p).mod_mul_truncated(q.clone(), len, m), r);
            assert_eq!(p.clone().mod_mul_truncated(&q, len, m), r);
            assert_eq!(p.clone().mod_mul_truncated(q.clone(), len, m), r);
            let mut s = p.clone();
            s.mod_mul_truncated_assign(&q, len, m);
            assert!(s.is_valid());
            assert_eq!(s, r);
            let mut s = p.clone();
            s.mod_mul_truncated_assign(q.clone(), len, m);
            assert!(s.is_valid());
            assert_eq!(s, r);

            // The result is reduced, and is the truncation of the whole product.
            assert!(r.mod_is_reduced(&m));
            assert_eq!(mod_mul_truncated_polynomial_naive(&p, &q, len, m), r);
            assert_eq!((&p).mod_mul(&q, m).truncate(len), r);
            // Only the first `len` coefficients of each factor matter.
            assert_eq!(
                p.truncate(len).mod_mul_truncated(q.truncate(len), len, m),
                r
            );
            // Multiplication is commutative.
            assert_eq!((&q).mod_mul_truncated(&p, len, m), r);
        },
    );
}

#[test]
fn mod_mul_truncated_public_properties() {
    apply_fn_to_unsigneds!(mod_mul_truncated_public_properties_helper);
}

// Long polynomials, past the Karatsuba threshold several times over and some unbalanced, against
// the schoolbook reference. The moduli give one-, two-, and three-word accumulation, and with `u8`
// the sums are long enough to be reduced partway through.
fn mod_mul_truncated_long_helper<T: PrimitiveUnsigned>() {
    for &(n, k, len) in
        &[(1000, 999, 1000), (2047, 1500, 1800), (2000, 37, 1500), (1500, 1500, 2999)]
    {
        for m in long_test_moduli::<T>() {
            let xs = mod_generated_coefficients::<T>(n, m, 5);
            let ys = mod_generated_coefficients::<T>(k, m, 6);
            let expected = mod_mul_truncated_naive(&xs, &ys, len, m);
            let mut out = vec![T::ZERO; len];
            mod_mul_truncated_to_out(&mut out, &xs, &ys, m);
            assert_eq!(out, expected);
            mod_mul_truncated_to_out_classical(&mut out, &xs, &ys, m);
            assert_eq!(out, expected);
            mod_mul_truncated_to_out_karatsuba(&mut out, &xs, &ys, m);
            assert_eq!(out, expected);
        }
    }
}

#[test]
fn test_mod_mul_truncated_long() {
    mod_mul_truncated_long_helper::<u8>();
    mod_mul_truncated_long_helper::<u64>();
}
