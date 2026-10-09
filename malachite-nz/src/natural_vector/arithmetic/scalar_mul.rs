// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::natural::Natural;
use crate::natural_vector::NaturalVector;
use alloc::vec;
use core::ops::{Mul, MulAssign};
use malachite_base::num::basic::traits::{One, Zero};

impl Mul<Natural> for NaturalVector {
    type Output = Self;

    /// Multiplies every element of a [`NaturalVector`] by a [`Natural`], taking the vector by value
    /// and the scalar by value.
    ///
    /// The product is taken element by element, so the result has the same dimension as the vector.
    ///
    /// $$
    /// f(v, c) = cv = (cv_0, cv_1, \ldots, cv_{n-1}).
    /// $$
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is the total number of bits of the
    /// elements and of the scalar.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_vector::NaturalVector;
    ///
    /// let v = NaturalVector::from_str("(1, 2, 3)").unwrap();
    /// let c = Natural::from(3u32);
    /// assert_eq!((v * c).to_string(), "(3, 6, 9)");
    /// ```
    ///
    /// This is equivalent to `_fmpz_vec_scalar_mul_fmpz` from `fmpz_vec/scalar.c`, FLINT 3.6.0.
    #[inline]
    fn mul(mut self, c: Natural) -> Self {
        self *= &c;
        self
    }
}

impl Mul<&Natural> for NaturalVector {
    type Output = Self;

    /// Multiplies every element of a [`NaturalVector`] by a [`Natural`], taking the vector by value
    /// and the scalar by reference.
    ///
    /// See the documentation for the [`Mul`] implementation that takes the vector and the scalar by
    /// value for details.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is the total number of bits of the
    /// elements and of the scalar.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_vector::NaturalVector;
    ///
    /// let v = NaturalVector::from_str("(1, 2, 3)").unwrap();
    /// let c = Natural::from(3u32);
    /// assert_eq!((v * &c).to_string(), "(3, 6, 9)");
    /// ```
    ///
    /// This is equivalent to `_fmpz_vec_scalar_mul_fmpz` from `fmpz_vec/scalar.c`, FLINT 3.6.0.
    #[inline]
    fn mul(mut self, c: &Natural) -> Self {
        self *= c;
        self
    }
}

impl Mul<Natural> for &NaturalVector {
    type Output = NaturalVector;

    /// Multiplies every element of a [`NaturalVector`] by a [`Natural`], taking the vector by
    /// reference and the scalar by value.
    ///
    /// See the documentation for the [`Mul`] implementation that takes the vector and the scalar by
    /// value for details.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is the total number of bits of the
    /// elements and of the scalar.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_vector::NaturalVector;
    ///
    /// let v = NaturalVector::from_str("(1, 2, 3)").unwrap();
    /// let c = Natural::from(3u32);
    /// assert_eq!((&v * c).to_string(), "(3, 6, 9)");
    /// ```
    ///
    /// This is equivalent to `_fmpz_vec_scalar_mul_fmpz` from `fmpz_vec/scalar.c`, FLINT 3.6.0.
    #[inline]
    fn mul(self, c: Natural) -> NaturalVector {
        self * &c
    }
}

impl Mul<&Natural> for &NaturalVector {
    type Output = NaturalVector;

    /// Multiplies every element of a [`NaturalVector`] by a [`Natural`], taking the vector by
    /// reference and the scalar by reference.
    ///
    /// See the documentation for the [`Mul`] implementation that takes the vector and the scalar by
    /// value for details.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is the total number of bits of the
    /// elements and of the scalar.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_vector::NaturalVector;
    ///
    /// let v = NaturalVector::from_str("(1, 2, 3)").unwrap();
    /// let c = Natural::from(3u32);
    /// assert_eq!((&v * &c).to_string(), "(3, 6, 9)");
    /// ```
    ///
    /// This is equivalent to `_fmpz_vec_scalar_mul_fmpz` from `fmpz_vec/scalar.c`, FLINT 3.6.0.
    fn mul(self, c: &Natural) -> NaturalVector {
        NaturalVector {
            elements: match *c {
                Natural::ZERO => vec![Natural::ZERO; self.elements.len()],
                Natural::ONE => self.elements.clone(),
                _ => self.elements.iter().map(|x| x * c).collect(),
            },
        }
    }
}

impl Mul<NaturalVector> for Natural {
    type Output = NaturalVector;

    /// Multiplies a [`Natural`] by every element of a [`NaturalVector`], taking the scalar by value
    /// and the vector by value.
    ///
    /// Scalar multiplication is commutative, so this is `v * c`; see the documentation for the
    /// [`Mul`] implementation that takes the vector and the scalar by value for details.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is the total number of bits of the
    /// elements and of the scalar.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_vector::NaturalVector;
    ///
    /// let v = NaturalVector::from_str("(1, 2, 3)").unwrap();
    /// let c = Natural::from(3u32);
    /// assert_eq!((c * v).to_string(), "(3, 6, 9)");
    /// ```
    ///
    /// This is equivalent to `_fmpz_vec_scalar_mul_fmpz` from `fmpz_vec/scalar.c`, FLINT 3.6.0.
    #[inline]
    fn mul(self, v: NaturalVector) -> NaturalVector {
        v * self
    }
}

impl Mul<&NaturalVector> for Natural {
    type Output = NaturalVector;

    /// Multiplies a [`Natural`] by every element of a [`NaturalVector`], taking the scalar by value
    /// and the vector by reference.
    ///
    /// Scalar multiplication is commutative, so this is `v * c`; see the documentation for the
    /// [`Mul`] implementation that takes the vector and the scalar by value for details.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is the total number of bits of the
    /// elements and of the scalar.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_vector::NaturalVector;
    ///
    /// let v = NaturalVector::from_str("(1, 2, 3)").unwrap();
    /// let c = Natural::from(3u32);
    /// assert_eq!((c * &v).to_string(), "(3, 6, 9)");
    /// ```
    ///
    /// This is equivalent to `_fmpz_vec_scalar_mul_fmpz` from `fmpz_vec/scalar.c`, FLINT 3.6.0.
    #[inline]
    fn mul(self, v: &NaturalVector) -> NaturalVector {
        v * self
    }
}

impl Mul<NaturalVector> for &Natural {
    type Output = NaturalVector;

    /// Multiplies a [`Natural`] by every element of a [`NaturalVector`], taking the scalar by
    /// reference and the vector by value.
    ///
    /// Scalar multiplication is commutative, so this is `v * c`; see the documentation for the
    /// [`Mul`] implementation that takes the vector and the scalar by value for details.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is the total number of bits of the
    /// elements and of the scalar.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_vector::NaturalVector;
    ///
    /// let v = NaturalVector::from_str("(1, 2, 3)").unwrap();
    /// let c = Natural::from(3u32);
    /// assert_eq!((&c * v).to_string(), "(3, 6, 9)");
    /// ```
    ///
    /// This is equivalent to `_fmpz_vec_scalar_mul_fmpz` from `fmpz_vec/scalar.c`, FLINT 3.6.0.
    #[inline]
    fn mul(self, v: NaturalVector) -> NaturalVector {
        v * self
    }
}

impl Mul<&NaturalVector> for &Natural {
    type Output = NaturalVector;

    /// Multiplies a [`Natural`] by every element of a [`NaturalVector`], taking the scalar by
    /// reference and the vector by reference.
    ///
    /// Scalar multiplication is commutative, so this is `v * c`; see the documentation for the
    /// [`Mul`] implementation that takes the vector and the scalar by value for details.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is the total number of bits of the
    /// elements and of the scalar.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_vector::NaturalVector;
    ///
    /// let v = NaturalVector::from_str("(1, 2, 3)").unwrap();
    /// let c = Natural::from(3u32);
    /// assert_eq!((&c * &v).to_string(), "(3, 6, 9)");
    /// ```
    ///
    /// This is equivalent to `_fmpz_vec_scalar_mul_fmpz` from `fmpz_vec/scalar.c`, FLINT 3.6.0.
    #[inline]
    fn mul(self, v: &NaturalVector) -> NaturalVector {
        v * self
    }
}

impl MulAssign<Natural> for NaturalVector {
    /// Multiplies every element of a [`NaturalVector`] by a [`Natural`] in place, taking the scalar
    /// by value.
    ///
    /// See the documentation for the [`Mul`] implementation that takes the vector and the scalar by
    /// value for details.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is the total number of bits of the
    /// elements and of the scalar.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_vector::NaturalVector;
    ///
    /// let mut v = NaturalVector::from_str("(1, 2, 3)").unwrap();
    /// v *= Natural::from(3u32);
    /// assert_eq!(v.to_string(), "(3, 6, 9)");
    /// ```
    ///
    /// This is equivalent to `_fmpz_vec_scalar_mul_fmpz` from `fmpz_vec/scalar.c`, FLINT 3.6.0,
    /// with `vec1` equal to `vec2`.
    #[inline]
    fn mul_assign(&mut self, c: Natural) {
        *self *= &c;
    }
}

impl MulAssign<&Natural> for NaturalVector {
    /// Multiplies every element of a [`NaturalVector`] by a [`Natural`] in place, taking the scalar
    /// by reference.
    ///
    /// See the documentation for the [`Mul`] implementation that takes the vector and the scalar by
    /// value for details.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is the total number of bits of the
    /// elements and of the scalar.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_vector::NaturalVector;
    ///
    /// let mut v = NaturalVector::from_str("(1, 2, 3)").unwrap();
    /// v *= &Natural::from(3u32);
    /// assert_eq!(v.to_string(), "(3, 6, 9)");
    /// ```
    ///
    /// This is equivalent to `_fmpz_vec_scalar_mul_fmpz` from `fmpz_vec/scalar.c`, FLINT 3.6.0,
    /// with `vec1` equal to `vec2`.
    fn mul_assign(&mut self, c: &Natural) {
        match *c {
            Natural::ZERO => self.elements.fill(Natural::ZERO),
            Natural::ONE => {}
            _ => {
                for x in &mut self.elements {
                    *x *= c;
                }
            }
        }
    }
}
