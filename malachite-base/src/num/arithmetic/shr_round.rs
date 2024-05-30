// Copyright © 2024 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::num::arithmetic::traits::{ShrRound, ShrRoundAssign, UnsignedAbs};
use crate::num::basic::integers::PrimitiveInt;
use crate::num::basic::signeds::PrimitiveSigned;
use crate::num::basic::unsigneds::PrimitiveUnsigned;
use crate::num::conversion::traits::WrappingFrom;
use crate::rounding_modes::RoundingMode::{self, *};
use core::cmp::Ordering::{self, *};
use core::ops::{Shl, ShlAssign, Shr, ShrAssign};

fn shr_round_unsigned_unsigned<
    T: PrimitiveUnsigned + Shl<U, Output = T> + Shr<U, Output = T>,
    U: PrimitiveUnsigned,
>(
    x: T,
    bits: U,
    rm: RoundingMode,
) -> (T, Ordering) {
    if bits == U::ZERO || x == T::ZERO {
        return (x, Equal);
    }
    let width = U::wrapping_from(T::WIDTH);
    match rm {
        Down | Floor if bits >= width => (T::ZERO, Less),
        Down | Floor => {
            let shifted = x >> bits;
            (shifted, if shifted << bits == x { Equal } else { Less })
        }
        Up | Ceiling if bits >= width => (T::ONE, Greater),
        Up | Ceiling => {
            let shifted = x >> bits;
            if shifted << bits == x {
                (shifted, Equal)
            } else {
                (shifted + T::ONE, Greater)
            }
        }
        Nearest if bits == width && x > T::power_of_2(T::WIDTH - 1) => (T::ONE, Greater),
        Nearest if bits >= width => (T::ZERO, Less),
        Nearest => {
            let bm1 = bits - U::ONE;
            let mostly_shifted = x >> bm1;
            let bm1_zeros = mostly_shifted << bm1 == x;
            if mostly_shifted.even() {
                // round down
                (mostly_shifted >> 1, if bm1_zeros { Equal } else { Less })
            } else if !bm1_zeros {
                // round up
                ((mostly_shifted >> 1) + T::ONE, Greater)
            } else {
                // result is half-integer; round to even
                let shifted: T = mostly_shifted >> 1;
                if shifted.even() {
                    (shifted, Less)
                } else {
                    (shifted + T::ONE, Greater)
                }
            }
        }
        Exact if bits >= width => {
            panic!("Right shift is not exact: {x} >> {bits}");
        }
        Exact => {
            let shifted = x >> bits;
            assert!(
                shifted << bits == x,
                "Right shift is not exact: {x} >> {bits}"
            );
            (shifted, Equal)
        }
    }
}

fn shr_round_assign_unsigned_unsigned<
    T: PrimitiveUnsigned + Shl<U, Output = T> + ShrAssign<U>,
    U: PrimitiveUnsigned,
>(
    x: &mut T,
    bits: U,
    rm: RoundingMode,
) -> Ordering {
    if bits == U::ZERO || *x == T::ZERO {
        return Equal;
    }
    let width = U::wrapping_from(T::WIDTH);
    match rm {
        Down | Floor if bits >= width => {
            *x = T::ZERO;
            Less
        }
        Down | Floor => {
            let original = *x;
            *x >>= bits;
            if *x << bits == original {
                Equal
            } else {
                Less
            }
        }
        Up | Ceiling if bits >= width => {
            *x = T::ONE;
            Greater
        }
        Up | Ceiling => {
            let original = *x;
            *x >>= bits;
            if *x << bits == original {
                Equal
            } else {
                *x += T::ONE;
                Greater
            }
        }
        Nearest if bits == width && *x > T::power_of_2(T::WIDTH - 1) => {
            *x = T::ONE;
            Greater
        }
        Nearest if bits >= width => {
            *x = T::ZERO;
            Less
        }
        Nearest => {
            let original = *x;
            let bm1 = bits - U::ONE;
            *x >>= bm1;
            let bm1_zeros = *x << bm1 == original;
            let old_x = *x;
            *x >>= 1;
            if old_x.even() {
                // round down
                if bm1_zeros {
                    Equal
                } else {
                    Less
                }
            } else if !bm1_zeros {
                // round up
                *x += T::ONE;
                Greater
            } else {
                // result is half-integer; round to even
                if x.even() {
                    Less
                } else {
                    *x += T::ONE;
                    Greater
                }
            }
        }
        Exact if bits >= width => {
            panic!("Right shift is not exact: {} >>= {}", *x, bits);
        }
        Exact => {
            let original = *x;
            *x >>= bits;
            assert!(
                *x << bits == original,
                "Right shift is not exact: {original} >>= {bits}"
            );
            Equal
        }
    }
}

macro_rules! impl_shr_round_unsigned_unsigned {
    ($t:ident) => {
        macro_rules! impl_shr_round_unsigned_unsigned_inner {
            ($u:ident) => {
                impl ShrRound<$u> for $t {
                    type Output = $t;

                    /// Shifts a number right (divides it by a power of 2) and rounds according to
                    /// the specified rounding mode. An [`Ordering`] is also returned, indicating
                    /// whether the returned value is less than, equal to, or greater than the exact
                    /// value.
                    ///
                    /// Passing `Floor` or `Down` is equivalent to using `>>`. To test whether
                    /// `Exact` can be passed, use `self.divisible_by_power_of_2(bits)`.
                    ///
                    /// Let $q = \frac{x}{2^k}$, and let $g$ be the function that just returns the
                    /// first element of the pair, without the [`Ordering`]:
                    ///
                    /// $g(x, k, \mathrm{Down}) = g(x, y, \mathrm{Floor}) = \lfloor q \rfloor.$
                    ///
                    /// $g(x, k, \mathrm{Up}) = g(x, y, \mathrm{Ceiling}) = \lceil q \rceil.$
                    ///
                    /// $$
                    /// g(x, k, \mathrm{Nearest}) = \begin{cases}
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
                    /// $g(x, k, \mathrm{Exact}) = q$, but panics if $q \notin \N$.
                    ///
                    /// Then
                    ///
                    /// $f(x, k, r) = (g(x, k, r), \operatorname{cmp}(g(x, k, r), q))$.
                    ///
                    /// # Worst-case complexity
                    /// Constant time and additional memory.
                    ///
                    /// # Panics
                    /// Panics if `rm` is `Exact` but `self` is not divisible by $2^b$.
                    ///
                    /// # Examples
                    /// See [here](super::shr_round#shr_round).
                    #[inline]
                    fn shr_round(self, bits: $u, rm: RoundingMode) -> ($t, Ordering) {
                        shr_round_unsigned_unsigned(self, bits, rm)
                    }
                }

                impl ShrRoundAssign<$u> for $t {
                    /// Shifts a number right (divides it by a power of 2) and rounds according to
                    /// the specified rounding mode, in place. An [`Ordering`] is returned,
                    /// indicating whether the assigned value is less than, equal to, or greater
                    /// than the exact value.
                    ///
                    /// Passing `Floor` or `Down` is equivalent to using `>>`. To test whether
                    /// `Exact` can be passed, use `self.divisible_by_power_of_2(bits)`.
                    ///
                    /// See the [`ShrRound`] documentation for details.
                    ///
                    /// # Worst-case complexity
                    /// Constant time and additional memory.
                    ///
                    /// # Panics
                    /// Panics if `rm` is `Exact` but `self` is not divisible by $2^b$.
                    ///
                    /// # Examples
                    /// See [here](super::shr_round#shr_round_assign).
                    #[inline]
                    fn shr_round_assign(&mut self, bits: $u, rm: RoundingMode) -> Ordering {
                        shr_round_assign_unsigned_unsigned(self, bits, rm)
                    }
                }
            };
        }
        apply_to_unsigneds!(impl_shr_round_unsigned_unsigned_inner);
    };
}
apply_to_unsigneds!(impl_shr_round_unsigned_unsigned);

fn shr_round_signed_unsigned<
    U: PrimitiveUnsigned + ShrRound<B, Output = U>,
    S: PrimitiveSigned + UnsignedAbs<Output = U> + WrappingFrom<U>,
    B,
>(
    x: S,
    bits: B,
    rm: RoundingMode,
) -> (S, Ordering) {
    let abs = x.unsigned_abs();
    if x >= S::ZERO {
        let (abs_shifted, o) = abs.shr_round(bits, rm);
        (S::wrapping_from(abs_shifted), o)
    } else {
        let (abs_shifted, o) = abs.shr_round(bits, -rm);
        (
            if abs_shifted == U::ZERO {
                S::ZERO
            } else if abs_shifted == S::MIN.unsigned_abs() {
                S::MIN
            } else {
                -S::wrapping_from(abs_shifted)
            },
            o.reverse(),
        )
    }
}

macro_rules! impl_shr_round_signed_unsigned {
    ($t:ident) => {
        macro_rules! impl_shr_round_signed_unsigned_inner {
            ($u:ident) => {
                impl ShrRound<$u> for $t {
                    type Output = $t;

                    /// Shifts a number right (divides it by a power of 2) and rounds according to
                    /// the specified rounding mode. An [`Ordering`] is also returned, indicating
                    /// whether the returned value is less than, equal to, or greater than the exact
                    /// value.
                    ///
                    /// Passing `Floor` or `Down` is equivalent to using `>>`. To test whether
                    /// `Exact` can be passed, use `self.divisible_by_power_of_2(bits)`.
                    ///
                    /// Let $q = \frac{x}{2^k}$, and let $g$ be the function that just returns the
                    /// first element of the pair, without the [`Ordering`]:
                    ///
                    /// $g(x, k, \mathrm{Down}) = g(x, y, \mathrm{Floor}) = \lfloor q \rfloor.$
                    ///
                    /// $g(x, k, \mathrm{Up}) = g(x, y, \mathrm{Ceiling}) = \lceil q \rceil.$
                    ///
                    /// $$
                    /// g(x, k, \mathrm{Nearest}) = \begin{cases}
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
                    /// $g(x, k, \mathrm{Exact}) = q$, but panics if $q \notin \N$.
                    ///
                    /// Then
                    ///
                    /// $f(x, k, r) = (g(x, k, r), \operatorname{cmp}(g(x, k, r), q))$.
                    ///
                    /// # Worst-case complexity
                    /// Constant time and additional memory.
                    ///
                    /// # Panics
                    /// Panics if `rm` is `Exact` but `self` is not divisible by $2^b$.
                    ///
                    /// # Examples
                    /// See [here](super::shr_round#shr_round).
                    #[inline]
                    fn shr_round(self, bits: $u, rm: RoundingMode) -> ($t, Ordering) {
                        shr_round_signed_unsigned(self, bits, rm)
                    }
                }

                impl ShrRoundAssign<$u> for $t {
                    /// Shifts a number right (divides it by a power of 2) and rounds according to
                    /// the specified rounding mode, in place. An [`Ordering`] isreturned,
                    /// indicating whether the assigned value is less than, equal to, or greater
                    /// than the exact value.
                    ///
                    /// Passing `Floor` or `Down` is equivalent to using `>>`. To test whether
                    /// `Exact` can be passed, use `self.divisible_by_power_of_2(bits)`.
                    ///
                    /// # Worst-case complexity
                    /// Constant time and additional memory.
                    ///
                    /// # Panics
                    /// Panics if `rm` is `Exact` but `self` is not divisible by $2^b$.
                    ///
                    /// # Examples
                    /// See [here](super::shr_round#shr_round_assign).
                    #[inline]
                    fn shr_round_assign(&mut self, bits: $u, rm: RoundingMode) -> Ordering {
                        let o;
                        (*self, o) = self.shr_round(bits, rm);
                        o
                    }
                }
            };
        }
        apply_to_unsigneds!(impl_shr_round_signed_unsigned_inner);
    };
}
apply_to_signeds!(impl_shr_round_signed_unsigned);

fn shr_round_primitive_signed<
    T: PrimitiveInt + Shl<U, Output = T> + ShrRound<U, Output = T>,
    U: PrimitiveUnsigned,
    S: PrimitiveSigned + UnsignedAbs<Output = U>,
>(
    x: T,
    bits: S,
    rm: RoundingMode,
) -> (T, Ordering) {
    if bits >= S::ZERO {
        x.shr_round(bits.unsigned_abs(), rm)
    } else {
        let abs = bits.unsigned_abs();
        (
            if abs >= U::wrapping_from(T::WIDTH) {
                T::ZERO
            } else {
                x << bits.unsigned_abs()
            },
            Equal,
        )
    }
}

fn shr_round_assign_primitive_signed<
    T: PrimitiveInt + ShlAssign<U> + ShrRoundAssign<U>,
    U: PrimitiveUnsigned,
    S: PrimitiveSigned + UnsignedAbs<Output = U>,
>(
    x: &mut T,
    bits: S,
    rm: RoundingMode,
) -> Ordering {
    if bits >= S::ZERO {
        x.shr_round_assign(bits.unsigned_abs(), rm)
    } else {
        let abs = bits.unsigned_abs();
        if abs >= U::wrapping_from(T::WIDTH) {
            *x = T::ZERO;
        } else {
            *x <<= bits.unsigned_abs();
        }
        Equal
    }
}

macro_rules! impl_shr_round_primitive_signed {
    ($t:ident) => {
        macro_rules! impl_shr_round_primitive_signed_inner {
            ($u:ident) => {
                impl ShrRound<$u> for $t {
                    type Output = $t;

                    /// Shifts a number right (divides it by a power of 2) and rounds according to
                    /// the specified rounding mode. An [`Ordering`] is also returned, indicating
                    /// whether the returned value is less than, equal to, or greater than the exact
                    /// value. If `bits` is negative, then the returned [`Ordering`] is always
                    /// `Equal`, even if the higher bits of the result are lost.
                    ///
                    /// Passing `Floor` or `Down` is equivalent to using `>>`. To test whether
                    /// `Exact` can be passed, use `self.divisible_by_power_of_2(bits)`. Rounding
                    /// might only be necessary if `bits` is non-negative.
                    ///
                    /// Let $q = \frac{x}{2^k}$, and let $g$ be the function that just returns the
                    /// first element of the pair, without the [`Ordering`]:
                    ///
                    /// $g(x, k, \mathrm{Down}) = \operatorname{sgn}(q) \lfloor |q| \rfloor.$
                    ///
                    /// $g(x, k, \mathrm{Up}) = \operatorname{sgn}(q) \lceil |q| \rceil.$
                    ///
                    /// $g(x, k, \mathrm{Floor}) = \lfloor q \rfloor.$
                    ///
                    /// $g(x, k, \mathrm{Ceiling}) = \lceil q \rceil.$
                    ///
                    /// $$
                    /// g(x, k, \mathrm{Nearest}) = \begin{cases}
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
                    /// $g(x, k, \mathrm{Exact}) = q$, but panics if $q \notin \Z$.
                    ///
                    /// Then
                    ///
                    /// $f(x, k, r) = (g(x, k, r), \operatorname{cmp}(g(x, k, r), q))$.
                    ///
                    /// # Worst-case complexity
                    /// Constant time and additional memory.
                    ///
                    /// # Panics
                    /// Panics if `bits` is positive and `rm` is `Exact` but `self` is not divisible
                    /// by $2^b$.
                    ///
                    /// # Examples
                    /// See [here](super::shr_round#shr_round).
                    #[inline]
                    fn shr_round(self, bits: $u, rm: RoundingMode) -> ($t, Ordering) {
                        shr_round_primitive_signed(self, bits, rm)
                    }
                }

                impl ShrRoundAssign<$u> for $t {
                    /// Shifts a number right (divides it by a power of 2) and rounds according to
                    /// the specified rounding mode, in place. An [`Ordering`] is returned,
                    /// indicating whether the assigned value is less than, equal to, or greater
                    /// than the exact value. If `bits` is negative, then the returned [`Ordering`]
                    /// is always `Equal`, even if the higher bits of the result are lost.
                    ///
                    /// Passing `Floor` or `Down` is equivalent to using `>>`. To test whether
                    /// `Exact` can be passed, use `self.divisible_by_power_of_2(bits)`. Rounding
                    /// might only be necessary if `bits` is non-negative.
                    ///
                    /// See the [`ShrRound`] documentation for details.
                    ///
                    /// # Worst-case complexity
                    /// Constant time and additional memory.
                    ///
                    /// # Panics
                    /// Panics if `bits` is positive and `rm` is `Exact` but `self` is not divisible
                    /// by $2^b$.
                    ///
                    /// # Examples
                    /// See [here](super::shr_round#shr_round_assign).
                    #[inline]
                    fn shr_round_assign(&mut self, bits: $u, rm: RoundingMode) -> Ordering {
                        shr_round_assign_primitive_signed(self, bits, rm)
                    }
                }
            };
        }
        apply_to_signeds!(impl_shr_round_primitive_signed_inner);
    };
}
apply_to_primitive_ints!(impl_shr_round_primitive_signed);
