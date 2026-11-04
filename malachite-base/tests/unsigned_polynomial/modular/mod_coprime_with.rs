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
use malachite_base::num::factorization::traits::{Factor, IsPrime};
use malachite_base::num::modular::traits::ModMul;
use malachite_base::polynomial::Polynomial;
use malachite_base::polynomial::modular::traits::{CheckedModGcd, ModCoprimeWith, ModGcd};
use malachite_base::test_util::generators::{
    unsigned_polynomial_unsigned_polynomial_unsigned_triple_gen_var_2,
    unsigned_polynomial_unsigned_polynomial_unsigned_triple_gen_var_10,
    unsigned_polynomial_unsigned_unsigned_triple_gen_var_2,
};
use malachite_base::test_util::unsigned_polynomial::modular::mod_coprime_with::*;
use malachite_base::test_util::unsigned_polynomial::modular::mod_mul::mod_generated_coefficients;
use malachite_base::unsigned_polynomial::UnsignedPolynomial;

// Checks the coprimality of `p` and `q` modulo `m` in every way of calling it, symmetry, and
// agreement with the checked GCD where that succeeds; and returns it.
fn verify<T: PrimitiveUnsigned>(
    p: &UnsignedPolynomial<T>,
    q: &UnsignedPolynomial<T>,
    m: T,
) -> bool {
    let coprime = p.mod_coprime_with(q, m);
    assert_eq!(p.clone().mod_coprime_with(q.clone(), m), coprime);
    assert_eq!(p.clone().mod_coprime_with(q, m), coprime);
    assert_eq!(p.mod_coprime_with(q.clone(), m), coprime);
    assert_eq!(q.mod_coprime_with(p, m), coprime);
    if m != T::ONE
        && let Some(g) = p.checked_mod_gcd(q, m)
    {
        assert_eq!(g == UnsignedPolynomial::one(), coprime);
    }
    coprime
}

fn test_mod_coprime_with_helper<T: PrimitiveUnsigned + Factor>()
where
    T::FACTORS: IntoIterator<Item = (T, u8)>,
{
    let test = |p, q, m: u8, out| {
        let p = UnsignedPolynomial::<T>::from_str(p).unwrap();
        let q = UnsignedPolynomial::<T>::from_str(q).unwrap();
        let m = T::from(m);
        assert_eq!(verify(&p, &q, m), out);
        assert_eq!(mod_coprime_with_naive(&p, &q, m), out);
    };
    // - m == T::ONE
    test("0", "0", 1, true);
    // - both 0
    // - ys.is_empty() && xs.is_empty()
    // - !(xs.len() < ys.len())
    // - coprime_or_non_invertible(..).is_ok()
    test("0", "0", 7, false);
    // - an invertible constant and 0
    // - ys.is_empty() && m.inverse(x).is_some()
    // - xs.len() < ys.len()
    test("3", "0", 7, true);
    // - a nonconstant polynomial and 0
    test("x+1", "0", 7, false);
    // - an invertible constant
    // - ys.len() == 1
    test("x^2+1", "3", 7, true);
    // - coprime modulo a prime
    // - ys.len() > 1
    test("x^2+1", "x+1", 7, true);
    // - a common factor modulo a prime
    test("x^2+3*x+2", "x^2+4*x+3", 7, false);
    // - a non-invertible constant and 0
    // - ys.is_empty() && m.inverse(x).is_none()
    // - coprime_or_non_invertible(..).is_err()
    // - not coprime modulo the first factor of m
    // - ys.last() == Some(&T::ZERO)
    test("2", "0", 6, false);
    // - a unit that is not a constant: 1 + 2x, whose square is 1 modulo 4
    test("2*x+1", "0", 4, true);
    // - a non-invertible leading coefficient, splitting 6 into 2 and 3
    // - m.inverse(y).is_none()
    test("x^2+1", "2*x+1", 6, true);
    // - coprime modulo 3 but not modulo 2
    test("x^2+1", "x+3", 6, false);
    // - coprime modulo 2 but not modulo 3
    test("x^2+x+1", "x+2", 6, false);
    // - nilpotent leading coefficients modulo 4
    test("2*x+1", "2*x+3", 4, true);
    test("x+1", "2*x+2", 4, false);
    // - nilpotent leading coefficients modulo 9
    test("3*x+1", "3*x+2", 9, true);
    // - a modulus with three prime factors
    test("5*x^2+x+3", "6*x+5", 30, true);
    test("x^2+1", "x+3", 30, false);
}

#[test]
fn test_mod_coprime_with() {
    test_mod_coprime_with_helper::<u8>();
    test_mod_coprime_with_helper::<u16>();
    test_mod_coprime_with_helper::<u32>();
    test_mod_coprime_with_helper::<u64>();
    test_mod_coprime_with_helper::<usize>();
}

// Polynomials long enough for the half-GCD algorithm, modulo 999983 * 1000003, against the naive
// version.
#[test]
fn test_mod_coprime_with_long() {
    let m: u64 = 999983 * 1000003;
    let generated = |len, seed| {
        UnsignedPolynomial::from_coefficients_asc(mod_generated_coefficients(len, m, seed))
    };
    let p = generated(2000, 1);
    let q = generated(1800, 2);
    // - the half-GCD algorithm succeeds
    // - ys.len() >= m.gcd_cutoffs().gcd_hgcd
    assert!(verify(&p, &q, m));
    assert!(mod_coprime_with_naive(&p, &q, m));
    // - a common factor
    let c = generated(300, 3);
    let pc = (&p).mod_mul(&c, m);
    let qc = (&q).mod_mul(&c, m);
    assert!(!verify(&pc, &qc, m));
    assert!(!mod_coprime_with_naive(&pc, &qc, m));
    // - the half-GCD algorithm meets a non-invertible leading coefficient, which splits the modulus
    // - recording.non_invertible.is_some()
    let mut coefficients = q.into_coefficients_asc();
    *coefficients.last_mut().unwrap() = 999983;
    let q = UnsignedPolynomial::from_coefficients_asc(coefficients);
    let coprime = verify(&p, &q, m);
    assert_eq!(mod_coprime_with_naive(&p, &q, m), coprime);
}

#[test]
#[should_panic]
fn mod_coprime_with_fail_1() {
    UnsignedPolynomial::<u8>::from_str("x+7")
        .unwrap()
        .mod_coprime_with(UnsignedPolynomial::from_str("x").unwrap(), 7);
}

#[test]
#[should_panic]
fn mod_coprime_with_fail_2() {
    UnsignedPolynomial::<u8>::from_str("x")
        .unwrap()
        .mod_coprime_with(UnsignedPolynomial::from_str("x").unwrap(), 0);
}

fn mod_coprime_with_properties_helper<T: PrimitiveUnsigned>() {
    unsigned_polynomial_unsigned_polynomial_unsigned_triple_gen_var_2::<T>().test_properties(
        |(p, q, m)| {
            verify(&p, &q, m);
        },
    );

    unsigned_polynomial_unsigned_unsigned_triple_gen_var_2::<T>().test_properties(|(p, _, m)| {
        assert!((&p).mod_coprime_with(UnsignedPolynomial::ZERO, m) == (&p).mod_coprime_with(&p, m));
        if m != T::ONE {
            assert!((&p).mod_coprime_with(UnsignedPolynomial::one(), m));
        }
    });
}

fn mod_coprime_with_naive_properties_helper<T: PrimitiveUnsigned + Factor>()
where
    T::FACTORS: IntoIterator<Item = (T, u8)>,
{
    unsigned_polynomial_unsigned_polynomial_unsigned_triple_gen_var_2::<T>().test_properties(
        |(p, q, m)| {
            assert_eq!(verify(&p, &q, m), mod_coprime_with_naive(&p, &q, m));
        },
    );
}

fn mod_coprime_with_prime_properties_helper<T: PrimitiveUnsigned + IsPrime>() {
    unsigned_polynomial_unsigned_polynomial_unsigned_triple_gen_var_10::<T>().test_properties(
        |(p, q, m)| {
            assert_eq!(
                verify(&p, &q, m),
                p.mod_gcd(q, m) == UnsignedPolynomial::one()
            );
        },
    );
}

#[test]
fn mod_coprime_with_properties() {
    apply_fn_to_unsigneds!(mod_coprime_with_properties_helper);
    mod_coprime_with_naive_properties_helper::<u8>();
    mod_coprime_with_naive_properties_helper::<u16>();
    mod_coprime_with_naive_properties_helper::<u32>();
    mod_coprime_with_naive_properties_helper::<u64>();
    mod_coprime_with_naive_properties_helper::<usize>();
    mod_coprime_with_prime_properties_helper::<u8>();
    mod_coprime_with_prime_properties_helper::<u16>();
    mod_coprime_with_prime_properties_helper::<u32>();
    mod_coprime_with_prime_properties_helper::<u64>();
    mod_coprime_with_prime_properties_helper::<usize>();
}
