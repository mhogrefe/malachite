use malachite_base::num::arithmetic::traits::{CeilingLogBase2, CheckedLogBase2, FloorLogBase2};
use malachite_base::num::basic::integers::PrimitiveInt;
use malachite_base::num::conversion::traits::ExactFrom;
use malachite_base::slices::slice_test_zero;
use natural::arithmetic::is_power_of_2::limbs_is_power_of_2;
use natural::logic::significant_bits::limbs_significant_bits;
use natural::InnerNatural::{Large, Small};
use natural::Natural;
use platform::Limb;

/// Given the limbs of a `Natural`, returns the floor of its base-2 logarithm.
///
/// This function assumes that `xs` is nonempty and the last (most significant) limb is nonzero.
///
/// $f((d_i)_ {i=0}^k) = \lfloor\log_2 x\rfloor$, where $x = \sum_{i=0}^kB^id_i$ and $B$ is one
/// more than `Limb::MAX`.
///
/// # Worst-case complexity
/// Constant time and additional memory.
///
/// # Panics
/// Panics if `xs` is empty.
///
/// # Examples
/// ```
/// use malachite_nz::natural::arithmetic::log_base_2::limbs_floor_log_base_2;
///
/// assert_eq!(limbs_floor_log_base_2(&[0b11]), 1);
/// assert_eq!(limbs_floor_log_base_2(&[0, 0b1101]), 35);
/// ```
#[inline]
pub fn limbs_floor_log_base_2(xs: &[Limb]) -> u64 {
    limbs_significant_bits(xs) - 1
}

/// Given the limbs of a `Natural`, returns the ceiling of its base-2 logarithm.
///
/// This function assumes that `xs` is nonempty and the last (most significant) limb is nonzero.
///
/// $f((d_i)_ {i=0}^k) = \lceil\log_2 x\rceil$, where $x = \sum_{i=0}^kB^id_i$ and $B$ is one more
/// than `Limb::MAX`.
///
/// # Worst-case complexity
/// $T(n) = O(n)$
///
/// $M(n) = O(1)$
///
/// where $T$ is time, $M$ is additional memory, and $n$ is `xs.len()`.
///
/// # Panics
/// Panics if `xs` is empty.
///
/// # Examples
/// ```
/// use malachite_nz::natural::arithmetic::log_base_2::limbs_ceiling_log_base_2;
///
/// assert_eq!(limbs_ceiling_log_base_2(&[0b11]), 2);
/// assert_eq!(limbs_ceiling_log_base_2(&[0, 0b1101]), 36);
/// ```
pub fn limbs_ceiling_log_base_2(xs: &[Limb]) -> u64 {
    let floor_log_base_2 = limbs_floor_log_base_2(xs);
    if limbs_is_power_of_2(xs) {
        floor_log_base_2
    } else {
        floor_log_base_2 + 1
    }
}

/// Given the limbs of a `Natural`, returns the its base-2 logarithm. If the `Natural` is not a
/// power of 2, returns `None`.
///
/// This function assumes that `xs` is nonempty and the last (most significant) limb is nonzero.
///
/// $$
/// f((d_i)_ {i=0}^k) = \\begin{cases}
///     \operatorname{Some}(\log_2 x) & \log_2 x \in \Z \\\\
///     \operatorname{None} & \textrm{otherwise},
/// \\end{cases}
/// $$
/// where $x = \sum_{i=0}^kB^id_i$ and $B$ is one more than `Limb::MAX`.
///
/// # Worst-case complexity
/// $T(n) = O(n)$
///
/// $M(n) = O(1)$
///
/// where $T$ is time, $M$ is additional memory, and $n$ is `xs.len()`.
///
/// # Panics
/// Panics if `xs` is empty.
///
/// # Examples
/// ```
/// use malachite_nz::natural::arithmetic::log_base_2::limbs_checked_log_base_2;
///
/// assert_eq!(limbs_checked_log_base_2(&[0b11]), None);
/// assert_eq!(limbs_checked_log_base_2(&[0b10]), Some(1));
/// assert_eq!(limbs_checked_log_base_2(&[0, 0b1101]), None);
/// assert_eq!(limbs_checked_log_base_2(&[0, 0b1000]), Some(35));
/// ```
pub fn limbs_checked_log_base_2(xs: &[Limb]) -> Option<u64> {
    let (xs_last, xs_init) = xs.split_last().unwrap();
    if slice_test_zero(xs_init) {
        xs_last
            .checked_log_base_2()
            .map(|log| log + (u64::exact_from(xs_init.len()) << Limb::LOG_WIDTH))
    } else {
        None
    }
}

impl<'a> FloorLogBase2 for &'a Natural {
    /// Returns the floor of the base-2 logarithm of a positive `Natural`.
    ///
    /// $f(x) = \lfloor\log_2 x\rfloor$.
    ///
    /// # Worst-case complexity
    /// Constant time and additional memory.
    ///
    /// # Panics
    /// Panics if `self` is 0.
    ///
    /// # Examples
    /// ```
    /// extern crate malachite_base;
    /// extern crate malachite_nz;
    ///
    /// use malachite_base::num::arithmetic::traits::FloorLogBase2;
    /// use malachite_nz::natural::Natural;
    ///
    /// assert_eq!(Natural::from(3u32).floor_log_base_2(), 1);
    /// assert_eq!(Natural::from(100u32).floor_log_base_2(), 6);
    /// ```
    fn floor_log_base_2(self) -> u64 {
        match *self {
            Natural(Small(small)) => small.floor_log_base_2(),
            Natural(Large(ref limbs)) => limbs_floor_log_base_2(limbs),
        }
    }
}

impl<'a> CeilingLogBase2 for &'a Natural {
    /// Returns the ceiling of the base-2 logarithm of a positive `Natural`.
    ///
    /// $f(x) = \lceil\log_2 x\rceil$.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(1)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `self.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `self` is 0.
    ///
    /// # Examples
    /// ```
    /// extern crate malachite_base;
    /// extern crate malachite_nz;
    ///
    /// use malachite_base::num::arithmetic::traits::CeilingLogBase2;
    /// use malachite_nz::natural::Natural;
    ///
    /// assert_eq!(Natural::from(3u32).ceiling_log_base_2(), 2);
    /// assert_eq!(Natural::from(100u32).ceiling_log_base_2(), 7);
    /// ```
    fn ceiling_log_base_2(self) -> u64 {
        match *self {
            Natural(Small(small)) => small.ceiling_log_base_2(),
            Natural(Large(ref limbs)) => limbs_ceiling_log_base_2(limbs),
        }
    }
}

impl<'a> CheckedLogBase2 for &'a Natural {
    /// Returns the base-2 logarithm of a positive `Natural`. If the integer is not a power of 2,
    /// `None` is returned.
    ///
    /// $$
    /// f(x) = \\begin{cases}
    ///     \operatorname{Some}(\log_2 x) & \log_2 x \in \Z \\\\
    ///     \operatorname{None} & \textrm{otherwise},
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
    /// Panics if `self` is 0.
    ///
    /// # Examples
    /// ```
    /// extern crate malachite_base;
    /// extern crate malachite_nz;
    ///
    /// use malachite_base::num::arithmetic::traits::CheckedLogBase2;
    /// use malachite_nz::natural::Natural;
    /// use std::str::FromStr;
    ///
    /// assert_eq!(Natural::from(3u32).checked_log_base_2(), None);
    /// assert_eq!(Natural::from(4u32).checked_log_base_2(), Some(2));
    /// assert_eq!(
    ///     Natural::from_str("1267650600228229401496703205376").unwrap().checked_log_base_2(),
    ///     Some(100)
    /// );
    /// ```
    fn checked_log_base_2(self) -> Option<u64> {
        match *self {
            Natural(Small(small)) => small.checked_log_base_2(),
            Natural(Large(ref limbs)) => limbs_checked_log_base_2(limbs),
        }
    }
}
