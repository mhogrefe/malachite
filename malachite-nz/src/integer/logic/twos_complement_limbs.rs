use integer::Integer;
use malachite_base::num::UnsignedAbs;

//TODO clean
impl Integer {
    /// Returns the limbs, or base-2<sup>32</sup> digits, of an `Integer`, in ascending order,
    /// so that less significant limbs have lower indices in the output vector. The limbs are in
    /// two's complement, and the most significant bit of the limbs indicates the sign; if the bit
    /// is zero, the `Integer` is positive, and if the bit is one it is negative. There are no
    /// trailing zero limbs if the `Integer` is positive or trailing !0 limbs if `Integer` is
    /// negative, except as necessary to include the correct sign bit. Zero is a special case: it
    /// contains no limbs.
    ///
    /// This method is more efficient than `twos_complement_limbs_desc`.
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
    ///     assert!(Integer::ZERO.twos_complement_limbs_asc().is_empty());
    ///     assert_eq!(Integer::from(123).twos_complement_limbs_asc(), vec![123]);
    ///     assert_eq!(Integer::from(-123).twos_complement_limbs_asc(), vec![4294967173]);
    ///     // 10^12 = 232 * 2^32 + 3567587328
    ///     assert_eq!(Integer::trillion().twos_complement_limbs_asc(), vec![3567587328, 232]);
    ///     assert_eq!((-Integer::trillion()).twos_complement_limbs_asc(),
    ///         vec![727379968, 4294967063]);
    /// }
    /// ```
    pub fn twos_complement_limbs_asc(&self) -> Vec<u32> {
        let mut limbs = self.unsigned_abs().into_limbs_asc();
        if *self >= 0 {
            if !limbs.is_empty() && limbs.last().unwrap() & 0x8000_0000 != 0 {
                // Sign-extend with an extra 0 limb to indicate a positive Integer
                limbs.push(0);
            }
            limbs
        } else {
            let mut carry = true;
            for limb in &mut limbs {
                if carry {
                    let (sum, overflow) = (!*limb).overflowing_add(1);
                    *limb = sum;
                    if !overflow {
                        carry = false;
                    }
                } else {
                    *limb = !*limb;
                }
            }
            // At this point carry must be false, because self is nonzero in this branch
            if limbs.last().unwrap() & 0x8000_0000 == 0 {
                // Sign-extend with an extra !0 limb to indicate a negative Integer
                limbs.push(!0);
            }
            limbs
        }
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
    /// This method is less efficient than `twos_complement_limbs_asc`.
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
    ///     assert!(Integer::ZERO.twos_complement_limbs_desc().is_empty());
    ///     assert_eq!(Integer::from(123).twos_complement_limbs_desc(), vec![123]);
    ///     assert_eq!(Integer::from(-123).twos_complement_limbs_desc(), vec![4294967173]);
    ///     // 10^12 = 232 * 2^32 + 3567587328
    ///     assert_eq!(Integer::trillion().twos_complement_limbs_desc(), vec![232, 3567587328]);
    ///     assert_eq!((-Integer::trillion()).twos_complement_limbs_desc(),
    ///         vec![4294967063, 727379968]);
    /// }
    pub fn twos_complement_limbs_desc(&self) -> Vec<u32> {
        self.twos_complement_limbs_asc().into_iter().rev().collect()
    }
}
