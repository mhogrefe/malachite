// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::matrix::Matrix;
use crate::num::arithmetic::traits::{ModPowerOf2, ModPowerOf2Assign};
use crate::num::basic::unsigneds::PrimitiveUnsigned;
use crate::unsigned_matrix::UnsignedMatrix;

impl<T: PrimitiveUnsigned> ModPowerOf2 for UnsignedMatrix<T> {
    type Output = Self;

    /// Divides every element of an [`UnsignedMatrix`] by $2^k$, keeping the remainders, taking the
    /// matrix by value.
    ///
    /// The result is reduced modulo $2^k$, which is to say that [`mod_power_of_2_is_reduced`](
    /// crate::num::arithmetic::traits::ModPowerOf2IsReduced::mod_power_of_2_is_reduced) returns
    /// `true` for it. The height and width are unchanged, and elements that reduce to zero stay, so
    /// $[4, 3]$ modulo 4 is $[0, 3]$.
    ///
    /// No `T` reaches $2^W$, where $W$ is `T::WIDTH`, so for $k \geq W$ this leaves the matrix
    /// alone.
    ///
    /// $$
    /// f(A, k) = B, \quad \text{where} \quad B_{ij} = A_{ij} - 2^k \left \lfloor
    /// \frac{A_{ij}}{2^k} \right \rfloor.
    /// $$
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(1)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `self.elements.len()`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::ModPowerOf2;
    /// use malachite_base::unsigned_matrix::UnsignedMatrix;
    ///
    /// // Every element is taken modulo 2.
    /// assert_eq!(
    ///     UnsignedMatrix::<u64>::from_str("[1, 3; 2, 5]")
    ///         .unwrap()
    ///         .mod_power_of_2(1)
    ///         .to_string(),
    ///     "[1, 1; 0, 1]"
    /// );
    /// // An element that reduces to zero stays, so the shape is unchanged.
    /// assert_eq!(
    ///     UnsignedMatrix::<u64>::from_str("[4, 3]")
    ///         .unwrap()
    ///         .mod_power_of_2(2)
    ///         .to_string(),
    ///     "[0, 3]"
    /// );
    /// // A power at least as wide as the element type leaves the matrix alone.
    /// assert_eq!(
    ///     UnsignedMatrix::<u8>::from_str("[255, 1]")
    ///         .unwrap()
    ///         .mod_power_of_2(8)
    ///         .to_string(),
    ///     "[255, 1]"
    /// );
    /// ```
    #[inline]
    fn mod_power_of_2(mut self, pow: u64) -> Self {
        self.mod_power_of_2_assign(pow);
        self
    }
}

impl<T: PrimitiveUnsigned> ModPowerOf2 for &UnsignedMatrix<T> {
    type Output = UnsignedMatrix<T>;

    /// Divides every element of an [`UnsignedMatrix`] by $2^k$, keeping the remainders, taking the
    /// matrix by reference.
    ///
    /// See the documentation for the [`ModPowerOf2`] implementation on [`UnsignedMatrix`] for
    /// details.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `self.elements.len()`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::ModPowerOf2;
    /// use malachite_base::unsigned_matrix::UnsignedMatrix;
    ///
    /// let m = UnsignedMatrix::<u64>::from_str("[4, 3; 5, 6]").unwrap();
    /// assert_eq!((&m).mod_power_of_2(2).to_string(), "[0, 3; 1, 2]");
    /// // The matrix is left alone.
    /// assert_eq!(m.to_string(), "[4, 3; 5, 6]");
    /// ```
    #[inline]
    fn mod_power_of_2(self, pow: u64) -> UnsignedMatrix<T> {
        UnsignedMatrix::from_owned_elements(
            self.height(),
            self.width(),
            self.elements
                .iter()
                .map(|&x| x.mod_power_of_2(pow))
                .collect(),
        )
    }
}

impl<T: PrimitiveUnsigned> ModPowerOf2Assign for UnsignedMatrix<T> {
    /// Divides every element of an [`UnsignedMatrix`] by $2^k$, replacing each element by the
    /// remainder.
    ///
    /// See the documentation for the [`ModPowerOf2`] implementation on [`UnsignedMatrix`] for
    /// details.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(1)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `self.elements.len()`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::ModPowerOf2Assign;
    /// use malachite_base::unsigned_matrix::UnsignedMatrix;
    ///
    /// let mut m = UnsignedMatrix::<u64>::from_str("[1, 3; 2, 5]").unwrap();
    /// m.mod_power_of_2_assign(1);
    /// assert_eq!(m.to_string(), "[1, 1; 0, 1]");
    ///
    /// let mut m = UnsignedMatrix::<u64>::from_str("[4, 3]").unwrap();
    /// m.mod_power_of_2_assign(2);
    /// assert_eq!(m.to_string(), "[0, 3]");
    /// ```
    fn mod_power_of_2_assign(&mut self, pow: u64) {
        // A shortcut rather than a necessity: no `T` reaches its own width as a power, so a wide
        // power is already the identity on every element.
        if pow >= T::WIDTH {
            return;
        }
        for x in &mut self.elements {
            x.mod_power_of_2_assign(pow);
        }
    }
}
