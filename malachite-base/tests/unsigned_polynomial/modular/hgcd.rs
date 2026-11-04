// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use core::str::FromStr;
use malachite_base::num::modular::traits::{ModAdd, ModMul, ModPowerOf2Add, ModPowerOf2Mul};
use malachite_base::polynomial::Polynomial;
use malachite_base::polynomial::modular::traits::ModMakeMonic;
use malachite_base::test_util::unsigned_polynomial::modular::mod_gcd::mod_gcd_naive;
use malachite_base::test_util::unsigned_polynomial::modular::mod_mul::mod_generated_coefficients;
use malachite_base::test_util::unsigned_polynomial::modular::mod_power_of_2_gcd::*;
use malachite_base::unsigned_polynomial::UnsignedPolynomial;
use malachite_base::unsigned_polynomial::modular::hgcd::{gcd_hgcd, xgcd_hgcd};
use malachite_base::unsigned_polynomial::modular::subproduct_tree::{Modulus, PowerOf2Modulus};

fn to_string(xs: &[u64]) -> String {
    UnsignedPolynomial::from_coefficients_asc(xs.to_vec()).to_string()
}

// The half-GCD GCD and extended GCD, with small cutoffs so that short polynomials reach every path,
// against Euclid's algorithm with schoolbook division.
#[test]
fn test_gcd_hgcd() {
    let test_m = |p, q, m: u64, inner, outer, out: &str, out_x: Option<(&str, &str, &str)>| {
        let p = UnsignedPolynomial::<u64>::from_str(p).unwrap();
        let q = UnsignedPolynomial::<u64>::from_str(q).unwrap();
        let (xs, ys) = (p.coefficients_asc(), q.coefficients_asc());
        let modulus = Modulus(m);
        let g = gcd_hgcd(xs, ys, inner, outer, &modulus).unwrap();
        assert_eq!(to_string(&g), out);
        assert_eq!(
            UnsignedPolynomial::from_coefficients_asc(g)
                .mod_make_monic(m)
                .unwrap(),
            mod_gcd_naive(&p, &q, m).unwrap()
        );
        let x = if ys.len() >= 2 {
            xgcd_hgcd(xs, ys, inner, outer, &modulus)
        } else {
            None
        };
        if let Some((g, s, t)) = &x {
            let (g, s, t) = (
                UnsignedPolynomial::from_coefficients_asc(g.clone()),
                UnsignedPolynomial::from_coefficients_asc(s.clone()),
                UnsignedPolynomial::from_coefficients_asc(t.clone()),
            );
            assert_eq!(s.mod_mul(&p, m).mod_add(t.mod_mul(&q, m), m), g);
        }
        assert_eq!(
            x.map(|(g, s, t)| (to_string(&g), to_string(&s), to_string(&t))),
            out_x.map(|(g, s, t)| (g.to_string(), s.to_string(), t.to_string()))
        );
    };
    let test_pow = |p, q, pow: u64, inner, outer, out: &str, out_x: Option<(&str, &str, &str)>| {
        let p = UnsignedPolynomial::<u64>::from_str(p).unwrap();
        let q = UnsignedPolynomial::<u64>::from_str(q).unwrap();
        let (xs, ys) = (p.coefficients_asc(), q.coefficients_asc());
        let modulus = PowerOf2Modulus(pow);
        let g = gcd_hgcd(xs, ys, inner, outer, &modulus).unwrap();
        assert_eq!(to_string(&g), out);
        assert_eq!(
            Some(
                UnsignedPolynomial::from_coefficients_asc(g)
                    .mod_make_monic(1u64 << pow)
                    .unwrap()
            ),
            mod_power_of_2_gcd_naive(&p, &q, pow)
        );
        let x = xgcd_hgcd(xs, ys, inner, outer, &modulus);
        if let Some((g, s, t)) = &x {
            let (g, s, t) = (
                UnsignedPolynomial::from_coefficients_asc(g.clone()),
                UnsignedPolynomial::from_coefficients_asc(s.clone()),
                UnsignedPolynomial::from_coefficients_asc(t.clone()),
            );
            assert_eq!(
                s.mod_power_of_2_mul(&p, pow)
                    .mod_power_of_2_add(t.mod_power_of_2_mul(&q, pow), pow),
                g
            );
        }
        assert_eq!(
            x.map(|(g, s, t)| (to_string(&g), to_string(&s), to_string(&t))),
            out_x.map(|(g, s, t)| (g.to_string(), s.to_string(), t.to_string()))
        );
    };
    // - r.is_empty(), the first
    test_m("1", "1", 2, 1, 1, "1", None);
    // - r.is_empty(), the third
    test_m("x^2", "x^2", 2, 1, 1, "x^2", Some(("x^2", "0", "1")));
    // - ys.len() > out.len(), the first
    // - b.len() <= half
    // - flag, the first
    // - !(flag), the first
    // - !j.is_empty()
    // - r.is_empty(), the second
    // - step.sign > 0, the first
    // - r.is_empty(), the fourth
    test_m("x^2", "x+1", 2, 1, 1, "1", Some(("1", "1", "x+1")));
    // - xs.len() >= ys.len(), the first
    // - !(xs.len() >= ys.len()), the first
    // - xs.is_empty() || ys.is_empty()
    // - xs.len() >= k
    // - !(xs.len() >= k)
    // - xs.len() < k + ys.len()
    // - shortest < 20
    // - !(a0.len() < cutoff)
    // - !(first.sign < 0), the first
    // - !(first.sign < 0), the second
    // - !(c0.len() < cutoff)
    // - !(second.sign < 0), the first
    // - !(second.sign < 0), the second
    // - flag, the third
    // - !(flag), the third
    // - !(step.sign > 0), the first
    // - j.is_empty()
    test_m("x^2", "x^2+x", 2, 1, 1, "x", Some(("x", "1", "1")));
    // - c0.len() < cutoff
    test_m("x^2", "x^2+x", 2, 2, 3, "x", Some(("x", "1", "1")));
    // - a0.len() < cutoff
    test_m("x^2", "x^2+x", 2, 4, 8, "x", Some(("x", "1", "1")));
    // - ys.len() == 1, the first
    test_m(
        "2*x^4+5*x^3+6",
        "2*x^3+3*x^2+5*x+4",
        7,
        2,
        3,
        "1",
        Some(("1", "4*x^2+4*x+5", "3*x^3+6*x^2+5")),
    );
    // - !(flag), the second
    test_m(
        "71*x^5+35*x^4+82*x^3+8*x^2+x+12",
        "33*x^4+95*x^3+58*x^2+96*x+11",
        101,
        1,
        1,
        "80*x^3+3*x^2+97*x+45",
        Some(("80*x^3+3*x^2+97*x+45", "1", "56*x+3")),
    );
    // - ys.len() > out.len(), the second
    test_pow("x^2", "x+1", 1, 1, 1, "1", Some(("1", "1", "x+1")));
    // - !(xs.len() >= ys.len()), the second
    test_pow("x^2", "x^2+x", 1, 1, 1, "x", Some(("x", "1", "1")));
}

// Polynomials modulo 7 of lengths `p_len` and `q_len`, sharing a factor of half the second length
// if `shared` and otherwise coprime with high probability.
pub(crate) fn generated_pair(
    p_len: usize,
    q_len: usize,
    shared: bool,
) -> (UnsignedPolynomial<u64>, UnsignedPolynomial<u64>) {
    let m = 7;
    if shared {
        let c =
            UnsignedPolynomial::from_coefficients_asc(mod_generated_coefficients(q_len >> 1, m, 1));
        (
            UnsignedPolynomial::from_coefficients_asc(mod_generated_coefficients(
                p_len - (q_len >> 1),
                m,
                2,
            ))
            .mod_mul(&c, m),
            UnsignedPolynomial::from_coefficients_asc(mod_generated_coefficients(
                q_len - (q_len >> 1),
                m,
                3,
            ))
            .mod_mul(&c, m),
        )
    } else {
        (
            UnsignedPolynomial::from_coefficients_asc(mod_generated_coefficients(p_len, m, 4)),
            UnsignedPolynomial::from_coefficients_asc(mod_generated_coefficients(q_len, m, 5)),
        )
    }
}

// The half-GCD GCD and extended GCD of longer polynomials, which reach the recursive steps and
// Strassen's matrix product, against Euclid's algorithm with schoolbook division.
#[test]
fn test_gcd_hgcd_generated() {
    let test = |p_len, q_len, shared, inner, outer| {
        let (p, q) = generated_pair(p_len, q_len, shared);
        let (xs, ys) = (p.coefficients_asc(), q.coefficients_asc());
        let modulus = Modulus(7);
        let g = UnsignedPolynomial::from_coefficients_asc(
            gcd_hgcd(xs, ys, inner, outer, &modulus).unwrap(),
        );
        let monic = mod_gcd_naive(&p, &q, 7).unwrap();
        assert_eq!(g.mod_make_monic(7).unwrap(), monic);
        let (g, s, t) = xgcd_hgcd(xs, ys, inner, outer, &modulus).unwrap();
        let (g, s, t) = (
            UnsignedPolynomial::from_coefficients_asc(g),
            UnsignedPolynomial::from_coefficients_asc(s),
            UnsignedPolynomial::from_coefficients_asc(t),
        );
        assert_eq!(s.mod_mul(&p, 7).mod_add(t.mod_mul(&q, 7), 7), g);
        assert_eq!(g.mod_make_monic(7).unwrap(), monic);
    };
    // - first.sign < 0, the first
    // - first.sign < 0, the second
    // - b2.len() <= half
    // - flag, the second
    // - second.sign < 0, the first
    // - second.sign < 0, the second
    test(60, 50, true, 1, 1);
    // - b.len() > half
    test(60, 50, true, 4, 8);
    // - step.sign > 0, the second
    // - !(step.sign > 0), the second
    // - step.sign > 0, the third
    // - !(step.sign > 0), the third
    test(60, 50, false, 1, 1);
    // - j.len() < cutoff, the first
    // - j.len() < cutoff, the second
    test(60, 50, false, 4, 8);
    // - !(shortest < 20)
    test(120, 100, true, 1, 1);
}
