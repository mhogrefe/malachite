// Copyright © 2024 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::integer::Integer;
use crate::natural::Natural;
use core::cmp::Ordering::{self, *};
use core::ops::{Shl, ShlAssign};
use malachite_base::num::arithmetic::traits::{ShrRound, ShrRoundAssign, UnsignedAbs};
use malachite_base::num::basic::traits::Zero;
use malachite_base::rounding_modes::RoundingMode;

fn shr_round_unsigned_ref_i<'a, T>(x: &'a Integer, bits: T, rm: RoundingMode) -> (Integer, Ordering)
where
    &'a Natural: ShrRound<T, Output = Natural>,
{
    match *x {
        Integer {
            sign: true,
            ref abs,
        } => {
            let (s, o) = abs.shr_round(bits, rm);
            (Integer { sign: true, abs: s }, o)
        }
        Integer {
            sign: false,
            ref abs,
        } => {
            let (abs_shifted, o) = abs.shr_round(bits, -rm);
            (
                if abs_shifted == 0 {
                    Integer::ZERO
                } else {
                    Integer {
                        sign: false,
                        abs: abs_shifted,
                    }
                },
                o.reverse(),
            )
        }
    }
}

fn shr_round_assign_unsigned_i<T>(x: &mut Integer, bits: T, rm: RoundingMode) -> Ordering
where
    Natural: ShrRoundAssign<T>,
{
    match *x {
        Integer {
            sign: true,
            ref mut abs,
        } => abs.shr_round_assign(bits, rm),
        Integer {
            sign: false,
            ref mut abs,
        } => {
            let o = abs.shr_round_assign(bits, -rm);
            if *abs == 0 {
                x.sign = true;
            }
            o.reverse()
        }
    }
}

macro_rules! impl_shr_round_unsigned {
    ($t:ident) => {
        impl ShrRound<$t> for Integer {
            type Output = Integer;

            /// Shifts an [`Integer`] right (divides it by a power of 2), taking it by value, and
            /// rounds according to the specified rounding mode. An [`Ordering`] is also returned,
            /// indicating whether the returned value is less than, equal to, or greater than the
            /// exact value.
            ///
            /// Passing `Floor` is equivalent to using `>>`. To test whether `Exact` can be passed,
            /// use `self.divisible_by_power_of_2(bits)`.
            ///
            /// Let $q = \frac{x}{2^k}$, and let $g$ be the function that just returns the first
            /// element of the pair, without the [`Ordering`]:
            ///
            /// $f(x, k, \mathrm{Down}) = \operatorname{sgn}(q) \lfloor |q| \rfloor.$
            ///
            /// $f(x, k, \mathrm{Up}) = \operatorname{sgn}(q) \lceil |q| \rceil.$
            ///
            /// $f(x, k, \mathrm{Floor}) = \lfloor q \rfloor.$
            ///
            /// $f(x, k, \mathrm{Ceiling}) = \lceil q \rceil.$
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
            /// $f(x, k, \mathrm{Exact}) = q$, but panics if $q \notin \Z$.
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
            fn shr_round(mut self, bits: $t, rm: RoundingMode) -> (Integer, Ordering) {
                let o = self.shr_round_assign(bits, rm);
                (self, o)
            }
        }

        impl<'a> ShrRound<$t> for &'a Integer {
            type Output = Integer;

            /// Shifts an [`Integer`] right (divides it by a power of 2), taking it by reference,
            /// and rounds according to the specified rounding mode. An [`Ordering`] is also
            /// returned, indicating whether the returned value is less than, equal to, or greater
            /// than the exact value.
            ///
            /// Passing `Floor` is equivalent to using `>>`. To test whether `Exact` can be passed,
            /// use `self.divisible_by_power_of_2(bits)`.
            ///
            /// Let $q = \frac{x}{2^k}$, and let $g$ be the function that just returns the first
            /// element of the pair, without the [`Ordering`]:
            ///
            /// $f(x, k, \mathrm{Down}) = \operatorname{sgn}(q) \lfloor |q| \rfloor.$
            ///
            /// $f(x, k, \mathrm{Up}) = \operatorname{sgn}(q) \lceil |q| \rceil.$
            ///
            /// $f(x, k, \mathrm{Floor}) = \lfloor q \rfloor.$
            ///
            /// $f(x, k, \mathrm{Ceiling}) = \lceil q \rceil.$
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
            /// $f(x, k, \mathrm{Exact}) = q$, but panics if $q \notin \Z$.
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
            fn shr_round(self, bits: $t, rm: RoundingMode) -> (Integer, Ordering) {
                shr_round_unsigned_ref_i(self, bits, rm)
            }
        }

        impl ShrRoundAssign<$t> for Integer {
            /// Shifts a [`Natural`] right (divides it by a power of 2) and rounds according to the
            /// specified rounding mode, in place. Passing `Floor` is equivalent to using `>>=`. To
            /// test whether `Exact` can be passed, use `self.divisible_by_power_of_2(bits)`. An
            /// [`Ordering`] is returned, indicating whether the assigned value is less than, equal
            /// to, or greater than the exact value.
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
            fn shr_round_assign(&mut self, bits: $t, rm: RoundingMode) -> Ordering {
                shr_round_assign_unsigned_i(self, bits, rm)
            }
        }
    };
}
apply_to_unsigneds!(impl_shr_round_unsigned);

fn shr_round_signed_ref_i<'a, U, S: Copy + Ord + UnsignedAbs<Output = U> + Zero>(
    x: &'a Integer,
    bits: S,
    rm: RoundingMode,
) -> (Integer, Ordering)
where
    &'a Integer: Shl<U, Output = Integer> + ShrRound<U, Output = Integer>,
{
    if bits >= S::ZERO {
        x.shr_round(bits.unsigned_abs(), rm)
    } else {
        (x << bits.unsigned_abs(), Equal)
    }
}

fn shr_round_assign_signed_i<U, S: Copy + Ord + UnsignedAbs<Output = U> + Zero>(
    x: &mut Integer,
    bits: S,
    rm: RoundingMode,
) -> Ordering
where
    Integer: ShlAssign<U> + ShrRoundAssign<U>,
{
    if bits >= S::ZERO {
        x.shr_round_assign(bits.unsigned_abs(), rm)
    } else {
        *x <<= bits.unsigned_abs();
        Equal
    }
}

macro_rules! impl_shr_round_signed {
    ($t:ident) => {
        impl ShrRound<$t> for Integer {
            type Output = Integer;

            /// Shifts an [`Integer`] right (divides or multiplies it by a power of 2), taking it by
            /// value, and rounds according to the specified rounding mode. An [`Ordering`] is also
            /// returned, indicating whether the returned value is less than, equal to, or greater
            /// than the exact value. If `bits` is negative, then the returned [`Ordering`] is
            /// always `Equal`, even if the higher bits of the result are lost.
            ///
            /// Passing `Floor` is equivalent to using `>>`. To test whether `Exact` can be passed,
            /// use `self.divisible_by_power_of_2(bits)`.
            ///
            /// Let $q = \frac{x}{2^k}$, and let $g$ be the function that just returns the first
            /// element of the pair, without the [`Ordering`]:
            ///
            /// $f(x, k, \mathrm{Down}) = \operatorname{sgn}(q) \lfloor |q| \rfloor.$
            ///
            /// $f(x, k, \mathrm{Up}) = \operatorname{sgn}(q) \lceil |q| \rceil.$
            ///
            /// $f(x, k, \mathrm{Floor}) = \lfloor q \rfloor.$
            ///
            /// $f(x, k, \mathrm{Ceiling}) = \lceil q \rceil.$
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
            /// $f(x, k, \mathrm{Exact}) = q$, but panics if $q \notin \Z$.
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
            fn shr_round(mut self, bits: $t, rm: RoundingMode) -> (Integer, Ordering) {
                let o = self.shr_round_assign(bits, rm);
                (self, o)
            }
        }

        impl<'a> ShrRound<$t> for &'a Integer {
            type Output = Integer;

            /// Shifts an [`Integer`] right (divides or multiplies it by a power of 2), taking it by
            /// reference, and rounds according to the specified rounding mode. An [`Ordering`] is
            /// also returned, indicating whether the returned value is less than, equal to, or
            /// greater than the exact value. If `bits` is negative, then the returned [`Ordering`]
            /// is always `Equal`, even if the higher bits of the result are lost.
            ///
            /// Passing `Floor` is equivalent to using `>>`. To test whether `Exact` can be passed,
            /// use `self.divisible_by_power_of_2(bits)`.
            ///
            /// Let $q = \frac{x}{2^k}$, and let $g$ be the function that just returns the first
            /// element of the pair, without the [`Ordering`]:
            ///
            /// $f(x, k, \mathrm{Down}) = \operatorname{sgn}(q) \lfloor |q| \rfloor.$
            ///
            /// $f(x, k, \mathrm{Up}) = \operatorname{sgn}(q) \lceil |q| \rceil.$
            ///
            /// $f(x, k, \mathrm{Floor}) = \lfloor q \rfloor.$
            ///
            /// $f(x, k, \mathrm{Ceiling}) = \lceil q \rceil.$
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
            /// $f(x, k, \mathrm{Exact}) = q$, but panics if $q \notin \Z$.
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
            fn shr_round(self, bits: $t, rm: RoundingMode) -> (Integer, Ordering) {
                shr_round_signed_ref_i(self, bits, rm)
            }
        }

        impl ShrRoundAssign<$t> for Integer {
            /// Shifts an [`Integer`] right (divides or multiplies it by a power of 2) and rounds
            /// according to the specified rounding mode, in place. An [`Ordering`] is returned,
            /// indicating whether the assigned value is less than, equal to, or greater than the
            /// exact value. If `bits` is negative, then the returned [`Ordering`] is always
            /// `Equal`, even if the higher bits of the result are lost.
            ///
            /// Passing `Floor` is equivalent to using `>>`. To test whether `Exact` can be passed,
            /// use `self.divisible_by_power_of_2(bits)`.
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
                shr_round_assign_signed_i(self, bits, rm)
            }
        }
    };
}
apply_to_signeds!(impl_shr_round_signed);
