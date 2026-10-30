// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::num::basic::unsigneds::PrimitiveUnsigned;
use crate::unsigned_polynomial::UnsignedPolynomial;
use crate::unsigned_polynomial::arithmetic::mod_power_of_2_div_exact::*;
use crate::unsigned_polynomial::arithmetic::mod_power_of_2_div_rem::mod_power_of_2_quotient_with;
use crate::unsigned_polynomial::arithmetic::mod_power_of_2_div_truncated::*;

// The quotient of two polynomials modulo 2^k, assuming that the division is exact, computed
// entirely from the top.
pub fn mod_power_of_2_div_exact_top_wrapper<T: PrimitiveUnsigned>(
    p: &UnsignedPolynomial<T>,
    q: &UnsignedPolynomial<T>,
    pow: u64,
) -> Option<UnsignedPolynomial<T>> {
    mod_power_of_2_div_exact_with(
        p.coefficients_asc(),
        q.coefficients_asc(),
        pow,
        |xs, ys, pow, inverse| {
            mod_power_of_2_quotient_with(
                xs,
                ys,
                pow,
                inverse,
                mod_power_of_2_div_truncated_dispatch,
            )
        },
    )
}

// The quotient of two polynomials modulo 2^k, assuming that the division is exact, computed from
// both ends, with the power series divided coefficient by coefficient.
pub fn mod_power_of_2_div_exact_bidirectional_basecase_wrapper<T: PrimitiveUnsigned>(
    p: &UnsignedPolynomial<T>,
    q: &UnsignedPolynomial<T>,
    pow: u64,
) -> Option<UnsignedPolynomial<T>> {
    mod_power_of_2_div_exact_with(
        p.coefficients_asc(),
        q.coefficients_asc(),
        pow,
        |xs, ys, pow, inverse| {
            mod_power_of_2_div_exact_bidirectional(xs, ys, pow, inverse, basecase)
        },
    )
}

// The quotient of two polynomials modulo 2^k, assuming that the division is exact, computed from
// both ends, with the power series divided by the method of Karp and Markstein.
pub fn mod_power_of_2_div_exact_bidirectional_newton_wrapper<T: PrimitiveUnsigned>(
    p: &UnsignedPolynomial<T>,
    q: &UnsignedPolynomial<T>,
    pow: u64,
) -> Option<UnsignedPolynomial<T>> {
    mod_power_of_2_div_exact_with(
        p.coefficients_asc(),
        q.coefficients_asc(),
        pow,
        |xs, ys, pow, inverse| {
            mod_power_of_2_div_exact_bidirectional(
                xs,
                ys,
                pow,
                inverse,
                mod_power_of_2_div_truncated_newton,
            )
        },
    )
}

// `mod_power_of_2_div_truncated_basecase`, with the signature of the other kernels.
fn basecase<T: PrimitiveUnsigned>(xs: &[T], ys: &[T], n: usize, _pow: u64, inverse: T) -> Vec<T> {
    mod_power_of_2_div_truncated_basecase(xs, ys, n, inverse)
}
