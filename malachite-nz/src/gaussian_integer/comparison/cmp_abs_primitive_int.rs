// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::gaussian_integer::GaussianInteger;
use crate::integer::Integer;
use core::cmp::Ordering::{self, Greater};
use malachite_base::num::arithmetic::traits::AbsSquared;
use malachite_base::num::comparison::traits::PartialOrdAbs;

macro_rules! impl_unsigned {
    ($t: ident) => {
        impl PartialOrdAbs<$t> for GaussianInteger {
            /// Compares the absolute values of a [`GaussianInteger`] and an unsigned primitive
            /// integer.
            ///
            /// # Worst-case complexity
            /// $T(n) = O(n \log n \log\log n)$
            ///
            /// $M(n) = O(n \log n)$
            ///
            /// where $T$ is time, $M$ is additional memory, and $n$ is the maximum number of
            /// significant bits of the real and imaginary parts of `self`.
            ///
            /// # Examples
            /// See [here](super::cmp_abs_primitive_int#partial_cmp_abs).
            fn partial_cmp_abs(&self, other: &$t) -> Option<Ordering> {
                if self.imaginary == 0u32 {
                    self.real.partial_cmp_abs(other)
                } else if self.real == 0u32 {
                    self.imaginary.partial_cmp_abs(other)
                } else if !self.real.lt_abs(other) || !self.imaginary.lt_abs(other) {
                    Some(Greater)
                } else {
                    Some(self.abs_squared().cmp(&Integer::from(*other).abs_squared()))
                }
            }
        }

        impl PartialOrdAbs<GaussianInteger> for $t {
            /// Compares the absolute values of an unsigned primitive integer and a
            /// [`GaussianInteger`].
            ///
            /// # Worst-case complexity
            /// $T(n) = O(n \log n \log\log n)$
            ///
            /// $M(n) = O(n \log n)$
            ///
            /// where $T$ is time, $M$ is additional memory, and $n$ is the maximum number of
            /// significant bits of the real and imaginary parts of `other`.
            ///
            /// # Examples
            /// See [here](super::cmp_abs_primitive_int#partial_cmp_abs).
            #[inline]
            fn partial_cmp_abs(&self, other: &GaussianInteger) -> Option<Ordering> {
                other.partial_cmp_abs(self).map(Ordering::reverse)
            }
        }
    };
}
apply_to_unsigneds!(impl_unsigned);

macro_rules! impl_signed {
    ($t: ident) => {
        impl PartialOrdAbs<$t> for GaussianInteger {
            /// Compares the absolute values of a [`GaussianInteger`] and a signed primitive
            /// integer.
            ///
            /// # Worst-case complexity
            /// $T(n) = O(n \log n \log\log n)$
            ///
            /// $M(n) = O(n \log n)$
            ///
            /// where $T$ is time, $M$ is additional memory, and $n$ is the maximum number of
            /// significant bits of the real and imaginary parts of `self`.
            ///
            /// # Examples
            /// See [here](super::cmp_abs_primitive_int#partial_cmp_abs).
            fn partial_cmp_abs(&self, other: &$t) -> Option<Ordering> {
                if self.imaginary == 0u32 {
                    self.real.partial_cmp_abs(other)
                } else if self.real == 0u32 {
                    self.imaginary.partial_cmp_abs(other)
                } else if !self.real.lt_abs(other) || !self.imaginary.lt_abs(other) {
                    Some(Greater)
                } else {
                    Some(self.abs_squared().cmp(&Integer::from(*other).abs_squared()))
                }
            }
        }

        impl PartialOrdAbs<GaussianInteger> for $t {
            /// Compares the absolute values of a signed primitive integer and a
            /// [`GaussianInteger`].
            ///
            /// # Worst-case complexity
            /// $T(n) = O(n \log n \log\log n)$
            ///
            /// $M(n) = O(n \log n)$
            ///
            /// where $T$ is time, $M$ is additional memory, and $n$ is the maximum number of
            /// significant bits of the real and imaginary parts of `other`.
            ///
            /// # Examples
            /// See [here](super::cmp_abs_primitive_int#partial_cmp_abs).
            #[inline]
            fn partial_cmp_abs(&self, other: &GaussianInteger) -> Option<Ordering> {
                other.partial_cmp_abs(self).map(Ordering::reverse)
            }
        }
    };
}
apply_to_signeds!(impl_signed);
