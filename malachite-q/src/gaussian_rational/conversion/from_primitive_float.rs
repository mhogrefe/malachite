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
pub struct GaussianRationalFromPrimitiveFloatError;

macro_rules! float_impls {
    ($f: ident) => {
        impl TryFrom<$f> for GaussianRational {
            type Error = GaussianRationalFromPrimitiveFloatError;

            /// Converts a primitive float to a [`GaussianRational`], producing a purely real value.
            ///
            /// If the input is NaN or infinite, an error is returned.
            ///
            /// # Worst-case complexity
            /// $T(n) = O(n)$
            ///
            /// $M(n) = O(n)$
            ///
            /// where $T$ is time, $M$ is additional memory, and $n$ is
            /// `value.sci_exponent().abs()`.
            ///
            /// # Examples
            /// See [here](super::from_primitive_float#try_from).
            fn try_from(value: $f) -> Result<GaussianRational, Self::Error> {
                Rational::try_from(value)
                    .map(GaussianRational::from)
                    .map_err(|_| GaussianRationalFromPrimitiveFloatError)
            }
        }

        impl ConvertibleFrom<$f> for GaussianRational {
            /// Determines whether a primitive float can be converted to a [`GaussianRational`]
            /// (that is, whether it is finite).
            ///
            /// # Worst-case complexity
            /// Constant time and additional memory.
            ///
            /// # Examples
            /// See [here](super::from_primitive_float#convertible_from).
            #[inline]
            fn convertible_from(value: $f) -> bool {
                Rational::convertible_from(value)
            }
        }
    };
}
apply_to_primitive_floats!(float_impls);
