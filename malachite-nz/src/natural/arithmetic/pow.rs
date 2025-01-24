// Copyright © 2025 Mikhail Hogrefe
//
// Uses code adopted from the GNU MP Library.
//
//      Copyright © 1991, 1993-1997, 1999-2016, 2020 Free Software Foundation, Inc.
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::natural::arithmetic::mul::limb::limbs_slice_mul_limb_in_place;
use crate::natural::arithmetic::mul::{
    limbs_mul_greater_to_out, limbs_mul_greater_to_out_scratch_len,
};
use crate::natural::arithmetic::shl::limbs_slice_shl_in_place;
use crate::natural::arithmetic::shr::limbs_shr_to_out;
use crate::natural::arithmetic::square::{limbs_square_to_out, limbs_square_to_out_scratch_len};
#[cfg(feature = "test_build")]
use crate::natural::logic::significant_bits::limbs_significant_bits;
use crate::natural::InnerNatural::{Large, Small};
use crate::natural::Natural;
#[cfg(feature = "test_build")]
use crate::platform::DoubleLimb;
use crate::platform::Limb;
use alloc::vec::Vec;
use core::mem::swap;
use malachite_base::num::arithmetic::traits::{
    EqModPowerOf2, Parity, Pow, PowAssign, Square, SquareAssign,
};
#[cfg(feature = "test_build")]
use malachite_base::num::arithmetic::traits::{IsPowerOf2, PowerOf2, ShrRound};
use malachite_base::num::basic::integers::PrimitiveInt;
use malachite_base::num::basic::traits::{One, Zero};
#[cfg(feature = "test_build")]
use malachite_base::num::conversion::traits::SplitInHalf;
use malachite_base::num::conversion::traits::{ExactFrom, WrappingFrom};
#[cfg(feature = "test_build")]
use malachite_base::num::logic::traits::BitAccess;
use malachite_base::num::logic::traits::{
    BitIterable, CountOnes, LeadingZeros, SignificantBits, TrailingZeros,
};
#[cfg(feature = "test_build")]
use malachite_base::rounding_modes::RoundingMode::*;
use malachite_base::slices::slice_leading_zeros;

/// This is equivalent to `GMP_NUMB_HALFMAX` from `mpz/n_pow_ui.c`, GMP 6.2.1.
const HALF_MAX: Limb = (1 << (Limb::WIDTH >> 1)) - 1;

// # Worst-case complexity
// $T(n, m) = O(nm \log (nm) \log\log (nm))$
//
// $M(n, m) = O(nm \log (nm))$
//
// where $T$ is time, $M$ is additional memory, $n$ is `xs.len()`, and $m$ is `exp`.
pub_crate_test! {limbs_pow(xs: &[Limb], exp: u64) -> Vec<Limb> {
    let mut out = Vec::new();
    let out_len = limbs_pow_to_out(&mut out, xs, exp);
    out.truncate(out_len);
    out
}}

// # Worst-case complexity
// $T(n) = O(n \log n \log\log n)$
//
// $M(n) = O(n \log n)$
//
// where $T$ is time, $M$ is additional memory, and $n$ is `exp`.
fn len_1_helper(x_0: &mut Limb, out_0: &mut Limb, trailing_zero_bits_out: &mut u64, exp: &mut u64) {
    // Power up as far as possible within `x_0`. We start here with `exp` != 0, but if `exp` is
    // small then we might reach `exp` == 0 and the whole `x` ^ `exp` in `out_0`.
    while *x_0 <= HALF_MAX {
        assert_ne!(*exp, 0);
        if exp.odd() {
            *out_0 *= *x_0;
        }
        *exp >>= 1;
        if *exp == 0 {
            break;
        }
        x_0.square_assign();
    }
    // Combine leftover `trailing_zero_bits_out` into `out_0` to be handled by the final
    // `limbs_slice_mul_limb_in_place` rather than a separate `limbs_slice_shl_in_place`.
    // - `out_0` mustn't be 1 (since then there's no final mul)
    // - `out_0` mustn't overflow
    if *trailing_zero_bits_out != 0
        && *out_0 != 1
        && *out_0 >> (Limb::WIDTH - *trailing_zero_bits_out) == 0
    {
        *out_0 <<= *trailing_zero_bits_out;
        *trailing_zero_bits_out = 0;
    }
}

// # Worst-case complexity
// $T(n, m) = O(nm \log (nm) \log\log (nm))$
//
// $M(n, m) = O(nm \log (nm))$
//
// where $T$ is time, $M$ is additional memory, $n$ is `xs.len()`, and $m$ is `exp`.
//
// This is equivalent to `mpz_n_pow_ui` from `mpz/n_pow_ui.c`, GMP 6.2.1, where `e > 1` and
// `bp.len() != 0`. Returns `rsize`.
fn limbs_pow_to_out(out: &mut Vec<Limb>, xs: &[Limb], mut exp: u64) -> usize {
    assert!(exp > 1);
    let leading_zeros_in = slice_leading_zeros(xs);
    let mut leading_zeros_out = leading_zeros_in * usize::exact_from(exp);
    let mut xs = &xs[leading_zeros_in..];
    let mut x = xs[0];
    // Strip low zero bits from b.
    let trailing_zero_bits_in = TrailingZeros::trailing_zeros(x);
    x >>= trailing_zero_bits_in;
    let mut trailing_zero_bits_out = exp * trailing_zero_bits_in;
    leading_zeros_out += usize::exact_from(trailing_zero_bits_out >> Limb::LOG_WIDTH);
    trailing_zero_bits_out &= Limb::WIDTH_MASK;
    let mut out_0 = 1;
    let mut scratch;
    let mut x_0_x_1 = [0; 2];
    match xs.len() {
        1 => len_1_helper(&mut x, &mut out_0, &mut trailing_zero_bits_out, &mut exp),
        2 => {
            let mut x_1 = xs[1];
            if trailing_zero_bits_in != 0 {
                x |= x_1 << (Limb::WIDTH - trailing_zero_bits_in);
            }
            x_1 >>= trailing_zero_bits_in;
            if x_1 == 0 {
                // Two limbs became one after rshift.
                xs = &xs[..1];
                len_1_helper(&mut x, &mut out_0, &mut trailing_zero_bits_out, &mut exp);
            } else {
                x_0_x_1[0] = x;
                x_0_x_1[1] = x_1;
                xs = &x_0_x_1;
                x = x_1;
            }
        }
        len => {
            if trailing_zero_bits_in != 0 {
                scratch = vec![0; len];
                limbs_shr_to_out(&mut scratch, xs, trailing_zero_bits_in);
                if *scratch.last().unwrap() == 0 {
                    scratch.pop();
                }
                xs = &scratch;
            }
            x = *xs.last().unwrap();
        }
    }
    let len = xs.len();
    // At this point `x` is the most significant limb of the base to use.
    //
    // Each factor of `xs` takes (len * 2 ^ `Limb::WIDTH` - `bits`) bits and there's `exp` of them;
    // +1 limb to round up the division; +1 for multiplies all using an extra limb over the true
    // size; +2 for `out_0` at the end; +1 for `limbs_slice_shl_in_place` at the end.
    //
    // The size calculation here is reasonably accurate. The base is at least half a limb, so in 32
    // bits the worst case is 2 ^ 16 + 1 treated as 17 bits when it will power up as just over 16,
    // an overestimate of 17/16 = 6.25%. For a 64-bit limb it's half that.
    assert_ne!(x, 0);
    let mut out_alloc = usize::exact_from(
        (((u64::exact_from(len) << Limb::LOG_WIDTH) - LeadingZeros::leading_zeros(x)) * exp)
            >> Limb::LOG_WIDTH,
    ) + 5;
    out.resize(out_alloc + leading_zeros_out, 0);
    // Low zero limbs resulting from powers of 2.
    let out_original = out;
    let mut out = &mut out_original[leading_zeros_out..];
    let mut out_len;
    let mut scratch;
    if exp == 0 {
        out[0] = out_0;
        out_len = 1;
        assert_ne!(out[0], 0);
    } else {
        // In the `limbs_slice_mul_limb_in_place` loop or in the `limbs_mul_greater_to_out` loop
        // when the low bit of `exp` is zero, `scratch` only has to hold the second last power step,
        // which is half the size of the final result. There's no need to round up the divide by 2,
        // since `out_alloc` includes a +2 for `out_0` which is not needed by `scratch`. In the
        // `limbs_mul_greater_to_out` loop when the low bit of `exp` is 1, `scratch` must hold
        // nearly the full result, so just size it the same as `out`.
        let mut scratch_len = out_alloc;
        if len == 1 || exp.even() {
            scratch_len >>= 1;
        }
        scratch = vec![0; scratch_len];
        let mut scratch: &mut [Limb] = &mut scratch;
        let bits = LeadingZeros::leading_zeros(exp);
        if len == 1 {
            // Arrange the final result ends up in `out`, not in `scratch`
            if bits.even() {
                swap(&mut out, &mut scratch);
                swap(&mut out_alloc, &mut scratch_len);
            }
            out[0] = x;
            out_len = 1;
            for bit in exp.bits().rev().skip(1) {
                assert!(out_len << 1 <= scratch_len);
                let mut square_scratch = vec![0; limbs_square_to_out_scratch_len(out_len)];
                limbs_square_to_out(scratch, &out[..out_len], &mut square_scratch);
                out_len <<= 1;
                if scratch[out_len - 1] == 0 {
                    out_len -= 1;
                }
                swap(&mut out, &mut scratch);
                swap(&mut out_alloc, &mut scratch_len);
                if bit {
                    assert!(out_len < out_alloc);
                    let carry = limbs_slice_mul_limb_in_place(&mut out[..out_len], x);
                    out[out_len] = carry;
                    if carry != 0 {
                        out_len += 1;
                    }
                }
            }
            if out_0 != 1 {
                assert!(out_len < out_alloc);
                let carry = limbs_slice_mul_limb_in_place(&mut out[..out_len], out_0);
                out[out_len] = carry;
                if carry != 0 {
                    out_len += 1;
                }
            }
        } else {
            // Arrange the final result ends up in `out`, not in `scratch`
            if !CountOnes::count_ones(exp).eq_mod_power_of_2(bits, 1) {
                swap(&mut out, &mut scratch);
                swap(&mut out_alloc, &mut scratch_len);
            }
            out[..len].copy_from_slice(xs);
            out_len = len;
            for bit in exp.bits().rev().skip(1) {
                assert!(out_len << 1 <= scratch_len);
                let mut square_scratch = vec![0; limbs_square_to_out_scratch_len(out_len)];
                limbs_square_to_out(scratch, &out[..out_len], &mut square_scratch);
                out_len <<= 1;
                if scratch[out_len - 1] == 0 {
                    out_len -= 1;
                }
                swap(&mut out, &mut scratch);
                swap(&mut out_alloc, &mut scratch_len);
                if bit {
                    assert!(out_len + len <= scratch_len);
                    let mut mul_scratch =
                        vec![0; limbs_mul_greater_to_out_scratch_len(out_len, len)];
                    let carry =
                        limbs_mul_greater_to_out(scratch, &out[..out_len], xs, &mut mul_scratch);
                    out_len += len;
                    if carry == 0 {
                        out_len -= 1;
                    }
                    swap(&mut out, &mut scratch);
                    swap(&mut out_alloc, &mut scratch_len);
                }
            }
        }
    }
    // Apply any partial limb factors of 2.
    if trailing_zero_bits_out != 0 {
        assert!(out_len < out_alloc);
        let carry = limbs_slice_shl_in_place(&mut out[..out_len], trailing_zero_bits_out);
        out[out_len] = carry;
        if carry != 0 {
            out_len += 1;
        }
    }
    assert_eq!(
        out as *const [Limb],
        &out_original[leading_zeros_out..] as *const [Limb]
    );
    out_len + leading_zeros_out
}

// # Worst-case complexity
// Constant time and additional memory.
#[cfg(feature = "test_build")]
fn exp_predecessor(exp: u64) -> u64 {
    if exp.even() {
        exp >> 1
    } else {
        exp - 1
    }
}

// # Worst-case complexity
// Constant time and additional memory.
#[cfg(feature = "test_build")]
fn estimated_limb_len_helper(x: Limb, exp: u64) -> usize {
    usize::exact_from(
        (x.significant_bits() * exp)
            .shr_round(Limb::LOG_WIDTH, Ceiling)
            .0,
    )
}

// # Worst-case complexity
// Constant time and additional memory.
//
// Never an underestimate.
#[cfg(feature = "test_build")]
fn limb_pow_alt_estimated_out_len(x: Limb, exp: u64) -> usize {
    if exp.even() {
        estimated_limb_len_helper(x, exp >> 1) << 1
    } else {
        estimated_limb_len_helper(x, exp - 1) + 1
    }
}

// # Worst-case complexity
// Constant time and additional memory.
//
// Never an underestimate.
#[cfg(feature = "test_build")]
#[inline]
fn limb_pow_alt_estimated_scratch_len(x: Limb, exp: u64) -> usize {
    limb_pow_alt_estimated_out_len(x, exp_predecessor(exp))
}

// TODO figure out how to find scratch len using mp_bases. x > 1.
//
// # Worst-case complexity
// $T(n) = O(n \log n \log\log n)$
//
// $M(n) = O(n \log n)$
//
// where $T$ is time, $M$ is additional memory, and $n$ is `exp`.
//
// This is equivalent to `mpn_pow_1` from `mpn/generic/pow_1.c`, GMP 6.2.1, where `exp > 1` and `bn
// == 1`.
#[cfg(feature = "test_build")]
fn limb_pow_to_out_alt<'a>(
    mut out: &'a mut [Limb],
    x: Limb,
    exp: u64,
    mut scratch: &'a mut [Limb],
) -> usize {
    assert!(x > 1);
    assert!(exp > 1);
    // Count number of bits in exp, and compute where to put initial square in order to magically
    // get results in the entry out.
    let bits = exp.significant_bits();
    if bits.odd() {
        swap(&mut out, &mut scratch);
    }
    (out[1], out[0]) = DoubleLimb::from(x).square().split_in_half();
    let mut out_len = if out[1] == 0 { 1 } else { 2 };
    for i in (0..bits - 1).rev() {
        if exp.get_bit(i) {
            let (out_last, out_init) = out[..=out_len].split_last_mut().unwrap();
            *out_last = limbs_slice_mul_limb_in_place(out_init, x);
            if *out_last != 0 {
                out_len += 1;
            }
        }
        if i == 0 {
            break;
        }
        let mut square_scratch = vec![0; limbs_square_to_out_scratch_len(out_len)];
        limbs_square_to_out(scratch, &out[..out_len], &mut square_scratch);
        out_len <<= 1;
        if scratch[out_len - 1] == 0 {
            out_len -= 1;
        }
        swap(&mut out, &mut scratch);
    }
    out_len
}

// # Worst-case complexity
// $T(n) = O(n \log n \log\log n)$
//
// $M(n) = O(n \log n)$
//
// where $T$ is time, $M$ is additional memory, and $n$ is `exp`.
#[cfg(feature = "test_build")]
fn limb_pow_alt(x: Limb, exp: u64) -> Vec<Limb> {
    let mut out = vec![0; limb_pow_alt_estimated_out_len(x, exp)];
    let mut scratch = vec![0; limb_pow_alt_estimated_scratch_len(x, exp)];
    let out_len = limb_pow_to_out_alt(&mut out, x, exp, &mut scratch);
    assert!(out_len <= out.len());
    out.truncate(out_len);
    out
}

// # Worst-case complexity
// Constant time and additional memory.
#[cfg(feature = "test_build")]
fn estimated_limbs_len_helper(xs: &[Limb], exp: u64) -> usize {
    usize::exact_from(
        (limbs_significant_bits(xs) * exp)
            .shr_round(Limb::LOG_WIDTH, Ceiling)
            .0,
    )
}

// # Worst-case complexity
// Constant time and additional memory.
//
// Never an underestimate.
#[cfg(feature = "test_build")]
fn limbs_pow_alt_estimated_out_len(xs: &[Limb], exp: u64) -> usize {
    if exp.even() {
        estimated_limbs_len_helper(xs, exp >> 1) << 1
    } else {
        estimated_limbs_len_helper(xs, exp - 1) + xs.len()
    }
}

// # Worst-case complexity
// Constant time and additional memory.
//
// Never an underestimate.
#[cfg(feature = "test_build")]
#[inline]
fn limbs_pow_alt_estimated_scratch_len(xs: &[Limb], exp: u64) -> usize {
    limbs_pow_alt_estimated_out_len(xs, exp_predecessor(exp))
}

// TODO figure out how to find scratch len using mp_bases.
//
// # Worst-case complexity
// $T(n, m) = O(nm \log (nm) \log\log (nm))$
//
// $M(n, m) = O(nm \log (nm))$
//
// where $T$ is time, $M$ is additional memory, $n$ is `xs.len()`, and $m$ is `exp`.
//
// This is equivalent to `mpn_pow_1` from `mpn/generic/pow_1.c`, GMP 6.2.1, where `exp > 1`, `bn >
// 1`, and the last element of `xs` is nonzero.
#[cfg(feature = "test_build")]
fn limbs_pow_to_out_alt<'a>(
    mut out: &'a mut [Limb],
    xs: &[Limb],
    exp: u64,
    mut scratch: &'a mut [Limb],
) -> usize {
    let len = xs.len();
    assert!(len > 1);
    assert!(exp > 1);
    // Count number of bits in exp, and compute where to put initial square in order to magically
    // get results in the entry out.
    let bits = exp.significant_bits();
    if bits.eq_mod_power_of_2(CountOnes::count_ones(exp), 1) {
        swap(&mut out, &mut scratch);
    }
    let mut square_scratch = vec![0; limbs_square_to_out_scratch_len(xs.len())];
    limbs_square_to_out(out, xs, &mut square_scratch);
    let mut out_len = len << 1;
    if out[out_len - 1] == 0 {
        out_len -= 1;
    }
    for i in (0..bits - 1).rev() {
        if exp.get_bit(i) {
            let mut mul_scratch = vec![0; limbs_mul_greater_to_out_scratch_len(out_len, len)];
            if limbs_mul_greater_to_out(scratch, &out[..out_len], xs, &mut mul_scratch) == 0 {
                out_len -= 1;
            }
            out_len += len;
            swap(&mut out, &mut scratch);
        }
        if i == 0 {
            break;
        }
        let mut square_scratch = vec![0; limbs_square_to_out_scratch_len(out_len)];
        limbs_square_to_out(scratch, &out[..out_len], &mut square_scratch);
        out_len <<= 1;
        if scratch[out_len - 1] == 0 {
            out_len -= 1;
        }
        swap(&mut out, &mut scratch);
    }
    out_len
}

// # Worst-case complexity
// $T(n, m) = O(nm \log (nm) \log\log (nm))$
//
// $M(n, m) = O(nm \log (nm))$
//
// where $T$ is time, $M$ is additional memory, $n$ is `xs.len()`, and $m$ is `exp`.
#[cfg(feature = "test_build")]
fn limbs_pow_alt(xs: &[Limb], exp: u64) -> Vec<Limb> {
    let mut out = vec![0; limbs_pow_alt_estimated_out_len(xs, exp)];
    let mut scratch = vec![0; limbs_pow_alt_estimated_scratch_len(xs, exp)];
    let out_len = limbs_pow_to_out_alt(&mut out, xs, exp, &mut scratch);
    assert!(out_len <= out.len());
    out.truncate(out_len);
    out
}

#[cfg(feature = "test_build")]
impl Natural {
    pub fn pow_ref_alt(&self, exp: u64) -> Natural {
        match (self, exp) {
            (_, 0) | (&Natural::ONE, _) => Natural::ONE,
            (&Natural::ZERO, _) => Natural::ZERO,
            (x, 1) => x.clone(),
            (x, 2) => x.square(),
            (x, exp) if x.is_power_of_2() => Natural::power_of_2((x.significant_bits() - 1) * exp),
            (Natural(Small(small)), exp) => {
                if small.significant_bits() * exp <= Limb::WIDTH {
                    Natural(Small(small.checked_pow(u32::wrapping_from(exp)).unwrap()))
                } else {
                    Natural::from_owned_limbs_asc(limb_pow_alt(*small, exp))
                }
            }
            (Natural(Large(ref limbs)), exp) => {
                Natural::from_owned_limbs_asc(limbs_pow_alt(limbs, exp))
            }
        }
    }

    pub fn pow_assign_alt(&mut self, exp: u64) {
        match (&mut *self, exp) {
            (x, 0) => *x = Natural::ONE,
            (_, 1) | (&mut (Natural::ZERO | Natural::ONE), _) => {}
            (x, 2) => x.square_assign(),
            (x, exp) if x.is_power_of_2() => {
                *x = Natural::power_of_2((x.significant_bits() - 1) * exp);
            }
            (Natural(Small(ref mut small)), exp) => {
                if small.significant_bits() * exp <= Limb::WIDTH {
                    *small = small.checked_pow(u32::wrapping_from(exp)).unwrap();
                } else {
                    *self = Natural::from_owned_limbs_asc(limb_pow_alt(*small, exp));
                }
            }
            (Natural(Large(ref mut limbs)), exp) => {
                *self = Natural::from_owned_limbs_asc(limbs_pow_alt(limbs, exp));
            }
        }
    }
}

impl Pow<u64> for Natural {
    type Output = Natural;

    /// Raises a [`Natural`] to a power, taking the [`Natural`] by value.
    ///
    /// $f(x, n) = x^n$.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(nm \log (nm) \log\log (nm))$
    ///
    /// $M(n, m) = O(nm \log (nm))$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `self.significant_bits()`, and $m$ is
    /// `exp`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::Pow;
    /// use malachite_nz::natural::Natural;
    ///
    /// assert_eq!(
    ///     Natural::from(3u32).pow(100).to_string(),
    ///     "515377520732011331036461129765621272702107522001"
    /// );
    /// assert_eq!(
    ///     Natural::from_str("12345678987654321")
    ///         .unwrap()
    ///         .pow(3)
    ///         .to_string(),
    ///     "1881676411868862234942354805142998028003108518161"
    /// );
    /// ```
    #[inline]
    fn pow(mut self, exp: u64) -> Natural {
        self.pow_assign(exp);
        self
    }
}

impl Pow<u64> for &Natural {
    type Output = Natural;

    /// Raises a [`Natural`] to a power, taking the [`Natural`] by reference.
    ///
    /// $f(x, n) = x^n$.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(nm \log (nm) \log\log (nm))$
    ///
    /// $M(n, m) = O(nm \log (nm))$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `self.significant_bits()`, and $m$ is
    /// `exp`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::Pow;
    /// use malachite_nz::natural::Natural;
    ///
    /// assert_eq!(
    ///     (&Natural::from(3u32)).pow(100).to_string(),
    ///     "515377520732011331036461129765621272702107522001"
    /// );
    /// assert_eq!(
    ///     (&Natural::from_str("12345678987654321").unwrap())
    ///         .pow(3)
    ///         .to_string(),
    ///     "1881676411868862234942354805142998028003108518161"
    /// );
    /// ```
    #[inline]
    fn pow(self, exp: u64) -> Natural {
        match (self, exp) {
            (_, 0) | (&Natural::ONE, _) => Natural::ONE,
            (&Natural::ZERO, _) => Natural::ZERO,
            (x, 1) => x.clone(),
            (x, 2) => x.square(),
            (Natural(Small(small)), exp) => {
                if small.significant_bits() * exp <= Limb::WIDTH {
                    Natural(Small(small.checked_pow(u32::wrapping_from(exp)).unwrap()))
                } else {
                    let mut out = Natural(Large(limbs_pow(&[*small], exp)));
                    out.demote_if_small();
                    out
                }
            }
            (Natural(Large(ref limbs)), exp) => {
                let mut out = Natural(Large(limbs_pow(limbs, exp)));
                out.demote_if_small();
                out
            }
        }
    }
}

impl PowAssign<u64> for Natural {
    /// Raises a [`Natural`] to a power in place.
    ///
    /// $x \gets x^n$.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(nm \log (nm) \log\log (nm))$
    ///
    /// $M(n, m) = O(nm \log (nm))$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `self.significant_bits()`, and $m$ is
    /// `exp`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::PowAssign;
    /// use malachite_nz::natural::Natural;
    ///
    /// let mut x = Natural::from(3u32);
    /// x.pow_assign(100);
    /// assert_eq!(
    ///     x.to_string(),
    ///     "515377520732011331036461129765621272702107522001"
    /// );
    ///
    /// let mut x = Natural::from_str("12345678987654321").unwrap();
    /// x.pow_assign(3);
    /// assert_eq!(
    ///     x.to_string(),
    ///     "1881676411868862234942354805142998028003108518161"
    /// );
    /// ```
    fn pow_assign(&mut self, exp: u64) {
        match (&mut *self, exp) {
            (x, 0) => *x = Natural::ONE,
            (_, 1) | (&mut (Natural::ZERO | Natural::ONE), _) => {}
            (x, 2) => x.square_assign(),
            (Natural(Small(ref mut small)), exp) => {
                if small.significant_bits() * exp <= Limb::WIDTH {
                    *small = small.checked_pow(u32::wrapping_from(exp)).unwrap();
                } else {
                    *self = Natural(Large(limbs_pow(&[*small], exp)));
                    self.demote_if_small();
                }
            }
            (Natural(Large(ref mut limbs)), exp) => {
                *self = Natural(Large(limbs_pow(limbs, exp)));
                self.demote_if_small();
            }
        }
    }
}
