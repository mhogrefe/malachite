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
use crate::test_util::natural_polynomial::mod_polynomial::mod_mul_mod_polynomial::*;
use malachite_base::num::basic::traits::Zero;
use malachite_base::num::modular::traits::ModAddAssign;
use malachite_base::polynomial::Polynomial;
use malachite_base::unsigned_polynomial::mod_polynomial::mod_compose_mod_polynomial::*;

// The composition $f(g)$ modulo `h`, with coefficients modulo `m`, by Horner's rule with naive
// products and divisions. `g` must be shorter than `h`.
pub fn mod_compose_mod_polynomial_naive(
    f: &NaturalPolynomial,
    g: &NaturalPolynomial,
    h: &NaturalPolynomial,
    m: &Natural,
) -> NaturalPolynomial {
    if *m == 1u32 || h.len() <= 1 {
        return NaturalPolynomial::ZERO;
    }
    let mut r = NaturalPolynomial::ZERO;
    for c in f.coefficients_asc().iter().rev() {
        r = mod_mul_mod_polynomial_naive(&r, g, h, m);
        let mut xs = r.into_coefficients_asc();
        if xs.is_empty() {
            xs.push(Natural::ZERO);
        }
        xs[0].mod_add_assign(c, m);
        r = NaturalPolynomial::from_coefficients_asc(xs);
    }
    r
}

fn run(
    f: &NaturalPolynomial,
    g: &NaturalPolynomial,
    h: &NaturalPolynomial,
    m: &Natural,
    kernel: impl Fn(&NaturalModRing, &[Natural], &[Natural], usize) -> Vec<Natural>,
) -> NaturalPolynomial {
    if assert_compose_mod_polynomial_arguments(f, g, h, m) {
        return NaturalPolynomial::ZERO;
    }
    let n = h.coefficients_asc().len() - 1;
    let mut gs = g.coefficients_asc().to_vec();
    gs.resize(n, Natural::ZERO);
    let data = precompute_helper(h, m);
    let ring = NaturalModRing { m, h, data: &data };
    NaturalPolynomial::from_coefficients_asc(kernel(&ring, f.coefficients_asc(), &gs, n))
}

pub fn mod_compose_mod_polynomial_horner(
    f: &NaturalPolynomial,
    g: &NaturalPolynomial,
    h: &NaturalPolynomial,
    m: &Natural,
) -> NaturalPolynomial {
    run(f, g, h, m, |r, f, g, n| compose_mod_horner(r, f, g, n))
}

pub fn mod_compose_mod_polynomial_brent_kung(
    f: &NaturalPolynomial,
    g: &NaturalPolynomial,
    h: &NaturalPolynomial,
    m: &Natural,
) -> NaturalPolynomial {
    run(f, g, h, m, |r, f, g, n| compose_mod_brent_kung(r, f, g, n))
}
