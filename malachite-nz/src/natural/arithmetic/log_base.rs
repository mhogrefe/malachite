// Copyright © 2024 Mikhail Hogrefe
//
// Uses code adopted from the FLINT Library.
//
//      Copyright © 2011 Sebastian Pancratz
//
//      Copyright © 2011 Fredrik Johansson
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::natural::Natural;
use core::cmp::Ordering::*;
use malachite_base::num::arithmetic::traits::{
    CeilingLogBase, CeilingLogBasePowerOf2, CheckedLogBase, CheckedLogBase2,
    CheckedLogBasePowerOf2, DivExactAssign, FloorLogBase, FloorLogBasePowerOf2, Pow,
};
use malachite_base::num::basic::traits::One;
use malachite_base::num::conversion::traits::RoundingFrom;
use malachite_base::num::conversion::traits::SciMantissaAndExponent;
use malachite_base::rounding_modes::RoundingMode::*;

impl Natural {
    /// Calculates the approximate natural logarithm of a nonzero [`Natural`].
    ///
    /// $f(x) = (1+\epsilon)(\log x)$, where $|\epsilon| < 2^{-52}.$
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
    /// use malachite_base::num::arithmetic::traits::Pow;
    /// use malachite_base::num::float::NiceFloat;
    /// use malachite_nz::natural::Natural;
    ///
    /// assert_eq!(
    ///     NiceFloat(Natural::from(10u32).approx_log()),
    ///     NiceFloat(2.3025850929940455)
    /// );
    /// assert_eq!(
    ///     NiceFloat(Natural::from(10u32).pow(10000).approx_log()),
    ///     NiceFloat(23025.850929940454)
    /// );
    /// ```
    ///
    /// This is equivalent to `fmpz_dlog` from `fmpz/dlog.c`, FLINT 2.7.1.
    pub fn approx_log(&self) -> f64 {
        assert_ne!(*self, 0);
        let (mantissa, exponent): (f64, u64) = self.sci_mantissa_and_exponent();
        libm::log(mantissa) + (exponent as f64) * core::f64::consts::LN_2
    }
}

// # Worst-case complexity
// $T(n) = O(n \log n \log\log n)$
//
// $M(n) = O(n \log n)$
//
// where $T$ is time, $M$ is additional memory, and $n$ is `x.significant_bits()`.
fn log_base_helper(x: &Natural, base: &Natural) -> (u64, bool) {
    assert_ne!(*x, 0);
    assert!(*base > 1);
    if *x == 1 {
        return (0, true);
    } else if x < base {
        return (0, false);
    }
    let mut log = u64::rounding_from(x.approx_log() / base.approx_log(), Floor).0;
    let mut power = base.pow(log);
    match power.cmp(x) {
        Equal => (log, true),
        Less => loop {
            power *= base;
            match power.cmp(x) {
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
            power.div_exact_assign(base);
            match power.cmp(x) {
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
}

// # Worst-case complexity
// $T(n) = O(n \log n \log\log n)$
//
// $M(n) = O(n \log n)$
//
// where $T$ is time, $M$ is additional memory, and $n$ is `x.significant_bits()`.
//
// Also returns base^p and p, where base^p is close to x.
pub(crate) fn log_base_helper_with_pow(x: &Natural, base: &Natural) -> (u64, bool, Natural, u64) {
    assert_ne!(*x, 0);
    assert!(*base > 1);
    if *x == 1 {
        return (0, true, Natural::ONE, 0);
    } else if x < base {
        return (0, false, Natural::ONE, 0);
    }
    let mut log = (x.approx_log() / base.approx_log()) as u64;
    let mut power = base.pow(log);
    match power.cmp(x) {
        Equal => (log, true, power, log),
        Less => loop {
            power *= base;
            match power.cmp(x) {
                Equal => {
                    log += 1;
                    return (log, true, power, log);
                }
                Less => {
                    log += 1;
                }
                Greater => {
                    return (log, false, power, log + 1);
                }
            }
        },
        Greater => loop {
            power.div_exact_assign(base);
            match power.cmp(x) {
                Equal => {
                    log -= 1;
                    return (log, true, power, log);
                }
                Less => {
                    log -= 1;
                    return (log, false, power, log);
                }
                Greater => {
                    log -= 1;
                }
            }
        },
    }
}

impl<'a, 'b> FloorLogBase<&'b Natural> for &'a Natural {
    type Output = u64;

    /// Returns the floor of the base-$b$ logarithm of a positive [`Natural`].
    ///
    /// $f(x, b) = \lfloor\log_b x\rfloor$.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `x.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `self` is 0 or `base` is less than 2.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::FloorLogBase;
    /// use malachite_nz::natural::Natural;
    ///
    /// assert_eq!(Natural::from(80u32).floor_log_base(&Natural::from(3u32)), 3);
    /// assert_eq!(Natural::from(81u32).floor_log_base(&Natural::from(3u32)), 4);
    /// assert_eq!(Natural::from(82u32).floor_log_base(&Natural::from(3u32)), 4);
    /// assert_eq!(
    ///     Natural::from(4294967296u64).floor_log_base(&Natural::from(10u32)),
    ///     9
    /// );
    /// ```
    ///
    /// This is equivalent to `fmpz_flog` from `fmpz/flog.c`, FLINT 2.7.1.
    fn floor_log_base(self, base: &Natural) -> u64 {
        if let Some(log_base) = base.checked_log_base_2() {
            return self.floor_log_base_power_of_2(log_base);
        }
        log_base_helper(self, base).0
    }
}

impl<'a, 'b> CeilingLogBase<&'b Natural> for &'a Natural {
    type Output = u64;

    /// Returns the ceiling of the base-$b$ logarithm of a positive [`Natural`].
    ///
    /// $f(x, b) = \lceil\log_b x\rceil$.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `x.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `self` is 0 or `base` is less than 2.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::CeilingLogBase;
    /// use malachite_nz::natural::Natural;
    ///
    /// assert_eq!(
    ///     Natural::from(80u32).ceiling_log_base(&Natural::from(3u32)),
    ///     4
    /// );
    /// assert_eq!(
    ///     Natural::from(81u32).ceiling_log_base(&Natural::from(3u32)),
    ///     4
    /// );
    /// assert_eq!(
    ///     Natural::from(82u32).ceiling_log_base(&Natural::from(3u32)),
    ///     5
    /// );
    /// assert_eq!(
    ///     Natural::from(4294967296u64).ceiling_log_base(&Natural::from(10u32)),
    ///     10
    /// );
    /// ```
    ///
    /// This is equivalent to `fmpz_clog` from `fmpz/clog.c`, FLINT 2.7.1.
    fn ceiling_log_base(self, base: &Natural) -> u64 {
        if let Some(log_base) = base.checked_log_base_2() {
            return self.ceiling_log_base_power_of_2(log_base);
        }
        let (log, exact) = log_base_helper(self, base);
        if exact {
            log
        } else {
            log + 1
        }
    }
}

impl<'a, 'b> CheckedLogBase<&'b Natural> for &'a Natural {
    type Output = u64;

    /// Returns the base-$b$ logarithm of a positive [`Natural`]. If the [`Natural`] is not a power
    /// of $b$, then `None` is returned.
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
    /// where $T$ is time, $M$ is additional memory, and $n$ is `x.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `self` is 0 or `base` is less than 2.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::CheckedLogBase;
    /// use malachite_nz::natural::Natural;
    ///
    /// assert_eq!(
    ///     Natural::from(80u32).checked_log_base(&Natural::from(3u32)),
    ///     None
    /// );
    /// assert_eq!(
    ///     Natural::from(81u32).checked_log_base(&Natural::from(3u32)),
    ///     Some(4)
    /// );
    /// assert_eq!(
    ///     Natural::from(82u32).checked_log_base(&Natural::from(3u32)),
    ///     None
    /// );
    /// assert_eq!(
    ///     Natural::from(4294967296u64).checked_log_base(&Natural::from(10u32)),
    ///     None
    /// );
    /// ```
    fn checked_log_base(self, base: &Natural) -> Option<u64> {
        if let Some(log_base) = base.checked_log_base_2() {
            return self.checked_log_base_power_of_2(log_base);
        }
        let (log, exact) = log_base_helper(self, base);
        if exact {
            Some(log)
        } else {
            None
        }
    }
}
