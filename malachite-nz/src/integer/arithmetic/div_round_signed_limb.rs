use integer::Integer;
use malachite_base::num::{DivRound, DivRoundAssign, UnsignedAbs, Zero};
use malachite_base::round::RoundingMode;
use natural::Natural;
use platform::SignedLimb;

impl DivRound<SignedLimb> for Integer {
    type Output = Integer;

    /// Divides an `Integer` by a `SignedLimb` and rounds according to a specified rounding mode,
    /// taking the `Integer` by value. See the `RoundingMode` documentation for details.
    ///
    /// Time: worst case O(n)
    ///
    /// Additional memory: worst case O(1)
    ///
    /// where n = `other.significant_bits()`
    ///
    /// # Examples
    /// ```
    /// extern crate malachite_base;
    /// extern crate malachite_nz;
    ///
    /// use malachite_base::num::DivRound;
    /// use malachite_base::round::RoundingMode;
    /// use malachite_nz::integer::Integer;
    ///
    /// fn main() {
    ///     assert_eq!(Integer::from(-10).div_round(4i32, RoundingMode::Down).to_string(), "-2");
    ///     assert_eq!((-Integer::trillion()).div_round(3i32, RoundingMode::Floor).to_string(),
    ///         "-333333333334");
    ///     assert_eq!(Integer::from(-10).div_round(4i32, RoundingMode::Up).to_string(), "-3");
    ///     assert_eq!((-Integer::trillion()).div_round(3i32, RoundingMode::Ceiling).to_string(),
    ///         "-333333333333");
    ///     assert_eq!(Integer::from(-10).div_round(5i32, RoundingMode::Exact).to_string(), "-2");
    ///     assert_eq!(Integer::from(-10).div_round(3i32, RoundingMode::Nearest).to_string(), "-3");
    ///     assert_eq!(Integer::from(-20).div_round(3i32, RoundingMode::Nearest).to_string(), "-7");
    ///     assert_eq!(Integer::from(-10).div_round(4i32, RoundingMode::Nearest).to_string(), "-2");
    ///     assert_eq!(Integer::from(-14).div_round(4i32, RoundingMode::Nearest).to_string(), "-4");
    ///
    ///     assert_eq!(Integer::from(-10).div_round(-4i32, RoundingMode::Down).to_string(), "2");
    ///     assert_eq!((-Integer::trillion()).div_round(-3i32, RoundingMode::Floor).to_string(),
    ///         "333333333333");
    ///     assert_eq!(Integer::from(-10).div_round(-4i32, RoundingMode::Up).to_string(), "3");
    ///     assert_eq!((-Integer::trillion()).div_round(-3i32, RoundingMode::Ceiling).to_string(),
    ///         "333333333334");
    ///     assert_eq!(Integer::from(-10).div_round(-5i32, RoundingMode::Exact).to_string(), "2");
    ///     assert_eq!(Integer::from(-10).div_round(-3i32, RoundingMode::Nearest).to_string(), "3");
    ///     assert_eq!(Integer::from(-20).div_round(-3i32, RoundingMode::Nearest).to_string(), "7");
    ///     assert_eq!(Integer::from(-10).div_round(-4i32, RoundingMode::Nearest).to_string(), "2");
    ///     assert_eq!(Integer::from(-14).div_round(-4i32, RoundingMode::Nearest).to_string(), "4");
    /// }
    /// ```
    fn div_round(mut self, other: SignedLimb, rm: RoundingMode) -> Integer {
        self.div_round_assign(other, rm);
        self
    }
}

impl<'a> DivRound<SignedLimb> for &'a Integer {
    type Output = Integer;

    /// Divides an `Integer` by a `SignedLimb` and rounds according to a specified rounding mode,
    /// taking the `Integer` by reference. See the `RoundingMode` documentation for details.
    ///
    /// Time: worst case O(n)
    ///
    /// Additional memory: worst case O(n)
    ///
    /// where n = `other.significant_bits()`
    ///
    /// # Examples
    /// ```
    /// extern crate malachite_base;
    /// extern crate malachite_nz;
    ///
    /// use malachite_base::num::DivRound;
    /// use malachite_base::round::RoundingMode;
    /// use malachite_nz::integer::Integer;
    ///
    /// fn main() {
    ///     assert_eq!((&Integer::from(-10)).div_round(4i32, RoundingMode::Down).to_string(), "-2");
    ///     assert_eq!((&-Integer::trillion()).div_round(3i32, RoundingMode::Floor).to_string(),
    ///         "-333333333334");
    ///     assert_eq!((&Integer::from(-10)).div_round(4i32, RoundingMode::Up).to_string(), "-3");
    ///     assert_eq!((&-Integer::trillion()).div_round(3i32, RoundingMode::Ceiling).to_string(),
    ///         "-333333333333");
    ///     assert_eq!((&Integer::from(-10)).div_round(5i32, RoundingMode::Exact).to_string(),
    ///         "-2");
    ///     assert_eq!((&Integer::from(-10)).div_round(3i32, RoundingMode::Nearest).to_string(),
    ///         "-3");
    ///     assert_eq!((&Integer::from(-20)).div_round(3i32, RoundingMode::Nearest).to_string(),
    ///         "-7");
    ///     assert_eq!((&Integer::from(-10)).div_round(4i32, RoundingMode::Nearest).to_string(),
    ///         "-2");
    ///     assert_eq!((&Integer::from(-14)).div_round(4i32, RoundingMode::Nearest).to_string(),
    ///         "-4");
    ///
    ///     assert_eq!(Integer::from(-10).div_round(-4i32, RoundingMode::Down).to_string(), "2");
    ///     assert_eq!((&-Integer::trillion()).div_round(-3i32, RoundingMode::Floor).to_string(),
    ///         "333333333333");
    ///     assert_eq!((&Integer::from(-10)).div_round(-4i32, RoundingMode::Up).to_string(), "3");
    ///     assert_eq!((&-Integer::trillion()).div_round(-3i32, RoundingMode::Ceiling).to_string(),
    ///         "333333333334");
    ///     assert_eq!((&Integer::from(-10)).div_round(-5i32, RoundingMode::Exact).to_string(),
    ///         "2");
    ///     assert_eq!((&Integer::from(-10)).div_round(-3i32, RoundingMode::Nearest).to_string(),
    ///         "3");
    ///     assert_eq!((&Integer::from(-20)).div_round(-3i32, RoundingMode::Nearest).to_string(),
    ///         "7");
    ///     assert_eq!((&Integer::from(-10)).div_round(-4i32, RoundingMode::Nearest).to_string(),
    ///         "2");
    ///     assert_eq!((&Integer::from(-14)).div_round(-4i32, RoundingMode::Nearest).to_string(),
    ///         "4");
    /// }
    /// ```
    fn div_round(self, other: SignedLimb, rm: RoundingMode) -> Integer {
        let result_sign = self.sign == (other >= 0);
        let abs = if result_sign {
            (&self.abs).div_round(other.unsigned_abs(), rm)
        } else {
            (&self.abs).div_round(other.unsigned_abs(), -rm)
        };
        if abs == 0 {
            Integer::ZERO
        } else {
            Integer {
                sign: result_sign,
                abs,
            }
        }
    }
}

impl DivRound<Integer> for SignedLimb {
    type Output = Integer;

    /// Divides a `SignedLimb` by an `Integer` and rounds according to a specified rounding mode,
    /// taking the `Integer` by value. See the `RoundingMode` documentation for details.
    ///
    /// Time: worst case O(n)
    ///
    /// Additional memory: worst case O(1)
    ///
    /// where n = `other.significant_bits()`
    ///
    /// # Examples
    /// ```
    /// extern crate malachite_base;
    /// extern crate malachite_nz;
    ///
    /// use malachite_base::num::DivRound;
    /// use malachite_base::round::RoundingMode;
    /// use malachite_nz::integer::Integer;
    ///
    /// fn main() {
    ///     assert_eq!(10i32.div_round(Integer::from(-4), RoundingMode::Down).to_string(), "-2");
    ///     assert_eq!(1000i32.div_round(-Integer::trillion(), RoundingMode::Floor).to_string(),
    ///         "-1");
    ///     assert_eq!(10i32.div_round(Integer::from(-4), RoundingMode::Up).to_string(), "-3");
    ///     assert_eq!(1000i32.div_round(-Integer::trillion(), RoundingMode::Ceiling).to_string(),
    ///         "0");
    ///     assert_eq!(10i32.div_round(Integer::from(-5), RoundingMode::Exact).to_string(), "-2");
    ///     assert_eq!(10i32.div_round(Integer::from(-3), RoundingMode::Nearest).to_string(), "-3");
    ///     assert_eq!(20i32.div_round(Integer::from(-3), RoundingMode::Nearest).to_string(), "-7");
    ///     assert_eq!(10i32.div_round(Integer::from(-4), RoundingMode::Nearest).to_string(), "-2");
    ///     assert_eq!(14i32.div_round(Integer::from(-4), RoundingMode::Nearest).to_string(), "-4");
    ///
    ///     assert_eq!((-10i32).div_round(Integer::from(-4), RoundingMode::Down).to_string(), "2");
    ///     assert_eq!((-1000i32).div_round(-Integer::trillion(), RoundingMode::Floor).to_string(),
    ///         "0");
    ///     assert_eq!((-10i32).div_round(Integer::from(-4), RoundingMode::Up).to_string(), "3");
    ///     assert_eq!((-1000i32).div_round(-Integer::trillion(),
    ///         RoundingMode::Ceiling).to_string(), "1");
    ///     assert_eq!((-10i32).div_round(Integer::from(-5), RoundingMode::Exact).to_string(), "2");
    ///     assert_eq!((-10i32).div_round(Integer::from(-3), RoundingMode::Nearest).to_string(),
    ///         "3");
    ///     assert_eq!((-20i32).div_round(Integer::from(-3), RoundingMode::Nearest).to_string(),
    ///         "7");
    ///     assert_eq!((-10i32).div_round(Integer::from(-4), RoundingMode::Nearest).to_string(),
    ///         "2");
    ///     assert_eq!((-14i32).div_round(Integer::from(-4), RoundingMode::Nearest).to_string(),
    ///         "4");
    /// }
    /// ```
    fn div_round(self, other: Integer, rm: RoundingMode) -> Integer {
        self.div_round(&other, rm)
    }
}

impl<'a> DivRound<&'a Integer> for SignedLimb {
    type Output = Integer;

    /// Divides a `SignedLimb` by an `Integer` and rounds according to a specified rounding mode,
    /// taking the `Integer` by reference. See the `RoundingMode` documentation for details.
    ///
    /// Time: worst case O(n)
    ///
    /// Additional memory: worst case O(1)
    ///
    /// where n = `other.significant_bits()`
    ///
    /// # Examples
    /// ```
    /// extern crate malachite_base;
    /// extern crate malachite_nz;
    ///
    /// use malachite_base::num::DivRound;
    /// use malachite_base::round::RoundingMode;
    /// use malachite_nz::integer::Integer;
    ///
    /// fn main() {
    ///     assert_eq!(10i32.div_round(&Integer::from(-4), RoundingMode::Down).to_string(), "-2");
    ///     assert_eq!(1000i32.div_round(&-Integer::trillion(), RoundingMode::Floor).to_string(),
    ///         "-1");
    ///     assert_eq!(10i32.div_round(&Integer::from(-4), RoundingMode::Up).to_string(), "-3");
    ///     assert_eq!(1000i32.div_round(&-Integer::trillion(), RoundingMode::Ceiling).to_string(),
    ///         "0");
    ///     assert_eq!(10i32.div_round(&Integer::from(-5), RoundingMode::Exact).to_string(), "-2");
    ///     assert_eq!(10i32.div_round(&Integer::from(-3), RoundingMode::Nearest).to_string(),
    ///         "-3");
    ///     assert_eq!(20i32.div_round(&Integer::from(-3), RoundingMode::Nearest).to_string(),
    ///         "-7");
    ///     assert_eq!(10i32.div_round(&Integer::from(-4), RoundingMode::Nearest).to_string(),
    ///         "-2");
    ///     assert_eq!(14i32.div_round(&Integer::from(-4), RoundingMode::Nearest).to_string(),
    ///         "-4");
    ///
    ///     assert_eq!((-10i32).div_round(&Integer::from(-4), RoundingMode::Down).to_string(), "2");
    ///     assert_eq!((-1000i32).div_round(&-Integer::trillion(), RoundingMode::Floor).to_string(),
    ///         "0");
    ///     assert_eq!((-10i32).div_round(&Integer::from(-4), RoundingMode::Up).to_string(), "3");
    ///     assert_eq!((-1000i32).div_round(&-Integer::trillion(),
    ///         RoundingMode::Ceiling).to_string(), "1");
    ///     assert_eq!((-10i32).div_round(&Integer::from(-5), RoundingMode::Exact).to_string(),
    ///         "2");
    ///     assert_eq!((-10i32).div_round(&Integer::from(-3), RoundingMode::Nearest).to_string(),
    ///         "3");
    ///     assert_eq!((-20i32).div_round(&Integer::from(-3), RoundingMode::Nearest).to_string(),
    ///         "7");
    ///     assert_eq!((-10i32).div_round(&Integer::from(-4), RoundingMode::Nearest).to_string(),
    ///         "2");
    ///     assert_eq!((-14i32).div_round(&Integer::from(-4), RoundingMode::Nearest).to_string(),
    ///         "4");
    /// }
    /// ```
    fn div_round(self, other: &'a Integer, rm: RoundingMode) -> Integer {
        let result_sign = (self >= 0) == other.sign;
        let abs = if result_sign {
            self.unsigned_abs().div_round(&other.abs, rm)
        } else {
            self.unsigned_abs().div_round(&other.abs, -rm)
        };
        if abs == 0 {
            Integer::ZERO
        } else {
            Integer {
                sign: result_sign,
                abs: Natural::from(abs),
            }
        }
    }
}

impl DivRoundAssign<SignedLimb> for Integer {
    /// Divides an `Integer` by a `SignedLimb` in place and rounds according to a specified rounding
    /// mode. See the `RoundingMode` documentation for details.
    ///
    /// Time: worst case O(n)
    ///
    /// Additional memory: worst case O(1)
    ///
    /// where n = `other.significant_bits()`
    ///
    /// # Examples
    /// ```
    /// extern crate malachite_base;
    /// extern crate malachite_nz;
    ///
    /// use malachite_base::num::DivRoundAssign;
    /// use malachite_base::round::RoundingMode;
    /// use malachite_nz::integer::Integer;
    ///
    /// fn main() {
    ///     let mut n = Integer::from(-10);
    ///     n.div_round_assign(4i32, RoundingMode::Down);
    ///     assert_eq!(n.to_string(), "-2");
    ///
    ///     let mut n = -Integer::trillion();
    ///     n.div_round_assign(3i32, RoundingMode::Floor);
    ///     assert_eq!(n.to_string(), "-333333333334");
    ///
    ///     let mut n = Integer::from(-10);
    ///     n.div_round_assign(4i32, RoundingMode::Up);
    ///     assert_eq!(n.to_string(), "-3");
    ///
    ///     let mut n = -Integer::trillion();
    ///     n.div_round_assign(3i32, RoundingMode::Ceiling);
    ///     assert_eq!(n.to_string(), "-333333333333");
    ///
    ///     let mut n = Integer::from(-10);
    ///     n.div_round_assign(5i32, RoundingMode::Exact);
    ///     assert_eq!(n.to_string(), "-2");
    ///
    ///     let mut n = Integer::from(-10);
    ///     n.div_round_assign(3i32, RoundingMode::Nearest);
    ///     assert_eq!(n.to_string(), "-3");
    ///
    ///     let mut n = Integer::from(-20);
    ///     n.div_round_assign(3i32, RoundingMode::Nearest);
    ///     assert_eq!(n.to_string(), "-7");
    ///
    ///     let mut n = Integer::from(-10);
    ///     n.div_round_assign(4i32, RoundingMode::Nearest);
    ///     assert_eq!(n.to_string(), "-2");
    ///
    ///     let mut n = Integer::from(-14);
    ///     n.div_round_assign(4i32, RoundingMode::Nearest);
    ///     assert_eq!(n.to_string(), "-4");
    ///
    ///     let mut n = Integer::from(-10);
    ///     n.div_round_assign(-4i32, RoundingMode::Down);
    ///     assert_eq!(n.to_string(), "2");
    ///
    ///     let mut n = -Integer::trillion();
    ///     n.div_round_assign(-3i32, RoundingMode::Floor);
    ///     assert_eq!(n.to_string(), "333333333333");
    ///
    ///     let mut n = Integer::from(-10);
    ///     n.div_round_assign(-4i32, RoundingMode::Up);
    ///     assert_eq!(n.to_string(), "3");
    ///
    ///     let mut n = -Integer::trillion();
    ///     n.div_round_assign(-3i32, RoundingMode::Ceiling);
    ///     assert_eq!(n.to_string(), "333333333334");
    ///
    ///     let mut n = Integer::from(-10);
    ///     n.div_round_assign(-5i32, RoundingMode::Exact);
    ///     assert_eq!(n.to_string(), "2");
    ///
    ///     let mut n = Integer::from(-10);
    ///     n.div_round_assign(-3i32, RoundingMode::Nearest);
    ///     assert_eq!(n.to_string(), "3");
    ///
    ///     let mut n = Integer::from(-20);
    ///     n.div_round_assign(-3i32, RoundingMode::Nearest);
    ///     assert_eq!(n.to_string(), "7");
    ///
    ///     let mut n = Integer::from(-10);
    ///     n.div_round_assign(-4i32, RoundingMode::Nearest);
    ///     assert_eq!(n.to_string(), "2");
    ///
    ///     let mut n = Integer::from(-14);
    ///     n.div_round_assign(-4i32, RoundingMode::Nearest);
    ///     assert_eq!(n.to_string(), "4");
    /// }
    /// ```
    fn div_round_assign(&mut self, other: SignedLimb, rm: RoundingMode) {
        let result_sign = self.sign == (other >= 0);
        if result_sign {
            self.abs.div_round_assign(other.unsigned_abs(), rm)
        } else {
            self.abs.div_round_assign(other.unsigned_abs(), -rm)
        }
        self.sign = result_sign || self.abs == 0;
    }
}
