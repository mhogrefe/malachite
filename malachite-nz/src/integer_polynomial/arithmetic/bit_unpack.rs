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
use alloc::vec::Vec;
use malachite_base::num::arithmetic::traits::{NegAssign, PowerOf2};
use malachite_base::num::basic::traits::One;
use malachite_base::num::conversion::traits::PowerOf2Digits;
use malachite_base::num::logic::traits::BitAccess;
use malachite_base::polynomial::{BitUnpack, Polynomial};

// Reads the base-2^bits digits of |n| as signed numbers, each negative one borrowing 1 from the
// digit above, and negates the result if n is negative.
//
// This is equivalent to `fmpz_poly_bit_unpack` from `fmpz_poly/bit_unpack.c`, FLINT 3.6.0.
fn bit_unpack_ref(n: &Integer, bits: u64) -> IntegerPolynomial {
    assert_ne!(bits, 0, "Cannot unpack a polynomial from fields of 0 bits");
    let digits: Vec<Natural> = n.unsigned_abs_ref().to_power_of_2_digits_asc(bits);
    let field = Integer::power_of_2(bits);
    let mut coefficients = Vec::with_capacity(digits.len() + 1);
    let mut borrow = false;
    for digit in digits {
        let negative = digit.get_bit(bits - 1);
        let mut c = Integer::from(digit);
        if negative {
            c -= &field;
        }
        if borrow {
            c += Integer::ONE;
        }
        coefficients.push(c);
        borrow = negative;
    }
    if borrow {
        coefficients.push(Integer::ONE);
    }
    if *n < 0u32 {
        for c in &mut coefficients {
            c.neg_assign();
        }
    }
    IntegerPolynomial::from_coefficients_asc(coefficients)
}

impl BitUnpack<Integer> for IntegerPolynomial {
    /// Unpacks an [`IntegerPolynomial`] from the `bits`-bit fields of an [`Integer`], taking it by
    /// value. The result $p$ satisfies $p(2^b) = n$.
    ///
    /// $$
    /// f(n, b) = p, \quad \text{where} \quad p(2^b) = n.
    /// $$
    ///
    /// The fields of $|n|$ are read as signed $b$-bit numbers, in two's complement, and each
    /// negative one borrows 1 from the field above, so the coefficients lie in $[-2^{b-1},
    /// 2^{b-1}]$; if $n$ is negative, every coefficient is then negated. This inverts
    /// [`bit_pack`](malachite_base::polynomial::BitPack::bit_pack) on polynomials whose
    /// coefficients' absolute values are less than $2^{b-1}$.
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
    /// use malachite_base::polynomial::BitUnpack;
    /// use malachite_nz::integer::Integer;
    /// use malachite_nz::integer_polynomial::IntegerPolynomial;
    ///
    /// assert_eq!(
    ///     IntegerPolynomial::bit_unpack(Integer::from(197121), 8).to_string(),
    ///     "3*x^2+2*x+1"
    /// );
    /// // A field whose top bit is set is negative, and borrows from the field above.
    /// assert_eq!(
    ///     IntegerPolynomial::bit_unpack(Integer::from(196097), 8).to_string(),
    ///     "3*x^2-2*x+1"
    /// );
    /// assert_eq!(
    ///     IntegerPolynomial::bit_unpack(Integer::from(128), 8).to_string(),
    ///     "x-128"
    /// );
    /// assert_eq!(
    ///     IntegerPolynomial::bit_unpack(Integer::from(-196097), 8).to_string(),
    ///     "-3*x^2+2*x-1"
    /// );
    /// ```
    ///
    /// This is equivalent to `fmpz_poly_bit_unpack` from `fmpz_poly/bit_unpack.c`, FLINT 3.6.0,
    /// except that it panics when `bits` is 0, where FLINT gives the zero polynomial.
    #[inline]
    fn bit_unpack(n: Integer, bits: u64) -> Self {
        bit_unpack_ref(&n, bits)
    }
}

impl BitUnpack<&Integer> for IntegerPolynomial {
    /// Unpacks an [`IntegerPolynomial`] from the `bits`-bit fields of an [`Integer`], taking it by
    /// reference. The result $p$ satisfies $p(2^b) = n$.
    ///
    /// $$
    /// f(n, b) = p, \quad \text{where} \quad p(2^b) = n.
    /// $$
    ///
    /// The fields of $|n|$ are read as signed $b$-bit numbers, in two's complement, and each
    /// negative one borrows 1 from the field above, so the coefficients lie in $[-2^{b-1},
    /// 2^{b-1}]$; if $n$ is negative, every coefficient is then negated. This inverts
    /// [`bit_pack`](malachite_base::polynomial::BitPack::bit_pack) on polynomials whose
    /// coefficients' absolute values are less than $2^{b-1}$.
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
    /// use malachite_base::polynomial::BitUnpack;
    /// use malachite_nz::integer::Integer;
    /// use malachite_nz::integer_polynomial::IntegerPolynomial;
    ///
    /// assert_eq!(
    ///     IntegerPolynomial::bit_unpack(&Integer::from(197121), 8).to_string(),
    ///     "3*x^2+2*x+1"
    /// );
    /// // A field whose top bit is set is negative, and borrows from the field above.
    /// assert_eq!(
    ///     IntegerPolynomial::bit_unpack(&Integer::from(196097), 8).to_string(),
    ///     "3*x^2-2*x+1"
    /// );
    /// assert_eq!(
    ///     IntegerPolynomial::bit_unpack(&Integer::from(128), 8).to_string(),
    ///     "x-128"
    /// );
    /// assert_eq!(
    ///     IntegerPolynomial::bit_unpack(&Integer::from(-196097), 8).to_string(),
    ///     "-3*x^2+2*x-1"
    /// );
    /// ```
    ///
    /// This is equivalent to `fmpz_poly_bit_unpack` from `fmpz_poly/bit_unpack.c`, FLINT 3.6.0,
    /// except that it panics when `bits` is 0, where FLINT gives the zero polynomial.
    #[inline]
    fn bit_unpack(n: &Integer, bits: u64) -> Self {
        bit_unpack_ref(n, bits)
    }
}
