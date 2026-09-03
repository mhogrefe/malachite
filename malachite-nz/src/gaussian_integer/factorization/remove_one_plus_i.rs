// Copyright © 2026 Mikhail Hogrefe
//
// Uses code adopted from the FLINT Library.
//
//      Copyright © 2022 Fredrik Johansson
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::gaussian_integer::GaussianInteger;
use core::cmp::Ordering::*;
use core::mem::take;
use malachite_base::num::arithmetic::traits::{ModPowerOf2, MulIPowAssign};
use malachite_base::num::basic::traits::Zero;

// The largest power of 2 dividing both parts, and whether one more factor of 1 + i remains after
// that power of 2 (which is (1 + i)^2 up to a unit) is removed. The input must be nonzero.
fn one_plus_i_valuation(x: &GaussianInteger) -> (u64, bool) {
    match (x.real.trailing_zeros(), x.imaginary.trailing_zeros()) {
        (Some(s), None) | (None, Some(s)) => (s, false),
        (Some(s), Some(t)) => match s.cmp(&t) {
            // Both parts are odd after the shift, so their sum is even and 1 + i divides once more.
            Equal => (s, true),
            Less => (s, false),
            Greater => (t, false),
        },
        (None, None) => unreachable!(),
    }
}

// Given x with the common power of 2 already shifted out, fixes up the unit and removes the
// remaining factor of 1 + i if there is one, returning the reduced number and the exponent.
fn remove_one_plus_i_helper(mut x: GaussianInteger, s: u64, odd: bool) -> (GaussianInteger, u64) {
    if s != 0 {
        // Multiply by i^(-s), the unit left over when (1 + i)^(2s) = (2i)^s is removed by a shift.
        x.mul_i_pow_assign(4 - s.mod_power_of_2(2));
    }
    if odd {
        // (a + bi) / (1 + i) = ((a + b) + (b - a)i) / 2
        let t = &x.real + &x.imaginary;
        x.imaginary -= &x.real;
        x.real = t >> 1u64;
        x.imaginary >>= 1u64;
    }
    (x, (s << 1) | u64::from(odd))
}

impl GaussianInteger {
    /// Removes the largest power of $1 + i$ from a [`GaussianInteger`], taking it by reference and
    /// returning the reduced [`GaussianInteger`] together with the exponent of that power.
    ///
    /// $1 + i$ is the Gaussian prime above 2, with $(1 + i)^2 = 2i$. If $(1 + i)^k$ is the largest
    /// power of $1 + i$ that divides `self`, this returns $(\text{self} / (1 + i)^k, k)$. The
    /// exponent is twice the largest power of 2 dividing both parts, plus one more when the parts
    /// have the same 2-adic valuation, since then both are odd after the shift and their sum is
    /// even. Zero is left alone, with an exponent of 0, since every power of $1 + i$ divides it.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `self.significant_bits()`.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::basic::traits::Two;
    /// use malachite_nz::gaussian_integer::GaussianInteger;
    /// use std::str::FromStr;
    ///
    /// // 6+2i = (-1-2i)(1+i)^3
    /// let (q, k) = GaussianInteger::from_str("6+2i")
    ///     .unwrap()
    ///     .remove_one_plus_i();
    /// assert_eq!(q.to_string(), "-1-2i");
    /// assert_eq!(k, 3);
    ///
    /// // 2 = (-i)(1+i)^2
    /// let (q, k) = GaussianInteger::TWO.remove_one_plus_i();
    /// assert_eq!(q.to_string(), "-i");
    /// assert_eq!(k, 2);
    ///
    /// // 3+2i is not divisible by 1+i
    /// let (q, k) = GaussianInteger::from_str("3+2i")
    ///     .unwrap()
    ///     .remove_one_plus_i();
    /// assert_eq!(q.to_string(), "3+2i");
    /// assert_eq!(k, 0);
    /// ```
    pub fn remove_one_plus_i(&self) -> (Self, u64) {
        if *self == 0u32 {
            return (Self::ZERO, 0);
        }
        let (s, odd) = one_plus_i_valuation(self);
        let x = if s == 0 {
            self.clone()
        } else {
            Self {
                real: &self.real >> s,
                imaginary: &self.imaginary >> s,
            }
        };
        remove_one_plus_i_helper(x, s, odd)
    }

    /// Removes the largest power of $1 + i$ from a [`GaussianInteger`] in place, returning the
    /// exponent of that power.
    ///
    /// $1 + i$ is the Gaussian prime above 2, with $(1 + i)^2 = 2i$. If $(1 + i)^k$ is the largest
    /// power of $1 + i$ that divides `self`, this replaces `self` with $\text{self} / (1 + i)^k$
    /// and returns $k$. Zero is left alone, with an exponent of 0, since every power of $1 + i$
    /// divides it.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `self.significant_bits()`.
    ///
    /// # Examples
    /// ```
    /// use malachite_nz::gaussian_integer::GaussianInteger;
    /// use std::str::FromStr;
    ///
    /// // 6+2i = (-1-2i)(1+i)^3
    /// let mut x = GaussianInteger::from_str("6+2i").unwrap();
    /// assert_eq!(x.remove_one_plus_i_assign(), 3);
    /// assert_eq!(x.to_string(), "-1-2i");
    ///
    /// // 1000000000000 = 244140625 (1+i)^24
    /// let mut x = GaussianInteger::from(1000000000000u64);
    /// assert_eq!(x.remove_one_plus_i_assign(), 24);
    /// assert_eq!(x.to_string(), "244140625");
    /// ```
    pub fn remove_one_plus_i_assign(&mut self) -> u64 {
        if *self == 0u32 {
            return 0;
        }
        let (s, odd) = one_plus_i_valuation(self);
        if s != 0 {
            self.real >>= s;
            self.imaginary >>= s;
        }
        let (x, k) = remove_one_plus_i_helper(take(self), s, odd);
        *self = x;
        k
    }
}
