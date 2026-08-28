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
pub struct GaussianIntegerFromPrimitiveFloatError;

macro_rules! float_impls {
    ($f: ident) => {
        impl TryFrom<$f> for GaussianInteger {
            type Error = GaussianIntegerFromPrimitiveFloatError;

            /// Converts a primitive float to a [`GaussianInteger`], producing a purely real value.
            ///
            /// If the input isn't exactly equal to some [`Integer`], an error is returned.
            ///
            /// # Worst-case complexity
            /// $T(n) = O(n)$
            ///
            /// $M(n) = O(n)$
            ///
            /// where $T$ is time, $M$ is additional memory, and $n$ is `value.sci_exponent()`.
            ///
            /// # Examples
            /// See [here](super::from_primitive_float#try_from).
            fn try_from(value: $f) -> Result<GaussianInteger, Self::Error> {
                Integer::try_from(value)
                    .map(GaussianInteger::from)
                    .map_err(|_| GaussianIntegerFromPrimitiveFloatError)
            }
        }

        impl ConvertibleFrom<$f> for GaussianInteger {
            /// Determines whether a primitive float can be converted to a [`GaussianInteger`] (that
            /// is, whether it is finite and an integer).
            ///
            /// # Worst-case complexity
            /// Constant time and additional memory.
            ///
            /// # Examples
            /// See [here](super::from_primitive_float#convertible_from).
            #[inline]
            fn convertible_from(value: $f) -> bool {
                Integer::convertible_from(value)
            }
        }
    };
}
apply_to_primitive_floats!(float_impls);
