// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::gaussian_integer::GaussianInteger;
use crate::natural::Natural;
use core::cmp::Ordering::{self, Greater, Less};
use malachite_base::num::arithmetic::traits::{AbsSquared, UnsignedAbs};
use malachite_base::num::comparison::traits::PartialOrdAbs;
use malachite_base::num::conversion::traits::IntegerMantissaAndExponent;

macro_rules! impl_float {
    ($t: ident) => {
        impl PartialOrdAbs<$t> for GaussianInteger {
            /// Compares the absolute values of a [`GaussianInteger`] and a primitive float.
            ///
            /// NaN is not comparable to any [`GaussianInteger`]. $\infty$ and $-\infty$ are greater
            /// in absolute value than any [`GaussianInteger`]. When the squared absolute values
            /// must be compared, the float's square is represented exactly as an odd square times a
            /// power of two, so the comparison is exact.
            ///
            /// # Worst-case complexity
            /// $T(n) = O(n \log n \log\log n)$
            ///
            /// $M(n) = O(n \log n)$
            ///
            /// where $T$ is time, $M$ is additional memory, and $n$ is the maximum number of
            /// significant bits of the real and imaginary parts of `self` and
            /// `other.sci_exponent().abs()`.
            ///
            /// # Examples
            /// See [here](super::cmp_abs_primitive_float#partial_cmp_abs).
            fn partial_cmp_abs(&self, other: &$t) -> Option<Ordering> {
                if other.is_nan() {
                    None
                } else if !other.is_finite() {
                    Some(Less)
                } else if self.imaginary == 0u32 {
                    self.real.partial_cmp_abs(other)
                } else if self.real == 0u32 {
                    self.imaginary.partial_cmp_abs(other)
                } else if !self.real.lt_abs(other) || !self.imaginary.lt_abs(other) {
                    // This also covers a zero float, whose absolute value cannot exceed either
                    // nonzero component.
                    Some(Greater)
                } else {
                    // |other| = m * 2^e with m odd, so |other|^2 = m^2 * 2^(2e), compared exactly
                    // against |self|^2 by shifting whichever side has the nonnegative exponent.
                    let (m, e) = other.abs().integer_mantissa_and_exponent();
                    let m_squared = Natural::from(u128::from(m) * u128::from(m));
                    let abs_squared = self.abs_squared().unsigned_abs();
                    let shift = e.unsigned_abs() << 1;
                    Some(if e >= 0 {
                        abs_squared.cmp(&(m_squared << shift))
                    } else {
                        (abs_squared << shift).cmp(&m_squared)
                    })
                }
            }
        }

        impl PartialOrdAbs<GaussianInteger> for $t {
            /// Compares the absolute values of a primitive float and a [`GaussianInteger`].
            ///
            /// NaN is not comparable to any [`GaussianInteger`]. $\infty$ and $-\infty$ are greater
            /// in absolute value than any [`GaussianInteger`].
            ///
            /// # Worst-case complexity
            /// $T(n) = O(n \log n \log\log n)$
            ///
            /// $M(n) = O(n \log n)$
            ///
            /// where $T$ is time, $M$ is additional memory, and $n$ is the maximum number of
            /// significant bits of the real and imaginary parts of `other` and
            /// `self.sci_exponent().abs()`.
            ///
            /// # Examples
            /// See [here](super::cmp_abs_primitive_float#partial_cmp_abs).
            #[inline]
            fn partial_cmp_abs(&self, other: &GaussianInteger) -> Option<Ordering> {
                other.partial_cmp_abs(self).map(Ordering::reverse)
            }
        }
    };
}
apply_to_primitive_floats!(impl_float);
