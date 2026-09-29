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
use crate::integer_polynomial::arithmetic::vec::small_value;
use crate::natural::Natural;
use crate::natural::arithmetic::add::limbs_slice_add_limb_in_place;
use crate::natural::arithmetic::shl::{limbs_shl_to_out, limbs_slice_shl_in_place};
use crate::natural::arithmetic::sub::limbs_sub_limb_in_place;
use crate::natural::logic::not::limbs_not_to_out;
use crate::natural_polynomial::arithmetic::bit_pack::limbs_bit_pack;
use crate::platform::Limb;
use malachite_base::num::arithmetic::traits::{ModPowerOf2Assign, PowerOf2, WrappingAddAssign};
use malachite_base::num::basic::integers::PrimitiveInt;
use malachite_base::num::conversion::traits::{ExactFrom, WrappingFrom};
use malachite_base::num::logic::traits::LowMask;
use malachite_base::polynomial::BitPack;

// The limb in which field `i` of consecutive `bits`-bit fields starts, and the bit within that
// limb.
pub(crate) fn field_start(i: usize, bits: u64) -> (usize, u64) {
    let start = u64::exact_from(i) * bits;
    (
        usize::exact_from(start >> Limb::LOG_WIDTH),
        start & Limb::WIDTH_MASK,
    )
}

// Packs `x` into the `bits`-bit field of `arr` that starts at bit `shift` of `arr[0]`, as a two's
// complement number from which `borrow` is subtracted, and returns whether the field is negative,
// that is, whether the field above must borrow from its own. If `negate`, `-x` is packed instead.
// The low `shift` bits of `arr[0]` hold the field below and are preserved; the bits above them are
// assumed to be zero. `arr` must extend to the limb holding the field's last bit, and for a
// negative `x` sometimes one limb further.
//
// # Worst-case complexity
// $T(n) = O(n)$
//
// $M(n) = O(1)$
//
// where $T$ is time, $M$ is additional memory, and $n$ is `bits`.
//
// This is equivalent to `fmpz_bit_pack` from `fmpz/bit_pack.c`, FLINT 3.6.0.
crate_test_fn! {limbs_pack_field(
    arr: &mut [Limb],
    shift: u64,
    bits: u64,
    x: &Integer,
    negate: bool,
    borrow: bool,
) -> bool {
    let save = arr[0];
    let limbs = usize::exact_from((shift + bits) >> Limb::LOG_WIDTH);
    let rem_bits = (shift + bits) & Limb::WIDTH_MASK;
    if *x == 0u32 {
        // Special case: store -borrow.
        if borrow {
            // Store -1 shifted, and add save back in.
            arr[0] = (Limb::MAX << shift).wrapping_add(save);
            // Complement the remaining limbs.
            if limbs > 1 {
                arr[1..limbs].fill(Limb::MAX);
            }
            if limbs != 0 {
                // Complement the remaining bits.
                if rem_bits != 0 {
                    arr[limbs] = Limb::low_mask(rem_bits);
                }
            } else {
                // Mask off the final limb.
                arr[limbs].mod_power_of_2_assign(rem_bits);
            }
        }
        return borrow;
    }
    // Let |x| = b. If x is negative and negate is false, or x is positive and negate is true, we
    // want -b - borrow; otherwise, we want b - borrow.
    if !x.sign ^ negate {
        // -b - borrow = !b + 1 - borrow
        let size = if let Some(c) = small_value(x) {
            let c_bits = Limb::wrapping_from(c);
            // d = -b - borrow
            let d = if c < 0 {
                c_bits.wrapping_sub(Limb::from(borrow))
            } else {
                c_bits.wrapping_neg().wrapping_sub(Limb::from(borrow))
            };
            // Store d << shift, and add save back into place.
            arr[0] = (d << shift).wrapping_add(save);
            // Store the carry from d << shift, and complement the remaining bits of the second
            // limb.
            if limbs != 0 {
                arr[1] = if shift != 0 {
                    (d >> (Limb::WIDTH - shift)).wrapping_add(Limb::MAX << shift)
                } else {
                    Limb::MAX
                };
            }
            2
        } else {
            let xs = x.abs.as_limbs_asc();
            let mut s = xs.len();
            // Complement the coefficient into arr.
            limbs_not_to_out(&mut arr[..s], xs);
            // Deal with + 1 - borrow; there cannot be a carry, or else we complemented 0.
            if !borrow {
                limbs_slice_add_limb_in_place(&mut arr[..s], 1);
            }
            // Shift into place.
            if shift != 0 {
                let carry = limbs_slice_shl_in_place(&mut arr[..s], shift);
                if limbs + usize::from(rem_bits != 0) > s {
                    arr[s] = (Limb::MAX << shift).wrapping_add(carry);
                    s += 1;
                }
            }
            // Add back in the saved bits from the start of the field.
            arr[0].wrapping_add_assign(save);
            s
        };
        if limbs >= size {
            // Complement any additional limbs.
            if limbs > size {
                arr[size..limbs].fill(Limb::MAX);
            }
            // Complement the remaining bits.
            if rem_bits != 0 {
                arr[limbs] = Limb::low_mask(rem_bits);
            }
        } else {
            // Mask off the final limb.
            arr[limbs].mod_power_of_2_assign(rem_bits);
        }
        true
    } else {
        // b - borrow
        if let Some(c) = small_value(x) {
            // d = b - borrow
            let d = Limb::wrapping_from(c.unsigned_abs()) - Limb::from(borrow);
            // Store d << shift, and add save back into place.
            arr[0] = (d << shift).wrapping_add(save);
            // Store the carry from d << shift.
            if limbs + usize::from(rem_bits != 0) > 1 && shift != 0 {
                arr[1] = d >> (Limb::WIDTH - shift);
            }
        } else {
            let xs = x.abs.as_limbs_asc();
            let mut s = xs.len();
            // Shift into place.
            if shift != 0 {
                let carry = limbs_shl_to_out(&mut arr[..s], xs, shift);
                if carry != 0 {
                    arr[s] = carry;
                    s += 1;
                }
            } else {
                arr[..s].copy_from_slice(xs);
            }
            // Deal with - borrow.
            if borrow {
                limbs_sub_limb_in_place(&mut arr[..s], Limb::power_of_2(shift));
            }
            // Add back in the saved bits from the start of the field.
            arr[0].wrapping_add_assign(save);
        }
        false
    }
}}

// Packs `xs` into consecutive `bits`-bit fields of `out`, starting at bit 0, as two's complement
// numbers, each negative one borrowing from the field above; if `negate`, the negations of `xs` are
// packed instead. `out` must be zeroed, and long enough for every field; if the last element packs
// as a negative number, and its field ends at a limb boundary, one limb more. (FLINT's callers
// choose `negate` so that the leading coefficient packs as nonnegative, which avoids the extra
// limb.)
//
// # Worst-case complexity
// $T(n) = O(n)$
//
// $M(n) = O(1)$
//
// where $T$ is time, $M$ is additional memory, and $n$ is `xs.len() * bits`.
//
// This is equivalent to `_fmpz_poly_bit_pack` from `fmpz_poly/bit_pack.c`, FLINT 3.6.0, where
// `negate` being true corresponds to a `negate` of -1.
crate_test_fn! {limbs_pack_coefficients(out: &mut [Limb], xs: &[Integer], bits: u64, negate: bool) {
    let mut borrow = false;
    for (i, x) in xs.iter().enumerate() {
        let (limbs, shift) = field_start(i, bits);
        borrow = limbs_pack_field(&mut out[limbs..], shift, bits, x, negate, borrow);
    }
}}

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
