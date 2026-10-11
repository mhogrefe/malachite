// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::natural::Natural;
use crate::natural_vector::NaturalVector;
use core::borrow::Borrow;
use malachite_base::num::arithmetic::traits::{ModIsReduced, ModSum};

fn assert_reduced(v: &NaturalVector, m: &Natural) {
    assert!(
        v.mod_is_reduced(m),
        "every vector must be reduced mod m, but {v} has an element >= {m}"
    );
}

const EMPTY: &str = "cannot sum an empty iterator of vectors: the dimension is unknown";

// Adds the remaining vectors to `sum`, the first, entry by entry and exactly, and then reduces each
// entry once: every entry of the exact sum is less than `m` times the number of vectors, so each
// final division has a short quotient.
fn finish<B: Borrow<NaturalVector>, I: Iterator<Item = B>>(
    mut sum: NaturalVector,
    xs: I,
    m: &Natural,
) -> NaturalVector {
    assert_ne!(*m, 0u32, "m cannot be zero");
    assert_reduced(&sum, m);
    for v in xs {
        let v = v.borrow();
        assert_eq!(
            v.elements.len(),
            sum.elements.len(),
            "cannot sum vectors of different dimensions"
        );
        assert_reduced(v, m);
        for (s, x) in sum.elements.iter_mut().zip(&v.elements) {
            *s += x;
        }
    }
    for s in &mut sum.elements {
        *s %= m;
    }
    sum
}

impl ModSum<Self, Natural> for NaturalVector {
    /// Adds up all the [`NaturalVector`]s in an iterator, entry by entry, modulo another
    /// [`Natural`] $m$, taking the vectors by value and the modulus by value. The vectors must all
    /// have the same dimension, and their elements must already be reduced modulo $m$.
    ///
    /// $f((v_i)_ {i=0}^{n-1}, m) = w$, where every element of $w$ is less than $m$ and $\sum_
    /// {i=0}^{n-1} v_i \equiv w \mod m$, entry by entry.
    ///
    /// # Worst-case complexity
    /// $T(n, k) = O(nk)$
    ///
    /// $M(d, k) = O(dk)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is the total number of elements of all the
    /// vectors, $d$ is the dimension of the vectors, and $k$ is `m.significant_bits()`.
    ///
    /// # Panics
    /// Panics if the iterator is empty, if the vectors do not all have the same dimension, if `m`
    /// is zero, or if any element is greater than or equal to `m`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::ModSum;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_vector::NaturalVector;
    ///
    /// let vs =
    ///     ["(5, 1, 3)", "(2, 6, 4)", "(6, 2, 5)"].map(|s| NaturalVector::from_str(s).unwrap());
    /// assert_eq!(
    ///     NaturalVector::mod_sum(vs.into_iter(), Natural::from(7u32)).to_string(),
    ///     "(6, 2, 5)"
    /// );
    ///
    /// let m = Natural::from_str("1000000000000000000000000000057").unwrap();
    /// let vs = [
    ///     "(1000000000000000000000000000056, 1)",
    ///     "(1000000000000000000000000000056, 1000000000000000000000000000055)",
    /// ]
    /// .map(|s| NaturalVector::from_str(s).unwrap());
    /// assert_eq!(
    ///     NaturalVector::mod_sum(vs.into_iter(), m).to_string(),
    ///     "(1000000000000000000000000000055, 1000000000000000000000000000056)"
    /// );
    /// ```
    fn mod_sum<I: Iterator<Item = Self>>(mut xs: I, m: Natural) -> Self {
        let first = xs.next().expect(EMPTY);
        finish(first, xs, &m)
    }
}

impl<'b> ModSum<Self, &'b Natural> for NaturalVector {
    /// Adds up all the [`NaturalVector`]s in an iterator, entry by entry, modulo another
    /// [`Natural`] $m$, taking the vectors by value and the modulus by reference. The vectors must
    /// all have the same dimension, and their elements must already be reduced modulo $m$.
    ///
    /// $f((v_i)_ {i=0}^{n-1}, m) = w$, where every element of $w$ is less than $m$ and $\sum_
    /// {i=0}^{n-1} v_i \equiv w \mod m$, entry by entry.
    ///
    /// # Worst-case complexity
    /// $T(n, k) = O(nk)$
    ///
    /// $M(d, k) = O(dk)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is the total number of elements of all the
    /// vectors, $d$ is the dimension of the vectors, and $k$ is `m.significant_bits()`.
    ///
    /// # Panics
    /// Panics if the iterator is empty, if the vectors do not all have the same dimension, if `m`
    /// is zero, or if any element is greater than or equal to `m`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::ModSum;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_vector::NaturalVector;
    ///
    /// let vs =
    ///     ["(5, 1, 3)", "(2, 6, 4)", "(6, 2, 5)"].map(|s| NaturalVector::from_str(s).unwrap());
    /// assert_eq!(
    ///     NaturalVector::mod_sum(vs.into_iter(), &Natural::from(7u32)).to_string(),
    ///     "(6, 2, 5)"
    /// );
    ///
    /// let m = Natural::from_str("1000000000000000000000000000057").unwrap();
    /// let vs = [
    ///     "(1000000000000000000000000000056, 1)",
    ///     "(1000000000000000000000000000056, 1000000000000000000000000000055)",
    /// ]
    /// .map(|s| NaturalVector::from_str(s).unwrap());
    /// assert_eq!(
    ///     NaturalVector::mod_sum(vs.into_iter(), &m).to_string(),
    ///     "(1000000000000000000000000000055, 1000000000000000000000000000056)"
    /// );
    /// ```
    fn mod_sum<I: Iterator<Item = Self>>(mut xs: I, m: &'b Natural) -> Self {
        let first = xs.next().expect(EMPTY);
        finish(first, xs, m)
    }
}

impl<'a> ModSum<&'a Self, Natural> for NaturalVector {
    /// Adds up all the [`NaturalVector`]s in an iterator, entry by entry, modulo another
    /// [`Natural`] $m$, taking the vectors by reference and the modulus by value. The vectors must
    /// all have the same dimension, and their elements must already be reduced modulo $m$.
    ///
    /// $f((v_i)_ {i=0}^{n-1}, m) = w$, where every element of $w$ is less than $m$ and $\sum_
    /// {i=0}^{n-1} v_i \equiv w \mod m$, entry by entry.
    ///
    /// # Worst-case complexity
    /// $T(n, k) = O(nk)$
    ///
    /// $M(d, k) = O(dk)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is the total number of elements of all the
    /// vectors, $d$ is the dimension of the vectors, and $k$ is `m.significant_bits()`.
    ///
    /// # Panics
    /// Panics if the iterator is empty, if the vectors do not all have the same dimension, if `m`
    /// is zero, or if any element is greater than or equal to `m`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::ModSum;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_vector::NaturalVector;
    ///
    /// let vs =
    ///     ["(5, 1, 3)", "(2, 6, 4)", "(6, 2, 5)"].map(|s| NaturalVector::from_str(s).unwrap());
    /// assert_eq!(
    ///     NaturalVector::mod_sum(vs.iter(), Natural::from(7u32)).to_string(),
    ///     "(6, 2, 5)"
    /// );
    ///
    /// let m = Natural::from_str("1000000000000000000000000000057").unwrap();
    /// let vs = [
    ///     "(1000000000000000000000000000056, 1)",
    ///     "(1000000000000000000000000000056, 1000000000000000000000000000055)",
    /// ]
    /// .map(|s| NaturalVector::from_str(s).unwrap());
    /// assert_eq!(
    ///     NaturalVector::mod_sum(vs.iter(), m).to_string(),
    ///     "(1000000000000000000000000000055, 1000000000000000000000000000056)"
    /// );
    /// ```
    fn mod_sum<I: Iterator<Item = &'a Self>>(mut xs: I, m: Natural) -> Self {
        let first = xs.next().expect(EMPTY);
        finish(first.clone(), xs, &m)
    }
}

impl<'a, 'b> ModSum<&'a Self, &'b Natural> for NaturalVector {
    /// Adds up all the [`NaturalVector`]s in an iterator, entry by entry, modulo another
    /// [`Natural`] $m$, taking the vectors by reference and the modulus by reference. The vectors
    /// must all have the same dimension, and their elements must already be reduced modulo $m$.
    ///
    /// $f((v_i)_ {i=0}^{n-1}, m) = w$, where every element of $w$ is less than $m$ and $\sum_
    /// {i=0}^{n-1} v_i \equiv w \mod m$, entry by entry.
    ///
    /// # Worst-case complexity
    /// $T(n, k) = O(nk)$
    ///
    /// $M(d, k) = O(dk)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is the total number of elements of all the
    /// vectors, $d$ is the dimension of the vectors, and $k$ is `m.significant_bits()`.
    ///
    /// # Panics
    /// Panics if the iterator is empty, if the vectors do not all have the same dimension, if `m`
    /// is zero, or if any element is greater than or equal to `m`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::ModSum;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_vector::NaturalVector;
    ///
    /// let vs =
    ///     ["(5, 1, 3)", "(2, 6, 4)", "(6, 2, 5)"].map(|s| NaturalVector::from_str(s).unwrap());
    /// assert_eq!(
    ///     NaturalVector::mod_sum(vs.iter(), &Natural::from(7u32)).to_string(),
    ///     "(6, 2, 5)"
    /// );
    ///
    /// let m = Natural::from_str("1000000000000000000000000000057").unwrap();
    /// let vs = [
    ///     "(1000000000000000000000000000056, 1)",
    ///     "(1000000000000000000000000000056, 1000000000000000000000000000055)",
    /// ]
    /// .map(|s| NaturalVector::from_str(s).unwrap());
    /// assert_eq!(
    ///     NaturalVector::mod_sum(vs.iter(), &m).to_string(),
    ///     "(1000000000000000000000000000055, 1000000000000000000000000000056)"
    /// );
    /// ```
    fn mod_sum<I: Iterator<Item = &'a Self>>(mut xs: I, m: &'b Natural) -> Self {
        let first = xs.next().expect(EMPTY);
        finish(first.clone(), xs, m)
    }
}
