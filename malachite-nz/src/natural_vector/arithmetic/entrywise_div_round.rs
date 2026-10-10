// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::natural::Natural;
use crate::natural_vector::NaturalVector;
use malachite_base::num::arithmetic::traits::{
    DivRound, DivRoundAssign, EntrywiseDivRound, EntrywiseDivRoundAssign,
};
use malachite_base::num::basic::traits::{One, Zero};
use malachite_base::rounding_modes::RoundingMode;

impl EntrywiseDivRound<Natural> for NaturalVector {
    type Output = Self;

    /// Divides every element of a [`NaturalVector`] by a [`Natural`], taking the vector by value
    /// and the scalar by value, and rounds every quotient according to the specified rounding mode.
    ///
    /// Each quotient is rounded as by
    /// [`DivRound`](malachite_base::num::arithmetic::traits::DivRound), but no
    /// [`Ordering`](core::cmp::Ordering) is returned, since with `Nearest` different elements may
    /// be rounded in different directions. Passing `Floor` or `Down` is equivalent to using `/`.
    ///
    /// $f(v, c, r)_i = \operatorname{round}(v_i/c)$, where $\operatorname{round}$ rounds according
    /// to `rm`.
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
    /// Panics if `c` is zero, or if `rm` is `Exact` and some element is not divisible by `c`.
    ///
    /// # Examples
    /// See [here](super::entrywise_div_round#entrywise_div_round).
    ///
    /// With `rm` equal to `Floor`, this is equivalent to `_fmpz_vec_scalar_fdiv_q_fmpz` from
    /// `fmpz_vec/scalar.c`, FLINT 3.6.0.
    #[inline]
    fn entrywise_div_round(mut self, c: Natural, rm: RoundingMode) -> Self {
        self.entrywise_div_round_assign(&c, rm);
        self
    }
}

impl EntrywiseDivRound<&Natural> for NaturalVector {
    type Output = Self;

    /// Divides every element of a [`NaturalVector`] by a [`Natural`], taking the vector by value
    /// and the scalar by reference, and rounds every quotient according to the specified rounding
    /// mode.
    ///
    /// Each quotient is rounded as by
    /// [`DivRound`](malachite_base::num::arithmetic::traits::DivRound), but no
    /// [`Ordering`](core::cmp::Ordering) is returned, since with `Nearest` different elements may
    /// be rounded in different directions. Passing `Floor` or `Down` is equivalent to using `/`.
    ///
    /// $f(v, c, r)_i = \operatorname{round}(v_i/c)$, where $\operatorname{round}$ rounds according
    /// to `rm`.
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
    /// Panics if `c` is zero, or if `rm` is `Exact` and some element is not divisible by `c`.
    ///
    /// # Examples
    /// See [here](super::entrywise_div_round#entrywise_div_round).
    ///
    /// With `rm` equal to `Floor`, this is equivalent to `_fmpz_vec_scalar_fdiv_q_fmpz` from
    /// `fmpz_vec/scalar.c`, FLINT 3.6.0.
    #[inline]
    fn entrywise_div_round(mut self, c: &Natural, rm: RoundingMode) -> Self {
        self.entrywise_div_round_assign(c, rm);
        self
    }
}

impl EntrywiseDivRound<Natural> for &NaturalVector {
    type Output = NaturalVector;

    /// Divides every element of a [`NaturalVector`] by a [`Natural`], taking the vector by
    /// reference and the scalar by value, and rounds every quotient according to the specified
    /// rounding mode.
    ///
    /// Each quotient is rounded as by
    /// [`DivRound`](malachite_base::num::arithmetic::traits::DivRound), but no
    /// [`Ordering`](core::cmp::Ordering) is returned, since with `Nearest` different elements may
    /// be rounded in different directions. Passing `Floor` or `Down` is equivalent to using `/`.
    ///
    /// $f(v, c, r)_i = \operatorname{round}(v_i/c)$, where $\operatorname{round}$ rounds according
    /// to `rm`.
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
    /// Panics if `c` is zero, or if `rm` is `Exact` and some element is not divisible by `c`.
    ///
    /// # Examples
    /// See [here](super::entrywise_div_round#entrywise_div_round).
    ///
    /// With `rm` equal to `Floor`, this is equivalent to `_fmpz_vec_scalar_fdiv_q_fmpz` from
    /// `fmpz_vec/scalar.c`, FLINT 3.6.0.
    #[inline]
    fn entrywise_div_round(self, c: Natural, rm: RoundingMode) -> NaturalVector {
        self.entrywise_div_round(&c, rm)
    }
}

impl EntrywiseDivRound<&Natural> for &NaturalVector {
    type Output = NaturalVector;

    /// Divides every element of a [`NaturalVector`] by a [`Natural`], taking the vector by
    /// reference and the scalar by reference, and rounds every quotient according to the specified
    /// rounding mode.
    ///
    /// Each quotient is rounded as by
    /// [`DivRound`](malachite_base::num::arithmetic::traits::DivRound), but no
    /// [`Ordering`](core::cmp::Ordering) is returned, since with `Nearest` different elements may
    /// be rounded in different directions. Passing `Floor` or `Down` is equivalent to using `/`.
    ///
    /// $f(v, c, r)_i = \operatorname{round}(v_i/c)$, where $\operatorname{round}$ rounds according
    /// to `rm`.
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
    /// Panics if `c` is zero, or if `rm` is `Exact` and some element is not divisible by `c`.
    ///
    /// # Examples
    /// See [here](super::entrywise_div_round#entrywise_div_round).
    ///
    /// With `rm` equal to `Floor`, this is equivalent to `_fmpz_vec_scalar_fdiv_q_fmpz` from
    /// `fmpz_vec/scalar.c`, FLINT 3.6.0.
    fn entrywise_div_round(self, c: &Natural, rm: RoundingMode) -> NaturalVector {
        NaturalVector {
            elements: match *c {
                Natural::ZERO => panic!("division by zero"),
                Natural::ONE => self.elements.clone(),
                _ => self.elements.iter().map(|x| x.div_round(c, rm).0).collect(),
            },
        }
    }
}

impl EntrywiseDivRoundAssign<Natural> for NaturalVector {
    /// Divides every element of a [`NaturalVector`] by a [`Natural`], in place, taking the scalar
    /// by value, and rounds every quotient according to the specified rounding mode.
    ///
    /// Each quotient is rounded as by
    /// [`DivRound`](malachite_base::num::arithmetic::traits::DivRound), but no
    /// [`Ordering`](core::cmp::Ordering) is returned, since with `Nearest` different elements may
    /// be rounded in different directions. Passing `Floor` or `Down` is equivalent to using `/`.
    ///
    /// $v_i \gets \operatorname{round}(v_i/c)$, where $\operatorname{round}$ rounds according to
    /// `rm`.
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
    /// Panics if `c` is zero, or if `rm` is `Exact` and some element is not divisible by `c`.
    ///
    /// # Examples
    /// See [here](super::entrywise_div_round#entrywise_div_round_assign).
    ///
    /// With `rm` equal to `Floor`, this is equivalent to `_fmpz_vec_scalar_fdiv_q_fmpz` from
    /// `fmpz_vec/scalar.c`, FLINT 3.6.0.
    #[inline]
    fn entrywise_div_round_assign(&mut self, c: Natural, rm: RoundingMode) {
        self.entrywise_div_round_assign(&c, rm);
    }
}

impl EntrywiseDivRoundAssign<&Natural> for NaturalVector {
    /// Divides every element of a [`NaturalVector`] by a [`Natural`], in place, taking the scalar
    /// by reference, and rounds every quotient according to the specified rounding mode.
    ///
    /// Each quotient is rounded as by
    /// [`DivRound`](malachite_base::num::arithmetic::traits::DivRound), but no
    /// [`Ordering`](core::cmp::Ordering) is returned, since with `Nearest` different elements may
    /// be rounded in different directions. Passing `Floor` or `Down` is equivalent to using `/`.
    ///
    /// $v_i \gets \operatorname{round}(v_i/c)$, where $\operatorname{round}$ rounds according to
    /// `rm`.
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
    /// Panics if `c` is zero, or if `rm` is `Exact` and some element is not divisible by `c`.
    ///
    /// # Examples
    /// See [here](super::entrywise_div_round#entrywise_div_round_assign).
    ///
    /// With `rm` equal to `Floor`, this is equivalent to `_fmpz_vec_scalar_fdiv_q_fmpz` from
    /// `fmpz_vec/scalar.c`, FLINT 3.6.0.
    fn entrywise_div_round_assign(&mut self, c: &Natural, rm: RoundingMode) {
        match *c {
            Natural::ZERO => panic!("division by zero"),
            Natural::ONE => {}
            _ => {
                for x in &mut self.elements {
                    x.div_round_assign(c, rm);
                }
            }
        }
    }
}
