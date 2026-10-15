// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::natural::Natural;
use crate::natural_vector::NaturalVector;
use crate::platform::Limb;
use alloc::vec;
use alloc::vec::Vec;
use malachite_base::num::arithmetic::traits::{
    ModPowerOf2Cross, ModPowerOf2CrossAssign, ModPowerOf2IsReduced, ModPowerOf2Mul, ModPowerOf2Sub,
};
use malachite_base::num::basic::integers::PrimitiveInt;
use malachite_base::num::conversion::traits::WrappingFrom;
use malachite_base::unsigned_vector::arithmetic::mod_power_of_2_cross::*;

fn assert_valid(v: &NaturalVector, w: &NaturalVector, pow: u64) {
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

// $ab - cd$ modulo $2^k$, where $k$ is `pow`, for $a$, $b$, $c$, and $d$ reduced modulo $2^k$.
fn mod_power_of_2_mul_sub_mul(
    a: &Natural,
    b: &Natural,
    c: &Natural,
    d: &Natural,
    pow: u64,
) -> Natural {
    a.mod_power_of_2_mul(b, pow)
        .mod_power_of_2_sub(c.mod_power_of_2_mul(d, pow), pow)
}

// The cross product modulo $2^k$, where $k$ is `pow`, of the 3-dimensional vectors with elements
// `xs` and `ys`, all reduced modulo $2^k$. When $2^k$ fits in a limb, the work is done in wrapping
// limb arithmetic.
fn naturals_mod_power_of_2_cross(xs: &[Natural], ys: &[Natural], pow: u64) -> Vec<Natural> {
    if pow <= Limb::WIDTH {
        let xs: Vec<Limb> = xs.iter().map(Limb::wrapping_from).collect();
        let ys: Vec<Limb> = ys.iter().map(Limb::wrapping_from).collect();
        return mod_power_of_2_cross_slices(&xs, &ys, pow)
            .into_iter()
            .map(Natural::from)
            .collect();
    }
    let [x0, x1, x2] = xs else { unreachable!() };
    let [y0, y1, y2] = ys else { unreachable!() };
    vec![
        mod_power_of_2_mul_sub_mul(x1, y2, x2, y1, pow),
        mod_power_of_2_mul_sub_mul(x2, y0, x0, y2, pow),
        mod_power_of_2_mul_sub_mul(x0, y1, x1, y0, pow),
    ]
}

impl ModPowerOf2Cross<Self> for NaturalVector {
    type Output = Self;

    /// Computes the cross product of two 3-dimensional [`NaturalVector`]s modulo $2^k$, taking both
    /// by value. The elements of both must already be reduced modulo $2^k$.
    ///
    /// $$
    /// f(v, w, k) = (v_1w_2 - v_2w_1, v_2w_0 - v_0w_2, v_0w_1 - v_1w_0) \\bmod 2^k.
    /// $$
    ///
    /// When $2^k$ fits in a limb, the products and differences are computed in wrapping limb
    /// arithmetic and reduced at the end; otherwise every product and difference is reduced modulo
    /// $2^k$.
    ///
    /// # Worst-case complexity
    /// $T(k) = O(k \log k \log\log k)$
    ///
    /// $M(k) = O(k \log k)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $k$ is `pow`.
    ///
    /// # Panics
    /// Panics if either vector does not have dimension 3, or if any element of either is greater
    /// than or equal to $2^k$.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::ModPowerOf2Cross;
    /// use malachite_nz::natural_vector::NaturalVector;
    ///
    /// assert_eq!(
    ///     NaturalVector::from_str("(1, 2, 3)")
    ///         .unwrap()
    ///         .mod_power_of_2_cross(NaturalVector::from_str("(4, 5, 6)").unwrap(), 4)
    ///         .to_string(),
    ///     "(13, 6, 13)"
    /// );
    /// // The cross product of the first two standard basis vectors is the third.
    /// assert_eq!(
    ///     NaturalVector::from_str("(1, 0, 0)")
    ///         .unwrap()
    ///         .mod_power_of_2_cross(NaturalVector::from_str("(0, 1, 0)").unwrap(), 8)
    ///         .to_string(),
    ///     "(0, 0, 1)"
    /// );
    /// // Modulo 8, the negative components wrap around.
    /// assert_eq!(
    ///     NaturalVector::from_str("(3, 5, 7)")
    ///         .unwrap()
    ///         .mod_power_of_2_cross(NaturalVector::from_str("(7, 5, 3)").unwrap(), 3)
    ///         .to_string(),
    ///     "(4, 0, 4)"
    /// );
    /// ```
    #[inline]
    fn mod_power_of_2_cross(self, other: Self, pow: u64) -> Self {
        assert_valid(&self, &other, pow);
        Self {
            elements: naturals_mod_power_of_2_cross(&self.elements, &other.elements, pow),
        }
    }
}

impl ModPowerOf2Cross<&Self> for NaturalVector {
    type Output = Self;

    /// Computes the cross product of two 3-dimensional [`NaturalVector`]s modulo $2^k$, taking the
    /// first by value and the second by reference. The elements of both must already be reduced
    /// modulo $2^k$.
    ///
    /// $$
    /// f(v, w, k) = (v_1w_2 - v_2w_1, v_2w_0 - v_0w_2, v_0w_1 - v_1w_0) \\bmod 2^k.
    /// $$
    ///
    /// When $2^k$ fits in a limb, the products and differences are computed in wrapping limb
    /// arithmetic and reduced at the end; otherwise every product and difference is reduced modulo
    /// $2^k$.
    ///
    /// # Worst-case complexity
    /// $T(k) = O(k \log k \log\log k)$
    ///
    /// $M(k) = O(k \log k)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $k$ is `pow`.
    ///
    /// # Panics
    /// Panics if either vector does not have dimension 3, or if any element of either is greater
    /// than or equal to $2^k$.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::ModPowerOf2Cross;
    /// use malachite_nz::natural_vector::NaturalVector;
    ///
    /// assert_eq!(
    ///     NaturalVector::from_str("(1, 2, 3)")
    ///         .unwrap()
    ///         .mod_power_of_2_cross(&NaturalVector::from_str("(4, 5, 6)").unwrap(), 4)
    ///         .to_string(),
    ///     "(13, 6, 13)"
    /// );
    /// // The cross product of the first two standard basis vectors is the third.
    /// assert_eq!(
    ///     NaturalVector::from_str("(1, 0, 0)")
    ///         .unwrap()
    ///         .mod_power_of_2_cross(&NaturalVector::from_str("(0, 1, 0)").unwrap(), 8)
    ///         .to_string(),
    ///     "(0, 0, 1)"
    /// );
    /// // Modulo 8, the negative components wrap around.
    /// assert_eq!(
    ///     NaturalVector::from_str("(3, 5, 7)")
    ///         .unwrap()
    ///         .mod_power_of_2_cross(&NaturalVector::from_str("(7, 5, 3)").unwrap(), 3)
    ///         .to_string(),
    ///     "(4, 0, 4)"
    /// );
    /// ```
    #[inline]
    fn mod_power_of_2_cross(self, other: &Self, pow: u64) -> Self {
        assert_valid(&self, other, pow);
        Self {
            elements: naturals_mod_power_of_2_cross(&self.elements, &other.elements, pow),
        }
    }
}

impl ModPowerOf2Cross<NaturalVector> for &NaturalVector {
    type Output = NaturalVector;

    /// Computes the cross product of two 3-dimensional [`NaturalVector`]s modulo $2^k$, taking the
    /// first by reference and the second by value. The elements of both must already be reduced
    /// modulo $2^k$.
    ///
    /// $$
    /// f(v, w, k) = (v_1w_2 - v_2w_1, v_2w_0 - v_0w_2, v_0w_1 - v_1w_0) \\bmod 2^k.
    /// $$
    ///
    /// When $2^k$ fits in a limb, the products and differences are computed in wrapping limb
    /// arithmetic and reduced at the end; otherwise every product and difference is reduced modulo
    /// $2^k$.
    ///
    /// # Worst-case complexity
    /// $T(k) = O(k \log k \log\log k)$
    ///
    /// $M(k) = O(k \log k)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $k$ is `pow`.
    ///
    /// # Panics
    /// Panics if either vector does not have dimension 3, or if any element of either is greater
    /// than or equal to $2^k$.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::ModPowerOf2Cross;
    /// use malachite_nz::natural_vector::NaturalVector;
    ///
    /// assert_eq!(
    ///     (&NaturalVector::from_str("(1, 2, 3)").unwrap())
    ///         .mod_power_of_2_cross(NaturalVector::from_str("(4, 5, 6)").unwrap(), 4)
    ///         .to_string(),
    ///     "(13, 6, 13)"
    /// );
    /// // The cross product of the first two standard basis vectors is the third.
    /// assert_eq!(
    ///     (&NaturalVector::from_str("(1, 0, 0)").unwrap())
    ///         .mod_power_of_2_cross(NaturalVector::from_str("(0, 1, 0)").unwrap(), 8)
    ///         .to_string(),
    ///     "(0, 0, 1)"
    /// );
    /// // Modulo 8, the negative components wrap around.
    /// assert_eq!(
    ///     (&NaturalVector::from_str("(3, 5, 7)").unwrap())
    ///         .mod_power_of_2_cross(NaturalVector::from_str("(7, 5, 3)").unwrap(), 3)
    ///         .to_string(),
    ///     "(4, 0, 4)"
    /// );
    /// ```
    #[inline]
    fn mod_power_of_2_cross(self, other: NaturalVector, pow: u64) -> NaturalVector {
        assert_valid(self, &other, pow);
        NaturalVector {
            elements: naturals_mod_power_of_2_cross(&self.elements, &other.elements, pow),
        }
    }
}

impl ModPowerOf2Cross<&NaturalVector> for &NaturalVector {
    type Output = NaturalVector;

    /// Computes the cross product of two 3-dimensional [`NaturalVector`]s modulo $2^k$, taking both
    /// by reference. The elements of both must already be reduced modulo $2^k$.
    ///
    /// $$
    /// f(v, w, k) = (v_1w_2 - v_2w_1, v_2w_0 - v_0w_2, v_0w_1 - v_1w_0) \\bmod 2^k.
    /// $$
    ///
    /// When $2^k$ fits in a limb, the products and differences are computed in wrapping limb
    /// arithmetic and reduced at the end; otherwise every product and difference is reduced modulo
    /// $2^k$.
    ///
    /// # Worst-case complexity
    /// $T(k) = O(k \log k \log\log k)$
    ///
    /// $M(k) = O(k \log k)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $k$ is `pow`.
    ///
    /// # Panics
    /// Panics if either vector does not have dimension 3, or if any element of either is greater
    /// than or equal to $2^k$.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::ModPowerOf2Cross;
    /// use malachite_nz::natural_vector::NaturalVector;
    ///
    /// assert_eq!(
    ///     (&NaturalVector::from_str("(1, 2, 3)").unwrap())
    ///         .mod_power_of_2_cross(&NaturalVector::from_str("(4, 5, 6)").unwrap(), 4)
    ///         .to_string(),
    ///     "(13, 6, 13)"
    /// );
    /// // The cross product of the first two standard basis vectors is the third.
    /// assert_eq!(
    ///     (&NaturalVector::from_str("(1, 0, 0)").unwrap())
    ///         .mod_power_of_2_cross(&NaturalVector::from_str("(0, 1, 0)").unwrap(), 8)
    ///         .to_string(),
    ///     "(0, 0, 1)"
    /// );
    /// // Modulo 8, the negative components wrap around.
    /// assert_eq!(
    ///     (&NaturalVector::from_str("(3, 5, 7)").unwrap())
    ///         .mod_power_of_2_cross(&NaturalVector::from_str("(7, 5, 3)").unwrap(), 3)
    ///         .to_string(),
    ///     "(4, 0, 4)"
    /// );
    /// ```
    #[inline]
    fn mod_power_of_2_cross(self, other: &NaturalVector, pow: u64) -> NaturalVector {
        assert_valid(self, other, pow);
        NaturalVector {
            elements: naturals_mod_power_of_2_cross(&self.elements, &other.elements, pow),
        }
    }
}

impl ModPowerOf2CrossAssign<Self> for NaturalVector {
    /// Replaces a 3-dimensional [`NaturalVector`] with its cross product with another modulo $2^k$,
    /// taking the other by value. The elements of both must already be reduced modulo $2^k$.
    ///
    /// $$
    /// v \\gets v \\times w \\bmod 2^k.
    /// $$
    ///
    /// When $2^k$ fits in a limb, the products and differences are computed in wrapping limb
    /// arithmetic and reduced at the end; otherwise every product and difference is reduced modulo
    /// $2^k$.
    ///
    /// # Worst-case complexity
    /// $T(k) = O(k \log k \log\log k)$
    ///
    /// $M(k) = O(k \log k)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $k$ is `pow`.
    ///
    /// # Panics
    /// Panics if either vector does not have dimension 3, or if any element of either is greater
    /// than or equal to $2^k$.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::ModPowerOf2CrossAssign;
    /// use malachite_nz::natural_vector::NaturalVector;
    ///
    /// let mut v = NaturalVector::from_str("(1, 2, 3)").unwrap();
    /// v.mod_power_of_2_cross_assign(NaturalVector::from_str("(4, 5, 6)").unwrap(), 4);
    /// assert_eq!(v.to_string(), "(13, 6, 13)");
    ///
    /// // The cross product of the first two standard basis vectors is the third.
    /// let mut v = NaturalVector::from_str("(1, 0, 0)").unwrap();
    /// v.mod_power_of_2_cross_assign(NaturalVector::from_str("(0, 1, 0)").unwrap(), 8);
    /// assert_eq!(v.to_string(), "(0, 0, 1)");
    ///
    /// // Modulo 8, the negative components wrap around.
    /// let mut v = NaturalVector::from_str("(3, 5, 7)").unwrap();
    /// v.mod_power_of_2_cross_assign(NaturalVector::from_str("(7, 5, 3)").unwrap(), 3);
    /// assert_eq!(v.to_string(), "(4, 0, 4)");
    /// ```
    #[inline]
    fn mod_power_of_2_cross_assign(&mut self, other: Self, pow: u64) {
        assert_valid(self, &other, pow);
        self.elements = naturals_mod_power_of_2_cross(&self.elements, &other.elements, pow);
    }
}

impl ModPowerOf2CrossAssign<&Self> for NaturalVector {
    /// Replaces a 3-dimensional [`NaturalVector`] with its cross product with another modulo $2^k$,
    /// taking the other by reference. The elements of both must already be reduced modulo $2^k$.
    ///
    /// $$
    /// v \\gets v \\times w \\bmod 2^k.
    /// $$
    ///
    /// When $2^k$ fits in a limb, the products and differences are computed in wrapping limb
    /// arithmetic and reduced at the end; otherwise every product and difference is reduced modulo
    /// $2^k$.
    ///
    /// # Worst-case complexity
    /// $T(k) = O(k \log k \log\log k)$
    ///
    /// $M(k) = O(k \log k)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $k$ is `pow`.
    ///
    /// # Panics
    /// Panics if either vector does not have dimension 3, or if any element of either is greater
    /// than or equal to $2^k$.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::ModPowerOf2CrossAssign;
    /// use malachite_nz::natural_vector::NaturalVector;
    ///
    /// let mut v = NaturalVector::from_str("(1, 2, 3)").unwrap();
    /// v.mod_power_of_2_cross_assign(&NaturalVector::from_str("(4, 5, 6)").unwrap(), 4);
    /// assert_eq!(v.to_string(), "(13, 6, 13)");
    ///
    /// // The cross product of the first two standard basis vectors is the third.
    /// let mut v = NaturalVector::from_str("(1, 0, 0)").unwrap();
    /// v.mod_power_of_2_cross_assign(&NaturalVector::from_str("(0, 1, 0)").unwrap(), 8);
    /// assert_eq!(v.to_string(), "(0, 0, 1)");
    ///
    /// // Modulo 8, the negative components wrap around.
    /// let mut v = NaturalVector::from_str("(3, 5, 7)").unwrap();
    /// v.mod_power_of_2_cross_assign(&NaturalVector::from_str("(7, 5, 3)").unwrap(), 3);
    /// assert_eq!(v.to_string(), "(4, 0, 4)");
    /// ```
    #[inline]
    fn mod_power_of_2_cross_assign(&mut self, other: &Self, pow: u64) {
        assert_valid(self, other, pow);
        self.elements = naturals_mod_power_of_2_cross(&self.elements, &other.elements, pow);
    }
}
