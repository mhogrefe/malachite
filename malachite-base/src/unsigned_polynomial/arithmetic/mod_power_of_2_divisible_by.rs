// Copyright © 2026 Mikhail Hogrefe
//
// Uses code adopted from the FLINT Library.
//
//      Copyright © 2010, 2011 Sebastian Pancratz
//
//      Copyright © 2011 William Hart
//
//      Copyright © 2023 Fredrik Johansson
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::num::basic::traits::Zero;
use crate::num::basic::unsigneds::PrimitiveUnsigned;
use crate::polynomial::ModPowerOf2DivisibleBy;
use crate::unsigned_polynomial::UnsignedPolynomial;
use crate::unsigned_polynomial::arithmetic::mod_power_of_2_div_exact::assert_invertible;
use crate::unsigned_polynomial::arithmetic::mod_power_of_2_div_rem::{
    assert_reduced, mod_power_of_2_div_rem_with,
};
use crate::unsigned_polynomial::arithmetic::mod_power_of_2_div_truncated::*;

// Whether `q` divides `p` modulo $2^k$, where $k$ is `pow`.
//
// This is equivalent to the result of `nmod_poly_divides` from `nmod_poly/divides.c`, FLINT 3.6.0,
// with the modulus $2^k$, except that Malachite panics where FLINT assumes the leading coefficient
// invertible.
fn helper<T: PrimitiveUnsigned>(
    p: &UnsignedPolynomial<T>,
    q: &UnsignedPolynomial<T>,
    pow: u64,
) -> bool {
    assert_reduced(p, q, pow);
    if pow == 0 || *p == UnsignedPolynomial::ZERO {
        return true;
    }
    if *q == UnsignedPolynomial::ZERO {
        return false;
    }
    assert_invertible(q, pow);
    mod_power_of_2_div_rem_with(
        &p.coefficients,
        &q.coefficients,
        pow,
        mod_power_of_2_div_truncated_dispatch,
    )
    .unwrap()
    .1 == UnsignedPolynomial::ZERO
}

impl<T: PrimitiveUnsigned> ModPowerOf2DivisibleBy<Self> for UnsignedPolynomial<T> {
    /// Determines whether an [`UnsignedPolynomial`] is divisible by another modulo $2^k$, taking
    /// the first by value and the second by value. The coefficients of both must already be reduced
    /// modulo $2^k$, and the leading coefficient of the second, unless it is 0, must be odd.
    ///
    /// $$
    /// f(a, b, k) = (\exists q : a \equiv bq \pmod{2^k}).
    /// $$
    ///
    /// 0 is divisible by every polynomial, and only 0 is divisible by 0. When $k$ is 0, every
    /// polynomial is divisible by every other.
    ///
    /// The remainder is found as by [`ModPowerOf2Rem`](crate::polynomial::ModPowerOf2Rem), and
    /// compared with 0.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `self.len()`.
    ///
    /// # Panics
    /// Panics if `pow` is greater than `T::WIDTH`, or if any coefficient of `self` or `other` is
    /// greater than or equal to $2^k$, or if `self` and `other` are nonzero and the leading
    /// coefficient of `other` is not odd.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::ModPowerOf2DivisibleBy;
    /// use malachite_base::unsigned_polynomial::UnsignedPolynomial;
    ///
    /// let p = UnsignedPolynomial::<u8>::from_str("x^4+4").unwrap();
    /// let q = UnsignedPolynomial::<u8>::from_str("5*x^2+2").unwrap();
    /// assert!(p.mod_power_of_2_divisible_by(q, 3));
    ///
    /// let p = UnsignedPolynomial::<u8>::from_str("x^4+7*x+5").unwrap();
    /// let q = UnsignedPolynomial::<u8>::from_str("5*x^2+3").unwrap();
    /// assert!(!p.mod_power_of_2_divisible_by(q, 3));
    /// ```
    ///
    /// This is equivalent to the result of `nmod_poly_divides` from `nmod_poly/divides.c`, FLINT
    /// 3.6.0, with the modulus $2^k$, except that Malachite panics where FLINT aborts or assumes
    /// the leading coefficient invertible.
    #[inline]
    fn mod_power_of_2_divisible_by(self, other: Self, pow: u64) -> bool {
        helper(&self, &other, pow)
    }
}

impl<T: PrimitiveUnsigned> ModPowerOf2DivisibleBy<&Self> for UnsignedPolynomial<T> {
    /// Determines whether an [`UnsignedPolynomial`] is divisible by another modulo $2^k$, taking
    /// the first by value and the second by reference. The coefficients of both must already be
    /// reduced modulo $2^k$, and the leading coefficient of the second, unless it is 0, must be
    /// odd.
    ///
    /// $$
    /// f(a, b, k) = (\exists q : a \equiv bq \pmod{2^k}).
    /// $$
    ///
    /// 0 is divisible by every polynomial, and only 0 is divisible by 0. When $k$ is 0, every
    /// polynomial is divisible by every other.
    ///
    /// The remainder is found as by [`ModPowerOf2Rem`](crate::polynomial::ModPowerOf2Rem), and
    /// compared with 0.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `self.len()`.
    ///
    /// # Panics
    /// Panics if `pow` is greater than `T::WIDTH`, or if any coefficient of `self` or `other` is
    /// greater than or equal to $2^k$, or if `self` and `other` are nonzero and the leading
    /// coefficient of `other` is not odd.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::ModPowerOf2DivisibleBy;
    /// use malachite_base::unsigned_polynomial::UnsignedPolynomial;
    ///
    /// let p = UnsignedPolynomial::<u8>::from_str("x^4+4").unwrap();
    /// let q = UnsignedPolynomial::<u8>::from_str("5*x^2+2").unwrap();
    /// assert!(p.mod_power_of_2_divisible_by(&q, 3));
    ///
    /// let p = UnsignedPolynomial::<u8>::from_str("x^4+7*x+5").unwrap();
    /// let q = UnsignedPolynomial::<u8>::from_str("5*x^2+3").unwrap();
    /// assert!(!p.mod_power_of_2_divisible_by(&q, 3));
    /// ```
    ///
    /// This is equivalent to the result of `nmod_poly_divides` from `nmod_poly/divides.c`, FLINT
    /// 3.6.0, with the modulus $2^k$, except that Malachite panics where FLINT aborts or assumes
    /// the leading coefficient invertible.
    #[inline]
    fn mod_power_of_2_divisible_by(self, other: &Self, pow: u64) -> bool {
        helper(&self, other, pow)
    }
}

impl<T: PrimitiveUnsigned> ModPowerOf2DivisibleBy<UnsignedPolynomial<T>>
    for &UnsignedPolynomial<T>
{
    /// Determines whether an [`UnsignedPolynomial`] is divisible by another modulo $2^k$, taking
    /// the first by reference and the second by value. The coefficients of both must already be
    /// reduced modulo $2^k$, and the leading coefficient of the second, unless it is 0, must be
    /// odd.
    ///
    /// $$
    /// f(a, b, k) = (\exists q : a \equiv bq \pmod{2^k}).
    /// $$
    ///
    /// 0 is divisible by every polynomial, and only 0 is divisible by 0. When $k$ is 0, every
    /// polynomial is divisible by every other.
    ///
    /// The remainder is found as by [`ModPowerOf2Rem`](crate::polynomial::ModPowerOf2Rem), and
    /// compared with 0.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `self.len()`.
    ///
    /// # Panics
    /// Panics if `pow` is greater than `T::WIDTH`, or if any coefficient of `self` or `other` is
    /// greater than or equal to $2^k$, or if `self` and `other` are nonzero and the leading
    /// coefficient of `other` is not odd.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::ModPowerOf2DivisibleBy;
    /// use malachite_base::unsigned_polynomial::UnsignedPolynomial;
    ///
    /// let p = UnsignedPolynomial::<u8>::from_str("x^4+4").unwrap();
    /// let q = UnsignedPolynomial::<u8>::from_str("5*x^2+2").unwrap();
    /// assert!((&p).mod_power_of_2_divisible_by(q, 3));
    ///
    /// let p = UnsignedPolynomial::<u8>::from_str("x^4+7*x+5").unwrap();
    /// let q = UnsignedPolynomial::<u8>::from_str("5*x^2+3").unwrap();
    /// assert!(!(&p).mod_power_of_2_divisible_by(q, 3));
    /// ```
    ///
    /// This is equivalent to the result of `nmod_poly_divides` from `nmod_poly/divides.c`, FLINT
    /// 3.6.0, with the modulus $2^k$, except that Malachite panics where FLINT aborts or assumes
    /// the leading coefficient invertible.
    #[inline]
    fn mod_power_of_2_divisible_by(self, other: UnsignedPolynomial<T>, pow: u64) -> bool {
        helper(self, &other, pow)
    }
}

impl<T: PrimitiveUnsigned> ModPowerOf2DivisibleBy<&UnsignedPolynomial<T>>
    for &UnsignedPolynomial<T>
{
    /// Determines whether an [`UnsignedPolynomial`] is divisible by another modulo $2^k$, taking
    /// the first by reference and the second by reference. The coefficients of both must already be
    /// reduced modulo $2^k$, and the leading coefficient of the second, unless it is 0, must be
    /// odd.
    ///
    /// $$
    /// f(a, b, k) = (\exists q : a \equiv bq \pmod{2^k}).
    /// $$
    ///
    /// 0 is divisible by every polynomial, and only 0 is divisible by 0. When $k$ is 0, every
    /// polynomial is divisible by every other.
    ///
    /// The remainder is found as by [`ModPowerOf2Rem`](crate::polynomial::ModPowerOf2Rem), and
    /// compared with 0.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `self.len()`.
    ///
    /// # Panics
    /// Panics if `pow` is greater than `T::WIDTH`, or if any coefficient of `self` or `other` is
    /// greater than or equal to $2^k$, or if `self` and `other` are nonzero and the leading
    /// coefficient of `other` is not odd.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::ModPowerOf2DivisibleBy;
    /// use malachite_base::unsigned_polynomial::UnsignedPolynomial;
    ///
    /// let p = UnsignedPolynomial::<u8>::from_str("x^4+4").unwrap();
    /// let q = UnsignedPolynomial::<u8>::from_str("5*x^2+2").unwrap();
    /// assert!((&p).mod_power_of_2_divisible_by(&q, 3));
    ///
    /// let p = UnsignedPolynomial::<u8>::from_str("x^4+7*x+5").unwrap();
    /// let q = UnsignedPolynomial::<u8>::from_str("5*x^2+3").unwrap();
    /// assert!(!(&p).mod_power_of_2_divisible_by(&q, 3));
    /// ```
    ///
    /// This is equivalent to the result of `nmod_poly_divides` from `nmod_poly/divides.c`, FLINT
    /// 3.6.0, with the modulus $2^k$, except that Malachite panics where FLINT aborts or assumes
    /// the leading coefficient invertible.
    #[inline]
    fn mod_power_of_2_divisible_by(self, other: &UnsignedPolynomial<T>, pow: u64) -> bool {
        helper(self, other, pow)
    }
}
