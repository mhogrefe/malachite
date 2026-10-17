// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::Rational;
use crate::rational_matrix::RationalMatrix;
use alloc::vec;
use core::ops::{Mul, MulAssign};
use malachite_base::matrix::Matrix;
use malachite_base::num::basic::traits::{One, Zero};

impl Mul<Rational> for RationalMatrix {
    type Output = Self;

    /// Multiplies every element of a [`RationalMatrix`] by a [`Rational`], taking the matrix by
    /// value and the scalar by value.
    ///
    /// The product is taken element by element, so the result has the same shape as the matrix.
    ///
    /// $$
    /// f(A, c) = cA, \quad \text{where} \quad (cA)_{ij} = cA_{ij}.
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
    /// use malachite_q::Rational;
    /// use malachite_q::rational_matrix::RationalMatrix;
    ///
    /// let a = RationalMatrix::from_str("[1/2, -2/3; 3, 0]").unwrap();
    /// let c = Rational::from_signeds(3, 4);
    /// assert_eq!((a * c).to_string(), "[3/8, -1/2; 9/4, 0]");
    /// ```
    #[inline]
    fn mul(mut self, c: Rational) -> Self {
        self *= &c;
        self
    }
}

impl Mul<&Rational> for RationalMatrix {
    type Output = Self;

    /// Multiplies every element of a [`RationalMatrix`] by a [`Rational`], taking the matrix by
    /// value and the scalar by reference.
    ///
    /// See the documentation for the [`Mul`] implementation that takes the matrix and the scalar by
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
    /// use malachite_q::Rational;
    /// use malachite_q::rational_matrix::RationalMatrix;
    ///
    /// let a = RationalMatrix::from_str("[1/2, -2/3; 3, 0]").unwrap();
    /// let c = Rational::from_signeds(3, 4);
    /// assert_eq!((a * &c).to_string(), "[3/8, -1/2; 9/4, 0]");
    /// ```
    #[inline]
    fn mul(mut self, c: &Rational) -> Self {
        self *= c;
        self
    }
}

impl Mul<Rational> for &RationalMatrix {
    type Output = RationalMatrix;

    /// Multiplies every element of a [`RationalMatrix`] by a [`Rational`], taking the matrix by
    /// reference and the scalar by value.
    ///
    /// See the documentation for the [`Mul`] implementation that takes the matrix and the scalar by
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
    /// use malachite_q::Rational;
    /// use malachite_q::rational_matrix::RationalMatrix;
    ///
    /// let a = RationalMatrix::from_str("[1/2, -2/3; 3, 0]").unwrap();
    /// let c = Rational::from_signeds(3, 4);
    /// assert_eq!((&a * c).to_string(), "[3/8, -1/2; 9/4, 0]");
    /// ```
    #[inline]
    fn mul(self, c: Rational) -> RationalMatrix {
        self * &c
    }
}

impl Mul<&Rational> for &RationalMatrix {
    type Output = RationalMatrix;

    /// Multiplies every element of a [`RationalMatrix`] by a [`Rational`], taking the matrix by
    /// reference and the scalar by reference.
    ///
    /// See the documentation for the [`Mul`] implementation that takes the matrix and the scalar by
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
    /// use malachite_q::Rational;
    /// use malachite_q::rational_matrix::RationalMatrix;
    ///
    /// let a = RationalMatrix::from_str("[1/2, -2/3; 3, 0]").unwrap();
    /// let c = Rational::from_signeds(3, 4);
    /// assert_eq!((&a * &c).to_string(), "[3/8, -1/2; 9/4, 0]");
    /// ```
    #[inline]
    fn mul(self, c: &Rational) -> RationalMatrix {
        RationalMatrix::from_owned_elements(
            self.height(),
            self.width(),
            match *c {
                Rational::ZERO => vec![Rational::ZERO; self.elements.len()],
                Rational::ONE => self.elements.clone(),
                // Each product is reduced by the cross-cancellation of `Rational`'s `Mul`.
                _ => self.elements.iter().map(|x| x * c).collect(),
            },
        )
    }
}

impl Mul<RationalMatrix> for Rational {
    type Output = RationalMatrix;

    /// Multiplies a [`Rational`] by every element of a [`RationalMatrix`], taking the scalar by
    /// value and the matrix by value.
    ///
    /// Scalar multiplication is commutative, so this is `a * c`; see the documentation for the
    /// [`Mul`] implementation that takes the matrix and the scalar by value for details.
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
    /// use malachite_q::Rational;
    /// use malachite_q::rational_matrix::RationalMatrix;
    ///
    /// let a = RationalMatrix::from_str("[1/2, -2/3; 3, 0]").unwrap();
    /// let c = Rational::from_signeds(3, 4);
    /// assert_eq!((c * a).to_string(), "[3/8, -1/2; 9/4, 0]");
    /// ```
    #[inline]
    fn mul(self, v: RationalMatrix) -> RationalMatrix {
        v * self
    }
}

impl Mul<&RationalMatrix> for Rational {
    type Output = RationalMatrix;

    /// Multiplies a [`Rational`] by every element of a [`RationalMatrix`], taking the scalar by
    /// value and the matrix by reference.
    ///
    /// Scalar multiplication is commutative, so this is `a * c`; see the documentation for the
    /// [`Mul`] implementation that takes the matrix and the scalar by value for details.
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
    /// use malachite_q::Rational;
    /// use malachite_q::rational_matrix::RationalMatrix;
    ///
    /// let a = RationalMatrix::from_str("[1/2, -2/3; 3, 0]").unwrap();
    /// let c = Rational::from_signeds(3, 4);
    /// assert_eq!((c * &a).to_string(), "[3/8, -1/2; 9/4, 0]");
    /// ```
    #[inline]
    fn mul(self, v: &RationalMatrix) -> RationalMatrix {
        v * self
    }
}

impl Mul<RationalMatrix> for &Rational {
    type Output = RationalMatrix;

    /// Multiplies a [`Rational`] by every element of a [`RationalMatrix`], taking the scalar by
    /// reference and the matrix by value.
    ///
    /// Scalar multiplication is commutative, so this is `a * c`; see the documentation for the
    /// [`Mul`] implementation that takes the matrix and the scalar by value for details.
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
    /// use malachite_q::Rational;
    /// use malachite_q::rational_matrix::RationalMatrix;
    ///
    /// let a = RationalMatrix::from_str("[1/2, -2/3; 3, 0]").unwrap();
    /// let c = Rational::from_signeds(3, 4);
    /// assert_eq!((&c * a).to_string(), "[3/8, -1/2; 9/4, 0]");
    /// ```
    #[inline]
    fn mul(self, v: RationalMatrix) -> RationalMatrix {
        v * self
    }
}

impl Mul<&RationalMatrix> for &Rational {
    type Output = RationalMatrix;

    /// Multiplies a [`Rational`] by every element of a [`RationalMatrix`], taking the scalar by
    /// reference and the matrix by reference.
    ///
    /// Scalar multiplication is commutative, so this is `a * c`; see the documentation for the
    /// [`Mul`] implementation that takes the matrix and the scalar by value for details.
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
    /// use malachite_q::Rational;
    /// use malachite_q::rational_matrix::RationalMatrix;
    ///
    /// let a = RationalMatrix::from_str("[1/2, -2/3; 3, 0]").unwrap();
    /// let c = Rational::from_signeds(3, 4);
    /// assert_eq!((&c * &a).to_string(), "[3/8, -1/2; 9/4, 0]");
    /// ```
    #[inline]
    fn mul(self, v: &RationalMatrix) -> RationalMatrix {
        v * self
    }
}

impl MulAssign<Rational> for RationalMatrix {
    /// Multiplies every element of a [`RationalMatrix`] by a [`Rational`] in place, taking the
    /// scalar by value.
    ///
    /// See the documentation for the [`Mul`] implementation that takes the matrix and the scalar by
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
    /// use malachite_q::Rational;
    /// use malachite_q::rational_matrix::RationalMatrix;
    ///
    /// let mut a = RationalMatrix::from_str("[1/2, -2/3; 3, 0]").unwrap();
    /// a *= Rational::from_signeds(3, 4);
    /// assert_eq!(a.to_string(), "[3/8, -1/2; 9/4, 0]");
    /// ```
    #[inline]
    fn mul_assign(&mut self, c: Rational) {
        *self *= &c;
    }
}

impl MulAssign<&Rational> for RationalMatrix {
    /// Multiplies every element of a [`RationalMatrix`] by a [`Rational`] in place, taking the
    /// scalar by reference.
    ///
    /// See the documentation for the [`Mul`] implementation that takes the matrix and the scalar by
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
    /// use malachite_q::Rational;
    /// use malachite_q::rational_matrix::RationalMatrix;
    ///
    /// let mut a = RationalMatrix::from_str("[1/2, -2/3; 3, 0]").unwrap();
    /// a *= &Rational::from_signeds(3, 4);
    /// assert_eq!(a.to_string(), "[3/8, -1/2; 9/4, 0]");
    /// ```
    #[inline]
    fn mul_assign(&mut self, c: &Rational) {
        match *c {
            Rational::ZERO => self.elements.fill(Rational::ZERO),
            Rational::ONE => {}
            _ => {
                for x in &mut self.elements {
                    *x *= c;
                }
            }
        }
    }
}
