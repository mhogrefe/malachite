// Copyright © 2025 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::malachite_base::num::logic::traits::SignificantBits;
use crate::Float;
use crate::InnerFloat::Finite;
use core::cmp::Ordering::{self, *};
use malachite_base::num::arithmetic::traits::{IsPowerOf2, ShlRound, ShlRoundAssign};
use malachite_base::num::basic::integers::PrimitiveInt;
use malachite_base::num::basic::traits::{Infinity, NegativeInfinity, NegativeZero, Zero};
use malachite_base::num::conversion::traits::WrappingFrom;
use malachite_base::rounding_modes::RoundingMode::{self, *};

pub(crate) fn shl_prec_round_assign_helper<T: PrimitiveInt>(
    x: &mut Float,
    bits: T,
    prec: u64,
    rm: RoundingMode,
    previous_o: Ordering,
) -> Ordering
where
    i32: WrappingFrom<T>,
{
    if let Float(Finite {
        significand,
        exponent,
        sign,
        precision,
    }) = x
    {
        let mut possibly_just_under_min = false;
        if let Ok(bits) = bits.try_into() {
            if let Some(new_exponent) = exponent.checked_add(bits) {
                possibly_just_under_min = true;
                if (Float::MIN_EXPONENT..=Float::MAX_EXPONENT).contains(&new_exponent) {
                    *exponent = new_exponent;
                    return previous_o;
                }
            }
        }
        assert!(rm != Exact, "Inexact Float left-shift");
        if bits > T::ZERO {
            match (*sign, rm) {
                (true, Up | Ceiling | Nearest) => {
                    *x = Float::INFINITY;
                    Greater
                }
                (true, Floor | Down) => {
                    *x = Float::max_finite_value_with_prec(prec);
                    Less
                }
                (false, Up | Floor | Nearest) => {
                    *x = Float::NEGATIVE_INFINITY;
                    Less
                }
                (false, Ceiling | Down) => {
                    *x = -Float::max_finite_value_with_prec(prec);
                    Greater
                }
                (_, Exact) => unreachable!(),
            }
        } else if rm == Nearest
            && possibly_just_under_min
            && *exponent + i32::wrapping_from(bits) == Float::MIN_EXPONENT - 1
            && (previous_o == if *sign { Less } else { Greater } || !significand.is_power_of_2())
        {
            if *sign {
                *x = Float::min_positive_value_prec(*precision);
                Greater
            } else {
                *x = -Float::min_positive_value_prec(*precision);
                Less
            }
        } else {
            match (*sign, rm) {
                (true, Up | Ceiling) => {
                    *x = Float::min_positive_value_prec(prec);
                    Greater
                }
                (true, Floor | Down | Nearest) => {
                    *x = Float::ZERO;
                    Less
                }
                (false, Up | Floor) => {
                    *x = -Float::min_positive_value_prec(prec);
                    Less
                }
                (false, Ceiling | Down | Nearest) => {
                    *x = Float::NEGATIVE_ZERO;
                    Greater
                }
                (_, Exact) => unreachable!(),
            }
        }
    } else {
        Equal
    }
}

fn shl_round_primitive_int_ref<T: PrimitiveInt>(
    x: &Float,
    bits: T,
    rm: RoundingMode,
) -> (Float, Ordering)
where
    i32: WrappingFrom<T>,
{
    if let Float(Finite {
        significand,
        exponent,
        sign,
        precision,
    }) = x
    {
        let mut possibly_just_under_min = false;
        if let Ok(bits) = bits.try_into() {
            if let Some(new_exponent) = exponent.checked_add(bits) {
                possibly_just_under_min = true;
                if (Float::MIN_EXPONENT..=Float::MAX_EXPONENT).contains(&new_exponent) {
                    return (
                        Float(Finite {
                            significand: significand.clone(),
                            exponent: new_exponent,
                            sign: *sign,
                            precision: *precision,
                        }),
                        Equal,
                    );
                }
            }
        }
        assert!(rm != Exact, "Inexact Float left-shift");
        if bits > T::ZERO {
            match (*sign, rm) {
                (true, Up | Ceiling | Nearest) => (Float::INFINITY, Greater),
                (true, Floor | Down) => (Float::max_finite_value_with_prec(*precision), Less),
                (false, Up | Floor | Nearest) => (Float::NEGATIVE_INFINITY, Less),
                (false, Ceiling | Down) => {
                    (-Float::max_finite_value_with_prec(*precision), Greater)
                }
                (_, Exact) => unreachable!(),
            }
        } else if rm == Nearest
            && possibly_just_under_min
            && *exponent + i32::wrapping_from(bits) == Float::MIN_EXPONENT - 1
            && !significand.is_power_of_2()
        {
            if *sign {
                (Float::min_positive_value_prec(*precision), Greater)
            } else {
                (-Float::min_positive_value_prec(*precision), Less)
            }
        } else {
            match (*sign, rm) {
                (true, Up | Ceiling) => (Float::min_positive_value_prec(*precision), Greater),
                (true, Floor | Down | Nearest) => (Float::ZERO, Less),
                (false, Up | Floor) => (-Float::min_positive_value_prec(*precision), Less),
                (false, Ceiling | Down | Nearest) => (Float::NEGATIVE_ZERO, Greater),
                (_, Exact) => unreachable!(),
            }
        }
    } else {
        (x.clone(), Equal)
    }
}

fn shl_round_assign_primitive_int<T: PrimitiveInt>(
    x: &mut Float,
    bits: T,
    rm: RoundingMode,
) -> Ordering
where
    i32: WrappingFrom<T>,
{
    shl_prec_round_assign_helper(x, bits, x.significant_bits(), rm, Equal)
}

macro_rules! impl_natural_shl_round {
    ($t:ident) => {
        impl ShlRound<$t> for Float {
            type Output = Float;

            /// Left-shifts a [`Float`] (multiplies it by a power of 2), taking the [`Float`] by
            /// value.
            ///
            /// `NaN`, infinities, and zeros are unchanged. If the [`Float`] has a precision, the
            /// output has the same precision.
            ///
            /// $$
            /// f(x,k,m) = x2^k.
            /// $$
            ///
            /// - If $f(x,k,m)\geq 2^{2^{30}-1}$ and $m$ is `Ceiling`, `Up`, or `Nearest`, $\infty$
            ///   is returned instead.
            /// - If $f(x,k,m)\geq 2^{2^{30}-1}$ and $m$ is `Floor` or `Down`,
            ///   $(1-(1/2)^p)2^{2^{30}-1}$ is returned instead, where `p` is the precision of the
            ///   input.
            /// - If $f(x,k,m)\geq 2^{2^{30}-1}$ and $m$ is `Floor`, `Up`, or `Nearest`, $-\infty$
            ///   is returned instead.
            /// - If $f(x,k,m)\geq 2^{2^{30}-1}$ and $m$ is `Ceiling` or `Down`,
            ///   $-(1-(1/2)^p)2^{2^{30}-1}$ is returned instead, where `p` is the precision of the
            ///   input.
            /// - If $0<f(x,k,m)<2^{-2^{30}}$, and $m$ is `Floor` or `Down`, $0.0$ is returned
            ///   instead.
            /// - If $0<f(x,k,m)<2^{-2^{30}}$, and $m$ is `Ceiling` or `Up`, $2^{-2^{30}}$ is
            ///   returned instead.
            /// - If $0<f(x,k,m)\leq2^{-2^{30}-1}$, and $m$ is `Nearest`, $0.0$ is returned instead.
            /// - If $2^{-2^{30}-1}<f(x,k,m)<2^{-2^{30}}$, and $m$ is `Nearest`, $2^{-2^{30}}$ is
            ///   returned instead.
            /// - If $-2^{-2^{30}}<f(x,k,m)<0$, and $m$ is `Ceiling` or `Down`, $-0.0$ is returned
            ///   instead.
            /// - If $-2^{-2^{30}}<f(x,k,m)<0$, and $m$ is `Floor` or `Up`, $-2^{-2^{30}}$ is
            ///   returned instead.
            /// - If $-2^{-2^{30}-1}\leq f(x,k,m)<0$, and $m$ is `Nearest`, $-0.0$ is returned
            ///   instead.
            /// - If $-2^{-2^{30}}<f(x,k,m)<-2^{-2^{30}-1}$, and $m$ is `Nearest`, $-2^{-2^{30}}$ is
            ///   returned instead.
            ///
            /// If you don't care about overflow or underflow behavior, or only want the behavior of
            /// the `Nearest` rounding mode, you can just use `<<` instead.
            ///
            /// # Worst-case complexity
            /// $T(n) = O(n)$
            ///
            /// $M(n) = O(n)$
            ///
            /// where $T$ is time, $M$ is additional memory, and $n$ is `self.significant_bits()`.
            ///
            /// # Panics
            /// Panics if the result overflows or underflows and `rm` is `Exact`.
            ///
            /// # Examples
            /// See [here](super::shl_round#shl_round).
            #[inline]
            fn shl_round(mut self, bits: $t, rm: RoundingMode) -> (Float, Ordering) {
                let o = self.shl_round_assign(bits, rm);
                (self, o)
            }
        }

        impl ShlRound<$t> for &Float {
            type Output = Float;

            /// Left-shifts a [`Float`] (multiplies it by a power of 2), taking the [`Float`] by
            /// reference.
            ///
            /// `NaN`, infinities, and zeros are unchanged. If the [`Float`] has a precision, the
            /// output has the same precision.
            ///
            /// $$
            /// f(x,k,m) = x2^k.
            /// $$
            ///
            /// - If $f(x,k,m)\geq 2^{2^{30}-1}$ and $m$ is `Ceiling`, `Up`, or `Nearest`, $\infty$
            ///   is returned instead.
            /// - If $f(x,k,m)\geq 2^{2^{30}-1}$ and $m$ is `Floor` or `Down`,
            ///   $(1-(1/2)^p)2^{2^{30}-1}$ is returned instead, where `p` is the precision of the
            ///   input.
            /// - If $f(x,k,m)\geq 2^{2^{30}-1}$ and $m$ is `Floor`, `Up`, or `Nearest`, $-\infty$
            ///   is returned instead.
            /// - If $f(x,k,m)\geq 2^{2^{30}-1}$ and $m$ is `Ceiling` or `Down`,
            ///   $-(1-(1/2)^p)2^{2^{30}-1}$ is returned instead, where `p` is the precision of the
            ///   input.
            /// - If $0<f(x,k,m)<2^{-2^{30}}$, and $m$ is `Floor` or `Down`, $0.0$ is returned
            ///   instead.
            /// - If $0<f(x,k,m)<2^{-2^{30}}$, and $m$ is `Ceiling` or `Up`, $2^{-2^{30}}$ is
            ///   returned instead.
            /// - If $0<f(x,k,m)\leq2^{-2^{30}-1}$, and $m$ is `Nearest`, $0.0$ is returned instead.
            /// - If $2^{-2^{30}-1}<f(x,k,m)<2^{-2^{30}}$, and $m$ is `Nearest`, $2^{-2^{30}}$ is
            ///   returned instead.
            /// - If $-2^{-2^{30}}<f(x,k,m)<0$, and $m$ is `Ceiling` or `Down`, $-0.0$ is returned
            ///   instead.
            /// - If $-2^{-2^{30}}<f(x,k,m)<0$, and $m$ is `Floor` or `Up`, $-2^{-2^{30}}$ is
            ///   returned instead.
            /// - If $-2^{-2^{30}-1}\leq f(x,k,m)<0$, and $m$ is `Nearest`, $-0.0$ is returned
            ///   instead.
            /// - If $-2^{-2^{30}}<f(x,k,m)<-2^{-2^{30}-1}$, and $m$ is `Nearest`, $-2^{-2^{30}}$ is
            ///   returned instead.
            ///
            /// If you don't care about overflow or underflow behavior, or only want the behavior of
            /// the `Nearest` rounding mode, you can just use `<<` instead.
            ///
            /// # Worst-case complexity
            /// $T(n) = O(n)$
            ///
            /// $M(n) = O(n)$
            ///
            /// where $T$ is time, $M$ is additional memory, and $n$ is `self.significant_bits()`.
            ///
            /// # Panics
            /// Panics if the result overflows or underflows and `rm` is `Exact`.
            ///
            /// # Examples
            /// See [here](super::shl_round#shl_round).
            #[inline]
            fn shl_round(self, bits: $t, rm: RoundingMode) -> (Float, Ordering) {
                shl_round_primitive_int_ref(self, bits, rm)
            }
        }

        impl ShlRoundAssign<$t> for Float {
            /// Left-shifts a [`Float`] (multiplies it by a power of 2), in place.
            ///
            /// `NaN`, infinities, and zeros are unchanged. If the [`Float`] has a precision, the
            /// precision is unchanged.
            ///
            /// $$
            /// x\gets x2^k.
            /// $$
            ///
            /// - If $f(x,k,m)\geq 2^{2^{30}-1}$ and $m$ is `Ceiling`, `Up`, or `Nearest`, $\infty$
            ///   is returned instead.
            /// - If $f(x,k,m)\geq 2^{2^{30}-1}$ and $m$ is `Floor` or `Down`,
            ///   $(1-(1/2)^p)2^{2^{30}-1}$ is returned instead, where `p` is the precision of the
            ///   input.
            /// - If $f(x,k,m)\geq 2^{2^{30}-1}$ and $m$ is `Floor`, `Up`, or `Nearest`, $-\infty$
            ///   is returned instead.
            /// - If $f(x,k,m)\geq 2^{2^{30}-1}$ and $m$ is `Ceiling` or `Down`,
            ///   $-(1-(1/2)^p)2^{2^{30}-1}$ is returned instead, where `p` is the precision of the
            ///   input.
            /// - If $0<f(x,k,m)<2^{-2^{30}}$, and $m$ is `Floor` or `Down`, $0.0$ is returned
            ///   instead.
            /// - If $0<f(x,k,m)<2^{-2^{30}}$, and $m$ is `Ceiling` or `Up`, $2^{-2^{30}}$ is
            ///   returned instead.
            /// - If $0<f(x,k,m)\leq2^{-2^{30}-1}$, and $m$ is `Nearest`, $0.0$ is returned instead.
            /// - If $2^{-2^{30}-1}<f(x,k,m)<2^{-2^{30}}$, and $m$ is `Nearest`, $2^{-2^{30}}$ is
            ///   returned instead.
            /// - If $-2^{-2^{30}}<f(x,k,m)<0$, and $m$ is `Ceiling` or `Down`, $-0.0$ is returned
            ///   instead.
            /// - If $-2^{-2^{30}}<f(x,k,m)<0$, and $m$ is `Floor` or `Up`, $-2^{-2^{30}}$ is
            ///   returned instead.
            /// - If $-2^{-2^{30}-1}\leq f(x,k,m)<0$, and $m$ is `Nearest`, $-0.0$ is returned
            ///   instead.
            /// - If $-2^{-2^{30}}<f(x,k,m)<-2^{-2^{30}-1}$, and $m$ is `Nearest`, $-2^{-2^{30}}$ is
            ///   returned instead.
            ///
            /// If you don't care about overflow or underflow behavior, or only want the behavior of
            /// the `Nearest` rounding mode, you can just use `<<=` instead.
            ///
            /// # Worst-case complexity
            /// $T(n) = O(n)$
            ///
            /// $M(n) = O(n)$
            ///
            /// where $T$ is time, $M$ is additional memory, and $n$ is `self.significant_bits()`.
            ///
            /// # Panics
            /// Panics if the result overflows or underflows and `rm` is `Exact`.
            ///
            /// # Examples
            /// See [here](super::shl_round#shl_round_assign).
            #[inline]
            fn shl_round_assign(&mut self, bits: $t, rm: RoundingMode) -> Ordering {
                shl_round_assign_primitive_int(self, bits, rm)
            }
        }
    };
}
apply_to_primitive_ints!(impl_natural_shl_round);
