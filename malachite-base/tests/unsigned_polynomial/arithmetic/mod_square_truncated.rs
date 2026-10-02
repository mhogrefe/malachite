// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.
use core::str::FromStr;
use malachite_base::num::arithmetic::traits::{ModIsReduced, ModSquare};
use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base::polynomial::{
    ModMulTruncated, ModSquareTruncated, ModSquareTruncatedAssign, Polynomial,
};
use malachite_base::test_util::generators::*;
use malachite_base::test_util::unsigned_polynomial::arithmetic::mod_mul::*;
use malachite_base::test_util::unsigned_polynomial::arithmetic::mod_square_truncated::*;
use malachite_base::unsigned_polynomial::UnsignedPolynomial;
use malachite_base::unsigned_polynomial::arithmetic::mod_square_truncated::*;

#[test]
fn test_mod_square_truncated_to_out() {
    fn test<T: PrimitiveUnsigned>(xs: &[T], len: usize, m: T, out: &[T]) {
        let mut result = vec![T::ZERO; len];
        mod_square_truncated_to_out(&mut result, xs, m);
        assert_eq!(result, out);
        let mut result = vec![T::ZERO; len];
        mod_square_truncated_to_out_classical(&mut result, xs, m);
        assert_eq!(result, out);
        let mut result = vec![T::ZERO; len];
        mod_square_truncated_to_out_karatsuba(&mut result, xs, m);
        assert_eq!(result, out);
        assert_eq!(mod_square_truncated_naive(xs, len, m), out);
    }
    // (x^2+3x+2)^2 = x^4+6x^3+13x^2+12x+4, truncated and reduced modulo 7.
    test::<u8>(&[2, 3, 1], 1, 7, &[4]);
    test::<u8>(&[2, 3, 1], 3, 7, &[4, 5, 6]);
    // A length past the end of the square pads with zeros.
    test::<u8>(&[2, 3, 1], 7, 7, &[4, 5, 6, 6, 1, 0, 0]);
    // (x - 1)^2 = x^2 - 2x + 1 modulo 2^64 - 1.
    test::<u64>(&[u64::MAX - 1, 1], 2, u64::MAX, &[1, u64::MAX - 2]);
}

fn mod_square_truncated_algorithms_helper<T: PrimitiveUnsigned>() {
    for &(n, len) in &[
        (1, 1),
        (2, 2),
        (3, 4),
        (63, 80),
        (64, 64),
        (65, 129),
        (100, 150),
        (129, 129),
        (129, 300),
        (200, 199),
        (300, 400),
    ] {
        for modulus in test_moduli::<T>() {
            let xs = mod_generated_coefficients::<T>(n, modulus, 4);
            let expected = mod_square_truncated_naive(&xs, len, modulus);
            let mut out = vec![T::ZERO; len];
            mod_square_truncated_to_out(&mut out, &xs, modulus);
            assert_eq!(out, expected);
            mod_square_truncated_to_out_classical(&mut out, &xs, modulus);
            assert_eq!(out, expected);
            mod_square_truncated_to_out_karatsuba(&mut out, &xs, modulus);
            assert_eq!(out, expected);
        }
    }
}

#[test]
fn test_mod_square_truncated_algorithms() {
    apply_fn_to_unsigneds!(mod_square_truncated_algorithms_helper);
}

#[test]
#[should_panic]
fn mod_square_truncated_to_out_fail_1() {
    mod_square_truncated_to_out::<u8>(&mut [], &[1, 2], 7);
}

#[test]
#[should_panic]
fn mod_square_truncated_to_out_fail_2() {
    mod_square_truncated_to_out::<u8>(&mut [0; 2], &[1, 2], 0);
}

fn mod_square_truncated_properties_helper<T: PrimitiveUnsigned>() {
    unsigned_polynomial_pair_unsigned_unsigned_quadruple_gen_var_2::<T>().test_properties(
        |(p, _, len, m)| {
            let xs = p.coefficients_asc();
            let len = usize::try_from(len).unwrap();
            if xs.is_empty() || len == 0 {
                return;
            }
            let expected = mod_square_truncated_naive(xs, len, m);
            let mut out = vec![T::ZERO; len];
            mod_square_truncated_to_out(&mut out, xs, m);
            assert_eq!(out, expected);
            mod_square_truncated_to_out_classical(&mut out, xs, m);
            assert_eq!(out, expected);
            mod_square_truncated_to_out_karatsuba(&mut out, xs, m);
            assert_eq!(out, expected);
            // It is the full square, truncated.
            let mut full = mod_mul_naive(xs, xs, m);
            full.resize(len, T::ZERO);
            assert_eq!(out, full);
        },
    );
}

#[test]
fn mod_square_truncated_properties() {
    apply_fn_to_unsigneds!(mod_square_truncated_properties_helper);
}

#[test]
fn test_mod_square_truncated() {
    fn test<T: PrimitiveUnsigned>(s: &str, len: u64, m: T, out: &str) {
        let p = UnsignedPolynomial::<T>::from_str(s).unwrap();
        let r = (&p).mod_square_truncated(len, m);
        assert!(r.is_valid());
        assert_eq!(r.to_string(), out);
        assert_eq!(p.clone().mod_square_truncated(len, m), r);
        let mut s = p.clone();
        s.mod_square_truncated_assign(len, m);
        assert!(s.is_valid());
        assert_eq!(s, r);
        assert_eq!(mod_square_truncated_polynomial_naive(&p, len, m), r);
    }
    test::<u8>("0", 3, 7, "0");
    test::<u8>("x^2+3*x+2", 0, 7, "0");
    // The square is x^4+6*x^3+13*x^2+12*x+4; its low three coefficients, modulo 7.
    test::<u8>("x^2+3*x+2", 3, 7, "6*x^2+5*x+4");
    // A length past the end of the square keeps all of it.
    test::<u8>("x^2+3*x+2", 10, 7, "x^4+6*x^3+6*x^2+5*x+4");
    // The linear coefficient of the square, 4, vanishes modulo 4.
    test::<u8>("2*x+1", 2, 4, "1");
    // Near the top of the type: (x - 1)^2 = x^2 - 2x + 1 modulo 2^64 - 1.
    test::<u64>(
        "x+18446744073709551614",
        2,
        u64::MAX,
        "18446744073709551613*x+1",
    );
}

#[test]
#[should_panic]
fn mod_square_truncated_val_self_fail() {
    // A coefficient of self is not reduced.
    let p = UnsignedPolynomial::<u8>::from_str("7*x+1").unwrap();
    let _ = p.mod_square_truncated(2, 7);
}

#[test]
#[should_panic]
fn mod_square_truncated_val_zero_fail() {
    // The modulus is 0.
    let p = UnsignedPolynomial::<u8>::from_str("0").unwrap();
    let _ = p.mod_square_truncated(2, 0);
}

#[test]
#[should_panic]
fn mod_square_truncated_ref_self_fail() {
    // A coefficient of self is not reduced.
    let p = UnsignedPolynomial::<u8>::from_str("7*x+1").unwrap();
    let _ = (&p).mod_square_truncated(2, 7);
}

#[test]
#[should_panic]
fn mod_square_truncated_ref_zero_fail() {
    // The modulus is 0.
    let p = UnsignedPolynomial::<u8>::from_str("0").unwrap();
    let _ = (&p).mod_square_truncated(2, 0);
}

#[test]
#[should_panic]
fn mod_square_truncated_assign_self_fail() {
    // A coefficient of self is not reduced.
    let mut p = UnsignedPolynomial::<u8>::from_str("7*x+1").unwrap();
    p.mod_square_truncated_assign(2, 7);
}

#[test]
#[should_panic]
fn mod_square_truncated_assign_zero_fail() {
    // The modulus is 0.
    let mut p = UnsignedPolynomial::<u8>::from_str("0").unwrap();
    p.mod_square_truncated_assign(2, 0);
}

fn mod_square_truncated_public_properties_helper<T: PrimitiveUnsigned>() {
    unsigned_polynomial_pair_unsigned_unsigned_quadruple_gen_var_2::<T>().test_properties(
        |(p, _, len, m)| {
            let r = (&p).mod_square_truncated(len, m);
            assert!(r.is_valid());
            // The forms agree.
            assert_eq!(p.clone().mod_square_truncated(len, m), r);
            let mut s = p.clone();
            s.mod_square_truncated_assign(len, m);
            assert!(s.is_valid());
            assert_eq!(s, r);

            // The result is reduced, is the truncation of the whole square, and is the truncated
            // product of the polynomial with itself.
            assert!(r.mod_is_reduced(&m));
            assert_eq!(mod_square_truncated_polynomial_naive(&p, len, m), r);
            assert_eq!((&p).mod_square(m).truncate(len), r);
            assert_eq!((&p).mod_mul_truncated(&p, len, m), r);
        },
    );
}

#[test]
fn mod_square_truncated_public_properties() {
    apply_fn_to_unsigneds!(mod_square_truncated_public_properties_helper);
}
