// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::natural::Natural;
use crate::natural::arithmetic::mod_mul::ModMulData;
use crate::natural_vector::NaturalVector;
use crate::platform::Limb;
use alloc::vec;
use alloc::vec::Vec;
use malachite_base::num::arithmetic::traits::{
    ModCross, ModCrossAssign, ModIsReduced, ModMulPrecomputed, ModSub,
};
use malachite_base::num::basic::integers::PrimitiveInt;
use malachite_base::num::conversion::traits::WrappingFrom;
use malachite_base::num::logic::traits::SignificantBits;
use malachite_base::unsigned_vector::arithmetic::mod_cross::*;

fn assert_valid(v: &NaturalVector, w: &NaturalVector, m: &Natural) {
    assert!(
        v.elements.len() == 3 && w.elements.len() == 3,
        "the cross product is only defined for 3-dimensional vectors"
    );
    assert!(
        v.mod_is_reduced(m),
        "self must be reduced mod m, but {v} has an element >= {m}"
    );
    assert!(
        w.mod_is_reduced(m),
        "other must be reduced mod m, but {w} has an element >= {m}"
    );
}

// The cross product modulo `m` of the 3-dimensional vectors with elements `xs` and `ys`, all
// reduced modulo `m`. When `m` fits in a limb, the work is done with limbs; otherwise each element
// is a 2-by-2 determinant, $ab - cd$, whose products share one set of precomputed data for
// multiplying modulo `m`.
fn naturals_mod_cross(xs: &[Natural], ys: &[Natural], m: &Natural) -> Vec<Natural> {
    if m.significant_bits() <= Limb::WIDTH {
        let xs: Vec<Limb> = xs.iter().map(Limb::wrapping_from).collect();
        let ys: Vec<Limb> = ys.iter().map(Limb::wrapping_from).collect();
        return mod_cross_slices(&xs, &ys, Limb::wrapping_from(m))
            .into_iter()
            .map(Natural::from)
            .collect();
    }
    let [x0, x1, x2] = xs else { unreachable!() };
    let [y0, y1, y2] = ys else { unreachable!() };
    let data: ModMulData =
        <Natural as ModMulPrecomputed<&Natural, &Natural>>::precompute_mod_mul_data(&m);
    let determinant = |a: &Natural, b: &Natural, c: &Natural, d: &Natural| {
        a.mod_mul_precomputed(b, m, &data)
            .mod_sub(c.mod_mul_precomputed(d, m, &data), m)
    };
    vec![determinant(x1, y2, x2, y1), determinant(x2, y0, x0, y2), determinant(x0, y1, x1, y0)]
}

impl ModCross<Self, Natural> for NaturalVector {
    type Output = Self;

    /// Computes the cross product of two 3-dimensional [`NaturalVector`]s modulo $m$, taking all
    /// three by value. The elements of both must already be reduced modulo $m$.
    ///
    /// $$
    /// f(v, w, m) = (v_1w_2 - v_2w_1, v_2w_0 - v_0w_2, v_0w_1 - v_1w_0) \\bmod m.
    /// $$
    ///
    /// When $m$ fits in a limb, the work is done with limbs; otherwise the six products share one
    /// set of precomputed data for multiplying modulo $m$. Nothing is divided modulo $m$, so any
    /// modulus works, prime or not.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `m.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `m` is 0, if either vector does not have dimension 3, or if any element of either
    /// is greater than or equal to `m`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::ModCross;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_vector::NaturalVector;
    ///
    /// assert_eq!(
    ///     NaturalVector::from_str("(1, 2, 3)")
    ///         .unwrap()
    ///         .mod_cross(
    ///             NaturalVector::from_str("(4, 5, 6)").unwrap(),
    ///             Natural::from(7u32)
    ///         )
    ///         .to_string(),
    ///     "(4, 6, 4)"
    /// );
    /// // The cross product of the first two standard basis vectors is the third.
    /// assert_eq!(
    ///     NaturalVector::from_str("(1, 0, 0)")
    ///         .unwrap()
    ///         .mod_cross(
    ///             NaturalVector::from_str("(0, 1, 0)").unwrap(),
    ///             Natural::from(5u32)
    ///         )
    ///         .to_string(),
    ///     "(0, 0, 1)"
    /// );
    /// // Modulo 10, the negative components wrap around.
    /// assert_eq!(
    ///     NaturalVector::from_str("(3, 5, 7)")
    ///         .unwrap()
    ///         .mod_cross(
    ///             NaturalVector::from_str("(7, 5, 3)").unwrap(),
    ///             Natural::from(10u32)
    ///         )
    ///         .to_string(),
    ///     "(0, 0, 0)"
    /// );
    /// ```
    #[inline]
    fn mod_cross(self, other: Self, m: Natural) -> Self {
        assert_valid(&self, &other, &m);
        Self {
            elements: naturals_mod_cross(&self.elements, &other.elements, &m),
        }
    }
}

impl ModCross<Self, &Natural> for NaturalVector {
    type Output = Self;

    /// Computes the cross product of two 3-dimensional [`NaturalVector`]s modulo $m$, taking the
    /// first vector by value, the second by value, and $m$ by reference. The elements of both must
    /// already be reduced modulo $m$.
    ///
    /// $$
    /// f(v, w, m) = (v_1w_2 - v_2w_1, v_2w_0 - v_0w_2, v_0w_1 - v_1w_0) \\bmod m.
    /// $$
    ///
    /// When $m$ fits in a limb, the work is done with limbs; otherwise the six products share one
    /// set of precomputed data for multiplying modulo $m$. Nothing is divided modulo $m$, so any
    /// modulus works, prime or not.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `m.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `m` is 0, if either vector does not have dimension 3, or if any element of either
    /// is greater than or equal to `m`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::ModCross;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_vector::NaturalVector;
    ///
    /// assert_eq!(
    ///     NaturalVector::from_str("(1, 2, 3)")
    ///         .unwrap()
    ///         .mod_cross(
    ///             NaturalVector::from_str("(4, 5, 6)").unwrap(),
    ///             &Natural::from(7u32)
    ///         )
    ///         .to_string(),
    ///     "(4, 6, 4)"
    /// );
    /// // The cross product of the first two standard basis vectors is the third.
    /// assert_eq!(
    ///     NaturalVector::from_str("(1, 0, 0)")
    ///         .unwrap()
    ///         .mod_cross(
    ///             NaturalVector::from_str("(0, 1, 0)").unwrap(),
    ///             &Natural::from(5u32)
    ///         )
    ///         .to_string(),
    ///     "(0, 0, 1)"
    /// );
    /// // Modulo 10, the negative components wrap around.
    /// assert_eq!(
    ///     NaturalVector::from_str("(3, 5, 7)")
    ///         .unwrap()
    ///         .mod_cross(
    ///             NaturalVector::from_str("(7, 5, 3)").unwrap(),
    ///             &Natural::from(10u32)
    ///         )
    ///         .to_string(),
    ///     "(0, 0, 0)"
    /// );
    /// ```
    #[inline]
    fn mod_cross(self, other: Self, m: &Natural) -> Self {
        assert_valid(&self, &other, m);
        Self {
            elements: naturals_mod_cross(&self.elements, &other.elements, m),
        }
    }
}

impl ModCross<&Self, Natural> for NaturalVector {
    type Output = Self;

    /// Computes the cross product of two 3-dimensional [`NaturalVector`]s modulo $m$, taking the
    /// first vector by value, the second by reference, and $m$ by value. The elements of both must
    /// already be reduced modulo $m$.
    ///
    /// $$
    /// f(v, w, m) = (v_1w_2 - v_2w_1, v_2w_0 - v_0w_2, v_0w_1 - v_1w_0) \\bmod m.
    /// $$
    ///
    /// When $m$ fits in a limb, the work is done with limbs; otherwise the six products share one
    /// set of precomputed data for multiplying modulo $m$. Nothing is divided modulo $m$, so any
    /// modulus works, prime or not.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `m.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `m` is 0, if either vector does not have dimension 3, or if any element of either
    /// is greater than or equal to `m`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::ModCross;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_vector::NaturalVector;
    ///
    /// assert_eq!(
    ///     NaturalVector::from_str("(1, 2, 3)")
    ///         .unwrap()
    ///         .mod_cross(
    ///             &NaturalVector::from_str("(4, 5, 6)").unwrap(),
    ///             Natural::from(7u32)
    ///         )
    ///         .to_string(),
    ///     "(4, 6, 4)"
    /// );
    /// // The cross product of the first two standard basis vectors is the third.
    /// assert_eq!(
    ///     NaturalVector::from_str("(1, 0, 0)")
    ///         .unwrap()
    ///         .mod_cross(
    ///             &NaturalVector::from_str("(0, 1, 0)").unwrap(),
    ///             Natural::from(5u32)
    ///         )
    ///         .to_string(),
    ///     "(0, 0, 1)"
    /// );
    /// // Modulo 10, the negative components wrap around.
    /// assert_eq!(
    ///     NaturalVector::from_str("(3, 5, 7)")
    ///         .unwrap()
    ///         .mod_cross(
    ///             &NaturalVector::from_str("(7, 5, 3)").unwrap(),
    ///             Natural::from(10u32)
    ///         )
    ///         .to_string(),
    ///     "(0, 0, 0)"
    /// );
    /// ```
    #[inline]
    fn mod_cross(self, other: &Self, m: Natural) -> Self {
        assert_valid(&self, other, &m);
        Self {
            elements: naturals_mod_cross(&self.elements, &other.elements, &m),
        }
    }
}

impl ModCross<&Self, &Natural> for NaturalVector {
    type Output = Self;

    /// Computes the cross product of two 3-dimensional [`NaturalVector`]s modulo $m$, taking the
    /// first vector by value, the second by reference, and $m$ by reference. The elements of both
    /// must already be reduced modulo $m$.
    ///
    /// $$
    /// f(v, w, m) = (v_1w_2 - v_2w_1, v_2w_0 - v_0w_2, v_0w_1 - v_1w_0) \\bmod m.
    /// $$
    ///
    /// When $m$ fits in a limb, the work is done with limbs; otherwise the six products share one
    /// set of precomputed data for multiplying modulo $m$. Nothing is divided modulo $m$, so any
    /// modulus works, prime or not.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `m.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `m` is 0, if either vector does not have dimension 3, or if any element of either
    /// is greater than or equal to `m`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::ModCross;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_vector::NaturalVector;
    ///
    /// assert_eq!(
    ///     NaturalVector::from_str("(1, 2, 3)")
    ///         .unwrap()
    ///         .mod_cross(
    ///             &NaturalVector::from_str("(4, 5, 6)").unwrap(),
    ///             &Natural::from(7u32)
    ///         )
    ///         .to_string(),
    ///     "(4, 6, 4)"
    /// );
    /// // The cross product of the first two standard basis vectors is the third.
    /// assert_eq!(
    ///     NaturalVector::from_str("(1, 0, 0)")
    ///         .unwrap()
    ///         .mod_cross(
    ///             &NaturalVector::from_str("(0, 1, 0)").unwrap(),
    ///             &Natural::from(5u32)
    ///         )
    ///         .to_string(),
    ///     "(0, 0, 1)"
    /// );
    /// // Modulo 10, the negative components wrap around.
    /// assert_eq!(
    ///     NaturalVector::from_str("(3, 5, 7)")
    ///         .unwrap()
    ///         .mod_cross(
    ///             &NaturalVector::from_str("(7, 5, 3)").unwrap(),
    ///             &Natural::from(10u32)
    ///         )
    ///         .to_string(),
    ///     "(0, 0, 0)"
    /// );
    /// ```
    #[inline]
    fn mod_cross(self, other: &Self, m: &Natural) -> Self {
        assert_valid(&self, other, m);
        Self {
            elements: naturals_mod_cross(&self.elements, &other.elements, m),
        }
    }
}

impl ModCross<NaturalVector, Natural> for &NaturalVector {
    type Output = NaturalVector;

    /// Computes the cross product of two 3-dimensional [`NaturalVector`]s modulo $m$, taking the
    /// first vector by reference, the second by value, and $m$ by value. The elements of both must
    /// already be reduced modulo $m$.
    ///
    /// $$
    /// f(v, w, m) = (v_1w_2 - v_2w_1, v_2w_0 - v_0w_2, v_0w_1 - v_1w_0) \\bmod m.
    /// $$
    ///
    /// When $m$ fits in a limb, the work is done with limbs; otherwise the six products share one
    /// set of precomputed data for multiplying modulo $m$. Nothing is divided modulo $m$, so any
    /// modulus works, prime or not.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `m.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `m` is 0, if either vector does not have dimension 3, or if any element of either
    /// is greater than or equal to `m`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::ModCross;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_vector::NaturalVector;
    ///
    /// assert_eq!(
    ///     (&NaturalVector::from_str("(1, 2, 3)").unwrap())
    ///         .mod_cross(
    ///             NaturalVector::from_str("(4, 5, 6)").unwrap(),
    ///             Natural::from(7u32)
    ///         )
    ///         .to_string(),
    ///     "(4, 6, 4)"
    /// );
    /// // The cross product of the first two standard basis vectors is the third.
    /// assert_eq!(
    ///     (&NaturalVector::from_str("(1, 0, 0)").unwrap())
    ///         .mod_cross(
    ///             NaturalVector::from_str("(0, 1, 0)").unwrap(),
    ///             Natural::from(5u32)
    ///         )
    ///         .to_string(),
    ///     "(0, 0, 1)"
    /// );
    /// // Modulo 10, the negative components wrap around.
    /// assert_eq!(
    ///     (&NaturalVector::from_str("(3, 5, 7)").unwrap())
    ///         .mod_cross(
    ///             NaturalVector::from_str("(7, 5, 3)").unwrap(),
    ///             Natural::from(10u32)
    ///         )
    ///         .to_string(),
    ///     "(0, 0, 0)"
    /// );
    /// ```
    #[inline]
    fn mod_cross(self, other: NaturalVector, m: Natural) -> NaturalVector {
        assert_valid(self, &other, &m);
        NaturalVector {
            elements: naturals_mod_cross(&self.elements, &other.elements, &m),
        }
    }
}

impl ModCross<NaturalVector, &Natural> for &NaturalVector {
    type Output = NaturalVector;

    /// Computes the cross product of two 3-dimensional [`NaturalVector`]s modulo $m$, taking the
    /// first vector by reference, the second by value, and $m$ by reference. The elements of both
    /// must already be reduced modulo $m$.
    ///
    /// $$
    /// f(v, w, m) = (v_1w_2 - v_2w_1, v_2w_0 - v_0w_2, v_0w_1 - v_1w_0) \\bmod m.
    /// $$
    ///
    /// When $m$ fits in a limb, the work is done with limbs; otherwise the six products share one
    /// set of precomputed data for multiplying modulo $m$. Nothing is divided modulo $m$, so any
    /// modulus works, prime or not.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `m.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `m` is 0, if either vector does not have dimension 3, or if any element of either
    /// is greater than or equal to `m`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::ModCross;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_vector::NaturalVector;
    ///
    /// assert_eq!(
    ///     (&NaturalVector::from_str("(1, 2, 3)").unwrap())
    ///         .mod_cross(
    ///             NaturalVector::from_str("(4, 5, 6)").unwrap(),
    ///             &Natural::from(7u32)
    ///         )
    ///         .to_string(),
    ///     "(4, 6, 4)"
    /// );
    /// // The cross product of the first two standard basis vectors is the third.
    /// assert_eq!(
    ///     (&NaturalVector::from_str("(1, 0, 0)").unwrap())
    ///         .mod_cross(
    ///             NaturalVector::from_str("(0, 1, 0)").unwrap(),
    ///             &Natural::from(5u32)
    ///         )
    ///         .to_string(),
    ///     "(0, 0, 1)"
    /// );
    /// // Modulo 10, the negative components wrap around.
    /// assert_eq!(
    ///     (&NaturalVector::from_str("(3, 5, 7)").unwrap())
    ///         .mod_cross(
    ///             NaturalVector::from_str("(7, 5, 3)").unwrap(),
    ///             &Natural::from(10u32)
    ///         )
    ///         .to_string(),
    ///     "(0, 0, 0)"
    /// );
    /// ```
    #[inline]
    fn mod_cross(self, other: NaturalVector, m: &Natural) -> NaturalVector {
        assert_valid(self, &other, m);
        NaturalVector {
            elements: naturals_mod_cross(&self.elements, &other.elements, m),
        }
    }
}

impl ModCross<&NaturalVector, Natural> for &NaturalVector {
    type Output = NaturalVector;

    /// Computes the cross product of two 3-dimensional [`NaturalVector`]s modulo $m$, taking the
    /// first vector by reference, the second by reference, and $m$ by value. The elements of both
    /// must already be reduced modulo $m$.
    ///
    /// $$
    /// f(v, w, m) = (v_1w_2 - v_2w_1, v_2w_0 - v_0w_2, v_0w_1 - v_1w_0) \\bmod m.
    /// $$
    ///
    /// When $m$ fits in a limb, the work is done with limbs; otherwise the six products share one
    /// set of precomputed data for multiplying modulo $m$. Nothing is divided modulo $m$, so any
    /// modulus works, prime or not.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `m.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `m` is 0, if either vector does not have dimension 3, or if any element of either
    /// is greater than or equal to `m`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::ModCross;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_vector::NaturalVector;
    ///
    /// assert_eq!(
    ///     (&NaturalVector::from_str("(1, 2, 3)").unwrap())
    ///         .mod_cross(
    ///             &NaturalVector::from_str("(4, 5, 6)").unwrap(),
    ///             Natural::from(7u32)
    ///         )
    ///         .to_string(),
    ///     "(4, 6, 4)"
    /// );
    /// // The cross product of the first two standard basis vectors is the third.
    /// assert_eq!(
    ///     (&NaturalVector::from_str("(1, 0, 0)").unwrap())
    ///         .mod_cross(
    ///             &NaturalVector::from_str("(0, 1, 0)").unwrap(),
    ///             Natural::from(5u32)
    ///         )
    ///         .to_string(),
    ///     "(0, 0, 1)"
    /// );
    /// // Modulo 10, the negative components wrap around.
    /// assert_eq!(
    ///     (&NaturalVector::from_str("(3, 5, 7)").unwrap())
    ///         .mod_cross(
    ///             &NaturalVector::from_str("(7, 5, 3)").unwrap(),
    ///             Natural::from(10u32)
    ///         )
    ///         .to_string(),
    ///     "(0, 0, 0)"
    /// );
    /// ```
    #[inline]
    fn mod_cross(self, other: &NaturalVector, m: Natural) -> NaturalVector {
        assert_valid(self, other, &m);
        NaturalVector {
            elements: naturals_mod_cross(&self.elements, &other.elements, &m),
        }
    }
}

impl ModCross<&NaturalVector, &Natural> for &NaturalVector {
    type Output = NaturalVector;

    /// Computes the cross product of two 3-dimensional [`NaturalVector`]s modulo $m$, taking all
    /// three by reference. The elements of both must already be reduced modulo $m$.
    ///
    /// $$
    /// f(v, w, m) = (v_1w_2 - v_2w_1, v_2w_0 - v_0w_2, v_0w_1 - v_1w_0) \\bmod m.
    /// $$
    ///
    /// When $m$ fits in a limb, the work is done with limbs; otherwise the six products share one
    /// set of precomputed data for multiplying modulo $m$. Nothing is divided modulo $m$, so any
    /// modulus works, prime or not.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `m.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `m` is 0, if either vector does not have dimension 3, or if any element of either
    /// is greater than or equal to `m`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::ModCross;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_vector::NaturalVector;
    ///
    /// assert_eq!(
    ///     (&NaturalVector::from_str("(1, 2, 3)").unwrap())
    ///         .mod_cross(
    ///             &NaturalVector::from_str("(4, 5, 6)").unwrap(),
    ///             &Natural::from(7u32)
    ///         )
    ///         .to_string(),
    ///     "(4, 6, 4)"
    /// );
    /// // The cross product of the first two standard basis vectors is the third.
    /// assert_eq!(
    ///     (&NaturalVector::from_str("(1, 0, 0)").unwrap())
    ///         .mod_cross(
    ///             &NaturalVector::from_str("(0, 1, 0)").unwrap(),
    ///             &Natural::from(5u32)
    ///         )
    ///         .to_string(),
    ///     "(0, 0, 1)"
    /// );
    /// // Modulo 10, the negative components wrap around.
    /// assert_eq!(
    ///     (&NaturalVector::from_str("(3, 5, 7)").unwrap())
    ///         .mod_cross(
    ///             &NaturalVector::from_str("(7, 5, 3)").unwrap(),
    ///             &Natural::from(10u32)
    ///         )
    ///         .to_string(),
    ///     "(0, 0, 0)"
    /// );
    /// ```
    #[inline]
    fn mod_cross(self, other: &NaturalVector, m: &Natural) -> NaturalVector {
        assert_valid(self, other, m);
        NaturalVector {
            elements: naturals_mod_cross(&self.elements, &other.elements, m),
        }
    }
}

impl ModCrossAssign<Self, Natural> for NaturalVector {
    /// Replaces a 3-dimensional [`NaturalVector`] with its cross product with another modulo $m$,
    /// taking the other by value and $m$ by value. The elements of both must already be reduced
    /// modulo $m$.
    ///
    /// $$
    /// v \\gets v \\times w \\bmod m.
    /// $$
    ///
    /// When $m$ fits in a limb, the work is done with limbs; otherwise the six products share one
    /// set of precomputed data for multiplying modulo $m$. Nothing is divided modulo $m$, so any
    /// modulus works, prime or not.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `m.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `m` is 0, if either vector does not have dimension 3, or if any element of either
    /// is greater than or equal to `m`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::ModCrossAssign;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_vector::NaturalVector;
    ///
    /// let mut v = NaturalVector::from_str("(1, 2, 3)").unwrap();
    /// v.mod_cross_assign(
    ///     NaturalVector::from_str("(4, 5, 6)").unwrap(),
    ///     Natural::from(7u32),
    /// );
    /// assert_eq!(v.to_string(), "(4, 6, 4)");
    ///
    /// // The cross product of the first two standard basis vectors is the third.
    /// let mut v = NaturalVector::from_str("(1, 0, 0)").unwrap();
    /// v.mod_cross_assign(
    ///     NaturalVector::from_str("(0, 1, 0)").unwrap(),
    ///     Natural::from(5u32),
    /// );
    /// assert_eq!(v.to_string(), "(0, 0, 1)");
    ///
    /// // Modulo 10, the negative components wrap around.
    /// let mut v = NaturalVector::from_str("(3, 5, 7)").unwrap();
    /// v.mod_cross_assign(
    ///     NaturalVector::from_str("(7, 5, 3)").unwrap(),
    ///     Natural::from(10u32),
    /// );
    /// assert_eq!(v.to_string(), "(0, 0, 0)");
    /// ```
    #[inline]
    fn mod_cross_assign(&mut self, other: Self, m: Natural) {
        assert_valid(self, &other, &m);
        self.elements = naturals_mod_cross(&self.elements, &other.elements, &m);
    }
}

impl ModCrossAssign<Self, &Natural> for NaturalVector {
    /// Replaces a 3-dimensional [`NaturalVector`] with its cross product with another modulo $m$,
    /// taking the other by value and $m$ by reference. The elements of both must already be reduced
    /// modulo $m$.
    ///
    /// $$
    /// v \\gets v \\times w \\bmod m.
    /// $$
    ///
    /// When $m$ fits in a limb, the work is done with limbs; otherwise the six products share one
    /// set of precomputed data for multiplying modulo $m$. Nothing is divided modulo $m$, so any
    /// modulus works, prime or not.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `m.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `m` is 0, if either vector does not have dimension 3, or if any element of either
    /// is greater than or equal to `m`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::ModCrossAssign;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_vector::NaturalVector;
    ///
    /// let mut v = NaturalVector::from_str("(1, 2, 3)").unwrap();
    /// v.mod_cross_assign(
    ///     NaturalVector::from_str("(4, 5, 6)").unwrap(),
    ///     &Natural::from(7u32),
    /// );
    /// assert_eq!(v.to_string(), "(4, 6, 4)");
    ///
    /// // The cross product of the first two standard basis vectors is the third.
    /// let mut v = NaturalVector::from_str("(1, 0, 0)").unwrap();
    /// v.mod_cross_assign(
    ///     NaturalVector::from_str("(0, 1, 0)").unwrap(),
    ///     &Natural::from(5u32),
    /// );
    /// assert_eq!(v.to_string(), "(0, 0, 1)");
    ///
    /// // Modulo 10, the negative components wrap around.
    /// let mut v = NaturalVector::from_str("(3, 5, 7)").unwrap();
    /// v.mod_cross_assign(
    ///     NaturalVector::from_str("(7, 5, 3)").unwrap(),
    ///     &Natural::from(10u32),
    /// );
    /// assert_eq!(v.to_string(), "(0, 0, 0)");
    /// ```
    #[inline]
    fn mod_cross_assign(&mut self, other: Self, m: &Natural) {
        assert_valid(self, &other, m);
        self.elements = naturals_mod_cross(&self.elements, &other.elements, m);
    }
}

impl ModCrossAssign<&Self, Natural> for NaturalVector {
    /// Replaces a 3-dimensional [`NaturalVector`] with its cross product with another modulo $m$,
    /// taking the other by reference and $m$ by value. The elements of both must already be reduced
    /// modulo $m$.
    ///
    /// $$
    /// v \\gets v \\times w \\bmod m.
    /// $$
    ///
    /// When $m$ fits in a limb, the work is done with limbs; otherwise the six products share one
    /// set of precomputed data for multiplying modulo $m$. Nothing is divided modulo $m$, so any
    /// modulus works, prime or not.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `m.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `m` is 0, if either vector does not have dimension 3, or if any element of either
    /// is greater than or equal to `m`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::ModCrossAssign;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_vector::NaturalVector;
    ///
    /// let mut v = NaturalVector::from_str("(1, 2, 3)").unwrap();
    /// v.mod_cross_assign(
    ///     &NaturalVector::from_str("(4, 5, 6)").unwrap(),
    ///     Natural::from(7u32),
    /// );
    /// assert_eq!(v.to_string(), "(4, 6, 4)");
    ///
    /// // The cross product of the first two standard basis vectors is the third.
    /// let mut v = NaturalVector::from_str("(1, 0, 0)").unwrap();
    /// v.mod_cross_assign(
    ///     &NaturalVector::from_str("(0, 1, 0)").unwrap(),
    ///     Natural::from(5u32),
    /// );
    /// assert_eq!(v.to_string(), "(0, 0, 1)");
    ///
    /// // Modulo 10, the negative components wrap around.
    /// let mut v = NaturalVector::from_str("(3, 5, 7)").unwrap();
    /// v.mod_cross_assign(
    ///     &NaturalVector::from_str("(7, 5, 3)").unwrap(),
    ///     Natural::from(10u32),
    /// );
    /// assert_eq!(v.to_string(), "(0, 0, 0)");
    /// ```
    #[inline]
    fn mod_cross_assign(&mut self, other: &Self, m: Natural) {
        assert_valid(self, other, &m);
        self.elements = naturals_mod_cross(&self.elements, &other.elements, &m);
    }
}

impl ModCrossAssign<&Self, &Natural> for NaturalVector {
    /// Replaces a 3-dimensional [`NaturalVector`] with its cross product with another modulo $m$,
    /// taking the other by reference and $m$ by reference. The elements of both must already be
    /// reduced modulo $m$.
    ///
    /// $$
    /// v \\gets v \\times w \\bmod m.
    /// $$
    ///
    /// When $m$ fits in a limb, the work is done with limbs; otherwise the six products share one
    /// set of precomputed data for multiplying modulo $m$. Nothing is divided modulo $m$, so any
    /// modulus works, prime or not.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `m.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `m` is 0, if either vector does not have dimension 3, or if any element of either
    /// is greater than or equal to `m`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::ModCrossAssign;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_vector::NaturalVector;
    ///
    /// let mut v = NaturalVector::from_str("(1, 2, 3)").unwrap();
    /// v.mod_cross_assign(
    ///     &NaturalVector::from_str("(4, 5, 6)").unwrap(),
    ///     &Natural::from(7u32),
    /// );
    /// assert_eq!(v.to_string(), "(4, 6, 4)");
    ///
    /// // The cross product of the first two standard basis vectors is the third.
    /// let mut v = NaturalVector::from_str("(1, 0, 0)").unwrap();
    /// v.mod_cross_assign(
    ///     &NaturalVector::from_str("(0, 1, 0)").unwrap(),
    ///     &Natural::from(5u32),
    /// );
    /// assert_eq!(v.to_string(), "(0, 0, 1)");
    ///
    /// // Modulo 10, the negative components wrap around.
    /// let mut v = NaturalVector::from_str("(3, 5, 7)").unwrap();
    /// v.mod_cross_assign(
    ///     &NaturalVector::from_str("(7, 5, 3)").unwrap(),
    ///     &Natural::from(10u32),
    /// );
    /// assert_eq!(v.to_string(), "(0, 0, 0)");
    /// ```
    #[inline]
    fn mod_cross_assign(&mut self, other: &Self, m: &Natural) {
        assert_valid(self, other, m);
        self.elements = naturals_mod_cross(&self.elements, &other.elements, m);
    }
}
