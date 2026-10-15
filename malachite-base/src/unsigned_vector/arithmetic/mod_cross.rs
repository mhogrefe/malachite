// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::num::arithmetic::traits::{ModCross, ModCrossAssign, ModIsReduced};
use crate::num::basic::unsigneds::PrimitiveUnsigned;
use crate::unsigned_vector::UnsignedVector;
use alloc::vec;
use alloc::vec::Vec;

fn assert_valid<T: PrimitiveUnsigned>(v: &UnsignedVector<T>, w: &UnsignedVector<T>, m: T) {
    assert!(
        v.elements.len() == 3 && w.elements.len() == 3,
        "the cross product is only defined for 3-dimensional vectors"
    );
    assert!(
        v.mod_is_reduced(&m),
        "self must be reduced mod m, but {v} has an element >= {m}"
    );
    assert!(
        w.mod_is_reduced(&m),
        "other must be reduced mod m, but {w} has an element >= {m}"
    );
}

/// The cross product modulo `m` of the 3-dimensional vectors with elements `xs` and `ys`, all
/// reduced modulo `m`. Each element is a 2-by-2 determinant, $ab - cd$, whose products share one
/// set of precomputed data for multiplying modulo `m`.
///
/// This is not part of the public API; it is public so that `malachite-nz` can compute cross
/// products of `NaturalVector`s modulo a number that fits in a limb.
#[doc(hidden)]
pub fn mod_cross_slices<T: PrimitiveUnsigned>(xs: &[T], ys: &[T], m: T) -> Vec<T> {
    let [x0, x1, x2] = *xs else { unreachable!() };
    let [y0, y1, y2] = *ys else { unreachable!() };
    let data = T::precompute_mod_mul_data(&m);
    let determinant = |a: T, b: T, c: T, d: T| {
        a.mod_mul_precomputed(b, m, &data)
            .mod_sub(c.mod_mul_precomputed(d, m, &data), m)
    };
    vec![determinant(x1, y2, x2, y1), determinant(x2, y0, x0, y2), determinant(x0, y1, x1, y0)]
}

impl<T: PrimitiveUnsigned> ModCross<Self, T> for UnsignedVector<T> {
    type Output = Self;

    /// Computes the cross product of two 3-dimensional [`UnsignedVector`]s modulo $m$, taking both
    /// by value. The elements of both must already be reduced modulo $m$.
    ///
    /// $$
    /// f(v, w, m) = (v_1w_2 - v_2w_1, v_2w_0 - v_0w_2, v_0w_1 - v_1w_0) \\bmod m.
    /// $$
    ///
    /// The six products share one set of precomputed data for multiplying modulo $m$. Nothing is
    /// divided modulo $m$, so any modulus works, prime or not.
    ///
    /// # Worst-case complexity
    /// Constant time and additional memory.
    ///
    /// # Panics
    /// Panics if `m` is 0, if either vector does not have dimension 3, or if any element of either
    /// is greater than or equal to `m`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::ModCross;
    /// use malachite_base::unsigned_vector::UnsignedVector;
    ///
    /// assert_eq!(
    ///     UnsignedVector::<u8>::from_str("(1, 2, 3)")
    ///         .unwrap()
    ///         .mod_cross(UnsignedVector::<u8>::from_str("(4, 5, 6)").unwrap(), 7)
    ///         .to_string(),
    ///     "(4, 6, 4)"
    /// );
    /// // The cross product of the first two standard basis vectors is the third.
    /// assert_eq!(
    ///     UnsignedVector::<u8>::from_str("(1, 0, 0)")
    ///         .unwrap()
    ///         .mod_cross(UnsignedVector::<u8>::from_str("(0, 1, 0)").unwrap(), 5)
    ///         .to_string(),
    ///     "(0, 0, 1)"
    /// );
    /// // Modulo 10, the negative components wrap around.
    /// assert_eq!(
    ///     UnsignedVector::<u8>::from_str("(3, 5, 7)")
    ///         .unwrap()
    ///         .mod_cross(UnsignedVector::<u8>::from_str("(7, 5, 3)").unwrap(), 10)
    ///         .to_string(),
    ///     "(0, 0, 0)"
    /// );
    /// ```
    #[inline]
    fn mod_cross(self, other: Self, m: T) -> Self {
        assert_valid(&self, &other, m);
        Self {
            elements: mod_cross_slices(&self.elements, &other.elements, m),
        }
    }
}

impl<T: PrimitiveUnsigned> ModCross<&Self, T> for UnsignedVector<T> {
    type Output = Self;

    /// Computes the cross product of two 3-dimensional [`UnsignedVector`]s modulo $m$, taking the
    /// first by value and the second by reference. The elements of both must already be reduced
    /// modulo $m$.
    ///
    /// $$
    /// f(v, w, m) = (v_1w_2 - v_2w_1, v_2w_0 - v_0w_2, v_0w_1 - v_1w_0) \\bmod m.
    /// $$
    ///
    /// The six products share one set of precomputed data for multiplying modulo $m$. Nothing is
    /// divided modulo $m$, so any modulus works, prime or not.
    ///
    /// # Worst-case complexity
    /// Constant time and additional memory.
    ///
    /// # Panics
    /// Panics if `m` is 0, if either vector does not have dimension 3, or if any element of either
    /// is greater than or equal to `m`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::ModCross;
    /// use malachite_base::unsigned_vector::UnsignedVector;
    ///
    /// assert_eq!(
    ///     UnsignedVector::<u8>::from_str("(1, 2, 3)")
    ///         .unwrap()
    ///         .mod_cross(&UnsignedVector::<u8>::from_str("(4, 5, 6)").unwrap(), 7)
    ///         .to_string(),
    ///     "(4, 6, 4)"
    /// );
    /// // The cross product of the first two standard basis vectors is the third.
    /// assert_eq!(
    ///     UnsignedVector::<u8>::from_str("(1, 0, 0)")
    ///         .unwrap()
    ///         .mod_cross(&UnsignedVector::<u8>::from_str("(0, 1, 0)").unwrap(), 5)
    ///         .to_string(),
    ///     "(0, 0, 1)"
    /// );
    /// // Modulo 10, the negative components wrap around.
    /// assert_eq!(
    ///     UnsignedVector::<u8>::from_str("(3, 5, 7)")
    ///         .unwrap()
    ///         .mod_cross(&UnsignedVector::<u8>::from_str("(7, 5, 3)").unwrap(), 10)
    ///         .to_string(),
    ///     "(0, 0, 0)"
    /// );
    /// ```
    #[inline]
    fn mod_cross(self, other: &Self, m: T) -> Self {
        assert_valid(&self, other, m);
        Self {
            elements: mod_cross_slices(&self.elements, &other.elements, m),
        }
    }
}

impl<T: PrimitiveUnsigned> ModCross<UnsignedVector<T>, T> for &UnsignedVector<T> {
    type Output = UnsignedVector<T>;

    /// Computes the cross product of two 3-dimensional [`UnsignedVector`]s modulo $m$, taking the
    /// first by reference and the second by value. The elements of both must already be reduced
    /// modulo $m$.
    ///
    /// $$
    /// f(v, w, m) = (v_1w_2 - v_2w_1, v_2w_0 - v_0w_2, v_0w_1 - v_1w_0) \\bmod m.
    /// $$
    ///
    /// The six products share one set of precomputed data for multiplying modulo $m$. Nothing is
    /// divided modulo $m$, so any modulus works, prime or not.
    ///
    /// # Worst-case complexity
    /// Constant time and additional memory.
    ///
    /// # Panics
    /// Panics if `m` is 0, if either vector does not have dimension 3, or if any element of either
    /// is greater than or equal to `m`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::ModCross;
    /// use malachite_base::unsigned_vector::UnsignedVector;
    ///
    /// assert_eq!(
    ///     (&UnsignedVector::<u8>::from_str("(1, 2, 3)").unwrap())
    ///         .mod_cross(UnsignedVector::<u8>::from_str("(4, 5, 6)").unwrap(), 7)
    ///         .to_string(),
    ///     "(4, 6, 4)"
    /// );
    /// // The cross product of the first two standard basis vectors is the third.
    /// assert_eq!(
    ///     (&UnsignedVector::<u8>::from_str("(1, 0, 0)").unwrap())
    ///         .mod_cross(UnsignedVector::<u8>::from_str("(0, 1, 0)").unwrap(), 5)
    ///         .to_string(),
    ///     "(0, 0, 1)"
    /// );
    /// // Modulo 10, the negative components wrap around.
    /// assert_eq!(
    ///     (&UnsignedVector::<u8>::from_str("(3, 5, 7)").unwrap())
    ///         .mod_cross(UnsignedVector::<u8>::from_str("(7, 5, 3)").unwrap(), 10)
    ///         .to_string(),
    ///     "(0, 0, 0)"
    /// );
    /// ```
    #[inline]
    fn mod_cross(self, other: UnsignedVector<T>, m: T) -> UnsignedVector<T> {
        assert_valid(self, &other, m);
        UnsignedVector {
            elements: mod_cross_slices(&self.elements, &other.elements, m),
        }
    }
}

impl<T: PrimitiveUnsigned> ModCross<&UnsignedVector<T>, T> for &UnsignedVector<T> {
    type Output = UnsignedVector<T>;

    /// Computes the cross product of two 3-dimensional [`UnsignedVector`]s modulo $m$, taking both
    /// by reference. The elements of both must already be reduced modulo $m$.
    ///
    /// $$
    /// f(v, w, m) = (v_1w_2 - v_2w_1, v_2w_0 - v_0w_2, v_0w_1 - v_1w_0) \\bmod m.
    /// $$
    ///
    /// The six products share one set of precomputed data for multiplying modulo $m$. Nothing is
    /// divided modulo $m$, so any modulus works, prime or not.
    ///
    /// # Worst-case complexity
    /// Constant time and additional memory.
    ///
    /// # Panics
    /// Panics if `m` is 0, if either vector does not have dimension 3, or if any element of either
    /// is greater than or equal to `m`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::ModCross;
    /// use malachite_base::unsigned_vector::UnsignedVector;
    ///
    /// assert_eq!(
    ///     (&UnsignedVector::<u8>::from_str("(1, 2, 3)").unwrap())
    ///         .mod_cross(&UnsignedVector::<u8>::from_str("(4, 5, 6)").unwrap(), 7)
    ///         .to_string(),
    ///     "(4, 6, 4)"
    /// );
    /// // The cross product of the first two standard basis vectors is the third.
    /// assert_eq!(
    ///     (&UnsignedVector::<u8>::from_str("(1, 0, 0)").unwrap())
    ///         .mod_cross(&UnsignedVector::<u8>::from_str("(0, 1, 0)").unwrap(), 5)
    ///         .to_string(),
    ///     "(0, 0, 1)"
    /// );
    /// // Modulo 10, the negative components wrap around.
    /// assert_eq!(
    ///     (&UnsignedVector::<u8>::from_str("(3, 5, 7)").unwrap())
    ///         .mod_cross(&UnsignedVector::<u8>::from_str("(7, 5, 3)").unwrap(), 10)
    ///         .to_string(),
    ///     "(0, 0, 0)"
    /// );
    /// ```
    #[inline]
    fn mod_cross(self, other: &UnsignedVector<T>, m: T) -> UnsignedVector<T> {
        assert_valid(self, other, m);
        UnsignedVector {
            elements: mod_cross_slices(&self.elements, &other.elements, m),
        }
    }
}

impl<T: PrimitiveUnsigned> ModCrossAssign<Self, T> for UnsignedVector<T> {
    /// Replaces a 3-dimensional [`UnsignedVector`] with its cross product with another modulo $m$,
    /// taking the other by value. The elements of both must already be reduced modulo $m$.
    ///
    /// $$
    /// v \\gets v \\times w \\bmod m.
    /// $$
    ///
    /// The six products share one set of precomputed data for multiplying modulo $m$. Nothing is
    /// divided modulo $m$, so any modulus works, prime or not.
    ///
    /// # Worst-case complexity
    /// Constant time and additional memory.
    ///
    /// # Panics
    /// Panics if `m` is 0, if either vector does not have dimension 3, or if any element of either
    /// is greater than or equal to `m`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::ModCrossAssign;
    /// use malachite_base::unsigned_vector::UnsignedVector;
    ///
    /// let mut v = UnsignedVector::<u8>::from_str("(1, 2, 3)").unwrap();
    /// v.mod_cross_assign(UnsignedVector::<u8>::from_str("(4, 5, 6)").unwrap(), 7);
    /// assert_eq!(v.to_string(), "(4, 6, 4)");
    ///
    /// // The cross product of the first two standard basis vectors is the third.
    /// let mut v = UnsignedVector::<u8>::from_str("(1, 0, 0)").unwrap();
    /// v.mod_cross_assign(UnsignedVector::<u8>::from_str("(0, 1, 0)").unwrap(), 5);
    /// assert_eq!(v.to_string(), "(0, 0, 1)");
    ///
    /// // Modulo 10, the negative components wrap around.
    /// let mut v = UnsignedVector::<u8>::from_str("(3, 5, 7)").unwrap();
    /// v.mod_cross_assign(UnsignedVector::<u8>::from_str("(7, 5, 3)").unwrap(), 10);
    /// assert_eq!(v.to_string(), "(0, 0, 0)");
    /// ```
    #[inline]
    fn mod_cross_assign(&mut self, other: Self, m: T) {
        assert_valid(self, &other, m);
        self.elements = mod_cross_slices(&self.elements, &other.elements, m);
    }
}

impl<T: PrimitiveUnsigned> ModCrossAssign<&Self, T> for UnsignedVector<T> {
    /// Replaces a 3-dimensional [`UnsignedVector`] with its cross product with another modulo $m$,
    /// taking the other by reference. The elements of both must already be reduced modulo $m$.
    ///
    /// $$
    /// v \\gets v \\times w \\bmod m.
    /// $$
    ///
    /// The six products share one set of precomputed data for multiplying modulo $m$. Nothing is
    /// divided modulo $m$, so any modulus works, prime or not.
    ///
    /// # Worst-case complexity
    /// Constant time and additional memory.
    ///
    /// # Panics
    /// Panics if `m` is 0, if either vector does not have dimension 3, or if any element of either
    /// is greater than or equal to `m`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::ModCrossAssign;
    /// use malachite_base::unsigned_vector::UnsignedVector;
    ///
    /// let mut v = UnsignedVector::<u8>::from_str("(1, 2, 3)").unwrap();
    /// v.mod_cross_assign(&UnsignedVector::<u8>::from_str("(4, 5, 6)").unwrap(), 7);
    /// assert_eq!(v.to_string(), "(4, 6, 4)");
    ///
    /// // The cross product of the first two standard basis vectors is the third.
    /// let mut v = UnsignedVector::<u8>::from_str("(1, 0, 0)").unwrap();
    /// v.mod_cross_assign(&UnsignedVector::<u8>::from_str("(0, 1, 0)").unwrap(), 5);
    /// assert_eq!(v.to_string(), "(0, 0, 1)");
    ///
    /// // Modulo 10, the negative components wrap around.
    /// let mut v = UnsignedVector::<u8>::from_str("(3, 5, 7)").unwrap();
    /// v.mod_cross_assign(&UnsignedVector::<u8>::from_str("(7, 5, 3)").unwrap(), 10);
    /// assert_eq!(v.to_string(), "(0, 0, 0)");
    /// ```
    #[inline]
    fn mod_cross_assign(&mut self, other: &Self, m: T) {
        assert_valid(self, other, m);
        self.elements = mod_cross_slices(&self.elements, &other.elements, m);
    }
}
