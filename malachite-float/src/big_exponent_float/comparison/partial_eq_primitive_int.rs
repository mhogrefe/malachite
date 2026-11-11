// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::big_exponent_float::BigExponentFloat;
use core::cmp::Ordering::*;

macro_rules! impl_unsigned {
    ($t: ident) => {
        impl PartialEq<$t> for BigExponentFloat {
            /// Determines whether a [`BigExponentFloat`] is equal to an unsigned primitive integer.
            ///
            /// $\infty$, $-\infty$, and NaN are not equal to any primitive integer. The
            /// [`BigExponentFloat`] zero is equal to the integer zero.
            ///
            /// # Worst-case complexity
            /// $T(n) = O(n)$
            ///
            /// $M(n) = O(1)$
            ///
            /// where $T$ is time, $M$ is additional memory, and $n$ is `self.complexity()`.
            ///
            /// # Examples
            /// See [here](super::partial_eq_primitive_int#eq).
            #[inline]
            fn eq(&self, other: &$t) -> bool {
                self.partial_cmp(other) == Some(Equal)
            }
        }

        impl PartialEq<BigExponentFloat> for $t {
            /// Determines whether an unsigned primitive integer is equal to a [`BigExponentFloat`].
            ///
            /// No primitive integer is equal to $\infty$, $-\infty$, or NaN. The integer zero is
            /// equal to the [`BigExponentFloat`] zero.
            ///
            /// # Worst-case complexity
            /// $T(n) = O(n)$
            ///
            /// $M(n) = O(1)$
            ///
            /// where $T$ is time, $M$ is additional memory, and $n$ is `other.complexity()`.
            ///
            /// # Examples
            /// See [here](super::partial_eq_primitive_int#eq).
            #[inline]
            fn eq(&self, other: &BigExponentFloat) -> bool {
                other == self
            }
        }
    };
}
apply_to_unsigneds!(impl_unsigned);

macro_rules! impl_signed {
    ($t: ident) => {
        impl PartialEq<$t> for BigExponentFloat {
            /// Determines whether a [`BigExponentFloat`] is equal to a signed primitive integer.
            ///
            /// $\infty$, $-\infty$, and NaN are not equal to any primitive integer. The
            /// [`BigExponentFloat`] zero is equal to the integer zero.
            ///
            /// # Worst-case complexity
            /// $T(n) = O(n)$
            ///
            /// $M(n) = O(1)$
            ///
            /// where $T$ is time, $M$ is additional memory, and $n$ is `self.complexity()`.
            ///
            /// # Examples
            /// See [here](super::partial_eq_primitive_int#eq).
            #[inline]
            fn eq(&self, other: &$t) -> bool {
                self.partial_cmp(other) == Some(Equal)
            }
        }

        impl PartialEq<BigExponentFloat> for $t {
            /// Determines whether a signed primitive integer is equal to a [`BigExponentFloat`].
            ///
            /// No primitive integer is equal to $\infty$, $-\infty$, or NaN. The integer zero is
            /// equal to the [`BigExponentFloat`] zero.
            ///
            /// # Worst-case complexity
            /// $T(n) = O(n)$
            ///
            /// $M(n) = O(1)$
            ///
            /// where $T$ is time, $M$ is additional memory, and $n$ is `other.complexity()`.
            ///
            /// # Examples
            /// See [here](super::partial_eq_primitive_int#eq).
            #[inline]
            fn eq(&self, other: &BigExponentFloat) -> bool {
                other == self
            }
        }
    };
}
apply_to_signeds!(impl_signed);
