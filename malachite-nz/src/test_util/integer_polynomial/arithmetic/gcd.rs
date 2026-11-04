// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::integer::Integer;
use crate::integer_polynomial::IntegerPolynomial;
use crate::integer_polynomial::arithmetic::gcd::{gcd_heuristic, gcd_modular, gcd_subresultant};
use alloc::vec::Vec;
use malachite_base::num::basic::traits::Zero;
use malachite_base::polynomial::Polynomial;

// The GCD by the given algorithm, which takes nonzero polynomials with the first at least as long;
// the other cases are done as the library does them.
fn gcd_with(
    p: &IntegerPolynomial,
    q: &IntegerPolynomial,
    algorithm: &dyn Fn(&[Integer], &[Integer]) -> Option<Vec<Integer>>,
) -> Option<IntegerPolynomial> {
    let (p, q) = if p.len() < q.len() { (q, p) } else { (p, q) };
    if *p == IntegerPolynomial::ZERO {
        Some(IntegerPolynomial::ZERO)
    } else if *q == IntegerPolynomial::ZERO {
        Some(if *p.leading_coefficient() > 0u32 {
            p.clone()
        } else {
            -p
        })
    } else {
        Some(IntegerPolynomial::from_coefficients_asc(algorithm(
            p.coefficients_asc(),
            q.coefficients_asc(),
        )?))
    }
}

pub fn gcd_subresultant_wrapper(p: &IntegerPolynomial, q: &IntegerPolynomial) -> IntegerPolynomial {
    gcd_with(p, q, &|xs, ys| Some(gcd_subresultant(xs, ys))).unwrap()
}

// `None` when the heuristic does not find the GCD.
pub fn gcd_heuristic_wrapper(
    p: &IntegerPolynomial,
    q: &IntegerPolynomial,
) -> Option<IntegerPolynomial> {
    gcd_with(p, q, &gcd_heuristic)
}

pub fn gcd_modular_wrapper(p: &IntegerPolynomial, q: &IntegerPolynomial) -> IntegerPolynomial {
    gcd_with(p, q, &|xs, ys| Some(gcd_modular(xs, ys))).unwrap()
}
