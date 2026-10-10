// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::natural::Natural;
use crate::natural_vector::NaturalVector;
use core::ops::{Div, DivAssign};
use malachite_base::num::basic::traits::{One, Zero};

impl Div<Natural> for NaturalVector {
    type Output = Self;

    /// Divides every element of a [`NaturalVector`] by a [`Natural`], taking the vector by value
    /// and the scalar by value.
    ///
    /// The quotient is taken element by element, so the result has the same dimension as the
    /// vector.
    ///
    /// Each quotient is rounded down.
    ///
    /// $$
    /// f(v, c)_i = \left\lfloor\frac{v_i}{c}\right\rfloor.
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
    fn div(mut self, c: Natural) -> Self {
        self /= &c;
        self
    }
}

impl Div<&Natural> for NaturalVector {
    type Output = Self;

    /// Divides every element of a [`NaturalVector`] by a [`Natural`], taking the vector by value
    /// and the scalar by reference.
    ///
    /// The quotient is taken element by element, so the result has the same dimension as the
    /// vector.
    ///
    /// Each quotient is rounded down.
    ///
    /// $$
    /// f(v, c)_i = \left\lfloor\frac{v_i}{c}\right\rfloor.
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
    fn div(mut self, c: &Natural) -> Self {
        self /= c;
        self
    }
}

impl Div<Natural> for &NaturalVector {
    type Output = NaturalVector;

    /// Divides every element of a [`NaturalVector`] by a [`Natural`], taking the vector by
    /// reference and the scalar by value.
    ///
    /// The quotient is taken element by element, so the result has the same dimension as the
    /// vector.
    ///
    /// Each quotient is rounded down.
    ///
    /// $$
    /// f(v, c)_i = \left\lfloor\frac{v_i}{c}\right\rfloor.
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
    fn div(self, c: Natural) -> NaturalVector {
        self / &c
    }
}

impl Div<&Natural> for &NaturalVector {
    type Output = NaturalVector;

    /// Divides every element of a [`NaturalVector`] by a [`Natural`], taking the vector by
    /// reference and the scalar by reference.
    ///
    /// The quotient is taken element by element, so the result has the same dimension as the
    /// vector.
    ///
    /// Each quotient is rounded down.
    ///
    /// $$
    /// f(v, c)_i = \left\lfloor\frac{v_i}{c}\right\rfloor.
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
    fn div(self, c: &Natural) -> NaturalVector {
        NaturalVector {
            elements: match *c {
                Natural::ZERO => panic!("division by zero"),
                Natural::ONE => self.elements.clone(),
                _ => self.elements.iter().map(|x| x / c).collect(),
            },
        }
    }
}

impl DivAssign<Natural> for NaturalVector {
    /// Divides every element of a [`NaturalVector`] by a [`Natural`], in place, taking the scalar
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
    fn div_assign(&mut self, c: Natural) {
        *self /= &c;
    }
}

impl DivAssign<&Natural> for NaturalVector {
    /// Divides every element of a [`NaturalVector`] by a [`Natural`], in place, taking the scalar
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
    fn div_assign(&mut self, c: &Natural) {
        match *c {
            Natural::ZERO => panic!("division by zero"),
            Natural::ONE => {}
            _ => {
                for x in &mut self.elements {
                    *x /= c;
                }
            }
        }
    }
}
