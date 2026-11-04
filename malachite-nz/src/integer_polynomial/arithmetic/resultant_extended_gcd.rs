// Copyright © 2026 Mikhail Hogrefe
//
// Uses code adopted from the FLINT Library.
//
//      Copyright © 2011 William Hart
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::integer::Integer;
use crate::integer_matrix::linear_algebra::determinant::BalancedCrtWord;
use crate::integer_polynomial::IntegerPolynomial;
use crate::integer_polynomial::arithmetic::gcd::{
    LargePrimes, lift_balanced, max_bits, reduce, trim,
};
use crate::integer_polynomial::arithmetic::resultant::resultant_nonzero;
use crate::natural::Natural;
use crate::platform::Limb;
use alloc::vec;
use alloc::vec::Vec;
use core::cmp::max;
use malachite_base::num::basic::traits::{One, Zero};
use malachite_base::num::logic::traits::SignificantBits;
use malachite_base::num::modular::traits::{ModAdd, ModMul};
use malachite_base::unsigned_polynomial::modular::mod_extended_gcd::checked_monic_extended_gcd;
use malachite_base::unsigned_polynomial::modular::mod_mul::mod_mul_to_out;
use malachite_base::unsigned_polynomial::modular::subproduct_tree::Modulus;

// `xs` reduced modulo `p` and padded with zeros to `len` coefficients.
fn reduce_padded(xs: &[Integer], len: usize, p: Limb) -> Vec<Limb> {
    let mut out: Vec<Limb> = xs.iter().map(|x| reduce(x, p)).collect();
    out.resize(len, 0);
    out
}

// The resultant $r$ of the polynomials $f$ and $g$ with coefficients `xs` and `ys`, where `xs` is
// at least as long as `ys`, neither is empty or ends with a zero, and both are primitive, together
// with $s$ and $t$ such that $sf + tg = r$, $\deg s < \deg g$, and $\deg t < \deg f$, found from
// their values modulo primes. If $r$ is 0, so are $s$ and $t$. (For polynomials that are not
// primitive, an integral $s$ and $t$ need not exist, and the CRT never stabilises.)
//
// This is equivalent to `_fmpz_poly_xgcd_modular` from `fmpz_poly/xgcd_modular.c`, FLINT 3.6.0,
// except that `s` and `t` are 0 when the resultant is.
crate_test_fn! {resultant_extended_gcd_nonzero(
    xs: &[Integer],
    ys: &[Integer],
) -> (Integer, Vec<Integer>, Vec<Integer>) {
    let (len1, len2) = (xs.len(), ys.len());
    let r = resultant_nonzero(xs, ys);
    if r == 0u32 {
        return (r, Vec::new(), Vec::new());
    }
    let mut product = Natural::ONE;
    // The first prime sets `s` and `t` before anything reads them.
    let mut s = Vec::new();
    let mut t = Vec::new();
    let mut primes = LargePrimes::new();
    let mut stabilised = false;
    let mut first = true;
    let mut s_bits = 0;
    let mut t_bits = 0;
    let lead1 = xs.last().unwrap();
    let lead2 = ys.last().unwrap();
    loop {
        let p = primes.next().unwrap();
        let r_mod = reduce(&r, p);
        // If p divides the resultant or either leading coefficient, discard p.
        if reduce(lead1, p) == 0 || reduce(lead2, p) == 0 || r_mod == 0 {
            continue;
        }
        let a = reduce_padded(xs, len1, p);
        let b = reduce_padded(ys, len2, p);
        if stabilised {
            // Multiply out A * S + B * T to see if it is R modulo p.
            let s_mod = reduce_padded(&s, len2, p);
            let t_mod = reduce_padded(&t, len1, p);
            let product_len = len1 + len2 - 1;
            let mut products = vec![0; product_len << 1];
            let (t1, t2) = products.split_at_mut(product_len);
            mod_mul_to_out(t1, &a, &s_mod, p);
            mod_mul_to_out(t2, &t_mod, &b, p);
            for (x, &y) in t1.iter_mut().zip(t2.iter()) {
                *x = x.mod_add(y, p);
            }
            let mut t1 = t1.to_vec();
            while t1.last() == Some(&0) {
                t1.pop();
            }
            if t1.len() == 1 && t1[0] == r_mod {
                // It is, so this prime is good.
                product *= Natural::from(p);
            } else {
                stabilised = false;
            }
        }
        if !stabilised {
            // The GCD modulo p is 1, since p does not divide the resultant.
            let (_, s_mod, t_mod) = checked_monic_extended_gcd(&a, &b, &Modulus(p)).unwrap();
            let mut s_mod: Vec<Limb> = s_mod.iter().map(|&c| c.mod_mul(r_mod, p)).collect();
            let mut t_mod: Vec<Limb> = t_mod.iter().map(|&c| c.mod_mul(r_mod, p)).collect();
            s_mod.resize(len2, 0);
            t_mod.resize(len1, 0);
            if first {
                s = s_mod.iter().map(|&c| lift_balanced(c, p)).collect();
                t = t_mod.iter().map(|&c| lift_balanced(c, p)).collect();
                product = Natural::from(p);
                // Optimize the case where one prime is enough.
                stabilised = true;
                first = false;
            } else {
                let crt = BalancedCrtWord::new(&product, p);
                for (x, &c) in s.iter_mut().zip(&s_mod) {
                    *x = crt.combine(x, c);
                }
                for (x, &c) in t.iter_mut().zip(&t_mod) {
                    *x = crt.combine(x, c);
                }
                product = crt.into_new_product();
                // Check whether the CRT has stabilised.
                let new_s_bits = max_bits(&s);
                let new_t_bits = max_bits(&t);
                stabilised = s_bits == new_s_bits && t_bits == new_t_bits;
                s_bits = new_s_bits;
                t_bits = new_t_bits;
            }
        }
        if stabilised {
            let log_len2 = len2.significant_bits();
            let bound1 = log_len2 + max_bits(xs) + max_bits(&s);
            let bound2 = log_len2 + max_bits(ys) + max_bits(&t);
            let bound = 4 + max(r.significant_bits(), max(bound1, bound2));
            if product.significant_bits() > bound {
                break;
            }
        }
    }
    trim(&mut s);
    trim(&mut t);
    (r, s, t)
}}

/// The resultant $r$ of `p` and `q`, which must be primitive or 0, together with $s$ and $t$ such
/// that $sp + tq = r$; if either polynomial is 0, or $r$ is, all three are 0. When `q` is longer
/// than `p`, the resultant is that of `q` and `p`, which may differ in sign.
///
/// This is not part of the public API; it is public so that `malachite-q` can compute extended GCDs
/// of `RationalPolynomial`s.
///
/// This is equivalent to `fmpz_poly_xgcd_modular` from `fmpz_poly/xgcd_modular.c`, FLINT 3.6.0.
#[doc(hidden)]
pub fn resultant_extended_gcd(
    p: &IntegerPolynomial,
    q: &IntegerPolynomial,
) -> (Integer, IntegerPolynomial, IntegerPolynomial) {
    if p.coefficients.len() < q.coefficients.len() {
        let (r, t, s) = resultant_extended_gcd(q, p);
        return (r, s, t);
    }
    if q.coefficients.is_empty() {
        return (
            Integer::ZERO,
            IntegerPolynomial::ZERO,
            IntegerPolynomial::ZERO,
        );
    }
    let (r, s, t) = resultant_extended_gcd_nonzero(&p.coefficients, &q.coefficients);
    (
        r,
        IntegerPolynomial { coefficients: s },
        IntegerPolynomial { coefficients: t },
    )
}
