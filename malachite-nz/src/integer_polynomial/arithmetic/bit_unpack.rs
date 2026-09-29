// Copyright © 2026 Mikhail Hogrefe
//
// Uses code adopted from the FLINT Library.
//
//      Copyright © 2010 William Hart
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::integer::Integer;
use crate::integer_polynomial::IntegerPolynomial;
use crate::integer_polynomial::arithmetic::bit_pack::field_start;
use crate::integer_polynomial::arithmetic::vec::SMALL_FMPZ_BITCOUNT_MAX;
use crate::natural::Natural;
use crate::natural::arithmetic::add::limbs_slice_add_limb_in_place;
use crate::natural::arithmetic::shr::limbs_shr_to_out;
use crate::natural::logic::not::limbs_not_in_place;
use crate::platform::{Limb, SignedLimb};
use alloc::vec;
use alloc::vec::Vec;
use malachite_base::num::arithmetic::traits::{
    ModPowerOf2Assign, NegAssign, PowerOf2, WrappingAddAssign,
};
use malachite_base::num::basic::integers::PrimitiveInt;
use malachite_base::num::basic::traits::One;
use malachite_base::num::conversion::traits::{ExactFrom, PowerOf2Digits, WrappingFrom};
use malachite_base::num::logic::traits::{BitAccess, LowMask};
use malachite_base::polynomial::{BitUnpack, Polynomial};

// The limbs of the `bits`-bit field of `arr` that starts at bit `shift` of `arr[0]`, as an unsigned
// number with `ceil(bits / Limb::WIDTH)` limbs.
fn limbs_extract_field(arr: &[Limb], shift: u64, bits: u64) -> Vec<Limb> {
    let limbs = usize::exact_from((shift + bits) >> Limb::LOG_WIDTH);
    let rem_bits = (shift + bits) & Limb::WIDTH_MASK;
    // The number of limbs that hold the field, including b extra bits:
    let l = usize::exact_from(bits.div_ceil(Limb::WIDTH));
    let b = bits & Limb::WIDTH_MASK;
    let mut p = vec![0; l];
    // Shift in l limbs.
    if shift != 0 {
        limbs_shr_to_out(&mut p, &arr[..l], shift);
    } else {
        p.copy_from_slice(&arr[..l]);
    }
    // Shift in any remaining bits that weren't already shifted.
    if limbs + usize::from(rem_bits != 0) > l {
        p[l - 1].wrapping_add_assign(arr[limbs] << (Limb::WIDTH - shift));
    }
    // Mask off the last limb, if it isn't full.
    if b != 0 {
        p[l - 1].mod_power_of_2_assign(b);
    }
    p
}

// Unpacks the `bits`-bit field of `arr` that starts at bit `shift` of `arr[0]`, as a two's
// complement number to which `borrow` is added, and returns it, negated if `negate`, together with
// whether the field was negative, that is, whether the field above must borrow from its own.
//
// # Worst-case complexity
// $T(n) = O(n)$
//
// $M(n) = O(n)$
//
// where $T$ is time, $M$ is additional memory, and $n$ is `bits`.
//
// This is equivalent to `fmpz_bit_unpack` from `fmpz/bit_unpack.c`, FLINT 3.6.0.
crate_test_fn! {limbs_unpack_field(
    arr: &[Limb],
    shift: u64,
    bits: u64,
    negate: bool,
    borrow: bool,
) -> (Integer, bool) {
    let limbs = usize::exact_from((shift + bits) >> Limb::LOG_WIDTH);
    let rem_bits = (shift + bits) & Limb::WIDTH_MASK;
    // Determine whether the field is positive or negative.
    let sign = if rem_bits != 0 {
        arr[limbs].get_bit(rem_bits - 1)
    } else {
        arr[limbs - 1].get_highest_bit()
    };
    let (mut value, negative) = if bits <= SMALL_FMPZ_BITCOUNT_MAX {
        // The field fits in a small coefficient.
        let mask = Limb::low_mask(bits);
        let mut c = if limbs + usize::from(rem_bits != 0) > 1 {
            // The field crosses a limb boundary.
            ((arr[0] >> shift).wrapping_add(arr[1] << (Limb::WIDTH - shift))) & mask
        } else {
            // The field is in the first limb only; mask it.
            (arr[0] >> shift) & mask
        };
        // Sign-extend.
        if sign {
            c.wrapping_add_assign(Limb::MAX << bits);
        }
        let c = SignedLimb::wrapping_from(c);
        // Determine whether we need to return a borrow, and deal with the borrow; since the field
        // has at most `SMALL_FMPZ_BITCOUNT_MAX` bits, adding it cannot overflow.
        (Integer::from(c + SignedLimb::from(borrow)), c < 0)
    } else {
        // A large coefficient.
        let mut p = limbs_extract_field(arr, shift, bits);
        let l = p.len();
        let b = bits & Limb::WIDTH_MASK;
        if sign {
            // Sign-extend.
            if b != 0 {
                p[l - 1].wrapping_add_assign(Limb::MAX << b);
            }
            // Negate.
            limbs_not_in_place(&mut p);
            if !borrow {
                limbs_slice_add_limb_in_place(&mut p, 1);
            }
            (-Integer::from(Natural::from_owned_limbs_asc(p)), true)
        } else {
            // Deal with the borrow.
            if borrow {
                limbs_slice_add_limb_in_place(&mut p, 1);
            }
            (Integer::from(Natural::from_owned_limbs_asc(p)), false)
        }
    };
    // Negate if required.
    if negate {
        value.neg_assign();
    }
    (value, negative)
}}

// Unpacks the `bits`-bit field of `arr` that starts at bit `shift` of `arr[0]`, as an unsigned
// number.
//
// # Worst-case complexity
// $T(n) = O(n)$
//
// $M(n) = O(n)$
//
// where $T$ is time, $M$ is additional memory, and $n$ is `bits`.
//
// This is equivalent to `fmpz_bit_unpack_unsigned` from `fmpz/bit_unpack.c`, FLINT 3.6.0.
crate_test_fn! {limbs_unpack_field_unsigned(arr: &[Limb], shift: u64, bits: u64) -> Integer {
    let limbs = usize::exact_from((shift + bits) >> Limb::LOG_WIDTH);
    let rem_bits = (shift + bits) & Limb::WIDTH_MASK;
    if bits <= SMALL_FMPZ_BITCOUNT_MAX {
        // The field fits in a small coefficient.
        let mask = Limb::low_mask(bits);
        Integer::from(if limbs + usize::from(rem_bits != 0) > 1 {
            // The field crosses a limb boundary.
            ((arr[0] >> shift).wrapping_add(arr[1] << (Limb::WIDTH - shift))) & mask
        } else {
            // The field is in the first limb only; mask it.
            (arr[0] >> shift) & mask
        })
    } else {
        // A large coefficient.
        Integer::from(Natural::from_owned_limbs_asc(limbs_extract_field(arr, shift, bits)))
    }
}}

// Unpacks the fields `nlo..nhi` of consecutive `bits`-bit fields of `xs`, as two's complement
// numbers, each negative one borrowing from the field above, into `out`; if `negate`, their
// negations are unpacked instead. Returns whether the last field unpacked was negative, in which
// case a coefficient of 1, or -1 if `negate`, belongs at index `nhi`. If `nlo` is not zero, the
// borrow into the first field unpacked is read from the top bit of the field below it.
//
// # Worst-case complexity
// $T(n) = O(n)$
//
// $M(n) = O(n)$
//
// where $T$ is time, $M$ is additional memory, and $n$ is `(nhi - nlo) * bits`.
//
// This is equivalent to `_fmpz_poly_bit_unpack` from `fmpz_poly/bit_unpack.c`, FLINT 3.6.0, where
// `negate` being true corresponds to a `negate` of -1.
crate_test_fn! {limbs_unpack_coefficients(
    out: &mut [Integer],
    nlo: usize,
    nhi: usize,
    xs: &[Limb],
    bits: u64,
    negate: bool,
) -> bool {
    // The borrow into field nlo is the sign bit of the field below it.
    let mut borrow = if nlo == 0 {
        false
    } else {
        let top = u64::exact_from(nlo) * bits - 1;
        xs[usize::exact_from(top >> Limb::LOG_WIDTH)].get_bit(top & Limb::WIDTH_MASK)
    };
    for (c, i) in out.iter_mut().zip(nlo..nhi) {
        let (limbs, shift) = field_start(i, bits);
        let next_borrow;
        (*c, next_borrow) = limbs_unpack_field(&xs[limbs..], shift, bits, negate, borrow);
        borrow = next_borrow;
    }
    borrow
}}

// Unpacks the fields `nlo..nhi` of consecutive `bits`-bit fields of `xs`, as unsigned numbers, into
// `out`.
//
// # Worst-case complexity
// $T(n) = O(n)$
//
// $M(n) = O(n)$
//
// where $T$ is time, $M$ is additional memory, and $n$ is `(nhi - nlo) * bits`.
//
// This is equivalent to `_fmpz_poly_bit_unpack_unsigned` from `fmpz_poly/bit_unpack.c`, FLINT
// 3.6.0.
crate_test_fn! {limbs_unpack_coefficients_unsigned(
    out: &mut [Integer],
    nlo: usize,
    nhi: usize,
    xs: &[Limb],
    bits: u64,
) {
    for (c, i) in out.iter_mut().zip(nlo..nhi) {
        let (limbs, shift) = field_start(i, bits);
        *c = limbs_unpack_field_unsigned(&xs[limbs..], shift, bits);
    }
}}

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
