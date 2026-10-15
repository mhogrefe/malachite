// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::num::arithmetic::traits::{
    ModPowerOf2Cross, ModPowerOf2CrossAssign, ModPowerOf2IsReduced,
};
use crate::num::basic::unsigneds::PrimitiveUnsigned;
use crate::unsigned_vector::UnsignedVector;
use alloc::vec;
use alloc::vec::Vec;

fn assert_valid<T: PrimitiveUnsigned>(v: &UnsignedVector<T>, w: &UnsignedVector<T>, pow: u64) {
    assert!(
        pow <= T::WIDTH,
        "pow must be at most T::WIDTH, but it is {pow}"
    );
    assert!(
        v.elements.len() == 3 && w.elements.len() == 3,
        "the cross product is only defined for 3-dimensional vectors"
    );
    assert!(
        v.mod_power_of_2_is_reduced(pow),
        "self must be reduced mod 2^pow, but {v} has an element >= 2^{pow}"
    );
    assert!(
        w.mod_power_of_2_is_reduced(pow),
        "other must be reduced mod 2^pow, but {w} has an element >= 2^{pow}"
    );
}

/// The cross product modulo $2^k$, where $k$ is `pow`, of the 3-dimensional vectors with elements
/// `xs` and `ys`, all reduced modulo $2^k$. Each element is a 2-by-2 determinant computed in
/// wrapping arithmetic and reduced at the end, which is right because $2^k$ divides $2^W$.
///
/// This is not part of the public API; it is public so that `malachite-nz` can compute cross
/// products of `NaturalVector`s modulo powers of 2 that fit in a limb.
#[doc(hidden)]
pub fn mod_power_of_2_cross_slices<T: PrimitiveUnsigned>(xs: &[T], ys: &[T], pow: u64) -> Vec<T> {
    let [x0, x1, x2] = *xs else { unreachable!() };
    let [y0, y1, y2] = *ys else { unreachable!() };
    let mask = T::low_mask(pow);
    vec![
        x1.wrapping_mul(y2).wrapping_sub(x2.wrapping_mul(y1)) & mask,
        x2.wrapping_mul(y0).wrapping_sub(x0.wrapping_mul(y2)) & mask,
        x0.wrapping_mul(y1).wrapping_sub(x1.wrapping_mul(y0)) & mask,
    ]
}

impl<T: PrimitiveUnsigned> ModPowerOf2Cross<Self> for UnsignedVector<T> {
    type Output = Self;

    /// Computes the cross product of two 3-dimensional [`UnsignedVector`]s modulo $2^k$, taking
    /// both by value. The elements of both must already be reduced modulo $2^k$.
    ///
    /// $$
    /// f(v, w, k) = (v_1w_2 - v_2w_1, v_2w_0 - v_0w_2, v_0w_1 - v_1w_0) \\bmod 2^k.
    /// $$
    ///
    /// The products and differences are computed in wrapping word arithmetic and reduced modulo
    /// $2^k$ at the end, which gives the right result because $2^k$ divides $2^W$.
    ///
    /// # Worst-case complexity
    /// Constant time and additional memory.
    ///
    /// # Panics
    /// Panics if `pow` is greater than `T::WIDTH`, if either vector does not have dimension 3, or
    /// if any element of either is greater than or equal to $2^k$.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::ModPowerOf2Cross;
    /// use malachite_base::unsigned_vector::UnsignedVector;
    ///
    /// assert_eq!(
    ///     UnsignedVector::<u8>::from_str("(1, 2, 3)")
    ///         .unwrap()
    ///         .mod_power_of_2_cross(UnsignedVector::<u8>::from_str("(4, 5, 6)").unwrap(), 4)
    ///         .to_string(),
    ///     "(13, 6, 13)"
    /// );
    /// // The cross product of the first two standard basis vectors is the third.
    /// assert_eq!(
    ///     UnsignedVector::<u8>::from_str("(1, 0, 0)")
    ///         .unwrap()
    ///         .mod_power_of_2_cross(UnsignedVector::<u8>::from_str("(0, 1, 0)").unwrap(), 8)
    ///         .to_string(),
    ///     "(0, 0, 1)"
    /// );
    /// // Modulo 8, the negative components wrap around.
    /// assert_eq!(
    ///     UnsignedVector::<u8>::from_str("(3, 5, 7)")
    ///         .unwrap()
    ///         .mod_power_of_2_cross(UnsignedVector::<u8>::from_str("(7, 5, 3)").unwrap(), 3)
    ///         .to_string(),
    ///     "(4, 0, 4)"
    /// );
    /// ```
    #[inline]
    fn mod_power_of_2_cross(self, other: Self, pow: u64) -> Self {
        assert_valid(&self, &other, pow);
        Self {
            elements: mod_power_of_2_cross_slices(&self.elements, &other.elements, pow),
        }
    }
}

impl<T: PrimitiveUnsigned> ModPowerOf2Cross<&Self> for UnsignedVector<T> {
    type Output = Self;

    /// Computes the cross product of two 3-dimensional [`UnsignedVector`]s modulo $2^k$, taking the
    /// first by value and the second by reference. The elements of both must already be reduced
    /// modulo $2^k$.
    ///
    /// $$
    /// f(v, w, k) = (v_1w_2 - v_2w_1, v_2w_0 - v_0w_2, v_0w_1 - v_1w_0) \\bmod 2^k.
    /// $$
    ///
    /// The products and differences are computed in wrapping word arithmetic and reduced modulo
    /// $2^k$ at the end, which gives the right result because $2^k$ divides $2^W$.
    ///
    /// # Worst-case complexity
    /// Constant time and additional memory.
    ///
    /// # Panics
    /// Panics if `pow` is greater than `T::WIDTH`, if either vector does not have dimension 3, or
    /// if any element of either is greater than or equal to $2^k$.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::ModPowerOf2Cross;
    /// use malachite_base::unsigned_vector::UnsignedVector;
    ///
    /// assert_eq!(
    ///     UnsignedVector::<u8>::from_str("(1, 2, 3)")
    ///         .unwrap()
    ///         .mod_power_of_2_cross(&UnsignedVector::<u8>::from_str("(4, 5, 6)").unwrap(), 4)
    ///         .to_string(),
    ///     "(13, 6, 13)"
    /// );
    /// // The cross product of the first two standard basis vectors is the third.
    /// assert_eq!(
    ///     UnsignedVector::<u8>::from_str("(1, 0, 0)")
    ///         .unwrap()
    ///         .mod_power_of_2_cross(&UnsignedVector::<u8>::from_str("(0, 1, 0)").unwrap(), 8)
    ///         .to_string(),
    ///     "(0, 0, 1)"
    /// );
    /// // Modulo 8, the negative components wrap around.
    /// assert_eq!(
    ///     UnsignedVector::<u8>::from_str("(3, 5, 7)")
    ///         .unwrap()
    ///         .mod_power_of_2_cross(&UnsignedVector::<u8>::from_str("(7, 5, 3)").unwrap(), 3)
    ///         .to_string(),
    ///     "(4, 0, 4)"
    /// );
    /// ```
    #[inline]
    fn mod_power_of_2_cross(self, other: &Self, pow: u64) -> Self {
        assert_valid(&self, other, pow);
        Self {
            elements: mod_power_of_2_cross_slices(&self.elements, &other.elements, pow),
        }
    }
}

impl<T: PrimitiveUnsigned> ModPowerOf2Cross<UnsignedVector<T>> for &UnsignedVector<T> {
    type Output = UnsignedVector<T>;

    /// Computes the cross product of two 3-dimensional [`UnsignedVector`]s modulo $2^k$, taking the
    /// first by reference and the second by value. The elements of both must already be reduced
    /// modulo $2^k$.
    ///
    /// $$
    /// f(v, w, k) = (v_1w_2 - v_2w_1, v_2w_0 - v_0w_2, v_0w_1 - v_1w_0) \\bmod 2^k.
    /// $$
    ///
    /// The products and differences are computed in wrapping word arithmetic and reduced modulo
    /// $2^k$ at the end, which gives the right result because $2^k$ divides $2^W$.
    ///
    /// # Worst-case complexity
    /// Constant time and additional memory.
    ///
    /// # Panics
    /// Panics if `pow` is greater than `T::WIDTH`, if either vector does not have dimension 3, or
    /// if any element of either is greater than or equal to $2^k$.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::ModPowerOf2Cross;
    /// use malachite_base::unsigned_vector::UnsignedVector;
    ///
    /// assert_eq!(
    ///     (&UnsignedVector::<u8>::from_str("(1, 2, 3)").unwrap())
    ///         .mod_power_of_2_cross(UnsignedVector::<u8>::from_str("(4, 5, 6)").unwrap(), 4)
    ///         .to_string(),
    ///     "(13, 6, 13)"
    /// );
    /// // The cross product of the first two standard basis vectors is the third.
    /// assert_eq!(
    ///     (&UnsignedVector::<u8>::from_str("(1, 0, 0)").unwrap())
    ///         .mod_power_of_2_cross(UnsignedVector::<u8>::from_str("(0, 1, 0)").unwrap(), 8)
    ///         .to_string(),
    ///     "(0, 0, 1)"
    /// );
    /// // Modulo 8, the negative components wrap around.
    /// assert_eq!(
    ///     (&UnsignedVector::<u8>::from_str("(3, 5, 7)").unwrap())
    ///         .mod_power_of_2_cross(UnsignedVector::<u8>::from_str("(7, 5, 3)").unwrap(), 3)
    ///         .to_string(),
    ///     "(4, 0, 4)"
    /// );
    /// ```
    #[inline]
    fn mod_power_of_2_cross(self, other: UnsignedVector<T>, pow: u64) -> UnsignedVector<T> {
        assert_valid(self, &other, pow);
        UnsignedVector {
            elements: mod_power_of_2_cross_slices(&self.elements, &other.elements, pow),
        }
    }
}

impl<T: PrimitiveUnsigned> ModPowerOf2Cross<&UnsignedVector<T>> for &UnsignedVector<T> {
    type Output = UnsignedVector<T>;

    /// Computes the cross product of two 3-dimensional [`UnsignedVector`]s modulo $2^k$, taking
    /// both by reference. The elements of both must already be reduced modulo $2^k$.
    ///
    /// $$
    /// f(v, w, k) = (v_1w_2 - v_2w_1, v_2w_0 - v_0w_2, v_0w_1 - v_1w_0) \\bmod 2^k.
    /// $$
    ///
    /// The products and differences are computed in wrapping word arithmetic and reduced modulo
    /// $2^k$ at the end, which gives the right result because $2^k$ divides $2^W$.
    ///
    /// # Worst-case complexity
    /// Constant time and additional memory.
    ///
    /// # Panics
    /// Panics if `pow` is greater than `T::WIDTH`, if either vector does not have dimension 3, or
    /// if any element of either is greater than or equal to $2^k$.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::ModPowerOf2Cross;
    /// use malachite_base::unsigned_vector::UnsignedVector;
    ///
    /// assert_eq!(
    ///     (&UnsignedVector::<u8>::from_str("(1, 2, 3)").unwrap())
    ///         .mod_power_of_2_cross(&UnsignedVector::<u8>::from_str("(4, 5, 6)").unwrap(), 4)
    ///         .to_string(),
    ///     "(13, 6, 13)"
    /// );
    /// // The cross product of the first two standard basis vectors is the third.
    /// assert_eq!(
    ///     (&UnsignedVector::<u8>::from_str("(1, 0, 0)").unwrap())
    ///         .mod_power_of_2_cross(&UnsignedVector::<u8>::from_str("(0, 1, 0)").unwrap(), 8)
    ///         .to_string(),
    ///     "(0, 0, 1)"
    /// );
    /// // Modulo 8, the negative components wrap around.
    /// assert_eq!(
    ///     (&UnsignedVector::<u8>::from_str("(3, 5, 7)").unwrap())
    ///         .mod_power_of_2_cross(&UnsignedVector::<u8>::from_str("(7, 5, 3)").unwrap(), 3)
    ///         .to_string(),
    ///     "(4, 0, 4)"
    /// );
    /// ```
    #[inline]
    fn mod_power_of_2_cross(self, other: &UnsignedVector<T>, pow: u64) -> UnsignedVector<T> {
        assert_valid(self, other, pow);
        UnsignedVector {
            elements: mod_power_of_2_cross_slices(&self.elements, &other.elements, pow),
        }
    }
}

impl<T: PrimitiveUnsigned> ModPowerOf2CrossAssign<Self> for UnsignedVector<T> {
    /// Replaces a 3-dimensional [`UnsignedVector`] with its cross product with another modulo
    /// $2^k$, taking the other by value. The elements of both must already be reduced modulo $2^k$.
    ///
    /// $$
    /// v \\gets v \\times w \\bmod 2^k.
    /// $$
    ///
    /// The products and differences are computed in wrapping word arithmetic and reduced modulo
    /// $2^k$ at the end, which gives the right result because $2^k$ divides $2^W$.
    ///
    /// # Worst-case complexity
    /// Constant time and additional memory.
    ///
    /// # Panics
    /// Panics if `pow` is greater than `T::WIDTH`, if either vector does not have dimension 3, or
    /// if any element of either is greater than or equal to $2^k$.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::ModPowerOf2CrossAssign;
    /// use malachite_base::unsigned_vector::UnsignedVector;
    ///
    /// let mut v = UnsignedVector::<u8>::from_str("(1, 2, 3)").unwrap();
    /// v.mod_power_of_2_cross_assign(UnsignedVector::<u8>::from_str("(4, 5, 6)").unwrap(), 4);
    /// assert_eq!(v.to_string(), "(13, 6, 13)");
    ///
    /// // The cross product of the first two standard basis vectors is the third.
    /// let mut v = UnsignedVector::<u8>::from_str("(1, 0, 0)").unwrap();
    /// v.mod_power_of_2_cross_assign(UnsignedVector::<u8>::from_str("(0, 1, 0)").unwrap(), 8);
    /// assert_eq!(v.to_string(), "(0, 0, 1)");
    ///
    /// // Modulo 8, the negative components wrap around.
    /// let mut v = UnsignedVector::<u8>::from_str("(3, 5, 7)").unwrap();
    /// v.mod_power_of_2_cross_assign(UnsignedVector::<u8>::from_str("(7, 5, 3)").unwrap(), 3);
    /// assert_eq!(v.to_string(), "(4, 0, 4)");
    /// ```
    #[inline]
    fn mod_power_of_2_cross_assign(&mut self, other: Self, pow: u64) {
        assert_valid(self, &other, pow);
        self.elements = mod_power_of_2_cross_slices(&self.elements, &other.elements, pow);
    }
}

impl<T: PrimitiveUnsigned> ModPowerOf2CrossAssign<&Self> for UnsignedVector<T> {
    /// Replaces a 3-dimensional [`UnsignedVector`] with its cross product with another modulo
    /// $2^k$, taking the other by reference. The elements of both must already be reduced modulo
    /// $2^k$.
    ///
    /// $$
    /// v \\gets v \\times w \\bmod 2^k.
    /// $$
    ///
    /// The products and differences are computed in wrapping word arithmetic and reduced modulo
    /// $2^k$ at the end, which gives the right result because $2^k$ divides $2^W$.
    ///
    /// # Worst-case complexity
    /// Constant time and additional memory.
    ///
    /// # Panics
    /// Panics if `pow` is greater than `T::WIDTH`, if either vector does not have dimension 3, or
    /// if any element of either is greater than or equal to $2^k$.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::ModPowerOf2CrossAssign;
    /// use malachite_base::unsigned_vector::UnsignedVector;
    ///
    /// let mut v = UnsignedVector::<u8>::from_str("(1, 2, 3)").unwrap();
    /// v.mod_power_of_2_cross_assign(&UnsignedVector::<u8>::from_str("(4, 5, 6)").unwrap(), 4);
    /// assert_eq!(v.to_string(), "(13, 6, 13)");
    ///
    /// // The cross product of the first two standard basis vectors is the third.
    /// let mut v = UnsignedVector::<u8>::from_str("(1, 0, 0)").unwrap();
    /// v.mod_power_of_2_cross_assign(&UnsignedVector::<u8>::from_str("(0, 1, 0)").unwrap(), 8);
    /// assert_eq!(v.to_string(), "(0, 0, 1)");
    ///
    /// // Modulo 8, the negative components wrap around.
    /// let mut v = UnsignedVector::<u8>::from_str("(3, 5, 7)").unwrap();
    /// v.mod_power_of_2_cross_assign(&UnsignedVector::<u8>::from_str("(7, 5, 3)").unwrap(), 3);
    /// assert_eq!(v.to_string(), "(4, 0, 4)");
    /// ```
    #[inline]
    fn mod_power_of_2_cross_assign(&mut self, other: &Self, pow: u64) {
        assert_valid(self, other, pow);
        self.elements = mod_power_of_2_cross_slices(&self.elements, &other.elements, pow);
    }
}
