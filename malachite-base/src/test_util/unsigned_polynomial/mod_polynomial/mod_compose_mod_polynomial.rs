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
use crate::test_util::unsigned_polynomial::mod_polynomial::mod_mul_mod_polynomial::*;
use crate::unsigned_polynomial::UnsignedPolynomial;
use crate::unsigned_polynomial::mod_polynomial::mod_compose_mod_polynomial::*;
use crate::unsigned_polynomial::modular::mod_div_rem_precomputed::precompute_helper;

// The composition $f(g)$ modulo `h`, with coefficients modulo `m`, by Horner's rule with naive
// products and divisions. `g` must be shorter than `h`.
pub fn mod_compose_mod_polynomial_naive<T: PrimitiveUnsigned>(
    f: &UnsignedPolynomial<T>,
    g: &UnsignedPolynomial<T>,
    h: &UnsignedPolynomial<T>,
    m: T,
) -> UnsignedPolynomial<T> {
    if m == T::ONE || h.len() <= 1 {
        return UnsignedPolynomial::ZERO;
    }
    let mut r = UnsignedPolynomial::ZERO;
    for &c in f.coefficients_asc().iter().rev() {
        r = mod_mul_mod_polynomial_naive(&r, g, h, m);
        let mut xs = r.into_coefficients_asc();
        if xs.is_empty() {
            xs.push(T::ZERO);
        }
        xs[0].mod_add_assign(c, m);
        r = UnsignedPolynomial::from_coefficients_asc(xs);
    }
    r
}

fn run<T: PrimitiveUnsigned>(
    f: &UnsignedPolynomial<T>,
    g: &UnsignedPolynomial<T>,
    h: &UnsignedPolynomial<T>,
    m: T,
    kernel: impl Fn(&ModRing<T>, &[T], &[T], usize) -> Vec<T>,
) -> UnsignedPolynomial<T> {
    if assert_compose_mod_polynomial_arguments(f, g, h, m) {
        return UnsignedPolynomial::ZERO;
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
    UnsignedPolynomial::from_coefficients_asc(kernel(&ring, f.coefficients_asc(), &gs, n))
}

pub fn mod_compose_mod_polynomial_horner<T: PrimitiveUnsigned>(
    f: &UnsignedPolynomial<T>,
    g: &UnsignedPolynomial<T>,
    h: &UnsignedPolynomial<T>,
    m: T,
) -> UnsignedPolynomial<T> {
    run(f, g, h, m, |r, f, g, n| compose_mod_horner(r, f, g, n))
}

pub fn mod_compose_mod_polynomial_brent_kung<T: PrimitiveUnsigned>(
    f: &UnsignedPolynomial<T>,
    g: &UnsignedPolynomial<T>,
    h: &UnsignedPolynomial<T>,
    m: T,
) -> UnsignedPolynomial<T> {
    run(f, g, h, m, |r, f, g, n| compose_mod_brent_kung(r, f, g, n))
}
