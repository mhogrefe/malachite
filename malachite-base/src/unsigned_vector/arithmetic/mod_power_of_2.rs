// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::num::arithmetic::traits::{ModPowerOf2, ModPowerOf2Assign};
use crate::num::basic::unsigneds::PrimitiveUnsigned;
use crate::unsigned_vector::UnsignedVector;

impl<T: PrimitiveUnsigned> ModPowerOf2 for UnsignedVector<T> {
    type Output = Self;

    /// Divides every element of an [`UnsignedVector`] by $2^k$, keeping the remainders, taking the
    /// vector by value.
    ///
    /// The result is reduced modulo $2^k$, which is to say that [`mod_power_of_2_is_reduced`](
    /// crate::num::arithmetic::traits::ModPowerOf2IsReduced::mod_power_of_2_is_reduced) returns
    /// `true` for it. The dimension is unchanged: unlike a polynomial, a vector keeps the elements
    /// that reduce to zero, so $(4, 3)$ modulo 4 is $(0, 3)$.
    ///
    /// No `T` reaches $2^W$, where $W$ is `T::WIDTH`, so for $k \geq W$ this leaves the vector
    /// alone.
    ///
    /// $$
    /// f(v, k) = w, \\quad \text{where} \\quad w_i = v_i - 2^k \left \lfloor \frac{v_i}{2^k}
    /// \right \rfloor.
    /// $$
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(1)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `self.dimension()`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::ModPowerOf2;
    /// use malachite_base::unsigned_vector::UnsignedVector;
    ///
    /// // Every element is taken modulo 2.
    /// assert_eq!(
    ///     UnsignedVector::<u64>::from_str("(1, 3, 2)")
    ///         .unwrap()
    ///         .mod_power_of_2(1)
    ///         .to_string(),
    ///     "(1, 1, 0)"
    /// );
    ///
    /// // An element that reduces to zero stays, so the dimension is unchanged.
    /// assert_eq!(
    ///     UnsignedVector::<u64>::from_str("(4, 3)")
    ///         .unwrap()
    ///         .mod_power_of_2(2)
    ///         .to_string(),
    ///     "(0, 3)"
    /// );
    ///
    /// // A power at least as wide as the element type leaves the vector alone.
    /// assert_eq!(
    ///     UnsignedVector::<u8>::from_str("(255, 1)")
    ///         .unwrap()
    ///         .mod_power_of_2(8)
    ///         .to_string(),
    ///     "(255, 1)"
    /// );
    /// ```
    #[inline]
    fn mod_power_of_2(mut self, pow: u64) -> Self {
        self.mod_power_of_2_assign(pow);
        self
    }
}

impl<T: PrimitiveUnsigned> ModPowerOf2 for &UnsignedVector<T> {
    type Output = UnsignedVector<T>;

    /// Divides every element of an [`UnsignedVector`] by $2^k$, keeping the remainders, taking the
    /// vector by reference.
    ///
    /// See the documentation for the [`ModPowerOf2`] implementation on [`UnsignedVector`] for
    /// details.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `self.dimension()`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::ModPowerOf2;
    /// use malachite_base::unsigned_vector::UnsignedVector;
    ///
    /// let v = UnsignedVector::<u64>::from_str("(4, 3)").unwrap();
    /// assert_eq!((&v).mod_power_of_2(2).to_string(), "(0, 3)");
    /// // The vector is left alone.
    /// assert_eq!(v.to_string(), "(4, 3)");
    /// ```
    #[inline]
    fn mod_power_of_2(self, pow: u64) -> UnsignedVector<T> {
        UnsignedVector {
            elements: self
                .elements
                .iter()
                .map(|&x| x.mod_power_of_2(pow))
                .collect(),
        }
    }
}

impl<T: PrimitiveUnsigned> ModPowerOf2Assign for UnsignedVector<T> {
    /// Divides every element of an [`UnsignedVector`] by $2^k$, replacing each element by the
    /// remainder.
    ///
    /// See the documentation for the [`ModPowerOf2`] implementation on [`UnsignedVector`] for
    /// details.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(1)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `self.dimension()`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::ModPowerOf2Assign;
    /// use malachite_base::unsigned_vector::UnsignedVector;
    ///
    /// let mut v = UnsignedVector::<u64>::from_str("(1, 3, 2)").unwrap();
    /// v.mod_power_of_2_assign(1);
    /// assert_eq!(v.to_string(), "(1, 1, 0)");
    ///
    /// let mut v = UnsignedVector::<u64>::from_str("(4, 3)").unwrap();
    /// v.mod_power_of_2_assign(2);
    /// assert_eq!(v.to_string(), "(0, 3)");
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
