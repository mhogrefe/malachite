// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::num::arithmetic::traits::{AbsAssign, CanonicalizeUnit, CanonicalizeUnitAssign};

macro_rules! impl_canonicalize_unit_unsigned {
    ($t:ident) => {
        impl CanonicalizeUnit for $t {
            type Output = $t;

            /// Brings a number into canonical unit form. An unsigned number is already in canonical
            /// form, so this is the identity.
            ///
            /// # Worst-case complexity
            /// Constant time and additional memory.
            ///
            /// # Examples
            /// See [here](super::canonicalize_unit#canonicalize_unit).
            #[inline]
            fn canonicalize_unit(self) -> $t {
                self
            }
        }

        impl CanonicalizeUnitAssign for $t {
            /// Replaces a number with its canonical unit form. An unsigned number is already in
            /// canonical form, so this does nothing.
            ///
            /// # Worst-case complexity
            /// Constant time and additional memory.
            ///
            /// # Examples
            /// See [here](super::canonicalize_unit#canonicalize_unit_assign).
            #[inline]
            fn canonicalize_unit_assign(&mut self) {}
        }
    };
}
apply_to_unsigneds!(impl_canonicalize_unit_unsigned);

macro_rules! impl_canonicalize_unit_abs {
    ($t:ident) => {
        impl CanonicalizeUnit for $t {
            type Output = $t;

            /// Brings a number into canonical unit form. The canonical form of a real number is its
            /// absolute value.
            ///
            /// # Worst-case complexity
            /// Constant time and additional memory.
            ///
            /// # Examples
            /// See [here](super::canonicalize_unit#canonicalize_unit).
            #[inline]
            fn canonicalize_unit(self) -> $t {
                self.abs()
            }
        }

        impl CanonicalizeUnitAssign for $t {
            /// Replaces a number with its canonical unit form, its absolute value.
            ///
            /// # Worst-case complexity
            /// Constant time and additional memory.
            ///
            /// # Examples
            /// See [here](super::canonicalize_unit#canonicalize_unit_assign).
            #[inline]
            fn canonicalize_unit_assign(&mut self) {
                self.abs_assign();
            }
        }
    };
}
apply_to_signeds!(impl_canonicalize_unit_abs);

macro_rules! impl_canonicalize_unit_primitive_float {
    ($t:ident) => {
        impl CanonicalizeUnit for $t {
            type Output = $t;

            /// Brings a number into canonical unit form. A finite nonzero float is a unit, since it
            /// has a multiplicative inverse, so its canonical form is 1. The other values are not
            /// units, and their canonical form is their absolute value: both zeros become $0.0$,
            /// both infinities become $\infty$, and NaN stays NaN.
            ///
            /// # Worst-case complexity
            /// Constant time and additional memory.
            ///
            /// # Examples
            /// See [here](super::canonicalize_unit#canonicalize_unit).
            #[inline]
            fn canonicalize_unit(self) -> $t {
                if self.is_finite() && self != 0.0 {
                    1.0
                } else {
                    self.abs()
                }
            }
        }

        impl CanonicalizeUnitAssign for $t {
            /// Replaces a number with its canonical unit form: 1 for a finite nonzero float, and
            /// the absolute value otherwise.
            ///
            /// # Worst-case complexity
            /// Constant time and additional memory.
            ///
            /// # Examples
            /// See [here](super::canonicalize_unit#canonicalize_unit_assign).
            #[inline]
            fn canonicalize_unit_assign(&mut self) {
                *self = self.canonicalize_unit();
            }
        }
    };
}
apply_to_primitive_floats!(impl_canonicalize_unit_primitive_float);
