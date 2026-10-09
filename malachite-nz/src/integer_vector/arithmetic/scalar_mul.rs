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
use alloc::vec::Vec;
use core::ops::{Mul, MulAssign};
use malachite_base::num::arithmetic::traits::NegAssign;
use malachite_base::num::basic::traits::{One, Zero};

// Multiplies every element of `xs` by `c`, in place.
//
// Nothing is trimmed: when `c` is zero, every element becomes zero, and a caller holding the
// coefficients of a polynomial must trim them.
//
// # Worst-case complexity
// $T(n) = O(n \log n \log\log n)$
//
// $M(n) = O(n \log n)$
//
// where $T$ is time, $M$ is additional memory, and $n$ is the total number of bits of the elements
// of `xs` and of `c`.
//
// This is equivalent to `_fmpz_vec_scalar_mul_fmpz` from `fmpz_vec/scalar.c`, FLINT 3.6.0, where
// `poly1 == poly2`.
#[doc(hidden)]
pub fn integers_mul_scalar_assign(xs: &mut [Integer], c: &Integer) {
    match *c {
        integer_zero!() => xs.fill(Integer::ZERO),
        integer_one!() => {}
        integer_negative_one!() => {
            for x in xs {
                x.neg_assign();
            }
        }
        _ => {
            for x in xs {
                *x *= c;
            }
        }
    }
}

// Returns a `Vec` holding every element of `xs` multiplied by `c`.
//
// Nothing is trimmed: when `c` is zero, every element is zero, and a caller holding the
// coefficients of a polynomial must trim them.
//
// # Worst-case complexity
// $T(n) = O(n \log n \log\log n)$
//
// $M(n) = O(n \log n)$
//
// where $T$ is time, $M$ is additional memory, and $n$ is the total number of bits of the elements
// of `xs` and of `c`.
//
// This is equivalent to `_fmpz_vec_scalar_mul_fmpz` from `fmpz_vec/scalar.c`, FLINT 3.6.0.
crate_test_fn! {
#[doc(hidden)]
integers_mul_scalar(xs: &[Integer], c: &Integer) -> Vec<Integer> {
    match *c {
        integer_zero!() => alloc::vec![Integer::ZERO; xs.len()],
        integer_one!() => xs.to_vec(),
        integer_negative_one!() => xs.iter().map(|x| -x).collect(),
        _ => xs.iter().map(|x| x * c).collect(),
    }
}
}

// Sets each element of `out` to the element of `xs` at the same index multiplied by `c`. `xs` must
// be at least as long as `out`.
//
// # Worst-case complexity
// $T(n) = O(n \log n \log\log n)$
//
// $M(n) = O(n \log n)$
//
// where $T$ is time, $M$ is additional memory, and $n$ is the total number of bits of the first
// `out.len()` elements of `xs` and of `c`.
//
// This is equivalent to `_fmpz_vec_scalar_mul_fmpz` from `fmpz_vec/scalar.c`, FLINT 3.6.0, where
// the output is separate from the input.
crate_test_fn! {
#[doc(hidden)]
integers_mul_scalar_to_out(out: &mut [Integer], xs: &[Integer], c: &Integer) {
    let xs = &xs[..out.len()];
    match *c {
        integer_zero!() => out.fill(Integer::ZERO),
        integer_one!() => out.clone_from_slice(xs),
        integer_negative_one!() => {
            for (o, x) in out.iter_mut().zip(xs) {
                *o = -x;
            }
        }
        _ => {
            for (o, x) in out.iter_mut().zip(xs) {
                *o = x * c;
            }
        }
    }
}}

impl Mul<Integer> for IntegerVector {
    type Output = Self;

    /// Multiplies every element of an [`IntegerVector`] by an [`Integer`], taking the vector by
    /// value and the scalar by value.
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
    /// use malachite_nz::integer::Integer;
    /// use malachite_nz::integer_vector::IntegerVector;
    ///
    /// let v = IntegerVector::from_str("(1, -2, 3)").unwrap();
    /// let c = Integer::from(-3);
    /// assert_eq!((v * c).to_string(), "(-3, 6, -9)");
    /// ```
    ///
    /// This is equivalent to `_fmpz_vec_scalar_mul_fmpz` from `fmpz_vec/scalar.c`, FLINT 3.6.0.
    #[inline]
    fn mul(mut self, c: Integer) -> Self {
        self *= &c;
        self
    }
}

impl Mul<&Integer> for IntegerVector {
    type Output = Self;

    /// Multiplies every element of an [`IntegerVector`] by an [`Integer`], taking the vector by
    /// value and the scalar by reference.
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
    /// use malachite_nz::integer::Integer;
    /// use malachite_nz::integer_vector::IntegerVector;
    ///
    /// let v = IntegerVector::from_str("(1, -2, 3)").unwrap();
    /// let c = Integer::from(-3);
    /// assert_eq!((v * &c).to_string(), "(-3, 6, -9)");
    /// ```
    ///
    /// This is equivalent to `_fmpz_vec_scalar_mul_fmpz` from `fmpz_vec/scalar.c`, FLINT 3.6.0.
    #[inline]
    fn mul(mut self, c: &Integer) -> Self {
        self *= c;
        self
    }
}

impl Mul<Integer> for &IntegerVector {
    type Output = IntegerVector;

    /// Multiplies every element of an [`IntegerVector`] by an [`Integer`], taking the vector by
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
    /// use malachite_nz::integer::Integer;
    /// use malachite_nz::integer_vector::IntegerVector;
    ///
    /// let v = IntegerVector::from_str("(1, -2, 3)").unwrap();
    /// let c = Integer::from(-3);
    /// assert_eq!((&v * c).to_string(), "(-3, 6, -9)");
    /// ```
    ///
    /// This is equivalent to `_fmpz_vec_scalar_mul_fmpz` from `fmpz_vec/scalar.c`, FLINT 3.6.0.
    #[inline]
    fn mul(self, c: Integer) -> IntegerVector {
        self * &c
    }
}

impl Mul<&Integer> for &IntegerVector {
    type Output = IntegerVector;

    /// Multiplies every element of an [`IntegerVector`] by an [`Integer`], taking the vector by
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
    /// use malachite_nz::integer::Integer;
    /// use malachite_nz::integer_vector::IntegerVector;
    ///
    /// let v = IntegerVector::from_str("(1, -2, 3)").unwrap();
    /// let c = Integer::from(-3);
    /// assert_eq!((&v * &c).to_string(), "(-3, 6, -9)");
    /// ```
    ///
    /// This is equivalent to `_fmpz_vec_scalar_mul_fmpz` from `fmpz_vec/scalar.c`, FLINT 3.6.0.
    fn mul(self, c: &Integer) -> IntegerVector {
        IntegerVector {
            elements: integers_mul_scalar(&self.elements, c),
        }
    }
}

impl Mul<IntegerVector> for Integer {
    type Output = IntegerVector;

    /// Multiplies an [`Integer`] by every element of an [`IntegerVector`], taking the scalar by
    /// value and the vector by value.
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
    /// use malachite_nz::integer::Integer;
    /// use malachite_nz::integer_vector::IntegerVector;
    ///
    /// let v = IntegerVector::from_str("(1, -2, 3)").unwrap();
    /// let c = Integer::from(-3);
    /// assert_eq!((c * v).to_string(), "(-3, 6, -9)");
    /// ```
    ///
    /// This is equivalent to `_fmpz_vec_scalar_mul_fmpz` from `fmpz_vec/scalar.c`, FLINT 3.6.0.
    #[inline]
    fn mul(self, v: IntegerVector) -> IntegerVector {
        v * self
    }
}

impl Mul<&IntegerVector> for Integer {
    type Output = IntegerVector;

    /// Multiplies an [`Integer`] by every element of an [`IntegerVector`], taking the scalar by
    /// value and the vector by reference.
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
    /// use malachite_nz::integer::Integer;
    /// use malachite_nz::integer_vector::IntegerVector;
    ///
    /// let v = IntegerVector::from_str("(1, -2, 3)").unwrap();
    /// let c = Integer::from(-3);
    /// assert_eq!((c * &v).to_string(), "(-3, 6, -9)");
    /// ```
    ///
    /// This is equivalent to `_fmpz_vec_scalar_mul_fmpz` from `fmpz_vec/scalar.c`, FLINT 3.6.0.
    #[inline]
    fn mul(self, v: &IntegerVector) -> IntegerVector {
        v * self
    }
}

impl Mul<IntegerVector> for &Integer {
    type Output = IntegerVector;

    /// Multiplies an [`Integer`] by every element of an [`IntegerVector`], taking the scalar by
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
    /// use malachite_nz::integer::Integer;
    /// use malachite_nz::integer_vector::IntegerVector;
    ///
    /// let v = IntegerVector::from_str("(1, -2, 3)").unwrap();
    /// let c = Integer::from(-3);
    /// assert_eq!((&c * v).to_string(), "(-3, 6, -9)");
    /// ```
    ///
    /// This is equivalent to `_fmpz_vec_scalar_mul_fmpz` from `fmpz_vec/scalar.c`, FLINT 3.6.0.
    #[inline]
    fn mul(self, v: IntegerVector) -> IntegerVector {
        v * self
    }
}

impl Mul<&IntegerVector> for &Integer {
    type Output = IntegerVector;

    /// Multiplies an [`Integer`] by every element of an [`IntegerVector`], taking the scalar by
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
    /// use malachite_nz::integer::Integer;
    /// use malachite_nz::integer_vector::IntegerVector;
    ///
    /// let v = IntegerVector::from_str("(1, -2, 3)").unwrap();
    /// let c = Integer::from(-3);
    /// assert_eq!((&c * &v).to_string(), "(-3, 6, -9)");
    /// ```
    ///
    /// This is equivalent to `_fmpz_vec_scalar_mul_fmpz` from `fmpz_vec/scalar.c`, FLINT 3.6.0.
    #[inline]
    fn mul(self, v: &IntegerVector) -> IntegerVector {
        v * self
    }
}

impl MulAssign<Integer> for IntegerVector {
    /// Multiplies every element of an [`IntegerVector`] by an [`Integer`] in place, taking the
    /// scalar by value.
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
    /// use malachite_nz::integer::Integer;
    /// use malachite_nz::integer_vector::IntegerVector;
    ///
    /// let mut v = IntegerVector::from_str("(1, -2, 3)").unwrap();
    /// v *= Integer::from(-3);
    /// assert_eq!(v.to_string(), "(-3, 6, -9)");
    /// ```
    ///
    /// This is equivalent to `_fmpz_vec_scalar_mul_fmpz` from `fmpz_vec/scalar.c`, FLINT 3.6.0,
    /// with `vec1` equal to `vec2`.
    #[inline]
    fn mul_assign(&mut self, c: Integer) {
        *self *= &c;
    }
}

impl MulAssign<&Integer> for IntegerVector {
    /// Multiplies every element of an [`IntegerVector`] by an [`Integer`] in place, taking the
    /// scalar by reference.
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
    /// use malachite_nz::integer::Integer;
    /// use malachite_nz::integer_vector::IntegerVector;
    ///
    /// let mut v = IntegerVector::from_str("(1, -2, 3)").unwrap();
    /// v *= &Integer::from(-3);
    /// assert_eq!(v.to_string(), "(-3, 6, -9)");
    /// ```
    ///
    /// This is equivalent to `_fmpz_vec_scalar_mul_fmpz` from `fmpz_vec/scalar.c`, FLINT 3.6.0,
    /// with `vec1` equal to `vec2`.
    fn mul_assign(&mut self, c: &Integer) {
        integers_mul_scalar_assign(&mut self.elements, c);
    }
}
