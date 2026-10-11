// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::num::arithmetic::traits::{ModIsReduced, ModSum};
use crate::num::basic::unsigneds::PrimitiveUnsigned;
use crate::unsigned_vector::UnsignedVector;
use core::borrow::Borrow;

fn assert_reduced<T: PrimitiveUnsigned>(v: &UnsignedVector<T>, m: T) {
    assert!(
        v.mod_is_reduced(&m),
        "every vector must be reduced mod m, but {v} has an element >= {m}"
    );
}

// The first vector fixes the dimension, and the others are added to it entry by entry with
// `mod_add`. (Wrapping additions would not give the sum modulo `m` unless `m` divided $2^W$.)
fn mod_sum<T: PrimitiveUnsigned, B: Borrow<UnsignedVector<T>>, I: Iterator<Item = B>>(
    mut xs: I,
    m: T,
) -> UnsignedVector<T> {
    let first = xs
        .next()
        .expect("cannot sum an empty iterator of vectors: the dimension is unknown");
    let first = first.borrow();
    assert_reduced(first, m);
    let mut sum = first.clone();
    for v in xs {
        let v = v.borrow();
        assert_eq!(
            v.elements.len(),
            sum.elements.len(),
            "cannot sum vectors of different dimensions"
        );
        assert_reduced(v, m);
        for (s, &x) in sum.elements.iter_mut().zip(&v.elements) {
            s.mod_add_assign(x, m);
        }
    }
    sum
}

impl<T: PrimitiveUnsigned> ModSum<Self, T> for UnsignedVector<T> {
    /// Adds up all the [`UnsignedVector`]s in an iterator, entry by entry, modulo another number
    /// $m$. The vectors must all have the same dimension, and their elements must already be
    /// reduced modulo $m$.
    ///
    /// $f((v_i)_ {i=0}^{n-1}, m) = w$, where every element of $w$ is less than $m$ and $\sum_
    /// {i=0}^{n-1} v_i \equiv w \mod m$, entry by entry.
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
    /// Panics if the iterator is empty, if the vectors do not all have the same dimension, or if
    /// any element is greater than or equal to `m`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::ModSum;
    /// use malachite_base::unsigned_vector::UnsignedVector;
    ///
    /// let vs = ["(5, 1, 3)", "(2, 6, 4)", "(6, 2, 5)"]
    ///     .map(|s| UnsignedVector::<u8>::from_str(s).unwrap());
    /// assert_eq!(
    ///     UnsignedVector::mod_sum(vs.into_iter(), 7).to_string(),
    ///     "(6, 2, 5)"
    /// );
    /// ```
    #[inline]
    fn mod_sum<I: Iterator<Item = Self>>(xs: I, m: T) -> Self {
        mod_sum(xs, m)
    }
}

impl<'a, T: PrimitiveUnsigned> ModSum<&'a Self, T> for UnsignedVector<T> {
    /// Adds up all the [`UnsignedVector`]s in an iterator of references, entry by entry, modulo
    /// another number $m$. The vectors must all have the same dimension, and their elements must
    /// already be reduced modulo $m$.
    ///
    /// $f((v_i)_ {i=0}^{n-1}, m) = w$, where every element of $w$ is less than $m$ and $\sum_
    /// {i=0}^{n-1} v_i \equiv w \mod m$, entry by entry.
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
    /// Panics if the iterator is empty, if the vectors do not all have the same dimension, or if
    /// any element is greater than or equal to `m`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::ModSum;
    /// use malachite_base::unsigned_vector::UnsignedVector;
    ///
    /// let vs = ["(5, 1, 3)", "(2, 6, 4)", "(6, 2, 5)"]
    ///     .map(|s| UnsignedVector::<u8>::from_str(s).unwrap());
    /// assert_eq!(
    ///     UnsignedVector::mod_sum(vs.iter(), 7).to_string(),
    ///     "(6, 2, 5)"
    /// );
    /// ```
    #[inline]
    fn mod_sum<I: Iterator<Item = &'a Self>>(xs: I, m: T) -> Self {
        mod_sum(xs, m)
    }
}
