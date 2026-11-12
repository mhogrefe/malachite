// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::big_exponent_float::BigExponentFloat;
use crate::big_exponent_float::InnerBigExponentFloat::{Finite, Infinity, NaN, Zero};
use crate::big_exponent_float::arithmetic::add::{finish_significand, kernel_prec, round_optional};
use alloc::borrow::Cow;
use core::cmp::Ordering::{self, *};
use malachite_base::num::arithmetic::traits::{Square, SquareAssign};
use malachite_base::num::basic::traits::{
    Infinity as InfinityTrait, NaN as NaNTrait, Zero as ZeroTrait,
};
use malachite_base::num::logic::traits::SignificantBits;
use malachite_base::rounding_modes::RoundingMode::{self, *};
use malachite_nz::integer::Integer;
use malachite_nz::natural::arithmetic::float::square::{
    square_float_significand_in_place, square_float_significand_ref,
};

// x^2, rounded with `prec_rm` if it is `Some`, and exact otherwise.
pub(crate) fn square_helper(
    x: Cow<'_, BigExponentFloat>,
    prec_rm: Option<(u64, RoundingMode)>,
) -> (BigExponentFloat, Ordering) {
    square_helper_with(x, prec_rm, true)
}

crate_test_fn! {
    // x^2 rounded to `prec` bits with `rm`, without `Float`'s kernels: the exact square of the
    // significand is formed with `Natural` arithmetic and then rounded. Used to check and benchmark
    // the kernel path.
    square_prec_round_generic(
        x: &BigExponentFloat,
        prec: u64,
        rm: RoundingMode,
    ) -> (BigExponentFloat, Ordering) {
        square_helper_with(Cow::Borrowed(x), Some((prec, rm)), false)
    }
}

// With `use_kernels`, the square is computed by `Float`'s squaring kernels, which have fast paths
// for operands and outputs of one, two, or three limbs; the output exponent is an offset from twice
// the operand's exponent. An exact square is the kernels' square at twice the operand's precision,
// which always suffices. When the operand is owned, its significand's memory holds the square.
fn square_helper_with(
    x: Cow<'_, BigExponentFloat>,
    prec_rm: Option<(u64, RoundingMode)>,
    use_kernels: bool,
) -> (BigExponentFloat, Ordering) {
    if let Some((prec, _)) = prec_rm {
        assert_ne!(prec, 0);
    }
    match x {
        Cow::Owned(BigExponentFloat(NaN)) | Cow::Borrowed(BigExponentFloat(NaN)) => {
            (BigExponentFloat::NAN, Equal)
        }
        Cow::Owned(BigExponentFloat(Infinity { .. }))
        | Cow::Borrowed(BigExponentFloat(Infinity { .. })) => (BigExponentFloat::INFINITY, Equal),
        Cow::Owned(BigExponentFloat(Zero)) | Cow::Borrowed(BigExponentFloat(Zero)) => {
            (BigExponentFloat::ZERO, Equal)
        }
        Cow::Owned(BigExponentFloat(Finite {
            exponent,
            significand: mut square,
            ..
        })) if use_kernels => {
            let x_prec = kernel_prec(&square, prec_rm.map(|(p, _)| p));
            let (prec, rm) = prec_rm.unwrap_or((x_prec << 1, Exact));
            let (exp_offset, o) = square_float_significand_in_place(&mut square, x_prec, prec, rm);
            (
                BigExponentFloat(Finite {
                    sign: true,
                    exponent: (exponent << 1u32) + Integer::from(exp_offset),
                    significand: finish_significand(square),
                }),
                o,
            )
        }
        Cow::Borrowed(BigExponentFloat(Finite {
            exponent,
            significand,
            ..
        })) if use_kernels => {
            let x_prec = kernel_prec(significand, prec_rm.map(|(p, _)| p));
            let (prec, rm) = prec_rm.unwrap_or((x_prec << 1, Exact));
            let (square, exp_offset, o) =
                square_float_significand_ref(significand, x_prec, prec, rm);
            (
                BigExponentFloat(Finite {
                    sign: true,
                    exponent: (exponent << 1u32) + Integer::from(exp_offset),
                    significand: finish_significand(square),
                }),
                o,
            )
        }
        x => {
            let BigExponentFloat(Finite {
                exponent,
                significand,
                ..
            }) = &*x
            else {
                unreachable!()
            };
            let low = (exponent - Integer::from(significand.significant_bits())) << 1u32;
            let m = significand.square();
            let exponent = low + Integer::from(m.significant_bits());
            round_optional(BigExponentFloat::from_parts(true, exponent, m), prec_rm)
        }
    }
}

impl BigExponentFloat {
    /// Squares a [`BigExponentFloat`], rounding the result to the specified precision and with the
    /// specified rounding mode. The [`BigExponentFloat`] is taken by value. An [`Ordering`] is also
    /// returned, indicating whether the rounded square is less than, equal to, or greater than the
    /// exact square. Although `NaN`s are not comparable to any [`BigExponentFloat`], whenever this
    /// function returns a `NaN` it also returns `Equal`.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// f(x,p,m) = x^2+\varepsilon.
    /// $$
    /// - If $x^2$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $x^2$ is finite and nonzero, and $m$ is not `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 x^2\rfloor-p+1}$.
    /// - If $x^2$ is finite and nonzero, and $m$ is `Nearest`, then $|\varepsilon| \leq
    ///   2^{\lfloor\log_2 x^2\rfloor-p}$.
    ///
    /// Special cases:
    /// - $f(\text{NaN},p,m)=\text{NaN}$
    /// - $f(\pm\infty,p,m)=\infty$
    /// - $f(0,p,m)=0$
    ///
    /// If you know you'll be using `Nearest`, consider using [`BigExponentFloat::square_prec`]
    /// instead. If you want the exact square, consider using [`BigExponentFloat::square`] instead.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n + m \log m \log\log m)$
    ///
    /// $M(n, m) = O(n + m \log m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `prec`, and $m$ is
    /// `self.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `prec` is zero, or if `rm` is `Exact` but `prec` is too small for an exact
    /// squaring.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::PI;
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::BigExponentFloat;
    /// use std::cmp::Ordering::*;
    ///
    /// let (square, o) = BigExponentFloat::from(PI).square_prec_round(5, Floor);
    /// assert_eq!(format!("{square:?}"), "0x9.8");
    /// assert_eq!(o, Less);
    ///
    /// let (square, o) = BigExponentFloat::from(PI).square_prec_round(5, Ceiling);
    /// assert_eq!(format!("{square:?}"), "0xa.0");
    /// assert_eq!(o, Greater);
    ///
    /// let (square, o) = BigExponentFloat::from(PI).square_prec_round(5, Nearest);
    /// assert_eq!(format!("{square:?}"), "0xa.0");
    /// assert_eq!(o, Greater);
    ///
    /// let (square, o) = BigExponentFloat::from(PI).square_prec_round(20, Floor);
    /// assert_eq!(format!("{square:?}"), "0x9.de9e");
    /// assert_eq!(o, Less);
    ///
    /// let (square, o) = BigExponentFloat::from(PI).square_prec_round(20, Ceiling);
    /// assert_eq!(format!("{square:?}"), "0x9.de9f");
    /// assert_eq!(o, Greater);
    ///
    /// let (square, o) = BigExponentFloat::from(PI).square_prec_round(20, Nearest);
    /// assert_eq!(format!("{square:?}"), "0x9.de9e");
    /// assert_eq!(o, Less);
    /// ```
    #[inline]
    pub fn square_prec_round(self, prec: u64, rm: RoundingMode) -> (Self, Ordering) {
        square_helper(Cow::Owned(self), Some((prec, rm)))
    }

    /// Squares a [`BigExponentFloat`], rounding the result to the specified precision and with the
    /// specified rounding mode. The [`BigExponentFloat`] is taken by reference. An [`Ordering`] is
    /// also returned, indicating whether the rounded square is less than, equal to, or greater than
    /// the exact square. Although `NaN`s are not comparable to any [`BigExponentFloat`], whenever
    /// this function returns a `NaN` it also returns `Equal`.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// f(x,p,m) = x^2+\varepsilon.
    /// $$
    /// - If $x^2$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $x^2$ is finite and nonzero, and $m$ is not `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 x^2\rfloor-p+1}$.
    /// - If $x^2$ is finite and nonzero, and $m$ is `Nearest`, then $|\varepsilon| \leq
    ///   2^{\lfloor\log_2 x^2\rfloor-p}$.
    ///
    /// Special cases:
    /// - $f(\text{NaN},p,m)=\text{NaN}$
    /// - $f(\pm\infty,p,m)=\infty$
    /// - $f(0,p,m)=0$
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n + m \log m \log\log m)$
    ///
    /// $M(n, m) = O(n + m \log m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `prec`, and $m$ is
    /// `self.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `prec` is zero, or if `rm` is `Exact` but `prec` is too small for an exact
    /// squaring.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::PI;
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::BigExponentFloat;
    /// use std::cmp::Ordering::*;
    ///
    /// let (square, o) = (&BigExponentFloat::from(PI)).square_prec_round_ref(5, Floor);
    /// assert_eq!(format!("{square:?}"), "0x9.8");
    /// assert_eq!(o, Less);
    ///
    /// let (square, o) = (&BigExponentFloat::from(PI)).square_prec_round_ref(5, Ceiling);
    /// assert_eq!(format!("{square:?}"), "0xa.0");
    /// assert_eq!(o, Greater);
    ///
    /// let (square, o) = (&BigExponentFloat::from(PI)).square_prec_round_ref(5, Nearest);
    /// assert_eq!(format!("{square:?}"), "0xa.0");
    /// assert_eq!(o, Greater);
    ///
    /// let (square, o) = (&BigExponentFloat::from(PI)).square_prec_round_ref(20, Floor);
    /// assert_eq!(format!("{square:?}"), "0x9.de9e");
    /// assert_eq!(o, Less);
    ///
    /// let (square, o) = (&BigExponentFloat::from(PI)).square_prec_round_ref(20, Ceiling);
    /// assert_eq!(format!("{square:?}"), "0x9.de9f");
    /// assert_eq!(o, Greater);
    ///
    /// let (square, o) = (&BigExponentFloat::from(PI)).square_prec_round_ref(20, Nearest);
    /// assert_eq!(format!("{square:?}"), "0x9.de9e");
    /// assert_eq!(o, Less);
    /// ```
    #[inline]
    pub fn square_prec_round_ref(&self, prec: u64, rm: RoundingMode) -> (Self, Ordering) {
        square_helper(Cow::Borrowed(self), Some((prec, rm)))
    }

    /// Squares a [`BigExponentFloat`] in place, rounding the result to the specified precision and
    /// with the specified rounding mode. An [`Ordering`] is returned, indicating whether the
    /// rounded square is less than, equal to, or greater than the exact square. Although `NaN`s are
    /// not comparable to any [`BigExponentFloat`], whenever this function sets the
    /// [`BigExponentFloat`] to `NaN` it also returns `Equal`.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// x \gets x^2+\varepsilon.
    /// $$
    /// - If $x^2$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $x^2$ is finite and nonzero, and $m$ is not `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 x^2\rfloor-p+1}$.
    /// - If $x^2$ is finite and nonzero, and $m$ is `Nearest`, then $|\varepsilon| \leq
    ///   2^{\lfloor\log_2 x^2\rfloor-p}$.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n + m \log m \log\log m)$
    ///
    /// $M(n, m) = O(n + m \log m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `prec`, and $m$ is
    /// `self.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `prec` is zero, or if `rm` is `Exact` but `prec` is too small for an exact
    /// squaring.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::PI;
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::BigExponentFloat;
    /// use std::cmp::Ordering::*;
    ///
    /// let mut x = BigExponentFloat::from(PI);
    /// assert_eq!(x.square_prec_round_assign(5, Floor), Less);
    /// assert_eq!(format!("{x:?}"), "0x9.8");
    ///
    /// let mut x = BigExponentFloat::from(PI);
    /// assert_eq!(x.square_prec_round_assign(5, Ceiling), Greater);
    /// assert_eq!(format!("{x:?}"), "0xa.0");
    ///
    /// let mut x = BigExponentFloat::from(PI);
    /// assert_eq!(x.square_prec_round_assign(5, Nearest), Greater);
    /// assert_eq!(format!("{x:?}"), "0xa.0");
    ///
    /// let mut x = BigExponentFloat::from(PI);
    /// assert_eq!(x.square_prec_round_assign(20, Floor), Less);
    /// assert_eq!(format!("{x:?}"), "0x9.de9e");
    ///
    /// let mut x = BigExponentFloat::from(PI);
    /// assert_eq!(x.square_prec_round_assign(20, Ceiling), Greater);
    /// assert_eq!(format!("{x:?}"), "0x9.de9f");
    ///
    /// let mut x = BigExponentFloat::from(PI);
    /// assert_eq!(x.square_prec_round_assign(20, Nearest), Less);
    /// assert_eq!(format!("{x:?}"), "0x9.de9e");
    /// ```
    #[inline]
    pub fn square_prec_round_assign(&mut self, prec: u64, rm: RoundingMode) -> Ordering {
        let x = core::mem::replace(self, Self::ZERO);
        let (result, o) = square_helper(Cow::Owned(x), Some((prec, rm)));
        *self = result;
        o
    }

    /// Squares a [`BigExponentFloat`], rounding the result to the nearest value of the specified
    /// precision. The [`BigExponentFloat`] is taken by value. An [`Ordering`] is also returned,
    /// indicating whether the rounded square is less than, equal to, or greater than the exact
    /// square. Although `NaN`s are not comparable to any [`BigExponentFloat`], whenever this
    /// function returns a `NaN` it also returns `Equal`.
    ///
    /// $$
    /// f(x,p) = x^2+\varepsilon.
    /// $$
    /// - If $x^2$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $x^2$ is finite and nonzero, then $|\varepsilon| \leq 2^{\lfloor\log_2 x^2\rfloor-p}$.
    ///
    /// Special cases:
    /// - $f(\text{NaN},p)=\text{NaN}$
    /// - $f(\pm\infty,p)=\infty$
    /// - $f(0,p)=0$
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`BigExponentFloat::square_prec_round`] instead. If you want the exact square, consider
    /// using [`BigExponentFloat::square`] instead.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n + m \log m \log\log m)$
    ///
    /// $M(n, m) = O(n + m \log m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `prec`, and $m$ is
    /// `self.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `prec` is zero.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::PI;
    /// use malachite_float::BigExponentFloat;
    /// use std::cmp::Ordering::*;
    ///
    /// let (square, o) = BigExponentFloat::from(PI).square_prec(5);
    /// assert_eq!(format!("{square:?}"), "0xa.0");
    /// assert_eq!(o, Greater);
    ///
    /// let (square, o) = BigExponentFloat::from(PI).square_prec(20);
    /// assert_eq!(format!("{square:?}"), "0x9.de9e");
    /// assert_eq!(o, Less);
    /// ```
    #[inline]
    pub fn square_prec(self, prec: u64) -> (Self, Ordering) {
        square_helper(Cow::Owned(self), Some((prec, Nearest)))
    }

    /// Squares a [`BigExponentFloat`], rounding the result to the nearest value of the specified
    /// precision. The [`BigExponentFloat`] is taken by reference. An [`Ordering`] is also returned,
    /// indicating whether the rounded square is less than, equal to, or greater than the exact
    /// square. Although `NaN`s are not comparable to any [`BigExponentFloat`], whenever this
    /// function returns a `NaN` it also returns `Equal`.
    ///
    /// $$
    /// f(x,p) = x^2+\varepsilon.
    /// $$
    /// - If $x^2$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $x^2$ is finite and nonzero, then $|\varepsilon| \leq 2^{\lfloor\log_2 x^2\rfloor-p}$.
    ///
    /// Special cases:
    /// - $f(\text{NaN},p)=\text{NaN}$
    /// - $f(\pm\infty,p)=\infty$
    /// - $f(0,p)=0$
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n + m \log m \log\log m)$
    ///
    /// $M(n, m) = O(n + m \log m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `prec`, and $m$ is
    /// `self.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `prec` is zero.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::PI;
    /// use malachite_float::BigExponentFloat;
    /// use std::cmp::Ordering::*;
    ///
    /// let (square, o) = (&BigExponentFloat::from(PI)).square_prec_ref(5);
    /// assert_eq!(format!("{square:?}"), "0xa.0");
    /// assert_eq!(o, Greater);
    ///
    /// let (square, o) = (&BigExponentFloat::from(PI)).square_prec_ref(20);
    /// assert_eq!(format!("{square:?}"), "0x9.de9e");
    /// assert_eq!(o, Less);
    /// ```
    #[inline]
    pub fn square_prec_ref(&self, prec: u64) -> (Self, Ordering) {
        square_helper(Cow::Borrowed(self), Some((prec, Nearest)))
    }

    /// Squares a [`BigExponentFloat`] in place, rounding the result to the nearest value of the
    /// specified precision. An [`Ordering`] is returned, indicating whether the rounded square is
    /// less than, equal to, or greater than the exact square. Although `NaN`s are not comparable to
    /// any [`BigExponentFloat`], whenever this function sets the [`BigExponentFloat`] to `NaN` it
    /// also returns `Equal`.
    ///
    /// $$
    /// x \gets x^2+\varepsilon.
    /// $$
    /// - If $x^2$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $x^2$ is finite and nonzero, then $|\varepsilon| \leq 2^{\lfloor\log_2 x^2\rfloor-p}$.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n + m \log m \log\log m)$
    ///
    /// $M(n, m) = O(n + m \log m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `prec`, and $m$ is
    /// `self.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `prec` is zero.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::PI;
    /// use malachite_float::BigExponentFloat;
    /// use std::cmp::Ordering::*;
    ///
    /// let mut x = BigExponentFloat::from(PI);
    /// assert_eq!(x.square_prec_assign(5), Greater);
    /// assert_eq!(format!("{x:?}"), "0xa.0");
    ///
    /// let mut x = BigExponentFloat::from(PI);
    /// assert_eq!(x.square_prec_assign(20), Less);
    /// assert_eq!(format!("{x:?}"), "0x9.de9e");
    /// ```
    #[inline]
    pub fn square_prec_assign(&mut self, prec: u64) -> Ordering {
        let x = core::mem::replace(self, Self::ZERO);
        let (result, o) = square_helper(Cow::Owned(x), Some((prec, Nearest)));
        *self = result;
        o
    }
}

impl Square for BigExponentFloat {
    type Output = Self;

    /// Squares a [`BigExponentFloat`], taking it by value. The square is exact.
    ///
    /// $$
    /// f(x) = x^2.
    /// $$
    ///
    /// Special cases:
    /// - $f(\text{NaN})=\text{NaN}$
    /// - $f(\pm\infty)=\infty$
    /// - $f(0)=0$
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `self.significant_bits()`.
    ///
    /// This is FLINT's `arf_mul` with both inputs the same and precision `ARF_PREC_EXACT`.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::Square;
    /// use malachite_base::num::basic::traits::{Infinity, NegativeInfinity};
    /// use malachite_float::BigExponentFloat;
    ///
    /// assert_eq!(
    ///     format!("{:?}", BigExponentFloat::from(-1.5).square()),
    ///     "0x2.4"
    /// );
    /// assert_eq!(
    ///     BigExponentFloat::NEGATIVE_INFINITY.square(),
    ///     BigExponentFloat::INFINITY
    /// );
    /// ```
    #[inline]
    fn square(self) -> Self {
        square_helper(Cow::Owned(self), None).0
    }
}

impl Square for &BigExponentFloat {
    type Output = BigExponentFloat;

    /// Squares a [`BigExponentFloat`], taking it by reference. The square is exact.
    ///
    /// $$
    /// f(x) = x^2.
    /// $$
    ///
    /// Special cases:
    /// - $f(\text{NaN})=\text{NaN}$
    /// - $f(\pm\infty)=\infty$
    /// - $f(0)=0$
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `self.significant_bits()`.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::Square;
    /// use malachite_base::num::basic::traits::{Infinity, NegativeInfinity};
    /// use malachite_float::BigExponentFloat;
    ///
    /// assert_eq!(
    ///     format!("{:?}", (&BigExponentFloat::from(-1.5)).square()),
    ///     "0x2.4"
    /// );
    /// assert_eq!(
    ///     (&BigExponentFloat::NEGATIVE_INFINITY).square(),
    ///     BigExponentFloat::INFINITY
    /// );
    /// ```
    #[inline]
    fn square(self) -> BigExponentFloat {
        square_helper(Cow::Borrowed(self), None).0
    }
}

impl SquareAssign for BigExponentFloat {
    /// Squares a [`BigExponentFloat`] in place. The square is exact.
    ///
    /// $$
    /// x \gets x^2.
    /// $$
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `self.significant_bits()`.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::SquareAssign;
    /// use malachite_float::BigExponentFloat;
    ///
    /// let mut x = BigExponentFloat::from(-1.5);
    /// x.square_assign();
    /// assert_eq!(format!("{x:?}"), "0x2.4");
    /// ```
    #[inline]
    fn square_assign(&mut self) {
        let x = core::mem::replace(self, Self::ZERO);
        *self = square_helper(Cow::Owned(x), None).0;
    }
}
