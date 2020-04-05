use malachite_base::num::arithmetic::traits::{DivRound, DivRoundAssign};
use malachite_base::round::RoundingMode;

use integer::Integer;

impl DivRound<Integer> for Integer {
    type Output = Integer;

    /// Divides an `Integer` by an `Integer` and rounds according to a specified rounding mode,
    /// taking both `Integer`s by value. See the `RoundingMode` documentation for details.
    ///
    /// Time: Worst case O(n * log(n) * log(log(n)))
    ///
    /// Additional memory: Worst case O(n * log(n))
    ///
    /// where n = `self.significant_bits()`
    ///
    /// # Panics
    /// Panics if `other` is zero, or if `rm` is `Exact` but `self` is not divisible by `other`.
    ///
    /// # Examples
    /// ```
    /// extern crate malachite_base;
    /// extern crate malachite_nz;
    ///
    /// use malachite_base::num::arithmetic::traits::DivRound;
    /// use malachite_base::round::RoundingMode;
    /// use malachite_nz::integer::Integer;
    ///
    /// assert_eq!(
    ///     Integer::from(-10).div_round(Integer::from(4), RoundingMode::Down).to_string(),
    ///     "-2"
    /// );
    /// assert_eq!(
    ///     (-Integer::trillion()).div_round(Integer::from(3), RoundingMode::Floor).to_string(),
    ///     "-333333333334"
    /// );
    /// assert_eq!(
    ///     Integer::from(-10).div_round(Integer::from(4), RoundingMode::Up).to_string(),
    ///     "-3"
    /// );
    /// assert_eq!(
    ///     (-Integer::trillion()).div_round(Integer::from(3), RoundingMode::Ceiling)
    ///         .to_string(),
    ///     "-333333333333"
    /// );
    /// assert_eq!(
    ///     Integer::from(-10).div_round(Integer::from(5), RoundingMode::Exact).to_string(),
    ///     "-2"
    /// );
    /// assert_eq!(
    ///     Integer::from(-10).div_round(Integer::from(3), RoundingMode::Nearest).to_string(),
    ///     "-3"
    /// );
    /// assert_eq!(
    ///     Integer::from(-20).div_round(Integer::from(3), RoundingMode::Nearest).to_string(),
    ///     "-7"
    /// );
    /// assert_eq!(
    ///     Integer::from(-10).div_round(Integer::from(4), RoundingMode::Nearest).to_string(),
    ///     "-2"
    /// );
    /// assert_eq!(
    ///     Integer::from(-14).div_round(Integer::from(4), RoundingMode::Nearest).to_string(),
    ///     "-4"
    /// );
    ///
    /// assert_eq!(
    ///     Integer::from(-10).div_round(Integer::from(-4), RoundingMode::Down).to_string(),
    ///     "2"
    /// );
    /// assert_eq!(
    ///     (-Integer::trillion()).div_round(Integer::from(-3), RoundingMode::Floor)
    ///         .to_string(),
    ///     "333333333333"
    /// );
    /// assert_eq!(
    ///     Integer::from(-10).div_round(Integer::from(-4), RoundingMode::Up).to_string(),
    ///     "3"
    /// );
    /// assert_eq!(
    ///     (-Integer::trillion()).div_round(Integer::from(-3), RoundingMode::Ceiling)
    ///         .to_string(),
    ///     "333333333334"
    /// );
    /// assert_eq!(
    ///     Integer::from(-10).div_round(Integer::from(-5), RoundingMode::Exact).to_string(),
    ///     "2"
    /// );
    /// assert_eq!(
    ///     Integer::from(-10).div_round(Integer::from(-3), RoundingMode::Nearest).to_string(),
    ///     "3"
    /// );
    /// assert_eq!(
    ///     Integer::from(-20).div_round(Integer::from(-3), RoundingMode::Nearest).to_string(),
    ///     "7"
    /// );
    /// assert_eq!(
    ///     Integer::from(-10).div_round(Integer::from(-4), RoundingMode::Nearest).to_string(),
    ///     "2"
    /// );
    /// assert_eq!(
    ///     Integer::from(-14).div_round(Integer::from(-4), RoundingMode::Nearest).to_string(),
    ///     "4"
    /// );
    /// ```
    #[inline]
    fn div_round(mut self, other: Integer, rm: RoundingMode) -> Integer {
        self.div_round_assign(other, rm);
        self
    }
}

impl<'a> DivRound<&'a Integer> for Integer {
    type Output = Integer;

    /// Divides an `Integer` by an `Integer` and rounds according to a specified rounding mode,
    /// taking the first `Integer` by value and the second by reference. See the `RoundingMode`
    /// documentation for details.
    ///
    /// Time: Worst case O(n * log(n) * log(log(n)))
    ///
    /// Additional memory: Worst case O(n * log(n))
    ///
    /// where n = `self.significant_bits()`
    ///
    /// # Panics
    /// Panics if `other` is zero, or if `rm` is `Exact` but `self` is not divisible by `other`.
    ///
    /// # Examples
    /// ```
    /// extern crate malachite_base;
    /// extern crate malachite_nz;
    ///
    /// use malachite_base::num::arithmetic::traits::DivRound;
    /// use malachite_base::round::RoundingMode;
    /// use malachite_nz::integer::Integer;
    ///
    /// assert_eq!(
    ///     Integer::from(-10).div_round(&Integer::from(4), RoundingMode::Down).to_string(),
    ///     "-2"
    /// );
    /// assert_eq!(
    ///     (-Integer::trillion()).div_round(&Integer::from(3), RoundingMode::Floor)
    ///         .to_string(),
    ///     "-333333333334"
    /// );
    /// assert_eq!(
    ///     Integer::from(-10).div_round(&Integer::from(4), RoundingMode::Up).to_string(),
    ///     "-3"
    /// );
    /// assert_eq!(
    ///     (-Integer::trillion()).div_round(&Integer::from(3), RoundingMode::Ceiling)
    ///         .to_string(),
    ///     "-333333333333"
    /// );
    /// assert_eq!(
    ///     Integer::from(-10).div_round(&Integer::from(5), RoundingMode::Exact).to_string(),
    ///     "-2"
    /// );
    /// assert_eq!(
    ///     Integer::from(-10).div_round(&Integer::from(3), RoundingMode::Nearest).to_string(),
    ///     "-3"
    /// );
    /// assert_eq!(
    ///     Integer::from(-20).div_round(&Integer::from(3), RoundingMode::Nearest).to_string(),
    ///     "-7"
    /// );
    /// assert_eq!(
    ///     Integer::from(-10).div_round(&Integer::from(4), RoundingMode::Nearest).to_string(),
    ///     "-2"
    /// );
    /// assert_eq!(
    ///     Integer::from(-14).div_round(&Integer::from(4), RoundingMode::Nearest).to_string(),
    ///     "-4"
    /// );
    ///
    /// assert_eq!(
    ///     Integer::from(-10).div_round(&Integer::from(-4), RoundingMode::Down).to_string(),
    ///     "2"
    /// );
    /// assert_eq!(
    ///     (-Integer::trillion()).div_round(&Integer::from(-3), RoundingMode::Floor)
    ///         .to_string(),
    ///     "333333333333"
    /// );
    /// assert_eq!(
    ///     Integer::from(-10).div_round(&Integer::from(-4), RoundingMode::Up).to_string(),
    ///     "3"
    /// );
    /// assert_eq!(
    ///     (-Integer::trillion()).div_round(&Integer::from(-3), RoundingMode::Ceiling)
    ///         .to_string(),
    ///     "333333333334"
    /// );
    /// assert_eq!(
    ///     Integer::from(-10).div_round(&Integer::from(-5), RoundingMode::Exact).to_string(),
    ///     "2"
    /// );
    /// assert_eq!(
    ///     Integer::from(-10).div_round(&Integer::from(-3), RoundingMode::Nearest).to_string(),
    ///     "3"
    /// );
    /// assert_eq!(
    ///     Integer::from(-20).div_round(&Integer::from(-3), RoundingMode::Nearest).to_string(),
    ///     "7"
    /// );
    /// assert_eq!(
    ///     Integer::from(-10).div_round(&Integer::from(-4), RoundingMode::Nearest).to_string(),
    ///     "2"
    /// );
    /// assert_eq!(
    ///     Integer::from(-14).div_round(&Integer::from(-4), RoundingMode::Nearest).to_string(),
    ///     "4"
    /// );
    /// ```
    #[inline]
    fn div_round(mut self, other: &'a Integer, rm: RoundingMode) -> Integer {
        self.div_round_assign(other, rm);
        self
    }
}

impl<'a> DivRound<Integer> for &'a Integer {
    type Output = Integer;

    /// Divides an `Integer` by an `Integer` and rounds according to a specified rounding mode,
    /// taking the first `Integer` by reference and the second by value. See the `RoundingMode`
    /// documentation for details.
    ///
    /// Time: Worst case O(n * log(n) * log(log(n)))
    ///
    /// Additional memory: Worst case O(n * log(n))
    ///
    /// where n = `self.significant_bits()`
    ///
    /// # Panics
    /// Panics if `other` is zero, or if `rm` is `Exact` but `self` is not divisible by `other`.
    ///
    /// # Examples
    /// ```
    /// extern crate malachite_base;
    /// extern crate malachite_nz;
    ///
    /// use malachite_base::num::arithmetic::traits::DivRound;
    /// use malachite_base::round::RoundingMode;
    /// use malachite_nz::integer::Integer;
    ///
    /// assert_eq!(
    ///     (&Integer::from(-10)).div_round(Integer::from(4), RoundingMode::Down).to_string(),
    ///     "-2"
    /// );
    /// assert_eq!(
    ///     (&-Integer::trillion()).div_round(Integer::from(3), RoundingMode::Floor)
    ///         .to_string(),
    ///     "-333333333334"
    /// );
    /// assert_eq!(
    ///     (&Integer::from(-10)).div_round(Integer::from(4), RoundingMode::Up).to_string(),
    ///     "-3"
    /// );
    /// assert_eq!(
    ///     (&-Integer::trillion()).div_round(Integer::from(3), RoundingMode::Ceiling)
    ///         .to_string(),
    ///     "-333333333333"
    /// );
    /// assert_eq!(
    ///     (&Integer::from(-10)).div_round(Integer::from(5), RoundingMode::Exact).to_string(),
    ///     "-2"
    /// );
    /// assert_eq!(
    ///     (&Integer::from(-10)).div_round(Integer::from(3), RoundingMode::Nearest)
    ///         .to_string(),
    ///     "-3"
    /// );
    /// assert_eq!(
    ///     (&Integer::from(-20)).div_round(Integer::from(3), RoundingMode::Nearest)
    ///         .to_string(),
    ///     "-7"
    /// );
    /// assert_eq!(
    ///     (&Integer::from(-10)).div_round(Integer::from(4), RoundingMode::Nearest)
    ///         .to_string(),
    ///     "-2"
    /// );
    /// assert_eq!(
    ///     (&Integer::from(-14)).div_round(Integer::from(4), RoundingMode::Nearest)
    ///         .to_string(),
    ///     "-4"
    /// );
    ///
    /// assert_eq!(
    ///     (&Integer::from(-10)).div_round(Integer::from(-4), RoundingMode::Down)
    ///         .to_string(),
    ///     "2"
    /// );
    /// assert_eq!(
    ///     (&-Integer::trillion()).div_round(Integer::from(-3), RoundingMode::Floor)
    ///         .to_string(),
    ///     "333333333333"
    /// );
    /// assert_eq!(
    ///     (&Integer::from(-10)).div_round(Integer::from(-4), RoundingMode::Up)
    ///         .to_string(),
    ///     "3"
    /// );
    /// assert_eq!(
    ///     (&-Integer::trillion()).div_round(Integer::from(-3), RoundingMode::Ceiling)
    ///         .to_string(),
    ///     "333333333334"
    /// );
    /// assert_eq!(
    ///     (&Integer::from(-10)).div_round(Integer::from(-5), RoundingMode::Exact)
    ///         .to_string(),
    ///     "2"
    /// );
    /// assert_eq!(
    ///     (&Integer::from(-10)).div_round(Integer::from(-3), RoundingMode::Nearest)
    ///         .to_string(),
    ///     "3"
    /// );
    /// assert_eq!(
    ///     (&Integer::from(-20)).div_round(Integer::from(-3), RoundingMode::Nearest)
    ///         .to_string(),
    ///     "7"
    /// );
    /// assert_eq!(
    ///     (&Integer::from(-10)).div_round(Integer::from(-4), RoundingMode::Nearest)
    ///         .to_string(),
    ///     "2"
    /// );
    /// assert_eq!(
    ///     (&Integer::from(-14)).div_round(Integer::from(-4), RoundingMode::Nearest)
    ///         .to_string(),
    ///     "4"
    /// );
    /// ```
    fn div_round(self, other: Integer, rm: RoundingMode) -> Integer {
        let q_sign = self.sign == other.sign;
        let q = (&self.abs).div_round(other.abs, if q_sign { rm } else { -rm });
        if q_sign {
            Integer::from(q)
        } else {
            -q
        }
    }
}

impl<'a, 'b> DivRound<&'b Integer> for &'a Integer {
    type Output = Integer;

    /// Divides an `Integer` by an `Integer` and rounds according to a specified rounding mode,
    /// taking both `Integer`s by reference. See the `RoundingMode` documentation for details.
    ///
    /// Time: Worst case O(n * log(n) * log(log(n)))
    ///
    /// Additional memory: Worst case O(n * log(n))
    ///
    /// where n = `self.significant_bits()`
    ///
    /// # Panics
    /// Panics if `other` is zero, or if `rm` is `Exact` but `self` is not divisible by `other`.
    ///
    /// # Examples
    /// ```
    /// extern crate malachite_base;
    /// extern crate malachite_nz;
    ///
    /// use malachite_base::num::arithmetic::traits::DivRound;
    /// use malachite_base::round::RoundingMode;
    /// use malachite_nz::integer::Integer;
    ///
    /// assert_eq!(
    ///     (&Integer::from(-10)).div_round(&Integer::from(4), RoundingMode::Down).to_string(),
    ///     "-2"
    /// );
    /// assert_eq!(
    ///     (&-Integer::trillion()).div_round(&Integer::from(3), RoundingMode::Floor)
    ///         .to_string(),
    ///     "-333333333334"
    /// );
    /// assert_eq!(
    ///     (&Integer::from(-10)).div_round(&Integer::from(4), RoundingMode::Up).to_string(),
    ///     "-3"
    /// );
    /// assert_eq!(
    ///     (&-Integer::trillion()).div_round(&Integer::from(3), RoundingMode::Ceiling)
    ///         .to_string(),
    ///     "-333333333333"
    /// );
    /// assert_eq!(
    ///     (&Integer::from(-10)).div_round(&Integer::from(5), RoundingMode::Exact).to_string(),
    ///     "-2"
    /// );
    /// assert_eq!(
    ///     (&Integer::from(-10)).div_round(&Integer::from(3), RoundingMode::Nearest)
    ///         .to_string(),
    ///     "-3"
    /// );
    /// assert_eq!(
    ///     (&Integer::from(-20)).div_round(&Integer::from(3), RoundingMode::Nearest)
    ///         .to_string(),
    ///     "-7"
    /// );
    /// assert_eq!(
    ///     (&Integer::from(-10)).div_round(&Integer::from(4), RoundingMode::Nearest)
    ///         .to_string(),
    ///     "-2"
    /// );
    /// assert_eq!(
    ///     (&Integer::from(-14)).div_round(&Integer::from(4), RoundingMode::Nearest)
    ///         .to_string(),
    ///     "-4"
    /// );
    ///
    /// assert_eq!(
    ///     (&Integer::from(-10)).div_round(&Integer::from(-4), RoundingMode::Down)
    ///         .to_string(),
    ///     "2"
    /// );
    /// assert_eq!(
    ///     (&-Integer::trillion()).div_round(&Integer::from(-3), RoundingMode::Floor)
    ///         .to_string(),
    ///     "333333333333"
    /// );
    /// assert_eq!(
    ///     (&Integer::from(-10)).div_round(&Integer::from(-4), RoundingMode::Up)
    ///         .to_string(),
    ///     "3"
    /// );
    /// assert_eq!(
    ///     (&-Integer::trillion()).div_round(&Integer::from(-3), RoundingMode::Ceiling)
    ///         .to_string(),
    ///     "333333333334"
    /// );
    /// assert_eq!(
    ///     (&Integer::from(-10)).div_round(&Integer::from(-5), RoundingMode::Exact)
    ///         .to_string(),
    ///     "2"
    /// );
    /// assert_eq!(
    ///     (&Integer::from(-10)).div_round(&Integer::from(-3), RoundingMode::Nearest)
    ///         .to_string(),
    ///     "3"
    /// );
    /// assert_eq!(
    ///     (&Integer::from(-20)).div_round(&Integer::from(-3), RoundingMode::Nearest)
    ///         .to_string(),
    ///     "7"
    /// );
    /// assert_eq!(
    ///     (&Integer::from(-10)).div_round(&Integer::from(-4), RoundingMode::Nearest)
    ///         .to_string(),
    ///     "2"
    /// );
    /// assert_eq!(
    ///     (&Integer::from(-14)).div_round(&Integer::from(-4), RoundingMode::Nearest)
    ///         .to_string(),
    ///     "4"
    /// );
    /// ```
    fn div_round(self, other: &'b Integer, rm: RoundingMode) -> Integer {
        let q_sign = self.sign == other.sign;
        let q = (&self.abs).div_round(&other.abs, if q_sign { rm } else { -rm });
        if q_sign {
            Integer::from(q)
        } else {
            -q
        }
    }
}

impl DivRoundAssign<Integer> for Integer {
    /// Divides an `Integer` by an `Integer` in place and rounds according to a specified rounding
    /// mode, taking the `Natural` on the RHS by value. See the `RoundingMode` documentation for
    /// details.
    ///
    /// Time: Worst case O(n * log(n) * log(log(n)))
    ///
    /// Additional memory: Worst case O(n * log(n))
    ///
    /// where n = `self.significant_bits()`
    ///
    /// # Panics
    /// Panics if `other` is zero, or if `rm` is `Exact` but `self` is not divisible by `other`.
    ///
    /// # Examples
    /// ```
    /// extern crate malachite_base;
    /// extern crate malachite_nz;
    ///
    /// use malachite_base::num::arithmetic::traits::DivRoundAssign;
    /// use malachite_base::round::RoundingMode;
    /// use malachite_nz::integer::Integer;
    ///
    /// let mut n = Integer::from(-10);
    /// n.div_round_assign(Integer::from(4), RoundingMode::Down);
    /// assert_eq!(n.to_string(), "-2");
    ///
    /// let mut n = -Integer::trillion();
    /// n.div_round_assign(Integer::from(3), RoundingMode::Floor);
    /// assert_eq!(n.to_string(), "-333333333334");
    ///
    /// let mut n = Integer::from(-10);
    /// n.div_round_assign(Integer::from(4), RoundingMode::Up);
    /// assert_eq!(n.to_string(), "-3");
    ///
    /// let mut n = -Integer::trillion();
    /// n.div_round_assign(Integer::from(3), RoundingMode::Ceiling);
    /// assert_eq!(n.to_string(), "-333333333333");
    ///
    /// let mut n = Integer::from(-10);
    /// n.div_round_assign(Integer::from(5), RoundingMode::Exact);
    /// assert_eq!(n.to_string(), "-2");
    ///
    /// let mut n = Integer::from(-10);
    /// n.div_round_assign(Integer::from(3), RoundingMode::Nearest);
    /// assert_eq!(n.to_string(), "-3");
    ///
    /// let mut n = Integer::from(-20);
    /// n.div_round_assign(Integer::from(3), RoundingMode::Nearest);
    /// assert_eq!(n.to_string(), "-7");
    ///
    /// let mut n = Integer::from(-10);
    /// n.div_round_assign(Integer::from(4), RoundingMode::Nearest);
    /// assert_eq!(n.to_string(), "-2");
    ///
    /// let mut n = Integer::from(-14);
    /// n.div_round_assign(Integer::from(4), RoundingMode::Nearest);
    /// assert_eq!(n.to_string(), "-4");
    ///
    /// let mut n = Integer::from(-10);
    /// n.div_round_assign(Integer::from(-4), RoundingMode::Down);
    /// assert_eq!(n.to_string(), "2");
    ///
    /// let mut n = -Integer::trillion();
    /// n.div_round_assign(Integer::from(-3), RoundingMode::Floor);
    /// assert_eq!(n.to_string(), "333333333333");
    ///
    /// let mut n = Integer::from(-10);
    /// n.div_round_assign(Integer::from(-4), RoundingMode::Up);
    /// assert_eq!(n.to_string(), "3");
    ///
    /// let mut n = -Integer::trillion();
    /// n.div_round_assign(Integer::from(-3), RoundingMode::Ceiling);
    /// assert_eq!(n.to_string(), "333333333334");
    ///
    /// let mut n = Integer::from(-10);
    /// n.div_round_assign(Integer::from(-5), RoundingMode::Exact);
    /// assert_eq!(n.to_string(), "2");
    ///
    /// let mut n = Integer::from(-10);
    /// n.div_round_assign(Integer::from(-3), RoundingMode::Nearest);
    /// assert_eq!(n.to_string(), "3");
    ///
    /// let mut n = Integer::from(-20);
    /// n.div_round_assign(Integer::from(-3), RoundingMode::Nearest);
    /// assert_eq!(n.to_string(), "7");
    ///
    /// let mut n = Integer::from(-10);
    /// n.div_round_assign(Integer::from(-4), RoundingMode::Nearest);
    /// assert_eq!(n.to_string(), "2");
    ///
    /// let mut n = Integer::from(-14);
    /// n.div_round_assign(Integer::from(-4), RoundingMode::Nearest);
    /// assert_eq!(n.to_string(), "4");
    /// ```
    fn div_round_assign(&mut self, other: Integer, rm: RoundingMode) {
        let q_sign = self.sign == other.sign;
        self.abs
            .div_round_assign(other.abs, if q_sign { rm } else { -rm });
        self.sign = q_sign || self.abs == 0;
    }
}

impl<'a> DivRoundAssign<&'a Integer> for Integer {
    /// Divides an `Integer` by an `Integer` in place and rounds according to a specified rounding
    /// mode, taking the `Natural` on the RHS by reference. See the `RoundingMode` documentation for
    /// details.
    ///
    /// Time: Worst case O(n * log(n) * log(log(n)))
    ///
    /// Additional memory: Worst case O(n * log(n))
    ///
    /// where n = `self.significant_bits()`
    ///
    /// # Panics
    /// Panics if `other` is zero, or if `rm` is `Exact` but `self` is not divisible by `other`.
    ///
    /// # Examples
    /// ```
    /// extern crate malachite_base;
    /// extern crate malachite_nz;
    ///
    /// use malachite_base::num::arithmetic::traits::DivRoundAssign;
    /// use malachite_base::round::RoundingMode;
    /// use malachite_nz::integer::Integer;
    ///
    /// let mut n = Integer::from(-10);
    /// n.div_round_assign(&Integer::from(4), RoundingMode::Down);
    /// assert_eq!(n.to_string(), "-2");
    ///
    /// let mut n = -Integer::trillion();
    /// n.div_round_assign(&Integer::from(3), RoundingMode::Floor);
    /// assert_eq!(n.to_string(), "-333333333334");
    ///
    /// let mut n = Integer::from(-10);
    /// n.div_round_assign(&Integer::from(4), RoundingMode::Up);
    /// assert_eq!(n.to_string(), "-3");
    ///
    /// let mut n = -Integer::trillion();
    /// n.div_round_assign(&Integer::from(3), RoundingMode::Ceiling);
    /// assert_eq!(n.to_string(), "-333333333333");
    ///
    /// let mut n = Integer::from(-10);
    /// n.div_round_assign(&Integer::from(5), RoundingMode::Exact);
    /// assert_eq!(n.to_string(), "-2");
    ///
    /// let mut n = Integer::from(-10);
    /// n.div_round_assign(Integer::from(3), RoundingMode::Nearest);
    /// assert_eq!(n.to_string(), "-3");
    ///
    /// let mut n = Integer::from(-20);
    /// n.div_round_assign(&Integer::from(3), RoundingMode::Nearest);
    /// assert_eq!(n.to_string(), "-7");
    ///
    /// let mut n = Integer::from(-10);
    /// n.div_round_assign(&Integer::from(4), RoundingMode::Nearest);
    /// assert_eq!(n.to_string(), "-2");
    ///
    /// let mut n = Integer::from(-14);
    /// n.div_round_assign(&Integer::from(4), RoundingMode::Nearest);
    /// assert_eq!(n.to_string(), "-4");
    ///
    /// let mut n = Integer::from(-10);
    /// n.div_round_assign(&Integer::from(-4), RoundingMode::Down);
    /// assert_eq!(n.to_string(), "2");
    ///
    /// let mut n = -Integer::trillion();
    /// n.div_round_assign(&Integer::from(-3), RoundingMode::Floor);
    /// assert_eq!(n.to_string(), "333333333333");
    ///
    /// let mut n = Integer::from(-10);
    /// n.div_round_assign(&Integer::from(-4), RoundingMode::Up);
    /// assert_eq!(n.to_string(), "3");
    ///
    /// let mut n = -Integer::trillion();
    /// n.div_round_assign(&Integer::from(-3), RoundingMode::Ceiling);
    /// assert_eq!(n.to_string(), "333333333334");
    ///
    /// let mut n = Integer::from(-10);
    /// n.div_round_assign(&Integer::from(-5), RoundingMode::Exact);
    /// assert_eq!(n.to_string(), "2");
    ///
    /// let mut n = Integer::from(-10);
    /// n.div_round_assign(&Integer::from(-3), RoundingMode::Nearest);
    /// assert_eq!(n.to_string(), "3");
    ///
    /// let mut n = Integer::from(-20);
    /// n.div_round_assign(&Integer::from(-3), RoundingMode::Nearest);
    /// assert_eq!(n.to_string(), "7");
    ///
    /// let mut n = Integer::from(-10);
    /// n.div_round_assign(&Integer::from(-4), RoundingMode::Nearest);
    /// assert_eq!(n.to_string(), "2");
    ///
    /// let mut n = Integer::from(-14);
    /// n.div_round_assign(&Integer::from(-4), RoundingMode::Nearest);
    /// assert_eq!(n.to_string(), "4");
    /// ```
    fn div_round_assign(&mut self, other: &'a Integer, rm: RoundingMode) {
        let q_sign = self.sign == other.sign;
        self.abs
            .div_round_assign(&other.abs, if q_sign { rm } else { -rm });
        self.sign = q_sign || self.abs == 0;
    }
}
