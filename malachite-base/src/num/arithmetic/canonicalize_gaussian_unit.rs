// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::num::arithmetic::traits::{
    AbsAssign, CanonicalizeGaussianUnit, CanonicalizeGaussianUnitAssign,
};

macro_rules! impl_canonicalize_gaussian_unit_unsigned {
    ($t:ident) => {
        impl CanonicalizeGaussianUnit for $t {
            type Output = $t;

            /// Brings a number into canonical Gaussian-unit form. An unsigned number is already in
            /// that form, so this is the identity.
            ///
            /// # Worst-case complexity
            /// Constant time and additional memory.
            ///
            /// # Examples
            /// See [here](super::canonicalize_gaussian_unit#canonicalize_gaussian_unit).
            #[inline]
            fn canonicalize_gaussian_unit(self) -> $t {
                self
            }
        }

        impl CanonicalizeGaussianUnitAssign for $t {
            /// Replaces a number with its canonical Gaussian-unit form. An unsigned number is
            /// already in that form, so this does nothing.
            ///
            /// # Worst-case complexity
            /// Constant time and additional memory.
            ///
            /// # Examples
            /// See [here](super::canonicalize_gaussian_unit#canonicalize_gaussian_unit_assign).
            #[inline]
            fn canonicalize_gaussian_unit_assign(&mut self) {}
        }
    };
}
apply_to_unsigneds!(impl_canonicalize_gaussian_unit_unsigned);

macro_rules! impl_canonicalize_gaussian_unit_abs {
    ($t:ident) => {
        impl CanonicalizeGaussianUnit for $t {
            type Output = $t;

            /// Brings a number into canonical Gaussian-unit form. The canonical Gaussian-unit form
            /// of a real number is its absolute value.
            ///
            /// # Worst-case complexity
            /// Constant time and additional memory.
            ///
            /// # Examples
            /// See [here](super::canonicalize_gaussian_unit#canonicalize_gaussian_unit).
            #[inline]
            fn canonicalize_gaussian_unit(self) -> $t {
                self.abs()
            }
        }

        impl CanonicalizeGaussianUnitAssign for $t {
            /// Replaces a number with its canonical Gaussian-unit form: its absolute value.
            ///
            /// # Worst-case complexity
            /// Constant time and additional memory.
            ///
            /// # Examples
            /// See [here](super::canonicalize_gaussian_unit#canonicalize_gaussian_unit_assign).
            #[inline]
            fn canonicalize_gaussian_unit_assign(&mut self) {
                self.abs_assign();
            }
        }
    };
}
apply_to_signeds!(impl_canonicalize_gaussian_unit_abs);
apply_to_primitive_floats!(impl_canonicalize_gaussian_unit_abs);
