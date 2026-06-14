// Copyright © 2026 Mikhail Hogrefe
//
// Uses code adopted from the FLINT Library.
//
//      Copyright (C) 2011 Fredrik Johansson
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::Rational;
use core::cmp::Ordering::*;
use malachite_base::num::arithmetic::traits::{
    CeilingLogBase, CeilingLogBasePowerOf2, CheckedLogBase, CheckedLogBase2,
    CheckedLogBasePowerOf2, FloorLogBase, FloorLogBasePowerOf2, Pow,
};
use malachite_base::num::comparison::traits::OrdAbs;
use malachite_base::num::conversion::traits::{ExactFrom, RoundingFrom, SciMantissaAndExponent};
use malachite_base::rounding_modes::RoundingMode::*;
use malachite_nz::natural::Natural;

fn approx_log_helper(x: &Rational) -> f64 {
    let (mantissa, exponent): (f64, i64) = x.sci_mantissa_and_exponent();
    mantissa.ln() + (exponent as f64) * core::f64::consts::LN_2
}

impl Rational {
    /// Calculates the approximate natural logarithm of a positive [`Rational`].
    ///
    /// $f(x) = (1+\varepsilon)(\log x)$, where $|\varepsilon| < 2^{-52}.$
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(1)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `self.significant_bits()`.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::{Pow, PowerOf2};
    /// use malachite_base::num::float::NiceFloat;
    /// use malachite_q::Rational;
    ///
    /// assert_eq!(
    ///     NiceFloat(Rational::from(10i32).approx_log()),
    ///     NiceFloat(2.3025850929940455)
    /// );
    /// assert_eq!(
    ///     NiceFloat(Rational::from(10i32).pow(100u64).approx_log()),
    ///     NiceFloat(230.25850929940455)
    /// );
    /// assert_eq!(
    ///     NiceFloat(Rational::power_of_2(1000000u64).approx_log()),
    ///     NiceFloat(693147.1805599453)
    /// );
    /// assert_eq!(
    ///     NiceFloat(Rational::power_of_2(-1000000i64).approx_log()),
    ///     NiceFloat(-693147.1805599453)
    /// );
    /// ```
    ///
    /// This is equivalent to `fmpz_dlog` from `fmpz/dlog.c`, FLINT 2.7.1.
    #[inline]
    pub fn approx_log(&self) -> f64 {
        assert!(*self > 0u32);
        approx_log_helper(self)
    }
}

// # Worst-case complexity
// $T(n, m) = O(nm \log (nm) \log\log (nm))$
//
// $M(n, m) = O(nm \log (nm))$
//
// where $T$ is time, $M$ is additional memory, $n$ is `base.significant_bits()`, and $m$ is
// $|\log_b x|$, where $b$ is `base` and $x$ is `x`.
pub(crate) fn log_base_helper(x: &Rational, base: &Rational) -> (i64, bool) {
    assert!(*base > 0u32);
    assert_ne!(*base, 1u32);
    if *x == 1u32 {
        return (0, true);
    }
    let mut log = i64::rounding_from(approx_log_helper(x) / approx_log_helper(base), Floor).0;
    let mut power = base.pow(log);
    if *base > 1u32 {
        match power.cmp_abs(x) {
            Equal => (log, true),
            Less => loop {
                power *= base;
                match power.cmp_abs(x) {
                    Equal => {
                        return (log + 1, true);
                    }
                    Less => {
                        log += 1;
                    }
                    Greater => {
                        return (log, false);
                    }
                }
            },
            Greater => loop {
                power /= base;
                match power.cmp_abs(x) {
                    Equal => {
                        return (log - 1, true);
                    }
                    Less => {
                        return (log - 1, false);
                    }
                    Greater => {
                        log -= 1;
                    }
                }
            },
        }
    } else {
        match power.cmp_abs(x) {
            Equal => (log, true),
            Less => loop {
                power /= base;
                match power.cmp_abs(x) {
                    Equal => {
                        return (log - 1, true);
                    }
                    Less => {
                        log -= 1;
                    }
                    Greater => {
                        return (log - 1, false);
                    }
                }
            },
            Greater => loop {
                power *= base;
                match power.cmp_abs(x) {
                    Equal => {
                        return (log + 1, true);
                    }
                    Less => {
                        return (log, false);
                    }
                    Greater => {
                        log += 1;
                    }
                }
            },
        }
    }
}

// Specialization of `log_base_helper` for an integer base greater than 1. Writing `x = n / d` in
// lowest terms, $\log_b(n/d) = \log_b n - \log_b d$, so $\lfloor\log_b(n/d)\rfloor$ is either
// $\lfloor\log_b n\rfloor - \lfloor\log_b d\rfloor$ or one less. Reusing the integer
// `FloorLogBase`, write $n = b^{\lfloor\log_b n\rfloor} a$ and $d = b^{\lfloor\log_b d\rfloor} c$
// with $a, c \in [1, b)$; the larger candidate is correct iff $a \geq c$, and the logarithm is
// exact iff $a = c$. The two are compared after clearing the common power of $b$, so the operands
// are no larger than `x` itself.
//
// # Worst-case complexity
// $T(n) = O(n \log n \log\log n)$
//
// $M(n) = O(n \log n)$
//
// where $T$ is time, $M$ is additional memory, and $n$ is `x.significant_bits()`.
fn log_base_u64_helper(x: &Rational, base: u64) -> (i64, bool) {
    let n = x.numerator_ref();
    let d = x.denominator_ref();
    let base = Natural::from(base);
    let log = i64::exact_from(n.floor_log_base(&base)) - i64::exact_from(d.floor_log_base(&base));
    let cmp = if log >= 0 {
        n.cmp(&(d * (&base).pow(u64::exact_from(log))))
    } else {
        (n * (&base).pow(u64::exact_from(-log))).cmp(d)
    };
    match cmp {
        Greater => (log, false),
        Equal => (log, true),
        Less => (log - 1, false),
    }
}

impl FloorLogBase<&Rational> for &Rational {
    type Output = i64;

    /// Returns the floor of the base-$b$ logarithm of a positive [`Rational`].
    ///
    /// Note that this function may be slow if the base is very close to 1.
    ///
    /// $f(x, b) = \lfloor\log_b x\rfloor$.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(nm \log (nm) \log\log (nm))$
    ///
    /// $M(n, m) = O(nm \log (nm))$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `base.significant_bits()`, and $m$ is
    /// $|\log_b x|$, where $b$ is `base` and $x$ is `x`.
    ///
    /// # Panics
    /// Panics if `self` less than or equal to zero or `base` is 1.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::FloorLogBase;
    /// use malachite_q::Rational;
    ///
    /// assert_eq!(
    ///     Rational::from(80u32).floor_log_base(&Rational::from(3u32)),
    ///     3
    /// );
    /// assert_eq!(
    ///     Rational::from(81u32).floor_log_base(&Rational::from(3u32)),
    ///     4
    /// );
    /// assert_eq!(
    ///     Rational::from(82u32).floor_log_base(&Rational::from(3u32)),
    ///     4
    /// );
    /// assert_eq!(
    ///     Rational::from(4294967296u64).floor_log_base(&Rational::from(10u32)),
    ///     9
    /// );
    /// assert_eq!(
    ///     Rational::from_signeds(936851431250i64, 1397).floor_log_base(&Rational::from(10u32)),
    ///     8
    /// );
    /// assert_eq!(
    ///     Rational::from_signeds(5153632, 16807).floor_log_base(&Rational::from_signeds(22, 7)),
    ///     5
    /// );
    /// ```
    fn floor_log_base(self, base: &Rational) -> i64 {
        assert!(*self > 0u32);
        if let Some(log_base) = base.checked_log_base_2() {
            return self.floor_log_base_power_of_2(log_base);
        }
        log_base_helper(self, base).0
    }
}

impl CeilingLogBase<&Rational> for &Rational {
    type Output = i64;

    /// Returns the ceiling of the base-$b$ logarithm of a positive [`Rational`].
    ///
    /// Note that this function may be slow if the base is very close to 1.
    ///
    /// $f(x, b) = \lceil\log_b x\rceil$.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(nm \log (nm) \log\log (nm))$
    ///
    /// $M(n, m) = O(nm \log (nm))$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `base.significant_bits()`, and $m$ is
    /// $|\log_b x|$, where $b$ is `base` and $x$ is `x`.
    ///
    /// # Panics
    /// Panics if `self` less than or equal to zero or `base` is 1.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::CeilingLogBase;
    /// use malachite_q::Rational;
    ///
    /// assert_eq!(
    ///     Rational::from(80u32).ceiling_log_base(&Rational::from(3u32)),
    ///     4
    /// );
    /// assert_eq!(
    ///     Rational::from(81u32).ceiling_log_base(&Rational::from(3u32)),
    ///     4
    /// );
    /// assert_eq!(
    ///     Rational::from(82u32).ceiling_log_base(&Rational::from(3u32)),
    ///     5
    /// );
    /// assert_eq!(
    ///     Rational::from(4294967296u64).ceiling_log_base(&Rational::from(10u32)),
    ///     10
    /// );
    /// assert_eq!(
    ///     Rational::from_signeds(936851431250i64, 1397).ceiling_log_base(&Rational::from(10u32)),
    ///     9
    /// );
    /// assert_eq!(
    ///     Rational::from_signeds(5153632, 16807).ceiling_log_base(&Rational::from_signeds(22, 7)),
    ///     5
    /// );
    /// ```
    fn ceiling_log_base(self, base: &Rational) -> i64 {
        assert!(*self > 0u32);
        if let Some(log_base) = base.checked_log_base_2() {
            return self.ceiling_log_base_power_of_2(log_base);
        }
        let (log, exact) = log_base_helper(self, base);
        if exact { log } else { log + 1 }
    }
}

impl CheckedLogBase<&Rational> for &Rational {
    type Output = i64;

    /// Returns the base-$b$ logarithm of a positive [`Rational`]. If the [`Rational`] is not a
    /// power of $b$, then `None` is returned.
    ///
    /// Note that this function may be slow if the base is very close to 1.
    ///
    /// $$
    /// f(x, b) = \\begin{cases}
    ///     \operatorname{Some}(\log_b x) & \text{if} \\quad \log_b x \in \Z, \\\\
    ///     \operatorname{None} & \textrm{otherwise}.
    /// \\end{cases}
    /// $$
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(nm \log (nm) \log\log (nm))$
    ///
    /// $M(n, m) = O(nm \log (nm))$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `base.significant_bits()`, and $m$ is
    /// $|\log_b x|$, where $b$ is `base` and $x$ is `x`.
    ///
    /// # Panics
    /// Panics if `self` less than or equal to zero or `base` is 1.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::CheckedLogBase;
    /// use malachite_q::Rational;
    ///
    /// assert_eq!(
    ///     Rational::from(80u32).checked_log_base(&Rational::from(3u32)),
    ///     None
    /// );
    /// assert_eq!(
    ///     Rational::from(81u32).checked_log_base(&Rational::from(3u32)),
    ///     Some(4)
    /// );
    /// assert_eq!(
    ///     Rational::from(82u32).checked_log_base(&Rational::from(3u32)),
    ///     None
    /// );
    /// assert_eq!(
    ///     Rational::from(4294967296u64).checked_log_base(&Rational::from(10u32)),
    ///     None
    /// );
    /// assert_eq!(
    ///     Rational::from_signeds(936851431250i64, 1397).checked_log_base(&Rational::from(10u32)),
    ///     None
    /// );
    /// assert_eq!(
    ///     Rational::from_signeds(5153632, 16807).checked_log_base(&Rational::from_signeds(22, 7)),
    ///     Some(5)
    /// );
    /// ```
    fn checked_log_base(self, base: &Rational) -> Option<i64> {
        assert!(*self > 0u32);
        if let Some(log_base) = base.checked_log_base_2() {
            return self.checked_log_base_power_of_2(log_base);
        }
        let (log, exact) = log_base_helper(self, base);
        if exact { Some(log) } else { None }
    }
}

impl FloorLogBase<u64> for &Rational {
    type Output = i64;

    /// Returns the floor of the base-$b$ logarithm of a positive [`Rational`], where $b$ is an
    /// integer greater than 1.
    ///
    /// $f(x, b) = \lfloor\log_b x\rfloor$.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `self.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `self` is less than or equal to zero or `base` is less than 2.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::FloorLogBase;
    /// use malachite_q::Rational;
    ///
    /// assert_eq!(Rational::from(80u32).floor_log_base(3u64), 3);
    /// assert_eq!(Rational::from(81u32).floor_log_base(3u64), 4);
    /// assert_eq!(Rational::from(82u32).floor_log_base(3u64), 4);
    /// assert_eq!(Rational::from(4294967296u64).floor_log_base(10u64), 9);
    /// assert_eq!(
    ///     Rational::from_signeds(936851431250i64, 1397).floor_log_base(10u64),
    ///     8
    /// );
    /// assert_eq!(Rational::from_signeds(1, 1000000).floor_log_base(10u64), -6);
    /// ```
    fn floor_log_base(self, base: u64) -> i64 {
        assert!(*self > 0u32);
        assert!(base > 1);
        if let Some(log_base) = base.checked_log_base_2() {
            return self.floor_log_base_power_of_2(i64::exact_from(log_base));
        }
        log_base_u64_helper(self, base).0
    }
}

impl CeilingLogBase<u64> for &Rational {
    type Output = i64;

    /// Returns the ceiling of the base-$b$ logarithm of a positive [`Rational`], where $b$ is an
    /// integer greater than 1.
    ///
    /// $f(x, b) = \lceil\log_b x\rceil$.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `self.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `self` is less than or equal to zero or `base` is less than 2.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::CeilingLogBase;
    /// use malachite_q::Rational;
    ///
    /// assert_eq!(Rational::from(80u32).ceiling_log_base(3u64), 4);
    /// assert_eq!(Rational::from(81u32).ceiling_log_base(3u64), 4);
    /// assert_eq!(Rational::from(82u32).ceiling_log_base(3u64), 5);
    /// assert_eq!(Rational::from(4294967296u64).ceiling_log_base(10u64), 10);
    /// assert_eq!(
    ///     Rational::from_signeds(936851431250i64, 1397).ceiling_log_base(10u64),
    ///     9
    /// );
    /// assert_eq!(Rational::from_signeds(1, 1000000).ceiling_log_base(10u64), -6);
    /// ```
    fn ceiling_log_base(self, base: u64) -> i64 {
        assert!(*self > 0u32);
        assert!(base > 1);
        if let Some(log_base) = base.checked_log_base_2() {
            return self.ceiling_log_base_power_of_2(i64::exact_from(log_base));
        }
        let (log, exact) = log_base_u64_helper(self, base);
        if exact { log } else { log + 1 }
    }
}

impl CheckedLogBase<u64> for &Rational {
    type Output = i64;

    /// Returns the base-$b$ logarithm of a positive [`Rational`], where $b$ is an integer greater
    /// than 1. If the [`Rational`] is not a power of $b$, then `None` is returned.
    ///
    /// $$
    /// f(x, b) = \\begin{cases}
    ///     \operatorname{Some}(\log_b x) & \text{if} \\quad \log_b x \in \Z, \\\\
    ///     \operatorname{None} & \textrm{otherwise}.
    /// \\end{cases}
    /// $$
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `self.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `self` is less than or equal to zero or `base` is less than 2.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::CheckedLogBase;
    /// use malachite_q::Rational;
    ///
    /// assert_eq!(Rational::from(80u32).checked_log_base(3u64), None);
    /// assert_eq!(Rational::from(81u32).checked_log_base(3u64), Some(4));
    /// assert_eq!(Rational::from(82u32).checked_log_base(3u64), None);
    /// assert_eq!(Rational::from(4294967296u64).checked_log_base(10u64), None);
    /// assert_eq!(Rational::from(1000000u32).checked_log_base(10u64), Some(6));
    /// assert_eq!(Rational::from_signeds(1, 1000000).checked_log_base(10u64), Some(-6));
    /// ```
    fn checked_log_base(self, base: u64) -> Option<i64> {
        assert!(*self > 0u32);
        assert!(base > 1);
        if let Some(log_base) = base.checked_log_base_2() {
            return self.checked_log_base_power_of_2(i64::exact_from(log_base));
        }
        let (log, exact) = log_base_u64_helper(self, base);
        if exact { Some(log) } else { None }
    }
}
