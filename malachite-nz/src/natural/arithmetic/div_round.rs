use std::cmp::Ordering;

use malachite_base::num::arithmetic::traits::{
    DivAssignMod, DivMod, DivRound, DivRoundAssign, Parity,
};
use malachite_base::round::RoundingMode;

use natural::Natural;
use platform::Limb;

fn div_round_nearest(quotient: Natural, remainder: Natural, denominator: &Natural) -> Natural {
    let compare = (remainder << 1u64).cmp(denominator);
    if compare == Ordering::Greater || compare == Ordering::Equal && quotient.odd() {
        quotient + 1 as Limb
    } else {
        quotient
    }
}

fn div_round_assign_nearest(quotient: &mut Natural, remainder: Natural, denominator: &Natural) {
    let compare = (remainder << 1u64).cmp(denominator);
    if compare == Ordering::Greater || compare == Ordering::Equal && quotient.odd() {
        *quotient += 1 as Limb;
    }
}

impl DivRound<Natural> for Natural {
    type Output = Natural;

    /// Divides a `Natural` by a `Natural` and rounds according to a specified rounding mode, taking
    /// both `Natural`s by value. See the `RoundingMode` documentation for details.
    ///
    /// Time: Worst case O(n * log(n) * log(log(n)))
    ///
    /// Additional memory: Worst case O(n * log(n))
    ///
    /// where n = `self.significant_bits()`
    ///
    /// # Examples
    /// ```
    /// extern crate malachite_base;
    /// extern crate malachite_nz;
    ///
    /// use malachite_base::num::arithmetic::traits::DivRound;
    /// use malachite_base::round::RoundingMode;
    /// use malachite_nz::natural::Natural;
    ///
    /// fn main() {
    ///     assert_eq!(
    ///         Natural::from(10u32).div_round(Natural::from(4u32), RoundingMode::Down).to_string(),
    ///         "2"
    ///     );
    ///     assert_eq!(
    ///         Natural::trillion().div_round(Natural::from(3u32), RoundingMode::Floor).to_string(),
    ///         "333333333333"
    ///     );
    ///     assert_eq!(
    ///         Natural::from(10u32).div_round(Natural::from(4u32), RoundingMode::Up).to_string(),
    ///         "3"
    ///     );
    ///     assert_eq!(
    ///         Natural::trillion()
    ///             .div_round(Natural::from(3u32), RoundingMode::Ceiling).to_string(),
    ///         "333333333334");
    ///     assert_eq!(
    ///         Natural::from(10u32)
    ///             .div_round(Natural::from(5u32), RoundingMode::Exact).to_string(),
    ///         "2"
    ///     );
    ///     assert_eq!(
    ///         Natural::from(10u32)
    ///             .div_round(Natural::from(3u32), RoundingMode::Nearest).to_string(),
    ///         "3"
    ///     );
    ///     assert_eq!(
    ///         Natural::from(20u32)
    ///             .div_round(Natural::from(3u32), RoundingMode::Nearest).to_string(),
    ///         "7"
    ///     );
    ///     assert_eq!(
    ///         Natural::from(10u32)
    ///             .div_round(Natural::from(4u32), RoundingMode::Nearest).to_string(),
    ///         "2"
    ///     );
    ///     assert_eq!(
    ///         Natural::from(14u32)
    ///             .div_round(Natural::from(4u32), RoundingMode::Nearest).to_string(),
    ///         "4"
    ///     );
    /// }
    /// ```
    #[inline]
    fn div_round(mut self, other: Natural, rm: RoundingMode) -> Natural {
        self.div_round_assign(other, rm);
        self
    }
}

impl<'a> DivRound<&'a Natural> for Natural {
    type Output = Natural;

    /// Divides a `Natural` by a `Natural` and rounds according to a specified rounding mode, taking
    /// the first `Natural` by value and the second by reference. See the `RoundingMode`
    /// documentation for details.
    ///
    /// Time: Worst case O(n * log(n) * log(log(n)))
    ///
    /// Additional memory: Worst case O(n * log(n))
    ///
    /// where n = `self.significant_bits()`
    ///
    /// # Examples
    /// ```
    /// extern crate malachite_base;
    /// extern crate malachite_nz;
    ///
    /// use malachite_base::num::arithmetic::traits::DivRound;
    /// use malachite_base::round::RoundingMode;
    /// use malachite_nz::natural::Natural;
    ///
    /// fn main() {
    ///     assert_eq!(
    ///         Natural::from(10u32)
    ///             .div_round(&Natural::from(4u32), RoundingMode::Down).to_string(),
    ///         "2"
    ///     );
    ///     assert_eq!(
    ///         Natural::trillion()
    ///             .div_round(&Natural::from(3u32), RoundingMode::Floor).to_string(),
    ///         "333333333333"
    ///     );
    ///     assert_eq!(
    ///         Natural::from(10u32).div_round(&Natural::from(4u32), RoundingMode::Up).to_string(),
    ///         "3"
    ///     );
    ///     assert_eq!(
    ///         Natural::trillion()
    ///             .div_round(&Natural::from(3u32), RoundingMode::Ceiling).to_string(),
    ///         "333333333334");
    ///     assert_eq!(
    ///         Natural::from(10u32)
    ///             .div_round(&Natural::from(5u32), RoundingMode::Exact).to_string(),
    ///         "2"
    ///     );
    ///     assert_eq!(
    ///         Natural::from(10u32)
    ///             .div_round(&Natural::from(3u32), RoundingMode::Nearest).to_string(),
    ///         "3"
    ///     );
    ///     assert_eq!(
    ///         Natural::from(20u32)
    ///             .div_round(&Natural::from(3u32), RoundingMode::Nearest).to_string(),
    ///         "7"
    ///     );
    ///     assert_eq!(
    ///         Natural::from(10u32)
    ///             .div_round(&Natural::from(4u32), RoundingMode::Nearest).to_string(),
    ///         "2"
    ///     );
    ///     assert_eq!(
    ///         Natural::from(14u32)
    ///             .div_round(&Natural::from(4u32), RoundingMode::Nearest).to_string(),
    ///         "4"
    ///     );
    /// }
    /// ```
    #[inline]
    fn div_round(mut self, other: &'a Natural, rm: RoundingMode) -> Natural {
        self.div_round_assign(other, rm);
        self
    }
}

impl<'a> DivRound<Natural> for &'a Natural {
    type Output = Natural;

    /// Divides a `Natural` by a `Natural` and rounds according to a specified rounding mode, taking
    /// the first `Natural` by reference and the second by value. See the `RoundingMode`
    /// documentation for details.
    ///
    /// Time: Worst case O(n * log(n) * log(log(n)))
    ///
    /// Additional memory: Worst case O(n * log(n))
    ///
    /// where n = `self.significant_bits()`
    ///
    /// # Examples
    /// ```
    /// extern crate malachite_base;
    /// extern crate malachite_nz;
    ///
    /// use malachite_base::num::arithmetic::traits::DivRound;
    /// use malachite_base::round::RoundingMode;
    /// use malachite_nz::natural::Natural;
    ///
    /// fn main() {
    ///     assert_eq!(
    ///         (&Natural::from(10u32))
    ///             .div_round(Natural::from(4u32), RoundingMode::Down).to_string(),
    ///         "2"
    ///     );
    ///     assert_eq!(
    ///         (&Natural::trillion())
    ///             .div_round(Natural::from(3u32), RoundingMode::Floor).to_string(),
    ///         "333333333333"
    ///     );
    ///     assert_eq!(
    ///         (&Natural::from(10u32))
    ///             .div_round(Natural::from(4u32), RoundingMode::Up).to_string(),
    ///         "3"
    ///     );
    ///     assert_eq!(
    ///         (&Natural::trillion())
    ///             .div_round(Natural::from(3u32), RoundingMode::Ceiling).to_string(),
    ///         "333333333334");
    ///     assert_eq!(
    ///         (&Natural::from(10u32))
    ///             .div_round(Natural::from(5u32), RoundingMode::Exact).to_string(),
    ///         "2"
    ///     );
    ///     assert_eq!(
    ///         (&Natural::from(10u32))
    ///             .div_round(Natural::from(3u32), RoundingMode::Nearest).to_string(),
    ///         "3"
    ///     );
    ///     assert_eq!(
    ///         (&Natural::from(20u32))
    ///             .div_round(Natural::from(3u32), RoundingMode::Nearest).to_string(),
    ///         "7"
    ///     );
    ///     assert_eq!(
    ///         (&Natural::from(10u32))
    ///             .div_round(Natural::from(4u32), RoundingMode::Nearest).to_string(),
    ///         "2"
    ///     );
    ///     assert_eq!(
    ///         (&Natural::from(14u32))
    ///             .div_round(Natural::from(4u32), RoundingMode::Nearest).to_string(),
    ///         "4"
    ///     );
    /// }
    /// ```
    fn div_round(self, other: Natural, rm: RoundingMode) -> Natural {
        if rm == RoundingMode::Floor || rm == RoundingMode::Down {
            self / other
        } else {
            let (q, r) = self.div_mod(&other);
            if r == 0 as Limb {
                q
            } else {
                match rm {
                    RoundingMode::Ceiling | RoundingMode::Up => q + 1 as Limb,
                    RoundingMode::Exact => panic!("Division is not exact"),
                    RoundingMode::Nearest => div_round_nearest(q, r, &other),
                    _ => unreachable!(),
                }
            }
        }
    }
}

impl<'a, 'b> DivRound<&'b Natural> for &'a Natural {
    type Output = Natural;

    /// Divides a `Natural` by a `Natural` and rounds according to a specified rounding mode, taking
    /// both `Natural`s by reference. See the `RoundingMode` documentation for details.
    ///
    /// Time: Worst case O(n * log(n) * log(log(n)))
    ///
    /// Additional memory: Worst case O(n * log(n))
    ///
    /// where n = `self.significant_bits()`
    ///
    /// # Examples
    /// ```
    /// extern crate malachite_base;
    /// extern crate malachite_nz;
    ///
    /// use malachite_base::num::arithmetic::traits::DivRound;
    /// use malachite_base::round::RoundingMode;
    /// use malachite_nz::natural::Natural;
    ///
    /// fn main() {
    ///     assert_eq!(
    ///         (&Natural::from(10u32))
    ///             .div_round(&Natural::from(4u32), RoundingMode::Down).to_string(),
    ///         "2"
    ///     );
    ///     assert_eq!(
    ///         (&Natural::trillion())
    ///             .div_round(&Natural::from(3u32), RoundingMode::Floor).to_string(),
    ///         "333333333333"
    ///     );
    ///     assert_eq!(
    ///         (&Natural::from(10u32))
    ///             .div_round(&Natural::from(4u32), RoundingMode::Up).to_string(),
    ///         "3"
    ///     );
    ///     assert_eq!(
    ///         (&Natural::trillion())
    ///             .div_round(&Natural::from(3u32), RoundingMode::Ceiling).to_string(),
    ///         "333333333334");
    ///     assert_eq!(
    ///         (&Natural::from(10u32))
    ///             .div_round(&Natural::from(5u32), RoundingMode::Exact).to_string(),
    ///         "2"
    ///     );
    ///     assert_eq!(
    ///         (&Natural::from(10u32))
    ///             .div_round(&Natural::from(3u32), RoundingMode::Nearest).to_string(),
    ///         "3"
    ///     );
    ///     assert_eq!(
    ///         (&Natural::from(20u32))
    ///             .div_round(&Natural::from(3u32), RoundingMode::Nearest).to_string(),
    ///         "7"
    ///     );
    ///     assert_eq!(
    ///         (&Natural::from(10u32))
    ///             .div_round(&Natural::from(4u32), RoundingMode::Nearest).to_string(),
    ///         "2"
    ///     );
    ///     assert_eq!(
    ///         (&Natural::from(14u32))
    ///             .div_round(&Natural::from(4u32), RoundingMode::Nearest).to_string(),
    ///         "4"
    ///     );
    /// }
    /// ```
    fn div_round(self, other: &'b Natural, rm: RoundingMode) -> Natural {
        if rm == RoundingMode::Floor || rm == RoundingMode::Down {
            self / other
        } else {
            let (q, r) = self.div_mod(other);
            if r == 0 as Limb {
                q
            } else {
                match rm {
                    RoundingMode::Ceiling | RoundingMode::Up => q + 1 as Limb,
                    RoundingMode::Exact => panic!("Division is not exact: {} / {}", self, other),
                    RoundingMode::Nearest => div_round_nearest(q, r, other),
                    _ => unreachable!(),
                }
            }
        }
    }
}

impl DivRoundAssign<Natural> for Natural {
    /// Divides a `Natural` by a `Natural` in place and rounds according to a specified rounding
    /// mode, taking the `Natural` on the RHS by value. See the `RoundingMode` documentation for
    /// details.
    ///
    /// Time: Worst case O(n * log(n) * log(log(n)))
    ///
    /// Additional memory: Worst case O(n * log(n))
    ///
    /// where n = `self.significant_bits()`
    ///
    /// # Examples
    /// ```
    /// extern crate malachite_base;
    /// extern crate malachite_nz;
    ///
    /// use malachite_base::num::arithmetic::traits::DivRoundAssign;
    /// use malachite_base::round::RoundingMode;
    /// use malachite_nz::natural::Natural;
    ///
    /// fn main() {
    ///     let mut n = Natural::from(10u32);
    ///     n.div_round_assign(Natural::from(4u32), RoundingMode::Down);
    ///     assert_eq!(n.to_string(), "2");
    ///
    ///     let mut n = Natural::trillion();
    ///     n.div_round_assign(Natural::from(3u32), RoundingMode::Floor);
    ///     assert_eq!(n.to_string(), "333333333333");
    ///
    ///     let mut n = Natural::from(10u32);
    ///     n.div_round_assign(Natural::from(4u32), RoundingMode::Up);
    ///     assert_eq!(n.to_string(), "3");
    ///
    ///     let mut n = Natural::trillion();
    ///     n.div_round_assign(Natural::from(3u32), RoundingMode::Ceiling);
    ///     assert_eq!(n.to_string(), "333333333334");
    ///
    ///     let mut n = Natural::from(10u32);
    ///     n.div_round_assign(Natural::from(5u32), RoundingMode::Exact);
    ///     assert_eq!(n.to_string(), "2");
    ///
    ///     let mut n = Natural::from(10u32);
    ///     n.div_round_assign(Natural::from(3u32), RoundingMode::Nearest);
    ///     assert_eq!(n.to_string(), "3");
    ///
    ///     let mut n = Natural::from(20u32);
    ///     n.div_round_assign(Natural::from(3u32), RoundingMode::Nearest);
    ///     assert_eq!(n.to_string(), "7");
    ///
    ///     let mut n = Natural::from(10u32);
    ///     n.div_round_assign(Natural::from(4u32), RoundingMode::Nearest);
    ///     assert_eq!(n.to_string(), "2");
    ///
    ///     let mut n = Natural::from(14u32);
    ///     n.div_round_assign(Natural::from(4u32), RoundingMode::Nearest);
    ///     assert_eq!(n.to_string(), "4");
    /// }
    /// ```
    fn div_round_assign(&mut self, other: Natural, rm: RoundingMode) {
        if rm == RoundingMode::Floor || rm == RoundingMode::Down {
            *self /= other;
        } else {
            let r = self.div_assign_mod(&other);
            if r != 0 as Limb {
                match rm {
                    RoundingMode::Ceiling | RoundingMode::Up => {
                        *self += 1 as Limb;
                    }
                    RoundingMode::Exact => panic!("Division is not exact"),
                    RoundingMode::Nearest => div_round_assign_nearest(self, r, &other),
                    _ => unreachable!(),
                }
            }
        }
    }
}

impl<'a> DivRoundAssign<&'a Natural> for Natural {
    /// Divides a `Natural` by a `Natural` in place and rounds according to a specified rounding
    /// mode, taking the `Natural` on the RHS by reference. See the `RoundingMode` documentation for
    /// details.
    ///
    /// Time: Worst case O(n * log(n) * log(log(n)))
    ///
    /// Additional memory: Worst case O(n * log(n))
    ///
    /// where n = `self.significant_bits()`
    ///
    /// # Examples
    /// ```
    /// extern crate malachite_base;
    /// extern crate malachite_nz;
    ///
    /// use malachite_base::num::arithmetic::traits::DivRoundAssign;
    /// use malachite_base::round::RoundingMode;
    /// use malachite_nz::natural::Natural;
    ///
    /// fn main() {
    ///     let mut n = Natural::from(10u32);
    ///     n.div_round_assign(&Natural::from(4u32), RoundingMode::Down);
    ///     assert_eq!(n.to_string(), "2");
    ///
    ///     let mut n = Natural::trillion();
    ///     n.div_round_assign(&Natural::from(3u32), RoundingMode::Floor);
    ///     assert_eq!(n.to_string(), "333333333333");
    ///
    ///     let mut n = Natural::from(10u32);
    ///     n.div_round_assign(&Natural::from(4u32), RoundingMode::Up);
    ///     assert_eq!(n.to_string(), "3");
    ///
    ///     let mut n = Natural::trillion();
    ///     n.div_round_assign(&Natural::from(3u32), RoundingMode::Ceiling);
    ///     assert_eq!(n.to_string(), "333333333334");
    ///
    ///     let mut n = Natural::from(10u32);
    ///     n.div_round_assign(&Natural::from(5u32), RoundingMode::Exact);
    ///     assert_eq!(n.to_string(), "2");
    ///
    ///     let mut n = Natural::from(10u32);
    ///     n.div_round_assign(&Natural::from(3u32), RoundingMode::Nearest);
    ///     assert_eq!(n.to_string(), "3");
    ///
    ///     let mut n = Natural::from(20u32);
    ///     n.div_round_assign(&Natural::from(3u32), RoundingMode::Nearest);
    ///     assert_eq!(n.to_string(), "7");
    ///
    ///     let mut n = Natural::from(10u32);
    ///     n.div_round_assign(&Natural::from(4u32), RoundingMode::Nearest);
    ///     assert_eq!(n.to_string(), "2");
    ///
    ///     let mut n = Natural::from(14u32);
    ///     n.div_round_assign(&Natural::from(4u32), RoundingMode::Nearest);
    ///     assert_eq!(n.to_string(), "4");
    /// }
    /// ```
    fn div_round_assign(&mut self, other: &'a Natural, rm: RoundingMode) {
        if rm == RoundingMode::Floor || rm == RoundingMode::Down {
            *self /= other;
        } else {
            let r = self.div_assign_mod(other);
            if r != 0 as Limb {
                match rm {
                    RoundingMode::Ceiling | RoundingMode::Up => {
                        *self += 1 as Limb;
                    }
                    RoundingMode::Exact => panic!("Division is not exact"),
                    RoundingMode::Nearest => div_round_assign_nearest(self, r, other),
                    _ => unreachable!(),
                }
            }
        }
    }
}
