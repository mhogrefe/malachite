// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::big_exponent_float::BigExponentFloat;
use malachite_base::num::comparison::traits::EqAbs;
use malachite_nz::natural::Natural;

macro_rules! impl_unsigned {
    ($t: ident) => {
        impl EqAbs<$t> for BigExponentFloat {
            /// Determines whether the absolute values of a [`BigExponentFloat`] and an unsigned
            /// primitive integer are equal.
            ///
            /// $\infty$, $-\infty$, and NaN are not equal to any primitive integer in absolute
            /// value. The [`BigExponentFloat`] zero is equal to the integer zero.
            ///
            /// # Worst-case complexity
            /// $T(n) = O(n)$
            ///
            /// $M(n) = O(1)$
            ///
            /// where $T$ is time, $M$ is additional memory, and $n$ is `self.complexity()`.
            ///
            /// # Examples
            /// See [here](super::eq_abs_primitive_int#eq_abs).
            #[inline]
            fn eq_abs(&self, other: &$t) -> bool {
                self.eq_abs(&Natural::from(*other))
            }
        }

        impl EqAbs<BigExponentFloat> for $t {
            /// Determines whether the absolute values of an unsigned primitive integer and a
            /// [`BigExponentFloat`] are equal.
            ///
            /// No primitive integer is equal to $\infty$, $-\infty$, or NaN in absolute value. The
            /// integer zero is equal to the [`BigExponentFloat`] zero.
            ///
            /// # Worst-case complexity
            /// $T(n) = O(n)$
            ///
            /// $M(n) = O(1)$
            ///
            /// where $T$ is time, $M$ is additional memory, and $n$ is `other.complexity()`.
            ///
            /// # Examples
            /// See [here](super::eq_abs_primitive_int#eq_abs).
            #[inline]
            fn eq_abs(&self, other: &BigExponentFloat) -> bool {
                other.eq_abs(self)
            }
        }
    };
}
apply_to_unsigneds!(impl_unsigned);

macro_rules! impl_signed {
    ($t: ident) => {
        impl EqAbs<$t> for BigExponentFloat {
            /// Determines whether the absolute values of a [`BigExponentFloat`] and a signed
            /// primitive integer are equal.
            ///
            /// $\infty$, $-\infty$, and NaN are not equal to any primitive integer in absolute
            /// value. The [`BigExponentFloat`] zero is equal to the integer zero.
            ///
            /// # Worst-case complexity
            /// $T(n) = O(n)$
            ///
            /// $M(n) = O(1)$
            ///
            /// where $T$ is time, $M$ is additional memory, and $n$ is `self.complexity()`.
            ///
            /// # Examples
            /// See [here](super::eq_abs_primitive_int#eq_abs).
            #[inline]
            fn eq_abs(&self, other: &$t) -> bool {
                self.eq_abs(&Natural::from(other.unsigned_abs()))
            }
        }

        impl EqAbs<BigExponentFloat> for $t {
            /// Determines whether the absolute values of a signed primitive integer and a
            /// [`BigExponentFloat`] are equal.
            ///
            /// No primitive integer is equal to $\infty$, $-\infty$, or NaN in absolute value. The
            /// integer zero is equal to the [`BigExponentFloat`] zero.
            ///
            /// # Worst-case complexity
            /// $T(n) = O(n)$
            ///
            /// $M(n) = O(1)$
            ///
            /// where $T$ is time, $M$ is additional memory, and $n$ is `other.complexity()`.
            ///
            /// # Examples
            /// See [here](super::eq_abs_primitive_int#eq_abs).
            #[inline]
            fn eq_abs(&self, other: &BigExponentFloat) -> bool {
                other.eq_abs(self)
            }
        }
    };
}
apply_to_signeds!(impl_signed);
