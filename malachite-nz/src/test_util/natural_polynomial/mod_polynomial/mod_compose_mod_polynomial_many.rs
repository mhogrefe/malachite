// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::natural::Natural;
use crate::natural_polynomial::NaturalPolynomial;
use crate::natural_polynomial::mod_polynomial::mod_compose_mod_polynomial::*;
use crate::natural_polynomial::modular::mod_div_rem_precomputed::precompute_helper;
use crate::test_util::natural_polynomial::mod_polynomial::mod_compose_mod_polynomial::*;
use alloc::vec::Vec;
use malachite_base::num::basic::traits::Zero;
use malachite_base::polynomial::Polynomial;
use malachite_base::polynomial::mod_polynomial::traits::ModComposeModPolynomial;
use malachite_base::unsigned_polynomial::mod_polynomial::mod_compose_mod_polynomial::*;

// The compositions $f_i(g)$ modulo `h`, with coefficients modulo `m`, composing each $f_i$ on its
// own.
pub fn mod_compose_mod_polynomial_many_one_at_a_time(
    fs: &[NaturalPolynomial],
    g: &NaturalPolynomial,
    h: &NaturalPolynomial,
    m: &Natural,
) -> Vec<NaturalPolynomial> {
    fs.iter()
        .map(|f| f.mod_compose_mod_polynomial(g, h, m))
        .collect()
}

// The compositions $f_i(g)$ modulo `h`, with coefficients modulo `m`, each by Horner's rule with
// naive products and divisions.
pub fn mod_compose_mod_polynomial_many_naive(
    fs: &[NaturalPolynomial],
    g: &NaturalPolynomial,
    h: &NaturalPolynomial,
    m: &Natural,
) -> Vec<NaturalPolynomial> {
    fs.iter()
        .map(|f| mod_compose_mod_polynomial_naive(f, g, h, m))
        .collect()
}

// The compositions $f_i(g)$ modulo `h`, with coefficients modulo $m$, by the Brent–Kung algorithm
// with a single matrix product, whatever the number and lengths of the $f_i$.
pub fn mod_compose_mod_polynomial_many_brent_kung(
    fs: &[NaturalPolynomial],
    g: &NaturalPolynomial,
    h: &NaturalPolynomial,
    m: &Natural,
) -> Vec<NaturalPolynomial> {
    if assert_compose_mod_polynomial_inner_and_modulus_arguments(g, h, m) {
        return vec![NaturalPolynomial::ZERO; fs.len()];
    }
    let n = h.coefficients_asc().len() - 1;
    let mut gs = g.coefficients_asc().to_vec();
    gs.resize(n, Natural::ZERO);
    let data = precompute_helper(h, m);
    let ring = NaturalModRing { m, h, data: &data };
    let fs: Vec<&[Natural]> = fs.iter().map(NaturalPolynomial::coefficients_asc).collect();
    compose_mod_brent_kung_vec(&ring, &fs, &gs, n)
        .into_iter()
        .map(NaturalPolynomial::from_coefficients_asc)
        .collect()
}
