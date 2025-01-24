// Copyright © 2025 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::Rational;
use malachite_base::num::basic::traits::One;
use malachite_nz::natural::Natural;
use malachite_nz::platform::{Limb, SignedLimb};

impl Rational {
    /// Converts a [`Limb`](crate#limbs) to a [`Rational`].
    ///
    /// This function is const, so it may be used to define constants.
    ///
    /// # Worst-case complexity
    /// Constant time and additional memory.
    ///
    /// # Examples
    /// ```
    /// use malachite_q::Rational;
    ///
    /// const TEN: Rational = Rational::const_from_unsigned(10);
    /// assert_eq!(TEN, 10);
    /// ```
    pub const fn const_from_unsigned(x: Limb) -> Rational {
        Rational {
            sign: true,
            numerator: Natural::const_from(x),
            denominator: Natural::ONE,
        }
    }

    /// Converts a [`SignedLimb`](crate#limbs) to a [`Rational`].
    ///
    /// This function is const, so it may be used to define constants.
    ///
    /// # Worst-case complexity
    /// Constant time and additional memory.
    ///
    /// # Examples
    /// ```
    /// use malachite_q::Rational;
    ///
    /// const TEN: Rational = Rational::const_from_signed(10);
    /// assert_eq!(TEN, 10);
    ///
    /// const NEGATIVE_TEN: Rational = Rational::const_from_signed(-10);
    /// assert_eq!(NEGATIVE_TEN, -10);
    /// ```
    pub const fn const_from_signed(x: SignedLimb) -> Rational {
        Rational {
            sign: x >= 0,
            numerator: Natural::const_from(x.unsigned_abs()),
            denominator: Natural::ONE,
        }
    }
}

macro_rules! impl_from_unsigned {
    ($t: ident) => {
        impl From<$t> for Rational {
            /// Converts an unsigned primitive integer to a [`Rational`].
            ///
            /// # Worst-case complexity
            /// Constant time and additional memory.
            ///
            /// # Examples
            /// See [here](super::from_primitive_int#from).
            #[inline]
            fn from(u: $t) -> Rational {
                Rational {
                    sign: true,
                    numerator: Natural::from(u),
                    denominator: Natural::ONE,
                }
            }
        }
    };
}
apply_to_unsigneds!(impl_from_unsigned);

macro_rules! impl_from_signed {
    ($t: ident) => {
        impl From<$t> for Rational {
            /// Converts a signed primitive integer to a [`Rational`].
            ///
            /// # Worst-case complexity
            /// Constant time and additional memory.
            ///
            /// # Examples
            /// See [here](super::from_primitive_int#from).
            #[inline]
            fn from(i: $t) -> Rational {
                Rational {
                    sign: i >= 0,
                    numerator: Natural::from(i.unsigned_abs()),
                    denominator: Natural::ONE,
                }
            }
        }
    };
}
apply_to_signeds!(impl_from_signed);
