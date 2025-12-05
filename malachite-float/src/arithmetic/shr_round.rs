// Copyright © 2025 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::Float;
use crate::InnerFloat::Finite;
use core::cmp::Ordering::{self, *};
use malachite_base::num::arithmetic::traits::{IsPowerOf2, ShrRound, ShrRoundAssign};
use malachite_base::num::basic::integers::PrimitiveInt;
use malachite_base::num::basic::traits::{Infinity, NegativeInfinity, NegativeZero, Zero};
use malachite_base::num::conversion::traits::SaturatingInto;
use malachite_base::num::logic::traits::SignificantBits;
use malachite_base::rounding_modes::RoundingMode::{self, *};

impl Float {
    pub(crate) fn shr_prec_round_assign_helper<T: PrimitiveInt>(
        &mut self,
        bits: T,
        prec: u64,
        rm: RoundingMode,
        previous_o: Ordering,
    ) -> Ordering {
        if let Self(Finite {
            significand,
            exponent,
            sign,
            precision,
        }) = self
        {
            let mut possibly_just_under_min = false;
            if let Ok(bits) = bits.try_into()
                && let Some(new_exponent) = exponent.checked_sub(bits)
            {
                possibly_just_under_min = true;
                if (Self::MIN_EXPONENT..=Self::MAX_EXPONENT).contains(&new_exponent) {
                    *exponent = new_exponent;
                    return previous_o;
                }
            }
            assert!(rm != Exact, "Inexact Float right-shift");
            if bits < T::ZERO {
                match (*sign, rm) {
                    (true, Up | Ceiling | Nearest) => {
                        *self = Self::INFINITY;
                        Greater
                    }
                    (true, Floor | Down) => {
                        *self = Self::max_finite_value_with_prec(prec);
                        Less
                    }
                    (false, Up | Floor | Nearest) => {
                        *self = Self::NEGATIVE_INFINITY;
                        Less
                    }
                    (false, Ceiling | Down) => {
                        *self = -Self::max_finite_value_with_prec(prec);
                        Greater
                    }
                    (_, Exact) => unreachable!(),
                }
            } else if rm == Nearest
                && possibly_just_under_min
                && *exponent - <T as SaturatingInto<i32>>::saturating_into(bits)
                    == Self::MIN_EXPONENT - 1
                && (previous_o == if *sign { Less } else { Greater }
                    || !significand.is_power_of_2())
            {
                if *sign {
                    *self = Self::min_positive_value_prec(*precision);
                    Greater
                } else {
                    *self = -Self::min_positive_value_prec(*precision);
                    Less
                }
            } else {
                match (*sign, rm) {
                    (true, Up | Ceiling) => {
                        *self = Self::min_positive_value_prec(prec);
                        Greater
                    }
                    (true, Floor | Down | Nearest) => {
                        *self = Self::ZERO;
                        Less
                    }
                    (false, Up | Floor) => {
                        *self = -Self::min_positive_value_prec(prec);
                        Less
                    }
                    (false, Ceiling | Down | Nearest) => {
                        *self = Self::NEGATIVE_ZERO;
                        Greater
                    }
                    (_, Exact) => unreachable!(),
                }
            }
        } else {
            Equal
        }
    }

    /// Right-shifts a [`Float`] (divides it by a power of 2), rounding the result with the
    /// specified rounding mode and precision, and taking the [`Float`] by value.
    ///
    /// `NaN`, infinities, and zeros are unchanged. If the output has a precision, it is `prec`.
    ///
    /// $$
    /// f(x,k,p,m) = x/2^k.
    /// $$
    ///
    /// - If $f(x,k,p,m)\geq 2^{2^{30}-1}$ and $m$ is `Ceiling`, `Up`, or `Nearest`, $\infty$ is
    ///   returned instead.
    /// - If $f(x,k,p,m)\geq 2^{2^{30}-1}$ and $m$ is `Floor` or `Down`, $(1-(1/2)^p)2^{2^{30}-1}$
    ///   is returned instead, where `p` is the precision of the input.
    /// - If $f(x,k,p,m)\geq 2^{2^{30}-1}$ and $m$ is `Floor`, `Up`, or `Nearest`, $-\infty$ is
    ///   returned instead.
    /// - If $f(x,k,p,m)\geq 2^{2^{30}-1}$ and $m$ is `Ceiling` or `Down`,
    ///   $-(1-(1/2)^p)2^{2^{30}-1}$ is returned instead, where `p` is `prec`.
    /// - If $0<f(x,k,p,m)<2^{-2^{30}}$, and $m$ is `Floor` or `Down`, $0.0$ is returned instead.
    /// - If $0<f(x,k,p,m)<2^{-2^{30}}$, and $m$ is `Ceiling` or `Up`, $2^{-2^{30}}$ is returned
    ///   instead.
    /// - If $0<f(x,k,p,m)\leq2^{-2^{30}-1}$, and $m$ is `Nearest`, $0.0$ is returned instead.
    /// - If $2^{-2^{30}-1}<f(x,k,p,m)<2^{-2^{30}}$, and $m$ is `Nearest`, $2^{-2^{30}}$ is returned
    ///   instead.
    /// - If $-2^{-2^{30}}<f(x,k,p,m)<0$, and $m$ is `Ceiling` or `Down`, $-0.0$ is returned
    ///   instead.
    /// - If $-2^{-2^{30}}<f(x,k,p,m)<0$, and $m$ is `Floor` or `Up`, $-2^{-2^{30}}$ is returned
    ///   instead.
    /// - If $-2^{-2^{30}-1}\leq f(x,k,p,m)<0$, and $m$ is `Nearest`, $-0.0$ is returned instead.
    /// - If $-2^{-2^{30}}<f(x,k,p,m)<-2^{-2^{30}-1}$, and $m$ is `Nearest`, $-2^{-2^{30}}$ is
    ///   returned instead.
    ///
    /// If you know you'll be using `Nearest`, consider using [`Float::shr_prec`] instead. If you
    /// know that your target precision is the precision of the input, consider using
    /// [`Float::shr_round`] instead. If both of these things are true, or you don't care about
    /// overflow or underflow behavior, consider using `>>` instead.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `self.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `rm` is `Exact` and the result overflows or underflows, or cannot be expressed
    /// exactly with the specified precision.
    ///
    /// # Examples
    /// See [here](super::shr_round#shr_prec_round).
    pub fn shr_prec_round<T: PrimitiveInt>(
        mut self,
        bits: T,
        prec: u64,
        rm: RoundingMode,
    ) -> (Self, Ordering) {
        let o = self.shr_prec_round_assign(bits, prec, rm);
        (self, o)
    }

    /// Right-shifts a [`Float`] (divides it by a power of 2), rounding the result with the
    /// specified rounding mode and precision, and taking the [`Float`] by reference.
    ///
    /// `NaN`, infinities, and zeros are unchanged. If the output has a precision, it is `prec`.
    ///
    /// $$
    /// f(x,k,p,m) = x/2^k.
    /// $$
    ///
    /// - If $f(x,k,p,m)\geq 2^{2^{30}-1}$ and $m$ is `Ceiling`, `Up`, or `Nearest`, $\infty$ is
    ///   returned instead.
    /// - If $f(x,k,p,m)\geq 2^{2^{30}-1}$ and $m$ is `Floor` or `Down`, $(1-(1/2)^p)2^{2^{30}-1}$
    ///   is returned instead, where `p` is the precision of the input.
    /// - If $f(x,k,p,m)\geq 2^{2^{30}-1}$ and $m$ is `Floor`, `Up`, or `Nearest`, $-\infty$ is
    ///   returned instead.
    /// - If $f(x,k,p,m)\geq 2^{2^{30}-1}$ and $m$ is `Ceiling` or `Down`,
    ///   $-(1-(1/2)^p)2^{2^{30}-1}$ is returned instead, where `p` is `prec`.
    /// - If $0<f(x,k,p,m)<2^{-2^{30}}$, and $m$ is `Floor` or `Down`, $0.0$ is returned instead.
    /// - If $0<f(x,k,p,m)<2^{-2^{30}}$, and $m$ is `Ceiling` or `Up`, $2^{-2^{30}}$ is returned
    ///   instead.
    /// - If $0<f(x,k,p,m)\leq2^{-2^{30}-1}$, and $m$ is `Nearest`, $0.0$ is returned instead.
    /// - If $2^{-2^{30}-1}<f(x,k,p,m)<2^{-2^{30}}$, and $m$ is `Nearest`, $2^{-2^{30}}$ is returned
    ///   instead.
    /// - If $-2^{-2^{30}}<f(x,k,p,m)<0$, and $m$ is `Ceiling` or `Down`, $-0.0$ is returned
    ///   instead.
    /// - If $-2^{-2^{30}}<f(x,k,p,m)<0$, and $m$ is `Floor` or `Up`, $-2^{-2^{30}}$ is returned
    ///   instead.
    /// - If $-2^{-2^{30}-1}\leq f(x,k,p,m)<0$, and $m$ is `Nearest`, $-0.0$ is returned instead.
    /// - If $-2^{-2^{30}}<f(x,k,p,m)<-2^{-2^{30}-1}$, and $m$ is `Nearest`, $-2^{-2^{30}}$ is
    ///   returned instead.
    ///
    /// If you know you'll be using `Nearest`, consider using [`Float::shr_prec_ref`] instead. If
    /// you know that your target precision is the precision of the input, consider using
    /// [`Float::shr_round`] instead. If both of these things are true, or you don't care about
    /// overflow or underflow behavior, consider using `>>` instead.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `self.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `rm` is `Exact` and the result overflows or underflows, or cannot be expressed
    /// exactly with the specified precision.
    ///
    /// # Examples
    /// See [here](super::shr_round#shr_prec_round).
    pub fn shr_prec_round_ref<T: PrimitiveInt>(
        &self,
        bits: T,
        prec: u64,
        rm: RoundingMode,
    ) -> (Self, Ordering) {
        let mut x = self.clone();
        let o = x.shr_prec_round_assign(bits, prec, rm);
        (x, o)
    }

    /// Right-shifts a [`Float`] (divides it by a power of 2) in place, rounding the result with the
    /// specified rounding mode and precision.
    ///
    /// `NaN`, infinities, and zeros are unchanged. If the output has a precision, it is `prec`.
    ///
    /// $$
    /// x \gets x/2^k.
    /// $$
    ///
    /// - If $f(x,k,p,m)\geq 2^{2^{30}-1}$ and $m$ is `Ceiling`, `Up`, or `Nearest`, $\infty$ is
    ///   returned instead.
    /// - If $f(x,k,p,m)\geq 2^{2^{30}-1}$ and $m$ is `Floor` or `Down`, $(1-(1/2)^p)2^{2^{30}-1}$
    ///   is returned instead, where `p` is the precision of the input.
    /// - If $f(x,k,p,m)\geq 2^{2^{30}-1}$ and $m$ is `Floor`, `Up`, or `Nearest`, $-\infty$ is
    ///   returned instead.
    /// - If $f(x,k,p,m)\geq 2^{2^{30}-1}$ and $m$ is `Ceiling` or `Down`,
    ///   $-(1-(1/2)^p)2^{2^{30}-1}$ is returned instead, where `p` is `prec`.
    /// - If $0<f(x,k,p,m)<2^{-2^{30}}$, and $m$ is `Floor` or `Down`, $0.0$ is returned instead.
    /// - If $0<f(x,k,p,m)<2^{-2^{30}}$, and $m$ is `Ceiling` or `Up`, $2^{-2^{30}}$ is returned
    ///   instead.
    /// - If $0<f(x,k,p,m)\leq2^{-2^{30}-1}$, and $m$ is `Nearest`, $0.0$ is returned instead.
    /// - If $2^{-2^{30}-1}<f(x,k,p,m)<2^{-2^{30}}$, and $m$ is `Nearest`, $2^{-2^{30}}$ is returned
    ///   instead.
    /// - If $-2^{-2^{30}}<f(x,k,p,m)<0$, and $m$ is `Ceiling` or `Down`, $-0.0$ is returned
    ///   instead.
    /// - If $-2^{-2^{30}}<f(x,k,p,m)<0$, and $m$ is `Floor` or `Up`, $-2^{-2^{30}}$ is returned
    ///   instead.
    /// - If $-2^{-2^{30}-1}\leq f(x,k,p,m)<0$, and $m$ is `Nearest`, $-0.0$ is returned instead.
    /// - If $-2^{-2^{30}}<f(x,k,p,m)<-2^{-2^{30}-1}$, and $m$ is `Nearest`, $-2^{-2^{30}}$ is
    ///   returned instead.
    ///
    /// If you know you'll be using `Nearest`, consider using [`Float::shr_prec_assign`] instead. If
    /// you know that your target precision is the precision of the input, consider using
    /// [`Float::shr_round_assign`] instead. If both of these things are true, or you don't care
    /// about overflow or underflow behavior, consider using `>>=` instead.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `self.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `rm` is `Exact` and the result overflows or underflows, or cannot be expressed
    /// exactly with the specified precision.
    ///
    /// # Examples
    /// See [here](super::shr_round#shr_prec_round).
    pub fn shr_prec_round_assign<T: PrimitiveInt>(
        &mut self,
        bits: T,
        prec: u64,
        rm: RoundingMode,
    ) -> Ordering {
        if let Self(Finite { exponent, .. }) = self {
            let old_exponent = *exponent;
            *exponent = 0;
            let o = self.set_prec_round(prec, rm);
            self.shr_prec_round_assign_helper(
                <T as SaturatingInto<i32>>::saturating_into(bits).saturating_sub(old_exponent),
                prec,
                rm,
                o,
            )
        } else {
            Equal
        }
    }

    /// Right-shifts a [`Float`] (divides it by a power of 2), rounding the result with the
    /// specified precision, and taking the [`Float`] by value.
    ///
    /// `NaN`, infinities, and zeros are unchanged. If the output has a precision, it is `prec`.
    ///
    /// $$
    /// f(x,k,p) = x/2^k.
    /// $$
    ///
    /// - If $f(x,k,p)\geq 2^{2^{30}-1}$, $\infty$ is returned instead.
    /// - If $f(x,k,p)\geq 2^{2^{30}-1}$, $-\infty$ is returned instead.
    /// - If $0<f(x,k,p)\leq2^{-2^{30}-1}$, $0.0$ is returned instead.
    /// - If $2^{-2^{30}-1}<f(x,k,p)<2^{-2^{30}}$, $2^{-2^{30}}$ is returned instead.
    /// - If $-2^{-2^{30}-1}\leq f(x,k,p)<0$, $-0.0$ is returned instead.
    /// - If $-2^{-2^{30}}<f(x,k,p)<-2^{-2^{30}-1}$, $-2^{-2^{30}}$ is returned instead.
    ///
    /// If you know that your target precision is the precision of the input, or you don't care
    /// about overflow or underflow behavior, consider using `>>` instead.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `self.significant_bits()`.
    ///
    /// # Examples
    /// See [here](super::shr_round#shr_prec).
    #[inline]
    pub fn shr_prec<T: PrimitiveInt>(self, bits: T, prec: u64) -> (Self, Ordering) {
        self.shr_prec_round(bits, prec, Nearest)
    }

    /// Right-shifts a [`Float`] (divides it by a power of 2), rounding the result with the
    /// specified precision, and taking the [`Float`] by reference.
    ///
    /// `NaN`, infinities, and zeros are unchanged. If the output has a precision, it is `prec`.
    ///
    /// $$
    /// f(x,k,p) = x/2^k.
    /// $$
    ///
    /// - If $f(x,k,p)\geq 2^{2^{30}-1}$, $\infty$ is returned instead.
    /// - If $f(x,k,p)\geq 2^{2^{30}-1}$, $-\infty$ is returned instead.
    /// - If $0<f(x,k,p)\leq2^{-2^{30}-1}$, $0.0$ is returned instead.
    /// - If $2^{-2^{30}-1}<f(x,k,p)<2^{-2^{30}}$, $2^{-2^{30}}$ is returned instead.
    /// - If $-2^{-2^{30}-1}\leq f(x,k,p)<0$, $-0.0$ is returned instead.
    /// - If $-2^{-2^{30}}<f(x,k,p)<-2^{-2^{30}-1}$, $-2^{-2^{30}}$ is returned instead.
    ///
    /// If you know that your target precision is the precision of the input, or you don't care
    /// about overflow or underflow behavior, consider using `>>` instead.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `self.significant_bits()`.
    ///
    /// # Examples
    /// See [here](super::shr_round#shr_prec).
    #[inline]
    pub fn shr_prec_ref<T: PrimitiveInt>(&self, bits: T, prec: u64) -> (Self, Ordering) {
        self.shr_prec_round_ref(bits, prec, Nearest)
    }

    /// Right-shifts a [`Float`] (divides it by a power of 2) in place, rounding the result with the
    /// specified precision.
    ///
    /// `NaN`, infinities, and zeros are unchanged. If the output has a precision, it is `prec`.
    ///
    /// $$
    /// x \gets x/2^k.
    /// $$
    ///
    /// - If $f(x,k,p)\geq 2^{2^{30}-1}$, $\infty$ is returned instead.
    /// - If $f(x,k,p)\geq 2^{2^{30}-1}$, $-\infty$ is returned instead.
    /// - If $0<f(x,k,p)\leq2^{-2^{30}-1}$, $0.0$ is returned instead.
    /// - If $2^{-2^{30}-1}<f(x,k,p)<2^{-2^{30}}$, $2^{-2^{30}}$ is returned instead.
    /// - If $-2^{-2^{30}-1}\leq f(x,k,p)<0$, $-0.0$ is returned instead.
    /// - If $-2^{-2^{30}}<f(x,k,p)<-2^{-2^{30}-1}$, $-2^{-2^{30}}$ is returned instead.
    ///
    /// If you know that your target precision is the precision of the input, or you don't care
    /// about overflow or underflow behavior, consider using `>>=` instead.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `self.significant_bits()`.
    ///
    /// # Examples
    /// See [here](super::shr_round#shr_prec).
    #[inline]
    pub fn shr_prec_assign<T: PrimitiveInt>(&mut self, bits: T, prec: u64) -> Ordering {
        self.shr_prec_round_assign(bits, prec, Nearest)
    }
}

fn shr_round_primitive_int_ref<T: PrimitiveInt>(
    x: &Float,
    bits: T,
    rm: RoundingMode,
) -> (Float, Ordering) {
    if let Float(Finite {
        significand,
        exponent,
        sign,
        precision,
    }) = x
    {
        let mut possibly_just_under_min = false;
        if let Ok(bits) = bits.try_into()
            && let Some(new_exponent) = exponent.checked_sub(bits)
        {
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
        assert!(rm != Exact, "Inexact Float right-shift");
        if bits < T::ZERO {
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
            && *exponent - <T as SaturatingInto<i32>>::saturating_into(bits)
                == Float::MIN_EXPONENT - 1
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

fn shr_round_assign_primitive_int<T: PrimitiveInt>(
    x: &mut Float,
    bits: T,
    rm: RoundingMode,
) -> Ordering {
    x.shr_prec_round_assign_helper(bits, x.significant_bits(), rm, Equal)
}

macro_rules! impl_natural_shr_round {
    ($t:ident) => {
        impl ShrRound<$t> for Float {
            type Output = Float;

            /// Right-shifts a [`Float`] (divides it by a power of 2), rounding the result with the
            /// specified rounding mode and taking the [`Float`] by value.
            ///
            /// `NaN`, infinities, and zeros are unchanged. If the [`Float`] has a precision, the
            /// output has the same precision.
            ///
            /// $$
            /// f(x,k,m) = x/2^k.
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
            /// the `Nearest` rounding mode, you can just use `>>` instead.
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
            /// See [here](super::shr_round#shr_round).
            #[inline]
            fn shr_round(mut self, bits: $t, rm: RoundingMode) -> (Float, Ordering) {
                let o = self.shr_round_assign(bits, rm);
                (self, o)
            }
        }

        impl ShrRound<$t> for &Float {
            type Output = Float;

            /// Right-shifts a [`Float`] (divides it by a power of 2), rounding the result with the
            /// specified rounding mode and taking the [`Float`] by reference.
            ///
            /// `NaN`, infinities, and zeros are unchanged. If the [`Float`] has a precision, the
            /// output has the same precision.
            ///
            /// $$
            /// f(x,k,m) = x/2^k.
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
            /// the `Nearest` rounding mode, you can just use `>>` instead.
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
            /// See [here](super::shr_round#shr_round).
            #[inline]
            fn shr_round(self, bits: $t, rm: RoundingMode) -> (Float, Ordering) {
                shr_round_primitive_int_ref(self, bits, rm)
            }
        }

        impl ShrRoundAssign<$t> for Float {
            /// Right-shifts a [`Float`] (divides it by a power of 2), in place, rounding the result
            /// with the specified rounding mode.
            ///
            /// `NaN`, infinities, and zeros are unchanged. If the [`Float`] has a precision, the
            /// precision is unchanged.
            ///
            /// $$
            /// x\gets x/2^k.
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
            /// the `Nearest` rounding mode, you can just use `>>=` instead.
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
            /// See [here](super::shr_round#shr_round_assign).
            #[inline]
            fn shr_round_assign(&mut self, bits: $t, rm: RoundingMode) -> Ordering {
                shr_round_assign_primitive_int(self, bits, rm)
            }
        }
    };
}
apply_to_primitive_ints!(impl_natural_shr_round);
