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
    CheckedUnsignedStirlingNumberOfTheFirstKind, UnsignedStirlingNumberOfTheFirstKind,
};
use alloc::vec;

// [n, k], the unsigned Stirling number of the first kind, in `T`, or `None` if it does not fit.
//
// The trivial cases and k = 1, n - 1, and n - 2 have closed forms. Otherwise [n, k] depends only on
// the entries [m, j] of the triangle with 1 <= j <= k and m - j <= n - k, which are found row by
// row with the recurrence [m, j] = (m - 1)[m - 1, j] + [m - 1, j - 1]. Every such entry is at most
// [n, k], since the recurrence never decreases along a row or a diagonal, so the first overflow
// means that [n, k] does not fit either.
//
// The recurrence also dominates Pascal's, so [n, k] >= C(n - 1, k - 1) >= 2^min(k - 1, n - k).
// Hence [n, k] can only fit when min(k - 1, n - k) < `T::WIDTH`, and then each row's band has at
// most `T::WIDTH` + 1 entries when it is indexed by column j (if k <= n - k) or by diagonal m - j
// (otherwise).
fn checked_unsigned_stirling_number_of_the_first_kind<T: PrimitiveUnsigned>(
    n: u64,
    k: u64,
) -> Option<T> {
    if k > n || k == 0 && n != 0 {
        return Some(T::ZERO);
    }
    if k == n {
        return Some(T::ONE);
    }
    if k == 1 {
        // [n, 1] = (n - 1)!
        return T::checked_factorial(n - 1);
    }
    let tn = T::try_from(n).ok()?;
    if k == n - 1 {
        // [n, n - 1] = C(n, 2)
        return T::checked_binomial_coefficient(tn, T::TWO);
    }
    if k == n - 2 {
        // [n, n - 2] = 2C(n, 3) + 3C(n, 4). Doubling C(n, 3) cannot overflow once C(n, 4) fits:
        // C(n, 4) = C(n, 3)(n - 3)/4 is at least 2C(n, 3) for n >= 11, and 2C(10, 3) = 240.
        let c3 = T::checked_binomial_coefficient(tn, T::wrapping_from(3u8))?;
        let c4 = T::checked_binomial_coefficient(tn, T::wrapping_from(4u8))?;
        return (c3 << 1u64).checked_add(c4.checked_mul(T::wrapping_from(3u8))?);
    }
    let d = n - k;
    if (k - 1).min(d) >= T::WIDTH {
        return None;
    }
    if k <= d {
        // c[j] = [m, j] for the j in the band of row m; c[0] = [m, 0]
        let uk = usize::exact_from(k);
        let mut c = vec![T::ZERO; uk + 1];
        c[0] = T::ONE;
        for m in 1..=n {
            let factor = T::wrapping_from(m - 1);
            let lo = usize::exact_from(m.saturating_sub(d).max(1));
            for j in (lo..=usize::exact_from(m.min(k))).rev() {
                c[j] = c[j].checked_mul(factor)?.checked_add(c[j - 1])?;
            }
            c[0] = T::ZERO;
        }
        Some(c[uk])
    } else {
        // b[e] = [m, m - e] for the e in the band of row m; b[0] = [m, m] = 1
        let ud = usize::exact_from(d);
        let mut b = vec![T::ZERO; ud + 1];
        b[0] = T::ONE;
        for m in 2..=n {
            let factor = T::wrapping_from(m - 1);
            let lo = usize::exact_from(m.saturating_sub(k).max(1));
            for e in (lo..=usize::exact_from((m - 1).min(d))).rev() {
                b[e] = b[e - 1].checked_mul(factor)?.checked_add(b[e])?;
            }
        }
        Some(b[ud])
    }
}

macro_rules! impl_unsigned_stirling_number_of_the_first_kind {
    ($t:ident) => {
        impl CheckedUnsignedStirlingNumberOfTheFirstKind for $t {
            /// Computes the unsigned Stirling number of the first kind $\left[{n \atop k}\right]$:
            /// the number of permutations of $n$ elements with $k$ cycles.
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
            /// [here](super::unsigned_stirling_number_of_the_first_kind#checked_unsigned_stirling_number_of_the_first_kind).
            #[inline]
            fn checked_unsigned_stirling_number_of_the_first_kind(n: u64, k: u64) -> Option<$t> {
                checked_unsigned_stirling_number_of_the_first_kind(n, k)
            }
        }

        impl UnsignedStirlingNumberOfTheFirstKind for $t {
            /// Computes the unsigned Stirling number of the first kind $\left[{n \atop k}\right]$:
            /// the number of permutations of $n$ elements with $k$ cycles.
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
            /// [here](super::unsigned_stirling_number_of_the_first_kind#unsigned_stirling_number_of_the_first_kind).
            #[inline]
            fn unsigned_stirling_number_of_the_first_kind(n: u64, k: u64) -> $t {
                $t::checked_unsigned_stirling_number_of_the_first_kind(n, k).unwrap()
            }
        }
    };
}
impl_unsigned_stirling_number_of_the_first_kind!(u8);
impl_unsigned_stirling_number_of_the_first_kind!(u16);
impl_unsigned_stirling_number_of_the_first_kind!(u32);
impl_unsigned_stirling_number_of_the_first_kind!(u64);
impl_unsigned_stirling_number_of_the_first_kind!(u128);
impl_unsigned_stirling_number_of_the_first_kind!(usize);
