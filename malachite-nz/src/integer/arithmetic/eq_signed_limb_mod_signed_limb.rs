use malachite_base::num::arithmetic::traits::{EqMod, NegMod, UnsignedAbs};

use integer::Integer;
use natural::Natural::{self, Large, Small};
use platform::{Limb, SignedLimb};

impl<'a> EqMod<SignedLimb, SignedLimb> for &'a Integer {
    /// Returns whether this `Integer` is equivalent to a `SignedLimb` mod a `SignedLimb` `modulus`;
    /// that is, whether `self` - other is a multiple of `modulus`. Two numbers are equal to each
    /// other mod 0 iff they are equal.
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
    /// use malachite_base::num::arithmetic::traits::EqMod;
    /// use malachite_nz::integer::Integer;
    /// use std::str::FromStr;
    ///
    /// fn main() {
    ///     assert_eq!(Integer::from(13).eq_mod(21i32, 8i32), true);
    ///     assert_eq!(Integer::from_str("987654321").unwrap().eq_mod(321i32, 1_000i32), true);
    ///     assert_eq!(Integer::from_str("987654321").unwrap().eq_mod(322i32, 1_000i32), false);
    ///     assert_eq!(Integer::from_str("-987654321").unwrap().eq_mod(679i32, 1_000i32), true);
    ///     assert_eq!(Integer::from_str("-987654321").unwrap().eq_mod(680i32, 1_000i32), false);
    ///     assert_eq!(Integer::from_str("987654321").unwrap().eq_mod(321i32, -1_000i32), true);
    ///     assert_eq!(Integer::from_str("987654321").unwrap().eq_mod(322i32, -1_000i32), false);
    ///     assert_eq!(Integer::from_str("-987654321").unwrap().eq_mod(679i32, -1_000i32), true);
    ///     assert_eq!(Integer::from_str("-987654321").unwrap().eq_mod(680i32, -1_000i32), false);
    ///     assert_eq!(Integer::from_str("987654321").unwrap().eq_mod(-679i32, 1_000i32), true);
    ///     assert_eq!(Integer::from_str("987654321").unwrap().eq_mod(-680i32, 1_000i32), false);
    ///     assert_eq!(Integer::from_str("-987654321").unwrap().eq_mod(-321i32, 1_000i32), true);
    ///     assert_eq!(Integer::from_str("-987654321").unwrap().eq_mod(-322i32, 1_000i32), false);
    ///     assert_eq!(Integer::from_str("987654321").unwrap().eq_mod(-679i32, -1_000i32), true);
    ///     assert_eq!(Integer::from_str("987654321").unwrap().eq_mod(-680i32, -1_000i32), false);
    ///     assert_eq!(Integer::from_str("-987654321").unwrap().eq_mod(-321i32, -1_000i32), true);
    ///     assert_eq!(Integer::from_str("-987654321").unwrap().eq_mod(-322i32, -1_000i32), false);
    /// }
    /// ```
    fn eq_mod(self, other: SignedLimb, modulus: SignedLimb) -> bool {
        if self.sign == (other >= 0) {
            self.abs
                .eq_mod(other.unsigned_abs(), modulus.unsigned_abs())
        } else {
            self.abs
                .eq_neg_limb_mod_limb(other.unsigned_abs(), modulus.unsigned_abs())
        }
    }
}

#[cfg(not(feature = "32_bit_limbs"))]
impl<'a> EqMod<i32, i32> for &'a Integer {
    #[inline]
    fn eq_mod(self, other: i32, modulus: i32) -> bool {
        self.eq_mod(SignedLimb::from(other), SignedLimb::from(modulus))
    }
}

impl<'a> EqMod<&'a Integer, SignedLimb> for SignedLimb {
    /// Returns whether this `SignedLimb` is equivalent to an `Integer` mod a `SignedLimb`
    /// `modulus`; that is, whether other - `self` is a multiple of `modulus`. Two numbers are equal
    /// to each other mod 0 iff they are equal.
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
    /// use malachite_base::num::arithmetic::traits::EqMod;
    /// use malachite_nz::integer::Integer;
    /// use std::str::FromStr;
    ///
    /// fn main() {
    ///     assert_eq!(21i32.eq_mod(&Integer::from(13), 8i32), true);
    ///     assert_eq!(321i32.eq_mod(&Integer::from_str("987654321").unwrap(), 1_000i32), true);
    ///     assert_eq!(322i32.eq_mod(&Integer::from_str("987654321").unwrap(), 1_000i32), false);
    ///     assert_eq!(679i32.eq_mod(&Integer::from_str("-987654321").unwrap(), 1_000i32), true);
    ///     assert_eq!(680i32.eq_mod(&Integer::from_str("-987654321").unwrap(), 1_000i32), false);
    ///     assert_eq!(321i32.eq_mod(&Integer::from_str("987654321").unwrap(), -1_000i32), true);
    ///     assert_eq!(322i32.eq_mod(&Integer::from_str("987654321").unwrap(), -1_000i32), false);
    ///     assert_eq!(679i32.eq_mod(&Integer::from_str("-987654321").unwrap(), -1_000i32), true);
    ///     assert_eq!(680i32.eq_mod(&Integer::from_str("-987654321").unwrap(), -1_000i32), false);
    ///     assert_eq!((-679).eq_mod(&Integer::from_str("987654321").unwrap(), 1_000i32), true);
    ///     assert_eq!((-680).eq_mod(&Integer::from_str("987654321").unwrap(), 1_000i32), false);
    ///     assert_eq!((-321).eq_mod(&Integer::from_str("-987654321").unwrap(), 1_000i32), true);
    ///     assert_eq!((-322).eq_mod(&Integer::from_str("-987654321").unwrap(), 1_000i32), false);
    ///     assert_eq!((-679).eq_mod(&Integer::from_str("987654321").unwrap(), -1_000i32), true);
    ///     assert_eq!((-680).eq_mod(&Integer::from_str("987654321").unwrap(), -1_000i32), false);
    ///     assert_eq!((-321).eq_mod(&Integer::from_str("-987654321").unwrap(), -1_000i32), true);
    ///     assert_eq!((-322).eq_mod(&Integer::from_str("-987654321").unwrap(), -1_000i32), false);
    /// }
    /// ```
    #[inline]
    fn eq_mod(self, other: &'a Integer, modulus: SignedLimb) -> bool {
        other.eq_mod(self, modulus)
    }
}

#[cfg(not(feature = "32_bit_limbs"))]
impl<'a> EqMod<&'a Integer, i32> for i32 {
    #[inline]
    fn eq_mod(self, other: &'a Integer, modulus: i32) -> bool {
        SignedLimb::from(self).eq_mod(other, SignedLimb::from(modulus))
    }
}

impl<'a> EqMod<SignedLimb, &'a Integer> for SignedLimb {
    /// Returns whether this `SignedLimb` is equivalent to a `SignedLimb` mod an `Integer`
    /// `modulus`; that is, whether other - `self` is a multiple of `modulus`. Two numbers are equal
    /// to each other mod 0 iff they are equal.
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
    /// use malachite_base::num::arithmetic::traits::EqMod;
    /// use malachite_nz::integer::Integer;
    /// use std::str::FromStr;
    ///
    /// fn main() {
    ///     assert_eq!(21i32.eq_mod(13i32, &Integer::from(8)), true);
    ///     assert_eq!(321i32.eq_mod(987_654_321i32, &Integer::from(1_000)), true);
    ///     assert_eq!(322i32.eq_mod(987_654_321i32, &Integer::from(1_000)), false);
    ///     assert_eq!(679i32.eq_mod(-987_654_321i32, &Integer::from(1_000)), true);
    ///     assert_eq!(680i32.eq_mod(-987_654_321i32, &Integer::from(1_000)), false);
    ///     assert_eq!(321i32.eq_mod(987_654_321i32, &Integer::from(-1_000)), true);
    ///     assert_eq!(322i32.eq_mod(987_654_321i32, &Integer::from(-1_000)), false);
    ///     assert_eq!(679i32.eq_mod(-987_654_321i32, &Integer::from(-1_000)), true);
    ///     assert_eq!(680i32.eq_mod(-987_654_321i32, &Integer::from(-1_000)), false);
    ///     assert_eq!((-679).eq_mod(987_654_321i32, &Integer::from(1_000)), true);
    ///     assert_eq!((-680).eq_mod(987_654_321i32, &Integer::from(1_000)), false);
    ///     assert_eq!((-321).eq_mod(-987_654_321i32, &Integer::from(1_000)), true);
    ///     assert_eq!((-322).eq_mod(-987_654_321i32, &Integer::from(1_000)), false);
    ///     assert_eq!((-679).eq_mod(987_654_321i32, &Integer::from(-1_000)), true);
    ///     assert_eq!((-680).eq_mod(987_654_321i32, &Integer::from(-1_000)), false);
    ///     assert_eq!((-321).eq_mod(-987_654_321i32, &Integer::from(-1_000)), true);
    ///     assert_eq!((-322).eq_mod(-987_654_321i32, &Integer::from(-1_000)), false);
    /// }
    /// ```
    fn eq_mod(self, other: SignedLimb, modulus: &'a Integer) -> bool {
        if (self >= 0) == (other >= 0) {
            self.unsigned_abs()
                .eq_mod(other.unsigned_abs(), &modulus.abs)
        } else {
            modulus
                .abs
                .limb_eq_neg_limb_mod_natural(self.unsigned_abs(), other.unsigned_abs())
        }
    }
}

#[cfg(not(feature = "32_bit_limbs"))]
impl<'a> EqMod<i32, &'a Integer> for i32 {
    #[inline]
    fn eq_mod(self, other: i32, modulus: &'a Integer) -> bool {
        SignedLimb::from(self).eq_mod(SignedLimb::from(other), modulus)
    }
}

impl Natural {
    // y cannot be zero.
    fn limb_eq_neg_limb_mod_natural(&self, x: Limb, y: Limb) -> bool {
        *self != 0 as Limb
            && match *self {
                Small(small) => x % small == y.neg_mod(small),
                Large(_) => false,
            }
    }
}
