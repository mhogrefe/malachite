// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::natural::Natural;
use crate::natural_polynomial::NaturalPolynomial;
use crate::natural_polynomial::special::touchard_polynomial::{
    stirling_2_vec_convolution, stirling_2_vec_multi_mod, stirling_2_vec_triangular,
};
use alloc::vec;
use malachite_base::num::basic::traits::{One, Zero};
use malachite_base::num::conversion::traits::ExactFrom;
use malachite_base::polynomial::Polynomial;

// The $n$th Touchard polynomial truncated to its first `len` coefficients, the Stirling numbers of
// the second kind $S(n, k)$ for $k < \text{len}$, by filling the whole triangle with the recurrence
// $S(m, k) = kS(m - 1, k) + S(m - 1, k - 1)$.
pub fn touchard_polynomial_truncated_naive(n: u64, len: u64) -> NaturalPolynomial {
    let un = usize::exact_from(n);
    // row[k] = S(m, k) for the current m, starting from m = 0
    let mut row = vec![Natural::ZERO; un + 1];
    row[0] = Natural::ONE;
    for m in 1..=un {
        for k in (1..=m).rev() {
            let previous = row[k - 1].clone();
            row[k] *= Natural::from(k);
            row[k] += previous;
        }
        row[0] = Natural::ZERO;
    }
    row.truncate(usize::exact_from(len));
    NaturalPolynomial::from_coefficients_asc(row)
}

fn run(f: fn(&mut [Natural], u64, usize), n: u64, len: u64) -> NaturalPolynomial {
    let len = usize::exact_from(len);
    let mut row = vec![Natural::ZERO; len];
    f(&mut row, n, len);
    NaturalPolynomial::from_coefficients_asc(row)
}

pub fn touchard_polynomial_truncated_triangular(n: u64, len: u64) -> NaturalPolynomial {
    run(stirling_2_vec_triangular, n, len)
}

pub fn touchard_polynomial_truncated_convolution(n: u64, len: u64) -> NaturalPolynomial {
    run(stirling_2_vec_convolution, n, len)
}

pub fn touchard_polynomial_truncated_multi_mod(n: u64, len: u64) -> NaturalPolynomial {
    run(stirling_2_vec_multi_mod, n, len)
}
