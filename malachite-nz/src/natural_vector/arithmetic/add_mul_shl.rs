// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::natural::Natural;
use crate::natural_vector::NaturalVector;
use malachite_base::num::arithmetic::traits::{AddMulAssign, AddMulShl, AddMulShlAssign};
use malachite_base::num::basic::traits::{One, Zero};

fn assert_same_dimension(v: &NaturalVector, w: &NaturalVector) {
    assert_eq!(
        v.elements.len(),
        w.elements.len(),
        "cannot add-multiply vectors of different dimensions"
    );
}

impl AddMulShl<Self, Natural> for NaturalVector {
    type Output = Self;

    /// Adds a scalar multiple of a [`NaturalVector`], shifted left by `bits`, to a
    /// [`NaturalVector`], taking both vectors and the scalar by value.
    ///
    /// The operation is taken element by element, so the result has the same dimension as the
    /// vectors. Like FLINT's, this special-cases multiplication by 0 and 1, and a shift by 0.
    ///
    /// $$
    /// f(v, w, c, k) = v + cw2^k.
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
    /// use malachite_base::num::arithmetic::traits::AddMulShl;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_vector::NaturalVector;
    ///
    /// let v = NaturalVector::from_str("(1, 2, 3)").unwrap();
    /// let w = NaturalVector::from_str("(4, 5, 6)").unwrap();
    /// assert_eq!(
    ///     v.add_mul_shl(w, Natural::from(10u32), 2).to_string(),
    ///     "(161, 202, 243)"
    /// );
    ///
    /// let v = NaturalVector::from_str("(1, 0)").unwrap();
    /// let w = NaturalVector::from_str("(18446744073709551615, 1)").unwrap();
    /// assert_eq!(
    ///     v.add_mul_shl(w, Natural::from_str("18446744073709551617").unwrap(), 3)
    ///         .to_string(),
    ///     "(2722258935367507707706996859454145691641, 147573952589676412936)"
    /// );
    /// ```
    ///
    /// This is equivalent to `_fmpz_vec_scalar_addmul_si_2exp` from `fmpz_vec/scalar.c`, FLINT
    /// 3.6.0.
    #[inline]
    fn add_mul_shl(mut self, w: Self, c: Natural, bits: u64) -> Self {
        self.add_mul_shl_assign(w, c, bits);
        self
    }
}

impl AddMulShl<Self, &Natural> for NaturalVector {
    type Output = Self;

    /// Adds a scalar multiple of a [`NaturalVector`], shifted left by `bits`, to a
    /// [`NaturalVector`], taking the first vector by value, the second by value, and the scalar by
    /// reference.
    ///
    /// See the documentation for the [`AddMulShl`] implementation that takes everything by value
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
    /// use malachite_base::num::arithmetic::traits::AddMulShl;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_vector::NaturalVector;
    ///
    /// let v = NaturalVector::from_str("(1, 2, 3)").unwrap();
    /// let w = NaturalVector::from_str("(4, 5, 6)").unwrap();
    /// assert_eq!(
    ///     v.add_mul_shl(w, &Natural::from(10u32), 2).to_string(),
    ///     "(161, 202, 243)"
    /// );
    ///
    /// let v = NaturalVector::from_str("(1, 0)").unwrap();
    /// let w = NaturalVector::from_str("(18446744073709551615, 1)").unwrap();
    /// assert_eq!(
    ///     v.add_mul_shl(w, &Natural::from_str("18446744073709551617").unwrap(), 3)
    ///         .to_string(),
    ///     "(2722258935367507707706996859454145691641, 147573952589676412936)"
    /// );
    /// ```
    ///
    /// This is equivalent to `_fmpz_vec_scalar_addmul_si_2exp` from `fmpz_vec/scalar.c`, FLINT
    /// 3.6.0.
    #[inline]
    fn add_mul_shl(mut self, w: Self, c: &Natural, bits: u64) -> Self {
        self.add_mul_shl_assign(w, c, bits);
        self
    }
}

impl AddMulShl<&Self, Natural> for NaturalVector {
    type Output = Self;

    /// Adds a scalar multiple of a [`NaturalVector`], shifted left by `bits`, to a
    /// [`NaturalVector`], taking the first vector by value, the second by reference, and the scalar
    /// by value.
    ///
    /// See the documentation for the [`AddMulShl`] implementation that takes everything by value
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
    /// use malachite_base::num::arithmetic::traits::AddMulShl;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_vector::NaturalVector;
    ///
    /// let v = NaturalVector::from_str("(1, 2, 3)").unwrap();
    /// let w = NaturalVector::from_str("(4, 5, 6)").unwrap();
    /// assert_eq!(
    ///     v.add_mul_shl(&w, Natural::from(10u32), 2).to_string(),
    ///     "(161, 202, 243)"
    /// );
    ///
    /// let v = NaturalVector::from_str("(1, 0)").unwrap();
    /// let w = NaturalVector::from_str("(18446744073709551615, 1)").unwrap();
    /// assert_eq!(
    ///     v.add_mul_shl(&w, Natural::from_str("18446744073709551617").unwrap(), 3)
    ///         .to_string(),
    ///     "(2722258935367507707706996859454145691641, 147573952589676412936)"
    /// );
    /// ```
    ///
    /// This is equivalent to `_fmpz_vec_scalar_addmul_si_2exp` from `fmpz_vec/scalar.c`, FLINT
    /// 3.6.0.
    #[inline]
    fn add_mul_shl(mut self, w: &Self, c: Natural, bits: u64) -> Self {
        self.add_mul_shl_assign(w, c, bits);
        self
    }
}

impl AddMulShl<&Self, &Natural> for NaturalVector {
    type Output = Self;

    /// Adds a scalar multiple of a [`NaturalVector`], shifted left by `bits`, to a
    /// [`NaturalVector`], taking the first vector by value, the second by reference, and the scalar
    /// by reference.
    ///
    /// See the documentation for the [`AddMulShl`] implementation that takes everything by value
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
    /// use malachite_base::num::arithmetic::traits::AddMulShl;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_vector::NaturalVector;
    ///
    /// let v = NaturalVector::from_str("(1, 2, 3)").unwrap();
    /// let w = NaturalVector::from_str("(4, 5, 6)").unwrap();
    /// assert_eq!(
    ///     v.add_mul_shl(&w, &Natural::from(10u32), 2).to_string(),
    ///     "(161, 202, 243)"
    /// );
    ///
    /// let v = NaturalVector::from_str("(1, 0)").unwrap();
    /// let w = NaturalVector::from_str("(18446744073709551615, 1)").unwrap();
    /// assert_eq!(
    ///     v.add_mul_shl(&w, &Natural::from_str("18446744073709551617").unwrap(), 3)
    ///         .to_string(),
    ///     "(2722258935367507707706996859454145691641, 147573952589676412936)"
    /// );
    /// ```
    ///
    /// This is equivalent to `_fmpz_vec_scalar_addmul_si_2exp` from `fmpz_vec/scalar.c`, FLINT
    /// 3.6.0.
    #[inline]
    fn add_mul_shl(mut self, w: &Self, c: &Natural, bits: u64) -> Self {
        self.add_mul_shl_assign(w, c, bits);
        self
    }
}

impl AddMulShl<&NaturalVector, &Natural> for &NaturalVector {
    type Output = NaturalVector;

    /// Adds a scalar multiple of a [`NaturalVector`], shifted left by `bits`, to a
    /// [`NaturalVector`], taking both vectors and the scalar by reference.
    ///
    /// See the documentation for the [`AddMulShl`] implementation that takes everything by value
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
    /// use malachite_base::num::arithmetic::traits::AddMulShl;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_vector::NaturalVector;
    ///
    /// let v = NaturalVector::from_str("(1, 2, 3)").unwrap();
    /// let w = NaturalVector::from_str("(4, 5, 6)").unwrap();
    /// assert_eq!(
    ///     (&v).add_mul_shl(&w, &Natural::from(10u32), 2).to_string(),
    ///     "(161, 202, 243)"
    /// );
    ///
    /// let v = NaturalVector::from_str("(1, 0)").unwrap();
    /// let w = NaturalVector::from_str("(18446744073709551615, 1)").unwrap();
    /// assert_eq!(
    ///     (&v).add_mul_shl(&w, &Natural::from_str("18446744073709551617").unwrap(), 3)
    ///         .to_string(),
    ///     "(2722258935367507707706996859454145691641, 147573952589676412936)"
    /// );
    /// ```
    ///
    /// This is equivalent to `_fmpz_vec_scalar_addmul_si_2exp` from `fmpz_vec/scalar.c`, FLINT
    /// 3.6.0.
    fn add_mul_shl(self, w: &NaturalVector, c: &Natural, bits: u64) -> NaturalVector {
        let mut v = self.clone();
        v.add_mul_shl_assign(w, c, bits);
        v
    }
}

impl AddMulShlAssign<Self, Natural> for NaturalVector {
    /// Adds a scalar multiple of a [`NaturalVector`], shifted left by `bits`, to a
    /// [`NaturalVector`], in place, taking the vector and the scalar on the right-hand side by
    /// value.
    ///
    /// See the documentation for the [`AddMulShl`] implementation that takes everything by value
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
    /// use malachite_base::num::arithmetic::traits::AddMulShlAssign;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_vector::NaturalVector;
    ///
    /// let mut v = NaturalVector::from_str("(1, 2, 3)").unwrap();
    /// let w = NaturalVector::from_str("(4, 5, 6)").unwrap();
    /// v.add_mul_shl_assign(w, Natural::from(10u32), 2);
    /// assert_eq!(v.to_string(), "(161, 202, 243)");
    ///
    /// let mut v = NaturalVector::from_str("(1, 0)").unwrap();
    /// let w = NaturalVector::from_str("(18446744073709551615, 1)").unwrap();
    /// v.add_mul_shl_assign(w, Natural::from_str("18446744073709551617").unwrap(), 3);
    /// assert_eq!(
    ///     v.to_string(),
    ///     "(2722258935367507707706996859454145691641, 147573952589676412936)"
    /// );
    /// ```
    ///
    /// This is equivalent to `_fmpz_vec_scalar_addmul_si_2exp` from `fmpz_vec/scalar.c`, FLINT
    /// 3.6.0.
    #[inline]
    fn add_mul_shl_assign(&mut self, w: Self, c: Natural, bits: u64) {
        self.add_mul_shl_assign(w, &c, bits);
    }
}

impl AddMulShlAssign<Self, &Natural> for NaturalVector {
    /// Adds a scalar multiple of a [`NaturalVector`], shifted left by `bits`, to a
    /// [`NaturalVector`], in place, taking the vector on the right-hand side by value and the
    /// scalar by reference.
    ///
    /// See the documentation for the [`AddMulShl`] implementation that takes everything by value
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
    /// use malachite_base::num::arithmetic::traits::AddMulShlAssign;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_vector::NaturalVector;
    ///
    /// let mut v = NaturalVector::from_str("(1, 2, 3)").unwrap();
    /// let w = NaturalVector::from_str("(4, 5, 6)").unwrap();
    /// v.add_mul_shl_assign(w, &Natural::from(10u32), 2);
    /// assert_eq!(v.to_string(), "(161, 202, 243)");
    ///
    /// let mut v = NaturalVector::from_str("(1, 0)").unwrap();
    /// let w = NaturalVector::from_str("(18446744073709551615, 1)").unwrap();
    /// v.add_mul_shl_assign(w, &Natural::from_str("18446744073709551617").unwrap(), 3);
    /// assert_eq!(
    ///     v.to_string(),
    ///     "(2722258935367507707706996859454145691641, 147573952589676412936)"
    /// );
    /// ```
    ///
    /// This is equivalent to `_fmpz_vec_scalar_addmul_si_2exp` from `fmpz_vec/scalar.c`, FLINT
    /// 3.6.0.
    fn add_mul_shl_assign(&mut self, w: Self, c: &Natural, bits: u64) {
        assert_same_dimension(self, &w);
        if bits == 0 {
            self.add_mul_assign(w, c);
            return;
        }
        match *c {
            Natural::ZERO => {}
            Natural::ONE => {
                for (x, mut y) in self.elements.iter_mut().zip(w.elements) {
                    y <<= bits;
                    *x += y;
                }
            }
            _ => {
                for (x, y) in self.elements.iter_mut().zip(w.elements) {
                    x.add_mul_shl_assign(y, c, bits);
                }
            }
        }
    }
}

impl AddMulShlAssign<&Self, Natural> for NaturalVector {
    /// Adds a scalar multiple of a [`NaturalVector`], shifted left by `bits`, to a
    /// [`NaturalVector`], in place, taking the vector on the right-hand side by reference and the
    /// scalar by value.
    ///
    /// See the documentation for the [`AddMulShl`] implementation that takes everything by value
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
    /// use malachite_base::num::arithmetic::traits::AddMulShlAssign;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_vector::NaturalVector;
    ///
    /// let mut v = NaturalVector::from_str("(1, 2, 3)").unwrap();
    /// let w = NaturalVector::from_str("(4, 5, 6)").unwrap();
    /// v.add_mul_shl_assign(&w, Natural::from(10u32), 2);
    /// assert_eq!(v.to_string(), "(161, 202, 243)");
    ///
    /// let mut v = NaturalVector::from_str("(1, 0)").unwrap();
    /// let w = NaturalVector::from_str("(18446744073709551615, 1)").unwrap();
    /// v.add_mul_shl_assign(&w, Natural::from_str("18446744073709551617").unwrap(), 3);
    /// assert_eq!(
    ///     v.to_string(),
    ///     "(2722258935367507707706996859454145691641, 147573952589676412936)"
    /// );
    /// ```
    ///
    /// This is equivalent to `_fmpz_vec_scalar_addmul_si_2exp` from `fmpz_vec/scalar.c`, FLINT
    /// 3.6.0.
    #[inline]
    fn add_mul_shl_assign(&mut self, w: &Self, c: Natural, bits: u64) {
        self.add_mul_shl_assign(w, &c, bits);
    }
}

impl AddMulShlAssign<&Self, &Natural> for NaturalVector {
    /// Adds a scalar multiple of a [`NaturalVector`], shifted left by `bits`, to a
    /// [`NaturalVector`], in place, taking the vector and the scalar on the right-hand side by
    /// reference.
    ///
    /// See the documentation for the [`AddMulShl`] implementation that takes everything by value
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
    /// use malachite_base::num::arithmetic::traits::AddMulShlAssign;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_vector::NaturalVector;
    ///
    /// let mut v = NaturalVector::from_str("(1, 2, 3)").unwrap();
    /// let w = NaturalVector::from_str("(4, 5, 6)").unwrap();
    /// v.add_mul_shl_assign(&w, &Natural::from(10u32), 2);
    /// assert_eq!(v.to_string(), "(161, 202, 243)");
    ///
    /// let mut v = NaturalVector::from_str("(1, 0)").unwrap();
    /// let w = NaturalVector::from_str("(18446744073709551615, 1)").unwrap();
    /// v.add_mul_shl_assign(&w, &Natural::from_str("18446744073709551617").unwrap(), 3);
    /// assert_eq!(
    ///     v.to_string(),
    ///     "(2722258935367507707706996859454145691641, 147573952589676412936)"
    /// );
    /// ```
    ///
    /// This is equivalent to `_fmpz_vec_scalar_addmul_si_2exp` from `fmpz_vec/scalar.c`, FLINT
    /// 3.6.0.
    fn add_mul_shl_assign(&mut self, w: &Self, c: &Natural, bits: u64) {
        assert_same_dimension(self, w);
        if bits == 0 {
            self.add_mul_assign(w, c);
            return;
        }
        match *c {
            Natural::ZERO => {}
            Natural::ONE => {
                for (x, y) in self.elements.iter_mut().zip(&w.elements) {
                    *x += y << bits;
                }
            }
            _ => {
                for (x, y) in self.elements.iter_mut().zip(&w.elements) {
                    x.add_mul_shl_assign(y, c, bits);
                }
            }
        }
    }
}
