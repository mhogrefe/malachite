// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.
use core::str::FromStr;
use malachite_base::num::arithmetic::traits::{ModPowerOf2IsReduced, ModPowerOf2Mul};
use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base::polynomial::{
    ModPowerOf2MulTruncated, ModPowerOf2MulTruncatedAssign, Polynomial,
};
use malachite_base::test_util::generators::*;
use malachite_base::test_util::unsigned_polynomial::arithmetic::mod_power_of_2_mul::*;
use malachite_base::test_util::unsigned_polynomial::arithmetic::mod_power_of_2_mul_truncated::*;
use malachite_base::unsigned_polynomial::UnsignedPolynomial;
use malachite_base::unsigned_polynomial::arithmetic::mod_power_of_2_mul_truncated::*;

#[test]
fn test_mod_power_of_2_mul_truncated_to_out() {
    fn test<T: PrimitiveUnsigned>(xs: &[T], ys: &[T], len: usize, pow: u64, out: &[T]) {
        let mut result = vec![T::ZERO; len];
        mod_power_of_2_mul_truncated_to_out(&mut result, xs, ys, pow);
        assert_eq!(result, out);
        let mut result = vec![T::ZERO; len];
        mod_power_of_2_mul_truncated_to_out_classical(&mut result, xs, ys, pow);
        assert_eq!(result, out);
        let mut result = vec![T::ZERO; len];
        mod_power_of_2_mul_truncated_to_out_karatsuba(&mut result, xs, ys, pow);
        assert_eq!(result, out);
        assert_eq!(mod_power_of_2_mul_truncated_naive(xs, ys, len, pow), out);
    }
    // (x^2+3x+2)(2x+5) = 2x^3+11x^2+19x+10, truncated and reduced modulo 16.
    test::<u8>(&[2, 3, 1], &[5, 2], 1, 4, &[10]);
    test::<u8>(&[2, 3, 1], &[5, 2], 2, 4, &[10, 3]);
    // A length past the end of the product pads with zeros.
    test::<u8>(&[2, 3, 1], &[5, 2], 6, 4, &[10, 3, 11, 2, 0, 0]);
    // (x - 1)(-x - 1) = 1 - x^2 modulo 2^64.
    test::<u64>(&[u64::MAX, 1], &[u64::MAX, u64::MAX], 2, 64, &[1, 0]);
}

fn mod_power_of_2_mul_truncated_algorithms_helper<T: PrimitiveUnsigned>() {
    for &(n, m, len) in &[
        (1, 1, 1),
        (3, 2, 2),
        (15, 15, 20),
        (16, 16, 16),
        (17, 16, 31),
        (33, 17, 40),
        (40, 40, 40),
        (40, 40, 79),
        (100, 37, 70),
        (129, 128, 129),
        (129, 128, 300),
    ] {
        for pow in [0, 1, T::WIDTH >> 1, T::WIDTH - 1, T::WIDTH] {
            let xs = mod_power_of_2_generated_coefficients::<T>(n, pow, 1);
            let ys = mod_power_of_2_generated_coefficients::<T>(m, pow, 2);
            let expected = mod_power_of_2_mul_truncated_naive(&xs, &ys, len, pow);
            let mut out = vec![T::ZERO; len];
            mod_power_of_2_mul_truncated_to_out(&mut out, &xs, &ys, pow);
            assert_eq!(out, expected);
            mod_power_of_2_mul_truncated_to_out(&mut out, &ys, &xs, pow);
            assert_eq!(out, expected);
            mod_power_of_2_mul_truncated_to_out_classical(&mut out, &xs, &ys, pow);
            assert_eq!(out, expected);
            mod_power_of_2_mul_truncated_to_out_karatsuba(&mut out, &xs, &ys, pow);
            assert_eq!(out, expected);
        }
    }
}

#[test]
fn test_mod_power_of_2_mul_truncated_algorithms() {
    apply_fn_to_unsigneds!(mod_power_of_2_mul_truncated_algorithms_helper);
}

#[test]
#[should_panic]
fn mod_power_of_2_mul_truncated_to_out_fail() {
    mod_power_of_2_mul_truncated_to_out::<u8>(&mut [0; 2], &[1, 2], &[3], 9);
}

fn mod_power_of_2_mul_truncated_properties_helper<T: PrimitiveUnsigned>() {
    unsigned_polynomial_pair_unsigned_unsigned_quadruple_gen_var_1::<T>().test_properties(
        |(p, q, len, pow)| {
            let xs = p.coefficients_asc();
            let ys = q.coefficients_asc();
            let len = usize::try_from(len).unwrap();
            if xs.is_empty() || ys.is_empty() || len == 0 {
                return;
            }
            let expected = mod_power_of_2_mul_truncated_naive(xs, ys, len, pow);
            let mut out = vec![T::ZERO; len];
            mod_power_of_2_mul_truncated_to_out(&mut out, xs, ys, pow);
            assert_eq!(out, expected);
            mod_power_of_2_mul_truncated_to_out_classical(&mut out, xs, ys, pow);
            assert_eq!(out, expected);
            mod_power_of_2_mul_truncated_to_out_karatsuba(&mut out, xs, ys, pow);
            assert_eq!(out, expected);
            // It is the full product, truncated.
            let mut full = mod_power_of_2_mul_naive(xs, ys, pow);
            full.resize(len, T::ZERO);
            assert_eq!(out, full);
        },
    );
}

#[test]
fn mod_power_of_2_mul_truncated_properties() {
    apply_fn_to_unsigneds!(mod_power_of_2_mul_truncated_properties_helper);
}

#[test]
fn test_mod_power_of_2_mul_truncated() {
    fn test<T: PrimitiveUnsigned>(s: &str, t: &str, len: u64, pow: u64, out: &str) {
        let p = UnsignedPolynomial::<T>::from_str(s).unwrap();
        let q = UnsignedPolynomial::<T>::from_str(t).unwrap();
        // All four combinations of value and reference, and in place with both.
        let r = (&p).mod_power_of_2_mul_truncated(&q, len, pow);
        assert!(r.is_valid());
        assert_eq!(r.to_string(), out);
        assert_eq!((&p).mod_power_of_2_mul_truncated(q.clone(), len, pow), r);
        assert_eq!(p.clone().mod_power_of_2_mul_truncated(&q, len, pow), r);
        assert_eq!(
            p.clone().mod_power_of_2_mul_truncated(q.clone(), len, pow),
            r
        );
        let mut s = p.clone();
        s.mod_power_of_2_mul_truncated_assign(&q, len, pow);
        assert!(s.is_valid());
        assert_eq!(s, r);
        let mut s = p.clone();
        s.mod_power_of_2_mul_truncated_assign(q.clone(), len, pow);
        assert!(s.is_valid());
        assert_eq!(s, r);
        assert_eq!(
            mod_power_of_2_mul_truncated_polynomial_naive(&p, &q, len, pow),
            r
        );
    }
    test::<u8>("0", "x", 3, 3, "0");
    test::<u8>("x^2+3*x+2", "2*x+5", 0, 4, "0");
    // The product is 2*x^3+11*x^2+19*x+10; its low two coefficients, modulo 16.
    test::<u8>("x^2+3*x+2", "2*x+5", 2, 4, "3*x+10");
    // A length past the end of the product keeps all of it.
    test::<u8>("x^2+3*x+2", "2*x+5", 10, 4, "2*x^3+11*x^2+3*x+10");
    // The linear coefficient of the product, 16, vanishes modulo 16.
    test::<u8>("x+15", "x+1", 2, 4, "15");
    // The full width of the type: (x - 1)(-x - 1) = 1 - x^2.
    test::<u64>(
        "x+18446744073709551615",
        "18446744073709551615*x+18446744073709551615",
        2,
        64,
        "1",
    );
}

#[test]
#[should_panic]
fn mod_power_of_2_mul_truncated_val_val_self_fail() {
    // A coefficient of self is not reduced.
    let p = UnsignedPolynomial::<u8>::from_str("8*x+1").unwrap();
    let q = UnsignedPolynomial::<u8>::from_str("x").unwrap();
    let _ = p.mod_power_of_2_mul_truncated(q, 2, 3);
}

#[test]
#[should_panic]
fn mod_power_of_2_mul_truncated_val_val_other_fail() {
    // A coefficient of other is not reduced.
    let p = UnsignedPolynomial::<u8>::from_str("x").unwrap();
    let q = UnsignedPolynomial::<u8>::from_str("8*x+1").unwrap();
    let _ = p.mod_power_of_2_mul_truncated(q, 2, 3);
}

#[test]
#[should_panic]
fn mod_power_of_2_mul_truncated_val_val_pow_fail() {
    // pow is greater than the width of the type.
    let p = UnsignedPolynomial::<u8>::from_str("x").unwrap();
    let q = UnsignedPolynomial::<u8>::from_str("x").unwrap();
    let _ = p.mod_power_of_2_mul_truncated(q, 2, 9);
}

#[test]
#[should_panic]
fn mod_power_of_2_mul_truncated_val_ref_self_fail() {
    // A coefficient of self is not reduced.
    let p = UnsignedPolynomial::<u8>::from_str("8*x+1").unwrap();
    let q = UnsignedPolynomial::<u8>::from_str("x").unwrap();
    let _ = p.mod_power_of_2_mul_truncated(&q, 2, 3);
}

#[test]
#[should_panic]
fn mod_power_of_2_mul_truncated_val_ref_other_fail() {
    // A coefficient of other is not reduced.
    let p = UnsignedPolynomial::<u8>::from_str("x").unwrap();
    let q = UnsignedPolynomial::<u8>::from_str("8*x+1").unwrap();
    let _ = p.mod_power_of_2_mul_truncated(&q, 2, 3);
}

#[test]
#[should_panic]
fn mod_power_of_2_mul_truncated_val_ref_pow_fail() {
    // pow is greater than the width of the type.
    let p = UnsignedPolynomial::<u8>::from_str("x").unwrap();
    let q = UnsignedPolynomial::<u8>::from_str("x").unwrap();
    let _ = p.mod_power_of_2_mul_truncated(&q, 2, 9);
}

#[test]
#[should_panic]
fn mod_power_of_2_mul_truncated_ref_val_self_fail() {
    // A coefficient of self is not reduced.
    let p = UnsignedPolynomial::<u8>::from_str("8*x+1").unwrap();
    let q = UnsignedPolynomial::<u8>::from_str("x").unwrap();
    let _ = (&p).mod_power_of_2_mul_truncated(q, 2, 3);
}

#[test]
#[should_panic]
fn mod_power_of_2_mul_truncated_ref_val_other_fail() {
    // A coefficient of other is not reduced.
    let p = UnsignedPolynomial::<u8>::from_str("x").unwrap();
    let q = UnsignedPolynomial::<u8>::from_str("8*x+1").unwrap();
    let _ = (&p).mod_power_of_2_mul_truncated(q, 2, 3);
}

#[test]
#[should_panic]
fn mod_power_of_2_mul_truncated_ref_val_pow_fail() {
    // pow is greater than the width of the type.
    let p = UnsignedPolynomial::<u8>::from_str("x").unwrap();
    let q = UnsignedPolynomial::<u8>::from_str("x").unwrap();
    let _ = (&p).mod_power_of_2_mul_truncated(q, 2, 9);
}

#[test]
#[should_panic]
fn mod_power_of_2_mul_truncated_ref_ref_self_fail() {
    // A coefficient of self is not reduced.
    let p = UnsignedPolynomial::<u8>::from_str("8*x+1").unwrap();
    let q = UnsignedPolynomial::<u8>::from_str("x").unwrap();
    let _ = (&p).mod_power_of_2_mul_truncated(&q, 2, 3);
}

#[test]
#[should_panic]
fn mod_power_of_2_mul_truncated_ref_ref_other_fail() {
    // A coefficient of other is not reduced.
    let p = UnsignedPolynomial::<u8>::from_str("x").unwrap();
    let q = UnsignedPolynomial::<u8>::from_str("8*x+1").unwrap();
    let _ = (&p).mod_power_of_2_mul_truncated(&q, 2, 3);
}

#[test]
#[should_panic]
fn mod_power_of_2_mul_truncated_ref_ref_pow_fail() {
    // pow is greater than the width of the type.
    let p = UnsignedPolynomial::<u8>::from_str("x").unwrap();
    let q = UnsignedPolynomial::<u8>::from_str("x").unwrap();
    let _ = (&p).mod_power_of_2_mul_truncated(&q, 2, 9);
}

#[test]
#[should_panic]
fn mod_power_of_2_mul_truncated_assign_self_fail() {
    // A coefficient of self is not reduced.
    let mut p = UnsignedPolynomial::<u8>::from_str("8*x+1").unwrap();
    let q = UnsignedPolynomial::<u8>::from_str("x").unwrap();
    p.mod_power_of_2_mul_truncated_assign(q, 2, 3);
}

#[test]
#[should_panic]
fn mod_power_of_2_mul_truncated_assign_other_fail() {
    // A coefficient of other is not reduced.
    let mut p = UnsignedPolynomial::<u8>::from_str("x").unwrap();
    let q = UnsignedPolynomial::<u8>::from_str("8*x+1").unwrap();
    p.mod_power_of_2_mul_truncated_assign(q, 2, 3);
}

#[test]
#[should_panic]
fn mod_power_of_2_mul_truncated_assign_pow_fail() {
    // pow is greater than the width of the type.
    let mut p = UnsignedPolynomial::<u8>::from_str("x").unwrap();
    let q = UnsignedPolynomial::<u8>::from_str("x").unwrap();
    p.mod_power_of_2_mul_truncated_assign(q, 2, 9);
}

#[test]
#[should_panic]
fn mod_power_of_2_mul_truncated_assign_ref_self_fail() {
    // A coefficient of self is not reduced.
    let mut p = UnsignedPolynomial::<u8>::from_str("8*x+1").unwrap();
    let q = UnsignedPolynomial::<u8>::from_str("x").unwrap();
    p.mod_power_of_2_mul_truncated_assign(&q, 2, 3);
}

#[test]
#[should_panic]
fn mod_power_of_2_mul_truncated_assign_ref_other_fail() {
    // A coefficient of other is not reduced.
    let mut p = UnsignedPolynomial::<u8>::from_str("x").unwrap();
    let q = UnsignedPolynomial::<u8>::from_str("8*x+1").unwrap();
    p.mod_power_of_2_mul_truncated_assign(&q, 2, 3);
}

#[test]
#[should_panic]
fn mod_power_of_2_mul_truncated_assign_ref_pow_fail() {
    // pow is greater than the width of the type.
    let mut p = UnsignedPolynomial::<u8>::from_str("x").unwrap();
    let q = UnsignedPolynomial::<u8>::from_str("x").unwrap();
    p.mod_power_of_2_mul_truncated_assign(&q, 2, 9);
}

fn mod_power_of_2_mul_truncated_public_properties_helper<T: PrimitiveUnsigned>() {
    unsigned_polynomial_pair_unsigned_unsigned_quadruple_gen_var_1::<T>().test_properties(
        |(p, q, len, pow)| {
            let r = (&p).mod_power_of_2_mul_truncated(&q, len, pow);
            assert!(r.is_valid());
            // The forms agree.
            assert_eq!((&p).mod_power_of_2_mul_truncated(q.clone(), len, pow), r);
            assert_eq!(p.clone().mod_power_of_2_mul_truncated(&q, len, pow), r);
            assert_eq!(
                p.clone().mod_power_of_2_mul_truncated(q.clone(), len, pow),
                r
            );
            let mut s = p.clone();
            s.mod_power_of_2_mul_truncated_assign(&q, len, pow);
            assert!(s.is_valid());
            assert_eq!(s, r);
            let mut s = p.clone();
            s.mod_power_of_2_mul_truncated_assign(q.clone(), len, pow);
            assert!(s.is_valid());
            assert_eq!(s, r);

            // The result is reduced, and is the truncation of the whole product.
            assert!(r.mod_power_of_2_is_reduced(pow));
            assert_eq!(
                mod_power_of_2_mul_truncated_polynomial_naive(&p, &q, len, pow),
                r
            );
            assert_eq!((&p).mod_power_of_2_mul(&q, pow).truncate(len), r);
            // Only the first `len` coefficients of each factor matter.
            assert_eq!(
                p.truncate(len)
                    .mod_power_of_2_mul_truncated(q.truncate(len), len, pow),
                r
            );
            // Multiplication is commutative.
            assert_eq!((&q).mod_power_of_2_mul_truncated(&p, len, pow), r);
        },
    );
}

#[test]
fn mod_power_of_2_mul_truncated_public_properties() {
    apply_fn_to_unsigneds!(mod_power_of_2_mul_truncated_public_properties_helper);
}
