// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::natural::Natural;
use alloc::vec::Vec;
use malachite_base::num::arithmetic::traits::AddMulAssign;
use malachite_base::num::basic::traits::One;
use malachite_base::num::conversion::traits::ExactFrom;

/// Computes the first `len` tangent numbers: $T_1$ through $T_{\mathrm{len}}$.
///
/// The tangent numbers $1, 2, 16, 272, 7936, \ldots$ are the coefficients of the Taylor series of
/// the tangent:
///
/// $$
/// \tan x = \sum_{j=1}^\infty T_j \frac{x^{2j-1}}{(2j-1)!}.
/// $$
///
/// $T_j$ is at index $j - 1$ of the result. The Bernoulli numbers are related to them by
/// $B_{2j} = (-1)^{j-1} 2j T_j / (4^{2j} - 2^{2j})$.
///
/// The algorithm starts from $T_j = (j - 1)!$ and makes $\mathrm{len} - 1$ passes over the table,
/// replacing $T_j$ by $(j - k) T_{j-1} + (j - k + 2) T_j$ for $j$ from $k$ to `len` in pass $k$,
/// so it uses only additions and multiplications by small numbers, and no cancellation occurs.
/// $T_j$ has $\Theta(j \log j)$ bits.
///
/// # Worst-case complexity
/// $T(n) = O(n^3 \log n)$
///
/// $M(n) = O(n^2 \log n)$
///
/// where $T$ is time, $M$ is additional memory, and $n$ is `len`.
///
/// # Examples
/// ```
/// use malachite_nz::natural::number_theory::tangent_numbers::tangent_numbers_prefix;
///
/// let prefix = tangent_numbers_prefix(6);
/// assert_eq!(
///     prefix.iter().map(|t| t.to_string()).collect::<Vec<_>>(),
///     ["1", "2", "16", "272", "7936", "353792"]
/// );
/// assert!(tangent_numbers_prefix(0).is_empty());
/// ```
///
/// This is equivalent to `tangentNumbers` from `AzNat/TangentNumbers.lean`, Azurite, which is
/// Algorithm 4.3 from Brent and Zimmermann, _Modern Computer Arithmetic_.
pub fn tangent_numbers_prefix(len: u64) -> Vec<Natural> {
    let len = usize::exact_from(len);
    let mut ts = Vec::with_capacity(len);
    let mut factorial = Natural::ONE;
    for j in 0..len {
        if j > 1 {
            factorial *= Natural::from(j);
        }
        ts.push(factorial.clone());
    }
    // `ts[j - 1]` holds T_j.
    for k in 2..=len {
        ts[k - 1] <<= 1u32;
        for j in k + 1..=len {
            let (lo, hi) = ts.split_at_mut(j - 1);
            let t = &mut hi[0];
            *t *= Natural::from(j - k + 2);
            t.add_mul_assign(&lo[j - 2], Natural::from(j - k));
        }
    }
    ts
}
