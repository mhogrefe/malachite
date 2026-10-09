// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::natural::Natural;
use crate::natural_vector::{NaturalVector, ZERO};
use malachite_base::num::arithmetic::traits::{Height, HeightRef};
use malachite_base::num::basic::traits::Zero;
use malachite_base::num::logic::traits::SignificantBits;

impl Height for NaturalVector {
    type Output = Natural;

    /// Returns the height of a [`NaturalVector`]: the largest of its elements, taking the vector by
    /// reference and cloning.
    ///
    /// The 0-dimensional vector has no elements, and its height is 0.
    ///
    /// $$
    /// f(v) = H(v) = \max_i v_i.
    /// $$
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is the total number of bits of the
    /// elements, and $m$ is the number of bits of the height.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::Height;
    /// use malachite_nz::natural_vector::NaturalVector;
    ///
    /// assert_eq!(NaturalVector::from_str("(1, 3, 2)").unwrap().to_height(), 3);
    /// assert_eq!(NaturalVector::from_str("()").unwrap().to_height(), 0);
    /// ```
    ///
    /// This is equivalent to `_fmpz_vec_height` from `fmpz_vec/height.c`, FLINT 3.6.0.
    #[inline]
    fn to_height(&self) -> Natural {
        self.height_ref().clone()
    }

    /// Returns the height of a [`NaturalVector`]: the largest of its elements, taking the vector by
    /// value.
    ///
    /// The element of largest height, or the part of it that is the height, is moved out of the
    /// vector rather than cloned.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(1)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is the total number of bits of the
    /// elements.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::Height;
    /// use malachite_nz::natural_vector::NaturalVector;
    ///
    /// assert_eq!(
    ///     NaturalVector::from_str("(1, 3, 2)").unwrap().into_height(),
    ///     3
    /// );
    /// assert_eq!(NaturalVector::from_str("()").unwrap().into_height(), 0);
    /// ```
    #[inline]
    fn into_height(self) -> Natural {
        self.elements.into_iter().max().unwrap_or(Natural::ZERO)
    }

    /// Returns the number of significant bits of the height of a [`NaturalVector`].
    ///
    /// Since bit length is monotone, this is the largest of the elements' heights' bit lengths, and
    /// 0 for the 0-dimensional vector, without materializing the height.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(1)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is the total number of bits of the
    /// elements.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::Height;
    /// use malachite_nz::natural_vector::NaturalVector;
    ///
    /// assert_eq!(
    ///     NaturalVector::from_str("(1, 3, 2)")
    ///         .unwrap()
    ///         .height_significant_bits(),
    ///     2
    /// );
    /// assert_eq!(
    ///     NaturalVector::from_str("()")
    ///         .unwrap()
    ///         .height_significant_bits(),
    ///     0
    /// );
    /// ```
    #[inline]
    fn height_significant_bits(&self) -> u64 {
        self.height_ref().significant_bits()
    }
}

impl HeightRef for NaturalVector {
    /// Returns a reference to the height of a [`NaturalVector`]: the largest of its elements.
    ///
    /// The height is one of the magnitudes the vector already holds, so it is lent rather than
    /// built. For the 0-dimensional vector, a reference to 0 is returned.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(1)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is the total number of bits of the
    /// elements.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::HeightRef;
    /// use malachite_nz::natural_vector::NaturalVector;
    ///
    /// assert_eq!(
    ///     *NaturalVector::from_str("(1, 3, 2)").unwrap().height_ref(),
    ///     3
    /// );
    /// assert_eq!(*NaturalVector::from_str("()").unwrap().height_ref(), 0);
    /// ```
    #[inline]
    fn height_ref(&self) -> &Natural {
        self.elements.iter().max().unwrap_or(&ZERO)
    }
}
