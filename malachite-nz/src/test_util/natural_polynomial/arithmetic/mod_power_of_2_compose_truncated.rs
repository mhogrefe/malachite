// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::natural_polynomial::NaturalPolynomial;
use crate::natural_polynomial::arithmetic::mod_power_of_2_compose_truncated::*;
use malachite_base::num::arithmetic::traits::ModPowerOf2;
use malachite_base::polynomial::ComposeTruncated;

// Composes two polynomials exactly as power series, keeping the first `len` coefficients, and
// reduces the result modulo 2^pow afterwards, letting the coefficients grow along the way.
pub fn mod_power_of_2_compose_truncated_naive(
    p: &NaturalPolynomial,
    q: &NaturalPolynomial,
    len: u64,
    pow: u64,
) -> NaturalPolynomial {
    p.compose_truncated(q, len).mod_power_of_2(pow)
}

// The composition of two polynomials modulo 2^pow as power series, by Horner's rule over
// `Natural`s, whatever the size of 2^pow.
pub fn mod_power_of_2_compose_truncated_horner_wrapper(
    p: &NaturalPolynomial,
    q: &NaturalPolynomial,
    len: u64,
    pow: u64,
) -> NaturalPolynomial {
    mod_power_of_2_compose_truncated_with(
        p.coefficients_asc(),
        q.coefficients_asc(),
        len,
        pow,
        mod_power_of_2_compose_truncated_horner,
    )
}

// The composition of two polynomials modulo 2^pow as power series, by divide and conquer over
// `Natural`s, whatever the size of 2^pow.
pub fn mod_power_of_2_compose_truncated_divide_and_conquer_wrapper(
    p: &NaturalPolynomial,
    q: &NaturalPolynomial,
    len: u64,
    pow: u64,
) -> NaturalPolynomial {
    mod_power_of_2_compose_truncated_with(
        p.coefficients_asc(),
        q.coefficients_asc(),
        len,
        pow,
        mod_power_of_2_compose_truncated_divide_and_conquer,
    )
}

// The composition of two polynomials modulo 2^pow as power series, by the Brent–Kung algorithm
// over `Natural`s, whatever the size of 2^pow.
pub fn mod_power_of_2_compose_truncated_brent_kung_wrapper(
    p: &NaturalPolynomial,
    q: &NaturalPolynomial,
    len: u64,
    pow: u64,
) -> NaturalPolynomial {
    mod_power_of_2_compose_truncated_with(
        p.coefficients_asc(),
        q.coefficients_asc(),
        len,
        pow,
        mod_power_of_2_compose_truncated_brent_kung,
    )
}
