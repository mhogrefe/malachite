// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::Rational;
use crate::rational_vector::RationalVector;
use alloc::vec;
use core::ops::{Mul, MulAssign};
use malachite_base::num::basic::traits::{One, Zero};

impl Mul<Rational> for RationalVector {
    type Output = Self;

    /// Multiplies every element of a [`RationalVector`] by a [`Rational`], taking the vector by
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
    /// use malachite_q::Rational;
    /// use malachite_q::rational_vector::RationalVector;
    ///
    /// let v = RationalVector::from_str("(1/2, -2/3, 3)").unwrap();
    /// let c = Rational::from_signeds(3, 4);
    /// assert_eq!((v * c).to_string(), "(3/8, -1/2, 9/4)");
    /// ```
    #[inline]
    fn mul(mut self, c: Rational) -> Self {
        self *= &c;
        self
    }
}

impl Mul<&Rational> for RationalVector {
    type Output = Self;

    /// Multiplies every element of a [`RationalVector`] by a [`Rational`], taking the vector by
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
    /// use malachite_q::Rational;
    /// use malachite_q::rational_vector::RationalVector;
    ///
    /// let v = RationalVector::from_str("(1/2, -2/3, 3)").unwrap();
    /// let c = Rational::from_signeds(3, 4);
    /// assert_eq!((v * &c).to_string(), "(3/8, -1/2, 9/4)");
    /// ```
    #[inline]
    fn mul(mut self, c: &Rational) -> Self {
        self *= c;
        self
    }
}

impl Mul<Rational> for &RationalVector {
    type Output = RationalVector;

    /// Multiplies every element of a [`RationalVector`] by a [`Rational`], taking the vector by
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
    /// use malachite_q::Rational;
    /// use malachite_q::rational_vector::RationalVector;
    ///
    /// let v = RationalVector::from_str("(1/2, -2/3, 3)").unwrap();
    /// let c = Rational::from_signeds(3, 4);
    /// assert_eq!((&v * c).to_string(), "(3/8, -1/2, 9/4)");
    /// ```
    #[inline]
    fn mul(self, c: Rational) -> RationalVector {
        self * &c
    }
}

impl Mul<&Rational> for &RationalVector {
    type Output = RationalVector;

    /// Multiplies every element of a [`RationalVector`] by a [`Rational`], taking the vector by
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
    /// use malachite_q::Rational;
    /// use malachite_q::rational_vector::RationalVector;
    ///
    /// let v = RationalVector::from_str("(1/2, -2/3, 3)").unwrap();
    /// let c = Rational::from_signeds(3, 4);
    /// assert_eq!((&v * &c).to_string(), "(3/8, -1/2, 9/4)");
    /// ```
    fn mul(self, c: &Rational) -> RationalVector {
        RationalVector {
            elements: match *c {
                Rational::ZERO => vec![Rational::ZERO; self.elements.len()],
                Rational::ONE => self.elements.clone(),
                // Each product is reduced by the cross-cancellation of `Rational`'s `Mul`.
                _ => self.elements.iter().map(|x| x * c).collect(),
            },
        }
    }
}

impl Mul<RationalVector> for Rational {
    type Output = RationalVector;

    /// Multiplies a [`Rational`] by every element of a [`RationalVector`], taking the scalar by
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
    /// use malachite_q::Rational;
    /// use malachite_q::rational_vector::RationalVector;
    ///
    /// let v = RationalVector::from_str("(1/2, -2/3, 3)").unwrap();
    /// let c = Rational::from_signeds(3, 4);
    /// assert_eq!((c * v).to_string(), "(3/8, -1/2, 9/4)");
    /// ```
    #[inline]
    fn mul(self, v: RationalVector) -> RationalVector {
        v * self
    }
}

impl Mul<&RationalVector> for Rational {
    type Output = RationalVector;

    /// Multiplies a [`Rational`] by every element of a [`RationalVector`], taking the scalar by
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
    /// use malachite_q::Rational;
    /// use malachite_q::rational_vector::RationalVector;
    ///
    /// let v = RationalVector::from_str("(1/2, -2/3, 3)").unwrap();
    /// let c = Rational::from_signeds(3, 4);
    /// assert_eq!((c * &v).to_string(), "(3/8, -1/2, 9/4)");
    /// ```
    #[inline]
    fn mul(self, v: &RationalVector) -> RationalVector {
        v * self
    }
}

impl Mul<RationalVector> for &Rational {
    type Output = RationalVector;

    /// Multiplies a [`Rational`] by every element of a [`RationalVector`], taking the scalar by
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
    /// use malachite_q::Rational;
    /// use malachite_q::rational_vector::RationalVector;
    ///
    /// let v = RationalVector::from_str("(1/2, -2/3, 3)").unwrap();
    /// let c = Rational::from_signeds(3, 4);
    /// assert_eq!((&c * v).to_string(), "(3/8, -1/2, 9/4)");
    /// ```
    #[inline]
    fn mul(self, v: RationalVector) -> RationalVector {
        v * self
    }
}

impl Mul<&RationalVector> for &Rational {
    type Output = RationalVector;

    /// Multiplies a [`Rational`] by every element of a [`RationalVector`], taking the scalar by
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
    /// use malachite_q::Rational;
    /// use malachite_q::rational_vector::RationalVector;
    ///
    /// let v = RationalVector::from_str("(1/2, -2/3, 3)").unwrap();
    /// let c = Rational::from_signeds(3, 4);
    /// assert_eq!((&c * &v).to_string(), "(3/8, -1/2, 9/4)");
    /// ```
    #[inline]
    fn mul(self, v: &RationalVector) -> RationalVector {
        v * self
    }
}

impl MulAssign<Rational> for RationalVector {
    /// Multiplies every element of a [`RationalVector`] by a [`Rational`] in place, taking the
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
    /// use malachite_q::Rational;
    /// use malachite_q::rational_vector::RationalVector;
    ///
    /// let mut v = RationalVector::from_str("(1/2, -2/3, 3)").unwrap();
    /// v *= Rational::from_signeds(3, 4);
    /// assert_eq!(v.to_string(), "(3/8, -1/2, 9/4)");
    /// ```
    #[inline]
    fn mul_assign(&mut self, c: Rational) {
        *self *= &c;
    }
}

impl MulAssign<&Rational> for RationalVector {
    /// Multiplies every element of a [`RationalVector`] by a [`Rational`] in place, taking the
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
    /// use malachite_q::Rational;
    /// use malachite_q::rational_vector::RationalVector;
    ///
    /// let mut v = RationalVector::from_str("(1/2, -2/3, 3)").unwrap();
    /// v *= &Rational::from_signeds(3, 4);
    /// assert_eq!(v.to_string(), "(3/8, -1/2, 9/4)");
    /// ```
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
