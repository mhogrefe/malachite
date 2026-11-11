// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::big_exponent_float::BigExponentFloat;
use core::cmp::Ordering;
use malachite_nz::integer::Integer;
use malachite_nz::natural::Natural;

macro_rules! impl_unsigned {
    ($t: ident) => {
        impl PartialOrd<$t> for BigExponentFloat {
            /// Compares a [`BigExponentFloat`] to an unsigned primitive integer.
            ///
            /// NaN is not comparable to any primitive integer. $\infty$ is greater than any
            /// primitive integer, and $-\infty$ is less. The [`BigExponentFloat`] zero is equal to
            /// the integer zero.
            ///
            /// # Worst-case complexity
            /// $T(n) = O(n)$
            ///
            /// $M(n) = O(1)$
            ///
            /// where $T$ is time, $M$ is additional memory, and $n$ is `self.complexity()`.
            ///
            /// # Examples
            /// See [here](super::partial_cmp_primitive_int#partial_cmp).
            #[inline]
            fn partial_cmp(&self, other: &$t) -> Option<Ordering> {
                self.partial_cmp(&Natural::from(*other))
            }
        }

        impl PartialOrd<BigExponentFloat> for $t {
            /// Compares an unsigned primitive integer to a [`BigExponentFloat`].
            ///
            /// No integer is comparable to NaN. Every integer is smaller than $\infty$ and greater
            /// than $-\infty$. The integer zero is equal to the [`BigExponentFloat`] zero.
            ///
            /// # Worst-case complexity
            /// $T(n) = O(n)$
            ///
            /// $M(n) = O(1)$
            ///
            /// where $T$ is time, $M$ is additional memory, and $n$ is `other.complexity()`.
            ///
            /// # Examples
            /// See [here](super::partial_cmp_primitive_int#partial_cmp).
            #[inline]
            fn partial_cmp(&self, other: &BigExponentFloat) -> Option<Ordering> {
                other.partial_cmp(self).map(Ordering::reverse)
            }
        }
    };
}
apply_to_unsigneds!(impl_unsigned);

macro_rules! impl_signed {
    ($t: ident) => {
        impl PartialOrd<$t> for BigExponentFloat {
            /// Compares a [`BigExponentFloat`] to a signed primitive integer.
            ///
            /// NaN is not comparable to any primitive integer. $\infty$ is greater than any
            /// primitive integer, and $-\infty$ is less. The [`BigExponentFloat`] zero is equal to
            /// the integer zero.
            ///
            /// # Worst-case complexity
            /// $T(n) = O(n)$
            ///
            /// $M(n) = O(1)$
            ///
            /// where $T$ is time, $M$ is additional memory, and $n$ is `self.complexity()`.
            ///
            /// # Examples
            /// See [here](super::partial_cmp_primitive_int#partial_cmp).
            #[inline]
            fn partial_cmp(&self, other: &$t) -> Option<Ordering> {
                self.partial_cmp(&Integer::from(*other))
            }
        }

        impl PartialOrd<BigExponentFloat> for $t {
            /// Compares a signed primitive integer to a [`BigExponentFloat`].
            ///
            /// No integer is comparable to NaN. Every integer is smaller than $\infty$ and greater
            /// than $-\infty$. The integer zero is equal to the [`BigExponentFloat`] zero.
            ///
            /// # Worst-case complexity
            /// $T(n) = O(n)$
            ///
            /// $M(n) = O(1)$
            ///
            /// where $T$ is time, $M$ is additional memory, and $n$ is `other.complexity()`.
            ///
            /// # Examples
            /// See [here](super::partial_cmp_primitive_int#partial_cmp).
            #[inline]
            fn partial_cmp(&self, other: &BigExponentFloat) -> Option<Ordering> {
                other.partial_cmp(self).map(Ordering::reverse)
            }
        }
    };
}
apply_to_signeds!(impl_signed);
