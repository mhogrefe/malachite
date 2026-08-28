// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::Rational;
use crate::gaussian_rational::GaussianRational;
use malachite_base::num::basic::traits::Zero;
use malachite_nz::gaussian_integer::GaussianInteger;
use malachite_nz::integer::Integer;
use malachite_nz::natural::Natural;

// These are enumerated rather than blanket impls (`where Rational: From<T>`) because a blanket
// would conflict, under the coherence rules, with the componentwise `From<GaussianInteger>` impls
// below.
macro_rules! impl_from_purely_real {
    ($($t: ty),*) => {
        $(
            impl From<$t> for GaussianRational {
                /// Converts a value to a purely real [`GaussianRational`].
                ///
                /// # Worst-case complexity
                /// Same as the complexity of the corresponding [`Rational`] conversion.
                ///
                /// # Examples
                /// See [here](super::from#from).
                fn from(x: $t) -> Self {
                    Self {
                        real: Rational::from(x),
                        imaginary: Rational::ZERO,
                    }
                }
            }
        )*
    };
}
impl_from_purely_real!(
    u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, isize, bool, Natural, &Natural,
    Integer, &Integer, Rational
);

impl From<GaussianInteger> for GaussianRational {
    /// Converts a [`GaussianInteger`] to a [`GaussianRational`], componentwise, taking the
    /// [`GaussianInteger`] by value.
    ///
    /// # Worst-case complexity
    /// Constant time and additional memory.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_nz::gaussian_integer::GaussianInteger;
    /// use malachite_q::gaussian_rational::GaussianRational;
    ///
    /// let g = GaussianInteger::from_str("2-3i").unwrap();
    /// assert_eq!(GaussianRational::from(g).to_string(), "2-3i");
    /// ```
    fn from(x: GaussianInteger) -> Self {
        Self {
            real: Rational::from(x.real),
            imaginary: Rational::from(x.imaginary),
        }
    }
}

impl From<&GaussianInteger> for GaussianRational {
    /// Converts a [`GaussianInteger`] to a [`GaussianRational`], componentwise, taking the
    /// [`GaussianInteger`] by reference.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is the maximum number of significant
    /// bits of the parts of `x`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_nz::gaussian_integer::GaussianInteger;
    /// use malachite_q::gaussian_rational::GaussianRational;
    ///
    /// let g = GaussianInteger::from_str("2-3i").unwrap();
    /// assert_eq!(GaussianRational::from(&g).to_string(), "2-3i");
    /// ```
    fn from(x: &GaussianInteger) -> Self {
        Self {
            real: Rational::from(&x.real),
            imaginary: Rational::from(&x.imaginary),
        }
    }
}
