// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::num::arithmetic::traits::{EntrywiseShrRound, EntrywiseShrRoundAssign};
use crate::num::basic::unsigneds::PrimitiveUnsigned;
use crate::rounding_modes::RoundingMode;
use crate::unsigned_vector::UnsignedVector;

macro_rules! impl_entrywise_shr_round_unsigned {
    ($t:ident) => {
        impl<T: PrimitiveUnsigned> EntrywiseShrRound<$t> for UnsignedVector<T> {
            type Output = UnsignedVector<T>;

            /// Right-shifts an [`UnsignedVector`] (divides it by a power of 2), taking it by value,
            /// and rounds every element according to the specified rounding mode.
            ///
            /// Each element is rounded as by
            /// [`ShrRound`](crate::num::arithmetic::traits::ShrRound), but no
            /// [`Ordering`](core::cmp::Ordering) is returned, since with `Nearest` different
            /// elements may be rounded in different directions. Passing `Floor` or `Down` is
            /// equivalent to using `>>`. If `bits` is at least `T::WIDTH`, every element rounds to
            /// 0 or 1.
            ///
            /// $f(v, k, r)_i = \operatorname{round}(v_i/2^k)$, where $\operatorname{round}$ rounds
            /// according to `rm`.
            ///
            /// # Worst-case complexity
            /// $T(n) = O(n)$
            ///
            /// $M(n) = O(1)$
            ///
            /// where $T$ is time, $M$ is additional memory, and $n$ is `self.dimension()`.
            ///
            /// # Panics
            /// Panics if `rm` is `Exact` and some element is not divisible by $2^k$.
            ///
            /// # Examples
            /// See [here](super::entrywise_shr_round#entrywise_shr_round).
            #[inline]
            fn entrywise_shr_round(mut self, bits: $t, rm: RoundingMode) -> UnsignedVector<T> {
                self.entrywise_shr_round_assign(bits, rm);
                self
            }
        }

        impl<T: PrimitiveUnsigned> EntrywiseShrRound<$t> for &UnsignedVector<T> {
            type Output = UnsignedVector<T>;

            /// Right-shifts an [`UnsignedVector`] (divides it by a power of 2), taking it by
            /// reference, and rounds every element according to the specified rounding mode.
            ///
            /// Each element is rounded as by
            /// [`ShrRound`](crate::num::arithmetic::traits::ShrRound), but no
            /// [`Ordering`](core::cmp::Ordering) is returned, since with `Nearest` different
            /// elements may be rounded in different directions. Passing `Floor` or `Down` is
            /// equivalent to using `>>`. If `bits` is at least `T::WIDTH`, every element rounds to
            /// 0 or 1.
            ///
            /// $f(v, k, r)_i = \operatorname{round}(v_i/2^k)$, where $\operatorname{round}$ rounds
            /// according to `rm`.
            ///
            /// # Worst-case complexity
            /// $T(n) = O(n)$
            ///
            /// $M(n) = O(n)$
            ///
            /// where $T$ is time, $M$ is additional memory, and $n$ is `self.dimension()`.
            ///
            /// # Panics
            /// Panics if `rm` is `Exact` and some element is not divisible by $2^k$.
            ///
            /// # Examples
            /// See [here](super::entrywise_shr_round#entrywise_shr_round).
            fn entrywise_shr_round(self, bits: $t, rm: RoundingMode) -> UnsignedVector<T> {
                UnsignedVector {
                    elements: self
                        .elements
                        .iter()
                        .map(|&x| x.shr_round(bits, rm).0)
                        .collect(),
                }
            }
        }

        impl<T: PrimitiveUnsigned> EntrywiseShrRoundAssign<$t> for UnsignedVector<T> {
            /// Right-shifts an [`UnsignedVector`] (divides it by a power of 2), in place, and
            /// rounds every element according to the specified rounding mode.
            ///
            /// Each element is rounded as by
            /// [`ShrRound`](crate::num::arithmetic::traits::ShrRound), but no
            /// [`Ordering`](core::cmp::Ordering) is returned, since with `Nearest` different
            /// elements may be rounded in different directions. Passing `Floor` or `Down` is
            /// equivalent to using `>>`. If `bits` is at least `T::WIDTH`, every element rounds to
            /// 0 or 1.
            ///
            /// $v_i \gets \operatorname{round}(v_i/2^k)$, where $\operatorname{round}$ rounds
            /// according to `rm`.
            ///
            /// # Worst-case complexity
            /// $T(n) = O(n)$
            ///
            /// $M(n) = O(1)$
            ///
            /// where $T$ is time, $M$ is additional memory, and $n$ is `self.dimension()`.
            ///
            /// # Panics
            /// Panics if `rm` is `Exact` and some element is not divisible by $2^k$.
            ///
            /// # Examples
            /// See [here](super::entrywise_shr_round#entrywise_shr_round_assign).
            fn entrywise_shr_round_assign(&mut self, bits: $t, rm: RoundingMode) {
                for x in &mut self.elements {
                    x.shr_round_assign(bits, rm);
                }
            }
        }
    };
}
apply_to_unsigneds!(impl_entrywise_shr_round_unsigned);
