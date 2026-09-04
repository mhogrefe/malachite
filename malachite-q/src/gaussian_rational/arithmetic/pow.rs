// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::Rational;
use crate::gaussian_rational::GaussianRational;
use core::mem::take;
use malachite_base::num::arithmetic::traits::{
    ContentAndPrimitivePart, MulIPow, Pow, PowAssign, Reciprocal, Square,
};
use malachite_base::num::basic::traits::One;
use malachite_nz::integer::Integer;

// A purely real or purely imaginary base is a real power times a unit. Any other base is split into
// content and primitive part, which are raised separately: the content as two `Natural` powers and
// the primitive part as a `GaussianInteger` power, with no rational reductions along the way, and
// then the two are recombined with one reduction per part. Square-and-multiply on
// `GaussianRational`s would instead reduce after every multiplication.

// The base has two nonzero parts and `exp` is at least 3.
fn pow_general(x: &GaussianRational, exp: u64) -> GaussianRational {
    let (content, primitive) = x.content_and_primitive_part();
    let (g, l) = content.into_numerator_and_denominator();
    let g_pow = Integer::from(g.pow(exp));
    let l_pow = Integer::from(l.pow(exp));
    let primitive_pow = primitive.pow(exp);
    let real_numerator = primitive_pow.real * &g_pow;
    GaussianRational {
        real: Rational::from_integers_ref(&real_numerator, &l_pow),
        imaginary: Rational::from_integers(primitive_pow.imaginary * g_pow, l_pow),
    }
}

fn pow_val(x: GaussianRational, exp: u64) -> GaussianRational {
    match exp {
        0 => GaussianRational::ONE,
        1 => x,
        2 => x.square(),
        _ if x.imaginary == 0u32 => GaussianRational::from(x.real.pow(exp)),
        // (bi)^n = b^n i^n
        _ if x.real == 0u32 => GaussianRational::from(x.imaginary.pow(exp)).mul_i_pow(exp),
        _ => pow_general(&x, exp),
    }
}

fn pow_ref(x: &GaussianRational, exp: u64) -> GaussianRational {
    match exp {
        0 => GaussianRational::ONE,
        1 => x.clone(),
        2 => x.square(),
        _ if x.imaginary == 0u32 => GaussianRational::from((&x.real).pow(exp)),
        // (bi)^n = b^n i^n
        _ if x.real == 0u32 => GaussianRational::from((&x.imaginary).pow(exp)).mul_i_pow(exp),
        _ => pow_general(x, exp),
    }
}

impl Pow<u64> for GaussianRational {
    type Output = Self;

    /// Raises a [`GaussianRational`] to a power, taking the [`GaussianRational`] by value.
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
    /// use malachite_q::gaussian_rational::GaussianRational;
    /// use std::str::FromStr;
    ///
    /// assert_eq!(
    ///     GaussianRational::from_str("2+i")
    ///         .unwrap()
    ///         .pow(5u64)
    ///         .to_string(),
    ///     "-38+41i"
    /// );
    /// assert_eq!(
    ///     GaussianRational::from_str("1/2+i/3")
    ///         .unwrap()
    ///         .pow(3u64)
    ///         .to_string(),
    ///     "-1/24+23i/108"
    /// );
    /// ```
    #[inline]
    fn pow(self, exp: u64) -> Self {
        pow_val(self, exp)
    }
}

impl Pow<u64> for &GaussianRational {
    type Output = GaussianRational;

    /// Raises a [`GaussianRational`] to a power, taking the [`GaussianRational`] by reference.
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
    /// use malachite_q::gaussian_rational::GaussianRational;
    /// use std::str::FromStr;
    ///
    /// assert_eq!(
    ///     (&GaussianRational::from_str("2+i").unwrap())
    ///         .pow(5u64)
    ///         .to_string(),
    ///     "-38+41i"
    /// );
    /// assert_eq!(
    ///     (&GaussianRational::from_str("1/2+i/3").unwrap())
    ///         .pow(3u64)
    ///         .to_string(),
    ///     "-1/24+23i/108"
    /// );
    /// ```
    #[inline]
    fn pow(self, exp: u64) -> GaussianRational {
        pow_ref(self, exp)
    }
}

impl PowAssign<u64> for GaussianRational {
    /// Raises a [`GaussianRational`] to a power in place.
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
    /// use malachite_q::gaussian_rational::GaussianRational;
    /// use std::str::FromStr;
    ///
    /// let mut x = GaussianRational::from_str("2+i").unwrap();
    /// x.pow_assign(5u64);
    /// assert_eq!(x.to_string(), "-38+41i");
    ///
    /// let mut x = GaussianRational::from_str("1/2+i/3").unwrap();
    /// x.pow_assign(3u64);
    /// assert_eq!(x.to_string(), "-1/24+23i/108");
    /// ```
    #[inline]
    fn pow_assign(&mut self, exp: u64) {
        *self = pow_val(take(self), exp);
    }
}

impl Pow<i64> for GaussianRational {
    type Output = Self;

    /// Raises a [`GaussianRational`] to a power, taking the [`GaussianRational`] by value. A
    /// negative power is the reciprocal of the corresponding positive one.
    ///
    /// $f(x, n) = x^n$.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(nm \log (nm) \log\log (nm))$
    ///
    /// $M(n, m) = O(nm \log (nm))$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is the maximum number of significant bits
    /// of the real and imaginary parts of `self`, and $m$ is `exp.abs()`.
    ///
    /// # Panics
    /// Panics if `self` is zero and `exp` is negative.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::Pow;
    /// use malachite_q::gaussian_rational::GaussianRational;
    /// use std::str::FromStr;
    ///
    /// assert_eq!(
    ///     GaussianRational::from_str("2+i")
    ///         .unwrap()
    ///         .pow(5i64)
    ///         .to_string(),
    ///     "-38+41i"
    /// );
    /// assert_eq!(
    ///     GaussianRational::from_str("2+i")
    ///         .unwrap()
    ///         .pow(-5i64)
    ///         .to_string(),
    ///     "-38/3125-41i/3125"
    /// );
    /// assert_eq!(
    ///     GaussianRational::from_str("1/2+i/3")
    ///         .unwrap()
    ///         .pow(-3i64)
    ///         .to_string(),
    ///     "-1944/2197-9936i/2197"
    /// );
    /// ```
    #[inline]
    fn pow(self, exp: i64) -> Self {
        let power = pow_val(self, exp.unsigned_abs());
        if exp >= 0 { power } else { power.reciprocal() }
    }
}

impl Pow<i64> for &GaussianRational {
    type Output = GaussianRational;

    /// Raises a [`GaussianRational`] to a power, taking the [`GaussianRational`] by reference. A
    /// negative power is the reciprocal of the corresponding positive one.
    ///
    /// $f(x, n) = x^n$.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(nm \log (nm) \log\log (nm))$
    ///
    /// $M(n, m) = O(nm \log (nm))$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is the maximum number of significant bits
    /// of the real and imaginary parts of `self`, and $m$ is `exp.abs()`.
    ///
    /// # Panics
    /// Panics if `self` is zero and `exp` is negative.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::Pow;
    /// use malachite_q::gaussian_rational::GaussianRational;
    /// use std::str::FromStr;
    ///
    /// assert_eq!(
    ///     (&GaussianRational::from_str("2+i").unwrap())
    ///         .pow(5i64)
    ///         .to_string(),
    ///     "-38+41i"
    /// );
    /// assert_eq!(
    ///     (&GaussianRational::from_str("2+i").unwrap())
    ///         .pow(-5i64)
    ///         .to_string(),
    ///     "-38/3125-41i/3125"
    /// );
    /// assert_eq!(
    ///     (&GaussianRational::from_str("1/2+i/3").unwrap())
    ///         .pow(-3i64)
    ///         .to_string(),
    ///     "-1944/2197-9936i/2197"
    /// );
    /// ```
    #[inline]
    fn pow(self, exp: i64) -> GaussianRational {
        let power = pow_ref(self, exp.unsigned_abs());
        if exp >= 0 { power } else { power.reciprocal() }
    }
}

impl PowAssign<i64> for GaussianRational {
    /// Raises a [`GaussianRational`] to a power in place. A negative power is the reciprocal of the
    /// corresponding positive one.
    ///
    /// $x \gets x^n$.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(nm \log (nm) \log\log (nm))$
    ///
    /// $M(n, m) = O(nm \log (nm))$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is the maximum number of significant bits
    /// of the real and imaginary parts of `self`, and $m$ is `exp.abs()`.
    ///
    /// # Panics
    /// Panics if `self` is zero and `exp` is negative.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::PowAssign;
    /// use malachite_q::gaussian_rational::GaussianRational;
    /// use std::str::FromStr;
    ///
    /// let mut x = GaussianRational::from_str("2+i").unwrap();
    /// x.pow_assign(-5i64);
    /// assert_eq!(x.to_string(), "-38/3125-41i/3125");
    ///
    /// let mut x = GaussianRational::from_str("1/2+i/3").unwrap();
    /// x.pow_assign(-3i64);
    /// assert_eq!(x.to_string(), "-1944/2197-9936i/2197");
    /// ```
    #[inline]
    fn pow_assign(&mut self, exp: i64) {
        *self = take(self).pow(exp);
    }
}
