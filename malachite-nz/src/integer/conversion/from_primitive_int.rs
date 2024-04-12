// Copyright © 2024 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::integer::Integer;
use crate::natural::Natural;
use crate::platform::{Limb, SignedLimb};

impl Integer {
    /// Converts a [`Limb`](crate#limbs) to an [`Integer`].
    ///
    /// This function is const, so it may be used to define constants.
    ///
    /// # Worst-case complexity
    /// Constant time and additional memory.
    ///
    /// # Examples
    /// ```
    /// use malachite_nz::integer::Integer;
    ///
    /// const TEN: Integer = Integer::const_from_unsigned(10);
    /// assert_eq!(TEN, 10);
    /// ```
    pub const fn const_from_unsigned(x: Limb) -> Integer {
        Integer {
            sign: true,
            abs: Natural::const_from(x),
        }
    }

    /// Converts a [`SignedLimb`](crate#limbs) to an [`Integer`].
    ///
    /// This function is const, so it may be used to define constants.
    ///
    /// # Worst-case complexity
    /// Constant time and additional memory.
    ///
    /// # Examples
    /// ```
    /// use malachite_nz::integer::Integer;
    ///
    /// const TEN: Integer = Integer::const_from_signed(10);
    /// assert_eq!(TEN, 10);
    ///
    /// const NEGATIVE_TEN: Integer = Integer::const_from_signed(-10);
    /// assert_eq!(NEGATIVE_TEN, -10);
    /// ```
    pub const fn const_from_signed(x: SignedLimb) -> Integer {
        Integer {
            sign: x >= 0,
            abs: Natural::const_from(x.unsigned_abs()),
        }
    }
}

macro_rules! impl_from_unsigned {
    ($t: ident) => {
        impl From<$t> for Integer {
            /// Converts an unsigned primitive integer to an [`Integer`].
            ///
            /// # Worst-case complexity
            /// Constant time and additional memory.
            ///
            /// # Examples
            /// See [here](super::from_primitive_int#from).
            #[inline]
            fn from(u: $t) -> Integer {
                Integer {
                    sign: true,
                    abs: Natural::from(u),
                }
            }
        }
    };
}
apply_to_unsigneds!(impl_from_unsigned);

macro_rules! impl_from_signed {
    ($t: ident) => {
        impl From<$t> for Integer {
            /// Converts a signed primitive integer to an [`Integer`].
            ///
            /// # Worst-case complexity
            /// Constant time and additional memory.
            ///
            /// # Examples
            /// See [here](super::from_primitive_int#from).
            #[inline]
            fn from(i: $t) -> Integer {
                Integer {
                    sign: i >= 0,
                    abs: Natural::from(i.unsigned_abs()),
                }
            }
        }
    };
}
apply_to_signeds!(impl_from_signed);
