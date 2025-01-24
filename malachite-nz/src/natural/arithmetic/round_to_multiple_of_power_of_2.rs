// Copyright © 2025 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::natural::arithmetic::add::limbs_slice_add_limb_in_place;
use crate::natural::arithmetic::divisible_by_power_of_2::limbs_divisible_by_power_of_2;
use crate::natural::logic::bit_access::limbs_get_bit;
use crate::natural::InnerNatural::{Large, Small};
use crate::natural::Natural;
use crate::platform::Limb;
use alloc::vec::Vec;
use core::cmp::Ordering::{self, *};
use malachite_base::num::arithmetic::traits::{
    ModPowerOf2, PowerOf2, RoundToMultipleOfPowerOf2, RoundToMultipleOfPowerOf2Assign, ShrRound,
    ShrRoundAssign,
};
use malachite_base::num::basic::integers::PrimitiveInt;
use malachite_base::num::basic::traits::Zero;
use malachite_base::num::conversion::traits::ExactFrom;
use malachite_base::num::logic::traits::{BitAccess, LowMask};
use malachite_base::rounding_modes::RoundingMode::{self, *};
use malachite_base::slices::{slice_set_zero, slice_test_zero};

// Interpreting a slice of `Limb`s as the limbs (in ascending order) of a `Natural`, returns the
// limbs of the `Natural` rounded down to a multiple of `2 ^ pow`.
//
// # Worst-case complexity
// $T(n) = O(n)$
//
// $M(n) = O(n)$
//
// where $T$ is time, $M$ is additional memory, and $n$ is `xs.len()`.
pub_test! {
    limbs_round_to_multiple_of_power_of_2_down(xs: &[Limb], pow: u64) -> (Vec<Limb>, Ordering) {
    let clear_count = usize::exact_from(pow >> Limb::LOG_WIDTH);
    let xs_len = xs.len();
    if clear_count >= xs_len {
        (Vec::new(), if slice_test_zero(xs) {Equal} else {Less})
    } else {
        let mut out = vec![0; xs_len];
        let (xs_lo, xs_hi) = xs.split_at(clear_count);
        let mut exact = slice_test_zero(xs_lo);
        out[clear_count..].copy_from_slice(xs_hi);
        let small_pow = pow & Limb::WIDTH_MASK;
        if small_pow != 0 {
            let out_cc = &mut out[clear_count];
            let old = *out_cc;
            *out_cc &= !Limb::low_mask(small_pow);
            exact &= *out_cc == old;
        }
        (out, if exact {Equal} else {Less})
    }
}}

// Interpreting a slice of `Limb`s as the limbs (in ascending order) of a `Natural`, returns the
// limbs of the `Natural` rounded up to a multiple of `2 ^ pow`. The limbs should not all be zero.
//
// # Worst-case complexity
// $T(n) = O(n)$
//
// $M(n) = O(n)$
//
// where $T$ is time, $M$ is additional memory, and $n$ is `max(xs.len(), pow / Limb::WIDTH)`.
pub_test! {
    limbs_round_to_multiple_of_power_of_2_up(xs: &[Limb], pow: u64) -> (Vec<Limb>, Ordering) {
    let clear_count = usize::exact_from(pow >> Limb::LOG_WIDTH);
    let xs_len = xs.len();
    let mut out;
    let small_pow = pow & Limb::WIDTH_MASK;
    let mut exact;
    if clear_count >= xs_len {
        out = vec![0; clear_count + 1];
        out[clear_count] = Limb::power_of_2(small_pow);
        exact = false;
    } else {
        let (xs_lo, xs_hi) = xs.split_at(clear_count);
        exact = slice_test_zero(xs_lo);
        out = vec![0; xs_len];
        let out_hi = &mut out[clear_count..];
        out_hi.copy_from_slice(xs_hi);
        if small_pow != 0 {
            let remainder = out_hi[0].mod_power_of_2(small_pow);
            if remainder != 0 {
                out_hi[0] -= remainder;
                exact = false;
            }
        }
        if !exact && limbs_slice_add_limb_in_place(out_hi, Limb::power_of_2(small_pow)) {
            out.push(1);
        }
    }
    (out, if exact {Equal} else {Greater})
}}

// # Worst-case complexity
// $T(n) = O(n)$
//
// $M(n) = O(n)$
//
// where $T$ is time, $M$ is additional memory, and $n$ is `max(xs.len(), pow / Limb::WIDTH)`.s
fn limbs_round_to_multiple_of_power_of_2_half_integer_to_even(
    xs: &[Limb],
    pow: u64,
) -> (Vec<Limb>, Ordering) {
    let clear_count = usize::exact_from(pow >> Limb::LOG_WIDTH);
    let xs_len = xs.len();
    if clear_count >= xs_len {
        (Vec::new(), if slice_test_zero(xs) { Equal } else { Less })
    } else {
        let (xs_lo, xs_hi) = xs.split_at(clear_count);
        let mut exact = slice_test_zero(xs_lo);
        let mut out = vec![0; xs_len];
        let out_hi = &mut out[clear_count..];
        out_hi.copy_from_slice(xs_hi);
        let small_pow = pow & Limb::WIDTH_MASK;
        if small_pow != 0 {
            out_hi[0] &= !Limb::low_mask(small_pow);
            exact = false;
        }
        if xs_hi[0].get_bit(small_pow) {
            if limbs_slice_add_limb_in_place(out_hi, Limb::power_of_2(small_pow)) {
                out.push(1);
            }
            (out, Greater)
        } else {
            (out, if exact { Equal } else { Less })
        }
    }
}

// Interpreting a slice of `Limb`s as the limbs (in ascending order) of a `Natural`, returns the
// limbs of the `Natural` rounded to the nearest multiple of `2 ^ pow`. If the original value is
// exactly between two multiples, it is rounded to the one whose `pow`th bit is zero.
//
// # Worst-case complexity
// $T(n) = O(n)$
//
// $M(n) = O(n)$
//
// where $T$ is time, $M$ is additional memory, and $n$ is `max(xs.len(), pow / Limb::WIDTH)`.
pub_test! {
    limbs_round_to_multiple_of_power_of_2_nearest(xs: &[Limb], pow: u64) -> (Vec<Limb>, Ordering) {
    if pow == 0 {
        (xs.to_vec(), Equal)
    } else if !limbs_get_bit(xs, pow - 1) {
        limbs_round_to_multiple_of_power_of_2_down(xs, pow)
    } else if !limbs_divisible_by_power_of_2(xs, pow - 1) {
        limbs_round_to_multiple_of_power_of_2_up(xs, pow)
    } else {
        limbs_round_to_multiple_of_power_of_2_half_integer_to_even(xs, pow)
    }
}}

// Interpreting a slice of `Limb`s as the limbs (in ascending order) of a `Natural`, returns the
// limbs of the `Natural` rounded to a multiple of `2 ^ pow`, using a specified rounding format. If
// the original value is not already a multiple of the power of 2, and the `RoundingMode` is
// `Exact`, `None` is returned. The limbs should not all be zero.
//
// # Worst-case complexity
// $T(n) = O(n)$
//
// $M(n) = O(n)$
//
// where $T$ is time, $M$ is additional memory, and $n$ is `max(xs.len(), pow / Limb::WIDTH)`.
pub_test! {limbs_round_to_multiple_of_power_of_2(
    xs: &[Limb],
    pow: u64,
    rm: RoundingMode,
) -> Option<(Vec<Limb>, Ordering)> {
    match rm {
        Down | Floor => {
            Some(limbs_round_to_multiple_of_power_of_2_down(xs, pow))
        }
        Up | Ceiling => {
            Some(limbs_round_to_multiple_of_power_of_2_up(xs, pow))
        }
        Nearest => Some(limbs_round_to_multiple_of_power_of_2_nearest(xs, pow)),
        Exact => {
            if limbs_divisible_by_power_of_2(xs, pow) {
                Some((xs.to_vec(), Equal))
            } else {
                None
            }
        }
    }
}}

// Interpreting a `Vec` of `Limb`s as the limbs (in ascending order) of a `Natural`, writes the
// limbs of the `Natural`, rounded down to a multiple of `2 ^ pow`, to the input `Vec`.
//
// # Worst-case complexity
// $T(n) = O(n)$
//
// $M(n) = O(1)$
//
// where $T$ is time, $M$ is additional memory, and $n$ is `xs.len()`.
pub_test! {
    limbs_round_to_multiple_of_power_of_2_down_in_place(xs: &mut Vec<Limb>, pow: u64) -> Ordering {
    let clear_count = usize::exact_from(pow >> Limb::LOG_WIDTH);
    let xs_len = xs.len();
    let mut exact;
    if clear_count >= xs_len {
        exact = slice_test_zero(xs);
        xs.clear();
    } else {
        let (xs_lo, xs_hi) = xs.split_at_mut(clear_count);
        exact = slice_test_zero(xs_lo);
        slice_set_zero(xs_lo);
        let small_pow = pow & Limb::WIDTH_MASK;
        if small_pow != 0 {
            let x0 = &mut xs_hi[0];
            let old = *x0;
            *x0 &= !Limb::low_mask(small_pow);
            exact &= *x0 == old;
        }
    }
    if exact {Equal} else {Less}
}}

// Interpreting a `Vec` of `Limb`s as the limbs (in ascending order) of a `Natural`, writes the
// limbs of the `Natural`, rounded up to a multiple of `2 ^ pow`, to the input `Vec`. The limbs
// should not all be zero.
//
// # Worst-case complexity
// $T(n) = O(n)$
//
// $M(n) = O(n)$
//
// where $T$ is time, $M$ is additional memory, and $n$ is `max(xs.len(), pow / Limb::WIDTH)`.
pub_test! {
    limbs_round_to_multiple_of_power_of_2_up_in_place(xs: &mut Vec<Limb>, pow: u64) -> Ordering {
    let clear_count = usize::exact_from(pow >> Limb::LOG_WIDTH);
    let xs_len = xs.len();
    let small_pow = pow & Limb::WIDTH_MASK;
    if clear_count >= xs_len {
        *xs = vec![0; clear_count + 1];
        xs[clear_count] = Limb::power_of_2(small_pow);
        Greater
    } else {
        let (xs_lo, xs_hi) = xs.split_at_mut(clear_count);
        let mut exact = slice_test_zero(xs_lo);
        slice_set_zero(xs_lo);
        if small_pow != 0 {
            let remainder = xs_hi[0].mod_power_of_2(small_pow);
            if remainder != 0 {
                xs_hi[0] -= remainder;
                exact = false;
            }
        }
        if !exact && limbs_slice_add_limb_in_place(xs_hi, Limb::power_of_2(small_pow)) {
            xs.push(1);
        }
        if exact {Equal} else {Greater}
    }
}}

// # Worst-case complexity
// $T(n) = O(n)$
//
// $M(n) = O(n)$
//
// where $T$ is time, $M$ is additional memory, and $n$ is `max(xs.len(), pow / Limb::WIDTH)`.
fn limbs_round_to_multiple_of_power_of_2_half_integer_to_even_in_place(
    xs: &mut Vec<Limb>,
    pow: u64,
) -> Ordering {
    let clear_count = usize::exact_from(pow >> Limb::LOG_WIDTH);
    let xs_len = xs.len();
    if clear_count >= xs_len {
        let exact = slice_test_zero(xs);
        xs.clear();
        if exact {
            Equal
        } else {
            Less
        }
    } else {
        let (xs_lo, xs_hi) = xs.split_at_mut(clear_count);
        let mut exact = true;
        if let Some(last) = xs_lo.last_mut() {
            if *last != 0 {
                exact = false;
            }
            *last = 0;
        }
        let small_pow = pow & Limb::WIDTH_MASK;
        if small_pow != 0 {
            xs_hi[0] &= !Limb::low_mask(small_pow);
            exact = false;
        }
        if xs_hi[0].get_bit(small_pow) {
            if limbs_slice_add_limb_in_place(xs_hi, Limb::power_of_2(small_pow)) {
                xs.push(1);
            }
            Greater
        } else if exact {
            Equal
        } else {
            Less
        }
    }
}

// Interpreting a `Vec` of `Limb`s as the limbs (in ascending order) of a `Natural`, writes the
// limbs of the `Natural`, rounded to the nearest multiple of `2 ^ pow`, to the input `Vec`. If the
// original value is exactly between two multiples, it is rounded to the one whose `pow`th bit is
// zero.
//
// # Worst-case complexity
// $T(n) = O(n)$
//
// $M(n) = O(n)$
//
// where $T$ is time, $M$ is additional memory, and $n$ is `max(xs.len(), pow / Limb::WIDTH)`.
pub_test! {limbs_round_to_multiple_of_power_of_2_nearest_in_place(
    xs: &mut Vec<Limb>,
    pow: u64
) -> Ordering {
    if pow == 0 {
        Equal
    } else if !limbs_get_bit(xs, pow - 1) {
        limbs_round_to_multiple_of_power_of_2_down_in_place(xs, pow)
    } else if !limbs_divisible_by_power_of_2(xs, pow - 1) {
        limbs_round_to_multiple_of_power_of_2_up_in_place(xs, pow)
    } else {
        limbs_round_to_multiple_of_power_of_2_half_integer_to_even_in_place(xs, pow)
    }
}}

// Interpreting a slice of `Limb`s as the limbs (in ascending order) of a `Natural`, writes the
// limbs of the `Natural` rounded to the nearest multiple of `2 ^ pow` to the input `Vec`, using a
// specified rounding format. If the original value is not already a multiple of the power of two,
// and the `RoundingMode` is `Exact`, the value of `xs` becomes unspecified and `None` is returned.
// Otherwise, an `Ordering` is returned. The limbs should not all be zero.
//
// # Worst-case complexity
// $T(n) = O(n)$
//
// $M(n) = O(n)$
//
// where $T$ is time, $M$ is additional memory, and $n$ is `max(xs.len(), pow / Limb::WIDTH)`.
pub_test! {limbs_round_to_multiple_of_power_of_2_in_place(
    xs: &mut Vec<Limb>,
    pow: u64,
    rm: RoundingMode,
) -> Option<Ordering> {
    match rm {
        Down | Floor => {
            Some(limbs_round_to_multiple_of_power_of_2_down_in_place(xs, pow))
        }
        Up | Ceiling => {
            Some(limbs_round_to_multiple_of_power_of_2_up_in_place(xs, pow))
        }
        Nearest => Some(limbs_round_to_multiple_of_power_of_2_nearest_in_place(
            xs, pow,
        )),
        Exact => {
            if limbs_divisible_by_power_of_2(xs, pow) {
                Some(Equal)
            } else {
                None
            }
        }
    }
}}

impl RoundToMultipleOfPowerOf2<u64> for Natural {
    type Output = Natural;

    /// Rounds a [`Natural`] to a multiple of $2^k$ according to a specified rounding mode. The
    /// [`Natural`] is taken by value. An [`Ordering`] is also returned, indicating whether the
    /// returned value is less than, equal to, or greater than the original value.
    ///
    /// Let $q = \frac{x}{2^k}$:
    ///
    /// $f(x, k, \mathrm{Down}) = f(x, k, \mathrm{Floor}) = 2^k \lfloor q \rfloor.$
    ///
    /// $f(x, k, \mathrm{Up}) = f(x, k, \mathrm{Ceiling}) = 2^k \lceil q \rceil.$
    ///
    /// $$
    /// f(x, k, \mathrm{Nearest}) = \begin{cases}
    ///     2^k \lfloor q \rfloor & \text{if} \\quad
    ///     q - \lfloor q \rfloor < \frac{1}{2} \\\\
    ///     2^k \lceil q \rceil & \text{if} \\quad q - \lfloor q \rfloor > \frac{1}{2} \\\\
    ///     2^k \lfloor q \rfloor &
    ///     \text{if} \\quad q - \lfloor q \rfloor =
    ///         \frac{1}{2} \\ \text{and} \\ \lfloor q \rfloor
    ///     \\ \text{is even} \\\\
    ///     2^k \lceil q \rceil &
    ///     \text{if} \\quad q - \lfloor q \rfloor =
    ///         \frac{1}{2} \\ \text{and} \\ \lfloor q \rfloor \\ \text{is odd.}
    /// \end{cases}
    /// $$
    ///
    /// $f(x, k, \mathrm{Exact}) = 2^k q$, but panics if $q \notin \Z$.
    ///
    /// The following two expressions are equivalent:
    /// - `x.round_to_multiple_of_power_of_2(pow, Exact)`
    /// - `{ assert!(x.divisible_by_power_of_2(pow)); x }`
    ///
    /// but the latter should be used as it is clearer and more efficient.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(self.significant_bits(), pow /
    /// Limb::WIDTH)`.
    ///
    /// # Panics
    /// Panics if `rm` is `Exact`, but `self` is not a multiple of the power of 2.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::RoundToMultipleOfPowerOf2;
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_base::strings::ToDebugString;
    /// use malachite_nz::natural::Natural;
    ///
    /// assert_eq!(
    ///     Natural::from(10u32)
    ///         .round_to_multiple_of_power_of_2(2, Floor)
    ///         .to_debug_string(),
    ///     "(8, Less)"
    /// );
    /// assert_eq!(
    ///     Natural::from(10u32)
    ///         .round_to_multiple_of_power_of_2(2, Ceiling)
    ///         .to_debug_string(),
    ///     "(12, Greater)"
    /// );
    /// assert_eq!(
    ///     Natural::from(10u32)
    ///         .round_to_multiple_of_power_of_2(2, Down)
    ///         .to_debug_string(),
    ///     "(8, Less)"
    /// );
    /// assert_eq!(
    ///     Natural::from(10u32)
    ///         .round_to_multiple_of_power_of_2(2, Up)
    ///         .to_debug_string(),
    ///     "(12, Greater)"
    /// );
    /// assert_eq!(
    ///     Natural::from(10u32)
    ///         .round_to_multiple_of_power_of_2(2, Nearest)
    ///         .to_debug_string(),
    ///     "(8, Less)"
    /// );
    /// assert_eq!(
    ///     Natural::from(12u32)
    ///         .round_to_multiple_of_power_of_2(2, Exact)
    ///         .to_debug_string(),
    ///     "(12, Equal)"
    /// );
    /// ```
    #[inline]
    fn round_to_multiple_of_power_of_2(
        mut self,
        pow: u64,
        rm: RoundingMode,
    ) -> (Natural, Ordering) {
        let o = self.round_to_multiple_of_power_of_2_assign(pow, rm);
        (self, o)
    }
}

impl RoundToMultipleOfPowerOf2<u64> for &Natural {
    type Output = Natural;

    /// Rounds a [`Natural`] to a multiple of $2^k$ according to a specified rounding mode. The
    /// [`Natural`] is taken by reference. An [`Ordering`] is also returned, indicating whether the
    /// returned value is less than, equal to, or greater than the original value.
    ///
    /// Let $q = \frac{x}{2^k}$:
    ///
    /// $f(x, k, \mathrm{Down}) = f(x, k, \mathrm{Floor}) = 2^k \lfloor q \rfloor.$
    ///
    /// $f(x, k, \mathrm{Up}) = f(x, k, \mathrm{Ceiling}) = 2^k \lceil q \rceil.$
    ///
    /// $$
    /// f(x, k, \mathrm{Nearest}) = \begin{cases}
    ///     2^k \lfloor q \rfloor & \text{if} \\quad
    ///     q - \lfloor q \rfloor < \frac{1}{2} \\\\
    ///     2^k \lceil q \rceil & \text{if} \\quad q - \lfloor q \rfloor > \frac{1}{2} \\\\
    ///     2^k \lfloor q \rfloor &
    ///     \text{if} \\quad q - \lfloor q \rfloor =
    ///         \frac{1}{2} \\ \text{and} \\ \lfloor q \rfloor
    ///     \\ \text{is even} \\\\
    ///     2^k \lceil q \rceil &
    ///     \text{if} \\quad q - \lfloor q \rfloor =
    ///         \frac{1}{2} \\ \text{and} \\ \lfloor q \rfloor \\ \text{is odd.}
    /// \end{cases}
    /// $$
    ///
    /// $f(x, k, \mathrm{Exact}) = 2^k q$, but panics if $q \notin \Z$.
    ///
    /// The following two expressions are equivalent:
    /// - `x.round_to_multiple_of_power_of_2(pow, Exact)`
    /// - `{ assert!(x.divisible_by_power_of_2(pow)); x }`
    ///
    /// but the latter should be used as it is clearer and more efficient.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(self.significant_bits(), pow /
    /// Limb::WIDTH)`.
    ///
    /// # Panics
    /// Panics if `rm` is `Exact`, but `self` is not a multiple of the power of 2.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::RoundToMultipleOfPowerOf2;
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_base::strings::ToDebugString;
    /// use malachite_nz::natural::Natural;
    ///
    /// assert_eq!(
    ///     (&Natural::from(10u32))
    ///         .round_to_multiple_of_power_of_2(2, Floor)
    ///         .to_debug_string(),
    ///     "(8, Less)"
    /// );
    /// assert_eq!(
    ///     (&Natural::from(10u32))
    ///         .round_to_multiple_of_power_of_2(2, Ceiling)
    ///         .to_debug_string(),
    ///     "(12, Greater)"
    /// );
    /// assert_eq!(
    ///     (&Natural::from(10u32))
    ///         .round_to_multiple_of_power_of_2(2, Down)
    ///         .to_debug_string(),
    ///     "(8, Less)"
    /// );
    /// assert_eq!(
    ///     (&Natural::from(10u32))
    ///         .round_to_multiple_of_power_of_2(2, Up)
    ///         .to_debug_string(),
    ///     "(12, Greater)"
    /// );
    /// assert_eq!(
    ///     (&Natural::from(10u32))
    ///         .round_to_multiple_of_power_of_2(2, Nearest)
    ///         .to_debug_string(),
    ///     "(8, Less)"
    /// );
    /// assert_eq!(
    ///     (&Natural::from(12u32))
    ///         .round_to_multiple_of_power_of_2(2, Exact)
    ///         .to_debug_string(),
    ///     "(12, Equal)"
    /// );
    /// ```
    fn round_to_multiple_of_power_of_2(self, pow: u64, rm: RoundingMode) -> (Natural, Ordering) {
        match (self, pow) {
            (_, 0) | (&Natural::ZERO, _) => (self.clone(), Equal),
            (Natural(Small(small)), pow) => {
                let (s, o) = small.shr_round(pow, rm);
                (Natural::from(s) << pow, o)
            }
            (Natural(Large(ref limbs)), pow) => {
                if let Some((result_limbs, o)) =
                    limbs_round_to_multiple_of_power_of_2(limbs, pow, rm)
                {
                    (Natural::from_owned_limbs_asc(result_limbs), o)
                } else {
                    panic!("Rounding {self} to multiple of 2^{pow} is not exact");
                }
            }
        }
    }
}

impl RoundToMultipleOfPowerOf2Assign<u64> for Natural {
    /// Rounds a [`Natural`] to a multiple of $2^k$ in place, according to a specified rounding
    /// mode. An [`Ordering`] is returned, indicating whether the returned value is less than, equal
    /// to, or greater than the original value.
    ///
    /// See the [`RoundToMultipleOfPowerOf2`] documentation for details.
    ///
    /// The following two expressions are equivalent:
    /// - `x.round_to_multiple_of_power_of_2_assign(pow, Exact);`
    /// - `assert!(x.divisible_by_power_of_2(pow));`
    ///
    /// but the latter should be used as it is clearer and more efficient.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(self.significant_bits(), pow /
    /// Limb::WIDTH)`.
    ///
    /// # Panics
    /// Panics if `rm` is `Exact`, but `self` is not a multiple of the power of 2.
    ///
    /// # Examples
    /// ```
    /// use core::cmp::Ordering::*;
    /// use malachite_base::num::arithmetic::traits::RoundToMultipleOfPowerOf2Assign;
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_nz::natural::Natural;
    ///
    /// let mut n = Natural::from(10u32);
    /// assert_eq!(n.round_to_multiple_of_power_of_2_assign(2, Floor), Less);
    /// assert_eq!(n, 8);
    ///
    /// let mut n = Natural::from(10u32);
    /// assert_eq!(
    ///     n.round_to_multiple_of_power_of_2_assign(2, Ceiling),
    ///     Greater
    /// );
    /// assert_eq!(n, 12);
    ///
    /// let mut n = Natural::from(10u32);
    /// assert_eq!(n.round_to_multiple_of_power_of_2_assign(2, Down), Less);
    /// assert_eq!(n, 8);
    ///
    /// let mut n = Natural::from(10u32);
    /// assert_eq!(n.round_to_multiple_of_power_of_2_assign(2, Up), Greater);
    /// assert_eq!(n, 12);
    ///
    /// let mut n = Natural::from(10u32);
    /// assert_eq!(n.round_to_multiple_of_power_of_2_assign(2, Nearest), Less);
    /// assert_eq!(n, 8);
    ///
    /// let mut n = Natural::from(12u32);
    /// assert_eq!(n.round_to_multiple_of_power_of_2_assign(2, Exact), Equal);
    /// assert_eq!(n, 12);
    /// ```
    fn round_to_multiple_of_power_of_2_assign(&mut self, pow: u64, rm: RoundingMode) -> Ordering {
        match (&mut *self, pow) {
            (_, 0) | (&mut Natural::ZERO, _) => Equal,
            (Natural(Small(ref mut small)), pow) => {
                let o = small.shr_round_assign(pow, rm);
                *self <<= pow;
                o
            }
            (Natural(Large(ref mut limbs)), pow) => {
                if let Some(o) = limbs_round_to_multiple_of_power_of_2_in_place(limbs, pow, rm) {
                    self.trim();
                    o
                } else {
                    panic!("Rounding is not exact");
                }
            }
        }
    }
}
