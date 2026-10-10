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
use malachite_base::num::arithmetic::traits::{DivExact, DivExactAssign, NegAssign};
use malachite_base::num::basic::traits::{One, Zero};

impl DivExact<Integer> for IntegerVector {
    type Output = Self;

    /// Divides every element of an [`IntegerVector`] by an [`Integer`], taking the vector by value
    /// and the Integer by value. Every element must be exactly divisible by the [`Integer`].
    ///
    /// If some element is not exactly divisible by `c`, this function may panic or return a
    /// meaningless result. Dividing by 1 returns the vector unchanged, and dividing by $-1$ negates
    /// it.
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
    fn div_exact(mut self, c: Integer) -> Self {
        self.div_exact_assign(&c);
        self
    }
}

impl DivExact<&Integer> for IntegerVector {
    type Output = Self;

    /// Divides every element of an [`IntegerVector`] by an [`Integer`], taking the vector by value
    /// and the Integer by reference. Every element must be exactly divisible by the [`Integer`].
    ///
    /// If some element is not exactly divisible by `c`, this function may panic or return a
    /// meaningless result. Dividing by 1 returns the vector unchanged, and dividing by $-1$ negates
    /// it.
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
    fn div_exact(mut self, c: &Integer) -> Self {
        self.div_exact_assign(c);
        self
    }
}

impl DivExact<Integer> for &IntegerVector {
    type Output = IntegerVector;

    /// Divides every element of an [`IntegerVector`] by an [`Integer`], taking the vector by
    /// reference and the Integer by value. Every element must be exactly divisible by the
    /// [`Integer`].
    ///
    /// If some element is not exactly divisible by `c`, this function may panic or return a
    /// meaningless result. Dividing by 1 returns the vector unchanged, and dividing by $-1$ negates
    /// it.
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
    fn div_exact(self, c: Integer) -> IntegerVector {
        self.div_exact(&c)
    }
}

impl DivExact<&Integer> for &IntegerVector {
    type Output = IntegerVector;

    /// Divides every element of an [`IntegerVector`] by an [`Integer`], taking the vector by
    /// reference and the Integer by reference. Every element must be exactly divisible by the
    /// [`Integer`].
    ///
    /// If some element is not exactly divisible by `c`, this function may panic or return a
    /// meaningless result. Dividing by 1 returns the vector unchanged, and dividing by $-1$ negates
    /// it.
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
    fn div_exact(self, c: &Integer) -> IntegerVector {
        IntegerVector {
            elements: match *c {
                integer_zero!() => panic!("division by zero"),
                integer_one!() => self.elements.clone(),
                integer_negative_one!() => self.elements.iter().map(|x| -x).collect(),
                _ => self.elements.iter().map(|x| x.div_exact(c)).collect(),
            },
        }
    }
}

impl DivExactAssign<Integer> for IntegerVector {
    /// Divides every element of an [`IntegerVector`] by an [`Integer`], in place, taking the
    /// Integer by value. Every element must be exactly divisible by the [`Integer`].
    ///
    /// If some element is not exactly divisible by `c`, this function may panic or return a
    /// meaningless result. Dividing by 1 returns the vector unchanged, and dividing by $-1$ negates
    /// it.
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
    fn div_exact_assign(&mut self, c: Integer) {
        self.div_exact_assign(&c);
    }
}

impl DivExactAssign<&Integer> for IntegerVector {
    /// Divides every element of an [`IntegerVector`] by an [`Integer`], in place, taking the
    /// Integer by reference. Every element must be exactly divisible by the [`Integer`].
    ///
    /// If some element is not exactly divisible by `c`, this function may panic or return a
    /// meaningless result. Dividing by 1 returns the vector unchanged, and dividing by $-1$ negates
    /// it.
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
    fn div_exact_assign(&mut self, c: &Integer) {
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
                    x.div_exact_assign(c);
                }
            }
        }
    }
}
