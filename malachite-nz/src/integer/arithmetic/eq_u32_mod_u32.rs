use integer::Integer;
use malachite_base::num::{EqMod, NegMod};
use natural::arithmetic::eq_u32_mod_u32::limbs_eq_limb_mod_limb;
use natural::Natural::{self, Large, Small};

/// Interpreting a slice of `u32`s as the limbs of a `Integer` in ascending order, determines
/// whether that `Integer` is equal to the negative of a limb mod a given `u32` modulus.
///
/// This function assumes that `modulus` is nonzero, `limbs` has at least two elements, and the last
/// element of `limbs` is nonzero.
///
/// Time: worst case O(n)
///
/// Additional memory: worst case O(1)
///
/// where n = `limbs.len()`
///
/// # Panics
/// Panics if the length of `limbs` is less than 2.
///
/// # Example
/// ```
/// use malachite_nz::integer::arithmetic::eq_u32_mod_u32::limbs_eq_limb_mod_neg_limb;
///
/// assert_eq!(limbs_eq_limb_mod_neg_limb(&[6, 7], 3, 2), false);
/// assert_eq!(limbs_eq_limb_mod_neg_limb(&[100, 101, 102], 1_232, 10), true);
/// ```
pub fn limbs_eq_limb_mod_neg_limb(limbs: &[u32], limb: u32, modulus: u32) -> bool {
    limbs_eq_limb_mod_limb(limbs, limb.neg_mod(modulus), modulus)
}

impl<'a> EqMod<u32, u32> for &'a Integer {
    /// Returns whether this `Integer` is equivalent to a `u32` mod a `u32` `modulus`; that is,
    /// whether `self` - other is a multiple of `modulus`. Two numbers are equal to each other mod 0
    /// iff they are equal.
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
    /// use malachite_base::num::EqMod;
    /// use malachite_nz::integer::Integer;
    /// use std::str::FromStr;
    ///
    /// fn main() {
    ///     assert_eq!(Integer::from(13u32).eq_mod(21, 8), true);
    ///     assert_eq!(Integer::from_str("987654321").unwrap().eq_mod(321u32, 1_000u32), true);
    ///     assert_eq!(Integer::from_str("987654321").unwrap().eq_mod(322u32, 1_000u32), false);
    ///     assert_eq!(Integer::from_str("-987654321").unwrap().eq_mod(679u32, 1_000u32), true);
    ///     assert_eq!(Integer::from_str("-987654321").unwrap().eq_mod(680u32, 1_000u32), false);
    /// }
    /// ```
    fn eq_mod(self, other: u32, modulus: u32) -> bool {
        if self.sign {
            self.abs.eq_mod(other, modulus)
        } else {
            self.abs.eq_mod_neg_u32(other, modulus)
        }
    }
}

impl<'a> EqMod<&'a Integer, u32> for u32 {
    /// Returns whether this `u32` is equivalent to an `Integer` mod a `u32` `modulus`; that is,
    /// whether other - `self` is a multiple of `modulus`. Two numbers are equal to each other mod 0
    /// iff they are equal.
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
    /// use malachite_base::num::EqMod;
    /// use malachite_nz::integer::Integer;
    /// use std::str::FromStr;
    ///
    /// fn main() {
    ///     assert_eq!(21u32.eq_mod(&Integer::from(13), 8u32), true);
    ///     assert_eq!(321u32.eq_mod(&Integer::from_str("987654321").unwrap(), 1_000u32), true);
    ///     assert_eq!(322u32.eq_mod(&Integer::from_str("987654321").unwrap(), 1_000u32), false);
    ///     assert_eq!(679u32.eq_mod(&Integer::from_str("-987654321").unwrap(), 1_000u32), true);
    ///     assert_eq!(680u32.eq_mod(&Integer::from_str("-987654321").unwrap(), 1_000u32), false);
    /// }
    /// ```
    fn eq_mod(self, other: &'a Integer, modulus: u32) -> bool {
        other.eq_mod(self, modulus)
    }
}

impl Natural {
    // other cannot be zero.
    pub(crate) fn eq_mod_neg_u32(&self, other: u32, modulus: u32) -> bool {
        modulus != 0
            && match *self {
                Small(small) => small % modulus == other.neg_mod(modulus),
                Large(ref limbs) => limbs_eq_limb_mod_neg_limb(limbs, other, modulus),
            }
    }
}
