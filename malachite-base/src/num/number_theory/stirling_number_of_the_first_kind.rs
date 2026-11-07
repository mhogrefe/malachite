// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::num::arithmetic::traits::Parity;
use crate::num::basic::signeds::PrimitiveSigned;
use crate::num::basic::unsigneds::PrimitiveUnsigned;
use crate::num::conversion::traits::{ConvertibleFrom, WrappingFrom};
use crate::num::number_theory::traits::{
    CheckedStirlingNumberOfTheFirstKind, StirlingNumberOfTheFirstKind,
};

// s(n, k) = (-1)^(n + k) [n, k] in `S`, or `None` if it does not fit. A negative value fits exactly
// when its magnitude is at most 2^(W - 1), where W is the width of `S`.
fn checked_stirling_number_of_the_first_kind<
    U: PrimitiveUnsigned,
    S: ConvertibleFrom<U> + PrimitiveSigned + WrappingFrom<U>,
>(
    n: u64,
    k: u64,
) -> Option<S> {
    let u = U::checked_unsigned_stirling_number_of_the_first_kind(n, k)?;
    if n.even() == k.even() {
        if S::convertible_from(u) {
            Some(S::wrapping_from(u))
        } else {
            None
        }
    } else if u <= U::power_of_2(S::WIDTH - 1) {
        Some(S::wrapping_from(u).wrapping_neg())
    } else {
        None
    }
}

macro_rules! impl_stirling_number_of_the_first_kind {
    ($u:ident, $s:ident) => {
        impl CheckedStirlingNumberOfTheFirstKind for $s {
            /// Computes the Stirling number of the first kind $s(n, k) = (-1)^{n + k} \left[{n
            /// \atop k}\right]$, the coefficient of $x^k$ in the falling factorial $x(x - 1) \cdots
            /// (x - n + 1)$.
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
            /// [here](super::stirling_number_of_the_first_kind#checked_stirling_number_of_the_first_kind).
            #[inline]
            fn checked_stirling_number_of_the_first_kind(n: u64, k: u64) -> Option<$s> {
                checked_stirling_number_of_the_first_kind::<$u, $s>(n, k)
            }
        }

        impl StirlingNumberOfTheFirstKind for $s {
            /// Computes the Stirling number of the first kind $s(n, k) = (-1)^{n + k} \left[{n
            /// \atop k}\right]$, the coefficient of $x^k$ in the falling factorial $x(x - 1) \cdots
            /// (x - n + 1)$.
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
            /// [here](super::stirling_number_of_the_first_kind#stirling_number_of_the_first_kind).
            #[inline]
            fn stirling_number_of_the_first_kind(n: u64, k: u64) -> $s {
                $s::checked_stirling_number_of_the_first_kind(n, k).unwrap()
            }
        }
    };
}
apply_to_unsigned_signed_pairs!(impl_stirling_number_of_the_first_kind);
