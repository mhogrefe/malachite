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
use core::mem::take;
use malachite_base::num::arithmetic::traits::{MulIPow, Pow, PowAssign, Square, SquareAssign};
use malachite_base::num::basic::traits::One;
use malachite_base::num::logic::traits::{BitAccess, SignificantBits};

// A port of `fmpzi_pow_ui`: a purely real or purely imaginary base is a real power times a unit,
// and anything else is binary exponentiation, which squares once per bit of the exponent and
// multiplies by the base once per set bit. The by-value and by-reference helpers differ only in
// what they consume; the general case needs the base throughout, so both borrow it there.

// The base has two nonzero parts and `exp` is at least 3. Starting from the square rather than from
// a copy of the base saves a clone.
fn pow_general(x: &GaussianInteger, exp: u64) -> GaussianInteger {
    let bits = exp.significant_bits();
    let mut power = x.square();
    if exp.get_bit(bits - 2) {
        power *= x;
    }
    for i in (0..bits - 2).rev() {
        power.square_assign();
        if exp.get_bit(i) {
            power *= x;
        }
    }
    power
}

fn pow_val(x: GaussianInteger, exp: u64) -> GaussianInteger {
    match exp {
        0 => GaussianInteger::ONE,
        1 => x,
        2 => x.square(),
        _ if x.imaginary == 0u32 => GaussianInteger::from(x.real.pow(exp)),
        // (bi)^n = b^n i^n
        _ if x.real == 0u32 => GaussianInteger::from(x.imaginary.pow(exp)).mul_i_pow(exp),
        _ => pow_general(&x, exp),
    }
}

fn pow_ref(x: &GaussianInteger, exp: u64) -> GaussianInteger {
    match exp {
        0 => GaussianInteger::ONE,
        1 => x.clone(),
        2 => x.square(),
        _ if x.imaginary == 0u32 => GaussianInteger::from((&x.real).pow(exp)),
        // (bi)^n = b^n i^n
        _ if x.real == 0u32 => GaussianInteger::from((&x.imaginary).pow(exp)).mul_i_pow(exp),
        _ => pow_general(x, exp),
    }
}

impl Pow<u64> for GaussianInteger {
    type Output = Self;

    /// Raises a [`GaussianInteger`] to a power, taking the [`GaussianInteger`] by value.
    ///
    /// $f(x, n) = x^n$.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(nm \log (nm) \log\log (nm))$
    ///
    /// $M(n, m) = O(nm \log (nm))$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is the maximum number of significant bits
    /// of the real and imaginary parts of `self`, and $m$ is `exp`.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::Pow;
    /// use malachite_nz::gaussian_integer::GaussianInteger;
    /// use std::str::FromStr;
    ///
    /// assert_eq!(
    ///     GaussianInteger::from_str("2+i").unwrap().pow(5).to_string(),
    ///     "-38+41i"
    /// );
    /// assert_eq!(
    ///     GaussianInteger::from_str("1+i")
    ///         .unwrap()
    ///         .pow(10)
    ///         .to_string(),
    ///     "32i"
    /// );
    /// assert_eq!(
    ///     GaussianInteger::from_str("-7+24i")
    ///         .unwrap()
    ///         .pow(4)
    ///         .to_string(),
    ///     "164833+354144i"
    /// );
    /// ```
    #[inline]
    fn pow(self, exp: u64) -> Self {
        pow_val(self, exp)
    }
}

impl Pow<u64> for &GaussianInteger {
    type Output = GaussianInteger;

    /// Raises a [`GaussianInteger`] to a power, taking the [`GaussianInteger`] by reference.
    ///
    /// $f(x, n) = x^n$.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(nm \log (nm) \log\log (nm))$
    ///
    /// $M(n, m) = O(nm \log (nm))$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is the maximum number of significant bits
    /// of the real and imaginary parts of `self`, and $m$ is `exp`.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::Pow;
    /// use malachite_nz::gaussian_integer::GaussianInteger;
    /// use std::str::FromStr;
    ///
    /// assert_eq!(
    ///     (&GaussianInteger::from_str("2+i").unwrap())
    ///         .pow(5)
    ///         .to_string(),
    ///     "-38+41i"
    /// );
    /// assert_eq!(
    ///     (&GaussianInteger::from_str("1+i").unwrap())
    ///         .pow(10)
    ///         .to_string(),
    ///     "32i"
    /// );
    /// assert_eq!(
    ///     (&GaussianInteger::from_str("-7+24i").unwrap())
    ///         .pow(4)
    ///         .to_string(),
    ///     "164833+354144i"
    /// );
    /// ```
    #[inline]
    fn pow(self, exp: u64) -> GaussianInteger {
        pow_ref(self, exp)
    }
}

impl PowAssign<u64> for GaussianInteger {
    /// Raises a [`GaussianInteger`] to a power in place.
    ///
    /// $x \gets x^n$.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(nm \log (nm) \log\log (nm))$
    ///
    /// $M(n, m) = O(nm \log (nm))$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is the maximum number of significant bits
    /// of the real and imaginary parts of `self`, and $m$ is `exp`.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::PowAssign;
    /// use malachite_nz::gaussian_integer::GaussianInteger;
    /// use std::str::FromStr;
    ///
    /// let mut x = GaussianInteger::from_str("2+i").unwrap();
    /// x.pow_assign(5);
    /// assert_eq!(x.to_string(), "-38+41i");
    ///
    /// let mut x = GaussianInteger::from_str("1+i").unwrap();
    /// x.pow_assign(10);
    /// assert_eq!(x.to_string(), "32i");
    /// ```
    #[inline]
    fn pow_assign(&mut self, exp: u64) {
        *self = pow_val(take(self), exp);
    }
}
