// Copyright © 2026 Mikhail Hogrefe
//
// Uses code adopted from the FLINT Library.
//
//      Copyright © 2008, 2009 William Hart
//
//      Copyright © 2010, 2011 Sebastian Pancratz
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::integer::Integer;
use crate::integer_polynomial::IntegerPolynomial;
use crate::integer_polynomial::arithmetic::pseudo_div_rem::*;
use malachite_base::num::arithmetic::traits::{DivExact, DivisibleBy, Pow, SubMulAssign};
use malachite_base::num::basic::traits::Zero;
use malachite_base::polynomial::Polynomial;

// The pseudo-quotient and pseudo-remainder of `p` by `q` with the exponent `deg p - deg q + 1`: the
// leading coefficient of the remainder is cancelled one term at a time, after the remainder is
// multiplied by the divisor's leading coefficient every time.
//
// This is equivalent to `fmpz_poly_pseudo_divrem_cohen` from `fmpz_poly/pseudo_divrem_cohen.c`,
// FLINT 3.6.0.
pub fn pseudo_div_rem_cohen(
    p: &IntegerPolynomial,
    q: &IntegerPolynomial,
) -> (IntegerPolynomial, IntegerPolynomial) {
    let xs = p.coefficients_asc();
    let ys = q.coefficients_asc();
    let lead = ys.last().expect("division by zero");
    if xs.len() < ys.len() {
        return (IntegerPolynomial::ZERO, p.clone());
    }
    let q_len = xs.len() - ys.len() + 1;
    let mut qs = vec![Integer::ZERO; q_len];
    let mut rs = xs.to_vec();
    for i in (0..q_len).rev() {
        let top = i + ys.len() - 1;
        for q in &mut qs {
            *q *= lead;
        }
        qs[i] += &rs[top];
        let c = rs[top].clone();
        for r in &mut rs[..top] {
            *r *= lead;
        }
        for (r, y) in rs[i..top].iter_mut().zip(ys) {
            r.sub_mul_assign(y, &c);
        }
        rs[top] = Integer::ZERO;
    }
    rs.truncate(ys.len() - 1);
    (
        IntegerPolynomial::from_coefficients_asc(qs),
        IntegerPolynomial::from_coefficients_asc(rs),
    )
}

// The pseudo-quotient, pseudo-remainder, and least exponent of `p` by `q`: the classical pseudo-
// quotient and pseudo-remainder, divided by the divisor's leading coefficient while it divides
// every coefficient of both.
pub fn pseudo_div_rem_naive(
    p: &IntegerPolynomial,
    q: &IntegerPolynomial,
) -> (IntegerPolynomial, IntegerPolynomial, u64) {
    let lead = q
        .coefficients_asc()
        .last()
        .expect("division by zero")
        .clone();
    if p.len() < q.len() {
        return (IntegerPolynomial::ZERO, p.clone(), 0);
    }
    let (mut s, mut r) = pseudo_div_rem_cohen(p, q);
    let mut d = p.len() - q.len() + 1;
    while d != 0
        && s.coefficients_asc()
            .iter()
            .chain(r.coefficients_asc())
            .all(|x| x.divisible_by(&lead))
    {
        s = IntegerPolynomial::from_coefficients_asc(
            s.coefficients_asc()
                .iter()
                .map(|x| x.div_exact(&lead))
                .collect(),
        );
        r = IntegerPolynomial::from_coefficients_asc(
            r.coefficients_asc()
                .iter()
                .map(|x| x.div_exact(&lead))
                .collect(),
        );
        d -= 1;
    }
    (s, r, d)
}

// The pseudo-quotient, pseudo-remainder, and least exponent of `p` by `q`, by the basecase
// algorithm.
pub fn pseudo_div_rem_basecase_wrapper(
    p: &IntegerPolynomial,
    q: &IntegerPolynomial,
) -> (IntegerPolynomial, IntegerPolynomial, u64) {
    pseudo_div_rem_with(
        p.coefficients_asc(),
        q.coefficients_asc(),
        pseudo_div_rem_basecase,
    )
}

// The pseudo-quotient, pseudo-remainder, and least exponent of `p` by `q`, by divide-and-conquer.
pub fn pseudo_div_rem_divide_and_conquer_wrapper(
    p: &IntegerPolynomial,
    q: &IntegerPolynomial,
) -> (IntegerPolynomial, IntegerPolynomial, u64) {
    pseudo_div_rem_with(
        p.coefficients_asc(),
        q.coefficients_asc(),
        pseudo_div_rem_divide_and_conquer,
    )
}

// The leading coefficient of `q` to the power `d`, as a constant check of the identity $\ell^d p =
// qs + r$.
pub fn pseudo_div_rem_scale(q: &IntegerPolynomial, d: u64) -> Integer {
    q.coefficients_asc().last().unwrap().pow(d)
}
