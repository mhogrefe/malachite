use integer::Integer;
use malachite_base::misc::Max;
use malachite_base::num::{EqModPowerOfTwo, PrimitiveInteger};
use natural::arithmetic::divisible_by_power_of_two::limbs_divisible_by_power_of_two;
use natural::Natural::{self, Large, Small};
use platform::Limb;

/// Interpreting a slice of `Limb`s as the limbs (in ascending order) of a `Natural`, returns
/// whether the negative of the `Natural` is equivalent to a limb mod two to the power of `pow`;
/// that is, whether the `pow` least-significant bits of the negative of the `Natural` and the limb
/// are equal.
///
/// This function assumes that `limbs` has length at least 2 and the last (most significant) limb is
/// nonzero.
///
/// Time: worst case O(n)
///
/// Additional memory: worst case O(1)
///
/// where n = `limbs.len()`
///
/// # Example
/// ```
/// use malachite_nz::integer::arithmetic::eq_limb_mod_power_of_two::*;
/// use std::u32;
///
/// assert_eq!(limbs_eq_mod_power_of_two_neg_limb(&[1, 1], u32::MAX, 0), true);
/// assert_eq!(limbs_eq_mod_power_of_two_neg_limb(&[1, 1], u32::MAX, 1), true);
/// assert_eq!(limbs_eq_mod_power_of_two_neg_limb(&[1, 1], u32::MAX, 32), true);
/// assert_eq!(limbs_eq_mod_power_of_two_neg_limb(&[1, 1], u32::MAX, 33), true);
/// assert_eq!(limbs_eq_mod_power_of_two_neg_limb(&[1, 2], u32::MAX, 33), false);
/// ```
pub fn limbs_eq_mod_power_of_two_neg_limb(limbs: &[Limb], limb: Limb, pow: u64) -> bool {
    if limb == 0 {
        return limbs_divisible_by_power_of_two(limbs, pow);
    }
    let i = (pow >> Limb::LOG_WIDTH) as usize;
    if i >= limbs.len() {
        false
    } else if i == 0 {
        limbs[0].eq_mod_power_of_two(limb.wrapping_neg(), pow)
    } else {
        limbs[0] == limb.wrapping_neg()
            && limbs[1..i].iter().all(|&x| x == Limb::MAX)
            && limbs[i].eq_mod_power_of_two(Limb::MAX, pow & u64::from(Limb::WIDTH_MASK))
    }
}

impl<'a> EqModPowerOfTwo<Limb> for &'a Integer {
    /// Returns whether this `Integer` is equivalent to a `Limb` mod two to the power of `pow`; that
    /// is, whether the `pow` least-significant twos-complement bits of the `Integer` and the `Limb`
    /// are equal.
    ///
    /// Time: worst case O(n)
    ///
    /// Additional memory: worst case O(1)
    ///
    /// where n = `self.significant_bits()`
    ///
    /// # Examples
    /// ```
    /// extern crate malachite_base;
    /// extern crate malachite_nz;
    ///
    /// use malachite_base::num::{EqModPowerOfTwo, Zero};
    /// use malachite_nz::integer::Integer;
    ///
    /// fn main() {
    ///     assert_eq!((&Integer::ZERO).eq_mod_power_of_two(256u32, 8), true);
    ///     assert_eq!((&Integer::from(-0b1101)).eq_mod_power_of_two(0b1011u32, 3), true);
    ///     assert_eq!((&Integer::from(-0b1101)).eq_mod_power_of_two(0b1011u32, 4), false);
    /// }
    /// ```
    fn eq_mod_power_of_two(self, other: Limb, pow: u64) -> bool {
        if self.sign {
            self.abs.eq_mod_power_of_two(other, pow)
        } else {
            self.abs.eq_mod_power_of_two_neg_limb(other, pow)
        }
    }
}

#[cfg(feature = "64_bit_limbs")]
impl<'a> EqModPowerOfTwo<u32> for &'a Integer {
    #[inline]
    fn eq_mod_power_of_two(self, other: u32, pow: u64) -> bool {
        self.eq_mod_power_of_two(Limb::from(other), pow)
    }
}

impl<'a> EqModPowerOfTwo<&'a Integer> for Limb {
    /// Returns whether this `Limb` is equivalent to a `Integer` mod two to the power of `pow`; that
    /// is, whether the `pow` least-significant twos-complement bits of the `Limb` and the `Integer`
    /// are equal.
    ///
    /// Time: worst case O(n)
    ///
    /// Additional memory: worst case O(1)
    ///
    /// where n = min(`pow`, `self.significant_bits()`)
    ///
    /// # Examples
    /// ```
    /// extern crate malachite_base;
    /// extern crate malachite_nz;
    ///
    /// use malachite_base::num::{EqModPowerOfTwo, Zero};
    /// use malachite_nz::integer::Integer;
    ///
    /// fn main() {
    ///     assert_eq!(256u32.eq_mod_power_of_two(&Integer::ZERO, 8), true);
    ///     assert_eq!(0b10101u32.eq_mod_power_of_two(&Integer::from(-0b11011), 3), true);
    ///     assert_eq!(0b10101u32.eq_mod_power_of_two(&Integer::from(-0b10011), 4), false);
    /// }
    /// ```
    #[inline]
    fn eq_mod_power_of_two(self, other: &'a Integer, pow: u64) -> bool {
        other.eq_mod_power_of_two(self, pow)
    }
}

#[cfg(feature = "64_bit_limbs")]
impl<'a> EqModPowerOfTwo<&'a Integer> for u32 {
    #[inline]
    fn eq_mod_power_of_two(self, other: &'a Integer, pow: u64) -> bool {
        Limb::from(self).eq_mod_power_of_two(other, pow)
    }
}

impl Natural {
    pub(crate) fn eq_mod_power_of_two_neg_limb(&self, other: Limb, pow: u64) -> bool {
        match *self {
            Small(ref small) => {
                pow <= u64::from(Limb::WIDTH)
                    && small.wrapping_neg().eq_mod_power_of_two(other, pow)
            }
            Large(ref limbs) => limbs_eq_mod_power_of_two_neg_limb(limbs, other, pow),
        }
    }
}
