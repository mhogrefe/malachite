use integer::Integer;
use malachite_base::num::{BitAccess, PrimitiveInteger};
use std::u32;

/// Given the limbs, or base-2<sup>32</sup> digits, of a non-negative `Integer`, in ascending order,
/// checks whether the most significant bit is `false`; if it isn't, appends an extra zero bit. This
/// way the `Integer`'s non-negativity is preserved in its limbs.
///
/// Time: worst case O(1)
///
/// Additional memory: worst case O(1)
///
/// # Examples
/// ```
/// use malachite_nz::integer::conversion::to_twos_complement_limbs::*;
///
/// let mut limbs = vec![1, 2, 3];
/// limbs_to_twos_complement_limbs_non_negative(&mut limbs);
/// assert_eq!(limbs, &[1, 2, 3]);
///
/// let mut limbs = vec![1, 2, 0xffff_ffff];
/// limbs_to_twos_complement_limbs_non_negative(&mut limbs);
/// assert_eq!(limbs, &[1, 2, 0xffff_ffff, 0]);
/// ```
pub fn limbs_to_twos_complement_limbs_non_negative(limbs: &mut Vec<u32>) {
    if !limbs.is_empty() && limbs.last().unwrap().get_bit(u64::from(u32::WIDTH) - 1) {
        // Sign-extend with an extra 0 limb to indicate a positive Integer
        limbs.push(0);
    }
}

/// Given the limbs, or base-2<sup>32</sup> digits, of the absolute value of a negative `Integer`,
/// in ascending order, converts the limbs to two's complement. Returns whether there is a carry
/// left over from the two's complement conversion process.
///
/// Time: worst case O(n)
///
/// Additional memory: worst case O(1)
///
/// where n = `limbs.len()`
///
/// # Examples
/// ```
/// use malachite_nz::integer::conversion::to_twos_complement_limbs::*;
///
/// let mut limbs = &mut [1, 2, 3];
/// assert!(!limbs_slice_to_twos_complement_limbs_negative(limbs));
/// assert_eq!(limbs, &[0xffff_ffff, 0xffff_fffd, 0xffff_fffc]);
///
/// let mut limbs = &mut [0, 0, 0];
/// assert!(limbs_slice_to_twos_complement_limbs_negative(limbs));
/// assert_eq!(limbs, &[0, 0, 0]);
/// ```
pub fn limbs_slice_to_twos_complement_limbs_negative(limbs: &mut [u32]) -> bool {
    let mut carry = true;
    for limb in limbs {
        if carry {
            if let (result, true) = limb.overflowing_neg() {
                *limb = result;
                carry = false;
            }
        } else {
            *limb = !*limb;
        }
    }
    carry
}

/// Given the limbs, or base-2<sup>32</sup> digits, of the absolute value of a negative `Integer`,
/// in ascending order, converts the limbs to two's complement and checks whether the most
/// significant bit is `true`; if it isn't, appends an extra `u32::MAX` bit. This way the
/// `Integer`'s negativity is preserved in its limbs. The limbs cannot be empty or contain only
/// zeros.
///
/// Time: worst case O(n)
///
/// Additional memory: worst case O(1)
///
/// where n = `limbs.len()`
///
/// # Panics
/// Panics if `limbs` contains only zeros.
///
/// # Examples
/// ```
/// use malachite_nz::integer::conversion::to_twos_complement_limbs::*;
///
/// let mut limbs = vec![1, 2, 3];
/// limbs_vec_to_twos_complement_limbs_negative(&mut limbs);
/// assert_eq!(limbs, &[0xffff_ffff, 0xffff_fffd, 0xffff_fffc]);
///
/// let mut limbs = vec![0, 0xffff_ffff];
/// limbs_vec_to_twos_complement_limbs_negative(&mut limbs);
/// assert_eq!(limbs, &[0, 1, 0xffff_ffff]);
/// ```
pub fn limbs_vec_to_twos_complement_limbs_negative(limbs: &mut Vec<u32>) {
    assert!(!limbs_slice_to_twos_complement_limbs_negative(limbs));
    if !limbs.last().unwrap().get_bit(u64::from(u32::WIDTH) - 1) {
        // Sign-extend with an extra !0 limb to indicate a negative Integer
        limbs.push(u32::MAX);
    }
}

impl Integer {
    /// Returns the limbs, or base-2<sup>32</sup> digits, of an `Integer`, in ascending order,
    /// so that less significant limbs have lower indices in the output vector. The limbs are in
    /// two's complement, and the most significant bit of the limbs indicates the sign; if the bit
    /// is zero, the `Integer` is positive, and if the bit is one it is negative. There are no
    /// trailing zero limbs if the `Integer` is positive or trailing !0 limbs if `Integer` is
    /// negative, except as necessary to include the correct sign bit. Zero is a special case: it
    /// contains no limbs.
    ///
    /// This function borrows `self`. If taking ownership of `self` is possible,
    /// `into_twos_complement_limbs_asc` is more efficient.
    ///
    /// This method is more efficient than `to_twos_complement_limbs_desc`.
    ///
    /// Time: worst case O(n)
    ///
    /// Additional memory: worst case O(n)
    ///
    /// where n = `self.significant_bits()`
    ///
    /// # Examples
    /// ```
    /// extern crate malachite_base;
    /// extern crate malachite_nz;
    ///
    /// use malachite_base::num::Zero;
    /// use malachite_nz::integer::Integer;
    ///
    /// fn main() {
    ///     assert!(Integer::ZERO.to_twos_complement_limbs_asc().is_empty());
    ///     assert_eq!(Integer::from(123).to_twos_complement_limbs_asc(), vec![123]);
    ///     assert_eq!(Integer::from(-123).to_twos_complement_limbs_asc(), vec![4294967173]);
    ///     // 10^12 = 232 * 2^32 + 3567587328
    ///     assert_eq!(Integer::trillion().to_twos_complement_limbs_asc(), vec![3567587328, 232]);
    ///     assert_eq!((-Integer::trillion()).to_twos_complement_limbs_asc(),
    ///         vec![727379968, 4294967063]);
    /// }
    /// ```
    pub fn to_twos_complement_limbs_asc(&self) -> Vec<u32> {
        let mut limbs = self.abs.to_limbs_asc();
        if self.sign {
            limbs_to_twos_complement_limbs_non_negative(&mut limbs);
        } else {
            limbs_vec_to_twos_complement_limbs_negative(&mut limbs);
        }
        limbs
    }

    /// Returns the limbs, or base-2<sup>32</sup> digits, of an `Integer`, in descending order, so
    /// that less significant limbs have higher indices in the output vector. The limbs are in two's
    /// complement, and the most significant bit of the limbs indicates the sign; if the bit is
    /// zero, the `Integer` is positive, and if the bit is one it is negative. There are no
    /// leading zero limbs if the `Integer` is non-negative or leading !0 limbs if `Integer` is
    /// negative, except as necessary to include the correct sign bit. Zero is a special case: it
    /// contains no limbs.
    ///
    /// This is similar to how BigIntegers in Java are represented.
    ///
    /// This function borrows `self`. If taking ownership of `self` is possible,
    /// `into_twos_complement_limbs_desc` is more efficient.
    ///
    /// This method is less efficient than `to_twos_complement_limbs_asc`.
    ///
    /// Time: worst case O(n)
    ///
    /// Additional memory: worst case O(n)
    ///
    /// where n = `self.significant_bits()`
    ///
    /// # Examples
    /// ```
    /// extern crate malachite_base;
    /// extern crate malachite_nz;
    ///
    /// use malachite_base::num::Zero;
    /// use malachite_nz::integer::Integer;
    ///
    /// fn main() {
    ///     assert!(Integer::ZERO.to_twos_complement_limbs_desc().is_empty());
    ///     assert_eq!(Integer::from(123).to_twos_complement_limbs_desc(), vec![123]);
    ///     assert_eq!(Integer::from(-123).to_twos_complement_limbs_desc(), vec![4294967173]);
    ///     // 10^12 = 232 * 2^32 + 3567587328
    ///     assert_eq!(Integer::trillion().to_twos_complement_limbs_desc(), vec![232, 3567587328]);
    ///     assert_eq!((-Integer::trillion()).to_twos_complement_limbs_desc(),
    ///         vec![4294967063, 727379968]);
    /// }
    pub fn to_twos_complement_limbs_desc(&self) -> Vec<u32> {
        let mut limbs = self.to_twos_complement_limbs_asc();
        limbs.reverse();
        limbs
    }

    /// Returns the limbs, or base-2<sup>32</sup> digits, of an `Integer`, in ascending order,
    /// so that less significant limbs have lower indices in the output vector. The limbs are in
    /// two's complement, and the most significant bit of the limbs indicates the sign; if the bit
    /// is zero, the `Integer` is positive, and if the bit is one it is negative. There are no
    /// trailing zero limbs if the `Integer` is positive or trailing !0 limbs if `Integer` is
    /// negative, except as necessary to include the correct sign bit. Zero is a special case: it
    /// contains no limbs.
    ///
    /// This function takes ownership of `self`. If it's necessary to borrow `self` instead, use
    /// `to_twos_complement_limbs_asc`.
    ///
    /// This method is more efficient than `into_twos_complement_limbs_desc`.
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
    /// use malachite_base::num::Zero;
    /// use malachite_nz::integer::Integer;
    ///
    /// fn main() {
    ///     assert!(Integer::ZERO.into_twos_complement_limbs_asc().is_empty());
    ///     assert_eq!(Integer::from(123).into_twos_complement_limbs_asc(), vec![123]);
    ///     assert_eq!(Integer::from(-123).into_twos_complement_limbs_asc(), vec![4294967173]);
    ///     // 10^12 = 232 * 2^32 + 3567587328
    ///     assert_eq!(Integer::trillion().into_twos_complement_limbs_asc(), vec![3567587328, 232]);
    ///     assert_eq!((-Integer::trillion()).into_twos_complement_limbs_asc(),
    ///         vec![727379968, 4294967063]);
    /// }
    /// ```
    pub fn into_twos_complement_limbs_asc(self) -> Vec<u32> {
        let mut limbs = self.abs.into_limbs_asc();
        if self.sign {
            limbs_to_twos_complement_limbs_non_negative(&mut limbs);
        } else {
            limbs_vec_to_twos_complement_limbs_negative(&mut limbs);
        }
        limbs
    }

    /// Returns the limbs, or base-2<sup>32</sup> digits, of an `Integer`, in descending order, so
    /// that less significant limbs have higher indices in the output vector. The limbs are in two's
    /// complement, and the most significant bit of the limbs indicates the sign; if the bit is
    /// zero, the `Integer` is positive, and if the bit is one it is negative. There are no
    /// leading zero limbs if the `Integer` is non-negative or leading !0 limbs if `Integer` is
    /// negative, except as necessary to include the correct sign bit. Zero is a special case: it
    /// contains no limbs.
    ///
    /// This is similar to how BigIntegers in Java are represented.
    ///
    /// This function takes ownership of `self`. If it's necessary to borrow `self` instead, use
    /// `to_twos_complement_limbs_desc`.
    ///
    /// This method is less efficient than `into_twos_complement_limbs_asc`.
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
    /// use malachite_base::num::Zero;
    /// use malachite_nz::integer::Integer;
    ///
    /// fn main() {
    ///     assert!(Integer::ZERO.into_twos_complement_limbs_desc().is_empty());
    ///     assert_eq!(Integer::from(123).into_twos_complement_limbs_desc(), vec![123]);
    ///     assert_eq!(Integer::from(-123).into_twos_complement_limbs_desc(), vec![4294967173]);
    ///     // 10^12 = 232 * 2^32 + 3567587328
    ///     assert_eq!(Integer::trillion().into_twos_complement_limbs_desc(),
    ///         vec![232, 3567587328]);
    ///     assert_eq!((-Integer::trillion()).into_twos_complement_limbs_desc(),
    ///         vec![4294967063, 727379968]);
    /// }
    pub fn into_twos_complement_limbs_desc(self) -> Vec<u32> {
        let mut limbs = self.into_twos_complement_limbs_asc();
        limbs.reverse();
        limbs
    }
}
