// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::Rational;
use crate::rational_vector::RationalVector;
use malachite_base::num::arithmetic::traits::{SubMulAssign, SubMulShl, SubMulShlAssign};
use malachite_base::num::basic::traits::{NegativeOne, One, Zero};

fn assert_same_dimension(v: &RationalVector, w: &RationalVector) {
    assert_eq!(
        v.elements.len(),
        w.elements.len(),
        "cannot sub-multiply vectors of different dimensions"
    );
}

impl SubMulShl<Self, Rational> for RationalVector {
    type Output = Self;

    /// Subtracts a scalar multiple of a [`RationalVector`], shifted left by `bits`, from a
    /// [`RationalVector`], taking both vectors and the scalar by value.
    ///
    /// The operation is taken element by element, so the result has the same dimension as the
    /// vectors. This special-cases multiplication by 0, 1, and $-1$, and a shift by 0.
    ///
    /// $$
    /// f(v, w, c, k) = v - cw2^k.
    /// $$
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n (\log n)^2 \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is the total number of bits of the
    /// elements of both vectors and of `c`, plus `bits` times `self.dimension()`.
    ///
    /// # Panics
    /// Panics if `self` and `w` have different dimensions.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::SubMulShl;
    /// use malachite_q::Rational;
    /// use malachite_q::rational_vector::RationalVector;
    ///
    /// let v = RationalVector::from_str("(1/2, -2, 3)").unwrap();
    /// let w = RationalVector::from_str("(4, 1/3, -6)").unwrap();
    /// assert_eq!(
    ///     v.sub_mul_shl(w, Rational::from_signeds(3, 2), 2)
    ///         .to_string(),
    ///     "(-47/2, -4, 39)"
    /// );
    ///
    /// let v = RationalVector::from_str("(1, 0)").unwrap();
    /// let w = RationalVector::from_str("(18446744073709551615, -1)").unwrap();
    /// assert_eq!(
    ///     v.sub_mul_shl(w, Rational::from_str("18446744073709551617").unwrap(), 3)
    ///         .to_string(),
    ///     "(-2722258935367507707706996859454145691639, 147573952589676412936)"
    /// );
    /// ```
    #[inline]
    fn sub_mul_shl(mut self, w: Self, c: Rational, bits: u64) -> Self {
        self.sub_mul_shl_assign(w, c, bits);
        self
    }
}

impl SubMulShl<Self, &Rational> for RationalVector {
    type Output = Self;

    /// Subtracts a scalar multiple of a [`RationalVector`], shifted left by `bits`, from a
    /// [`RationalVector`], taking the first vector by value, the second by value, and the scalar by
    /// reference.
    ///
    /// See the documentation for the [`SubMulShl`] implementation that takes everything by value
    /// for details.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n (\log n)^2 \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is the total number of bits of the
    /// elements of both vectors and of `c`, plus `bits` times `self.dimension()`.
    ///
    /// # Panics
    /// Panics if `self` and `w` have different dimensions.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::SubMulShl;
    /// use malachite_q::Rational;
    /// use malachite_q::rational_vector::RationalVector;
    ///
    /// let v = RationalVector::from_str("(1/2, -2, 3)").unwrap();
    /// let w = RationalVector::from_str("(4, 1/3, -6)").unwrap();
    /// assert_eq!(
    ///     v.sub_mul_shl(w, &Rational::from_signeds(3, 2), 2)
    ///         .to_string(),
    ///     "(-47/2, -4, 39)"
    /// );
    ///
    /// let v = RationalVector::from_str("(1, 0)").unwrap();
    /// let w = RationalVector::from_str("(18446744073709551615, -1)").unwrap();
    /// assert_eq!(
    ///     v.sub_mul_shl(w, &Rational::from_str("18446744073709551617").unwrap(), 3)
    ///         .to_string(),
    ///     "(-2722258935367507707706996859454145691639, 147573952589676412936)"
    /// );
    /// ```
    #[inline]
    fn sub_mul_shl(mut self, w: Self, c: &Rational, bits: u64) -> Self {
        self.sub_mul_shl_assign(w, c, bits);
        self
    }
}

impl SubMulShl<&Self, Rational> for RationalVector {
    type Output = Self;

    /// Subtracts a scalar multiple of a [`RationalVector`], shifted left by `bits`, from a
    /// [`RationalVector`], taking the first vector by value, the second by reference, and the
    /// scalar by value.
    ///
    /// See the documentation for the [`SubMulShl`] implementation that takes everything by value
    /// for details.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n (\log n)^2 \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is the total number of bits of the
    /// elements of both vectors and of `c`, plus `bits` times `self.dimension()`.
    ///
    /// # Panics
    /// Panics if `self` and `w` have different dimensions.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::SubMulShl;
    /// use malachite_q::Rational;
    /// use malachite_q::rational_vector::RationalVector;
    ///
    /// let v = RationalVector::from_str("(1/2, -2, 3)").unwrap();
    /// let w = RationalVector::from_str("(4, 1/3, -6)").unwrap();
    /// assert_eq!(
    ///     v.sub_mul_shl(&w, Rational::from_signeds(3, 2), 2)
    ///         .to_string(),
    ///     "(-47/2, -4, 39)"
    /// );
    ///
    /// let v = RationalVector::from_str("(1, 0)").unwrap();
    /// let w = RationalVector::from_str("(18446744073709551615, -1)").unwrap();
    /// assert_eq!(
    ///     v.sub_mul_shl(&w, Rational::from_str("18446744073709551617").unwrap(), 3)
    ///         .to_string(),
    ///     "(-2722258935367507707706996859454145691639, 147573952589676412936)"
    /// );
    /// ```
    #[inline]
    fn sub_mul_shl(mut self, w: &Self, c: Rational, bits: u64) -> Self {
        self.sub_mul_shl_assign(w, c, bits);
        self
    }
}

impl SubMulShl<&Self, &Rational> for RationalVector {
    type Output = Self;

    /// Subtracts a scalar multiple of a [`RationalVector`], shifted left by `bits`, from a
    /// [`RationalVector`], taking the first vector by value, the second by reference, and the
    /// scalar by reference.
    ///
    /// See the documentation for the [`SubMulShl`] implementation that takes everything by value
    /// for details.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n (\log n)^2 \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is the total number of bits of the
    /// elements of both vectors and of `c`, plus `bits` times `self.dimension()`.
    ///
    /// # Panics
    /// Panics if `self` and `w` have different dimensions.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::SubMulShl;
    /// use malachite_q::Rational;
    /// use malachite_q::rational_vector::RationalVector;
    ///
    /// let v = RationalVector::from_str("(1/2, -2, 3)").unwrap();
    /// let w = RationalVector::from_str("(4, 1/3, -6)").unwrap();
    /// assert_eq!(
    ///     v.sub_mul_shl(&w, &Rational::from_signeds(3, 2), 2)
    ///         .to_string(),
    ///     "(-47/2, -4, 39)"
    /// );
    ///
    /// let v = RationalVector::from_str("(1, 0)").unwrap();
    /// let w = RationalVector::from_str("(18446744073709551615, -1)").unwrap();
    /// assert_eq!(
    ///     v.sub_mul_shl(&w, &Rational::from_str("18446744073709551617").unwrap(), 3)
    ///         .to_string(),
    ///     "(-2722258935367507707706996859454145691639, 147573952589676412936)"
    /// );
    /// ```
    #[inline]
    fn sub_mul_shl(mut self, w: &Self, c: &Rational, bits: u64) -> Self {
        self.sub_mul_shl_assign(w, c, bits);
        self
    }
}

impl SubMulShl<&RationalVector, &Rational> for &RationalVector {
    type Output = RationalVector;

    /// Subtracts a scalar multiple of a [`RationalVector`], shifted left by `bits`, from a
    /// [`RationalVector`], taking both vectors and the scalar by reference.
    ///
    /// See the documentation for the [`SubMulShl`] implementation that takes everything by value
    /// for details.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n (\log n)^2 \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is the total number of bits of the
    /// elements of both vectors and of `c`, plus `bits` times `self.dimension()`.
    ///
    /// # Panics
    /// Panics if `self` and `w` have different dimensions.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::SubMulShl;
    /// use malachite_q::Rational;
    /// use malachite_q::rational_vector::RationalVector;
    ///
    /// let v = RationalVector::from_str("(1/2, -2, 3)").unwrap();
    /// let w = RationalVector::from_str("(4, 1/3, -6)").unwrap();
    /// assert_eq!(
    ///     (&v).sub_mul_shl(&w, &Rational::from_signeds(3, 2), 2)
    ///         .to_string(),
    ///     "(-47/2, -4, 39)"
    /// );
    ///
    /// let v = RationalVector::from_str("(1, 0)").unwrap();
    /// let w = RationalVector::from_str("(18446744073709551615, -1)").unwrap();
    /// assert_eq!(
    ///     (&v).sub_mul_shl(&w, &Rational::from_str("18446744073709551617").unwrap(), 3)
    ///         .to_string(),
    ///     "(-2722258935367507707706996859454145691639, 147573952589676412936)"
    /// );
    /// ```
    fn sub_mul_shl(self, w: &RationalVector, c: &Rational, bits: u64) -> RationalVector {
        let mut v = self.clone();
        v.sub_mul_shl_assign(w, c, bits);
        v
    }
}

impl SubMulShlAssign<Self, Rational> for RationalVector {
    /// Subtracts a scalar multiple of a [`RationalVector`], shifted left by `bits`, from a
    /// [`RationalVector`], in place, taking the vector and the scalar on the right-hand side by
    /// value.
    ///
    /// See the documentation for the [`SubMulShl`] implementation that takes everything by value
    /// for details.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n (\log n)^2 \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is the total number of bits of the
    /// elements of both vectors and of `c`, plus `bits` times `self.dimension()`.
    ///
    /// # Panics
    /// Panics if `self` and `w` have different dimensions.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::SubMulShlAssign;
    /// use malachite_q::Rational;
    /// use malachite_q::rational_vector::RationalVector;
    ///
    /// let mut v = RationalVector::from_str("(1/2, -2, 3)").unwrap();
    /// let w = RationalVector::from_str("(4, 1/3, -6)").unwrap();
    /// v.sub_mul_shl_assign(w, Rational::from_signeds(3, 2), 2);
    /// assert_eq!(v.to_string(), "(-47/2, -4, 39)");
    ///
    /// let mut v = RationalVector::from_str("(1, 0)").unwrap();
    /// let w = RationalVector::from_str("(18446744073709551615, -1)").unwrap();
    /// v.sub_mul_shl_assign(w, Rational::from_str("18446744073709551617").unwrap(), 3);
    /// assert_eq!(
    ///     v.to_string(),
    ///     "(-2722258935367507707706996859454145691639, 147573952589676412936)"
    /// );
    /// ```
    #[inline]
    fn sub_mul_shl_assign(&mut self, w: Self, c: Rational, bits: u64) {
        self.sub_mul_shl_assign(w, &c, bits);
    }
}

impl SubMulShlAssign<Self, &Rational> for RationalVector {
    /// Subtracts a scalar multiple of a [`RationalVector`], shifted left by `bits`, from a
    /// [`RationalVector`], in place, taking the vector on the right-hand side by value and the
    /// scalar by reference.
    ///
    /// See the documentation for the [`SubMulShl`] implementation that takes everything by value
    /// for details.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n (\log n)^2 \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is the total number of bits of the
    /// elements of both vectors and of `c`, plus `bits` times `self.dimension()`.
    ///
    /// # Panics
    /// Panics if `self` and `w` have different dimensions.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::SubMulShlAssign;
    /// use malachite_q::Rational;
    /// use malachite_q::rational_vector::RationalVector;
    ///
    /// let mut v = RationalVector::from_str("(1/2, -2, 3)").unwrap();
    /// let w = RationalVector::from_str("(4, 1/3, -6)").unwrap();
    /// v.sub_mul_shl_assign(w, &Rational::from_signeds(3, 2), 2);
    /// assert_eq!(v.to_string(), "(-47/2, -4, 39)");
    ///
    /// let mut v = RationalVector::from_str("(1, 0)").unwrap();
    /// let w = RationalVector::from_str("(18446744073709551615, -1)").unwrap();
    /// v.sub_mul_shl_assign(w, &Rational::from_str("18446744073709551617").unwrap(), 3);
    /// assert_eq!(
    ///     v.to_string(),
    ///     "(-2722258935367507707706996859454145691639, 147573952589676412936)"
    /// );
    /// ```
    fn sub_mul_shl_assign(&mut self, w: Self, c: &Rational, bits: u64) {
        assert_same_dimension(self, &w);
        if bits == 0 {
            self.sub_mul_assign(w, c);
            return;
        }
        match *c {
            Rational::ZERO => {}
            Rational::ONE => {
                for (x, mut y) in self.elements.iter_mut().zip(w.elements) {
                    y <<= bits;
                    *x -= y;
                }
            }
            Rational::NEGATIVE_ONE => {
                for (x, mut y) in self.elements.iter_mut().zip(w.elements) {
                    y <<= bits;
                    *x += y;
                }
            }
            _ => {
                for (x, y) in self.elements.iter_mut().zip(w.elements) {
                    x.sub_mul_shl_assign(y, c, bits);
                }
            }
        }
    }
}

impl SubMulShlAssign<&Self, Rational> for RationalVector {
    /// Subtracts a scalar multiple of a [`RationalVector`], shifted left by `bits`, from a
    /// [`RationalVector`], in place, taking the vector on the right-hand side by reference and the
    /// scalar by value.
    ///
    /// See the documentation for the [`SubMulShl`] implementation that takes everything by value
    /// for details.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n (\log n)^2 \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is the total number of bits of the
    /// elements of both vectors and of `c`, plus `bits` times `self.dimension()`.
    ///
    /// # Panics
    /// Panics if `self` and `w` have different dimensions.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::SubMulShlAssign;
    /// use malachite_q::Rational;
    /// use malachite_q::rational_vector::RationalVector;
    ///
    /// let mut v = RationalVector::from_str("(1/2, -2, 3)").unwrap();
    /// let w = RationalVector::from_str("(4, 1/3, -6)").unwrap();
    /// v.sub_mul_shl_assign(&w, Rational::from_signeds(3, 2), 2);
    /// assert_eq!(v.to_string(), "(-47/2, -4, 39)");
    ///
    /// let mut v = RationalVector::from_str("(1, 0)").unwrap();
    /// let w = RationalVector::from_str("(18446744073709551615, -1)").unwrap();
    /// v.sub_mul_shl_assign(&w, Rational::from_str("18446744073709551617").unwrap(), 3);
    /// assert_eq!(
    ///     v.to_string(),
    ///     "(-2722258935367507707706996859454145691639, 147573952589676412936)"
    /// );
    /// ```
    #[inline]
    fn sub_mul_shl_assign(&mut self, w: &Self, c: Rational, bits: u64) {
        self.sub_mul_shl_assign(w, &c, bits);
    }
}

impl SubMulShlAssign<&Self, &Rational> for RationalVector {
    /// Subtracts a scalar multiple of a [`RationalVector`], shifted left by `bits`, from a
    /// [`RationalVector`], in place, taking the vector and the scalar on the right-hand side by
    /// reference.
    ///
    /// See the documentation for the [`SubMulShl`] implementation that takes everything by value
    /// for details.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n (\log n)^2 \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is the total number of bits of the
    /// elements of both vectors and of `c`, plus `bits` times `self.dimension()`.
    ///
    /// # Panics
    /// Panics if `self` and `w` have different dimensions.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::SubMulShlAssign;
    /// use malachite_q::Rational;
    /// use malachite_q::rational_vector::RationalVector;
    ///
    /// let mut v = RationalVector::from_str("(1/2, -2, 3)").unwrap();
    /// let w = RationalVector::from_str("(4, 1/3, -6)").unwrap();
    /// v.sub_mul_shl_assign(&w, &Rational::from_signeds(3, 2), 2);
    /// assert_eq!(v.to_string(), "(-47/2, -4, 39)");
    ///
    /// let mut v = RationalVector::from_str("(1, 0)").unwrap();
    /// let w = RationalVector::from_str("(18446744073709551615, -1)").unwrap();
    /// v.sub_mul_shl_assign(&w, &Rational::from_str("18446744073709551617").unwrap(), 3);
    /// assert_eq!(
    ///     v.to_string(),
    ///     "(-2722258935367507707706996859454145691639, 147573952589676412936)"
    /// );
    /// ```
    fn sub_mul_shl_assign(&mut self, w: &Self, c: &Rational, bits: u64) {
        assert_same_dimension(self, w);
        if bits == 0 {
            self.sub_mul_assign(w, c);
            return;
        }
        match *c {
            Rational::ZERO => {}
            Rational::ONE => {
                for (x, y) in self.elements.iter_mut().zip(&w.elements) {
                    *x -= y << bits;
                }
            }
            Rational::NEGATIVE_ONE => {
                for (x, y) in self.elements.iter_mut().zip(&w.elements) {
                    *x += y << bits;
                }
            }
            _ => {
                for (x, y) in self.elements.iter_mut().zip(&w.elements) {
                    x.sub_mul_shl_assign(y, c, bits);
                }
            }
        }
    }
}
