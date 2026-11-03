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
use malachite_base::num::conversion::traits::ExactFrom;
use malachite_base::num::logic::traits::LowMask;
use malachite_base::polynomial::Polynomial;
use malachite_base::polynomial::modular::traits::{
    ModEvaluate, ModEvaluateGeometric, ModEvaluateMany,
};
use malachite_base::test_util::generators::*;
use malachite_base::test_util::unsigned_polynomial::modular::mod_evaluate_geometric::*;
use malachite_base::test_util::unsigned_polynomial::modular::mod_mul::*;
use malachite_base::unsigned_polynomial::UnsignedPolynomial;
use malachite_base::unsigned_polynomial::modular::mod_evaluate_geometric::*;

#[test]
fn test_mod_evaluate_geometric() {
    fn test<T: PrimitiveUnsigned>(s: &str, q: T, k: u64, m: T, out: &[T]) {
        let p = UnsignedPolynomial::<T>::from_str(s).unwrap();
        assert_eq!((&p).mod_evaluate_geometric(q, k, m), out);
        assert_eq!(mod_evaluate_geometric_naive(&p, q, k, m), out);
    }
    // - no points
    test::<u8>("5*x^2+3*x+7", 2, 0, 13, &[]);
    // - modulo 1, where the first power, 1, is 0
    test::<u8>("0", 0, 3, 1, &[0, 0, 0]);
    // - powers by Shoup multiplication
    test::<u8>("5*x^2+3*x+7", 2, 4, 13, &[2, 7, 8, 0]);
    // - powers by Horner multiplication, since the top bit of m is set
    test::<u8>("5*x^2+3*x+7", 2, 5, 251, &[15, 33, 99, 100, 80]);
    // - powers by Shoup multiplication in a u64
    test::<u64>(
        "12*x^11+11*x^10+10*x^9+9*x^8+8*x^7+7*x^6+6*x^5+5*x^4+4*x^3+3*x^2+2*x+1",
        3,
        6,
        1000003,
        &[78, 55777, 85524, 367025, 953372, 980990],
    );
    // - powers by Horner multiplication in a u64
    test::<u64>(
        "3*x^2+2*x+1",
        18446744073709551556,
        6,
        18446744073709551557,
        &[6, 2, 6, 2, 6, 2],
    );
    // - q == 0, so every value after the first is the constant term
    test::<u64>("x+5", 0, 4, 7, &[6, 5, 5, 5]);
    // - powers in a u128
    test::<u128>(
        "4*x^3+3*x^2+2*x+1",
        1267650600228229401496703205376,
        5,
        170141183460469231731687303715884105727,
        &[
            10,
            2535301228790657981686254403585,
            19807059518032015876968415233,
            14855280471424704036277854209,
            576461576938192897,
        ],
    );
}

// Bluestein's trick against evaluation at each power, at lengths and numbers of points on both
// sides of the threshold, for every modulus of the test set and several ratios that are units.
fn mod_evaluate_geometric_algorithms_helper<T: PrimitiveUnsigned>() {
    for &(n, k) in &[(1, 1), (1, 5), (5, 1), (3, 7), (64, 64), (65, 100), (200, 70), (130, 300)] {
        for m in test_moduli::<T>() {
            let coefficients = mod_generated_coefficients::<T>(n, m, 8);
            // The same polynomial with its first coefficients zero, to skip them.
            let mut shifted = coefficients.clone();
            for c in shifted.iter_mut().take(n >> 1) {
                *c = T::ZERO;
            }
            for q in
                [T::ONE % m, T::TWO % m, m - T::ONE, mod_generated_coefficients::<T>(1, m, 9)[0]]
            {
                if q == T::ZERO {
                    continue;
                }
                let Some(q_inverse) = q.mod_inverse(m) else {
                    continue;
                };
                for cs in [&coefficients, &shifted] {
                    let expected = mod_evaluate_geometric_iter(cs, q, k, m);
                    assert_eq!(
                        mod_evaluate_geometric_fast(cs, q, q_inverse, k, m),
                        expected
                    );
                    assert_eq!(
                        mod_evaluate_geometric_fast_truncated(cs, q, q_inverse, k, m),
                        expected
                    );
                    // The public function, which uses Bluestein's trick at some of these sizes,
                    // with moduli whose top bit is set.
                    assert_eq!(
                        (&UnsignedPolynomial::from_coefficients_asc(cs.clone()))
                            .mod_evaluate_geometric(q, u64::exact_from(k), m),
                        expected
                    );
                }
            }
        }
    }
}

// The public function on either side of the boundaries where it switches to Bluestein's trick, with
// `u64` moduli of 20, 40, 62, 63, and 64 bits.
#[test]
fn test_mod_evaluate_geometric_dispatch() {
    for &(bits, n, k) in &[
        // - mod_evaluate_geometric_fast_preferred with bits == width
        (64, 32, 32),
        (64, 31, 32),
        (64, 32, 31),
        (64, 4096, 32),
        // - bits == width - 1
        (63, 256, 256),
        (63, 255, 256),
        // - bits + 8 > width
        (62, 512, 1024),
        (62, 512, 1023),
        // - bits << 4 > width * 5
        (40, 256, 256),
        (40, 255, 4096),
        // - otherwise
        (20, 128, 128),
        (20, 4096, 127),
    ] {
        let m = u64::low_mask(bits) - 58;
        let coefficients = mod_generated_coefficients::<u64>(n, m, 10);
        let q = 3;
        let expected = mod_evaluate_geometric_iter(&coefficients, q, k, m);
        assert_eq!(
            (&UnsignedPolynomial::from_coefficients_asc(coefficients)).mod_evaluate_geometric(
                q,
                u64::exact_from(k),
                m
            ),
            expected
        );
    }
}

#[test]
fn test_mod_evaluate_geometric_algorithms() {
    apply_fn_to_unsigneds!(mod_evaluate_geometric_algorithms_helper);
}

#[test]
#[should_panic]
fn mod_evaluate_geometric_fail_1() {
    // m is 0.
    (&UnsignedPolynomial::<u8>::ZERO).mod_evaluate_geometric(0, 1, 0);
}

#[test]
#[should_panic]
fn mod_evaluate_geometric_fail_2() {
    // A coefficient is not reduced.
    (&UnsignedPolynomial::<u8>::from_str("5*x+1").unwrap()).mod_evaluate_geometric(1, 3, 5);
}

#[test]
#[should_panic]
fn mod_evaluate_geometric_fail_3() {
    // q is not reduced.
    (&UnsignedPolynomial::<u8>::from_str("x+1").unwrap()).mod_evaluate_geometric(5, 3, 5);
}

fn mod_evaluate_geometric_properties_helper<T: PrimitiveUnsigned>() {
    unsigned_polynomial_unsigned_unsigned_unsigned_quadruple_gen_var_1::<T>().test_properties(
        |(p, q, k, m)| {
            let ys = (&p).mod_evaluate_geometric(q, k, m);
            assert_eq!(u64::exact_from(ys.len()), k);
            assert!(ys.iter().all(|&y| y < m));
            assert_eq!(mod_evaluate_geometric_naive(&p, q, k, m), ys);
            let k_usize = usize::exact_from(k);
            assert_eq!(
                mod_evaluate_geometric_iter(p.coefficients_asc(), q, k_usize, m),
                ys
            );
            if q != T::ZERO
                && let Some(q_inverse) = q.mod_inverse(m)
            {
                assert_eq!(
                    mod_evaluate_geometric_fast(p.coefficients_asc(), q, q_inverse, k_usize, m),
                    ys
                );
                assert_eq!(
                    mod_evaluate_geometric_fast_truncated(
                        p.coefficients_asc(),
                        q,
                        q_inverse,
                        k_usize,
                        m
                    ),
                    ys
                );
            }

            // It is evaluation at the powers of q.
            let powers: Vec<T> = (0..k).map(|j| q.mod_pow(j, m)).collect();
            assert_eq!((&p).mod_evaluate_many(&powers, m), ys);

            // The first value is p(1), and, when q is 0, every later one is p(0).
            if k != 0 {
                assert_eq!(ys[0], (&p).mod_evaluate(T::ONE % m, m));
            }
            if q == T::ZERO && k > 1 {
                assert!(ys[1..].iter().all(|&y| y == p.coefficient(0)));
            }
            // Fewer points give a prefix.
            if k != 0 {
                assert_eq!((&p).mod_evaluate_geometric(q, k - 1, m), ys[..ys.len() - 1]);
            }
        },
    );
}

#[test]
fn mod_evaluate_geometric_properties() {
    apply_fn_to_unsigneds!(mod_evaluate_geometric_properties_helper);
}
