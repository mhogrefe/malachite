// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::natural::Natural;
use crate::natural_vector::NaturalVector;
use crate::natural_vector::arithmetic::mod_dot::mod_dot_general;
use malachite_base::num::arithmetic::traits::{ModAddDot, ModAddDotAssign};

impl ModAddDot<NaturalVector, NaturalVector, Self> for Natural {
    type Output = Self;

    /// Adds the dot product of two [`NaturalVector`]s to a [`Natural`] modulo $m$, taking the
    /// number by value, the first vector by value, and the second by value, and the modulus by
    /// value. The number and the elements must already be reduced.
    ///
    /// $$
    /// f(x, v, w, m) = x + \sum_{i=0}^{n-1} v_iw_i \bmod m,
    /// $$
    ///
    /// where $n$ is the dimension of $v$ and $w$.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(nm \log m \log\log m)$
    ///
    /// $M(m) = O(m \log m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is the dimension of the vectors, and $m$ is
    /// `m.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `m` is zero, if the vectors have different dimensions, or if `x` or any element is
    /// greater than or equal to `m`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::ModAddDot;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_vector::NaturalVector;
    ///
    /// let v = NaturalVector::from_str("(1, 2, 3)").unwrap();
    /// let w = NaturalVector::from_str("(4, 5, 7)").unwrap();
    /// assert_eq!(
    ///     Natural::from(3u32).mod_add_dot(v.clone(), w.clone(), Natural::from(11u32)),
    ///     5u32
    /// );
    /// ```
    #[inline]
    fn mod_add_dot(self, v: NaturalVector, w: NaturalVector, m: Self) -> Self {
        mod_dot_general(&self, false, &v, &w, false, &m)
    }
}

impl ModAddDot<NaturalVector, NaturalVector, &Self> for Natural {
    type Output = Self;

    /// Adds the dot product of two [`NaturalVector`]s to a [`Natural`] modulo $m$, taking the
    /// number by value, the first vector by value, and the second by value, and the modulus by
    /// reference. The number and the elements must already be reduced.
    ///
    /// $$
    /// f(x, v, w, m) = x + \sum_{i=0}^{n-1} v_iw_i \bmod m,
    /// $$
    ///
    /// where $n$ is the dimension of $v$ and $w$.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(nm \log m \log\log m)$
    ///
    /// $M(m) = O(m \log m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is the dimension of the vectors, and $m$ is
    /// `m.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `m` is zero, if the vectors have different dimensions, or if `x` or any element is
    /// greater than or equal to `m`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::ModAddDot;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_vector::NaturalVector;
    ///
    /// let v = NaturalVector::from_str("(1, 2, 3)").unwrap();
    /// let w = NaturalVector::from_str("(4, 5, 7)").unwrap();
    /// assert_eq!(
    ///     Natural::from(3u32).mod_add_dot(v.clone(), w.clone(), &Natural::from(11u32)),
    ///     5u32
    /// );
    /// ```
    #[inline]
    fn mod_add_dot(self, v: NaturalVector, w: NaturalVector, m: &Self) -> Self {
        mod_dot_general(&self, false, &v, &w, false, m)
    }
}

impl ModAddDot<NaturalVector, &NaturalVector, Self> for Natural {
    type Output = Self;

    /// Adds the dot product of two [`NaturalVector`]s to a [`Natural`] modulo $m$, taking the
    /// number by value, the first vector by value, and the second by reference, and the modulus by
    /// value. The number and the elements must already be reduced.
    ///
    /// $$
    /// f(x, v, w, m) = x + \sum_{i=0}^{n-1} v_iw_i \bmod m,
    /// $$
    ///
    /// where $n$ is the dimension of $v$ and $w$.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(nm \log m \log\log m)$
    ///
    /// $M(m) = O(m \log m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is the dimension of the vectors, and $m$ is
    /// `m.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `m` is zero, if the vectors have different dimensions, or if `x` or any element is
    /// greater than or equal to `m`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::ModAddDot;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_vector::NaturalVector;
    ///
    /// let v = NaturalVector::from_str("(1, 2, 3)").unwrap();
    /// let w = NaturalVector::from_str("(4, 5, 7)").unwrap();
    /// assert_eq!(
    ///     Natural::from(3u32).mod_add_dot(v.clone(), &w, Natural::from(11u32)),
    ///     5u32
    /// );
    /// ```
    #[inline]
    fn mod_add_dot(self, v: NaturalVector, w: &NaturalVector, m: Self) -> Self {
        mod_dot_general(&self, false, &v, w, false, &m)
    }
}

impl ModAddDot<NaturalVector, &NaturalVector, &Self> for Natural {
    type Output = Self;

    /// Adds the dot product of two [`NaturalVector`]s to a [`Natural`] modulo $m$, taking the
    /// number by value, the first vector by value, and the second by reference, and the modulus by
    /// reference. The number and the elements must already be reduced.
    ///
    /// $$
    /// f(x, v, w, m) = x + \sum_{i=0}^{n-1} v_iw_i \bmod m,
    /// $$
    ///
    /// where $n$ is the dimension of $v$ and $w$.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(nm \log m \log\log m)$
    ///
    /// $M(m) = O(m \log m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is the dimension of the vectors, and $m$ is
    /// `m.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `m` is zero, if the vectors have different dimensions, or if `x` or any element is
    /// greater than or equal to `m`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::ModAddDot;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_vector::NaturalVector;
    ///
    /// let v = NaturalVector::from_str("(1, 2, 3)").unwrap();
    /// let w = NaturalVector::from_str("(4, 5, 7)").unwrap();
    /// assert_eq!(
    ///     Natural::from(3u32).mod_add_dot(v.clone(), &w, &Natural::from(11u32)),
    ///     5u32
    /// );
    /// ```
    #[inline]
    fn mod_add_dot(self, v: NaturalVector, w: &NaturalVector, m: &Self) -> Self {
        mod_dot_general(&self, false, &v, w, false, m)
    }
}

impl ModAddDot<&NaturalVector, NaturalVector, Self> for Natural {
    type Output = Self;

    /// Adds the dot product of two [`NaturalVector`]s to a [`Natural`] modulo $m$, taking the
    /// number by value, the first vector by reference, and the second by value, and the modulus by
    /// value. The number and the elements must already be reduced.
    ///
    /// $$
    /// f(x, v, w, m) = x + \sum_{i=0}^{n-1} v_iw_i \bmod m,
    /// $$
    ///
    /// where $n$ is the dimension of $v$ and $w$.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(nm \log m \log\log m)$
    ///
    /// $M(m) = O(m \log m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is the dimension of the vectors, and $m$ is
    /// `m.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `m` is zero, if the vectors have different dimensions, or if `x` or any element is
    /// greater than or equal to `m`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::ModAddDot;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_vector::NaturalVector;
    ///
    /// let v = NaturalVector::from_str("(1, 2, 3)").unwrap();
    /// let w = NaturalVector::from_str("(4, 5, 7)").unwrap();
    /// assert_eq!(
    ///     Natural::from(3u32).mod_add_dot(&v, w.clone(), Natural::from(11u32)),
    ///     5u32
    /// );
    /// ```
    #[inline]
    fn mod_add_dot(self, v: &NaturalVector, w: NaturalVector, m: Self) -> Self {
        mod_dot_general(&self, false, v, &w, false, &m)
    }
}

impl ModAddDot<&NaturalVector, NaturalVector, &Self> for Natural {
    type Output = Self;

    /// Adds the dot product of two [`NaturalVector`]s to a [`Natural`] modulo $m$, taking the
    /// number by value, the first vector by reference, and the second by value, and the modulus by
    /// reference. The number and the elements must already be reduced.
    ///
    /// $$
    /// f(x, v, w, m) = x + \sum_{i=0}^{n-1} v_iw_i \bmod m,
    /// $$
    ///
    /// where $n$ is the dimension of $v$ and $w$.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(nm \log m \log\log m)$
    ///
    /// $M(m) = O(m \log m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is the dimension of the vectors, and $m$ is
    /// `m.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `m` is zero, if the vectors have different dimensions, or if `x` or any element is
    /// greater than or equal to `m`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::ModAddDot;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_vector::NaturalVector;
    ///
    /// let v = NaturalVector::from_str("(1, 2, 3)").unwrap();
    /// let w = NaturalVector::from_str("(4, 5, 7)").unwrap();
    /// assert_eq!(
    ///     Natural::from(3u32).mod_add_dot(&v, w.clone(), &Natural::from(11u32)),
    ///     5u32
    /// );
    /// ```
    #[inline]
    fn mod_add_dot(self, v: &NaturalVector, w: NaturalVector, m: &Self) -> Self {
        mod_dot_general(&self, false, v, &w, false, m)
    }
}

impl ModAddDot<&NaturalVector, &NaturalVector, Self> for Natural {
    type Output = Self;

    /// Adds the dot product of two [`NaturalVector`]s to a [`Natural`] modulo $m$, taking the
    /// number by value, the first vector by reference, and the second by reference, and the modulus
    /// by value. The number and the elements must already be reduced.
    ///
    /// $$
    /// f(x, v, w, m) = x + \sum_{i=0}^{n-1} v_iw_i \bmod m,
    /// $$
    ///
    /// where $n$ is the dimension of $v$ and $w$.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(nm \log m \log\log m)$
    ///
    /// $M(m) = O(m \log m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is the dimension of the vectors, and $m$ is
    /// `m.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `m` is zero, if the vectors have different dimensions, or if `x` or any element is
    /// greater than or equal to `m`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::ModAddDot;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_vector::NaturalVector;
    ///
    /// let v = NaturalVector::from_str("(1, 2, 3)").unwrap();
    /// let w = NaturalVector::from_str("(4, 5, 7)").unwrap();
    /// assert_eq!(
    ///     Natural::from(3u32).mod_add_dot(&v, &w, Natural::from(11u32)),
    ///     5u32
    /// );
    /// ```
    #[inline]
    fn mod_add_dot(self, v: &NaturalVector, w: &NaturalVector, m: Self) -> Self {
        mod_dot_general(&self, false, v, w, false, &m)
    }
}

impl ModAddDot<&NaturalVector, &NaturalVector, &Self> for Natural {
    type Output = Self;

    /// Adds the dot product of two [`NaturalVector`]s to a [`Natural`] modulo $m$, taking the
    /// number by value, the first vector by reference, and the second by reference, and the modulus
    /// by reference. The number and the elements must already be reduced.
    ///
    /// $$
    /// f(x, v, w, m) = x + \sum_{i=0}^{n-1} v_iw_i \bmod m,
    /// $$
    ///
    /// where $n$ is the dimension of $v$ and $w$.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(nm \log m \log\log m)$
    ///
    /// $M(m) = O(m \log m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is the dimension of the vectors, and $m$ is
    /// `m.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `m` is zero, if the vectors have different dimensions, or if `x` or any element is
    /// greater than or equal to `m`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::ModAddDot;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_vector::NaturalVector;
    ///
    /// let v = NaturalVector::from_str("(1, 2, 3)").unwrap();
    /// let w = NaturalVector::from_str("(4, 5, 7)").unwrap();
    /// assert_eq!(
    ///     Natural::from(3u32).mod_add_dot(&v, &w, &Natural::from(11u32)),
    ///     5u32
    /// );
    /// ```
    #[inline]
    fn mod_add_dot(self, v: &NaturalVector, w: &NaturalVector, m: &Self) -> Self {
        mod_dot_general(&self, false, v, w, false, m)
    }
}

impl ModAddDot<&NaturalVector, &NaturalVector, &Natural> for &Natural {
    type Output = Natural;

    /// Adds the dot product of two [`NaturalVector`]s to a [`Natural`] modulo $m$, taking the
    /// number by reference, the first vector by reference, and the second by reference, and the
    /// modulus by reference. The number and the elements must already be reduced.
    ///
    /// $$
    /// f(x, v, w, m) = x + \sum_{i=0}^{n-1} v_iw_i \bmod m,
    /// $$
    ///
    /// where $n$ is the dimension of $v$ and $w$.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(nm \log m \log\log m)$
    ///
    /// $M(m) = O(m \log m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is the dimension of the vectors, and $m$ is
    /// `m.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `m` is zero, if the vectors have different dimensions, or if `x` or any element is
    /// greater than or equal to `m`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::ModAddDot;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_vector::NaturalVector;
    ///
    /// let v = NaturalVector::from_str("(1, 2, 3)").unwrap();
    /// let w = NaturalVector::from_str("(4, 5, 7)").unwrap();
    /// assert_eq!(
    ///     (&Natural::from(3u32)).mod_add_dot(&v, &w, &Natural::from(11u32)),
    ///     5u32
    /// );
    /// ```
    #[inline]
    fn mod_add_dot(self, v: &NaturalVector, w: &NaturalVector, m: &Natural) -> Natural {
        mod_dot_general(self, false, v, w, false, m)
    }
}

impl ModAddDotAssign<NaturalVector, NaturalVector, Self> for Natural {
    /// Adds the dot product of two [`NaturalVector`]s to a [`Natural`] in place modulo $m$, taking
    /// the first vector by value and the second by value, and the modulus by value. The number and
    /// the elements must already be reduced.
    ///
    /// $$
    /// x \gets x + \sum_{i=0}^{n-1} v_iw_i \bmod m,
    /// $$
    ///
    /// where $n$ is the dimension of $v$ and $w$.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(nm \log m \log\log m)$
    ///
    /// $M(m) = O(m \log m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is the dimension of the vectors, and $m$ is
    /// `m.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `m` is zero, if the vectors have different dimensions, or if `x` or any element is
    /// greater than or equal to `m`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::ModAddDotAssign;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_vector::NaturalVector;
    ///
    /// let v = NaturalVector::from_str("(1, 2, 3)").unwrap();
    /// let w = NaturalVector::from_str("(4, 5, 7)").unwrap();
    /// let mut x = Natural::from(3u32);
    /// x.mod_add_dot_assign(v.clone(), w.clone(), Natural::from(11u32));
    /// assert_eq!(x, 5u32);
    /// ```
    #[inline]
    fn mod_add_dot_assign(&mut self, v: NaturalVector, w: NaturalVector, m: Self) {
        *self = mod_dot_general(self, false, &v, &w, false, &m);
    }
}

impl ModAddDotAssign<NaturalVector, NaturalVector, &Self> for Natural {
    /// Adds the dot product of two [`NaturalVector`]s to a [`Natural`] in place modulo $m$, taking
    /// the first vector by value and the second by value, and the modulus by reference. The number
    /// and the elements must already be reduced.
    ///
    /// $$
    /// x \gets x + \sum_{i=0}^{n-1} v_iw_i \bmod m,
    /// $$
    ///
    /// where $n$ is the dimension of $v$ and $w$.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(nm \log m \log\log m)$
    ///
    /// $M(m) = O(m \log m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is the dimension of the vectors, and $m$ is
    /// `m.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `m` is zero, if the vectors have different dimensions, or if `x` or any element is
    /// greater than or equal to `m`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::ModAddDotAssign;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_vector::NaturalVector;
    ///
    /// let v = NaturalVector::from_str("(1, 2, 3)").unwrap();
    /// let w = NaturalVector::from_str("(4, 5, 7)").unwrap();
    /// let mut x = Natural::from(3u32);
    /// x.mod_add_dot_assign(v.clone(), w.clone(), &Natural::from(11u32));
    /// assert_eq!(x, 5u32);
    /// ```
    #[inline]
    fn mod_add_dot_assign(&mut self, v: NaturalVector, w: NaturalVector, m: &Self) {
        *self = mod_dot_general(self, false, &v, &w, false, m);
    }
}

impl ModAddDotAssign<NaturalVector, &NaturalVector, Self> for Natural {
    /// Adds the dot product of two [`NaturalVector`]s to a [`Natural`] in place modulo $m$, taking
    /// the first vector by value and the second by reference, and the modulus by value. The number
    /// and the elements must already be reduced.
    ///
    /// $$
    /// x \gets x + \sum_{i=0}^{n-1} v_iw_i \bmod m,
    /// $$
    ///
    /// where $n$ is the dimension of $v$ and $w$.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(nm \log m \log\log m)$
    ///
    /// $M(m) = O(m \log m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is the dimension of the vectors, and $m$ is
    /// `m.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `m` is zero, if the vectors have different dimensions, or if `x` or any element is
    /// greater than or equal to `m`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::ModAddDotAssign;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_vector::NaturalVector;
    ///
    /// let v = NaturalVector::from_str("(1, 2, 3)").unwrap();
    /// let w = NaturalVector::from_str("(4, 5, 7)").unwrap();
    /// let mut x = Natural::from(3u32);
    /// x.mod_add_dot_assign(v.clone(), &w, Natural::from(11u32));
    /// assert_eq!(x, 5u32);
    /// ```
    #[inline]
    fn mod_add_dot_assign(&mut self, v: NaturalVector, w: &NaturalVector, m: Self) {
        *self = mod_dot_general(self, false, &v, w, false, &m);
    }
}

impl ModAddDotAssign<NaturalVector, &NaturalVector, &Self> for Natural {
    /// Adds the dot product of two [`NaturalVector`]s to a [`Natural`] in place modulo $m$, taking
    /// the first vector by value and the second by reference, and the modulus by reference. The
    /// number and the elements must already be reduced.
    ///
    /// $$
    /// x \gets x + \sum_{i=0}^{n-1} v_iw_i \bmod m,
    /// $$
    ///
    /// where $n$ is the dimension of $v$ and $w$.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(nm \log m \log\log m)$
    ///
    /// $M(m) = O(m \log m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is the dimension of the vectors, and $m$ is
    /// `m.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `m` is zero, if the vectors have different dimensions, or if `x` or any element is
    /// greater than or equal to `m`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::ModAddDotAssign;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_vector::NaturalVector;
    ///
    /// let v = NaturalVector::from_str("(1, 2, 3)").unwrap();
    /// let w = NaturalVector::from_str("(4, 5, 7)").unwrap();
    /// let mut x = Natural::from(3u32);
    /// x.mod_add_dot_assign(v.clone(), &w, &Natural::from(11u32));
    /// assert_eq!(x, 5u32);
    /// ```
    #[inline]
    fn mod_add_dot_assign(&mut self, v: NaturalVector, w: &NaturalVector, m: &Self) {
        *self = mod_dot_general(self, false, &v, w, false, m);
    }
}

impl ModAddDotAssign<&NaturalVector, NaturalVector, Self> for Natural {
    /// Adds the dot product of two [`NaturalVector`]s to a [`Natural`] in place modulo $m$, taking
    /// the first vector by reference and the second by value, and the modulus by value. The number
    /// and the elements must already be reduced.
    ///
    /// $$
    /// x \gets x + \sum_{i=0}^{n-1} v_iw_i \bmod m,
    /// $$
    ///
    /// where $n$ is the dimension of $v$ and $w$.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(nm \log m \log\log m)$
    ///
    /// $M(m) = O(m \log m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is the dimension of the vectors, and $m$ is
    /// `m.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `m` is zero, if the vectors have different dimensions, or if `x` or any element is
    /// greater than or equal to `m`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::ModAddDotAssign;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_vector::NaturalVector;
    ///
    /// let v = NaturalVector::from_str("(1, 2, 3)").unwrap();
    /// let w = NaturalVector::from_str("(4, 5, 7)").unwrap();
    /// let mut x = Natural::from(3u32);
    /// x.mod_add_dot_assign(&v, w.clone(), Natural::from(11u32));
    /// assert_eq!(x, 5u32);
    /// ```
    #[inline]
    fn mod_add_dot_assign(&mut self, v: &NaturalVector, w: NaturalVector, m: Self) {
        *self = mod_dot_general(self, false, v, &w, false, &m);
    }
}

impl ModAddDotAssign<&NaturalVector, NaturalVector, &Self> for Natural {
    /// Adds the dot product of two [`NaturalVector`]s to a [`Natural`] in place modulo $m$, taking
    /// the first vector by reference and the second by value, and the modulus by reference. The
    /// number and the elements must already be reduced.
    ///
    /// $$
    /// x \gets x + \sum_{i=0}^{n-1} v_iw_i \bmod m,
    /// $$
    ///
    /// where $n$ is the dimension of $v$ and $w$.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(nm \log m \log\log m)$
    ///
    /// $M(m) = O(m \log m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is the dimension of the vectors, and $m$ is
    /// `m.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `m` is zero, if the vectors have different dimensions, or if `x` or any element is
    /// greater than or equal to `m`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::ModAddDotAssign;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_vector::NaturalVector;
    ///
    /// let v = NaturalVector::from_str("(1, 2, 3)").unwrap();
    /// let w = NaturalVector::from_str("(4, 5, 7)").unwrap();
    /// let mut x = Natural::from(3u32);
    /// x.mod_add_dot_assign(&v, w.clone(), &Natural::from(11u32));
    /// assert_eq!(x, 5u32);
    /// ```
    #[inline]
    fn mod_add_dot_assign(&mut self, v: &NaturalVector, w: NaturalVector, m: &Self) {
        *self = mod_dot_general(self, false, v, &w, false, m);
    }
}

impl ModAddDotAssign<&NaturalVector, &NaturalVector, Self> for Natural {
    /// Adds the dot product of two [`NaturalVector`]s to a [`Natural`] in place modulo $m$, taking
    /// the first vector by reference and the second by reference, and the modulus by value. The
    /// number and the elements must already be reduced.
    ///
    /// $$
    /// x \gets x + \sum_{i=0}^{n-1} v_iw_i \bmod m,
    /// $$
    ///
    /// where $n$ is the dimension of $v$ and $w$.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(nm \log m \log\log m)$
    ///
    /// $M(m) = O(m \log m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is the dimension of the vectors, and $m$ is
    /// `m.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `m` is zero, if the vectors have different dimensions, or if `x` or any element is
    /// greater than or equal to `m`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::ModAddDotAssign;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_vector::NaturalVector;
    ///
    /// let v = NaturalVector::from_str("(1, 2, 3)").unwrap();
    /// let w = NaturalVector::from_str("(4, 5, 7)").unwrap();
    /// let mut x = Natural::from(3u32);
    /// x.mod_add_dot_assign(&v, &w, Natural::from(11u32));
    /// assert_eq!(x, 5u32);
    /// ```
    #[inline]
    fn mod_add_dot_assign(&mut self, v: &NaturalVector, w: &NaturalVector, m: Self) {
        *self = mod_dot_general(self, false, v, w, false, &m);
    }
}

impl ModAddDotAssign<&NaturalVector, &NaturalVector, &Self> for Natural {
    /// Adds the dot product of two [`NaturalVector`]s to a [`Natural`] in place modulo $m$, taking
    /// the first vector by reference and the second by reference, and the modulus by reference. The
    /// number and the elements must already be reduced.
    ///
    /// $$
    /// x \gets x + \sum_{i=0}^{n-1} v_iw_i \bmod m,
    /// $$
    ///
    /// where $n$ is the dimension of $v$ and $w$.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(nm \log m \log\log m)$
    ///
    /// $M(m) = O(m \log m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is the dimension of the vectors, and $m$ is
    /// `m.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `m` is zero, if the vectors have different dimensions, or if `x` or any element is
    /// greater than or equal to `m`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::ModAddDotAssign;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_vector::NaturalVector;
    ///
    /// let v = NaturalVector::from_str("(1, 2, 3)").unwrap();
    /// let w = NaturalVector::from_str("(4, 5, 7)").unwrap();
    /// let mut x = Natural::from(3u32);
    /// x.mod_add_dot_assign(&v, &w, &Natural::from(11u32));
    /// assert_eq!(x, 5u32);
    /// ```
    #[inline]
    fn mod_add_dot_assign(&mut self, v: &NaturalVector, w: &NaturalVector, m: &Self) {
        *self = mod_dot_general(self, false, v, w, false, m);
    }
}
