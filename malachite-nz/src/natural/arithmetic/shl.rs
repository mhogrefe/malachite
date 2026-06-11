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
use malachite_base::num::arithmetic::traits::{ArithmeticCheckedShl, UnsignedAbs};
use malachite_base::num::basic::integers::PrimitiveInt;
use malachite_base::num::basic::signeds::PrimitiveSigned;
use malachite_base::num::basic::traits::Zero;
use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base::num::conversion::traits::ExactFrom;
use malachite_base::vecs::vec_pad_left;

// Interpreting a slice of `Limb`s as the limbs (in ascending order) of a `Natural`, returns the
// limbs of the `Natural` left-shifted by a `Limb`.
//
// # Worst-case complexity
// $T(n, m) = O(n + m)$
//
// $M(n, m) = O(n + m)$
//
// where $T$ is time, $M$ is additional memory, $n$ is `xs.len()`, and $m$ is `bits / Limb::WIDTH`.
//
// This is equivalent to `mpn_lshift` from `mpn/generic/lshift.c`, GMP 6.2.1, where the result is
// returned.
pub_crate_test! {limbs_shl(xs: &[Limb], bits: u64) -> Vec<Limb> {
    let small_bits = bits & Limb::WIDTH_MASK;
    let limb_offset = bit_to_limb_count_floor(bits);
    let xs_len = xs.len();
    // Allocating the full result up front and using the vectorizable kernel is much faster than
    // pushing one limb at a time (which is guaranteed to reallocate mid-shift).
    if small_bits == 0 {
        let mut out = vec![0; limb_offset + xs_len];
        out[limb_offset..].copy_from_slice(xs);
        out
    } else if xs_len == 0 {
        vec![0; limb_offset]
    } else {
        // Writing each limb exactly once via extend avoids vec![0; n]'s zero-init pass; this
        // measured up to 1.9x faster than zero-init-then-overwrite at large sizes (one store pass
        // instead of two).
        let cobits = Limb::WIDTH - small_bits;
        let mut out = Vec::with_capacity(limb_offset + xs_len + 1);
        out.resize(limb_offset, 0);
        out.push(xs[0] << small_bits);
        out.extend(
            xs.windows(2)
                .map(|w| (w[1] << small_bits) | (w[0] >> cobits)),
        );
        let remaining_bits = xs[xs_len - 1] >> cobits;
        if remaining_bits != 0 {
            out.push(remaining_bits);
        }
        out
    }
}}

// Interpreting a slice of `Limb`s as the limbs (in ascending order) of a `Natural`, writes the
// limbs of the `Natural` left-shifted by a `Limb` to an output slice. The output slice must be at
// least as long as the input slice. The `Limb` must be between 1 and `Limb::WIDTH` - 1, inclusive.
// The carry, or the bits that are shifted past the width of the input slice, is returned.
//
// # Worst-case complexity
// $T(n) = O(n)$
//
// $M(n) = O(n)$
//
// where $T$ is time, $M$ is additional memory, and $n$ is `xs.len()`.
//
// # Panics
// Panics if `out` is shorter than `xs`, `bits` is 0, or `bits` is greater than or equal to
// `Limb::WIDTH`.
//
// This is equivalent to `mpn_lshift` from `mpn/generic/lshift.c`, GMP 6.2.1.
pub_crate_test! {limbs_shl_to_out(out: &mut [Limb], xs: &[Limb], bits: u64) -> Limb {
    assert_ne!(bits, 0);
    assert!(bits < Limb::WIDTH);
    let len = xs.len();
    if len == 0 {
        return 0;
    }
    let cobits = Limb::WIDTH - bits;
    // Windows form: each output limb depends only on two adjacent input limbs, so iterations are
    // independent (no loop-carried value) and LLVM auto-vectorizes; this measured 2-2.4x faster
    // than the serial remaining_bits formulation.
    out[0] = xs[0] << bits;
    for (o, w) in out[1..len].iter_mut().zip(xs.windows(2)) {
        *o = (w[1] << bits) | (w[0] >> cobits);
    }
    xs[len - 1] >> cobits
}}

// Interpreting a slice of `Limb`s as the limbs (in ascending order) of a `Natural`, writes the
// limbs of the `Natural` left-shifted by a `Limb` to the input slice. The `Limb` must be between 1
// and `Limb::WIDTH` - 1, inclusive. The carry, or the bits that are shifted past the width of the
// input slice, is returned.
//
// # Worst-case complexity
// $T(n) = O(n)$
//
// $M(n) = O(1)$
//
// where $T$ is time, $M$ is additional memory, and $n$ is `xs.len()`.
//
// This is equivalent to `mpn_lshift` from `mpn/generic/lshift.c`, GMP 6.2.1, where `rp == up`.
pub_crate_test! {limbs_slice_shl_in_place(xs: &mut [Limb], bits: u64) -> Limb {
    assert_ne!(bits, 0);
    assert!(bits < Limb::WIDTH);
    let len = xs.len();
    if len == 0 {
        return 0;
    }
    let cobits = Limb::WIDTH - bits;
    // In-place windows form, descending so that each limb is read before it is overwritten;
    // iterations are independent (no loop-carried value), letting LLVM auto-vectorize.
    let remaining_bits = xs[len - 1] >> cobits;
    for i in (1..len).rev() {
        xs[i] = (xs[i] << bits) | (xs[i - 1] >> cobits);
    }
    xs[0] <<= bits;
    remaining_bits
}}

// Interpreting a nonempty `Vec` of `Limb`s as the limbs (in ascending order) of a `Natural`, writes
// the limbs of the `Natural` left-shifted by a `Limb` to the input `Vec`.
//
// # Worst-case complexity
// $T(n, m) = O(n + m)$
//
// $M(n, m) = O(n + m)$
//
// # Panics
// Panics if `xs` is empty.
//
// This is equivalent to `mpn_lshift` from `mpn/generic/lshift.c`, GMP 6.2.1, where `rp == up` and
// the carry is appended to `rp`.
pub_crate_test! {limbs_vec_shl_in_place(xs: &mut Vec<Limb>, bits: u64) {
    let small_bits = bits & Limb::WIDTH_MASK;
    let limb_offset = bit_to_limb_count_floor(bits);
    if small_bits == 0 {
        vec_pad_left(xs, limb_offset, 0);
        return;
    }
    let old_len = xs.len();
    if old_len == 0 {
        xs.resize(limb_offset, 0);
        return;
    }
    let cobits = Limb::WIDTH - small_bits;
    // A single descending pass both shifts and translates by limb_offset, rather than shifting in
    // place and then making a second full pass to insert the low zero limbs. Descending order
    // ensures every limb is read before being overwritten, and iterations are independent, letting
    // LLVM auto-vectorize.
    xs.resize(old_len + limb_offset + 1, 0);
    let remaining_bits = xs[old_len - 1] >> cobits;
    xs[old_len + limb_offset] = remaining_bits;
    for i in (1..old_len).rev() {
        xs[i + limb_offset] = (xs[i] << small_bits) | (xs[i - 1] >> cobits);
    }
    xs[limb_offset] = xs[0] << small_bits;
    xs[..limb_offset].fill(0);
    if remaining_bits == 0 {
        xs.pop();
    }
}}

// Interpreting a slice of `Limb`s as the limbs (in ascending order) of a `Natural`, writes the
// limbs of the `Natural` left-shifted by a `Limb`, and complemented, to an output slice. The output
// slice must be at least as long as the input slice. The `Limb` must be between 1 and `Limb::WIDTH`
// - 1, inclusive. The carry, or the bits that are shifted past the width of the input slice, is
// returned. The carry is not complemented.
//
// # Worst-case complexity
// $T(n) = O(n)$
//
// $M(n) = O(1)$
//
// where $T$ is time, $M$ is additional memory, and $n$ is `xs.len()`.
//
// # Panics
// Panics if `out` is shorter than `xs`, `xs` is empty, `bits` is 0, or `bits` is greater than or
// equal to `Limb::WIDTH`.
//
// This is equivalent to `mpn_lshiftc` from `mpn/generic/lshift.c`, GMP 6.2.1.
pub_crate_test! {limbs_shl_with_complement_to_out(
    out: &mut [Limb],
    xs: &[Limb],
    bits: u64
) -> Limb {
    let n = xs.len();
    assert_ne!(n, 0);
    assert_ne!(bits, 0);
    assert!(bits < Limb::WIDTH);
    let cobits = Limb::WIDTH - bits;
    // Windows form: iterations are independent, letting LLVM auto-vectorize.
    out[0] = !(xs[0] << bits);
    for (o, w) in out[1..n].iter_mut().zip(xs.windows(2)) {
        *o = !((w[1] << bits) | (w[0] >> cobits));
    }
    xs[n - 1] >> cobits
}}

fn shl_ref_unsigned<T: PrimitiveUnsigned>(x: &Natural, bits: T) -> Natural
where
    u64: ExactFrom<T>,
    Limb: ArithmeticCheckedShl<T, Output = Limb>,
{
    match (x, bits) {
        (&Natural::ZERO, _) => x.clone(),
        (_, bits) if bits == T::ZERO => x.clone(),
        (Natural(Small(small)), bits) => {
            Natural(if let Some(shifted) = small.arithmetic_checked_shl(bits) {
                Small(shifted)
            } else {
                Large(limbs_shl(&[*small], u64::exact_from(bits)))
            })
        }
        (Natural(Large(limbs)), bits) => Natural(Large(limbs_shl(limbs, u64::exact_from(bits)))),
    }
}

fn shl_assign<T: PrimitiveUnsigned>(x: &mut Natural, bits: T)
where
    u64: ExactFrom<T>,
    Limb: ArithmeticCheckedShl<T, Output = Limb>,
{
    match (&mut *x, bits) {
        (&mut Natural::ZERO, _) => {}
        (_, bits) if bits == T::ZERO => {}
        (Natural(Small(small)), bits) => {
            if let Some(shifted) = small.arithmetic_checked_shl(bits) {
                *small = shifted;
            } else {
                *x = Natural(Large(limbs_shl(&[*small], u64::exact_from(bits))));
            }
        }
        (Natural(Large(limbs)), bits) => {
            limbs_vec_shl_in_place(limbs, u64::exact_from(bits));
        }
    }
}

macro_rules! impl_natural_shl_unsigned {
    ($t:ident) => {
        impl Shl<$t> for Natural {
            type Output = Natural;

            /// Left-shifts a [`Natural`] (multiplies it by a power of 2), taking it by value.
            ///
            /// $f(x, k) = x2^k$.
            ///
            /// # Worst-case complexity
            /// $T(n, m) = O(n + m)$
            ///
            /// $M(n, m) = O(n + m)$
            ///
            /// where $T$ is time, $M$ is additional memory, $n$ is `self.significant_bits()`, and
            /// $m$ is `bits`.
            ///
            /// # Examples
            /// See [here](super::shl#shl).
            #[inline]
            fn shl(mut self, bits: $t) -> Natural {
                self <<= bits;
                self
            }
        }

        impl Shl<$t> for &Natural {
            type Output = Natural;

            /// Left-shifts a [`Natural`] (multiplies it by a power of 2), taking it by reference.
            ///
            /// $f(x, k) = x2^k$.
            ///
            /// # Worst-case complexity
            /// $T(n, m) = O(n + m)$
            ///
            /// $M(n, m) = O(n + m)$
            ///
            /// where $T$ is time, $M$ is additional memory, $n$ is `self.significant_bits()`, and
            /// $m$ is `bits`.
            ///
            /// # Examples
            /// See [here](super::shl#shl).
            #[inline]
            fn shl(self, bits: $t) -> Natural {
                shl_ref_unsigned(self, bits)
            }
        }

        impl ShlAssign<$t> for Natural {
            /// Left-shifts a [`Natural`] (multiplies it by a power of 2), in place.
            ///
            /// $x \gets x2^k$.
            ///
            /// # Worst-case complexity
            /// $T(n, m) = O(n + m)$
            ///
            /// $M(n, m) = O(n + m)$
            ///
            /// where $T$ is time, $M$ is additional memory, $n$ is `self.significant_bits()`, and
            /// $m$ is `bits`.s
            ///
            /// # Examples
            /// See [here](super::shl#shl_assign).
            #[inline]
            fn shl_assign(&mut self, bits: $t) {
                shl_assign(self, bits);
            }
        }
    };
}
apply_to_unsigneds!(impl_natural_shl_unsigned);

fn shl_ref_signed<'a, U, S: PrimitiveSigned + UnsignedAbs<Output = U>>(
    x: &'a Natural,
    bits: S,
) -> Natural
where
    &'a Natural: Shl<U, Output = Natural> + Shr<U, Output = Natural>,
{
    if bits >= S::ZERO {
        x << bits.unsigned_abs()
    } else {
        x >> bits.unsigned_abs()
    }
}

fn shl_assign_signed<U, S: PrimitiveSigned + UnsignedAbs<Output = U>>(x: &mut Natural, bits: S)
where
    Natural: ShlAssign<U> + ShrAssign<U>,
{
    if bits >= S::ZERO {
        *x <<= bits.unsigned_abs();
    } else {
        *x >>= bits.unsigned_abs();
    }
}

macro_rules! impl_natural_shl_signed {
    ($t:ident) => {
        impl Shl<$t> for Natural {
            type Output = Natural;

            /// Left-shifts a [`Natural`] (multiplies it by a power of 2 or divides it by a power of
            /// 2 and takes the floor), taking it by value.
            ///
            /// $$
            /// f(x, k) = \lfloor x2^k \rfloor.
            /// $$
            ///
            /// # Worst-case complexity
            /// $T(n, m) = O(n + m)$
            ///
            /// $M(n, m) = O(n + m)$
            ///
            /// where $T$ is time, $M$ is additional memory, $n$ is `self.significant_bits()`, and
            /// $m$ is `max(bits, 0)`.
            ///
            /// # Examples
            /// See [here](super::shl#shl).
            #[inline]
            fn shl(mut self, bits: $t) -> Natural {
                self <<= bits;
                self
            }
        }

        impl<'a> Shl<$t> for &Natural {
            type Output = Natural;

            /// Left-shifts a [`Natural`] (multiplies it by a power of 2 or divides it by a power of
            /// 2 and takes the floor), taking it by reference.
            ///
            /// $$
            /// f(x, k) = \lfloor x2^k \rfloor.
            /// $$
            ///
            /// # Worst-case complexity
            /// $T(n, m) = O(n + m)$
            ///
            /// $M(n, m) = O(n + m)$
            ///
            /// where $T$ is time, $M$ is additional memory, $n$ is `self.significant_bits()`, and
            /// $m$ is `max(bits, 0)`.
            ///
            /// # Examples
            /// See [here](super::shl#shl).
            #[inline]
            fn shl(self, bits: $t) -> Natural {
                shl_ref_signed(self, bits)
            }
        }

        impl ShlAssign<$t> for Natural {
            /// Left-shifts a [`Natural`] (multiplies it by a power of 2 or divides it by a power of
            /// 2 and takes the floor), in place.
            ///
            /// $$
            /// x \gets \lfloor x2^k \rfloor.
            /// $$
            ///
            /// # Worst-case complexity
            /// $T(n, m) = O(n + m)$
            ///
            /// $M(n, m) = O(n + m)$
            ///
            /// where $T$ is time, $M$ is additional memory, $n$ is `self.significant_bits()`, and
            /// $m$ is `max(bits, 0)`.
            ///
            /// See [here](super::shl#shl_assign).
            #[inline]
            fn shl_assign(&mut self, bits: $t) {
                shl_assign_signed(self, bits);
            }
        }
    };
}
apply_to_signeds!(impl_natural_shl_signed);
