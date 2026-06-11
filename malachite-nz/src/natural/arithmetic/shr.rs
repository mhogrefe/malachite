// Copyright © 2026 Mikhail Hogrefe
//
// Uses code adopted from the GNU MP Library.
//
//      Copyright © 1991, 1993, 1994, 1996, 2000-2002 Free Software Foundation, Inc.
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::natural::InnerNatural::{Large, Small};
use crate::natural::{Natural, bit_to_limb_count_floor};
use crate::platform::Limb;
use alloc::vec::Vec;
use core::ops::{Shl, ShlAssign, Shr, ShrAssign};
use malachite_base::num::arithmetic::traits::UnsignedAbs;
use malachite_base::num::basic::integers::PrimitiveInt;
use malachite_base::num::basic::signeds::PrimitiveSigned;
use malachite_base::num::basic::traits::Zero;
use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base::num::conversion::traits::{ExactFrom, WrappingFrom};
use malachite_base::vecs::vec_delete_left;

// Interpreting a slice of `Limb`s as the limbs (in ascending order) of a `Natural`, returns the
// limbs of the `Natural` right-shifted by a `Limb`, rounding down.
//
// # Worst-case complexity
// $T(n) = O(n)$
//
// $M(n) = O(n)$
//
// where $T$ is time, $M$ is additional memory and $n$ is `max(1, xs.len() - bits / Limb::WIDTH)`.
//
// This is equivalent to `mpn_rshift` from `mpn/generic/rshift.c`, GMP 6.2.1, where the result is
// returned.
pub_crate_test! {limbs_shr(xs: &[Limb], bits: u64) -> Vec<Limb> {
    let delete_count = bit_to_limb_count_floor(bits);
    if delete_count >= xs.len() {
        Vec::new()
    } else {
        let small_bits = bits & Limb::WIDTH_MASK;
        let src = &xs[delete_count..];
        if small_bits == 0 {
            src.to_vec()
        } else {
            // Writing each limb exactly once via extend is faster than copying and then shifting in
            // place (one pass instead of two).
            let cobits = Limb::WIDTH - small_bits;
            let mut out = Vec::with_capacity(src.len());
            out.extend(
                src.windows(2)
                    .map(|w| (w[0] >> small_bits) | (w[1] << cobits)),
            );
            out.push(src[src.len() - 1] >> small_bits);
            out
        }
    }
}}

// Interpreting a nonempty slice of `Limb`s as the limbs (in ascending order) of a `Natural`, writes
// the limbs of the `Natural` right-shifted by a `Limb` to an output slice. The output slice must be
// at least as long as the input slice. The `Limb` must be between 1 and `Limb::WIDTH` - 1,
// inclusive. The carry, or the bits that are shifted past the width of the input slice, is
// returned. The input slice should not only contain zeros.
//
// # Worst-case complexity
// $T(n) = O(n)$
//
// $M(n) = O(1)$
//
// where $T$ is time, $M$ is additional memory and $n$ is `xs.len()`.
//
// # Panics
// Panics if `xs` is empty, `out` is shorter than `xs`, `bits` is 0, or `bits` is greater than or
// equal to `Limb::WIDTH`.
//
// This is equivalent to `mpn_rshift` from `mpn/generic/rshift.c`, GMP 6.2.1.
pub_crate_test! {limbs_shr_to_out(out: &mut [Limb], xs: &[Limb], bits: u64) -> Limb {
    let len = xs.len();
    assert_ne!(len, 0);
    assert_ne!(bits, 0);
    assert!(bits < Limb::WIDTH);
    assert!(out.len() >= len);
    let cobits = Limb::WIDTH - bits;
    // Windows form: each output limb depends only on two adjacent input limbs, so iterations are
    // independent (no loop-carried value) and LLVM auto-vectorizes (see the analogous
    // limbs_shl_to_out).
    let remaining_bits = xs[0] << cobits;
    for (o, w) in out[..len - 1].iter_mut().zip(xs.windows(2)) {
        *o = (w[0] >> bits) | (w[1] << cobits);
    }
    out[len - 1] = xs[len - 1] >> bits;
    remaining_bits
}}

// Interpreting a nonempty slice of `Limb`s as the limbs (in ascending order) of a `Natural`, writes
// the limbs of the `Natural` right-shifted by a `Limb` to the input slice. The `Limb` must be
// between 1 and `Limb::WIDTH` - 1, inclusive. The carry, or the bits that are shifted past the
// width of the input slice, is returned.
//
// # Worst-case complexity
// $T(n) = O(n)$
//
// $M(n) = O(1)$
//
// where $T$ is time, $M$ is additional memory and $n$ is `xs.len()`.
//
// # Panics
// Panics if `xs` is empty, `bits` is 0, or `bits` is greater than or equal to `Limb::WIDTH`.
//
// This is equivalent to `mpn_rshift` from `mpn/generic/rshift.c`, GMP 6.2.1, where `rp == up`.
pub_crate_test! {limbs_slice_shr_in_place<T: PrimitiveUnsigned>(xs: &mut [T], bits: u64) -> T {
    assert_ne!(bits, 0);
    assert!(bits < T::WIDTH);
    let len = xs.len();
    assert_ne!(len, 0);
    let cobits = T::WIDTH - bits;
    // In-place windows form, ascending so that each limb is read before it is overwritten;
    // iterations are independent (no loop-carried value), letting LLVM auto-vectorize.
    let remaining_bits = xs[0] << cobits;
    for i in 0..len - 1 {
        xs[i] = (xs[i] >> bits) | (xs[i + 1] << cobits);
    }
    xs[len - 1] >>= bits;
    remaining_bits
}}

// Interpreting a `Vec` of `Limb`s as the limbs (in ascending order) of a `Natural`, writes the
// limbs of the `Natural` right-shifted by a `Limb` to the input `Vec`.
//
// # Worst-case complexity
// $T(n) = O(n)$
//
// $M(n) = O(1)$
//
// where $T$ is time, $M$ is additional memory and $n$ is `max(1, xs.len() - bits / Limb::WIDTH)`.
//
// This is equivalent to `mpn_rshift` from `mpn/generic/rshift.c`, GMP 6.2.1, where `rp == up` and
// if `cnt` is sufficiently large, limbs are removed from `rp`.
pub_crate_test! {limbs_vec_shr_in_place(xs: &mut Vec<Limb>, bits: u64) {
    let delete_count = bit_to_limb_count_floor(bits);
    if delete_count >= xs.len() {
        xs.clear();
    } else {
        let small_shift = bits & Limb::WIDTH_MASK;
        if small_shift == 0 {
            vec_delete_left(xs, delete_count);
        } else {
            // A single ascending pass both shifts and translates by delete_count, rather than
            // deleting the low limbs (a full memmove) and then making a second pass to shift.
            // Ascending order ensures every limb is read before it is overwritten, and iterations
            // are independent, letting LLVM auto-vectorize.
            let old_len = xs.len();
            let new_len = old_len - delete_count;
            let cobits = Limb::WIDTH - small_shift;
            for i in 0..new_len - 1 {
                xs[i] = (xs[i + delete_count] >> small_shift)
                    | (xs[i + delete_count + 1] << cobits);
            }
            xs[new_len - 1] = xs[old_len - 1] >> small_shift;
            xs.truncate(new_len);
        }
    }
}}

fn shr_unsigned_ref<T: Copy + Eq + Ord + WrappingFrom<u64> + Zero>(x: &Natural, bits: T) -> Natural
where
    u64: ExactFrom<T>,
    Limb: Shr<T, Output = Limb>,
{
    match (x, bits) {
        (&Natural::ZERO, _) => x.clone(),
        (_, bits) if bits == T::ZERO => x.clone(),
        (Natural(Small(_)), bits) if bits >= T::wrapping_from(Limb::WIDTH) => Natural::ZERO,
        (Natural(Small(small)), bits) => Natural(Small(*small >> bits)),
        (Natural(Large(limbs)), bits) => {
            Natural::from_owned_limbs_asc(limbs_shr(limbs, u64::exact_from(bits)))
        }
    }
}

fn shr_assign_unsigned<T: PrimitiveUnsigned>(x: &mut Natural, bits: T)
where
    u64: ExactFrom<T>,
    Limb: ShrAssign<T>,
{
    match (&mut *x, bits) {
        (&mut Natural::ZERO, _) => {}
        (_, bits) if bits == T::ZERO => {}
        (Natural(Small(small)), bits) if bits >= T::wrapping_from(Limb::WIDTH) => {
            *small = 0;
        }
        (Natural(Small(small)), bits) => {
            *small >>= bits;
        }
        (Natural(Large(limbs)), bits) => {
            limbs_vec_shr_in_place(limbs, u64::exact_from(bits));
            x.trim();
        }
    }
}

macro_rules! impl_natural_shr_unsigned {
    ($t:ident) => {
        impl Shr<$t> for Natural {
            type Output = Natural;

            /// Right-shifts a [`Natural`] (divides it by a power of 2 and takes the floor), taking
            /// it by value.
            ///
            /// $$
            /// f(x, k) = \left \lfloor \frac{x}{2^k} \right \rfloor.
            /// $$
            ///
            /// # Worst-case complexity
            /// $T(n) = O(n)$
            ///
            /// $M(n) = O(1)$
            ///
            /// where $T$ is time, $M$ is additional memory and $n$ is `max(1,
            /// self.significant_bits() - bits)`.
            ///
            /// # Examples
            /// See [here](super::shr#shr).
            #[inline]
            fn shr(mut self, bits: $t) -> Natural {
                self >>= bits;
                self
            }
        }

        impl<'a> Shr<$t> for &Natural {
            type Output = Natural;

            /// Right-shifts a [`Natural`] (divides it by a power of 2 and takes the floor), taking
            /// it by reference.
            ///
            /// $$
            /// f(x, k) = \left \lfloor \frac{x}{2^k} \right \rfloor.
            /// $$
            ///
            /// # Worst-case complexity
            /// $T(n) = O(n)$
            ///
            /// $M(n) = O(n)$
            ///
            /// where $T$ is time, $M$ is additional memory and $n$ is `max(1,
            /// self.significant_bits() - bits)`.
            ///
            /// # Examples
            /// See [here](super::shr#shr).
            #[inline]
            fn shr(self, bits: $t) -> Natural {
                shr_unsigned_ref(self, bits)
            }
        }

        impl ShrAssign<$t> for Natural {
            /// Right-shifts a [`Natural`] (divides it by a power of 2 and takes the floor), in
            /// place.
            ///
            /// $$
            /// x \gets \left \lfloor \frac{x}{2^k} \right \rfloor.
            /// $$
            ///
            /// # Worst-case complexity
            /// $T(n) = O(n)$
            ///
            /// $M(n) = O(1)$
            ///
            /// where $T$ is time, $M$ is additional memory and $n$ is `max(1,
            /// self.significant_bits() - bits)`.
            ///
            /// # Examples
            /// See [here](super::shr#shr_assign).
            #[inline]
            fn shr_assign(&mut self, bits: $t) {
                shr_assign_unsigned(self, bits);
            }
        }
    };
}
apply_to_unsigneds!(impl_natural_shr_unsigned);

fn shr_signed_ref<'a, U, S: PrimitiveSigned + UnsignedAbs<Output = U>>(
    x: &'a Natural,
    bits: S,
) -> Natural
where
    &'a Natural: Shl<U, Output = Natural> + Shr<U, Output = Natural>,
{
    if bits >= S::ZERO {
        x >> bits.unsigned_abs()
    } else {
        x << bits.unsigned_abs()
    }
}

fn shr_assign_signed<U, S: PrimitiveSigned + UnsignedAbs<Output = U>>(x: &mut Natural, bits: S)
where
    Natural: ShlAssign<U> + ShrAssign<U>,
{
    if bits >= S::ZERO {
        *x >>= bits.unsigned_abs();
    } else {
        *x <<= bits.unsigned_abs();
    }
}

macro_rules! impl_natural_shr_signed {
    ($t:ident) => {
        impl Shr<$t> for Natural {
            type Output = Natural;

            /// Right-shifts a [`Natural`] (divides it by a power of 2 and takes the floor or
            /// multiplies it by a power of 2), taking it by value.
            ///
            /// $$
            /// f(x, k) = \left \lfloor \frac{x}{2^k} \right \rfloor.
            /// $$
            ///
            /// # Worst-case complexity
            /// $T(n) = O(n)$
            ///
            /// $M(n) = O(1)$
            ///
            /// where $T$ is time, $M$ is additional memory and $n$ is `max(1,
            /// self.significant_bits() - bits)`.
            ///
            /// # Examples
            /// See [here](super::shr#shr).
            #[inline]
            fn shr(mut self, bits: $t) -> Natural {
                self >>= bits;
                self
            }
        }

        impl<'a> Shr<$t> for &Natural {
            type Output = Natural;

            /// Right-shifts a [`Natural`] (divides it by a power of 2 and takes the floor or
            /// multiplies it by a power of 2), taking it by reference.
            ///
            /// $$
            /// f(x, k) = \left \lfloor \frac{x}{2^k} \right \rfloor.
            /// $$
            ///
            /// # Worst-case complexity
            /// $T(n) = O(n)$
            ///
            /// $M(n) = O(n)$
            ///
            /// where $T$ is time, $M$ is additional memory and $n$ is `max(1,
            /// self.significant_bits() - bits)`.
            ///
            /// # Examples
            /// See [here](super::shr#shr).
            #[inline]
            fn shr(self, bits: $t) -> Natural {
                shr_signed_ref(self, bits)
            }
        }

        impl ShrAssign<$t> for Natural {
            /// Right-shifts a [`Natural`] (divides it by a power of 2 and takes the floor or
            /// multiplies it by a power of 2), in place.
            ///
            /// $$
            /// x \gets \left \lfloor \frac{x}{2^k} \right \rfloor.
            /// $$
            ///
            /// # Worst-case complexity
            /// $T(n) = O(n)$
            ///
            /// $M(n) = O(1)$
            ///
            /// where $T$ is time, $M$ is additional memory and $n$ is `max(1,
            /// self.significant_bits() - bits)`.
            ///
            /// # Examples
            /// See [here](super::shr#shr_assign).
            #[inline]
            fn shr_assign(&mut self, bits: $t) {
                shr_assign_signed(self, bits);
            }
        }
    };
}
apply_to_signeds!(impl_natural_shr_signed);
