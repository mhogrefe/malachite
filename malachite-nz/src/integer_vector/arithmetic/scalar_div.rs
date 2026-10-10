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
use core::ops::{Div, DivAssign};
use malachite_base::num::arithmetic::traits::NegAssign;
use malachite_base::num::basic::traits::{One, Zero};

impl Div<Integer> for IntegerVector {
    type Output = Self;

    /// Divides every element of an [`IntegerVector`] by an [`Integer`], taking the vector by value
    /// and the scalar by value.
    ///
    /// The quotient is taken element by element, so the result has the same dimension as the
    /// vector.
    ///
    /// Each quotient is rounded toward zero, as by [`Div`] on [`Integer`]s.
    ///
    /// $$
    /// f(v, c)_i = \operatorname{sgn}(cv_i) \left\lfloor \left|\frac{v_i}{c}\right| \right\rfloor.
    /// $$
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is the total number of bits of the
    /// elements of elements and of `c`.
    ///
    /// # Panics
    /// Panics if `c` is zero.
    ///
    /// # Examples
    /// See [here](super::scalar_div#div).
    ///
    /// This is equivalent to `_fmpz_vec_scalar_tdiv_q_fmpz` from `fmpz_vec/scalar.c`, FLINT 3.6.0.
    #[inline]
    fn div(mut self, c: Integer) -> Self {
        self /= &c;
        self
    }
}

impl Div<&Integer> for IntegerVector {
    type Output = Self;

    /// Divides every element of an [`IntegerVector`] by an [`Integer`], taking the vector by value
    /// and the scalar by reference.
    ///
    /// The quotient is taken element by element, so the result has the same dimension as the
    /// vector.
    ///
    /// Each quotient is rounded toward zero, as by [`Div`] on [`Integer`]s.
    ///
    /// $$
    /// f(v, c)_i = \operatorname{sgn}(cv_i) \left\lfloor \left|\frac{v_i}{c}\right| \right\rfloor.
    /// $$
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is the total number of bits of the
    /// elements of elements and of `c`.
    ///
    /// # Panics
    /// Panics if `c` is zero.
    ///
    /// # Examples
    /// See [here](super::scalar_div#div).
    ///
    /// This is equivalent to `_fmpz_vec_scalar_tdiv_q_fmpz` from `fmpz_vec/scalar.c`, FLINT 3.6.0.
    #[inline]
    fn div(mut self, c: &Integer) -> Self {
        self /= c;
        self
    }
}

impl Div<Integer> for &IntegerVector {
    type Output = IntegerVector;

    /// Divides every element of an [`IntegerVector`] by an [`Integer`], taking the vector by
    /// reference and the scalar by value.
    ///
    /// The quotient is taken element by element, so the result has the same dimension as the
    /// vector.
    ///
    /// Each quotient is rounded toward zero, as by [`Div`] on [`Integer`]s.
    ///
    /// $$
    /// f(v, c)_i = \operatorname{sgn}(cv_i) \left\lfloor \left|\frac{v_i}{c}\right| \right\rfloor.
    /// $$
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is the total number of bits of the
    /// elements of elements and of `c`.
    ///
    /// # Panics
    /// Panics if `c` is zero.
    ///
    /// # Examples
    /// See [here](super::scalar_div#div).
    ///
    /// This is equivalent to `_fmpz_vec_scalar_tdiv_q_fmpz` from `fmpz_vec/scalar.c`, FLINT 3.6.0.
    #[inline]
    fn div(self, c: Integer) -> IntegerVector {
        self / &c
    }
}

impl Div<&Integer> for &IntegerVector {
    type Output = IntegerVector;

    /// Divides every element of an [`IntegerVector`] by an [`Integer`], taking the vector by
    /// reference and the scalar by reference.
    ///
    /// The quotient is taken element by element, so the result has the same dimension as the
    /// vector.
    ///
    /// Each quotient is rounded toward zero, as by [`Div`] on [`Integer`]s.
    ///
    /// $$
    /// f(v, c)_i = \operatorname{sgn}(cv_i) \left\lfloor \left|\frac{v_i}{c}\right| \right\rfloor.
    /// $$
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is the total number of bits of the
    /// elements of elements and of `c`.
    ///
    /// # Panics
    /// Panics if `c` is zero.
    ///
    /// # Examples
    /// See [here](super::scalar_div#div).
    ///
    /// This is equivalent to `_fmpz_vec_scalar_tdiv_q_fmpz` from `fmpz_vec/scalar.c`, FLINT 3.6.0.
    fn div(self, c: &Integer) -> IntegerVector {
        IntegerVector {
            elements: match *c {
                integer_zero!() => panic!("division by zero"),
                integer_one!() => self.elements.clone(),
                integer_negative_one!() => self.elements.iter().map(|x| -x).collect(),
                _ => self.elements.iter().map(|x| x / c).collect(),
            },
        }
    }
}

impl DivAssign<Integer> for IntegerVector {
    /// Divides every element of an [`IntegerVector`] by an [`Integer`], in place, taking the scalar
    /// by value.
    ///
    /// Every element is divided by `c`.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is the total number of bits of the
    /// elements of elements and of `c`.
    ///
    /// # Panics
    /// Panics if `c` is zero.
    ///
    /// # Examples
    /// See [here](super::scalar_div#div_assign).
    ///
    /// This is equivalent to `_fmpz_vec_scalar_tdiv_q_fmpz` from `fmpz_vec/scalar.c`, FLINT 3.6.0.
    #[inline]
    fn div_assign(&mut self, c: Integer) {
        *self /= &c;
    }
}

impl DivAssign<&Integer> for IntegerVector {
    /// Divides every element of an [`IntegerVector`] by an [`Integer`], in place, taking the scalar
    /// by reference.
    ///
    /// Every element is divided by `c`.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is the total number of bits of the
    /// elements of elements and of `c`.
    ///
    /// # Panics
    /// Panics if `c` is zero.
    ///
    /// # Examples
    /// See [here](super::scalar_div#div_assign).
    ///
    /// This is equivalent to `_fmpz_vec_scalar_tdiv_q_fmpz` from `fmpz_vec/scalar.c`, FLINT 3.6.0.
    fn div_assign(&mut self, c: &Integer) {
        match *c {
            integer_zero!() => panic!("division by zero"),
            integer_one!() => {}
            integer_negative_one!() => {
                for x in &mut self.elements {
                    x.neg_assign();
                }
            }
            _ => {
                for x in &mut self.elements {
                    *x /= c;
                }
            }
        }
    }
}
