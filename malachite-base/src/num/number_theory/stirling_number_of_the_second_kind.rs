// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::num::basic::unsigneds::PrimitiveUnsigned;
use crate::num::conversion::traits::ExactFrom;
use crate::num::number_theory::traits::{
    CheckedStirlingNumberOfTheSecondKind, StirlingNumberOfTheSecondKind,
};
use alloc::vec;

// S(n, k) in `T`, or `None` if it does not fit.
//
// The trivial cases and k = 2, n - 1, and n - 2 have closed forms. Otherwise S(n, k) depends only
// on the entries S(m, j) of the triangle with 1 <= j <= k and m - j <= n - k, which are found row
// by row with the recurrence S(m, j) = jS(m - 1, j) + S(m - 1, j - 1). Every such entry is at most
// S(n, k), since the recurrence never decreases along a row or a diagonal, so the first overflow
// means that S(n, k) does not fit either.
//
// The recurrence also dominates Pascal's, so S(n, k) >= C(n - 1, k - 1) >= 2^min(k - 1, n - k).
// Hence S(n, k) can only fit when min(k - 1, n - k) < `T::WIDTH`, and then each row's band has at
// most `T::WIDTH` + 1 entries when it is indexed by column j (if k <= n - k) or by diagonal m - j
// (otherwise).
fn checked_stirling_number_of_the_second_kind<T: PrimitiveUnsigned>(n: u64, k: u64) -> Option<T> {
    if k > n || k == 0 && n != 0 {
        return Some(T::ZERO);
    }
    if k == n || k == 1 {
        return Some(T::ONE);
    }
    if k == 2 {
        // S(n, 2) = 2^(n - 1) - 1
        return if n - 1 <= T::WIDTH {
            Some(T::low_mask(n - 1))
        } else {
            None
        };
    }
    let tn = T::try_from(n).ok()?;
    if k == n - 1 {
        // S(n, n - 1) = C(n, 2)
        return T::checked_binomial_coefficient(tn, T::TWO);
    }
    if k == n - 2 {
        // S(n, n - 2) = C(n, 3) + 3C(n, 4)
        let c3 = T::checked_binomial_coefficient(tn, T::wrapping_from(3u8))?;
        let c4 = T::checked_binomial_coefficient(tn, T::wrapping_from(4u8))?;
        return c4.checked_mul(T::wrapping_from(3u8))?.checked_add(c3);
    }
    let d = n - k;
    if (k - 1).min(d) >= T::WIDTH {
        return None;
    }
    if k <= d {
        // c[j] = S(m, j) for the j in the band of row m; c[0] = S(m, 0)
        let uk = usize::exact_from(k);
        let mut c = vec![T::ZERO; uk + 1];
        c[0] = T::ONE;
        for m in 1..=n {
            let lo = usize::exact_from(m.saturating_sub(d).max(1));
            for j in (lo..=usize::exact_from(m.min(k))).rev() {
                c[j] = c[j]
                    .checked_mul(T::wrapping_from(j))?
                    .checked_add(c[j - 1])?;
            }
            c[0] = T::ZERO;
        }
        Some(c[uk])
    } else {
        // b[e] = S(m, m - e) for the e in the band of row m; b[0] = S(m, m) = 1
        let ud = usize::exact_from(d);
        let mut b = vec![T::ZERO; ud + 1];
        b[0] = T::ONE;
        for m in 2..=n {
            let lo = usize::exact_from(m.saturating_sub(k).max(1));
            for e in (lo..=usize::exact_from((m - 1).min(d))).rev() {
                b[e] = b[e - 1]
                    .checked_mul(T::wrapping_from(m - e as u64))?
                    .checked_add(b[e])?;
            }
        }
        Some(b[ud])
    }
}

macro_rules! impl_stirling_number_of_the_second_kind {
    ($t:ident) => {
        impl CheckedStirlingNumberOfTheSecondKind for $t {
            /// Computes the Stirling number of the second kind $S(n, k)$: the number of ways to
            /// partition a set of $n$ elements into $k$ nonempty subsets.
            ///
            /// If the result is too large to be represented, the function returns `None`.
            ///
            /// # Worst-case complexity
            /// $T(n, k) = O(n \min(k, n - k))$
            ///
            /// $M(n, k) = O(\min(k, n - k))$
            ///
            /// where $T$ is time, $M$ is additional memory, $n$ is `n`, and $k$ is `k`.
            ///
            /// # Examples
            /// See
            /// [here](super::stirling_number_of_the_second_kind#checked_stirling_number_of_the_second_kind).
            #[inline]
            fn checked_stirling_number_of_the_second_kind(n: u64, k: u64) -> Option<$t> {
                checked_stirling_number_of_the_second_kind(n, k)
            }
        }

        impl StirlingNumberOfTheSecondKind for $t {
            /// Computes the Stirling number of the second kind $S(n, k)$: the number of ways to
            /// partition a set of $n$ elements into $k$ nonempty subsets.
            ///
            /// # Worst-case complexity
            /// $T(n, k) = O(n \min(k, n - k))$
            ///
            /// $M(n, k) = O(\min(k, n - k))$
            ///
            /// where $T$ is time, $M$ is additional memory, $n$ is `n`, and $k$ is `k`.
            ///
            /// # Panics
            /// Panics if the result is too large to be represented.
            ///
            /// # Examples
            /// See
            /// [here](super::stirling_number_of_the_second_kind#stirling_number_of_the_second_kind).
            #[inline]
            fn stirling_number_of_the_second_kind(n: u64, k: u64) -> $t {
                $t::checked_stirling_number_of_the_second_kind(n, k).unwrap()
            }
        }
    };
}
impl_stirling_number_of_the_second_kind!(u8);
impl_stirling_number_of_the_second_kind!(u16);
impl_stirling_number_of_the_second_kind!(u32);
impl_stirling_number_of_the_second_kind!(u64);
impl_stirling_number_of_the_second_kind!(u128);
impl_stirling_number_of_the_second_kind!(usize);
