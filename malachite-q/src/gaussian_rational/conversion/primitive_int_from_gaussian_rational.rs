// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::Rational;
use crate::gaussian_rational::GaussianRational;
use malachite_base::num::conversion::traits::ConvertibleFrom;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct PrimitiveIntFromGaussianRationalError;

fn try_from_helper<'a, T: TryFrom<&'a Rational>>(
    x: &'a GaussianRational,
) -> Result<T, PrimitiveIntFromGaussianRationalError> {
    if x.imaginary == 0u32 {
        T::try_from(&x.real).map_err(|_| PrimitiveIntFromGaussianRationalError)
    } else {
        Err(PrimitiveIntFromGaussianRationalError)
    }
}

fn convertible_from_helper<T: for<'a> ConvertibleFrom<&'a Rational>>(x: &GaussianRational) -> bool {
    x.imaginary == 0u32 && T::convertible_from(&x.real)
}

macro_rules! impl_primitive_int_from_gaussian_rational {
    ($t:ident) => {
        impl TryFrom<&GaussianRational> for $t {
            type Error = PrimitiveIntFromGaussianRationalError;

            /// Converts a [`GaussianRational`] to a primitive integer, returning an error if the
            /// [`GaussianRational`] is not real or cannot be represented.
            ///
            /// # Worst-case complexity
            /// Constant time and additional memory.
            ///
            /// # Examples
            /// See [here](super::primitive_int_from_gaussian_rational#try_from).
            #[inline]
            fn try_from(x: &GaussianRational) -> Result<$t, Self::Error> {
                try_from_helper(x)
            }
        }

        impl ConvertibleFrom<&GaussianRational> for $t {
            /// Determines whether a [`GaussianRational`] can be converted to a primitive integer
            /// (that is, whether it is real and representable), taking the [`GaussianRational`] by
            /// reference.
            ///
            /// # Worst-case complexity
            /// Constant time and additional memory.
            ///
            /// # Examples
            /// See [here](super::primitive_int_from_gaussian_rational#convertible_from).
            #[inline]
            fn convertible_from(x: &GaussianRational) -> bool {
                convertible_from_helper::<$t>(x)
            }
        }
    };
}
apply_to_primitive_ints!(impl_primitive_int_from_gaussian_rational);
