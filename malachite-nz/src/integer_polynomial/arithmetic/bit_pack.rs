// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::integer::Integer;
use crate::integer_polynomial::IntegerPolynomial;
use crate::natural::Natural;
use crate::natural_polynomial::arithmetic::bit_pack::limbs_bit_pack;
use malachite_base::polynomial::BitPack;

// Packs the magnitudes of the positive and of the negative coefficients separately, each at its own
// index, and subtracts the second total from the first.
fn bit_pack_ref(p: &IntegerPolynomial, bits: u64) -> Integer {
    let positive = limbs_bit_pack(
        p.coefficients
            .iter()
            .enumerate()
            .filter(|(_, c)| **c > 0u32)
            .map(|(i, c)| (i, c.unsigned_abs_ref())),
        bits,
    );
    let negative = limbs_bit_pack(
        p.coefficients
            .iter()
            .enumerate()
            .filter(|(_, c)| **c < 0u32)
            .map(|(i, c)| (i, c.unsigned_abs_ref())),
        bits,
    );
    let positive = Integer::from(Natural::from_owned_limbs_asc(positive));
    if negative.is_empty() {
        positive
    } else {
        positive - Integer::from(Natural::from_owned_limbs_asc(negative))
    }
}

impl BitPack for IntegerPolynomial {
    type Output = Integer;

    /// Packs the coefficients of an [`IntegerPolynomial`] into an [`Integer`], placing the
    /// coefficient of $x^i$ at bit $ib$, taking it by value. The result is the value of the
    /// polynomial at $2^b$.
    ///
    /// $$
    /// f(p, b) = p(2^b) = \sum_i a_i2^{ib}.
    /// $$
    ///
    /// When every coefficient's absolute value is less than $2^b$, each occupies its own $b$-bit
    /// field, and the sign of the result is the sign of the leading coefficient. Wider coefficients
    /// overlap the fields above them, and the result is still $p(2^b)$.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n + m)$
    ///
    /// $M(n, m) = O(n + m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is the total number of bits of the
    /// coefficients, and $m$ is `self.len()` times `bits`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::BitPack;
    /// use malachite_nz::integer_polynomial::IntegerPolynomial;
    ///
    /// // 3 * 2^16 + 2 * 2^8 + 1
    /// let p = IntegerPolynomial::from_str("3*x^2+2*x+1").unwrap();
    /// assert_eq!(p.clone().bit_pack(8).to_string(), "197121");
    /// // A negative coefficient borrows from the field above it: 3 * 2^16 - 2 * 2^8 + 1.
    /// let p = IntegerPolynomial::from_str("3*x^2-2*x+1").unwrap();
    /// assert_eq!(p.clone().bit_pack(8).to_string(), "196097");
    /// // The sign of the result is the sign of the leading coefficient.
    /// let p = IntegerPolynomial::from_str("-x^2+255*x+255").unwrap();
    /// assert_eq!(p.clone().bit_pack(8).to_string(), "-1");
    /// // Coefficients wider than the fields overlap, and the result is still p(2^b).
    /// let p = IntegerPolynomial::from_str("1000*x+1000").unwrap();
    /// assert_eq!(p.bit_pack(8).to_string(), "257000");
    /// ```
    ///
    /// This is equivalent to `fmpz_poly_bit_pack` from `fmpz_poly/bit_pack.c`, FLINT 3.6.0, when
    /// `bits` is positive and every coefficient's absolute value is less than $2^b$; FLINT gives 0
    /// when `bits` is 0 rather than $p(1)$, and truncates wider coefficients.
    #[inline]
    fn bit_pack(self, bits: u64) -> Integer {
        bit_pack_ref(&self, bits)
    }
}

impl BitPack for &IntegerPolynomial {
    type Output = Integer;

    /// Packs the coefficients of an [`IntegerPolynomial`] into an [`Integer`], placing the
    /// coefficient of $x^i$ at bit $ib$, taking it by reference. The result is the value of the
    /// polynomial at $2^b$.
    ///
    /// $$
    /// f(p, b) = p(2^b) = \sum_i a_i2^{ib}.
    /// $$
    ///
    /// When every coefficient's absolute value is less than $2^b$, each occupies its own $b$-bit
    /// field, and the sign of the result is the sign of the leading coefficient. Wider coefficients
    /// overlap the fields above them, and the result is still $p(2^b)$.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n + m)$
    ///
    /// $M(n, m) = O(n + m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is the total number of bits of the
    /// coefficients, and $m$ is `self.len()` times `bits`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::BitPack;
    /// use malachite_nz::integer_polynomial::IntegerPolynomial;
    ///
    /// // 3 * 2^16 + 2 * 2^8 + 1
    /// let p = IntegerPolynomial::from_str("3*x^2+2*x+1").unwrap();
    /// assert_eq!((&p).bit_pack(8).to_string(), "197121");
    /// // A negative coefficient borrows from the field above it: 3 * 2^16 - 2 * 2^8 + 1.
    /// let p = IntegerPolynomial::from_str("3*x^2-2*x+1").unwrap();
    /// assert_eq!((&p).bit_pack(8).to_string(), "196097");
    /// // The sign of the result is the sign of the leading coefficient.
    /// let p = IntegerPolynomial::from_str("-x^2+255*x+255").unwrap();
    /// assert_eq!((&p).bit_pack(8).to_string(), "-1");
    /// // Coefficients wider than the fields overlap, and the result is still p(2^b).
    /// let p = IntegerPolynomial::from_str("1000*x+1000").unwrap();
    /// assert_eq!((&p).bit_pack(8).to_string(), "257000");
    /// ```
    ///
    /// This is equivalent to `fmpz_poly_bit_pack` from `fmpz_poly/bit_pack.c`, FLINT 3.6.0, when
    /// `bits` is positive and every coefficient's absolute value is less than $2^b$; FLINT gives 0
    /// when `bits` is 0 rather than $p(1)$, and truncates wider coefficients.
    #[inline]
    fn bit_pack(self, bits: u64) -> Integer {
        bit_pack_ref(self, bits)
    }
}
