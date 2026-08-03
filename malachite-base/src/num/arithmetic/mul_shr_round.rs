// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::num::arithmetic::traits::{MulShrRound, MulShrRoundAssign, UnsignedAbs};
use crate::num::basic::signeds::PrimitiveSigned;
use crate::num::basic::unsigneds::PrimitiveUnsigned;
use crate::num::conversion::traits::{SaturatingFrom, WrappingFrom};
use crate::rounding_modes::RoundingMode::{self, *};
use core::cmp::Ordering::{self, *};

// Bit `i` of the double word `(hi, lo)`, or `false` if `i` is off the top.
fn wide_bit<T: PrimitiveUnsigned>(hi: T, lo: T, i: u64) -> bool {
    let w = T::WIDTH;
    if i < w {
        lo.get_bit(i)
    } else if i < w << 1 {
        hi.get_bit(i - w)
    } else {
        false
    }
}

// Whether any of the low `i` bits of the double word `(hi, lo)` is set. `i` may exceed the double
// word's width.
fn wide_low_bits_nonzero<T: PrimitiveUnsigned>(hi: T, lo: T, i: u64) -> bool {
    let w = T::WIDTH;
    if i == 0 {
        false
    } else if i < w {
        lo << (w - i) != T::ZERO
    } else if i == w {
        lo != T::ZERO
    } else if i < w << 1 {
        lo != T::ZERO || hi << ((w << 1) - i) != T::ZERO
    } else {
        lo != T::ZERO || hi != T::ZERO
    }
}

// The shifted product, rounded according to `rm`. The product is exact, held in a double word, so
// only the rounded result has to fit; when `bits >= T::WIDTH` it always does. This is where the
// `u128` implementation earns its keep: with no wider type to widen into, the kept part, the
// half-ulp bit, and the sticky bits are read straight out of the double word, and no 256-bit value
// is ever materialized or shifted.
fn mul_shr_round_unsigned<T: PrimitiveUnsigned>(
    x: T,
    y: T,
    bits: u64,
    rm: RoundingMode,
) -> (T, Ordering) {
    if x == T::ZERO || y == T::ZERO {
        return (T::ZERO, Equal);
    }
    let w = T::WIDTH;
    let (hi, lo) = T::x_mul_y_to_zz(x, y);
    if bits == 0 {
        assert!(
            hi == T::ZERO,
            "Shifted product does not fit: {x} * {y} >> {bits}"
        );
        return (lo, Equal);
    }
    // The kept part: the double word shifted right by `bits`, which must fit in a single word.
    let k = if bits < w {
        assert!(
            hi >> bits == T::ZERO,
            "Shifted product does not fit: {x} * {y} >> {bits}"
        );
        hi << (w - bits) | lo >> bits
    } else if bits < w << 1 {
        hi >> (bits - w)
    } else {
        T::ZERO
    };
    // The discarded part, characterized by its top bit (a half ulp) and whether anything is set
    // below it.
    let top = wide_bit(hi, lo, bits - 1);
    let rest = wide_low_bits_nonzero(hi, lo, bits - 1);
    let round_up = |k: T| {
        (
            k.checked_add(T::ONE)
                .unwrap_or_else(|| panic!("Shifted product does not fit: {x} * {y} >> {bits}")),
            Greater,
        )
    };
    match rm {
        _ if !top && !rest => (k, Equal),
        Down | Floor => (k, Less),
        Up | Ceiling => round_up(k),
        Exact => panic!("Product right shift is not exact: {x} * {y} >> {bits}"),
        Nearest => {
            if !top {
                (k, Less)
            } else if rest || k.odd() {
                round_up(k)
            } else {
                (k, Less)
            }
        }
    }
}

fn mul_shr_round_signed<
    U: PrimitiveUnsigned,
    T: PrimitiveSigned + UnsignedAbs<Output = U> + WrappingFrom<U>,
>(
    x: T,
    y: T,
    bits: u64,
    rm: RoundingMode,
) -> (T, Ordering) {
    if x == T::ZERO || y == T::ZERO {
        return (T::ZERO, Equal);
    }
    let negative = (x < T::ZERO) != (y < T::ZERO);
    // Rounding a negative value with `rm` is rounding its magnitude with `-rm`, and the magnitude's
    // `Ordering` flips on the way back.
    let (mag, o) = mul_shr_round_unsigned(
        x.unsigned_abs(),
        y.unsigned_abs(),
        bits,
        if negative { -rm } else { rm },
    );
    let lim = U::power_of_2(U::WIDTH - 1);
    if negative {
        match mag.cmp(&lim) {
            Greater => panic!("Shifted product does not fit: {x} * {y} >> {bits}"),
            Equal => (T::MIN, o.reverse()),
            Less => (-T::wrapping_from(mag), o.reverse()),
        }
    } else {
        assert!(
            mag < lim,
            "Shifted product does not fit: {x} * {y} >> {bits}"
        );
        (T::wrapping_from(mag), o)
    }
}

// The product left-shifted by `bits`, for the negative-`bits` case of the signed-`bits` impls. The
// shift is always exact, so there is no rounding; the result must fit.
fn mul_shl_exact_unsigned<T: PrimitiveUnsigned>(x: T, y: T, bits: u64) -> T {
    if x == T::ZERO || y == T::ZERO {
        return T::ZERO;
    }
    let w = T::WIDTH;
    let (hi, lo) = T::x_mul_y_to_zz(x, y);
    assert!(
        hi == T::ZERO && bits < w && lo >> (w - bits) == T::ZERO,
        "Shifted product does not fit: {x} * {y} << {bits}"
    );
    lo << bits
}

fn mul_shl_exact_signed<
    U: PrimitiveUnsigned,
    T: PrimitiveSigned + UnsignedAbs<Output = U> + WrappingFrom<U>,
>(
    x: T,
    y: T,
    bits: u64,
) -> T {
    if x == T::ZERO || y == T::ZERO {
        return T::ZERO;
    }
    let negative = (x < T::ZERO) != (y < T::ZERO);
    let mag = mul_shl_exact_unsigned(x.unsigned_abs(), y.unsigned_abs(), bits);
    let lim = U::power_of_2(U::WIDTH - 1);
    if negative {
        match mag.cmp(&lim) {
            Greater => panic!("Shifted product does not fit: {x} * {y} << {bits}"),
            Equal => T::MIN,
            Less => -T::wrapping_from(mag),
        }
    } else {
        assert!(
            mag < lim,
            "Shifted product does not fit: {x} * {y} << {bits}"
        );
        T::wrapping_from(mag)
    }
}

macro_rules! impl_mul_shr_round_unsigned_unsigned {
    ($t:ident) => {
        macro_rules! impl_mul_shr_round_unsigned_unsigned_inner {
            ($u:ident) => {
                impl MulShrRound<$t, $u> for $t {
                    type Output = $t;

                    /// Multiplies two numbers and right-shifts the product (divides it by a power
                    /// of 2), rounding according to a specified rounding mode. An [`Ordering`] is
                    /// also returned, indicating whether the returned value is less than, equal to,
                    /// or greater than the exact value.
                    ///
                    /// The product is computed at twice the width of the type, so it cannot
                    /// overflow; only the shifted result must fit, and if `bits` is at least
                    /// `Self::WIDTH` it always does.
                    ///
                    /// Let $q = \frac{xy}{2^k}$. Then $f(x, y, k, \mathrm{Down}) = f(x, y, k,
                    /// \mathrm{Floor}) = \lfloor q \rfloor$ and $f(x, y, k, \mathrm{Up}) = f(x, y,
                    /// k, \mathrm{Ceiling}) = \lceil q \rceil$; $\mathrm{Nearest}$ rounds to the
                    /// integer closest to $q$, breaking ties toward the even integer; and $f(x, y,
                    /// k, \mathrm{Exact}) = q$, but panics if $q \notin \Z$.
                    ///
                    /// # Worst-case complexity
                    /// Constant time and additional memory.
                    ///
                    /// # Panics
                    /// Panics if the shifted product does not fit in the type, which is possible
                    /// only when `bits < Self::WIDTH`, or if `rm` is `Exact` but $q \notin \Z$.
                    ///
                    /// # Examples
                    /// See [here](super::mul_shr_round#mul_shr_round).
                    #[inline]
                    fn mul_shr_round(
                        self,
                        other: $t,
                        bits: $u,
                        rm: RoundingMode,
                    ) -> ($t, Ordering) {
                        mul_shr_round_unsigned(self, other, u64::saturating_from(bits), rm)
                    }
                }

                impl MulShrRoundAssign<$t, $u> for $t {
                    /// Multiplies two numbers and right-shifts the product (divides it by a power
                    /// of 2) in place, rounding according to a specified rounding mode. An
                    /// [`Ordering`] is returned, indicating whether the assigned value is less
                    /// than, equal to, or greater than the exact value.
                    ///
                    /// See the [`MulShrRound`] documentation for details.
                    ///
                    /// # Worst-case complexity
                    /// Constant time and additional memory.
                    ///
                    /// # Panics
                    /// Panics if the shifted product does not fit in the type, which is possible
                    /// only when `bits < Self::WIDTH`, or if `rm` is `Exact` but the shift is not
                    /// exact.
                    ///
                    /// # Examples
                    /// See [here](super::mul_shr_round#mul_shr_round_assign).
                    #[inline]
                    fn mul_shr_round_assign(
                        &mut self,
                        other: $t,
                        bits: $u,
                        rm: RoundingMode,
                    ) -> Ordering {
                        let o;
                        (*self, o) = self.mul_shr_round(other, bits, rm);
                        o
                    }
                }
            };
        }
        apply_to_unsigneds!(impl_mul_shr_round_unsigned_unsigned_inner);
    };
}
apply_to_unsigneds!(impl_mul_shr_round_unsigned_unsigned);

macro_rules! impl_mul_shr_round_signed_unsigned {
    ($t:ident) => {
        macro_rules! impl_mul_shr_round_signed_unsigned_inner {
            ($u:ident) => {
                impl MulShrRound<$t, $u> for $t {
                    type Output = $t;

                    /// Multiplies two numbers and right-shifts the product (divides it by a power
                    /// of 2), rounding according to a specified rounding mode. An [`Ordering`] is
                    /// also returned, indicating whether the returned value is less than, equal to,
                    /// or greater than the exact value.
                    ///
                    /// The product is computed exactly, in sign-magnitude form at twice the width
                    /// of the type, so it cannot overflow; only the shifted result must fit, and if
                    /// `bits` is at least `Self::WIDTH` it always does. `Floor` rounds toward
                    /// negative infinity and `Down` rounds toward zero, so they differ when the
                    /// product is negative.
                    ///
                    /// Let $q = \frac{xy}{2^k}$. Then $f(x, y, k, \mathrm{Floor}) = \lfloor q
                    /// \rfloor$, $f(x, y, k, \mathrm{Ceiling}) = \lceil q \rceil$, and
                    /// $\mathrm{Down}$ and $\mathrm{Up}$ round toward and away from zero,
                    /// respectively; $\mathrm{Nearest}$ rounds to the integer closest to $q$,
                    /// breaking ties toward the even integer; and $f(x, y, k, \mathrm{Exact}) = q$,
                    /// but panics if $q \notin \Z$.
                    ///
                    /// # Worst-case complexity
                    /// Constant time and additional memory.
                    ///
                    /// # Panics
                    /// Panics if the shifted product does not fit in the type, which is possible
                    /// only when `bits < Self::WIDTH`, or if `rm` is `Exact` but $q \notin \Z$.
                    ///
                    /// # Examples
                    /// See [here](super::mul_shr_round#mul_shr_round).
                    #[inline]
                    fn mul_shr_round(
                        self,
                        other: $t,
                        bits: $u,
                        rm: RoundingMode,
                    ) -> ($t, Ordering) {
                        mul_shr_round_signed(self, other, u64::saturating_from(bits), rm)
                    }
                }

                impl MulShrRoundAssign<$t, $u> for $t {
                    /// Multiplies two numbers and right-shifts the product (divides it by a power
                    /// of 2) in place, rounding according to a specified rounding mode. An
                    /// [`Ordering`] is returned, indicating whether the assigned value is less
                    /// than, equal to, or greater than the exact value.
                    ///
                    /// See the [`MulShrRound`] documentation for details.
                    ///
                    /// # Worst-case complexity
                    /// Constant time and additional memory.
                    ///
                    /// # Panics
                    /// Panics if the shifted product does not fit in the type, which is possible
                    /// only when `bits < Self::WIDTH`, or if `rm` is `Exact` but the shift is not
                    /// exact.
                    ///
                    /// # Examples
                    /// See [here](super::mul_shr_round#mul_shr_round_assign).
                    #[inline]
                    fn mul_shr_round_assign(
                        &mut self,
                        other: $t,
                        bits: $u,
                        rm: RoundingMode,
                    ) -> Ordering {
                        let o;
                        (*self, o) = self.mul_shr_round(other, bits, rm);
                        o
                    }
                }
            };
        }
        apply_to_unsigneds!(impl_mul_shr_round_signed_unsigned_inner);
    };
}
apply_to_signeds!(impl_mul_shr_round_signed_unsigned);

macro_rules! impl_mul_shr_round_unsigned_signed {
    ($t:ident) => {
        macro_rules! impl_mul_shr_round_unsigned_signed_inner {
            ($s:ident) => {
                impl MulShrRound<$t, $s> for $t {
                    type Output = $t;

                    /// Multiplies two numbers and right-shifts the product (divides it by a power
                    /// of 2), rounding according to a specified rounding mode. An [`Ordering`] is
                    /// also returned, indicating whether the returned value is less than, equal to,
                    /// or greater than the exact value.
                    ///
                    /// If `bits` is negative, the product is left-shifted by `-bits` instead. That
                    /// shift is always exact, so the returned [`Ordering`] is `Equal`; but unlike
                    /// [`ShrRound`](crate::num::arithmetic::traits::ShrRound), whose left shifts
                    /// discard overflowing bits, this operation panics if the result does not fit,
                    /// since its purpose is to return the exact rounded value.
                    ///
                    /// # Worst-case complexity
                    /// Constant time and additional memory.
                    ///
                    /// # Panics
                    /// Panics if the shifted product does not fit in the type, which is possible
                    /// only when `bits < Self::WIDTH`, or if `rm` is `Exact` but the shift is not
                    /// exact.
                    ///
                    /// # Examples
                    /// See [here](super::mul_shr_round#mul_shr_round).
                    #[inline]
                    fn mul_shr_round(
                        self,
                        other: $t,
                        bits: $s,
                        rm: RoundingMode,
                    ) -> ($t, Ordering) {
                        if bits >= 0 {
                            self.mul_shr_round(other, bits.unsigned_abs(), rm)
                        } else {
                            (
                                mul_shl_exact_unsigned(
                                    self,
                                    other,
                                    u64::saturating_from(bits.unsigned_abs()),
                                ),
                                Equal,
                            )
                        }
                    }
                }

                impl MulShrRoundAssign<$t, $s> for $t {
                    /// Multiplies two numbers and right-shifts the product (divides it by a power
                    /// of 2) in place, rounding according to a specified rounding mode. An
                    /// [`Ordering`] is returned, indicating whether the assigned value is less
                    /// than, equal to, or greater than the exact value.
                    ///
                    /// See the [`MulShrRound`] documentation for details.
                    ///
                    /// # Worst-case complexity
                    /// Constant time and additional memory.
                    ///
                    /// # Panics
                    /// Panics if the shifted product does not fit in the type, which is possible
                    /// only when `bits < Self::WIDTH`, or if `rm` is `Exact` but the shift is not
                    /// exact.
                    ///
                    /// # Examples
                    /// See [here](super::mul_shr_round#mul_shr_round_assign).
                    #[inline]
                    fn mul_shr_round_assign(
                        &mut self,
                        other: $t,
                        bits: $s,
                        rm: RoundingMode,
                    ) -> Ordering {
                        let o;
                        (*self, o) = self.mul_shr_round(other, bits, rm);
                        o
                    }
                }
            };
        }
        apply_to_signeds!(impl_mul_shr_round_unsigned_signed_inner);
    };
}
apply_to_unsigneds!(impl_mul_shr_round_unsigned_signed);

macro_rules! impl_mul_shr_round_signed_signed {
    ($t:ident) => {
        macro_rules! impl_mul_shr_round_signed_signed_inner {
            ($s:ident) => {
                impl MulShrRound<$t, $s> for $t {
                    type Output = $t;

                    /// Multiplies two numbers and right-shifts the product (divides it by a power
                    /// of 2), rounding according to a specified rounding mode. An [`Ordering`] is
                    /// also returned, indicating whether the returned value is less than, equal to,
                    /// or greater than the exact value.
                    ///
                    /// If `bits` is negative, the product is left-shifted by `-bits` instead. That
                    /// shift is always exact, so the returned [`Ordering`] is `Equal`; but unlike
                    /// [`ShrRound`](crate::num::arithmetic::traits::ShrRound), whose left shifts
                    /// discard overflowing bits, this operation panics if the result does not fit,
                    /// since its purpose is to return the exact rounded value.
                    ///
                    /// # Worst-case complexity
                    /// Constant time and additional memory.
                    ///
                    /// # Panics
                    /// Panics if the shifted product does not fit in the type, which is possible
                    /// only when `bits < Self::WIDTH`, or if `rm` is `Exact` but the shift is not
                    /// exact.
                    ///
                    /// # Examples
                    /// See [here](super::mul_shr_round#mul_shr_round).
                    #[inline]
                    fn mul_shr_round(
                        self,
                        other: $t,
                        bits: $s,
                        rm: RoundingMode,
                    ) -> ($t, Ordering) {
                        if bits >= 0 {
                            self.mul_shr_round(other, bits.unsigned_abs(), rm)
                        } else {
                            (
                                mul_shl_exact_signed(
                                    self,
                                    other,
                                    u64::saturating_from(bits.unsigned_abs()),
                                ),
                                Equal,
                            )
                        }
                    }
                }

                impl MulShrRoundAssign<$t, $s> for $t {
                    /// Multiplies two numbers and right-shifts the product (divides it by a power
                    /// of 2) in place, rounding according to a specified rounding mode. An
                    /// [`Ordering`] is returned, indicating whether the assigned value is less
                    /// than, equal to, or greater than the exact value.
                    ///
                    /// See the [`MulShrRound`] documentation for details.
                    ///
                    /// # Worst-case complexity
                    /// Constant time and additional memory.
                    ///
                    /// # Panics
                    /// Panics if the shifted product does not fit in the type, which is possible
                    /// only when `bits < Self::WIDTH`, or if `rm` is `Exact` but the shift is not
                    /// exact.
                    ///
                    /// # Examples
                    /// See [here](super::mul_shr_round#mul_shr_round_assign).
                    #[inline]
                    fn mul_shr_round_assign(
                        &mut self,
                        other: $t,
                        bits: $s,
                        rm: RoundingMode,
                    ) -> Ordering {
                        let o;
                        (*self, o) = self.mul_shr_round(other, bits, rm);
                        o
                    }
                }
            };
        }
        apply_to_signeds!(impl_mul_shr_round_signed_signed_inner);
    };
}
apply_to_signeds!(impl_mul_shr_round_signed_signed);
