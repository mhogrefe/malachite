// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::num::arithmetic::traits::{ModPowerOf2IsReduced, ModPowerOf2Sum};
use crate::num::basic::unsigneds::PrimitiveUnsigned;
use crate::unsigned_vector::UnsignedVector;
use core::borrow::Borrow;

fn assert_reduced<T: PrimitiveUnsigned>(v: &UnsignedVector<T>, pow: u64) {
    assert!(
        v.mod_power_of_2_is_reduced(pow),
        "every vector must be reduced mod 2^pow, but {v} has an element >= 2^{pow}"
    );
}

// The first vector fixes the dimension. Since $2^k$ divides $2^W$, where $W$ is the width of `T`,
// wrapping additions followed by a single reduction of each entry give the sum modulo $2^k$.
fn mod_power_of_2_sum<T: PrimitiveUnsigned, B: Borrow<UnsignedVector<T>>, I: Iterator<Item = B>>(
    mut xs: I,
    pow: u64,
) -> UnsignedVector<T> {
    assert!(
        pow <= T::WIDTH,
        "pow must be at most T::WIDTH, but it is {pow}"
    );
    let first = xs
        .next()
        .expect("cannot sum an empty iterator of vectors: the dimension is unknown");
    let first = first.borrow();
    assert_reduced(first, pow);
    let mut sum = first.clone();
    for v in xs {
        let v = v.borrow();
        assert_eq!(
            v.elements.len(),
            sum.elements.len(),
            "cannot sum vectors of different dimensions"
        );
        assert_reduced(v, pow);
        for (s, &x) in sum.elements.iter_mut().zip(&v.elements) {
            s.wrapping_add_assign(x);
        }
    }
    for s in &mut sum.elements {
        s.mod_power_of_2_assign(pow);
    }
    sum
}

impl<T: PrimitiveUnsigned> ModPowerOf2Sum for UnsignedVector<T> {
    /// Adds up all the [`UnsignedVector`]s in an iterator, entry by entry, modulo $2^k$. The
    /// vectors must all have the same dimension, and their elements must already be reduced modulo
    /// $2^k$.
    ///
    /// $f((v_i)_ {i=0}^{n-1}, k) = w$, where every element of $w$ is less than $2^k$ and $\sum_
    /// {i=0}^{n-1} v_i \equiv w \mod 2^k$, entry by entry.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(m) = O(m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is the total number of elements of all the
    /// vectors, and $m$ is the dimension of the vectors.
    ///
    /// # Panics
    /// Panics if the iterator is empty, if the vectors do not all have the same dimension, if `pow`
    /// is greater than `T::WIDTH`, or if any element is greater than or equal to $2^k$.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::ModPowerOf2Sum;
    /// use malachite_base::unsigned_vector::UnsignedVector;
    ///
    /// let vs = ["(5, 1, 3)", "(2, 7, 4)", "(6, 6, 6)"]
    ///     .map(|s| UnsignedVector::<u8>::from_str(s).unwrap());
    /// assert_eq!(
    ///     UnsignedVector::mod_power_of_2_sum(vs.into_iter(), 3).to_string(),
    ///     "(5, 6, 5)"
    /// );
    /// ```
    #[inline]
    fn mod_power_of_2_sum<I: Iterator<Item = Self>>(xs: I, pow: u64) -> Self {
        mod_power_of_2_sum(xs, pow)
    }
}

impl<'a, T: PrimitiveUnsigned> ModPowerOf2Sum<&'a Self> for UnsignedVector<T> {
    /// Adds up all the [`UnsignedVector`]s in an iterator of references, entry by entry, modulo
    /// $2^k$. The vectors must all have the same dimension, and their elements must already be
    /// reduced modulo $2^k$.
    ///
    /// $f((v_i)_ {i=0}^{n-1}, k) = w$, where every element of $w$ is less than $2^k$ and $\sum_
    /// {i=0}^{n-1} v_i \equiv w \mod 2^k$, entry by entry.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(m) = O(m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is the total number of elements of all the
    /// vectors, and $m$ is the dimension of the vectors.
    ///
    /// # Panics
    /// Panics if the iterator is empty, if the vectors do not all have the same dimension, if `pow`
    /// is greater than `T::WIDTH`, or if any element is greater than or equal to $2^k$.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::ModPowerOf2Sum;
    /// use malachite_base::unsigned_vector::UnsignedVector;
    ///
    /// let vs = ["(5, 1, 3)", "(2, 7, 4)", "(6, 6, 6)"]
    ///     .map(|s| UnsignedVector::<u8>::from_str(s).unwrap());
    /// assert_eq!(
    ///     UnsignedVector::mod_power_of_2_sum(vs.iter(), 3).to_string(),
    ///     "(5, 6, 5)"
    /// );
    /// ```
    #[inline]
    fn mod_power_of_2_sum<I: Iterator<Item = &'a Self>>(xs: I, pow: u64) -> Self {
        mod_power_of_2_sum(xs, pow)
    }
}
