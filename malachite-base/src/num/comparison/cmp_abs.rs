// Copyright © 2024 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::num::arithmetic::traits::UnsignedAbs;
use crate::num::basic::floats::PrimitiveFloat;
use crate::num::comparison::traits::{OrdAbs, PartialOrdAbs};
use core::cmp::Ordering;

macro_rules! impl_partial_ord_abs {
    ($t:ident) => {
        impl PartialOrdAbs<$t> for $t {
            /// Compares the absolute values of two numbers, taking both by reference.
            ///
            /// The [`PartialOrdAbs`] interface allows for pairs of incomparable elements, but for
            /// primitive integers these never occur.
            ///
            /// # Worst-case complexity
            /// Constant time and additional memory.
            ///
            /// # Examples
            /// See [here](super::cmp_abs#partial_cmp_abs).
            #[inline]
            fn partial_cmp_abs(&self, other: &$t) -> Option<Ordering> {
                Some(self.cmp_abs(other))
            }
        }
    };
}
apply_to_primitive_ints!(impl_partial_ord_abs);

macro_rules! impl_ord_abs_unsigned {
    ($t:ident) => {
        impl OrdAbs for $t {
            /// Compares the absolute values of two numbers, taking both by reference.
            ///
            /// For unsigned values, this is the same as ordinary comparison.
            ///
            /// # Worst-case complexity
            /// Constant time and additional memory.
            ///
            /// # Examples
            /// See [here](super::cmp_abs#cmp_abs).
            #[inline]
            fn cmp_abs(&self, other: &Self) -> Ordering {
                self.cmp(other)
            }
        }
    };
}
apply_to_unsigneds!(impl_ord_abs_unsigned);

fn cmp_abs_signed<U: Ord, S: Copy + UnsignedAbs<Output = U>>(x: &S, y: &S) -> Ordering {
    x.unsigned_abs().cmp(&y.unsigned_abs())
}

macro_rules! impl_ord_abs_signed {
    ($t:ident) => {
        impl OrdAbs for $t {
            /// Compares the absolute values of two numbers, taking both by reference.
            ///
            /// # Worst-case complexity
            /// Constant time and additional memory.
            ///
            /// # Examples
            /// See [here](super::cmp_abs#cmp_abs).
            #[inline]
            fn cmp_abs(&self, other: &Self) -> Ordering {
                cmp_abs_signed(self, other)
            }
        }
    };
}
apply_to_signeds!(impl_ord_abs_signed);

fn partial_cmp_abs_primitive_float<T: PrimitiveFloat>(x: &T, y: &T) -> Option<Ordering> {
    x.abs().partial_cmp(&y.abs())
}

macro_rules! impl_ord_abs_primitive_float {
    ($t:ident) => {
        impl PartialOrdAbs for $t {
            /// Compares the absolute values of two numbers, taking both by reference.
            ///
            /// For unsigned values, this is the same as ordinary comparison.
            ///
            /// # Worst-case complexity
            /// Constant time and additional memory.
            ///
            /// # Examples
            /// See [here](super::cmp_abs#cmp_abs).
            #[inline]
            fn partial_cmp_abs(&self, other: &Self) -> Option<Ordering> {
                partial_cmp_abs_primitive_float(self, other)
            }
        }
    };
}
apply_to_primitive_floats!(impl_ord_abs_primitive_float);
