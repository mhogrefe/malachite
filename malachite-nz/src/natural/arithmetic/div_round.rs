use malachite_base::num::arithmetic::traits::{
    DivAssignMod, DivMod, DivRound, DivRoundAssign, Parity,
};
use malachite_base::num::basic::integers::PrimitiveInt;
use malachite_base::num::basic::traits::{Iverson, One};
use malachite_base::rounding_modes::RoundingMode;
use natural::Natural;
use platform::Limb;
use std::cmp::Ordering;

/// Interpreting a slice of `Limb`s as the limbs (in ascending order) of a `Natural`, returns the
/// quotient limbs of a `Limb` divided by the `Natural` and rounded according to a specified
/// `RoundingMode`. The limb slice must have at least two elements and cannot have any trailing
/// zeros.
///
/// This function returns a `None` iff the rounding mode is `Exact` but the remainder of the
/// division would be nonzero.
///
/// Note that this function may only return `None`, `Some(0)`, or `Some(1)` because of the
/// restrictions placed on the input slice.
///
/// Time: worst case O(1)
///
/// Additional memory: worst case O(1)
///
/// # Examples
/// ```
/// extern crate malachite_base;
/// extern crate malachite_nz;
///
/// use malachite_base::rounding_modes::RoundingMode;
/// use malachite_nz::natural::arithmetic::div_round::limbs_limb_div_round_limbs;
/// use malachite_nz::natural::Natural;
///
/// assert_eq!(limbs_limb_div_round_limbs(789, &[123, 456], RoundingMode::Down), Some(0));
/// assert_eq!(limbs_limb_div_round_limbs(789, &[123, 456], RoundingMode::Floor), Some(0));
/// assert_eq!(limbs_limb_div_round_limbs(789, &[123, 456], RoundingMode::Up), Some(1));
/// assert_eq!(limbs_limb_div_round_limbs(789, &[123, 456], RoundingMode::Ceiling), Some(1));
/// assert_eq!(limbs_limb_div_round_limbs(0, &[123, 456], RoundingMode::Exact), Some(0));
/// assert_eq!(limbs_limb_div_round_limbs(789, &[123, 456], RoundingMode::Exact), None);
/// assert_eq!(limbs_limb_div_round_limbs(789, &[123, 456], RoundingMode::Nearest), Some(0));
/// assert_eq!(limbs_limb_div_round_limbs(u32::MAX, &[123, 1], RoundingMode::Nearest), Some(1));
/// assert_eq!(limbs_limb_div_round_limbs(u32::MAX, &[u32::MAX, 1],
///     RoundingMode::Nearest), Some(0));
///
/// assert_eq!(limbs_limb_div_round_limbs(u32::MAX, &[u32::MAX - 1, 1],
///     RoundingMode::Nearest), Some(0));
///
/// assert_eq!(limbs_limb_div_round_limbs(u32::MAX, &[0xfffffffd, 1],
///     RoundingMode::Nearest), Some(1));
/// ```
#[doc(hidden)]
pub fn limbs_limb_div_round_limbs(n: Limb, ds: &[Limb], rm: RoundingMode) -> Option<Limb> {
    if n == 0 {
        Some(0)
    } else {
        match rm {
            RoundingMode::Down | RoundingMode::Floor => Some(0),
            RoundingMode::Up | RoundingMode::Ceiling => Some(1),
            RoundingMode::Exact => None,
            // 1 if 2 * n > Natural::from_limbs_asc(ds); otherwise, 0
            RoundingMode::Nearest => Some(Limb::iverson(
                ds.len() == 2 && ds[1] == 1 && n.get_highest_bit() && (n << 1) > ds[0],
            )),
        }
    }
}

fn div_round_nearest(q: Natural, r: Natural, d: &Natural) -> Natural {
    let compare = (r << 1u64).cmp(d);
    if compare == Ordering::Greater || compare == Ordering::Equal && q.odd() {
        q.add_limb(1)
    } else {
        q
    }
}

fn div_round_assign_nearest(q: &mut Natural, r: Natural, d: &Natural) {
    let compare = (r << 1u64).cmp(d);
    if compare == Ordering::Greater || compare == Ordering::Equal && q.odd() {
        *q += Natural::ONE;
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
    /// # Panics
    /// Panics if `other` is zero, or if `rm` is `Exact` but `self` is not divisible by `other`.
    ///
    /// # Examples
    /// ```
    /// extern crate malachite_base;
    /// extern crate malachite_nz;
    ///
    /// use malachite_base::num::arithmetic::traits::DivRound;
    /// use malachite_base::rounding_modes::RoundingMode;
    /// use malachite_nz::natural::Natural;
    ///
    /// assert_eq!(
    ///     Natural::from(10u32).div_round(Natural::from(4u32), RoundingMode::Down).to_string(),
    ///     "2"
    /// );
    /// assert_eq!(
    ///     Natural::trillion().div_round(Natural::from(3u32), RoundingMode::Floor).to_string(),
    ///     "333333333333"
    /// );
    /// assert_eq!(
    ///     Natural::from(10u32).div_round(Natural::from(4u32), RoundingMode::Up).to_string(),
    ///     "3"
    /// );
    /// assert_eq!(
    ///     Natural::trillion()
    ///         .div_round(Natural::from(3u32), RoundingMode::Ceiling).to_string(),
    ///     "333333333334");
    /// assert_eq!(
    ///     Natural::from(10u32)
    ///         .div_round(Natural::from(5u32), RoundingMode::Exact).to_string(),
    ///     "2"
    /// );
    /// assert_eq!(
    ///     Natural::from(10u32)
    ///         .div_round(Natural::from(3u32), RoundingMode::Nearest).to_string(),
    ///     "3"
    /// );
    /// assert_eq!(
    ///     Natural::from(20u32)
    ///         .div_round(Natural::from(3u32), RoundingMode::Nearest).to_string(),
    ///     "7"
    /// );
    /// assert_eq!(
    ///     Natural::from(10u32)
    ///         .div_round(Natural::from(4u32), RoundingMode::Nearest).to_string(),
    ///     "2"
    /// );
    /// assert_eq!(
    ///     Natural::from(14u32)
    ///         .div_round(Natural::from(4u32), RoundingMode::Nearest).to_string(),
    ///     "4"
    /// );
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
    /// # Panics
    /// Panics if `other` is zero, or if `rm` is `Exact` but `self` is not divisible by `other`.
    ///
    /// # Examples
    /// ```
    /// extern crate malachite_base;
    /// extern crate malachite_nz;
    ///
    /// use malachite_base::num::arithmetic::traits::DivRound;
    /// use malachite_base::rounding_modes::RoundingMode;
    /// use malachite_nz::natural::Natural;
    ///
    /// assert_eq!(
    ///     Natural::from(10u32)
    ///         .div_round(&Natural::from(4u32), RoundingMode::Down).to_string(),
    ///     "2"
    /// );
    /// assert_eq!(
    ///     Natural::trillion()
    ///         .div_round(&Natural::from(3u32), RoundingMode::Floor).to_string(),
    ///     "333333333333"
    /// );
    /// assert_eq!(
    ///     Natural::from(10u32).div_round(&Natural::from(4u32), RoundingMode::Up).to_string(),
    ///     "3"
    /// );
    /// assert_eq!(
    ///     Natural::trillion()
    ///         .div_round(&Natural::from(3u32), RoundingMode::Ceiling).to_string(),
    ///     "333333333334");
    /// assert_eq!(
    ///     Natural::from(10u32)
    ///         .div_round(&Natural::from(5u32), RoundingMode::Exact).to_string(),
    ///     "2"
    /// );
    /// assert_eq!(
    ///     Natural::from(10u32)
    ///         .div_round(&Natural::from(3u32), RoundingMode::Nearest).to_string(),
    ///     "3"
    /// );
    /// assert_eq!(
    ///     Natural::from(20u32)
    ///         .div_round(&Natural::from(3u32), RoundingMode::Nearest).to_string(),
    ///     "7"
    /// );
    /// assert_eq!(
    ///     Natural::from(10u32)
    ///         .div_round(&Natural::from(4u32), RoundingMode::Nearest).to_string(),
    ///     "2"
    /// );
    /// assert_eq!(
    ///     Natural::from(14u32)
    ///         .div_round(&Natural::from(4u32), RoundingMode::Nearest).to_string(),
    ///     "4"
    /// );
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
    /// # Panics
    /// Panics if `other` is zero, or if `rm` is `Exact` but `self` is not divisible by `other`.
    ///
    /// # Examples
    /// ```
    /// extern crate malachite_base;
    /// extern crate malachite_nz;
    ///
    /// use malachite_base::num::arithmetic::traits::DivRound;
    /// use malachite_base::rounding_modes::RoundingMode;
    /// use malachite_nz::natural::Natural;
    ///
    /// assert_eq!(
    ///     (&Natural::from(10u32))
    ///         .div_round(Natural::from(4u32), RoundingMode::Down).to_string(),
    ///     "2"
    /// );
    /// assert_eq!(
    ///     (&Natural::trillion())
    ///         .div_round(Natural::from(3u32), RoundingMode::Floor).to_string(),
    ///     "333333333333"
    /// );
    /// assert_eq!(
    ///     (&Natural::from(10u32))
    ///         .div_round(Natural::from(4u32), RoundingMode::Up).to_string(),
    ///     "3"
    /// );
    /// assert_eq!(
    ///     (&Natural::trillion())
    ///         .div_round(Natural::from(3u32), RoundingMode::Ceiling).to_string(),
    ///     "333333333334");
    /// assert_eq!(
    ///     (&Natural::from(10u32))
    ///         .div_round(Natural::from(5u32), RoundingMode::Exact).to_string(),
    ///     "2"
    /// );
    /// assert_eq!(
    ///     (&Natural::from(10u32))
    ///         .div_round(Natural::from(3u32), RoundingMode::Nearest).to_string(),
    ///     "3"
    /// );
    /// assert_eq!(
    ///     (&Natural::from(20u32))
    ///         .div_round(Natural::from(3u32), RoundingMode::Nearest).to_string(),
    ///     "7"
    /// );
    /// assert_eq!(
    ///     (&Natural::from(10u32))
    ///         .div_round(Natural::from(4u32), RoundingMode::Nearest).to_string(),
    ///     "2"
    /// );
    /// assert_eq!(
    ///     (&Natural::from(14u32))
    ///         .div_round(Natural::from(4u32), RoundingMode::Nearest).to_string(),
    ///     "4"
    /// );
    /// ```
    fn div_round(self, other: Natural, rm: RoundingMode) -> Natural {
        if rm == RoundingMode::Floor || rm == RoundingMode::Down {
            self / other
        } else {
            let (q, r) = self.div_mod(&other);
            if r == 0 {
                q
            } else {
                match rm {
                    RoundingMode::Ceiling | RoundingMode::Up => q.add_limb(1),
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
    /// # Panics
    /// Panics if `other` is zero, or if `rm` is `Exact` but `self` is not divisible by `other`.
    ///
    /// # Examples
    /// ```
    /// extern crate malachite_base;
    /// extern crate malachite_nz;
    ///
    /// use malachite_base::num::arithmetic::traits::DivRound;
    /// use malachite_base::rounding_modes::RoundingMode;
    /// use malachite_nz::natural::Natural;
    ///
    /// assert_eq!(
    ///     (&Natural::from(10u32))
    ///         .div_round(&Natural::from(4u32), RoundingMode::Down).to_string(),
    ///     "2"
    /// );
    /// assert_eq!(
    ///     (&Natural::trillion())
    ///         .div_round(&Natural::from(3u32), RoundingMode::Floor).to_string(),
    ///     "333333333333"
    /// );
    /// assert_eq!(
    ///     (&Natural::from(10u32))
    ///         .div_round(&Natural::from(4u32), RoundingMode::Up).to_string(),
    ///     "3"
    /// );
    /// assert_eq!(
    ///     (&Natural::trillion())
    ///         .div_round(&Natural::from(3u32), RoundingMode::Ceiling).to_string(),
    ///     "333333333334");
    /// assert_eq!(
    ///     (&Natural::from(10u32))
    ///         .div_round(&Natural::from(5u32), RoundingMode::Exact).to_string(),
    ///     "2"
    /// );
    /// assert_eq!(
    ///     (&Natural::from(10u32))
    ///         .div_round(&Natural::from(3u32), RoundingMode::Nearest).to_string(),
    ///     "3"
    /// );
    /// assert_eq!(
    ///     (&Natural::from(20u32))
    ///         .div_round(&Natural::from(3u32), RoundingMode::Nearest).to_string(),
    ///     "7"
    /// );
    /// assert_eq!(
    ///     (&Natural::from(10u32))
    ///         .div_round(&Natural::from(4u32), RoundingMode::Nearest).to_string(),
    ///     "2"
    /// );
    /// assert_eq!(
    ///     (&Natural::from(14u32))
    ///         .div_round(&Natural::from(4u32), RoundingMode::Nearest).to_string(),
    ///     "4"
    /// );
    /// ```
    fn div_round(self, other: &'b Natural, rm: RoundingMode) -> Natural {
        if rm == RoundingMode::Floor || rm == RoundingMode::Down {
            self / other
        } else {
            let (q, r) = self.div_mod(other);
            if r == 0 {
                q
            } else {
                match rm {
                    RoundingMode::Ceiling | RoundingMode::Up => q.add_limb(1),
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
    /// mode, taking the `Natural` on the right-hand side by value. See the `RoundingMode`
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
    /// use malachite_base::num::arithmetic::traits::DivRoundAssign;
    /// use malachite_base::rounding_modes::RoundingMode;
    /// use malachite_nz::natural::Natural;
    ///
    /// let mut n = Natural::from(10u32);
    /// n.div_round_assign(Natural::from(4u32), RoundingMode::Down);
    /// assert_eq!(n.to_string(), "2");
    ///
    /// let mut n = Natural::trillion();
    /// n.div_round_assign(Natural::from(3u32), RoundingMode::Floor);
    /// assert_eq!(n.to_string(), "333333333333");
    ///
    /// let mut n = Natural::from(10u32);
    /// n.div_round_assign(Natural::from(4u32), RoundingMode::Up);
    /// assert_eq!(n.to_string(), "3");
    ///
    /// let mut n = Natural::trillion();
    /// n.div_round_assign(Natural::from(3u32), RoundingMode::Ceiling);
    /// assert_eq!(n.to_string(), "333333333334");
    ///
    /// let mut n = Natural::from(10u32);
    /// n.div_round_assign(Natural::from(5u32), RoundingMode::Exact);
    /// assert_eq!(n.to_string(), "2");
    ///
    /// let mut n = Natural::from(10u32);
    /// n.div_round_assign(Natural::from(3u32), RoundingMode::Nearest);
    /// assert_eq!(n.to_string(), "3");
    ///
    /// let mut n = Natural::from(20u32);
    /// n.div_round_assign(Natural::from(3u32), RoundingMode::Nearest);
    /// assert_eq!(n.to_string(), "7");
    ///
    /// let mut n = Natural::from(10u32);
    /// n.div_round_assign(Natural::from(4u32), RoundingMode::Nearest);
    /// assert_eq!(n.to_string(), "2");
    ///
    /// let mut n = Natural::from(14u32);
    /// n.div_round_assign(Natural::from(4u32), RoundingMode::Nearest);
    /// assert_eq!(n.to_string(), "4");
    /// ```
    fn div_round_assign(&mut self, other: Natural, rm: RoundingMode) {
        if rm == RoundingMode::Floor || rm == RoundingMode::Down {
            *self /= other;
        } else {
            let r = self.div_assign_mod(&other);
            if r != 0 {
                match rm {
                    RoundingMode::Ceiling | RoundingMode::Up => {
                        *self += Natural::ONE;
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
    /// mode, taking the `Natural` on the right-hand side by reference. See the `RoundingMode`
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
    /// use malachite_base::num::arithmetic::traits::DivRoundAssign;
    /// use malachite_base::rounding_modes::RoundingMode;
    /// use malachite_nz::natural::Natural;
    ///
    /// let mut n = Natural::from(10u32);
    /// n.div_round_assign(&Natural::from(4u32), RoundingMode::Down);
    /// assert_eq!(n.to_string(), "2");
    ///
    /// let mut n = Natural::trillion();
    /// n.div_round_assign(&Natural::from(3u32), RoundingMode::Floor);
    /// assert_eq!(n.to_string(), "333333333333");
    ///
    /// let mut n = Natural::from(10u32);
    /// n.div_round_assign(&Natural::from(4u32), RoundingMode::Up);
    /// assert_eq!(n.to_string(), "3");
    ///
    /// let mut n = Natural::trillion();
    /// n.div_round_assign(&Natural::from(3u32), RoundingMode::Ceiling);
    /// assert_eq!(n.to_string(), "333333333334");
    ///
    /// let mut n = Natural::from(10u32);
    /// n.div_round_assign(&Natural::from(5u32), RoundingMode::Exact);
    /// assert_eq!(n.to_string(), "2");
    ///
    /// let mut n = Natural::from(10u32);
    /// n.div_round_assign(&Natural::from(3u32), RoundingMode::Nearest);
    /// assert_eq!(n.to_string(), "3");
    ///
    /// let mut n = Natural::from(20u32);
    /// n.div_round_assign(&Natural::from(3u32), RoundingMode::Nearest);
    /// assert_eq!(n.to_string(), "7");
    ///
    /// let mut n = Natural::from(10u32);
    /// n.div_round_assign(&Natural::from(4u32), RoundingMode::Nearest);
    /// assert_eq!(n.to_string(), "2");
    ///
    /// let mut n = Natural::from(14u32);
    /// n.div_round_assign(&Natural::from(4u32), RoundingMode::Nearest);
    /// assert_eq!(n.to_string(), "4");
    /// ```
    fn div_round_assign(&mut self, other: &'a Natural, rm: RoundingMode) {
        if rm == RoundingMode::Floor || rm == RoundingMode::Down {
            *self /= other;
        } else {
            let r = self.div_assign_mod(other);
            if r != 0 {
                match rm {
                    RoundingMode::Ceiling | RoundingMode::Up => {
                        *self += Natural::ONE;
                    }
                    RoundingMode::Exact => panic!("Division is not exact"),
                    RoundingMode::Nearest => div_round_assign_nearest(self, r, other),
                    _ => unreachable!(),
                }
            }
        }
    }
}
