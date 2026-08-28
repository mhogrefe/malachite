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
pub struct PrimitiveFloatFromGaussianIntegerError;

fn try_from_helper<'a, T: TryFrom<&'a Integer>>(
    x: &'a GaussianInteger,
) -> Result<T, PrimitiveFloatFromGaussianIntegerError> {
    if x.imaginary == 0u32 {
        T::try_from(&x.real).map_err(|_| PrimitiveFloatFromGaussianIntegerError)
    } else {
        Err(PrimitiveFloatFromGaussianIntegerError)
    }
}

fn convertible_from_helper<T: for<'a> ConvertibleFrom<&'a Integer>>(x: &GaussianInteger) -> bool {
    x.imaginary == 0u32 && T::convertible_from(&x.real)
}

macro_rules! float_impls {
    ($f: ident) => {
        impl TryFrom<&GaussianInteger> for $f {
            type Error = PrimitiveFloatFromGaussianIntegerError;

            /// Converts a [`GaussianInteger`] to a primitive float, returning an error if the
            /// [`GaussianInteger`] is not real or isn't exactly equal to some float.
            ///
            /// # Worst-case complexity
            /// $T(n) = O(n)$
            ///
            /// $M(n) = O(1)$
            ///
            /// where $T$ is time, $M$ is additional memory, and $n$ is `x.real.significant_bits()`.
            ///
            /// # Examples
            /// See [here](super::primitive_float_from_gaussian_integer#try_from).
            #[inline]
            fn try_from(x: &GaussianInteger) -> Result<$f, Self::Error> {
                try_from_helper(x)
            }
        }

        impl ConvertibleFrom<&GaussianInteger> for $f {
            /// Determines whether a [`GaussianInteger`] can be converted to a primitive float (that
            /// is, whether it is real and exactly equal to some float), taking the
            /// [`GaussianInteger`] by reference.
            ///
            /// # Worst-case complexity
            /// $T(n) = O(n)$
            ///
            /// $M(n) = O(1)$
            ///
            /// where $T$ is time, $M$ is additional memory, and $n$ is `x.real.significant_bits()`.
            ///
            /// # Examples
            /// See [here](super::primitive_float_from_gaussian_integer#convertible_from).
            #[inline]
            fn convertible_from(x: &GaussianInteger) -> bool {
                convertible_from_helper::<$f>(x)
            }
        }
    };
}
apply_to_primitive_floats!(float_impls);
