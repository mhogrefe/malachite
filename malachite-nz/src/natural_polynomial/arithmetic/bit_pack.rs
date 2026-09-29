// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::natural::Natural;
use crate::natural::arithmetic::add::limbs_slice_add_greater_in_place_left;
use crate::natural::arithmetic::shl::limbs_shl;
use crate::natural_polynomial::NaturalPolynomial;
use crate::platform::Limb;
use alloc::vec;
use alloc::vec::Vec;
use malachite_base::num::basic::integers::PrimitiveInt;
use malachite_base::num::conversion::traits::ExactFrom;
use malachite_base::num::logic::traits::SignificantBits;
use malachite_base::polynomial::BitPack;

// Adds each `x`, for `(i, x)` in `xs`, shifted left by `i * bits`, into a limb buffer, and returns
// the buffer, or an empty one if `xs` is empty. The buffer is sized for the widest shifted `x` plus
// a limb for carries, which are only produced when fields overlap; when every `x` is less than
// 2^bits, the additions touch disjoint limbs apart from the one each field shares with its
// neighbor, and this is packing.
pub(crate) fn limbs_bit_pack<'a>(
    xs: impl Iterator<Item = (usize, &'a Natural)> + Clone,
    bits: u64,
) -> Vec<Limb> {
    let Some(top_bits) = xs
        .clone()
        .map(|(i, x)| u64::exact_from(i).checked_mul(bits).unwrap() + x.significant_bits())
        .max()
    else {
        return Vec::new();
    };
    let mut out = vec![0; usize::exact_from(top_bits >> Limb::LOG_WIDTH) + 2];
    for (i, x) in xs {
        let offset = u64::exact_from(i) * bits;
        let shifted = limbs_shl(x.as_limbs_asc(), offset & Limb::WIDTH_MASK);
        assert!(!limbs_slice_add_greater_in_place_left(
            &mut out[usize::exact_from(offset >> Limb::LOG_WIDTH)..],
            &shifted
        ));
    }
    out
}

impl BitPack for NaturalPolynomial {
    type Output = Natural;

    /// Packs the coefficients of a [`NaturalPolynomial`] into a [`Natural`], placing the
    /// coefficient of $x^i$ at bit $ib$, taking it by value. The result is the value of the
    /// polynomial at $2^b$.
    ///
    /// $$
    /// f(p, b) = p(2^b) = \sum_i a_i2^{ib}.
    /// $$
    ///
    /// When every coefficient is less than $2^b$, each occupies its own $b$-bit field, and the
    /// coefficients are the result's digits in base $2^b$. Wider coefficients overlap the fields
    /// above them, and the result is still $p(2^b)$.
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
    /// use malachite_nz::natural_polynomial::NaturalPolynomial;
    ///
    /// // 3 * 2^16 + 2 * 2^8 + 1
    /// let p = NaturalPolynomial::from_str("3*x^2+2*x+1").unwrap();
    /// assert_eq!(p.clone().bit_pack(8).to_string(), "197121");
    /// // With 0 bits, this is p(1).
    /// assert_eq!(p.clone().bit_pack(0).to_string(), "6");
    /// // Coefficients wider than the fields overlap, and the result is still p(2^b).
    /// let p = NaturalPolynomial::from_str("1000*x+1000").unwrap();
    /// assert_eq!(p.bit_pack(8).to_string(), "257000");
    /// ```
    ///
    /// This is equivalent to `fmpz_poly_bit_pack` from `fmpz_poly/bit_pack.c`, FLINT 3.6.0, when
    /// `bits` is positive and every coefficient is less than $2^b$; FLINT gives 0 when `bits` is 0
    /// rather than $p(1)$, and truncates wider coefficients.
    #[inline]
    fn bit_pack(self, bits: u64) -> Natural {
        (&self).bit_pack(bits)
    }
}

impl BitPack for &NaturalPolynomial {
    type Output = Natural;

    /// Packs the coefficients of a [`NaturalPolynomial`] into a [`Natural`], placing the
    /// coefficient of $x^i$ at bit $ib$, taking it by reference. The result is the value of the
    /// polynomial at $2^b$.
    ///
    /// $$
    /// f(p, b) = p(2^b) = \sum_i a_i2^{ib}.
    /// $$
    ///
    /// When every coefficient is less than $2^b$, each occupies its own $b$-bit field, and the
    /// coefficients are the result's digits in base $2^b$. Wider coefficients overlap the fields
    /// above them, and the result is still $p(2^b)$.
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
    /// use malachite_nz::natural_polynomial::NaturalPolynomial;
    ///
    /// // 3 * 2^16 + 2 * 2^8 + 1
    /// let p = NaturalPolynomial::from_str("3*x^2+2*x+1").unwrap();
    /// assert_eq!((&p).bit_pack(8).to_string(), "197121");
    /// // With 0 bits, this is p(1).
    /// assert_eq!((&p).bit_pack(0).to_string(), "6");
    /// // Coefficients wider than the fields overlap, and the result is still p(2^b).
    /// let p = NaturalPolynomial::from_str("1000*x+1000").unwrap();
    /// assert_eq!((&p).bit_pack(8).to_string(), "257000");
    /// ```
    ///
    /// This is equivalent to `fmpz_poly_bit_pack` from `fmpz_poly/bit_pack.c`, FLINT 3.6.0, when
    /// `bits` is positive and every coefficient is less than $2^b$; FLINT gives 0 when `bits` is 0
    /// rather than $p(1)$, and truncates wider coefficients.
    #[inline]
    fn bit_pack(self, bits: u64) -> Natural {
        Natural::from_owned_limbs_asc(limbs_bit_pack(
            self.coefficients
                .iter()
                .enumerate()
                .filter(|(_, c)| **c != 0u32),
            bits,
        ))
    }
}
