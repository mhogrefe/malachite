// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::gaussian_integer::GaussianInteger;
use crate::integer::Integer;
use malachite_base::num::conversion::traits::ConvertibleFrom;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct PrimitiveIntFromGaussianIntegerError;

fn try_from_helper<'a, T: TryFrom<&'a Integer>>(
    x: &'a GaussianInteger,
) -> Result<T, PrimitiveIntFromGaussianIntegerError> {
    if x.imaginary == 0u32 {
        T::try_from(&x.real).map_err(|_| PrimitiveIntFromGaussianIntegerError)
    } else {
        Err(PrimitiveIntFromGaussianIntegerError)
    }
}

fn convertible_from_helper<T: for<'a> ConvertibleFrom<&'a Integer>>(x: &GaussianInteger) -> bool {
    x.imaginary == 0u32 && T::convertible_from(&x.real)
}

macro_rules! impl_primitive_int_from_gaussian_integer {
    ($t:ident) => {
        impl TryFrom<&GaussianInteger> for $t {
            type Error = PrimitiveIntFromGaussianIntegerError;

            /// Converts a [`GaussianInteger`] to a primitive integer, returning an error if the
            /// [`GaussianInteger`] is not real or cannot be represented.
            ///
            /// # Worst-case complexity
            /// Constant time and additional memory.
            ///
            /// # Examples
            /// See [here](super::primitive_int_from_gaussian_integer#try_from).
            #[inline]
            fn try_from(x: &GaussianInteger) -> Result<$t, Self::Error> {
                try_from_helper(x)
            }
        }

        impl ConvertibleFrom<&GaussianInteger> for $t {
            /// Determines whether a [`GaussianInteger`] can be converted to a primitive integer
            /// (that is, whether it is real and representable), taking the [`GaussianInteger`] by
            /// reference.
            ///
            /// # Worst-case complexity
            /// Constant time and additional memory.
            ///
            /// # Examples
            /// See [here](super::primitive_int_from_gaussian_integer#convertible_from).
            #[inline]
            fn convertible_from(x: &GaussianInteger) -> bool {
                convertible_from_helper::<$t>(x)
            }
        }
    };
}
apply_to_primitive_ints!(impl_primitive_int_from_gaussian_integer);
