// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::integer::Integer;
use crate::integer_vector::IntegerVector;
use crate::natural::Natural;
use malachite_base::num::arithmetic::traits::{SubMulAssign, SubMulShl, SubMulShlAssign};
use malachite_base::num::basic::traits::{One, Zero};

fn assert_same_dimension(v: &IntegerVector, w: &IntegerVector) {
    assert_eq!(
        v.elements.len(),
        w.elements.len(),
        "cannot sub-multiply vectors of different dimensions"
    );
}

impl SubMulShl<Self, Integer> for IntegerVector {
    type Output = Self;

    /// Subtracts a scalar multiple of an [`IntegerVector`], shifted left by `bits`, from an
    /// [`IntegerVector`], taking both vectors and the scalar by value.
    ///
    /// The operation is taken element by element, so the result has the same dimension as the
    /// vectors. Like FLINT's, this special-cases multiplication by 0, 1, and $-1$, and a shift by
    /// 0.
    ///
    /// $$
    /// f(v, w, c, k) = v - cw2^k.
    /// $$
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(m + n \log n \log\log n)$
    ///
    /// $M(n, m) = O(m + n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is the total number of bits of the elements
    /// of `w` and of `c`, and $m$ is the total number of bits of the elements of `self` plus `bits`
    /// times `self.dimension()`.
    ///
    /// # Panics
    /// Panics if `self` and `w` have different dimensions.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::SubMulShl;
    /// use malachite_nz::integer::Integer;
    /// use malachite_nz::integer_vector::IntegerVector;
    ///
    /// let v = IntegerVector::from_str("(1, -2, 3)").unwrap();
    /// let w = IntegerVector::from_str("(4, 5, -6)").unwrap();
    /// assert_eq!(
    ///     v.sub_mul_shl(w, Integer::from(10), 2).to_string(),
    ///     "(-159, -202, 243)"
    /// );
    ///
    /// let v = IntegerVector::from_str("(1, 0)").unwrap();
    /// let w = IntegerVector::from_str("(18446744073709551615, -1)").unwrap();
    /// assert_eq!(
    ///     v.sub_mul_shl(w, Integer::from_str("18446744073709551617").unwrap(), 3)
    ///         .to_string(),
    ///     "(-2722258935367507707706996859454145691639, 147573952589676412936)"
    /// );
    /// ```
    ///
    /// This is equivalent to `_fmpz_vec_scalar_submul_si_2exp` from `fmpz_vec/scalar.c`, FLINT
    /// 3.6.0.
    #[inline]
    fn sub_mul_shl(mut self, w: Self, c: Integer, bits: u64) -> Self {
        self.sub_mul_shl_assign(w, c, bits);
        self
    }
}

impl SubMulShl<Self, &Integer> for IntegerVector {
    type Output = Self;

    /// Subtracts a scalar multiple of an [`IntegerVector`], shifted left by `bits`, from an
    /// [`IntegerVector`], taking the first vector by value, the second by value, and the scalar by
    /// reference.
    ///
    /// See the documentation for the [`SubMulShl`] implementation that takes everything by value
    /// for details.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(m + n \log n \log\log n)$
    ///
    /// $M(n, m) = O(m + n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is the total number of bits of the elements
    /// of `w` and of `c`, and $m$ is the total number of bits of the elements of `self` plus `bits`
    /// times `self.dimension()`.
    ///
    /// # Panics
    /// Panics if `self` and `w` have different dimensions.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::SubMulShl;
    /// use malachite_nz::integer::Integer;
    /// use malachite_nz::integer_vector::IntegerVector;
    ///
    /// let v = IntegerVector::from_str("(1, -2, 3)").unwrap();
    /// let w = IntegerVector::from_str("(4, 5, -6)").unwrap();
    /// assert_eq!(
    ///     v.sub_mul_shl(w, &Integer::from(10), 2).to_string(),
    ///     "(-159, -202, 243)"
    /// );
    ///
    /// let v = IntegerVector::from_str("(1, 0)").unwrap();
    /// let w = IntegerVector::from_str("(18446744073709551615, -1)").unwrap();
    /// assert_eq!(
    ///     v.sub_mul_shl(w, &Integer::from_str("18446744073709551617").unwrap(), 3)
    ///         .to_string(),
    ///     "(-2722258935367507707706996859454145691639, 147573952589676412936)"
    /// );
    /// ```
    ///
    /// This is equivalent to `_fmpz_vec_scalar_submul_si_2exp` from `fmpz_vec/scalar.c`, FLINT
    /// 3.6.0.
    #[inline]
    fn sub_mul_shl(mut self, w: Self, c: &Integer, bits: u64) -> Self {
        self.sub_mul_shl_assign(w, c, bits);
        self
    }
}

impl SubMulShl<&Self, Integer> for IntegerVector {
    type Output = Self;

    /// Subtracts a scalar multiple of an [`IntegerVector`], shifted left by `bits`, from an
    /// [`IntegerVector`], taking the first vector by value, the second by reference, and the scalar
    /// by value.
    ///
    /// See the documentation for the [`SubMulShl`] implementation that takes everything by value
    /// for details.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(m + n \log n \log\log n)$
    ///
    /// $M(n, m) = O(m + n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is the total number of bits of the elements
    /// of `w` and of `c`, and $m$ is the total number of bits of the elements of `self` plus `bits`
    /// times `self.dimension()`.
    ///
    /// # Panics
    /// Panics if `self` and `w` have different dimensions.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::SubMulShl;
    /// use malachite_nz::integer::Integer;
    /// use malachite_nz::integer_vector::IntegerVector;
    ///
    /// let v = IntegerVector::from_str("(1, -2, 3)").unwrap();
    /// let w = IntegerVector::from_str("(4, 5, -6)").unwrap();
    /// assert_eq!(
    ///     v.sub_mul_shl(&w, Integer::from(10), 2).to_string(),
    ///     "(-159, -202, 243)"
    /// );
    ///
    /// let v = IntegerVector::from_str("(1, 0)").unwrap();
    /// let w = IntegerVector::from_str("(18446744073709551615, -1)").unwrap();
    /// assert_eq!(
    ///     v.sub_mul_shl(&w, Integer::from_str("18446744073709551617").unwrap(), 3)
    ///         .to_string(),
    ///     "(-2722258935367507707706996859454145691639, 147573952589676412936)"
    /// );
    /// ```
    ///
    /// This is equivalent to `_fmpz_vec_scalar_submul_si_2exp` from `fmpz_vec/scalar.c`, FLINT
    /// 3.6.0.
    #[inline]
    fn sub_mul_shl(mut self, w: &Self, c: Integer, bits: u64) -> Self {
        self.sub_mul_shl_assign(w, c, bits);
        self
    }
}

impl SubMulShl<&Self, &Integer> for IntegerVector {
    type Output = Self;

    /// Subtracts a scalar multiple of an [`IntegerVector`], shifted left by `bits`, from an
    /// [`IntegerVector`], taking the first vector by value, the second by reference, and the scalar
    /// by reference.
    ///
    /// See the documentation for the [`SubMulShl`] implementation that takes everything by value
    /// for details.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(m + n \log n \log\log n)$
    ///
    /// $M(n, m) = O(m + n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is the total number of bits of the elements
    /// of `w` and of `c`, and $m$ is the total number of bits of the elements of `self` plus `bits`
    /// times `self.dimension()`.
    ///
    /// # Panics
    /// Panics if `self` and `w` have different dimensions.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::SubMulShl;
    /// use malachite_nz::integer::Integer;
    /// use malachite_nz::integer_vector::IntegerVector;
    ///
    /// let v = IntegerVector::from_str("(1, -2, 3)").unwrap();
    /// let w = IntegerVector::from_str("(4, 5, -6)").unwrap();
    /// assert_eq!(
    ///     v.sub_mul_shl(&w, &Integer::from(10), 2).to_string(),
    ///     "(-159, -202, 243)"
    /// );
    ///
    /// let v = IntegerVector::from_str("(1, 0)").unwrap();
    /// let w = IntegerVector::from_str("(18446744073709551615, -1)").unwrap();
    /// assert_eq!(
    ///     v.sub_mul_shl(&w, &Integer::from_str("18446744073709551617").unwrap(), 3)
    ///         .to_string(),
    ///     "(-2722258935367507707706996859454145691639, 147573952589676412936)"
    /// );
    /// ```
    ///
    /// This is equivalent to `_fmpz_vec_scalar_submul_si_2exp` from `fmpz_vec/scalar.c`, FLINT
    /// 3.6.0.
    #[inline]
    fn sub_mul_shl(mut self, w: &Self, c: &Integer, bits: u64) -> Self {
        self.sub_mul_shl_assign(w, c, bits);
        self
    }
}

impl SubMulShl<&IntegerVector, &Integer> for &IntegerVector {
    type Output = IntegerVector;

    /// Subtracts a scalar multiple of an [`IntegerVector`], shifted left by `bits`, from an
    /// [`IntegerVector`], taking both vectors and the scalar by reference.
    ///
    /// See the documentation for the [`SubMulShl`] implementation that takes everything by value
    /// for details.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(m + n \log n \log\log n)$
    ///
    /// $M(n, m) = O(m + n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is the total number of bits of the elements
    /// of `w` and of `c`, and $m$ is the total number of bits of the elements of `self` plus `bits`
    /// times `self.dimension()`.
    ///
    /// # Panics
    /// Panics if `self` and `w` have different dimensions.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::SubMulShl;
    /// use malachite_nz::integer::Integer;
    /// use malachite_nz::integer_vector::IntegerVector;
    ///
    /// let v = IntegerVector::from_str("(1, -2, 3)").unwrap();
    /// let w = IntegerVector::from_str("(4, 5, -6)").unwrap();
    /// assert_eq!(
    ///     (&v).sub_mul_shl(&w, &Integer::from(10), 2).to_string(),
    ///     "(-159, -202, 243)"
    /// );
    ///
    /// let v = IntegerVector::from_str("(1, 0)").unwrap();
    /// let w = IntegerVector::from_str("(18446744073709551615, -1)").unwrap();
    /// assert_eq!(
    ///     (&v).sub_mul_shl(&w, &Integer::from_str("18446744073709551617").unwrap(), 3)
    ///         .to_string(),
    ///     "(-2722258935367507707706996859454145691639, 147573952589676412936)"
    /// );
    /// ```
    ///
    /// This is equivalent to `_fmpz_vec_scalar_submul_si_2exp` from `fmpz_vec/scalar.c`, FLINT
    /// 3.6.0.
    fn sub_mul_shl(self, w: &IntegerVector, c: &Integer, bits: u64) -> IntegerVector {
        let mut v = self.clone();
        v.sub_mul_shl_assign(w, c, bits);
        v
    }
}

impl SubMulShlAssign<Self, Integer> for IntegerVector {
    /// Subtracts a scalar multiple of an [`IntegerVector`], shifted left by `bits`, from an
    /// [`IntegerVector`], in place, taking the vector and the scalar on the right-hand side by
    /// value.
    ///
    /// See the documentation for the [`SubMulShl`] implementation that takes everything by value
    /// for details.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(m + n \log n \log\log n)$
    ///
    /// $M(n, m) = O(m + n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is the total number of bits of the elements
    /// of `w` and of `c`, and $m$ is the total number of bits of the elements of `self` plus `bits`
    /// times `self.dimension()`.
    ///
    /// # Panics
    /// Panics if `self` and `w` have different dimensions.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::SubMulShlAssign;
    /// use malachite_nz::integer::Integer;
    /// use malachite_nz::integer_vector::IntegerVector;
    ///
    /// let mut v = IntegerVector::from_str("(1, -2, 3)").unwrap();
    /// let w = IntegerVector::from_str("(4, 5, -6)").unwrap();
    /// v.sub_mul_shl_assign(w, Integer::from(10), 2);
    /// assert_eq!(v.to_string(), "(-159, -202, 243)");
    ///
    /// let mut v = IntegerVector::from_str("(1, 0)").unwrap();
    /// let w = IntegerVector::from_str("(18446744073709551615, -1)").unwrap();
    /// v.sub_mul_shl_assign(w, Integer::from_str("18446744073709551617").unwrap(), 3);
    /// assert_eq!(
    ///     v.to_string(),
    ///     "(-2722258935367507707706996859454145691639, 147573952589676412936)"
    /// );
    /// ```
    ///
    /// This is equivalent to `_fmpz_vec_scalar_submul_si_2exp` from `fmpz_vec/scalar.c`, FLINT
    /// 3.6.0.
    #[inline]
    fn sub_mul_shl_assign(&mut self, w: Self, c: Integer, bits: u64) {
        self.sub_mul_shl_assign(w, &c, bits);
    }
}

impl SubMulShlAssign<Self, &Integer> for IntegerVector {
    /// Subtracts a scalar multiple of an [`IntegerVector`], shifted left by `bits`, from an
    /// [`IntegerVector`], in place, taking the vector on the right-hand side by value and the
    /// scalar by reference.
    ///
    /// See the documentation for the [`SubMulShl`] implementation that takes everything by value
    /// for details.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(m + n \log n \log\log n)$
    ///
    /// $M(n, m) = O(m + n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is the total number of bits of the elements
    /// of `w` and of `c`, and $m$ is the total number of bits of the elements of `self` plus `bits`
    /// times `self.dimension()`.
    ///
    /// # Panics
    /// Panics if `self` and `w` have different dimensions.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::SubMulShlAssign;
    /// use malachite_nz::integer::Integer;
    /// use malachite_nz::integer_vector::IntegerVector;
    ///
    /// let mut v = IntegerVector::from_str("(1, -2, 3)").unwrap();
    /// let w = IntegerVector::from_str("(4, 5, -6)").unwrap();
    /// v.sub_mul_shl_assign(w, &Integer::from(10), 2);
    /// assert_eq!(v.to_string(), "(-159, -202, 243)");
    ///
    /// let mut v = IntegerVector::from_str("(1, 0)").unwrap();
    /// let w = IntegerVector::from_str("(18446744073709551615, -1)").unwrap();
    /// v.sub_mul_shl_assign(w, &Integer::from_str("18446744073709551617").unwrap(), 3);
    /// assert_eq!(
    ///     v.to_string(),
    ///     "(-2722258935367507707706996859454145691639, 147573952589676412936)"
    /// );
    /// ```
    ///
    /// This is equivalent to `_fmpz_vec_scalar_submul_si_2exp` from `fmpz_vec/scalar.c`, FLINT
    /// 3.6.0.
    fn sub_mul_shl_assign(&mut self, w: Self, c: &Integer, bits: u64) {
        assert_same_dimension(self, &w);
        if bits == 0 {
            self.sub_mul_assign(w, c);
            return;
        }
        match *c {
            integer_zero!() => {}
            integer_one!() => {
                for (x, mut y) in self.elements.iter_mut().zip(w.elements) {
                    y <<= bits;
                    *x -= y;
                }
            }
            integer_negative_one!() => {
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

impl SubMulShlAssign<&Self, Integer> for IntegerVector {
    /// Subtracts a scalar multiple of an [`IntegerVector`], shifted left by `bits`, from an
    /// [`IntegerVector`], in place, taking the vector on the right-hand side by reference and the
    /// scalar by value.
    ///
    /// See the documentation for the [`SubMulShl`] implementation that takes everything by value
    /// for details.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(m + n \log n \log\log n)$
    ///
    /// $M(n, m) = O(m + n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is the total number of bits of the elements
    /// of `w` and of `c`, and $m$ is the total number of bits of the elements of `self` plus `bits`
    /// times `self.dimension()`.
    ///
    /// # Panics
    /// Panics if `self` and `w` have different dimensions.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::SubMulShlAssign;
    /// use malachite_nz::integer::Integer;
    /// use malachite_nz::integer_vector::IntegerVector;
    ///
    /// let mut v = IntegerVector::from_str("(1, -2, 3)").unwrap();
    /// let w = IntegerVector::from_str("(4, 5, -6)").unwrap();
    /// v.sub_mul_shl_assign(&w, Integer::from(10), 2);
    /// assert_eq!(v.to_string(), "(-159, -202, 243)");
    ///
    /// let mut v = IntegerVector::from_str("(1, 0)").unwrap();
    /// let w = IntegerVector::from_str("(18446744073709551615, -1)").unwrap();
    /// v.sub_mul_shl_assign(&w, Integer::from_str("18446744073709551617").unwrap(), 3);
    /// assert_eq!(
    ///     v.to_string(),
    ///     "(-2722258935367507707706996859454145691639, 147573952589676412936)"
    /// );
    /// ```
    ///
    /// This is equivalent to `_fmpz_vec_scalar_submul_si_2exp` from `fmpz_vec/scalar.c`, FLINT
    /// 3.6.0.
    #[inline]
    fn sub_mul_shl_assign(&mut self, w: &Self, c: Integer, bits: u64) {
        self.sub_mul_shl_assign(w, &c, bits);
    }
}

impl SubMulShlAssign<&Self, &Integer> for IntegerVector {
    /// Subtracts a scalar multiple of an [`IntegerVector`], shifted left by `bits`, from an
    /// [`IntegerVector`], in place, taking the vector and the scalar on the right-hand side by
    /// reference.
    ///
    /// See the documentation for the [`SubMulShl`] implementation that takes everything by value
    /// for details.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(m + n \log n \log\log n)$
    ///
    /// $M(n, m) = O(m + n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is the total number of bits of the elements
    /// of `w` and of `c`, and $m$ is the total number of bits of the elements of `self` plus `bits`
    /// times `self.dimension()`.
    ///
    /// # Panics
    /// Panics if `self` and `w` have different dimensions.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::SubMulShlAssign;
    /// use malachite_nz::integer::Integer;
    /// use malachite_nz::integer_vector::IntegerVector;
    ///
    /// let mut v = IntegerVector::from_str("(1, -2, 3)").unwrap();
    /// let w = IntegerVector::from_str("(4, 5, -6)").unwrap();
    /// v.sub_mul_shl_assign(&w, &Integer::from(10), 2);
    /// assert_eq!(v.to_string(), "(-159, -202, 243)");
    ///
    /// let mut v = IntegerVector::from_str("(1, 0)").unwrap();
    /// let w = IntegerVector::from_str("(18446744073709551615, -1)").unwrap();
    /// v.sub_mul_shl_assign(&w, &Integer::from_str("18446744073709551617").unwrap(), 3);
    /// assert_eq!(
    ///     v.to_string(),
    ///     "(-2722258935367507707706996859454145691639, 147573952589676412936)"
    /// );
    /// ```
    ///
    /// This is equivalent to `_fmpz_vec_scalar_submul_si_2exp` from `fmpz_vec/scalar.c`, FLINT
    /// 3.6.0.
    fn sub_mul_shl_assign(&mut self, w: &Self, c: &Integer, bits: u64) {
        assert_same_dimension(self, w);
        if bits == 0 {
            self.sub_mul_assign(w, c);
            return;
        }
        match *c {
            integer_zero!() => {}
            integer_one!() => {
                for (x, y) in self.elements.iter_mut().zip(&w.elements) {
                    *x -= y << bits;
                }
            }
            integer_negative_one!() => {
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
