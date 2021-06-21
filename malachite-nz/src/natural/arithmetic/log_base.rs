use malachite_base::num::arithmetic::traits::{
    CeilingLogBase, CeilingLogBasePowerOf2, CheckedLogBase, CheckedLogBase2,
    CheckedLogBasePowerOf2, DivExactAssign, FloorLogBase, FloorLogBasePowerOf2, Pow,
};
use natural::Natural;
use std::cmp::Ordering;

impl Natural {
    /// Calculates the approximate natural logarithm of a `Natural`.
    ///
    /// $f(x) = \log x \pm O(\log x)$.
    ///
    /// TODO complexity
    ///
    /// # Examples
    /// ```
    /// extern crate malachite_base;
    /// extern crate malachite_nz;
    ///
    /// use malachite_base::num::arithmetic::traits::Pow;
    /// use malachite_base::num::float::nice_float::NiceFloat;
    /// use malachite_nz::natural::Natural;
    ///
    /// assert_eq!(NiceFloat(Natural::from(10u32).approx_log()), NiceFloat(2.3025850929940455));
    /// assert_eq!(
    ///     NiceFloat(Natural::from(10u32).pow(100).approx_log()),
    ///     NiceFloat(230.25850929940455)
    /// );
    /// ```
    ///
    /// This is fmpz_dlog from fmpz/dlog.c, Flint 2.7.1.
    pub fn approx_log(&self) -> f64 {
        assert_ne!(*self, 0);
        let (mantissa, exponent) = self.sci_mantissa_and_exponent::<f64>();
        mantissa.ln() + (exponent as f64) * std::f64::consts::LN_2
    }
}

fn log_base_helper(x: &Natural, base: &Natural) -> (u64, bool) {
    assert_ne!(*x, 0);
    assert!(*base > 1);
    if *x == 1 {
        return (0, true);
    } else if x < base {
        return (0, false);
    }
    let mut log = (x.approx_log() / base.approx_log()) as u64;
    let mut power = base.pow(log);
    match power.cmp(x) {
        Ordering::Equal => (log, true),
        Ordering::Less => loop {
            power *= base;
            match power.cmp(x) {
                Ordering::Equal => {
                    return (log + 1, true);
                }
                Ordering::Less => {
                    log += 1;
                }
                Ordering::Greater => {
                    return (log, false);
                }
            }
        },
        Ordering::Greater => loop {
            power.div_exact_assign(base);
            match power.cmp(x) {
                Ordering::Equal => {
                    return (log - 1, true);
                }
                Ordering::Less => {
                    return (log - 1, false);
                }
                Ordering::Greater => {
                    log -= 1;
                }
            }
        },
    }
}

impl<'a, 'b> FloorLogBase<&'b Natural> for &'a Natural {
    /// Returns the floor of the base-$b$ logarithm of a positive `Natural`.
    ///
    /// $f(x, b) = \lfloor\log_b x\rfloor$.
    ///
    /// TODO complexity
    ///
    /// # Panics
    /// Panics if `self` is 0 or `base` is less than 2.
    ///
    /// # Examples
    /// ```
    /// extern crate malachite_base;
    /// extern crate malachite_nz;
    ///
    /// use malachite_base::num::arithmetic::traits::FloorLogBase;
    /// use malachite_nz::natural::Natural;
    ///
    /// assert_eq!(Natural::from(80u32).floor_log_base(&Natural::from(3u32)), 3);
    /// assert_eq!(Natural::from(81u32).floor_log_base(&Natural::from(3u32)), 4);
    /// assert_eq!(Natural::from(82u32).floor_log_base(&Natural::from(3u32)), 4);
    /// assert_eq!(Natural::from(4294967296u64).floor_log_base(&Natural::from(10u32)), 9);
    /// ```
    ///
    /// This is fmpz_flog from fmpz/flog.c, FLINT 2.7.1.
    fn floor_log_base(self, base: &Natural) -> u64 {
        if let Some(log_base) = base.checked_log_base_2() {
            return self.floor_log_base_power_of_2(log_base);
        }
        log_base_helper(self, base).0
    }
}

impl<'a, 'b> CeilingLogBase<&'b Natural> for &'a Natural {
    /// Returns the ceiling of the base-$b$ logarithm of a positive `Natural`.
    ///
    /// $f(x, b) = \lceil\log_b x\rceil$.
    ///
    /// TODO complexity
    ///
    /// # Panics
    /// Panics if `self` is 0 or `base` is less than 2.
    ///
    /// # Examples
    /// ```
    /// extern crate malachite_base;
    /// extern crate malachite_nz;
    ///
    /// use malachite_base::num::arithmetic::traits::CeilingLogBase;
    /// use malachite_nz::natural::Natural;
    ///
    /// assert_eq!(Natural::from(80u32).ceiling_log_base(&Natural::from(3u32)), 4);
    /// assert_eq!(Natural::from(81u32).ceiling_log_base(&Natural::from(3u32)), 4);
    /// assert_eq!(Natural::from(82u32).ceiling_log_base(&Natural::from(3u32)), 5);
    /// assert_eq!(Natural::from(4294967296u64).ceiling_log_base(&Natural::from(10u32)), 10);
    /// ```
    ///
    /// This is fmpz_clog from fmpz/clog.c, FLINT 2.7.1.
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
    /// Returns the base-$b$ logarithm of a positive `Natural`. If the integer is not a power of
    /// $b$, `None` is returned.
    ///
    /// $$
    /// f(x, b) = \\begin{cases}
    ///     \operatorname{Some}(\log_b x) & \log_b x \in \Z \\\\
    ///     \operatorname{None} & \textrm{otherwise},
    /// \\end{cases}
    /// $$
    ///
    /// TODO complexity
    ///
    /// # Panics
    /// Panics if `self` is 0 or `base` is less than 2.
    ///
    /// # Examples
    /// ```
    /// extern crate malachite_base;
    /// extern crate malachite_nz;
    ///
    /// use malachite_base::num::arithmetic::traits::CheckedLogBase;
    /// use malachite_nz::natural::Natural;
    ///
    /// assert_eq!(Natural::from(80u32).checked_log_base(&Natural::from(3u32)), None);
    /// assert_eq!(Natural::from(81u32).checked_log_base(&Natural::from(3u32)), Some(4));
    /// assert_eq!(Natural::from(82u32).checked_log_base(&Natural::from(3u32)), None);
    /// assert_eq!(Natural::from(4294967296u64).checked_log_base(&Natural::from(10u32)), None);
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
