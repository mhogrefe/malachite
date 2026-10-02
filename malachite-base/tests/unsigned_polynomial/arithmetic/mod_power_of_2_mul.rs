// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.
use core::str::FromStr;
use malachite_base::num::arithmetic::traits::{
    ModPowerOf2IsReduced, ModPowerOf2Mul, ModPowerOf2MulAssign, ModPowerOf2Square,
};
use malachite_base::num::basic::traits::Zero;
use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base::polynomial::Polynomial;
use malachite_base::test_util::generators::*;
use malachite_base::test_util::unsigned_polynomial::arithmetic::mod_power_of_2_mul::*;
use malachite_base::unsigned_polynomial::UnsignedPolynomial;
use malachite_base::unsigned_polynomial::arithmetic::mod_power_of_2_mul::*;

#[test]
fn test_mod_power_of_2_mul_to_out() {
    fn test<T: PrimitiveUnsigned>(xs: &[T], ys: &[T], pow: u64, out: &[T]) {
        let len = xs.len() + ys.len() - 1;
        let mut result = vec![T::ZERO; len];
        mod_power_of_2_mul_to_out(&mut result, xs, ys, pow);
        assert_eq!(result, out);
        let mut result = vec![T::ZERO; len];
        mod_power_of_2_mul_to_out_classical(&mut result, xs, ys, pow);
        assert_eq!(result, out);
        let mut result = vec![T::ZERO; len];
        mod_power_of_2_mul_to_out_karatsuba(&mut result, xs, ys, pow);
        assert_eq!(result, out);
        assert_eq!(mod_power_of_2_mul_naive(xs, ys, pow), out);
    }
    test::<u8>(&[0], &[0], 0, &[0]);
    // (x^2+3x+2)(2x+5) = 2x^3+11x^2+19x+10, reduced modulo 16.
    test::<u8>(&[2, 3, 1], &[5, 2], 4, &[10, 3, 11, 2]);
    test::<u64>(&[2, 3, 1], &[5, 2], 64, &[10, 19, 11, 2]);
    // The products wrap around modulo 2^64: (x - 1)(-x - 1) = 1 - x^2.
    test::<u64>(&[u64::MAX, 1], &[u64::MAX, u64::MAX], 64, &[1, 0, u64::MAX]);
    test::<u128>(&[u128::MAX], &[2], 128, &[u128::MAX - 1]);
}

fn mod_power_of_2_mul_algorithms_helper<T: PrimitiveUnsigned>() {
    for &(n, m) in
        &[(1, 1), (3, 2), (15, 15), (16, 16), (17, 16), (33, 17), (40, 40), (100, 37), (129, 128)]
    {
        for pow in [0, 1, T::WIDTH >> 1, T::WIDTH - 1, T::WIDTH] {
            let xs = mod_power_of_2_generated_coefficients::<T>(n, pow, 1);
            let ys = mod_power_of_2_generated_coefficients::<T>(m, pow, 2);
            let expected = mod_power_of_2_mul_naive(&xs, &ys, pow);
            let mut out = vec![T::ZERO; n + m - 1];
            mod_power_of_2_mul_to_out(&mut out, &xs, &ys, pow);
            assert_eq!(out, expected);
            mod_power_of_2_mul_to_out(&mut out, &ys, &xs, pow);
            assert_eq!(out, expected);
            mod_power_of_2_mul_to_out_classical(&mut out, &xs, &ys, pow);
            assert_eq!(out, expected);
            mod_power_of_2_mul_to_out_karatsuba(&mut out, &xs, &ys, pow);
            assert_eq!(out, expected);
        }
    }
}

#[test]
fn test_mod_power_of_2_mul_algorithms() {
    apply_fn_to_unsigneds!(mod_power_of_2_mul_algorithms_helper);
}

#[test]
#[should_panic]
fn mod_power_of_2_mul_to_out_fail_1() {
    mod_power_of_2_mul_to_out::<u8>(&mut [0; 2], &[1, 2], &[3], 9);
}

#[test]
#[should_panic]
fn mod_power_of_2_mul_to_out_fail_2() {
    mod_power_of_2_mul_to_out::<u8>(&mut [0; 3], &[1, 2], &[3], 4);
}

fn mod_power_of_2_mul_properties_helper<T: PrimitiveUnsigned>() {
    unsigned_polynomial_unsigned_polynomial_unsigned_triple_gen_var_1::<T>().test_properties(
        |(p, q, pow)| {
            let xs = p.coefficients_asc();
            let ys = q.coefficients_asc();
            if xs.is_empty() || ys.is_empty() {
                return;
            }
            let expected = mod_power_of_2_mul_naive(xs, ys, pow);
            let mut out = vec![T::ZERO; xs.len() + ys.len() - 1];
            mod_power_of_2_mul_to_out(&mut out, xs, ys, pow);
            assert_eq!(out, expected);
            mod_power_of_2_mul_to_out(&mut out, ys, xs, pow);
            assert_eq!(out, expected);
            mod_power_of_2_mul_to_out_classical(&mut out, xs, ys, pow);
            assert_eq!(out, expected);
            mod_power_of_2_mul_to_out_karatsuba(&mut out, xs, ys, pow);
            assert_eq!(out, expected);
        },
    );
}

#[test]
fn mod_power_of_2_mul_properties() {
    apply_fn_to_unsigneds!(mod_power_of_2_mul_properties_helper);
}

#[test]
fn test_mod_power_of_2_mul() {
    fn test<T: PrimitiveUnsigned>(s: &str, t: &str, pow: u64, out: &str) {
        let p = UnsignedPolynomial::<T>::from_str(s).unwrap();
        let q = UnsignedPolynomial::<T>::from_str(t).unwrap();
        // All four combinations of value and reference, and in place with both.
        let r = (&p).mod_power_of_2_mul(&q, pow);
        assert!(r.is_valid());
        assert_eq!(r.to_string(), out);
        assert_eq!((&p).mod_power_of_2_mul(q.clone(), pow), r);
        assert_eq!(p.clone().mod_power_of_2_mul(&q, pow), r);
        assert_eq!(p.clone().mod_power_of_2_mul(q.clone(), pow), r);
        let mut s = p.clone();
        s.mod_power_of_2_mul_assign(&q, pow);
        assert!(s.is_valid());
        assert_eq!(s, r);
        let mut s = p.clone();
        s.mod_power_of_2_mul_assign(q.clone(), pow);
        assert!(s.is_valid());
        assert_eq!(s, r);
        assert_eq!(mod_power_of_2_mul_polynomial_naive(&p, &q, pow), r);
    }
    // With pow 0, only the zero polynomial is reduced.
    test::<u8>("0", "0", 0, "0");
    // Multiplying by zero, either way round.
    test::<u8>("x+1", "0", 3, "0");
    test::<u8>("0", "x+1", 3, "0");
    // Multiplying by one.
    test::<u8>("1", "x^2+x+1", 3, "x^2+x+1");
    // The coefficients wrap around modulo 16.
    test::<u8>("x^2+3*x+2", "2*x+5", 4, "2*x^3+11*x^2+3*x+10");
    // The leading coefficient vanishes modulo 16, so the degree drops.
    test::<u8>("8*x+1", "2*x+1", 4, "10*x+1");
    // (2x+1)^2 = 4x^2+4x+1, which is 1 modulo 4.
    test::<u8>("2*x+1", "2*x+1", 2, "1");
    // The full width of the type: (x - 1)(-x - 1) = 1 - x^2.
    test::<u64>(
        "x+18446744073709551615",
        "18446744073709551615*x+18446744073709551615",
        64,
        "18446744073709551615*x^2+1",
    );
}

#[test]
#[should_panic]
fn mod_power_of_2_mul_val_val_self_fail() {
    // A coefficient of self is not reduced.
    let p = UnsignedPolynomial::<u8>::from_str("8*x+1").unwrap();
    let q = UnsignedPolynomial::<u8>::from_str("x").unwrap();
    let _ = p.mod_power_of_2_mul(q, 3);
}

#[test]
#[should_panic]
fn mod_power_of_2_mul_val_val_other_fail() {
    // A coefficient of other is not reduced.
    let p = UnsignedPolynomial::<u8>::from_str("x").unwrap();
    let q = UnsignedPolynomial::<u8>::from_str("8*x+1").unwrap();
    let _ = p.mod_power_of_2_mul(q, 3);
}

#[test]
#[should_panic]
fn mod_power_of_2_mul_val_val_pow_fail() {
    // pow is greater than the width of the type.
    let p = UnsignedPolynomial::<u8>::from_str("x").unwrap();
    let q = UnsignedPolynomial::<u8>::from_str("x").unwrap();
    let _ = p.mod_power_of_2_mul(q, 9);
}

#[test]
#[should_panic]
fn mod_power_of_2_mul_val_ref_self_fail() {
    // A coefficient of self is not reduced.
    let p = UnsignedPolynomial::<u8>::from_str("8*x+1").unwrap();
    let q = UnsignedPolynomial::<u8>::from_str("x").unwrap();
    let _ = p.mod_power_of_2_mul(&q, 3);
}

#[test]
#[should_panic]
fn mod_power_of_2_mul_val_ref_other_fail() {
    // A coefficient of other is not reduced.
    let p = UnsignedPolynomial::<u8>::from_str("x").unwrap();
    let q = UnsignedPolynomial::<u8>::from_str("8*x+1").unwrap();
    let _ = p.mod_power_of_2_mul(&q, 3);
}

#[test]
#[should_panic]
fn mod_power_of_2_mul_val_ref_pow_fail() {
    // pow is greater than the width of the type.
    let p = UnsignedPolynomial::<u8>::from_str("x").unwrap();
    let q = UnsignedPolynomial::<u8>::from_str("x").unwrap();
    let _ = p.mod_power_of_2_mul(&q, 9);
}

#[test]
#[should_panic]
fn mod_power_of_2_mul_ref_val_self_fail() {
    // A coefficient of self is not reduced.
    let p = UnsignedPolynomial::<u8>::from_str("8*x+1").unwrap();
    let q = UnsignedPolynomial::<u8>::from_str("x").unwrap();
    let _ = (&p).mod_power_of_2_mul(q, 3);
}

#[test]
#[should_panic]
fn mod_power_of_2_mul_ref_val_other_fail() {
    // A coefficient of other is not reduced.
    let p = UnsignedPolynomial::<u8>::from_str("x").unwrap();
    let q = UnsignedPolynomial::<u8>::from_str("8*x+1").unwrap();
    let _ = (&p).mod_power_of_2_mul(q, 3);
}

#[test]
#[should_panic]
fn mod_power_of_2_mul_ref_val_pow_fail() {
    // pow is greater than the width of the type.
    let p = UnsignedPolynomial::<u8>::from_str("x").unwrap();
    let q = UnsignedPolynomial::<u8>::from_str("x").unwrap();
    let _ = (&p).mod_power_of_2_mul(q, 9);
}

#[test]
#[should_panic]
fn mod_power_of_2_mul_ref_ref_self_fail() {
    // A coefficient of self is not reduced.
    let p = UnsignedPolynomial::<u8>::from_str("8*x+1").unwrap();
    let q = UnsignedPolynomial::<u8>::from_str("x").unwrap();
    let _ = (&p).mod_power_of_2_mul(&q, 3);
}

#[test]
#[should_panic]
fn mod_power_of_2_mul_ref_ref_other_fail() {
    // A coefficient of other is not reduced.
    let p = UnsignedPolynomial::<u8>::from_str("x").unwrap();
    let q = UnsignedPolynomial::<u8>::from_str("8*x+1").unwrap();
    let _ = (&p).mod_power_of_2_mul(&q, 3);
}

#[test]
#[should_panic]
fn mod_power_of_2_mul_ref_ref_pow_fail() {
    // pow is greater than the width of the type.
    let p = UnsignedPolynomial::<u8>::from_str("x").unwrap();
    let q = UnsignedPolynomial::<u8>::from_str("x").unwrap();
    let _ = (&p).mod_power_of_2_mul(&q, 9);
}

#[test]
#[should_panic]
fn mod_power_of_2_mul_assign_self_fail() {
    // A coefficient of self is not reduced.
    let mut p = UnsignedPolynomial::<u8>::from_str("8*x+1").unwrap();
    let q = UnsignedPolynomial::<u8>::from_str("x").unwrap();
    p.mod_power_of_2_mul_assign(q, 3);
}

#[test]
#[should_panic]
fn mod_power_of_2_mul_assign_other_fail() {
    // A coefficient of other is not reduced.
    let mut p = UnsignedPolynomial::<u8>::from_str("x").unwrap();
    let q = UnsignedPolynomial::<u8>::from_str("8*x+1").unwrap();
    p.mod_power_of_2_mul_assign(q, 3);
}

#[test]
#[should_panic]
fn mod_power_of_2_mul_assign_pow_fail() {
    // pow is greater than the width of the type.
    let mut p = UnsignedPolynomial::<u8>::from_str("x").unwrap();
    let q = UnsignedPolynomial::<u8>::from_str("x").unwrap();
    p.mod_power_of_2_mul_assign(q, 9);
}

#[test]
#[should_panic]
fn mod_power_of_2_mul_assign_ref_self_fail() {
    // A coefficient of self is not reduced.
    let mut p = UnsignedPolynomial::<u8>::from_str("8*x+1").unwrap();
    let q = UnsignedPolynomial::<u8>::from_str("x").unwrap();
    p.mod_power_of_2_mul_assign(&q, 3);
}

#[test]
#[should_panic]
fn mod_power_of_2_mul_assign_ref_other_fail() {
    // A coefficient of other is not reduced.
    let mut p = UnsignedPolynomial::<u8>::from_str("x").unwrap();
    let q = UnsignedPolynomial::<u8>::from_str("8*x+1").unwrap();
    p.mod_power_of_2_mul_assign(&q, 3);
}

#[test]
#[should_panic]
fn mod_power_of_2_mul_assign_ref_pow_fail() {
    // pow is greater than the width of the type.
    let mut p = UnsignedPolynomial::<u8>::from_str("x").unwrap();
    let q = UnsignedPolynomial::<u8>::from_str("x").unwrap();
    p.mod_power_of_2_mul_assign(&q, 9);
}

fn mod_power_of_2_mul_public_properties_helper<T: PrimitiveUnsigned>() {
    unsigned_polynomial_unsigned_polynomial_unsigned_triple_gen_var_1::<T>().test_properties(
        |(p, q, pow)| {
            let r = (&p).mod_power_of_2_mul(&q, pow);
            assert!(r.is_valid());
            // The forms agree.
            assert_eq!((&p).mod_power_of_2_mul(q.clone(), pow), r);
            assert_eq!(p.clone().mod_power_of_2_mul(&q, pow), r);
            assert_eq!(p.clone().mod_power_of_2_mul(q.clone(), pow), r);
            let mut s = p.clone();
            s.mod_power_of_2_mul_assign(&q, pow);
            assert!(s.is_valid());
            assert_eq!(s, r);
            let mut s = p.clone();
            s.mod_power_of_2_mul_assign(q.clone(), pow);
            assert!(s.is_valid());
            assert_eq!(s, r);

            // The result is reduced, and is the schoolbook product.
            assert!(r.mod_power_of_2_is_reduced(pow));
            assert_eq!(mod_power_of_2_mul_polynomial_naive(&p, &q, pow), r);
            // Multiplication is commutative.
            assert_eq!((&q).mod_power_of_2_mul(&p, pow), r);
            // The product of a polynomial with itself is its square.
            assert_eq!(
                (&p).mod_power_of_2_mul(&p, pow),
                (&p).mod_power_of_2_square(pow)
            );
            // Multiplying by zero gives zero, and multiplying by one changes nothing.
            assert_eq!(
                (&p).mod_power_of_2_mul(&UnsignedPolynomial::ZERO, pow),
                UnsignedPolynomial::ZERO
            );
            if pow != 0 {
                let one = UnsignedPolynomial::from_coefficients_asc(vec![T::ONE]);
                assert_eq!((&p).mod_power_of_2_mul(&one, pow), p);
            }
        },
    );
}

#[test]
fn mod_power_of_2_mul_public_properties() {
    apply_fn_to_unsigneds!(mod_power_of_2_mul_public_properties_helper);
}

// Long polynomials, past the Karatsuba threshold several times over and some unbalanced, against
// the schoolbook reference.
fn mod_power_of_2_mul_long_helper<T: PrimitiveUnsigned>() {
    for &(n, k) in &[(1000, 999), (2047, 1500), (2000, 37)] {
        for pow in [T::WIDTH >> 1, T::WIDTH - 1, T::WIDTH] {
            let xs = mod_power_of_2_generated_coefficients::<T>(n, pow, 5);
            let ys = mod_power_of_2_generated_coefficients::<T>(k, pow, 6);
            let expected = mod_power_of_2_mul_naive(&xs, &ys, pow);
            let mut out = vec![T::ZERO; n + k - 1];
            mod_power_of_2_mul_to_out(&mut out, &xs, &ys, pow);
            assert_eq!(out, expected);
            mod_power_of_2_mul_to_out_classical(&mut out, &xs, &ys, pow);
            assert_eq!(out, expected);
            mod_power_of_2_mul_to_out_karatsuba(&mut out, &xs, &ys, pow);
            assert_eq!(out, expected);
        }
    }
}

#[test]
fn test_mod_power_of_2_mul_long() {
    mod_power_of_2_mul_long_helper::<u8>();
    mod_power_of_2_mul_long_helper::<u64>();
}
