// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::num::comparison::traits::{
    OrdAbsDouble, OrdDouble, PartialOrdAbsDouble, PartialOrdDouble,
};
use core::cmp::Ordering::{self, *};

macro_rules! impl_ord_double_unsigned {
    ($t:ident) => {
        impl OrdDouble for $t {
            /// Compares a number with twice another number.
            ///
            /// The doubling is not actually performed, so it cannot overflow.
            ///
            /// $$
            /// f(x, y) = \operatorname{cmp}(x, 2y).
            /// $$
            ///
            /// # Worst-case complexity
            /// Constant time and additional memory.
            ///
            /// # Examples
            /// See [here](super::cmp_double#cmp_double).
            #[inline]
            fn cmp_double(&self, other: &Self) -> Ordering {
                if other.leading_zeros() == 0 {
                    // doubling would carry past the top bit, so it exceeds anything representable
                    Less
                } else {
                    self.cmp(&(other << 1))
                }
            }
        }

        impl PartialOrdDouble for $t {
            /// Compares a number with twice another number.
            ///
            /// See the documentation for the [`OrdDouble`] implementation.
            #[inline]
            fn partial_cmp_double(&self, other: &Self) -> Option<Ordering> {
                Some(self.cmp_double(other))
            }
        }
    };
}
apply_to_unsigneds!(impl_ord_double_unsigned);

macro_rules! impl_ord_abs_double_signed {
    ($t:ident) => {
        impl OrdAbsDouble for $t {
            /// Compares the absolute value of a number with twice the absolute value of another
            /// number.
            ///
            /// The doubling is not actually performed, so it cannot overflow; in particular the
            /// magnitude of the most negative value, which is not representable, is handled.
            ///
            /// $$
            /// f(x, y) = \operatorname{cmp}(|x|, 2|y|).
            /// $$
            ///
            /// # Worst-case complexity
            /// Constant time and additional memory.
            ///
            /// # Examples
            /// See [here](super::cmp_double#cmp_abs_double).
            #[inline]
            fn cmp_abs_double(&self, other: &Self) -> Ordering {
                self.unsigned_abs().cmp_double(&other.unsigned_abs())
            }
        }

        impl PartialOrdAbsDouble for $t {
            /// Compares the absolute value of a number with twice the absolute value of another
            /// number.
            ///
            /// See the documentation for the [`OrdAbsDouble`] implementation.
            #[inline]
            fn partial_cmp_abs_double(&self, other: &Self) -> Option<Ordering> {
                Some(self.cmp_abs_double(other))
            }
        }
    };
}
apply_to_signeds!(impl_ord_abs_double_signed);
