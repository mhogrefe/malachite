// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::num::arithmetic::traits::{ModDiv, ModIsReduced};
use crate::num::basic::unsigneds::PrimitiveUnsigned;
use crate::unsigned_vector::UnsignedVector;
use crate::unsigned_vector::arithmetic::mod_mul::{mod_mul_assign_unchecked, mod_mul_unchecked};
use alloc::vec::Vec;

fn assert_reduced<T: PrimitiveUnsigned>(v: &UnsignedVector<T>, c: T, m: T) {
    assert!(
        v.mod_is_reduced(&m),
        "self must be reduced mod m, but {v} has an element >= {m}"
    );
    assert!(
        c.mod_is_reduced(&m),
        "c must be reduced mod m, but {c} >= {m}"
    );
}

// The inverse of `c` modulo `m`, or `None` if `c` is not invertible. Modulo 1, 0 is invertible, but
// `mod_inverse` does not accept 0, so 0 always takes the element-by-element path, which is correct
// for every modulus.
fn inverse<T: PrimitiveUnsigned>(c: T, m: T) -> Option<T> {
    if c == T::ZERO { None } else { c.mod_inverse(m) }
}

// Each element divided by `c` modulo `m`, one element at a time, or `None` if some element has no
// quotient.
fn mod_div_elementwise<T: PrimitiveUnsigned>(xs: &[T], c: T, m: T) -> Option<Vec<T>> {
    xs.iter().map(|&x| x.mod_div(c, m)).collect()
}

impl<T: PrimitiveUnsigned> ModDiv<T, T> for UnsignedVector<T> {
    type Output = Self;

    /// Divides every element of an [`UnsignedVector`] by a number $c$ modulo $m$, taking the vector
    /// by value, and returning `None` if some element has no quotient. The elements and $c$ must
    /// already be reduced modulo $m$.
    ///
    /// If $c$ is invertible modulo $m$, which is the usual case, every element has exactly one
    /// quotient, and this computes the inverse of $c$ once and multiplies every element by it.
    /// Otherwise an element $x$ has a quotient if and only if $\gcd(c, m)$ divides $x$; the result
    /// is `None` if some element has none, and otherwise each element of the result is the quotient
    /// that [`ModDiv`] returns for that element alone.
    ///
    /// $f(v, c, m) = \operatorname{Some}(w)$, where every $w_i < m$ and $cw_i \equiv v_i \mod m$,
    /// if such $w_i$ exist.
    ///
    /// # Worst-case complexity
    /// $T(n, b) = O(nb)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `self.dimension()`, and $b$ is
    /// `m.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `m` is zero, or if `c` or any element of `self` is greater than or equal to `m`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::ModDiv;
    /// use malachite_base::unsigned_vector::UnsignedVector;
    ///
    /// let v = UnsignedVector::<u8>::from_str("(1, 2, 3)").unwrap();
    /// assert_eq!(v.clone().mod_div(3, 7).unwrap().to_string(), "(5, 3, 1)");
    /// // 2 is not invertible modulo 6, but every element is even.
    /// let v = UnsignedVector::<u8>::from_str("(2, 4, 0)").unwrap();
    /// assert_eq!(v.clone().mod_div(2, 6).unwrap().to_string(), "(1, 2, 0)");
    /// let v = UnsignedVector::<u8>::from_str("(2, 3)").unwrap();
    /// assert_eq!(v.clone().mod_div(2, 6), None);
    /// ```
    ///
    /// This is equivalent to `_fmpz_mod_vec_scalar_div_fmpz_mod` from `fmpz_mod_vec/scalar.c`,
    /// FLINT 3.6.0, with words in place of `fmpz`s, when $c$ is invertible; FLINT throws an
    /// exception otherwise.
    fn mod_div(mut self, c: T, m: T) -> Option<Self> {
        assert_reduced(&self, c, m);
        if let Some(inverse) = inverse(c, m) {
            mod_mul_assign_unchecked(&mut self.elements, inverse, m);
        } else {
            self.elements = mod_div_elementwise(&self.elements, c, m)?;
        }
        Some(self)
    }
}

impl<T: PrimitiveUnsigned> ModDiv<T, T> for &UnsignedVector<T> {
    type Output = UnsignedVector<T>;

    /// Divides every element of an [`UnsignedVector`] by a number $c$ modulo $m$, taking the vector
    /// by reference, and returning `None` if some element has no quotient. The elements and $c$
    /// must already be reduced modulo $m$.
    ///
    /// If $c$ is invertible modulo $m$, which is the usual case, every element has exactly one
    /// quotient, and this computes the inverse of $c$ once and multiplies every element by it.
    /// Otherwise an element $x$ has a quotient if and only if $\gcd(c, m)$ divides $x$; the result
    /// is `None` if some element has none, and otherwise each element of the result is the quotient
    /// that [`ModDiv`] returns for that element alone.
    ///
    /// $f(v, c, m) = \operatorname{Some}(w)$, where every $w_i < m$ and $cw_i \equiv v_i \mod m$,
    /// if such $w_i$ exist.
    ///
    /// # Worst-case complexity
    /// $T(n, b) = O(nb)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `self.dimension()`, and $b$ is
    /// `m.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `m` is zero, or if `c` or any element of `self` is greater than or equal to `m`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::ModDiv;
    /// use malachite_base::unsigned_vector::UnsignedVector;
    ///
    /// let v = UnsignedVector::<u8>::from_str("(1, 2, 3)").unwrap();
    /// assert_eq!((&v).mod_div(3, 7).unwrap().to_string(), "(5, 3, 1)");
    /// // 2 is not invertible modulo 6, but every element is even.
    /// let v = UnsignedVector::<u8>::from_str("(2, 4, 0)").unwrap();
    /// assert_eq!((&v).mod_div(2, 6).unwrap().to_string(), "(1, 2, 0)");
    /// let v = UnsignedVector::<u8>::from_str("(2, 3)").unwrap();
    /// assert_eq!((&v).mod_div(2, 6), None);
    /// ```
    ///
    /// This is equivalent to `_fmpz_mod_vec_scalar_div_fmpz_mod` from `fmpz_mod_vec/scalar.c`,
    /// FLINT 3.6.0, with words in place of `fmpz`s, when $c$ is invertible; FLINT throws an
    /// exception otherwise.
    fn mod_div(self, c: T, m: T) -> Option<UnsignedVector<T>> {
        assert_reduced(self, c, m);
        Some(UnsignedVector {
            elements: if let Some(inverse) = inverse(c, m) {
                mod_mul_unchecked(&self.elements, inverse, m)
            } else {
                mod_div_elementwise(&self.elements, c, m)?
            },
        })
    }
}
