// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::Rational;
use crate::rational_vector::RationalVector;
use core::ops::{Div, DivAssign};
use malachite_base::num::basic::traits::{One, Zero};

impl Div<Rational> for RationalVector {
    type Output = Self;

    /// Divides every element of a [`RationalVector`] by a [`Rational`], taking the vector by value
    /// and the scalar by value.
    ///
    /// The quotient is taken element by element, so the result has the same dimension as the
    /// vector.
    ///
    /// $$
    /// f(v, c) = \frac{v}{c} = (v_0/c, v_1/c, \ldots, v_{n-1}/c).
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
    /// # Panics
    /// Panics if `c` is zero.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_q::Rational;
    /// use malachite_q::rational_vector::RationalVector;
    ///
    /// let v = RationalVector::from_str("(1/2, -2/3, 3)").unwrap();
    /// let c = Rational::from_signeds(3, 4);
    /// assert_eq!((v / c).to_string(), "(2/3, -8/9, 4)");
    /// ```
    #[inline]
    fn div(mut self, c: Rational) -> Self {
        self /= &c;
        self
    }
}

impl Div<&Rational> for RationalVector {
    type Output = Self;

    /// Divides every element of a [`RationalVector`] by a [`Rational`], taking the vector by value
    /// and the scalar by reference.
    ///
    /// See the documentation for the [`Div`] implementation that takes the vector and the scalar by
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
    /// # Panics
    /// Panics if `c` is zero.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_q::Rational;
    /// use malachite_q::rational_vector::RationalVector;
    ///
    /// let v = RationalVector::from_str("(1/2, -2/3, 3)").unwrap();
    /// let c = Rational::from_signeds(3, 4);
    /// assert_eq!((v / &c).to_string(), "(2/3, -8/9, 4)");
    /// ```
    #[inline]
    fn div(mut self, c: &Rational) -> Self {
        self /= c;
        self
    }
}

impl Div<Rational> for &RationalVector {
    type Output = RationalVector;

    /// Divides every element of a [`RationalVector`] by a [`Rational`], taking the vector by
    /// reference and the scalar by value.
    ///
    /// See the documentation for the [`Div`] implementation that takes the vector and the scalar by
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
    /// # Panics
    /// Panics if `c` is zero.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_q::Rational;
    /// use malachite_q::rational_vector::RationalVector;
    ///
    /// let v = RationalVector::from_str("(1/2, -2/3, 3)").unwrap();
    /// let c = Rational::from_signeds(3, 4);
    /// assert_eq!((&v / c).to_string(), "(2/3, -8/9, 4)");
    /// ```
    #[inline]
    fn div(self, c: Rational) -> RationalVector {
        self / &c
    }
}

impl Div<&Rational> for &RationalVector {
    type Output = RationalVector;

    /// Divides every element of a [`RationalVector`] by a [`Rational`], taking the vector by
    /// reference and the scalar by reference.
    ///
    /// See the documentation for the [`Div`] implementation that takes the vector and the scalar by
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
    /// # Panics
    /// Panics if `c` is zero.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_q::Rational;
    /// use malachite_q::rational_vector::RationalVector;
    ///
    /// let v = RationalVector::from_str("(1/2, -2/3, 3)").unwrap();
    /// let c = Rational::from_signeds(3, 4);
    /// assert_eq!((&v / &c).to_string(), "(2/3, -8/9, 4)");
    /// ```
    fn div(self, c: &Rational) -> RationalVector {
        RationalVector {
            elements: match *c {
                Rational::ZERO => panic!("division by zero"),
                Rational::ONE => self.elements.clone(),
                // Each quotient is reduced by the cross-cancellation of `Rational`'s `Div`.
                _ => self.elements.iter().map(|x| x / c).collect(),
            },
        }
    }
}

impl DivAssign<Rational> for RationalVector {
    /// Divides every element of a [`RationalVector`] by a [`Rational`] in place, taking the scalar
    /// by value.
    ///
    /// See the documentation for the [`Div`] implementation that takes the vector and the scalar by
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
    /// # Panics
    /// Panics if `c` is zero.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_q::Rational;
    /// use malachite_q::rational_vector::RationalVector;
    ///
    /// let mut v = RationalVector::from_str("(1/2, -2/3, 3)").unwrap();
    /// v /= Rational::from_signeds(3, 4);
    /// assert_eq!(v.to_string(), "(2/3, -8/9, 4)");
    /// ```
    #[inline]
    fn div_assign(&mut self, c: Rational) {
        *self /= &c;
    }
}

impl DivAssign<&Rational> for RationalVector {
    /// Divides every element of a [`RationalVector`] by a [`Rational`] in place, taking the scalar
    /// by reference.
    ///
    /// See the documentation for the [`Div`] implementation that takes the vector and the scalar by
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
    /// # Panics
    /// Panics if `c` is zero.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_q::Rational;
    /// use malachite_q::rational_vector::RationalVector;
    ///
    /// let mut v = RationalVector::from_str("(1/2, -2/3, 3)").unwrap();
    /// v /= &Rational::from_signeds(3, 4);
    /// assert_eq!(v.to_string(), "(2/3, -8/9, 4)");
    /// ```
    fn div_assign(&mut self, c: &Rational) {
        match *c {
            Rational::ZERO => panic!("division by zero"),
            Rational::ONE => {}
            _ => {
                for x in &mut self.elements {
                    *x /= c;
                }
            }
        }
    }
}
