// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::gaussian_integer::GaussianInteger;
use crate::natural::Natural;
use core::cmp::max;
use malachite_base::num::arithmetic::traits::{Height, HeightRef, UnsignedAbs};
use malachite_base::num::logic::traits::SignificantBits;

impl Height for GaussianInteger {
    type Output = Natural;

    /// Returns the height of a [`GaussianInteger`]: the larger of the absolute values of its real
    /// and imaginary parts, taking the [`GaussianInteger`] by reference and cloning.
    ///
    /// $$
    /// f(a + bi) = H(a + bi) = \max(|a|, |b|).
    /// $$
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `self.significant_bits()`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::Height;
    /// use malachite_nz::gaussian_integer::GaussianInteger;
    ///
    /// assert_eq!(GaussianInteger::from_str("3-5i").unwrap().to_height(), 5);
    /// assert_eq!(GaussianInteger::from_str("-7").unwrap().to_height(), 7);
    /// assert_eq!(GaussianInteger::from_str("0").unwrap().to_height(), 0);
    /// ```
    #[inline]
    fn to_height(&self) -> Natural {
        self.height_ref().clone()
    }

    /// Returns the height of a [`GaussianInteger`]: the larger of the absolute values of its real
    /// and imaginary parts, taking the [`GaussianInteger`] by value.
    ///
    /// The larger part is moved out rather than cloned.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(1)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `self.significant_bits()`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::Height;
    /// use malachite_nz::gaussian_integer::GaussianInteger;
    ///
    /// assert_eq!(GaussianInteger::from_str("3-5i").unwrap().into_height(), 5);
    /// assert_eq!(GaussianInteger::from_str("0").unwrap().into_height(), 0);
    /// ```
    #[inline]
    fn into_height(self) -> Natural {
        max(self.real.unsigned_abs(), self.imaginary.unsigned_abs())
    }

    /// Returns the number of significant bits of the height of a [`GaussianInteger`].
    ///
    /// Since bit length is monotone, this is the larger of the two parts' bit lengths, without
    /// materializing the height.
    ///
    /// # Worst-case complexity
    /// Constant time and additional memory.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::Height;
    /// use malachite_nz::gaussian_integer::GaussianInteger;
    ///
    /// assert_eq!(
    ///     GaussianInteger::from_str("3-5i")
    ///         .unwrap()
    ///         .height_significant_bits(),
    ///     3
    /// );
    /// assert_eq!(
    ///     GaussianInteger::from_str("0")
    ///         .unwrap()
    ///         .height_significant_bits(),
    ///     0
    /// );
    /// ```
    #[inline]
    fn height_significant_bits(&self) -> u64 {
        max(
            self.real.significant_bits(),
            self.imaginary.significant_bits(),
        )
    }
}

impl HeightRef for GaussianInteger {
    /// Returns a reference to the height of a [`GaussianInteger`]: the larger of the absolute
    /// values of its real and imaginary parts.
    ///
    /// An [`Integer`](crate::integer::Integer) holds its magnitude as a
    /// [`Natural`](crate::natural::Natural), so the height is already there to be lent and nothing
    /// needs to be built.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(1)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `self.significant_bits()`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::HeightRef;
    /// use malachite_nz::gaussian_integer::GaussianInteger;
    ///
    /// assert_eq!(*GaussianInteger::from_str("3-5i").unwrap().height_ref(), 5);
    /// assert_eq!(*GaussianInteger::from_str("0").unwrap().height_ref(), 0);
    /// ```
    #[inline]
    fn height_ref(&self) -> &Natural {
        max(
            self.real.unsigned_abs_ref(),
            self.imaginary.unsigned_abs_ref(),
        )
    }
}
