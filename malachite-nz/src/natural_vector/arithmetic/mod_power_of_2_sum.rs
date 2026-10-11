// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::natural_vector::NaturalVector;
use core::borrow::Borrow;
use malachite_base::num::arithmetic::traits::{
    ModPowerOf2Assign, ModPowerOf2IsReduced, ModPowerOf2Sum,
};

fn assert_reduced(v: &NaturalVector, pow: u64) {
    assert!(
        v.mod_power_of_2_is_reduced(pow),
        "every vector must be reduced mod 2^pow, but {v} has an element >= 2^{pow}"
    );
}

fn assert_same_dimension(v: &NaturalVector, w: &NaturalVector) {
    assert_eq!(
        v.elements.len(),
        w.elements.len(),
        "cannot sum vectors of different dimensions"
    );
}

const EMPTY: &str = "cannot sum an empty iterator of vectors: the dimension is unknown";

// Adds the remaining vectors to `sum`, the first, entry by entry and exactly, and then reduces each
// entry once: the running sums exceed `pow` bits by only about the logarithm of the number of
// vectors, so this is cheaper than reducing after every addition.
fn finish<B: Borrow<NaturalVector>, I: Iterator<Item = B>>(
    mut sum: NaturalVector,
    xs: I,
    pow: u64,
) -> NaturalVector {
    assert_reduced(&sum, pow);
    for v in xs {
        let v = v.borrow();
        assert_same_dimension(v, &sum);
        assert_reduced(v, pow);
        for (s, x) in sum.elements.iter_mut().zip(&v.elements) {
            *s += x;
        }
    }
    for s in &mut sum.elements {
        s.mod_power_of_2_assign(pow);
    }
    sum
}

impl ModPowerOf2Sum for NaturalVector {
    /// Adds up all the [`NaturalVector`]s in an iterator, entry by entry, modulo $2^k$. The vectors
    /// must all have the same dimension, and their elements must already be reduced modulo $2^k$.
    ///
    /// $f((v_i)_ {i=0}^{n-1}, k) = w$, where every element of $w$ is less than $2^k$ and $\sum_
    /// {i=0}^{n-1} v_i \equiv w \mod 2^k$, entry by entry.
    ///
    /// # Worst-case complexity
    /// $T(n, k) = O(nk)$
    ///
    /// $M(m, k) = O(mk)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is the total number of elements of all the
    /// vectors, $m$ is the dimension of the vectors, and $k$ is `pow`.
    ///
    /// # Panics
    /// Panics if the iterator is empty, if the vectors do not all have the same dimension, or if
    /// any element is greater than or equal to $2^k$.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::ModPowerOf2Sum;
    /// use malachite_nz::natural_vector::NaturalVector;
    ///
    /// let vs =
    ///     ["(5, 1, 3)", "(2, 7, 4)", "(6, 6, 6)"].map(|s| NaturalVector::from_str(s).unwrap());
    /// assert_eq!(
    ///     NaturalVector::mod_power_of_2_sum(vs.into_iter(), 3).to_string(),
    ///     "(5, 6, 5)"
    /// );
    ///
    /// let vs = [
    ///     "(18446744073709551615, 1)",
    ///     "(18446744073709551615, 18446744073709551616)",
    ///     "(3, 36893488147419103231)",
    /// ]
    /// .map(|s| NaturalVector::from_str(s).unwrap());
    /// assert_eq!(
    ///     NaturalVector::mod_power_of_2_sum(vs.into_iter(), 65).to_string(),
    ///     "(1, 18446744073709551616)"
    /// );
    /// ```
    fn mod_power_of_2_sum<I: Iterator<Item = Self>>(mut xs: I, pow: u64) -> Self {
        let first = xs.next().expect(EMPTY);
        finish(first, xs, pow)
    }
}

impl<'a> ModPowerOf2Sum<&'a Self> for NaturalVector {
    /// Adds up all the [`NaturalVector`]s in an iterator of references, entry by entry, modulo
    /// $2^k$. The vectors must all have the same dimension, and their elements must already be
    /// reduced modulo $2^k$.
    ///
    /// $f((v_i)_ {i=0}^{n-1}, k) = w$, where every element of $w$ is less than $2^k$ and $\sum_
    /// {i=0}^{n-1} v_i \equiv w \mod 2^k$, entry by entry.
    ///
    /// # Worst-case complexity
    /// $T(n, k) = O(nk)$
    ///
    /// $M(m, k) = O(mk)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is the total number of elements of all the
    /// vectors, $m$ is the dimension of the vectors, and $k$ is `pow`.
    ///
    /// # Panics
    /// Panics if the iterator is empty, if the vectors do not all have the same dimension, or if
    /// any element is greater than or equal to $2^k$.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::ModPowerOf2Sum;
    /// use malachite_nz::natural_vector::NaturalVector;
    ///
    /// let vs =
    ///     ["(5, 1, 3)", "(2, 7, 4)", "(6, 6, 6)"].map(|s| NaturalVector::from_str(s).unwrap());
    /// assert_eq!(
    ///     NaturalVector::mod_power_of_2_sum(vs.iter(), 3).to_string(),
    ///     "(5, 6, 5)"
    /// );
    ///
    /// let vs = [
    ///     "(18446744073709551615, 1)",
    ///     "(18446744073709551615, 18446744073709551616)",
    ///     "(3, 36893488147419103231)",
    /// ]
    /// .map(|s| NaturalVector::from_str(s).unwrap());
    /// assert_eq!(
    ///     NaturalVector::mod_power_of_2_sum(vs.iter(), 65).to_string(),
    ///     "(1, 18446744073709551616)"
    /// );
    /// ```
    fn mod_power_of_2_sum<I: Iterator<Item = &'a Self>>(mut xs: I, pow: u64) -> Self {
        let first = xs.next().expect(EMPTY);
        finish(first.clone(), xs, pow)
    }
}
