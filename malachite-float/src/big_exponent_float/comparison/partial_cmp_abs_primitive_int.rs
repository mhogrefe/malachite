// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::big_exponent_float::BigExponentFloat;
use core::cmp::Ordering;
use malachite_base::num::comparison::traits::PartialOrdAbs;
use malachite_nz::natural::Natural;

macro_rules! impl_unsigned {
    ($t: ident) => {
        impl PartialOrdAbs<$t> for BigExponentFloat {
            /// Compares the absolute values of a [`BigExponentFloat`] and an unsigned primitive
            /// integer.
            ///
            /// NaN is not comparable to any primitive integer. $\infty$ and $-\infty$ are greater
            /// than any primitive integer in absolute value. The [`BigExponentFloat`] zero is equal
            /// to the integer zero.
            ///
            /// # Worst-case complexity
            /// $T(n) = O(n)$
            ///
            /// $M(n) = O(1)$
            ///
            /// where $T$ is time, $M$ is additional memory, and $n$ is `self.complexity()`.
            ///
            /// # Examples
            /// See [here](super::partial_cmp_abs_primitive_int#partial_cmp_abs).
            #[inline]
            fn partial_cmp_abs(&self, other: &$t) -> Option<Ordering> {
                self.partial_cmp_abs(&Natural::from(*other))
            }
        }

        impl PartialOrdAbs<BigExponentFloat> for $t {
            /// Compares the absolute values of an unsigned primitive integer and a
            /// [`BigExponentFloat`].
            ///
            /// No integer is comparable to NaN. Every integer is smaller than $\infty$ and
            /// $-\infty$ in absolute value. The integer zero is equal to the [`BigExponentFloat`]
            /// zero.
            ///
            /// # Worst-case complexity
            /// $T(n) = O(n)$
            ///
            /// $M(n) = O(1)$
            ///
            /// where $T$ is time, $M$ is additional memory, and $n$ is `other.complexity()`.
            ///
            /// # Examples
            /// See [here](super::partial_cmp_abs_primitive_int#partial_cmp_abs).
            #[inline]
            fn partial_cmp_abs(&self, other: &BigExponentFloat) -> Option<Ordering> {
                other.partial_cmp_abs(self).map(Ordering::reverse)
            }
        }
    };
}
apply_to_unsigneds!(impl_unsigned);

macro_rules! impl_signed {
    ($t: ident) => {
        impl PartialOrdAbs<$t> for BigExponentFloat {
            /// Compares the absolute values of a [`BigExponentFloat`] and a signed primitive
            /// integer.
            ///
            /// NaN is not comparable to any primitive integer. $\infty$ and $-\infty$ are greater
            /// than any primitive integer in absolute value. The [`BigExponentFloat`] zero is equal
            /// to the integer zero.
            ///
            /// # Worst-case complexity
            /// $T(n) = O(n)$
            ///
            /// $M(n) = O(1)$
            ///
            /// where $T$ is time, $M$ is additional memory, and $n$ is `self.complexity()`.
            ///
            /// # Examples
            /// See [here](super::partial_cmp_abs_primitive_int#partial_cmp_abs).
            #[inline]
            fn partial_cmp_abs(&self, other: &$t) -> Option<Ordering> {
                self.partial_cmp_abs(&Natural::from(other.unsigned_abs()))
            }
        }

        impl PartialOrdAbs<BigExponentFloat> for $t {
            /// Compares the absolute values of a signed primitive integer and a
            /// [`BigExponentFloat`].
            ///
            /// No integer is comparable to NaN. Every integer is smaller than $\infty$ and
            /// $-\infty$ in absolute value. The integer zero is equal to the [`BigExponentFloat`]
            /// zero.
            ///
            /// # Worst-case complexity
            /// $T(n) = O(n)$
            ///
            /// $M(n) = O(1)$
            ///
            /// where $T$ is time, $M$ is additional memory, and $n$ is `other.complexity()`.
            ///
            /// # Examples
            /// See [here](super::partial_cmp_abs_primitive_int#partial_cmp_abs).
            #[inline]
            fn partial_cmp_abs(&self, other: &BigExponentFloat) -> Option<Ordering> {
                other.partial_cmp_abs(self).map(Ordering::reverse)
            }
        }
    };
}
apply_to_signeds!(impl_signed);
