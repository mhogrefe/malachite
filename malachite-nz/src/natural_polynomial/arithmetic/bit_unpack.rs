// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::natural::Natural;
use crate::natural_polynomial::NaturalPolynomial;
use malachite_base::num::conversion::traits::PowerOf2Digits;
use malachite_base::polynomial::BitUnpack;

impl BitUnpack<Natural> for NaturalPolynomial {
    /// Unpacks a [`NaturalPolynomial`] from the `bits`-bit fields of a [`Natural`], taking it by
    /// value. The coefficients are the [`Natural`]'s digits in base $2^b$, so the result $p$
    /// satisfies $p(2^b) = n$.
    ///
    /// $$
    /// f(n, b) = \sum_i \left ( \left \lfloor \frac{n}{2^{ib}} \right \rfloor \bmod 2^b \right )
    /// x^i.
    /// $$
    ///
    /// This inverts [`bit_pack`](malachite_base::polynomial::BitPack::bit_pack) on polynomials
    /// whose coefficients are less than $2^b$.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `n.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `bits` is 0.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::basic::traits::Zero;
    /// use malachite_base::polynomial::BitUnpack;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_polynomial::NaturalPolynomial;
    ///
    /// assert_eq!(
    ///     NaturalPolynomial::bit_unpack(Natural::from(197121u32), 8).to_string(),
    ///     "3*x^2+2*x+1"
    /// );
    /// assert_eq!(
    ///     NaturalPolynomial::bit_unpack(Natural::ZERO, 8),
    ///     NaturalPolynomial::ZERO
    /// );
    /// // 257000 = 1000 * 2^8 + 1000 = 3 * 2^16 + 235 * 2^8 + 232
    /// assert_eq!(
    ///     NaturalPolynomial::bit_unpack(Natural::from(257000u32), 8).to_string(),
    ///     "3*x^2+235*x+232"
    /// );
    /// ```
    ///
    /// This is equivalent to `fmpz_poly_bit_unpack_unsigned` from `fmpz_poly/bit_unpack.c`, FLINT
    /// 3.6.0, except that it panics when `bits` is 0, where FLINT gives the zero polynomial.
    #[inline]
    fn bit_unpack(n: Natural, bits: u64) -> Self {
        Self::bit_unpack(&n, bits)
    }
}

impl BitUnpack<&Natural> for NaturalPolynomial {
    /// Unpacks a [`NaturalPolynomial`] from the `bits`-bit fields of a [`Natural`], taking it by
    /// reference. The coefficients are the [`Natural`]'s digits in base $2^b$, so the result $p$
    /// satisfies $p(2^b) = n$.
    ///
    /// $$
    /// f(n, b) = \sum_i \left ( \left \lfloor \frac{n}{2^{ib}} \right \rfloor \bmod 2^b \right )
    /// x^i.
    /// $$
    ///
    /// This inverts [`bit_pack`](malachite_base::polynomial::BitPack::bit_pack) on polynomials
    /// whose coefficients are less than $2^b$.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `n.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `bits` is 0.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::basic::traits::Zero;
    /// use malachite_base::polynomial::BitUnpack;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_polynomial::NaturalPolynomial;
    ///
    /// assert_eq!(
    ///     NaturalPolynomial::bit_unpack(&Natural::from(197121u32), 8).to_string(),
    ///     "3*x^2+2*x+1"
    /// );
    /// assert_eq!(
    ///     NaturalPolynomial::bit_unpack(&Natural::ZERO, 8),
    ///     NaturalPolynomial::ZERO
    /// );
    /// // 257000 = 1000 * 2^8 + 1000 = 3 * 2^16 + 235 * 2^8 + 232
    /// assert_eq!(
    ///     NaturalPolynomial::bit_unpack(&Natural::from(257000u32), 8).to_string(),
    ///     "3*x^2+235*x+232"
    /// );
    /// ```
    ///
    /// This is equivalent to `fmpz_poly_bit_unpack_unsigned` from `fmpz_poly/bit_unpack.c`, FLINT
    /// 3.6.0, except that it panics when `bits` is 0, where FLINT gives the zero polynomial.
    fn bit_unpack(n: &Natural, bits: u64) -> Self {
        assert_ne!(bits, 0, "Cannot unpack a polynomial from fields of 0 bits");
        // The digits of a nonzero number have a nonzero last digit, so this is normalized.
        Self {
            coefficients: PowerOf2Digits::<Natural>::to_power_of_2_digits_asc(n, bits),
        }
    }
}
