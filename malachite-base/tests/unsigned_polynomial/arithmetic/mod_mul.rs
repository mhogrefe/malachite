// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.
use core::str::FromStr;
use malachite_base::num::arithmetic::traits::{ModIsReduced, ModMul, ModMulAssign, ModSquare};
use malachite_base::num::basic::traits::Zero;
use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base::polynomial::Polynomial;
use malachite_base::test_util::generators::*;
use malachite_base::test_util::unsigned_polynomial::arithmetic::mod_mul::*;
use malachite_base::unsigned_polynomial::UnsignedPolynomial;
use malachite_base::unsigned_polynomial::arithmetic::mod_mul::*;

#[test]
fn test_mod_mul_to_out() {
    fn test<T: PrimitiveUnsigned>(xs: &[T], ys: &[T], m: T, out: &[T]) {
        let len = xs.len() + ys.len() - 1;
        let mut result = vec![T::ZERO; len];
        mod_mul_to_out(&mut result, xs, ys, m);
        assert_eq!(result, out);
        let mut result = vec![T::ZERO; len];
        mod_mul_to_out_classical(&mut result, xs, ys, m);
        assert_eq!(result, out);
        let mut result = vec![T::ZERO; len];
        mod_mul_to_out_karatsuba(&mut result, xs, ys, m);
        assert_eq!(result, out);
        assert_eq!(mod_mul_naive(xs, ys, m), out);
    }
    test::<u8>(&[0], &[0], 1, &[0]);
    // (x^2+3x+2)(2x+5) = 2x^3+11x^2+19x+10, reduced modulo 7.
    test::<u8>(&[2, 3, 1], &[5, 2], 7, &[3, 5, 4, 2]);
    test::<u64>(&[2, 3, 1], &[5, 2], 1000, &[10, 19, 11, 2]);
    // (x - 1)(-x - 1) = 1 - x^2 modulo 255.
    test::<u8>(&[254, 1], &[254, 254], 255, &[1, 0, 254]);
    test::<u128>(&[u128::MAX - 1], &[2], u128::MAX, &[u128::MAX - 2]);
}

fn mod_mul_algorithms_helper<T: PrimitiveUnsigned>() {
    for &(n, m) in &[
        (1, 1),
        (3, 2),
        (31, 31),
        (32, 32),
        (33, 32),
        (65, 33),
        (80, 80),
        (100, 37),
        (129, 128),
        (300, 260),
    ] {
        for modulus in test_moduli::<T>() {
            let xs = mod_generated_coefficients::<T>(n, modulus, 1);
            let ys = mod_generated_coefficients::<T>(m, modulus, 2);
            let expected = mod_mul_naive(&xs, &ys, modulus);
            let mut out = vec![T::ZERO; n + m - 1];
            mod_mul_to_out(&mut out, &xs, &ys, modulus);
            assert_eq!(out, expected);
            mod_mul_to_out(&mut out, &ys, &xs, modulus);
            assert_eq!(out, expected);
            mod_mul_to_out_classical(&mut out, &xs, &ys, modulus);
            assert_eq!(out, expected);
            mod_mul_to_out_karatsuba(&mut out, &xs, &ys, modulus);
            assert_eq!(out, expected);
        }
    }
}

#[test]
fn test_mod_mul_algorithms() {
    apply_fn_to_unsigneds!(mod_mul_algorithms_helper);
}

#[test]
#[should_panic]
fn mod_mul_to_out_fail_1() {
    mod_mul_to_out::<u8>(&mut [0; 3], &[1, 2], &[3], 7);
}

#[test]
#[should_panic]
fn mod_mul_to_out_fail_2() {
    mod_mul_to_out::<u8>(&mut [0; 2], &[1, 2], &[3], 0);
}

fn mod_mul_properties_helper<T: PrimitiveUnsigned>() {
    unsigned_polynomial_unsigned_polynomial_unsigned_triple_gen_var_2::<T>().test_properties(
        |(p, q, m)| {
            let xs = p.coefficients_asc();
            let ys = q.coefficients_asc();
            if xs.is_empty() || ys.is_empty() {
                return;
            }
            let expected = mod_mul_naive(xs, ys, m);
            let mut out = vec![T::ZERO; xs.len() + ys.len() - 1];
            mod_mul_to_out(&mut out, xs, ys, m);
            assert_eq!(out, expected);
            mod_mul_to_out(&mut out, ys, xs, m);
            assert_eq!(out, expected);
            mod_mul_to_out_classical(&mut out, xs, ys, m);
            assert_eq!(out, expected);
            mod_mul_to_out_karatsuba(&mut out, xs, ys, m);
            assert_eq!(out, expected);
        },
    );
}

#[test]
fn mod_mul_properties() {
    apply_fn_to_unsigneds!(mod_mul_properties_helper);
}

#[test]
fn test_mod_mul() {
    fn test<T: PrimitiveUnsigned>(s: &str, t: &str, m: T, out: &str) {
        let p = UnsignedPolynomial::<T>::from_str(s).unwrap();
        let q = UnsignedPolynomial::<T>::from_str(t).unwrap();
        // All four combinations of value and reference, and in place with both.
        let r = (&p).mod_mul(&q, m);
        assert!(r.is_valid());
        assert_eq!(r.to_string(), out);
        assert_eq!((&p).mod_mul(q.clone(), m), r);
        assert_eq!(p.clone().mod_mul(&q, m), r);
        assert_eq!(p.clone().mod_mul(q.clone(), m), r);
        let mut s = p.clone();
        s.mod_mul_assign(&q, m);
        assert!(s.is_valid());
        assert_eq!(s, r);
        let mut s = p.clone();
        s.mod_mul_assign(q.clone(), m);
        assert!(s.is_valid());
        assert_eq!(s, r);
        assert_eq!(mod_mul_polynomial_naive(&p, &q, m), r);
    }
    // Modulo 1, only the zero polynomial is reduced.
    test::<u8>("0", "0", 1, "0");
    // Multiplying by zero, either way round.
    test::<u8>("x+1", "0", 7, "0");
    test::<u8>("0", "x+1", 7, "0");
    // Multiplying by one.
    test::<u8>("1", "x^2+x+1", 7, "x^2+x+1");
    // The product is 2*x^3+11*x^2+19*x+10; its coefficients modulo 7.
    test::<u8>("x^2+3*x+2", "2*x+5", 7, "2*x^3+4*x^2+5*x+3");
    // The leading coefficient of the product, 6, vanishes modulo 6, so the degree drops.
    test::<u8>("2*x+1", "3*x+1", 6, "5*x+1");
    // Near the top of the type: (x - 1)(-x - 1) = 1 - x^2 modulo 255.
    test::<u8>("x+254", "254*x+254", 255, "254*x^2+1");
    test::<u64>(
        "x+18446744073709551614",
        "18446744073709551614*x+18446744073709551614",
        u64::MAX,
        "18446744073709551614*x^2+1",
    );
}

#[test]
#[should_panic]
fn mod_mul_val_val_self_fail() {
    // A coefficient of self is not reduced.
    let p = UnsignedPolynomial::<u8>::from_str("7*x+1").unwrap();
    let q = UnsignedPolynomial::<u8>::from_str("x").unwrap();
    let _ = p.mod_mul(q, 7);
}

#[test]
#[should_panic]
fn mod_mul_val_val_other_fail() {
    // A coefficient of other is not reduced.
    let p = UnsignedPolynomial::<u8>::from_str("x").unwrap();
    let q = UnsignedPolynomial::<u8>::from_str("7*x+1").unwrap();
    let _ = p.mod_mul(q, 7);
}

#[test]
#[should_panic]
fn mod_mul_val_val_zero_fail() {
    // The modulus is 0.
    let p = UnsignedPolynomial::<u8>::from_str("0").unwrap();
    let q = UnsignedPolynomial::<u8>::from_str("0").unwrap();
    let _ = p.mod_mul(q, 0);
}

#[test]
#[should_panic]
fn mod_mul_val_ref_self_fail() {
    // A coefficient of self is not reduced.
    let p = UnsignedPolynomial::<u8>::from_str("7*x+1").unwrap();
    let q = UnsignedPolynomial::<u8>::from_str("x").unwrap();
    let _ = p.mod_mul(&q, 7);
}

#[test]
#[should_panic]
fn mod_mul_val_ref_other_fail() {
    // A coefficient of other is not reduced.
    let p = UnsignedPolynomial::<u8>::from_str("x").unwrap();
    let q = UnsignedPolynomial::<u8>::from_str("7*x+1").unwrap();
    let _ = p.mod_mul(&q, 7);
}

#[test]
#[should_panic]
fn mod_mul_val_ref_zero_fail() {
    // The modulus is 0.
    let p = UnsignedPolynomial::<u8>::from_str("0").unwrap();
    let q = UnsignedPolynomial::<u8>::from_str("0").unwrap();
    let _ = p.mod_mul(&q, 0);
}

#[test]
#[should_panic]
fn mod_mul_ref_val_self_fail() {
    // A coefficient of self is not reduced.
    let p = UnsignedPolynomial::<u8>::from_str("7*x+1").unwrap();
    let q = UnsignedPolynomial::<u8>::from_str("x").unwrap();
    let _ = (&p).mod_mul(q, 7);
}

#[test]
#[should_panic]
fn mod_mul_ref_val_other_fail() {
    // A coefficient of other is not reduced.
    let p = UnsignedPolynomial::<u8>::from_str("x").unwrap();
    let q = UnsignedPolynomial::<u8>::from_str("7*x+1").unwrap();
    let _ = (&p).mod_mul(q, 7);
}

#[test]
#[should_panic]
fn mod_mul_ref_val_zero_fail() {
    // The modulus is 0.
    let p = UnsignedPolynomial::<u8>::from_str("0").unwrap();
    let q = UnsignedPolynomial::<u8>::from_str("0").unwrap();
    let _ = (&p).mod_mul(q, 0);
}

#[test]
#[should_panic]
fn mod_mul_ref_ref_self_fail() {
    // A coefficient of self is not reduced.
    let p = UnsignedPolynomial::<u8>::from_str("7*x+1").unwrap();
    let q = UnsignedPolynomial::<u8>::from_str("x").unwrap();
    let _ = (&p).mod_mul(&q, 7);
}

#[test]
#[should_panic]
fn mod_mul_ref_ref_other_fail() {
    // A coefficient of other is not reduced.
    let p = UnsignedPolynomial::<u8>::from_str("x").unwrap();
    let q = UnsignedPolynomial::<u8>::from_str("7*x+1").unwrap();
    let _ = (&p).mod_mul(&q, 7);
}

#[test]
#[should_panic]
fn mod_mul_ref_ref_zero_fail() {
    // The modulus is 0.
    let p = UnsignedPolynomial::<u8>::from_str("0").unwrap();
    let q = UnsignedPolynomial::<u8>::from_str("0").unwrap();
    let _ = (&p).mod_mul(&q, 0);
}

#[test]
#[should_panic]
fn mod_mul_assign_self_fail() {
    // A coefficient of self is not reduced.
    let mut p = UnsignedPolynomial::<u8>::from_str("7*x+1").unwrap();
    let q = UnsignedPolynomial::<u8>::from_str("x").unwrap();
    p.mod_mul_assign(q, 7);
}

#[test]
#[should_panic]
fn mod_mul_assign_other_fail() {
    // A coefficient of other is not reduced.
    let mut p = UnsignedPolynomial::<u8>::from_str("x").unwrap();
    let q = UnsignedPolynomial::<u8>::from_str("7*x+1").unwrap();
    p.mod_mul_assign(q, 7);
}

#[test]
#[should_panic]
fn mod_mul_assign_zero_fail() {
    // The modulus is 0.
    let mut p = UnsignedPolynomial::<u8>::from_str("0").unwrap();
    let q = UnsignedPolynomial::<u8>::from_str("0").unwrap();
    p.mod_mul_assign(q, 0);
}

#[test]
#[should_panic]
fn mod_mul_assign_ref_self_fail() {
    // A coefficient of self is not reduced.
    let mut p = UnsignedPolynomial::<u8>::from_str("7*x+1").unwrap();
    let q = UnsignedPolynomial::<u8>::from_str("x").unwrap();
    p.mod_mul_assign(&q, 7);
}

#[test]
#[should_panic]
fn mod_mul_assign_ref_other_fail() {
    // A coefficient of other is not reduced.
    let mut p = UnsignedPolynomial::<u8>::from_str("x").unwrap();
    let q = UnsignedPolynomial::<u8>::from_str("7*x+1").unwrap();
    p.mod_mul_assign(&q, 7);
}

#[test]
#[should_panic]
fn mod_mul_assign_ref_zero_fail() {
    // The modulus is 0.
    let mut p = UnsignedPolynomial::<u8>::from_str("0").unwrap();
    let q = UnsignedPolynomial::<u8>::from_str("0").unwrap();
    p.mod_mul_assign(&q, 0);
}

fn mod_mul_public_properties_helper<T: PrimitiveUnsigned>() {
    unsigned_polynomial_unsigned_polynomial_unsigned_triple_gen_var_2::<T>().test_properties(
        |(p, q, m)| {
            let r = (&p).mod_mul(&q, m);
            assert!(r.is_valid());
            // The forms agree.
            assert_eq!((&p).mod_mul(q.clone(), m), r);
            assert_eq!(p.clone().mod_mul(&q, m), r);
            assert_eq!(p.clone().mod_mul(q.clone(), m), r);
            let mut s = p.clone();
            s.mod_mul_assign(&q, m);
            assert!(s.is_valid());
            assert_eq!(s, r);
            let mut s = p.clone();
            s.mod_mul_assign(q.clone(), m);
            assert!(s.is_valid());
            assert_eq!(s, r);

            // The result is reduced, and is the schoolbook product.
            assert!(r.mod_is_reduced(&m));
            assert_eq!(mod_mul_polynomial_naive(&p, &q, m), r);
            // Multiplication is commutative.
            assert_eq!((&q).mod_mul(&p, m), r);
            // The product of a polynomial with itself is its square.
            assert_eq!((&p).mod_mul(&p, m), (&p).mod_square(m));
            // Multiplying by zero gives zero, and multiplying by one changes nothing.
            assert_eq!(
                (&p).mod_mul(&UnsignedPolynomial::ZERO, m),
                UnsignedPolynomial::ZERO
            );
            if m != T::ONE {
                let one = UnsignedPolynomial::from_coefficients_asc(vec![T::ONE]);
                assert_eq!((&p).mod_mul(&one, m), p);
            }
        },
    );
}

#[test]
fn mod_mul_public_properties() {
    apply_fn_to_unsigneds!(mod_mul_public_properties_helper);
}
