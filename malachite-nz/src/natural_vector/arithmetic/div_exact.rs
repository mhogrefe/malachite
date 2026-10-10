// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::natural::Natural;
use crate::natural_vector::NaturalVector;
use malachite_base::num::arithmetic::traits::{DivExact, DivExactAssign};
use malachite_base::num::basic::traits::{One, Zero};

impl DivExact<Natural> for NaturalVector {
    type Output = Self;

    /// Divides every element of a [`NaturalVector`] by a [`Natural`], taking the vector by value
    /// and the Natural by value. Every element must be exactly divisible by the [`Natural`].
    ///
    /// If some element is not exactly divisible by `c`, this function may panic or return a
    /// meaningless result. Dividing by 1 returns the vector unchanged.
    ///
    /// $f(v, c) = v/c$.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is the total number of bits of the
    /// elements and of `c`.
    ///
    /// # Panics
    /// Panics if `c` is zero.
    ///
    /// # Examples
    /// See [here](super::div_exact#div_exact).
    ///
    /// This is equivalent to `_fmpz_vec_scalar_divexact_fmpz` from `fmpz_vec/scalar_divexact.c`,
    /// FLINT 3.6.0.
    #[inline]
    fn div_exact(mut self, c: Natural) -> Self {
        self.div_exact_assign(&c);
        self
    }
}

impl DivExact<&Natural> for NaturalVector {
    type Output = Self;

    /// Divides every element of a [`NaturalVector`] by a [`Natural`], taking the vector by value
    /// and the Natural by reference. Every element must be exactly divisible by the [`Natural`].
    ///
    /// If some element is not exactly divisible by `c`, this function may panic or return a
    /// meaningless result. Dividing by 1 returns the vector unchanged.
    ///
    /// $f(v, c) = v/c$.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is the total number of bits of the
    /// elements and of `c`.
    ///
    /// # Panics
    /// Panics if `c` is zero.
    ///
    /// # Examples
    /// See [here](super::div_exact#div_exact).
    ///
    /// This is equivalent to `_fmpz_vec_scalar_divexact_fmpz` from `fmpz_vec/scalar_divexact.c`,
    /// FLINT 3.6.0.
    #[inline]
    fn div_exact(mut self, c: &Natural) -> Self {
        self.div_exact_assign(c);
        self
    }
}

impl DivExact<Natural> for &NaturalVector {
    type Output = NaturalVector;

    /// Divides every element of a [`NaturalVector`] by a [`Natural`], taking the vector by
    /// reference and the Natural by value. Every element must be exactly divisible by the
    /// [`Natural`].
    ///
    /// If some element is not exactly divisible by `c`, this function may panic or return a
    /// meaningless result. Dividing by 1 returns the vector unchanged.
    ///
    /// $f(v, c) = v/c$.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is the total number of bits of the
    /// elements and of `c`.
    ///
    /// # Panics
    /// Panics if `c` is zero.
    ///
    /// # Examples
    /// See [here](super::div_exact#div_exact).
    ///
    /// This is equivalent to `_fmpz_vec_scalar_divexact_fmpz` from `fmpz_vec/scalar_divexact.c`,
    /// FLINT 3.6.0.
    #[inline]
    fn div_exact(self, c: Natural) -> NaturalVector {
        self.div_exact(&c)
    }
}

impl DivExact<&Natural> for &NaturalVector {
    type Output = NaturalVector;

    /// Divides every element of a [`NaturalVector`] by a [`Natural`], taking the vector by
    /// reference and the Natural by reference. Every element must be exactly divisible by the
    /// [`Natural`].
    ///
    /// If some element is not exactly divisible by `c`, this function may panic or return a
    /// meaningless result. Dividing by 1 returns the vector unchanged.
    ///
    /// $f(v, c) = v/c$.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is the total number of bits of the
    /// elements and of `c`.
    ///
    /// # Panics
    /// Panics if `c` is zero.
    ///
    /// # Examples
    /// See [here](super::div_exact#div_exact).
    ///
    /// This is equivalent to `_fmpz_vec_scalar_divexact_fmpz` from `fmpz_vec/scalar_divexact.c`,
    /// FLINT 3.6.0.
    fn div_exact(self, c: &Natural) -> NaturalVector {
        NaturalVector {
            elements: match *c {
                Natural::ZERO => panic!("division by zero"),
                Natural::ONE => self.elements.clone(),
                _ => self.elements.iter().map(|x| x.div_exact(c)).collect(),
            },
        }
    }
}

impl DivExactAssign<Natural> for NaturalVector {
    /// Divides every element of a [`NaturalVector`] by a [`Natural`], in place, taking the Natural
    /// by value. Every element must be exactly divisible by the [`Natural`].
    ///
    /// If some element is not exactly divisible by `c`, this function may panic or return a
    /// meaningless result. Dividing by 1 returns the vector unchanged.
    ///
    /// $v \gets v/c$.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is the total number of bits of the
    /// elements and of `c`.
    ///
    /// # Panics
    /// Panics if `c` is zero.
    ///
    /// # Examples
    /// See [here](super::div_exact#div_exact_assign).
    ///
    /// This is equivalent to `_fmpz_vec_scalar_divexact_fmpz` from `fmpz_vec/scalar_divexact.c`,
    /// FLINT 3.6.0.
    #[inline]
    fn div_exact_assign(&mut self, c: Natural) {
        self.div_exact_assign(&c);
    }
}

impl DivExactAssign<&Natural> for NaturalVector {
    /// Divides every element of a [`NaturalVector`] by a [`Natural`], in place, taking the Natural
    /// by reference. Every element must be exactly divisible by the [`Natural`].
    ///
    /// If some element is not exactly divisible by `c`, this function may panic or return a
    /// meaningless result. Dividing by 1 returns the vector unchanged.
    ///
    /// $v \gets v/c$.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is the total number of bits of the
    /// elements and of `c`.
    ///
    /// # Panics
    /// Panics if `c` is zero.
    ///
    /// # Examples
    /// See [here](super::div_exact#div_exact_assign).
    ///
    /// This is equivalent to `_fmpz_vec_scalar_divexact_fmpz` from `fmpz_vec/scalar_divexact.c`,
    /// FLINT 3.6.0.
    fn div_exact_assign(&mut self, c: &Natural) {
        match *c {
            Natural::ZERO => panic!("division by zero"),
            Natural::ONE => {}
            _ => {
                for x in &mut self.elements {
                    x.div_exact_assign(c);
                }
            }
        }
    }
}
