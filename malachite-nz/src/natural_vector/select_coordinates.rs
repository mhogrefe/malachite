// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::natural_vector::NaturalVector;
use malachite_base::vector::{
    SelectCoordinates, SelectCoordinatesAssign, select_elements, select_elements_in_place,
};

impl SelectCoordinates for NaturalVector {
    type Output = Self;

    /// Selects coordinates of a [`NaturalVector`] by index, taking the vector by value.
    ///
    /// Element $j$ of the result is element `indices[j]` of the original, so the result's dimension
    /// is the number of indices. The indices may repeat and may appear in any order.
    ///
    /// Elements are moved rather than cloned wherever possible: when the indices are strictly
    /// increasing, nothing is cloned or allocated, and otherwise an element is cloned only once for
    /// each extra time it is selected.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `self.dimension()` plus
    /// `indices.len()`, plus the total size of the elements that are cloned.
    ///
    /// # Panics
    /// Panics if any index is greater than or equal to `self.dimension()`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::vector::SelectCoordinates;
    /// use malachite_nz::natural_vector::NaturalVector;
    ///
    /// let v = NaturalVector::from_str("(10, 20, 30)").unwrap();
    /// assert_eq!(
    ///     v.clone().select_coordinates(&[2, 0]).to_string(),
    ///     "(30, 10)"
    /// );
    /// assert_eq!(
    ///     v.clone().select_coordinates(&[1, 1, 1]).to_string(),
    ///     "(20, 20, 20)"
    /// );
    /// assert_eq!(v.select_coordinates(&[0, 2]).to_string(), "(10, 30)");
    /// ```
    #[inline]
    fn select_coordinates(mut self, indices: &[u64]) -> Self {
        self.select_coordinates_assign(indices);
        self
    }
}

impl SelectCoordinates for &NaturalVector {
    type Output = NaturalVector;

    /// Selects coordinates of a [`NaturalVector`] by index, taking the vector by reference.
    ///
    /// Element $j$ of the result is element `indices[j]` of the original, so the result's dimension
    /// is the number of indices. The indices may repeat and may appear in any order.
    ///
    /// The selected elements are cloned.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `indices.len()` plus the total size
    /// of the selected elements.
    ///
    /// # Panics
    /// Panics if any index is greater than or equal to `self.dimension()`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::vector::SelectCoordinates;
    /// use malachite_nz::natural_vector::NaturalVector;
    ///
    /// let v = NaturalVector::from_str("(10, 20, 30)").unwrap();
    /// assert_eq!((&v).select_coordinates(&[2, 0]).to_string(), "(30, 10)");
    /// assert_eq!(
    ///     (&v).select_coordinates(&[1, 1, 1]).to_string(),
    ///     "(20, 20, 20)"
    /// );
    /// assert_eq!((&v).select_coordinates(&[]).to_string(), "()");
    /// ```
    #[inline]
    fn select_coordinates(self, indices: &[u64]) -> NaturalVector {
        NaturalVector {
            elements: select_elements(&self.elements, indices),
        }
    }
}

impl SelectCoordinatesAssign for NaturalVector {
    /// Selects coordinates of a [`NaturalVector`] by index, in place.
    ///
    /// Afterwards, element $j$ of the vector is what element `indices[j]` was before, so its
    /// dimension is the number of indices. The indices may repeat and may appear in any order.
    ///
    /// Elements are moved rather than cloned wherever possible: when the indices are strictly
    /// increasing, nothing is cloned or allocated, and otherwise an element is cloned only once for
    /// each extra time it is selected.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `self.dimension()` plus
    /// `indices.len()`, plus the total size of the elements that are cloned.
    ///
    /// # Panics
    /// Panics if any index is greater than or equal to `self.dimension()`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::vector::SelectCoordinatesAssign;
    /// use malachite_nz::natural_vector::NaturalVector;
    ///
    /// let mut v = NaturalVector::from_str("(10, 20, 30)").unwrap();
    /// v.select_coordinates_assign(&[2, 0]);
    /// assert_eq!(v.to_string(), "(30, 10)");
    /// v.select_coordinates_assign(&[1, 1]);
    /// assert_eq!(v.to_string(), "(10, 10)");
    /// ```
    #[inline]
    fn select_coordinates_assign(&mut self, indices: &[u64]) {
        select_elements_in_place(&mut self.elements, indices);
    }
}
