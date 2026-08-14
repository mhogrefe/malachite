// Copyright © 2026 Mikhail Hogrefe
//
// Uses code adopted from the FLINT Library.
//
//      Copyright © 2012 Fredrik Johansson
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::natural::Natural;
use alloc::vec;
use alloc::vec::Vec;
use malachite_base::num::basic::traits::One;
use malachite_base::num::conversion::traits::ExactFrom;
use malachite_base::num::factorization::traits::IsPrime;

/// Computes the first `len` values of Landau's function: $g(0)$ through $g(\mathrm{len} - 1)$,
/// where $g(n)$ is the largest order of a permutation of $n$ elements, or equivalently the largest
/// least common multiple of any partition of $n$.
///
/// The values are found by a knapsack over prime powers: an optimal partition uses distinct
/// prime-power parts, and no prime larger than about $1.328\sqrt{n \ln n}$ can appear, so each
/// prime up to that bound is offered to every index in descending order.
///
/// # Worst-case complexity
/// $T(n) = O(n^2)$
///
/// $M(n) = O(n^2)$
///
/// where $T$ is time, $M$ is additional memory, and $n$ is `len`.
///
/// # Examples
/// ```
/// use malachite_nz::natural::arithmetic::landau_function::landau_function_prefix;
///
/// let prefix = landau_function_prefix(13);
/// assert_eq!(
///     prefix.iter().map(|g| g.to_string()).collect::<Vec<_>>(),
///     ["1", "1", "2", "3", "4", "6", "6", "12", "15", "20", "30", "30", "60"]
/// );
/// assert!(landau_function_prefix(0).is_empty());
/// ```
///
/// This is equivalent to `arith_landau_function_vec` from `arith/landau_function_vec.c`, FLINT
/// 3.6.0.
pub fn landau_function_prefix(len: u64) -> Vec<Natural> {
    let ulen = usize::exact_from(len);
    let mut res = vec![Natural::ONE; ulen];
    if ulen < 2 {
        return res;
    }
    // No prime beyond this bound can be part of an optimal factorization.
    let pmax = (1.328 * libm::sqrt(len as f64 * libm::log(len as f64) + 1.0)) as u64;
    let mut p = 2u64;
    while p <= pmax {
        // Descending indices make this a knapsack in which each prime contributes at most one power
        // to each value.
        for n in (usize::exact_from(p)..ulen).rev() {
            let mut pk = p;
            while pk <= u64::exact_from(n) {
                let candidate = &res[n - usize::exact_from(pk)] * Natural::from(pk);
                if res[n] < candidate {
                    res[n] = candidate;
                }
                // A power that no longer fits in a word cannot be at most n either; FLINT tracks
                // the same overflow with a high word.
                match pk.checked_mul(p) {
                    Some(next) => pk = next,
                    None => break,
                }
            }
        }
        p += 1;
        while !p.is_prime() {
            p += 1;
        }
    }
    res
}
