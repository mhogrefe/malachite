// Copyright © 2026 Mikhail Hogrefe
//
// Uses code adopted from the FLINT Library.
//
//      Copyright © 2007, 2008, 2009 William Hart
//
//      Copyright © 2011 Sebastian Pancratz
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::integer::Integer;
use crate::integer_polynomial::arithmetic::gcd::{
    LargePrimes, div_exact_by, max_bits, pseudo_rem_cohen, reduce, trim,
};
use crate::integer_vector::arithmetic::content::content;
use crate::natural::modular::crt_comb::CrtComb;
use crate::natural::{Natural, WIDTH_MINUS_1};
use crate::platform::Limb;
use alloc::vec::Vec;
use malachite_base::num::arithmetic::traits::{
    DivExact, FloorSqrt, Parity, Pow, Square, UnsignedAbs,
};
use malachite_base::num::basic::traits::{One, Zero};
use malachite_base::num::conversion::traits::ExactFrom;
use malachite_base::num::logic::traits::{NotAssign, SignificantBits};
use malachite_base::unsigned_polynomial::modular::mod_resultant::checked_resultant;
use malachite_base::unsigned_polynomial::modular::subproduct_tree::Modulus;

fn both_even(len1: usize, len2: usize) -> bool {
    len1.even() && len2.even()
}

// The resultant of the polynomials with coefficients `xs` and `ys`, where `xs` is at least as long
// as `ys` and neither is empty or ends with a zero, by the subresultant algorithm.
//
// This is equivalent to `_fmpz_poly_resultant_euclidean` from `fmpz_poly/resultant_euclidean.c`,
// FLINT 3.6.0.
crate_test_fn! {resultant_euclidean(xs: &[Integer], ys: &[Integer]) -> Integer {
    let (mut len1, mut len2) = (xs.len(), ys.len());
    if len2 == 1 {
        return (&ys[0]).pow(u64::exact_from(len1 - 1));
    }
    let a_content = Integer::from(content(xs));
    let b_content = Integer::from(content(ys));
    let mut a = div_exact_by(xs, &a_content);
    let mut b = div_exact_by(ys, &b_content);
    let mut g = Integer::ONE;
    let mut h = Integer::ONE;
    let t = a_content.pow(u64::exact_from(len2 - 1)) * b_content.pow(u64::exact_from(len1 - 1));
    let mut negative = false;
    loop {
        let d = u64::exact_from(len1 - len2);
        if both_even(len1, len2) {
            negative.not_assign();
        }
        a = pseudo_rem_cohen(&a, &b);
        trim(&mut a);
        len1 = a.len();
        if len1 == 0 {
            return Integer::ZERO;
        }
        core::mem::swap(&mut a, &mut b);
        core::mem::swap(&mut len1, &mut len2);
        let h_pow = (&h).pow(d);
        b = div_exact_by(&b, &(&g * &h_pow));
        g = a.last().unwrap().pow(d);
        h = (h * &g).div_exact(h_pow);
        g = a.last().unwrap().clone();
        if len2 <= 1 {
            break;
        }
    }
    let e = u64::exact_from(len1 - 1);
    let g = (&h).pow(e);
    let h = (h * b.last().unwrap().pow(e)).div_exact(g);
    let res = t * h;
    if negative { -res } else { res }
}}

// The resultant of the polynomials with coefficients `xs` and `ys`, where `xs` is at least as long
// as `ys` and neither is empty or ends with a zero, from their resultants modulo primes.
//
// This is equivalent to `_fmpz_poly_resultant_modular` from `fmpz_poly/resultant_modular.c`, FLINT
// 3.6.0.
crate_test_fn! {resultant_modular(xs: &[Integer], ys: &[Integer]) -> Integer {
    let (len1, len2) = (xs.len(), ys.len());
    if len2 == 1 {
        return (&ys[0]).pow(u64::exact_from(len1 - 1));
    }
    let a_content = content(xs);
    let b_content = content(ys);
    let a = div_exact_by(xs, &Integer::from(&a_content));
    let b = div_exact_by(ys, &Integer::from(&b_content));
    let l = (a.last().unwrap() * b.last().unwrap()).unsigned_abs();
    let mut candidates = LargePrimes::new();
    // |res(f, g)| <= |f|_2^deg(g) |g|_2^deg(f), by Theorem 7 of
    // https://doi.org/10.1016/j.laa.2009.08.012.
    let b1 = a.iter().map(|c| c.unsigned_abs_ref().square()).sum::<Natural>()
        .pow(u64::exact_from(len2 - 1));
    let b2 = b.iter().map(|c| c.unsigned_abs_ref().square()).sum::<Natural>()
        .pow(u64::exact_from(len1 - 1));
    let bound = ((b1 * b2).floor_sqrt() + Natural::ONE).significant_bits() + 2;
    let mut curr_bits = 0;
    let mut primes = Vec::new();
    let mut residues = Vec::new();
    let mut a_p: Vec<Limb> = Vec::with_capacity(len1);
    let mut b_p: Vec<Limb> = Vec::with_capacity(len2);
    while curr_bits < bound {
        let p = candidates.next().unwrap();
        if l.rem_limb_ref(p) == 0 {
            continue;
        }
        curr_bits += WIDTH_MINUS_1;
        a_p.clear();
        a_p.extend(a.iter().map(|c| reduce(c, p)));
        b_p.clear();
        b_p.extend(b.iter().map(|c| reduce(c, p)));
        primes.push(p);
        residues.push(checked_resultant(&a_p, &b_p, &Modulus(p)).unwrap());
    }
    let mut res = CrtComb::new(&primes).unwrap().combine_balanced(&residues);
    if a_content != 1u32 {
        res *= Integer::from(a_content).pow(u64::exact_from(len2 - 1));
    }
    if b_content != 1u32 {
        res *= Integer::from(b_content).pow(u64::exact_from(len1 - 1));
    }
    res
}}

// The resultant of the polynomials with coefficients `xs` and `ys`, where `xs` is at least as long
// as `ys` and neither is empty or ends with a zero.
//
// This is equivalent to `_fmpz_poly_resultant` from `fmpz_poly/resultant.c`, FLINT 3.6.0.
pub(crate) fn resultant_nonzero(xs: &[Integer], ys: &[Integer]) -> Integer {
    let len2 = ys.len();
    let bits = max_bits(xs) + max_bits(ys);
    let len2_cubed = u64::exact_from(len2).pow(3);
    if len2 > 144 || len2_cubed.saturating_mul(bits) > 6000000 {
        resultant_modular(xs, ys)
    } else {
        resultant_euclidean(xs, ys)
    }
}
