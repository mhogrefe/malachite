// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::natural::Natural;
use crate::natural_polynomial::NaturalPolynomial;
use crate::natural_polynomial::arithmetic::mod_div_exact::*;
use crate::natural_polynomial::arithmetic::mod_div_rem::mod_quotient_with;
use crate::natural_polynomial::arithmetic::mod_div_truncated::*;

// The quotient of two polynomials modulo m, assuming that the division is exact, computed entirely
// from the top.
pub fn mod_div_exact_top_wrapper(
    p: &NaturalPolynomial,
    q: &NaturalPolynomial,
    m: &Natural,
) -> Option<NaturalPolynomial> {
    mod_div_exact_with(
        p.coefficients_asc(),
        q.coefficients_asc(),
        m,
        |xs, ys, m, inverse| mod_quotient_with(xs, ys, m, inverse, mod_div_truncated_dispatch),
    )
}

// The quotient of two polynomials modulo m, assuming that the division is exact, computed from both
// ends, with the power series divided coefficient by coefficient.
pub fn mod_div_exact_bidirectional_basecase_wrapper(
    p: &NaturalPolynomial,
    q: &NaturalPolynomial,
    m: &Natural,
) -> Option<NaturalPolynomial> {
    mod_div_exact_with(
        p.coefficients_asc(),
        q.coefficients_asc(),
        m,
        |xs, ys, m, inverse| {
            mod_div_exact_bidirectional(xs, ys, m, inverse, mod_div_truncated_basecase)
        },
    )
}

// The quotient of two polynomials modulo m, assuming that the division is exact, computed from both
// ends, with the power series divided by the method of Karp and Markstein.
pub fn mod_div_exact_bidirectional_newton_wrapper(
    p: &NaturalPolynomial,
    q: &NaturalPolynomial,
    m: &Natural,
) -> Option<NaturalPolynomial> {
    mod_div_exact_with(
        p.coefficients_asc(),
        q.coefficients_asc(),
        m,
        |xs, ys, m, inverse| {
            mod_div_exact_bidirectional(xs, ys, m, inverse, mod_div_truncated_newton)
        },
    )
}
