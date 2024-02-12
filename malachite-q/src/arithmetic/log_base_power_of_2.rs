use crate::Rational;
use core::cmp::Ordering;
use malachite_base::num::arithmetic::traits::{
    CeilingLogBase2, CeilingLogBasePowerOf2, CheckedLogBase2, CheckedLogBasePowerOf2, DivMod,
    DivRound, FloorLogBase2, FloorLogBasePowerOf2, Sign,
};
use malachite_base::rounding_modes::RoundingMode;

impl<'a> FloorLogBasePowerOf2<i64> for &'a Rational {
    type Output = i64;

    /// Returns the floor of the base-$2^k$ logarithm of a positive [`Rational`].
    ///
    /// $k$ may be negative.
    ///
    /// $f(x, k) = \lfloor\log_{2^k} x\rfloor$.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(1)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `self.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `self` is less than or equal to 0 or `pow` is 0.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::FloorLogBasePowerOf2;
    /// use malachite_q::Rational;
    ///
    /// assert_eq!(Rational::from(100).floor_log_base_power_of_2(2), 3);
    /// assert_eq!(Rational::from(4294967296u64).floor_log_base_power_of_2(8), 4);
    ///
    /// // 4^(-2) < 1/10 < 4^(-1)
    /// assert_eq!(Rational::from_signeds(1, 10).floor_log_base_power_of_2(2), -2);
    /// // (1/4)^2 < 1/10 < (1/4)^1
    /// assert_eq!(Rational::from_signeds(1, 10).floor_log_base_power_of_2(-2), 1);
    /// ```
    fn floor_log_base_power_of_2(self, pow: i64) -> i64 {
        assert!(*self > 0u32);
        match pow.sign() {
            Ordering::Equal => panic!("Cannot take base-1 logarithm"),
            Ordering::Greater => {
                self.floor_log_base_2()
                    .div_round(pow, RoundingMode::Floor)
                    .0
            }
            Ordering::Less => {
                -(self
                    .ceiling_log_base_2()
                    .div_round(-pow, RoundingMode::Ceiling)
                    .0)
            }
        }
    }
}

impl<'a> CeilingLogBasePowerOf2<i64> for &'a Rational {
    type Output = i64;

    /// Returns the ceiling of the base-$2^k$ logarithm of a positive [`Rational`].
    ///
    /// $k$ may be negative.
    ///
    /// $f(x, p) = \lceil\log_{2^p} x\rceil$.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(1)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `self.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `self` is less than or equal to 0 or `pow` is 0.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::CeilingLogBasePowerOf2;
    /// use malachite_q::Rational;
    ///
    /// assert_eq!(Rational::from(100).ceiling_log_base_power_of_2(2), 4);
    /// assert_eq!(Rational::from(4294967296u64).ceiling_log_base_power_of_2(8), 4);
    ///
    /// // 4^(-2) < 1/10 < 4^(-1)
    /// assert_eq!(Rational::from_signeds(1, 10).ceiling_log_base_power_of_2(2), -1);
    /// // (1/4)^2 < 1/10 < (1/4)^1
    /// assert_eq!(Rational::from_signeds(1, 10).ceiling_log_base_power_of_2(-2), 2);
    /// ```
    fn ceiling_log_base_power_of_2(self, pow: i64) -> i64 {
        assert!(*self > 0u32);
        match pow.sign() {
            Ordering::Equal => panic!("Cannot take base-1 logarithm"),
            Ordering::Greater => {
                self.ceiling_log_base_2()
                    .div_round(pow, RoundingMode::Ceiling)
                    .0
            }
            Ordering::Less => {
                -self
                    .floor_log_base_2()
                    .div_round(-pow, RoundingMode::Floor)
                    .0
            }
        }
    }
}

impl<'a> CheckedLogBasePowerOf2<i64> for &'a Rational {
    type Output = i64;

    /// Returns the base-$2^k$ logarithm of a positive [`Rational`]. If the [`Rational`] is not a
    /// power of $2^k$, then `None` is returned.
    ///
    /// $k$ may be negative.
    ///
    /// $$
    /// f(x, p) = \\begin{cases}
    ///     \operatorname{Some}(\log_{2^p} x) & \text{if} \\quad \log_{2^p} x \in \Z, \\\\
    ///     \operatorname{None} & \textrm{otherwise}.
    /// \\end{cases}
    /// $$
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(1)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `self.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `self` is 0 or `pow` is 0.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::CheckedLogBasePowerOf2;
    /// use malachite_q::Rational;
    ///
    /// assert_eq!(Rational::from(100).checked_log_base_power_of_2(2), None);
    /// assert_eq!(Rational::from(4294967296u64).checked_log_base_power_of_2(8), Some(4));
    ///
    /// // 4^(-2) < 1/10 < 4^(-1)
    /// assert_eq!(Rational::from_signeds(1, 10).checked_log_base_power_of_2(2), None);
    /// assert_eq!(Rational::from_signeds(1, 16).checked_log_base_power_of_2(2), Some(-2));
    /// // (1/4)^2 < 1/10 < (1/4)^1
    /// assert_eq!(Rational::from_signeds(1, 10).checked_log_base_power_of_2(-2), None);
    /// assert_eq!(Rational::from_signeds(1, 16).checked_log_base_power_of_2(-2), Some(2));
    /// ```
    fn checked_log_base_power_of_2(self, pow: i64) -> Option<i64> {
        assert!(*self > 0u32);
        let log_base_2 = self.checked_log_base_2()?;
        let (pow, neg) = match pow.sign() {
            Ordering::Equal => panic!("Cannot take base-1 logarithm"),
            Ordering::Greater => (pow, false),
            Ordering::Less => (-pow, true),
        };
        let (log, rem) = log_base_2.div_mod(pow);
        if rem != 0 {
            None
        } else if neg {
            Some(-log)
        } else {
            Some(log)
        }
    }
}
