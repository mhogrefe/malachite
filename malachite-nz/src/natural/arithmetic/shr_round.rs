// Copyright © 2024 Mikhail Hogrefe
//
// Uses code adopted from the GNU MP Library.
//
//      Copyright © 1991, 1993, 1994, 1996, 1998, 1999, 2001, 2002, 2004, 2012, 2015 Free Software
//      Foundation, Inc.
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::natural::arithmetic::add::limbs_vec_add_limb_in_place;
use crate::natural::arithmetic::divisible_by_power_of_2::limbs_divisible_by_power_of_2;
use crate::natural::arithmetic::shr::{
    limbs_shr, limbs_slice_shr_in_place, limbs_vec_shr_in_place,
};
use crate::natural::logic::bit_access::limbs_get_bit;
use crate::natural::InnerNatural::{Large, Small};
use crate::natural::Natural;
use crate::platform::Limb;
use alloc::vec::Vec;
use core::cmp::Ordering::{self, *};
use core::ops::{Shl, ShlAssign};
use malachite_base::num::arithmetic::traits::{Parity, ShrRound, ShrRoundAssign, UnsignedAbs};
use malachite_base::num::basic::integers::PrimitiveInt;
use malachite_base::num::basic::signeds::PrimitiveSigned;
use malachite_base::num::basic::traits::Zero;
use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base::num::conversion::traits::ExactFrom;
use malachite_base::rounding_modes::RoundingMode::{self, *};
use malachite_base::slices::slice_test_zero;
use malachite_base::vecs::vec_delete_left;

// Interpreting a slice of `Limb`s as the limbs (in ascending order) of a `Natural`, returns the
// limbs of the `Natural` right-shifted by a `Limb`, rounding up. The limbs should not all be zero.
//
// # Worst-case complexity
// $T(n) = O(n)$
//
// $M(n) = O(n)$
//
// where $T$ is time, $M$ is additional memory and $n$ is `max(1, xs.len() - bits / Limb::WIDTH)`.
//
// This is equivalent to `cfdiv_q_2exp` from `mpz/cfdiv_q_2exp.c`, GMP 6.2.1, where `u` is
// non-negative, `dir == 1`, and the result is returned.
pub_test! {limbs_shr_round_up(xs: &[Limb], bits: u64) -> (Vec<Limb>, Ordering) {
    let delete_count = usize::exact_from(bits >> Limb::LOG_WIDTH);
    if delete_count >= xs.len() {
        (vec![1], Greater)
    } else {
        let (xs_lo, xs_hi) = xs.split_at(delete_count);
        let mut exact = slice_test_zero(xs_lo);
        let mut out = xs_hi.to_vec();
        let small_bits = bits & Limb::WIDTH_MASK;
        if small_bits != 0 {
            exact &= limbs_slice_shr_in_place(&mut out, small_bits) == 0;
        }
        if !exact {
            limbs_vec_add_limb_in_place(&mut out, 1);
        }
        (
            out,
            if exact {
                Equal
            } else {
                Greater
            },
        )
    }
}}

// # Worst-case complexity
// $T(n) = O(n)$
//
// $M(n) = O(n)$
//
// where $T$ is time, $M$ is additional memory and $n$ is `max(1, xs.len() - bits / Limb::WIDTH)`.
fn limbs_shr_round_half_integer_to_even(xs: &[Limb], bits: u64) -> (Vec<Limb>, Ordering) {
    let delete_count = usize::exact_from(bits >> Limb::LOG_WIDTH);
    if delete_count >= xs.len() {
        (Vec::new(), if slice_test_zero(xs) { Equal } else { Less })
    } else {
        let small_bits = bits & Limb::WIDTH_MASK;
        let (xs_lo, xs_hi) = xs.split_at(delete_count);
        let mut exact = slice_test_zero(xs_lo);
        let mut out = xs_hi.to_vec();
        if small_bits != 0 {
            exact &= limbs_slice_shr_in_place(&mut out, small_bits) == 0;
        }
        if !out.is_empty() && out[0].odd() {
            limbs_vec_add_limb_in_place(&mut out, 1);
            (out, Greater)
        } else {
            (out, if exact { Equal } else { Less })
        }
    }
}

// Interpreting a slice of `Limb`s as the limbs (in ascending order) of a `Natural`, returns the
// limbs of the `Natural` right-shifted by a `Limb`, rounding to the `Natural` nearest to the actual
// value of `self` divided by `2 ^ bits`. If the actual value is exactly between two integers, it is
// rounded to the even one.
//
// # Worst-case complexity
// $T(n) = O(n)$
//
// $M(m) = O(m)$
//
// where $T$ is time, $M$ is additional memory, $n$ is `xs.len()`, and $m$ is `max(1, xs.len() -
// bits / Limb::WIDTH)`.
pub_test! {limbs_shr_round_nearest(xs: &[Limb], bits: u64) -> (Vec<Limb>, Ordering) {
    if bits == 0 {
        (xs.to_vec(), Equal)
    } else {
        let d = slice_test_zero(xs) || limbs_divisible_by_power_of_2(xs, bits - 1);
        if !limbs_get_bit(xs, bits - 1) {
            (
                limbs_shr(xs, bits),
                if d { Equal } else { Less },
            )
        } else if d {
            limbs_shr_round_half_integer_to_even(xs, bits)
        } else {
            limbs_shr_round_up(xs, bits)
        }
    }
}}

// Interpreting a slice of `Limb`s as the limbs (in ascending order) of a `Natural`, returns the
// limbs of the `Natural` right-shifted by a `Limb`, if the shift is exact (doesn't remove any
// `true` bits). If the shift is inexact, `None` is returned. The limbs should not all be zero.
//
// # Worst-case complexity
// $T(n) = O(n)$
//
// $M(n) = O(m)$
//
// where $T$ is time, $M$ is additional memory, $n$ is `xs.len()`, and $m$ is `max(1, xs.len() -
// bits / Limb::WIDTH)`.
pub_test! {limbs_shr_exact(xs: &[Limb], bits: u64) -> Option<Vec<Limb>> {
    if limbs_divisible_by_power_of_2(xs, bits) {
        Some(limbs_shr(xs, bits))
    } else {
        None
    }
}}

// Interpreting a slice of `Limb`s as the limbs (in ascending order) of a `Natural`, returns the
// limbs of the `Natural` right-shifted by a `Limb`, rounded using a specified rounding format. The
// limbs should not all be zero.
//
// # Worst-case complexity
// $T(n) = O(n)$
//
// $M(m) = O(m)$
//
// where $T$ is time, $M$ is additional memory, $n$ is `xs.len()`, and $m$ is `max(1, xs.len() -
// bits / Limb::WIDTH)`.
pub_test! {
    limbs_shr_round(xs: &[Limb], bits: u64, rm: RoundingMode) -> Option<(Vec<Limb>, Ordering)> {
    match rm {
        Down | Floor => Some((
            limbs_shr(xs, bits),
            if limbs_divisible_by_power_of_2(xs, bits) {
                Equal
            } else {
                Less
            },
        )),
        Up | Ceiling => Some(limbs_shr_round_up(xs, bits)),
        Nearest => Some(limbs_shr_round_nearest(xs, bits)),
        Exact => limbs_shr_exact(xs, bits).map(|ss| (ss, Equal)),
    }
}}

// Interpreting a `Vec` of `Limb`s as the limbs (in ascending order) of a `Natural`, writes the
// limbs of the `Natural` right-shifted by a `Limb`, rounding up, to the input `Vec`. The limbs
// should not all be zero.
//
// # Worst-case complexity
// $T(n) = O(n)$
//
// $M(n) = O(1)$
//
// where $T$ is time, $M$ is additional memory and $n$ is `max(1, xs.len() - bits / Limb::WIDTH)`.
//
// This is equivalent to `cfdiv_q_2exp` from `mpz/cfdiv_q_2exp.c`, GMP 6.2.1, where `u` is
// non-negative, `dir == 1`, and `w == u`.
pub_test! {limbs_vec_shr_round_up_in_place(xs: &mut Vec<Limb>, bits: u64) -> Ordering {
    let delete_count = usize::exact_from(bits >> Limb::LOG_WIDTH);
    if delete_count >= xs.len() {
        xs.truncate(1);
        xs[0] = 1;
        Greater
    } else {
        let mut exact = slice_test_zero(&xs[..delete_count]);
        let small_bits = bits & Limb::WIDTH_MASK;
        vec_delete_left(xs, delete_count);
        if small_bits != 0 {
            exact &= limbs_slice_shr_in_place(xs, small_bits) == 0;
        }
        if !exact {
            limbs_vec_add_limb_in_place(xs, 1);
        }
        if exact {
            Equal
        } else {
            Greater
        }
    }
}}

// # Worst-case complexity
// $T(n) = O(n)$
//
// $M(n) = O(1)$
//
// where $T$ is time, $M$ is additional memory and $n$ is `max(1, xs.len() - bits / Limb::WIDTH)`.
fn limbs_vec_shr_round_half_integer_to_even_in_place(xs: &mut Vec<Limb>, bits: u64) -> Ordering {
    let delete_count = usize::exact_from(bits >> Limb::LOG_WIDTH);
    if delete_count >= xs.len() {
        let o = if slice_test_zero(xs) { Equal } else { Less };
        xs.clear();
        o
    } else {
        let small_bits = bits & Limb::WIDTH_MASK;
        let mut exact = slice_test_zero(&xs[..delete_count]);
        vec_delete_left(xs, delete_count);
        if small_bits != 0 {
            exact &= limbs_slice_shr_in_place(xs, small_bits) == 0;
        }
        if !xs.is_empty() && xs[0].odd() {
            limbs_vec_add_limb_in_place(xs, 1);
            Greater
        } else if exact {
            Equal
        } else {
            Less
        }
    }
}

// Interpreting a `Vec` of `Limb`s as the limbs (in ascending order) of a `Natural`, writes the
// limbs of the `Natural` right-shifted by a `Limb` to the input `Vec`, rounding to the `Natural`
// nearest to the actual value of `self` divided by `2 ^ bits`. If the actual value is exactly
// between two integers, it is rounded to the even one.
//
// # Worst-case complexity
// $T(n) = O(n)$
//
// $M(n) = O(1)$
//
// where $T$ is time, $M$ is additional memory and $n$ is `xs.len()`.
pub_test! {limbs_vec_shr_round_nearest_in_place(xs: &mut Vec<Limb>, bits: u64) -> Ordering {
    if bits == 0 {
        Equal
    } else {
        let d = slice_test_zero(xs) || limbs_divisible_by_power_of_2(xs, bits - 1);
        if !limbs_get_bit(xs, bits - 1) {
            limbs_vec_shr_in_place(xs, bits);
            if d {
                Equal
            } else {
                Less
            }
        } else if d {
            limbs_vec_shr_round_half_integer_to_even_in_place(xs, bits)
        } else {
            limbs_vec_shr_round_up_in_place(xs, bits)
        }
    }
}}

// Interpreting a `Vec` of `Limb`s as the limbs (in ascending order) of a `Natural`, writes the
// limbs of the `Natural` right-shifted by a `Limb` to the input `Vec`, if the shift is exact
// (doesn't remove any `true` bits). Returns whether the shift was exact. The limbs should not all
// be zero.
//
// # Worst-case complexity
// $T(n) = O(n)$
//
// $M(n) = O(1)$
//
// where $T$ is time, $M$ is additional memory and $n$ is `xs.len()`.
pub_test! {limbs_vec_shr_exact_in_place(xs: &mut Vec<Limb>, bits: u64) -> bool {
    if limbs_divisible_by_power_of_2(xs, bits) {
        limbs_vec_shr_in_place(xs, bits);
        true
    } else {
        false
    }
}}

// Interpreting a slice of `Limb`s as the limbs (in ascending order) of a `Natural`, writes the
// limbs of the `Natural` right-shifted by a `Limb` to the input `Vec`, rounded using a specified
// rounding format. If the shift is inexact (removes some `true` bits) and the `RoundingMode` is
// `Exact`, the value of `xs` becomes unspecified and `false` is returned. Otherwise, `true` is
// returned. The limbs should not all be zero.
//
// # Worst-case complexity
// $T(n) = O(n)$
//
// $M(n) = O(1)$
//
// where $T$ is time, $M$ is additional memory and $n$ is `xs.len()`.
pub_test! {limbs_vec_shr_round_in_place(
    xs: &mut Vec<Limb>,
    bits: u64,
    rm: RoundingMode,
) -> (bool, Ordering) {
    match rm {
        Down | Floor => {
            let exact = limbs_divisible_by_power_of_2(xs, bits);
            limbs_vec_shr_in_place(xs, bits);
            (
                true,
                if exact {
                    Equal
                } else {
                    Less
                },
            )
        }
        Up | Ceiling => {
            (true, limbs_vec_shr_round_up_in_place(xs, bits))
        }
        Nearest => (true, limbs_vec_shr_round_nearest_in_place(xs, bits)),
        Exact => (limbs_vec_shr_exact_in_place(xs, bits), Equal),
    }
}}

fn shr_round_unsigned_ref_n<T: PrimitiveUnsigned>(
    x: &Natural,
    bits: T,
    rm: RoundingMode,
) -> (Natural, Ordering)
where
    u64: ExactFrom<T>,
    Limb: ShrRound<T, Output = Limb>,
{
    match (x, bits) {
        (&Natural::ZERO, _) => (x.clone(), Equal),
        (_, bits) if bits == T::ZERO => (x.clone(), Equal),
        (Natural(Small(ref small)), bits) => {
            let (s, o) = small.shr_round(bits, rm);
            (Natural(Small(s)), o)
        }
        (Natural(Large(ref limbs)), bits) => {
            if let Some((out, o)) = limbs_shr_round(limbs, u64::exact_from(bits), rm) {
                (Natural::from_owned_limbs_asc(out), o)
            } else {
                panic!("Right shift is not exact: {x} >> {bits}");
            }
        }
    }
}

fn shr_round_assign_unsigned_n<T: PrimitiveUnsigned>(
    x: &mut Natural,
    bits: T,
    rm: RoundingMode,
) -> Ordering
where
    u64: ExactFrom<T>,
    Limb: ShrRoundAssign<T>,
{
    match (&mut *x, bits) {
        (&mut Natural::ZERO, _) => Equal,
        (_, bits) if bits == T::ZERO => Equal,
        (Natural(Small(ref mut small)), bits) => small.shr_round_assign(bits, rm),
        (Natural(Large(ref mut limbs)), bits) => {
            let (b, o) = limbs_vec_shr_round_in_place(limbs, u64::exact_from(bits), rm);
            assert!(b, "Right shift is not exact.");
            x.trim();
            o
        }
    }
}

macro_rules! impl_natural_shr_round_unsigned {
    ($t:ident) => {
        impl ShrRound<$t> for Natural {
            type Output = Natural;

            /// Shifts a [`Natural`] right (divides it by a power of 2), taking it by value, and
            /// rounds according to the specified rounding mode. An [`Ordering`] is also returned,
            /// indicating whether the returned value is less than, equal to, or greater than the
            /// exact value.
            ///
            /// Passing `Floor` or `Down` is equivalent to using `>>`. To test whether `Exact` can
            /// be passed, use `self.divisible_by_power_of_2(bits)`.
            ///
            /// Let $q = \frac{x}{2^k}$, and let $g$ be the function that just returns the first
            /// element of the pair, without the [`Ordering`]:
            ///
            /// $f(x, k, \mathrm{Down}) = f(x, y, \mathrm{Floor}) = \lfloor q \rfloor.$
            ///
            /// $f(x, k, \mathrm{Up}) = f(x, y, \mathrm{Ceiling}) = \lceil q \rceil.$
            ///
            /// $$
            /// f(x, k, \mathrm{Nearest}) = \begin{cases}
            ///     \lfloor q \rfloor & \text{if}
            ///         \\quad q - \lfloor q \rfloor < \frac{1}{2}, \\\\
            ///     \lceil q \rceil & \text{if}
            ///         \\quad q - \lfloor q \rfloor > \frac{1}{2}, \\\\
            ///     \lfloor q \rfloor & \text{if} \\quad q - \lfloor q \rfloor =
            ///         \frac{1}{2} \\ \text{and} \\ \lfloor q \rfloor
            ///     \\ \text{is even}, \\\\
            ///     \lceil q \rceil &
            ///     \text{if} \\quad q - \lfloor q \rfloor = \frac{1}{2} \\ \text{and}
            ///         \\ \lfloor q \rfloor \\ \text{is odd}.
            /// \end{cases}
            /// $$
            ///
            /// $f(x, k, \mathrm{Exact}) = q$, but panics if $q \notin \N$.
            ///
            /// Then
            ///
            /// $f(x, k, r) = (g(x, k, r), \operatorname{cmp}(g(x, k, r), q))$.
            ///
            /// # Worst-case complexity
            /// $T(n) = O(n)$
            ///
            /// $M(n) = O(1)$
            ///
            /// where $T$ is time, $M$ is additional memory and $n$ is `self.significant_bits()`.
            ///
            /// # Panics
            /// Let $k$ be `bits`. Panics if `rm` is `Exact` but `self` is not divisible by $2^k$.
            ///
            /// # Examples
            /// See [here](super::shr_round#shr_round).
            #[inline]
            fn shr_round(mut self, bits: $t, rm: RoundingMode) -> (Natural, Ordering) {
                let o = self.shr_round_assign(bits, rm);
                (self, o)
            }
        }

        impl<'a> ShrRound<$t> for &'a Natural {
            type Output = Natural;

            /// Shifts a [`Natural`] right (divides it by a power of 2), taking it by reference, and
            /// rounds according to the specified rounding mode. An [`Ordering`] is also returned,
            /// indicating whether the returned value is less than, equal to, or greater than the
            /// exact value.
            ///
            /// Passing `Floor` or `Down` is equivalent to using `>>`. To test whether `Exact` can
            /// be passed, use `self.divisible_by_power_of_2(bits)`.
            ///
            /// Let $q = \frac{x}{2^k}$, and let $g$ be the function that just returns the first
            /// element of the pair, without the [`Ordering`]:
            ///
            /// $f(x, k, \mathrm{Down}) = f(x, y, \mathrm{Floor}) = \lfloor q \rfloor.$
            ///
            /// $f(x, k, \mathrm{Up}) = f(x, y, \mathrm{Ceiling}) = \lceil q \rceil.$
            ///
            /// $$
            /// f(x, k, \mathrm{Nearest}) = \begin{cases}
            ///     \lfloor q \rfloor & \text{if}
            ///         \\quad q - \lfloor q \rfloor < \frac{1}{2}, \\\\
            ///     \lceil q \rceil & \text{if}
            ///         \\quad q - \lfloor q \rfloor > \frac{1}{2}, \\\\
            ///     \lfloor q \rfloor & \text{if} \\quad q - \lfloor q \rfloor =
            ///         \frac{1}{2} \\ \text{and} \\ \lfloor q \rfloor
            ///     \\ \text{is even}, \\\\
            ///     \lceil q \rceil &
            ///     \text{if} \\quad q - \lfloor q \rfloor = \frac{1}{2} \\ \text{and}
            ///         \\ \lfloor q \rfloor \\ \text{is odd}.
            /// \end{cases}
            /// $$
            ///
            /// $f(x, k, \mathrm{Exact}) = q$, but panics if $q \notin \N$.
            ///
            /// Then
            ///
            /// $f(x, k, r) = (g(x, k, r), \operatorname{cmp}(g(x, k, r), q))$.
            ///
            /// # Worst-case complexity
            /// $T(n) = O(n)$
            ///
            /// $M(m) = O(m)$
            ///
            /// where $T$ is time, $M$ is additional memory, $n$ is `self.significant_bits()`, and
            /// $m$ is `max(1, self.significant_bits() - bits)`.
            ///
            /// # Panics
            /// Let $k$ be `bits`. Panics if `rm` is `Exact` but `self` is not divisible by $2^k$.
            ///
            /// # Examples
            /// See [here](super::shr_round#shr_round).
            #[inline]
            fn shr_round(self, bits: $t, rm: RoundingMode) -> (Natural, Ordering) {
                shr_round_unsigned_ref_n(self, bits, rm)
            }
        }

        impl ShrRoundAssign<$t> for Natural {
            /// Shifts a [`Natural`] right (divides it by a power of 2) and rounds according to the
            /// specified rounding mode, in place. An [`Ordering`] is returned, indicating whether
            /// the assigned value is less than, equal to, or greater than the exact value.
            ///
            /// Passing `Floor` or `Down` is equivalent to using `>>=`. To test whether `Exact` can
            /// be passed, use `self.divisible_by_power_of_2(bits)`.
            ///
            /// See the [`ShrRound`] documentation for details.
            ///
            /// # Worst-case complexity
            /// $T(n) = O(n)$
            ///
            /// $M(n) = O(1)$
            ///
            /// where $T$ is time, $M$ is additional memory, and $n$ is `self.significant_bits()`.
            ///
            /// # Panics
            /// Let $k$ be `bits`. Panics if `rm` is `Exact` but `self` is not divisible by $2^k$.
            ///
            /// # Examples
            /// See [here](super::shr_round#shr_round_assign).
            #[inline]
            fn shr_round_assign(&mut self, bits: $t, rm: RoundingMode) -> Ordering {
                shr_round_assign_unsigned_n(self, bits, rm)
            }
        }
    };
}
apply_to_unsigneds!(impl_natural_shr_round_unsigned);

fn shr_round_signed_ref_n<'a, U, S: PrimitiveSigned + UnsignedAbs<Output = U>>(
    x: &'a Natural,
    bits: S,
    rm: RoundingMode,
) -> (Natural, Ordering)
where
    &'a Natural: Shl<U, Output = Natural> + ShrRound<U, Output = Natural>,
{
    if bits >= S::ZERO {
        x.shr_round(bits.unsigned_abs(), rm)
    } else {
        (x << bits.unsigned_abs(), Equal)
    }
}

fn shr_round_assign_signed_n<U, S: PrimitiveSigned + UnsignedAbs<Output = U>>(
    x: &mut Natural,
    bits: S,
    rm: RoundingMode,
) -> Ordering
where
    Natural: ShlAssign<U> + ShrRoundAssign<U>,
{
    if bits >= S::ZERO {
        x.shr_round_assign(bits.unsigned_abs(), rm)
    } else {
        *x <<= bits.unsigned_abs();
        Equal
    }
}

macro_rules! impl_natural_shr_round_signed {
    ($t:ident) => {
        impl ShrRound<$t> for Natural {
            type Output = Natural;

            /// Shifts a [`Natural`] right (divides or multiplies it by a power of 2), taking it by
            /// value, and rounds according to the specified rounding mode. An [`Ordering`] is also
            /// returned, indicating whether the returned value is less than, equal to, or greater
            /// than the exact value. If `bits` is negative, then the returned [`Ordering`] is
            /// always `Equal`, even if the higher bits of the result are lost.
            ///
            /// Passing `Floor` or `Down` is equivalent to using `>>`. To test whether `Exact` can
            /// be passed, use `self.divisible_by_power_of_2(bits)`.
            ///
            /// Let $q = \frac{x}{2^k}$, and let $g$ be the function that just returns the first
            /// element of the pair, without the [`Ordering`]:
            ///
            /// $f(x, k, \mathrm{Down}) = f(x, y, \mathrm{Floor}) = \lfloor q \rfloor.$
            ///
            /// $f(x, k, \mathrm{Up}) = f(x, y, \mathrm{Ceiling}) = \lceil q \rceil.$
            ///
            /// $$
            /// f(x, k, \mathrm{Nearest}) = \begin{cases}
            ///     \lfloor q \rfloor & \text{if}
            ///         \\quad q - \lfloor q \rfloor < \frac{1}{2}, \\\\
            ///     \lceil q \rceil & \text{if}
            ///         \\quad q - \lfloor q \rfloor > \frac{1}{2}, \\\\
            ///     \lfloor q \rfloor & \text{if} \\quad q - \lfloor q \rfloor =
            ///         \frac{1}{2} \\ \text{and} \\ \lfloor q \rfloor
            ///     \\ \text{is even}, \\\\
            ///     \lceil q \rceil &
            ///     \text{if} \\quad q - \lfloor q \rfloor = \frac{1}{2} \\ \text{and}
            ///         \\ \lfloor q \rfloor \\ \text{is odd}.
            /// \end{cases}
            /// $$
            ///
            /// $f(x, k, \mathrm{Exact}) = q$, but panics if $q \notin \N$.
            ///
            /// Then
            ///
            /// $f(x, k, r) = (g(x, k, r), \operatorname{cmp}(g(x, k, r), q))$.
            ///
            /// # Worst-case complexity
            /// $T(n, m) = O(n + m)$
            ///
            /// $M(n, m) = O(n + m)$
            ///
            /// where $T$ is time, $M$ is additional memory, $n$ is `self.significant_bits()`, and
            /// $m$ is `max(-bits, 0)`.
            ///
            /// # Panics
            /// Let $k$ be `bits`. Panics if $k$ is positive and `rm` is `Exact` but `self` is not
            /// divisible by $2^k$.
            ///
            /// # Examples
            /// See [here](super::shr_round#shr_round).
            #[inline]
            fn shr_round(mut self, bits: $t, rm: RoundingMode) -> (Natural, Ordering) {
                let o = self.shr_round_assign(bits, rm);
                (self, o)
            }
        }

        impl<'a> ShrRound<$t> for &'a Natural {
            type Output = Natural;

            /// Shifts a [`Natural`] right (divides or multiplies it by a power of 2), taking it by
            /// reference, and rounds according to the specified rounding mode. An [`Ordering`] is
            /// also returned, indicating whether the returned value is less than, equal to, or
            /// greater than the exact value. If `bits` is negative, then the returned [`Ordering`]
            /// is always `Equal`, even if the higher bits of the result are lost.
            ///
            /// Passing `Floor` or `Down` is equivalent to using `>>`. To test whether `Exact` can
            /// be passed, use `self.divisible_by_power_of_2(bits)`.
            ///
            /// Let $q = \frac{x}{2^k}$, and let $g$ be the function that just returns the first
            /// element of the pair, without the [`Ordering`]:
            ///
            /// $f(x, k, \mathrm{Down}) = f(x, y, \mathrm{Floor}) = \lfloor q \rfloor.$
            ///
            /// $f(x, k, \mathrm{Up}) = f(x, y, \mathrm{Ceiling}) = \lceil q \rceil.$
            ///
            /// $$
            /// f(x, k, \mathrm{Nearest}) = \begin{cases}
            ///     \lfloor q \rfloor & \text{if}
            ///         \\quad q - \lfloor q \rfloor < \frac{1}{2}, \\\\
            ///     \lceil q \rceil & \text{if}
            ///         \\quad q - \lfloor q \rfloor > \frac{1}{2}, \\\\
            ///     \lfloor q \rfloor & \text{if} \\quad q - \lfloor q \rfloor =
            ///         \frac{1}{2} \\ \text{and} \\ \lfloor q \rfloor
            ///     \\ \text{is even}, \\\\
            ///     \lceil q \rceil &
            ///     \text{if} \\quad q - \lfloor q \rfloor = \frac{1}{2} \\ \text{and}
            ///         \\ \lfloor q \rfloor \\ \text{is odd}.
            /// \end{cases}
            /// $$
            ///
            /// $f(x, k, \mathrm{Exact}) = q$, but panics if $q \notin \N$.
            ///
            /// Then
            ///
            /// $f(x, k, r) = (g(x, k, r), \operatorname{cmp}(g(x, k, r), q))$.
            ///
            /// # Worst-case complexity
            /// $T(n, m) = O(n + m)$
            ///
            /// $M(n, m) = O(n + m)$
            ///
            /// where $T$ is time, $M$ is additional memory, $n$ is `self.significant_bits()`, and
            /// $m$ is `max(-bits, 0)`.
            ///
            /// # Worst-case complexity
            /// $T(n, m) = O(n + m)$
            ///
            /// $M(n, m) = O(n + m)$
            ///
            /// where $T$ is time, $M$ is additional memory, $n$ is `self.significant_bits()`, and
            /// $m$ is `max(-bits, 0)`.
            ///
            /// # Panics
            /// Let $k$ be `bits`. Panics if $k$ is positive and `rm` is `Exact` but `self` is not
            /// divisible by $2^k$.
            ///
            /// # Examples
            /// See [here](super::shr_round#shr_round).
            #[inline]
            fn shr_round(self, bits: $t, rm: RoundingMode) -> (Natural, Ordering) {
                shr_round_signed_ref_n(self, bits, rm)
            }
        }

        impl ShrRoundAssign<$t> for Natural {
            /// Shifts a [`Natural`] right (divides or multiplies it by a power of 2) and rounds
            /// according to the specified rounding mode, in place. An [`Ordering`] is returned,
            /// indicating whether the assigned value is less than, equal to, or greater than the
            /// exact value. If `bits` is negative, then the returned [`Ordering`] is always
            /// `Equal`, even if the higher bits of the result are lost.
            ///
            /// Passing `Floor` or `Down` is equivalent to using `>>`. To test whether `Exact` can
            /// be passed, use `self.divisible_by_power_of_2(bits)`.
            ///
            /// See the [`ShrRound`] documentation for details.
            ///
            /// # Worst-case complexity
            /// $T(n, m) = O(n + m)$
            ///
            /// $M(n, m) = O(n + m)$
            ///
            /// where $T$ is time, $M$ is additional memory, $n$ is `self.significant_bits()`, and
            /// $m$ is `max(-bits, 0)`.
            ///
            /// # Panics
            /// Let $k$ be `bits`. Panics if $k$ is positive and `rm` is `Exact` but `self` is not
            /// divisible by $2^k$.
            ///
            /// # Examples
            /// See [here](super::shr_round#shr_round_assign).
            #[inline]
            fn shr_round_assign(&mut self, bits: $t, rm: RoundingMode) -> Ordering {
                shr_round_assign_signed_n(self, bits, rm)
            }
        }
    };
}
apply_to_signeds!(impl_natural_shr_round_signed);
