// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::num::basic::traits::Zero;
use crate::num::basic::unsigneds::PrimitiveUnsigned;
use crate::polynomial::Polynomial;
use crate::polynomial::mod_polynomial::traits::ModComposeModPolynomial;
use crate::test_util::unsigned_polynomial::mod_polynomial::mod_compose_mod_polynomial::*;
use crate::unsigned_polynomial::UnsignedPolynomial;
use crate::unsigned_polynomial::mod_polynomial::mod_compose_mod_polynomial::*;
use crate::unsigned_polynomial::modular::mod_div_rem_precomputed::precompute_helper;
use alloc::vec::Vec;

// The compositions $f_i(g)$ modulo `h`, with coefficients modulo `m`, composing each $f_i$ on its
// own.
pub fn mod_compose_mod_polynomial_many_one_at_a_time<T: PrimitiveUnsigned>(
    fs: &[UnsignedPolynomial<T>],
    g: &UnsignedPolynomial<T>,
    h: &UnsignedPolynomial<T>,
    m: T,
) -> Vec<UnsignedPolynomial<T>> {
    fs.iter()
        .map(|f| f.mod_compose_mod_polynomial(g, h, m))
        .collect()
}

// The compositions $f_i(g)$ modulo `h`, with coefficients modulo `m`, each by Horner's rule with
// naive products and divisions.
pub fn mod_compose_mod_polynomial_many_naive<T: PrimitiveUnsigned>(
    fs: &[UnsignedPolynomial<T>],
    g: &UnsignedPolynomial<T>,
    h: &UnsignedPolynomial<T>,
    m: T,
) -> Vec<UnsignedPolynomial<T>> {
    fs.iter()
        .map(|f| mod_compose_mod_polynomial_naive(f, g, h, m))
        .collect()
}

// The compositions $f_i(g)$ modulo `h`, with coefficients modulo $m$, by the Brent–Kung algorithm
// with a single matrix product, whatever the number and lengths of the $f_i$.
pub fn mod_compose_mod_polynomial_many_brent_kung<T: PrimitiveUnsigned>(
    fs: &[UnsignedPolynomial<T>],
    g: &UnsignedPolynomial<T>,
    h: &UnsignedPolynomial<T>,
    m: T,
) -> Vec<UnsignedPolynomial<T>> {
    if assert_compose_mod_polynomial_inner_and_modulus_arguments(g, h, m) {
        return vec![UnsignedPolynomial::ZERO; fs.len()];
    }
    let n = h.coefficients_asc().len() - 1;
    let mut gs = g.coefficients_asc().to_vec();
    gs.resize(n, T::ZERO);
    let data = precompute_helper(h, m);
    let ring = ModRing {
        m,
        h: h.coefficients_asc(),
        inverse: &data.inverse,
    };
    let fs: Vec<&[T]> = fs
        .iter()
        .map(UnsignedPolynomial::coefficients_asc)
        .collect();
    compose_mod_brent_kung_vec(&ring, &fs, &gs, n)
        .into_iter()
        .map(UnsignedPolynomial::from_coefficients_asc)
        .collect()
}
