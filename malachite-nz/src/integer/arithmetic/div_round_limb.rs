use malachite_base::num::arithmetic::traits::{DivRound, DivRoundAssign};
use malachite_base::num::basic::traits::Zero;
use malachite_base::round::RoundingMode;

use integer::Integer;
use natural::Natural;
use platform::Limb;

impl DivRound<Limb> for Integer {
    type Output = Integer;

    /// Divides an `Integer` by a `Limb` and rounds according to a specified rounding mode, taking
    /// the `Integer` by value. See the `RoundingMode` documentation for details.
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
    /// use malachite_base::num::arithmetic::traits::DivRound;
    /// use malachite_base::round::RoundingMode;
    /// use malachite_nz::integer::Integer;
    ///
    /// fn main() {
    ///     assert_eq!(Integer::from(-10).div_round(4u32, RoundingMode::Down).to_string(), "-2");
    ///     assert_eq!((-Integer::trillion()).div_round(3u32, RoundingMode::Floor).to_string(),
    ///         "-333333333334");
    ///     assert_eq!(Integer::from(-10).div_round(4u32, RoundingMode::Up).to_string(), "-3");
    ///     assert_eq!((-Integer::trillion()).div_round(3u32, RoundingMode::Ceiling).to_string(),
    ///         "-333333333333");
    ///     assert_eq!(Integer::from(-10).div_round(5u32, RoundingMode::Exact).to_string(), "-2");
    ///     assert_eq!(Integer::from(-10).div_round(3u32, RoundingMode::Nearest).to_string(), "-3");
    ///     assert_eq!(Integer::from(-20).div_round(3u32, RoundingMode::Nearest).to_string(), "-7");
    ///     assert_eq!(Integer::from(-10).div_round(4u32, RoundingMode::Nearest).to_string(), "-2");
    ///     assert_eq!(Integer::from(-14).div_round(4u32, RoundingMode::Nearest).to_string(), "-4");
    /// }
    /// ```
    #[inline]
    fn div_round(mut self, other: Limb, rm: RoundingMode) -> Integer {
        self.div_round_assign(other, rm);
        self
    }
}

#[cfg(feature = "64_bit_limbs")]
impl DivRound<u32> for Integer {
    type Output = Integer;

    #[inline]
    fn div_round(self, other: u32, rm: RoundingMode) -> Integer {
        self.div_round(Limb::from(other), rm)
    }
}

impl<'a> DivRound<Limb> for &'a Integer {
    type Output = Integer;

    /// Divides an `Integer` by a `Limb` and rounds according to a specified rounding mode, taking
    /// the `Integer` by reference. See the `RoundingMode` documentation for details.
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
    /// use malachite_base::num::arithmetic::traits::DivRound;
    /// use malachite_base::round::RoundingMode;
    /// use malachite_nz::integer::Integer;
    ///
    /// fn main() {
    ///     assert_eq!((&Integer::from(-10)).div_round(4u32, RoundingMode::Down).to_string(), "-2");
    ///     assert_eq!((&-Integer::trillion()).div_round(3u32, RoundingMode::Floor).to_string(),
    ///         "-333333333334");
    ///     assert_eq!((&Integer::from(-10)).div_round(4u32, RoundingMode::Up).to_string(), "-3");
    ///     assert_eq!((&-Integer::trillion()).div_round(3u32, RoundingMode::Ceiling).to_string(),
    ///         "-333333333333");
    ///     assert_eq!((&Integer::from(-10)).div_round(5u32, RoundingMode::Exact).to_string(),
    ///         "-2");
    ///     assert_eq!((&Integer::from(-10)).div_round(3u32, RoundingMode::Nearest).to_string(),
    ///         "-3");
    ///     assert_eq!((&Integer::from(-20)).div_round(3u32, RoundingMode::Nearest).to_string(),
    ///         "-7");
    ///     assert_eq!((&Integer::from(-10)).div_round(4u32, RoundingMode::Nearest).to_string(),
    ///         "-2");
    ///     assert_eq!((&Integer::from(-14)).div_round(4u32, RoundingMode::Nearest).to_string(),
    ///         "-4");
    /// }
    /// ```
    fn div_round(self, other: Limb, rm: RoundingMode) -> Integer {
        let abs = if self.sign {
            (&self.abs).div_round(other, rm)
        } else {
            (&self.abs).div_round(other, -rm)
        };
        if abs == 0 as Limb {
            Integer::ZERO
        } else {
            Integer {
                sign: self.sign,
                abs,
            }
        }
    }
}

#[cfg(feature = "64_bit_limbs")]
impl<'a> DivRound<u32> for &'a Integer {
    type Output = Integer;

    #[inline]
    fn div_round(self, other: u32, rm: RoundingMode) -> Integer {
        self.div_round(Limb::from(other), rm)
    }
}

impl DivRound<Integer> for Limb {
    type Output = Integer;

    /// Divides a `Limb` by an `Integer` and rounds according to a specified rounding mode, taking
    /// the `Integer` by value. See the `RoundingMode` documentation for details.
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
    /// use malachite_base::num::arithmetic::traits::DivRound;
    /// use malachite_base::round::RoundingMode;
    /// use malachite_nz::integer::Integer;
    ///
    /// fn main() {
    ///     assert_eq!(10u32.div_round(Integer::from(-4), RoundingMode::Down).to_string(), "-2");
    ///     assert_eq!(1000u32.div_round(-Integer::trillion(), RoundingMode::Floor).to_string(),
    ///         "-1");
    ///     assert_eq!(10u32.div_round(Integer::from(-4), RoundingMode::Up).to_string(), "-3");
    ///     assert_eq!(1000u32.div_round(-Integer::trillion(), RoundingMode::Ceiling).to_string(),
    ///         "0");
    ///     assert_eq!(10u32.div_round(Integer::from(-5), RoundingMode::Exact).to_string(), "-2");
    ///     assert_eq!(10u32.div_round(Integer::from(-3), RoundingMode::Nearest).to_string(), "-3");
    ///     assert_eq!(20u32.div_round(Integer::from(-3), RoundingMode::Nearest).to_string(), "-7");
    ///     assert_eq!(10u32.div_round(Integer::from(-4), RoundingMode::Nearest).to_string(), "-2");
    ///     assert_eq!(14u32.div_round(Integer::from(-4), RoundingMode::Nearest).to_string(), "-4");
    /// }
    /// ```
    #[inline]
    fn div_round(self, other: Integer, rm: RoundingMode) -> Integer {
        self.div_round(&other, rm)
    }
}

#[cfg(feature = "64_bit_limbs")]
impl DivRound<Integer> for u32 {
    type Output = Integer;

    #[inline]
    fn div_round(self, other: Integer, rm: RoundingMode) -> Integer {
        Limb::from(self).div_round(other, rm)
    }
}

impl<'a> DivRound<&'a Integer> for Limb {
    type Output = Integer;

    /// Divides a `Limb` by an `Integer` and rounds according to a specified rounding mode, taking
    /// the `Integer` by reference. See the `RoundingMode` documentation for details.
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
    /// use malachite_base::num::arithmetic::traits::DivRound;
    /// use malachite_base::round::RoundingMode;
    /// use malachite_nz::integer::Integer;
    ///
    /// fn main() {
    ///     assert_eq!(10u32.div_round(&Integer::from(-4), RoundingMode::Down).to_string(), "-2");
    ///     assert_eq!(1000u32.div_round(&-Integer::trillion(), RoundingMode::Floor).to_string(),
    ///         "-1");
    ///     assert_eq!(10u32.div_round(&Integer::from(-4), RoundingMode::Up).to_string(), "-3");
    ///     assert_eq!(1000u32.div_round(&-Integer::trillion(), RoundingMode::Ceiling).to_string(),
    ///         "0");
    ///     assert_eq!(10u32.div_round(&Integer::from(-5), RoundingMode::Exact).to_string(), "-2");
    ///     assert_eq!(10u32.div_round(&Integer::from(-3), RoundingMode::Nearest).to_string(),
    ///         "-3");
    ///     assert_eq!(20u32.div_round(&Integer::from(-3), RoundingMode::Nearest).to_string(),
    ///         "-7");
    ///     assert_eq!(10u32.div_round(&Integer::from(-4), RoundingMode::Nearest).to_string(),
    ///         "-2");
    ///     assert_eq!(14u32.div_round(&Integer::from(-4), RoundingMode::Nearest).to_string(),
    ///         "-4");
    /// }
    /// ```
    fn div_round(self, other: &'a Integer, rm: RoundingMode) -> Integer {
        let abs = if other.sign {
            self.div_round(&other.abs, rm)
        } else {
            self.div_round(&other.abs, -rm)
        };
        if abs == 0 {
            Integer::ZERO
        } else {
            Integer {
                sign: other.sign,
                abs: Natural::from(abs),
            }
        }
    }
}

#[cfg(feature = "64_bit_limbs")]
impl<'a> DivRound<&'a Integer> for u32 {
    type Output = Integer;

    #[inline]
    fn div_round(self, other: &'a Integer, rm: RoundingMode) -> Integer {
        Limb::from(self).div_round(other, rm)
    }
}

impl DivRoundAssign<Limb> for Integer {
    /// Divides an `Integer` by a `Limb` in place and rounds according to a specified rounding mode.
    /// See the `RoundingMode` documentation for details.
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
    /// use malachite_base::num::arithmetic::traits::DivRoundAssign;
    /// use malachite_base::round::RoundingMode;
    /// use malachite_nz::integer::Integer;
    ///
    /// fn main() {
    ///     let mut n = Integer::from(-10);
    ///     n.div_round_assign(4u32, RoundingMode::Down);
    ///     assert_eq!(n.to_string(), "-2");
    ///
    ///     let mut n = -Integer::trillion();
    ///     n.div_round_assign(3u32, RoundingMode::Floor);
    ///     assert_eq!(n.to_string(), "-333333333334");
    ///
    ///     let mut n = Integer::from(-10);
    ///     n.div_round_assign(4u32, RoundingMode::Up);
    ///     assert_eq!(n.to_string(), "-3");
    ///
    ///     let mut n = -Integer::trillion();
    ///     n.div_round_assign(3u32, RoundingMode::Ceiling);
    ///     assert_eq!(n.to_string(), "-333333333333");
    ///
    ///     let mut n = Integer::from(-10);
    ///     n.div_round_assign(5u32, RoundingMode::Exact);
    ///     assert_eq!(n.to_string(), "-2");
    ///
    ///     let mut n = Integer::from(-10);
    ///     n.div_round_assign(3u32, RoundingMode::Nearest);
    ///     assert_eq!(n.to_string(), "-3");
    ///
    ///     let mut n = Integer::from(-20);
    ///     n.div_round_assign(3u32, RoundingMode::Nearest);
    ///     assert_eq!(n.to_string(), "-7");
    ///
    ///     let mut n = Integer::from(-10);
    ///     n.div_round_assign(4u32, RoundingMode::Nearest);
    ///     assert_eq!(n.to_string(), "-2");
    ///
    ///     let mut n = Integer::from(-14);
    ///     n.div_round_assign(4u32, RoundingMode::Nearest);
    ///     assert_eq!(n.to_string(), "-4");
    /// }
    /// ```
    fn div_round_assign(&mut self, other: Limb, rm: RoundingMode) {
        if self.sign {
            self.abs.div_round_assign(other, rm)
        } else {
            self.abs.div_round_assign(other, -rm)
        }
        if !self.sign && self.abs == 0 as Limb {
            self.sign = true;
        }
    }
}

#[cfg(feature = "64_bit_limbs")]
impl DivRoundAssign<u32> for Integer {
    #[inline]
    fn div_round_assign(&mut self, other: u32, rm: RoundingMode) {
        self.div_round_assign(Limb::from(other), rm)
    }
}
