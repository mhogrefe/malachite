use malachite_base::num::arithmetic::traits::{
    CeilingLogBasePowerOf2, CheckedLogBasePowerOf2, DivMod, FloorLogBasePowerOf2,
};
use malachite_base::num::basic::traits::{One, Zero};
use natural::arithmetic::is_power_of_2::limbs_is_power_of_2;
use natural::logic::significant_bits::limbs_significant_bits;
use natural::InnerNatural::{Large, Small};
use natural::Natural;
use platform::Limb;

/// Given the limbs of a `Natural`, returns the floor of its base-$2^p$ logarithm.
///
/// This function assumes that `xs` is nonempty and the last (most significant) limb is nonzero.
///
/// $f((d_i)_ {i=0}^k, p) = \lfloor\log_{2^p} x\rfloor$, where $x = \sum_{i=0}^kB^id_i$ and $B$ is
/// one more than `Limb::MAX`.
///
/// # Worst-case complexity
/// Constant time and additional memory.
///
/// # Panics
/// Panics if `xs` is empty or `pow` is 0.
///
/// # Examples
/// ```
/// use malachite_nz::natural::arithmetic::log_base_power_of_2::limbs_floor_log_base_power_of_2;
///
/// assert_eq!(limbs_floor_log_base_power_of_2(&[100], 2), 3);
/// assert_eq!(limbs_floor_log_base_power_of_2(&[0, 1], 8), 4);
/// ```
pub fn limbs_floor_log_base_power_of_2(xs: &[Limb], pow: u64) -> u64 {
    assert_ne!(pow, 0);
    (limbs_significant_bits(xs) - 1) / pow
}

/// Given the limbs of a `Natural`, returns the ceiling of its base-$2^p$ logarithm.
///
/// This function assumes that `xs` is nonempty and the last (most significant) limb is nonzero.
///
/// $f((d_i)_ {i=0}^k, p) = \lceil\log_{2^p} x\rceil$, where $x = \sum_{i=0}^kB^id_i$ and $B$ is
/// one more than `Limb::MAX`.
///
/// # Worst-case complexity
/// $T(n) = O(n)$
///
/// $M(n) = O(1)$
///
/// where $T$ is time, $M$ is additional memory, and $n$ is `xs.len()`.
///
/// # Panics
/// Panics if `xs` is empty or `pow` is 0.
///
/// # Examples
/// ```
/// use malachite_nz::natural::arithmetic::log_base_power_of_2::limbs_ceiling_log_base_power_of_2;
///
/// assert_eq!(limbs_ceiling_log_base_power_of_2(&[100], 2), 4);
/// assert_eq!(limbs_ceiling_log_base_power_of_2(&[0, 1], 8), 4);
/// ```
pub fn limbs_ceiling_log_base_power_of_2(xs: &[Limb], pow: u64) -> u64 {
    assert_ne!(pow, 0);
    let significant_bits_m_1 = limbs_significant_bits(xs) - 1;
    let (floor_log, rem) = significant_bits_m_1.div_mod(pow);
    if limbs_is_power_of_2(xs) && rem == 0 {
        floor_log
    } else {
        floor_log + 1
    }
}

/// Given the limbs of a `Natural`, returns the its base-$2^p$ logarithm. If the `Natural` is not a
/// power of $2^p$, returns `None`.
///
/// This function assumes that `xs` is nonempty and the last (most significant) limb is nonzero.
///
/// $$
/// f((d_i)_ {i=0}^k, p) = \\begin{cases}
///     \operatorname{Some}(\log_{2^p} x) & \log_{2^p} x \in \Z \\\\
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
/// Panics if `xs` is empty or `pow` is 0.
///
/// # Examples
/// ```
/// use malachite_nz::natural::arithmetic::log_base_power_of_2::limbs_checked_log_base_power_of_2;
///
/// assert_eq!(limbs_checked_log_base_power_of_2(&[100], 2), None);
/// assert_eq!(limbs_checked_log_base_power_of_2(&[0, 1], 8), Some(4));
/// ```
pub fn limbs_checked_log_base_power_of_2(xs: &[Limb], pow: u64) -> Option<u64> {
    assert_ne!(pow, 0);
    let significant_bits_m_1 = limbs_significant_bits(xs) - 1;
    let (floor_log, rem) = significant_bits_m_1.div_mod(pow);
    if limbs_is_power_of_2(xs) && rem == 0 {
        Some(floor_log)
    } else {
        None
    }
}

pub fn _ceiling_log_base_power_of_2_naive(x: &Natural, pow: u64) -> u64 {
    assert_ne!(*x, Natural::ZERO);
    assert_ne!(pow, 0);
    let mut result = 0;
    let mut p = Natural::ONE;
    while p < *x {
        result += 1;
        p <<= pow;
    }
    result
}

impl<'a> FloorLogBasePowerOf2 for &'a Natural {
    /// Returns the floor of the base-$2^p$ logarithm of a positive `Natural`.
    ///
    /// $f(x, p) = \lfloor\log_{2^p} x\rfloor$.
    ///
    /// # Worst-case complexity
    /// Constant time and additional memory.
    ///
    /// # Panics
    /// Panics if `self` is 0 or `pow` is 0.
    ///
    /// # Examples
    /// ```
    /// extern crate malachite_base;
    /// extern crate malachite_nz;
    ///
    /// use malachite_base::num::arithmetic::traits::FloorLogBasePowerOf2;
    /// use malachite_nz::natural::Natural;
    ///
    /// assert_eq!(Natural::from(100u32).floor_log_base_power_of_2(2), 3);
    /// assert_eq!(Natural::from(4294967296u64).floor_log_base_power_of_2(8), 4);
    /// ```
    fn floor_log_base_power_of_2(self, pow: u64) -> u64 {
        match *self {
            Natural(Small(small)) => small.floor_log_base_power_of_2(pow),
            Natural(Large(ref limbs)) => limbs_floor_log_base_power_of_2(limbs, pow),
        }
    }
}

impl<'a> CeilingLogBasePowerOf2 for &'a Natural {
    /// Returns the ceiling of the base-$2^p$ logarithm of a positive `Natural`.
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
    /// Panics if `self` is 0 or `pow` is 0.
    ///
    /// # Examples
    /// ```
    /// extern crate malachite_base;
    /// extern crate malachite_nz;
    ///
    /// use malachite_base::num::arithmetic::traits::CeilingLogBasePowerOf2;
    /// use malachite_nz::natural::Natural;
    ///
    /// assert_eq!(Natural::from(100u32).ceiling_log_base_power_of_2(2), 4);
    /// assert_eq!(Natural::from(4294967296u64).ceiling_log_base_power_of_2(8), 4);
    /// ```
    fn ceiling_log_base_power_of_2(self, pow: u64) -> u64 {
        match *self {
            Natural(Small(small)) => small.ceiling_log_base_power_of_2(pow),
            Natural(Large(ref limbs)) => limbs_ceiling_log_base_power_of_2(limbs, pow),
        }
    }
}

impl<'a> CheckedLogBasePowerOf2 for &'a Natural {
    /// Returns the base-$2^p$ logarithm of a positive `Natural`. If the integer is not a power of
    /// $2^p$, `None` is returned.
    ///
    /// $$
    /// f(x, p) = \\begin{cases}
    ///     \operatorname{Some}(\log_{2^p} x) & \log_{2^p} x \in \Z \\\\
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
    /// Panics if `self` is 0 or `pow` is 0.
    ///
    /// # Examples
    /// ```
    /// extern crate malachite_base;
    /// extern crate malachite_nz;
    ///
    /// use malachite_base::num::arithmetic::traits::CheckedLogBasePowerOf2;
    /// use malachite_nz::natural::Natural;
    /// use std::str::FromStr;
    ///
    /// assert_eq!(Natural::from(100u32).checked_log_base_power_of_2(2), None);
    /// assert_eq!(Natural::from(4294967296u64).checked_log_base_power_of_2(8), Some(4));
    /// ```
    fn checked_log_base_power_of_2(self, pow: u64) -> Option<u64> {
        match *self {
            Natural(Small(small)) => small.checked_log_base_power_of_2(pow),
            Natural(Large(ref limbs)) => limbs_checked_log_base_power_of_2(limbs, pow),
        }
    }
}
