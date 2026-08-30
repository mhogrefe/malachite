// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::Rational;
use crate::gaussian_rational::GaussianRational;
use malachite_base::num::arithmetic::traits::AbsSquared;
use malachite_base::num::comparison::traits::{EqAbs, PartialOrdAbs};

macro_rules! impl_unsigned {
    ($t: ident) => {
        impl EqAbs<$t> for GaussianRational {
            /// Determines whether the absolute values of a [`GaussianRational`] and an unsigned
            /// primitive integer are equal.
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
            /// See [here](super::eq_abs_primitive_int#eq_abs).
            fn eq_abs(&self, other: &$t) -> bool {
                if self.imaginary == 0u32 {
                    self.real.eq_abs(other)
                } else if self.real == 0u32 {
                    self.imaginary.eq_abs(other)
                } else {
                    self.real.lt_abs(other)
                        && self.imaginary.lt_abs(other)
                        && self.abs_squared() == Rational::from(*other).abs_squared()
                }
            }
        }

        impl EqAbs<GaussianRational> for $t {
            /// Determines whether the absolute values of an unsigned primitive integer and a
            /// [`GaussianRational`] are equal.
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
            /// See [here](super::eq_abs_primitive_int#eq_abs).
            #[inline]
            fn eq_abs(&self, other: &GaussianRational) -> bool {
                other.eq_abs(self)
            }
        }
    };
}
apply_to_unsigneds!(impl_unsigned);

macro_rules! impl_signed {
    ($t: ident) => {
        impl EqAbs<$t> for GaussianRational {
            /// Determines whether the absolute values of a [`GaussianRational`] and a signed
            /// primitive integer are equal.
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
            /// See [here](super::eq_abs_primitive_int#eq_abs).
            fn eq_abs(&self, other: &$t) -> bool {
                if self.imaginary == 0u32 {
                    self.real.eq_abs(other)
                } else if self.real == 0u32 {
                    self.imaginary.eq_abs(other)
                } else {
                    self.real.lt_abs(other)
                        && self.imaginary.lt_abs(other)
                        && self.abs_squared() == Rational::from(*other).abs_squared()
                }
            }
        }

        impl EqAbs<GaussianRational> for $t {
            /// Determines whether the absolute values of a signed primitive integer and a
            /// [`GaussianRational`] are equal.
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
            /// See [here](super::eq_abs_primitive_int#eq_abs).
            #[inline]
            fn eq_abs(&self, other: &GaussianRational) -> bool {
                other.eq_abs(self)
            }
        }
    };
}
apply_to_signeds!(impl_signed);
