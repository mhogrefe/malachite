use integer::Integer;

impl Integer {
    /// Returns the limbs, or base-2^(32) digits, of an `Integer`, in little-endian order, so that
    /// less significant limbs have lower indices in the output vector. The limbs are in two's
    /// complement, and the most significant bit of the limbs indicates the sign; if the bit is
    /// zero, the `Integer` is positive, and if the bit is one it is negative. There are no
    /// trailing zero limbs if the `Integer` is positive or trailing !0 limbs if `Integer` is
    /// negative, except as necessary to include the correct sign bit. Zero is a special case: it
    /// contains no limbs. Although GMP may use 32- or 64-bit limbs internally, this method always
    /// returns 32-bit limbs.
    ///
    /// This method is more efficient than `twos_complement_limbs_be`.
    ///
    /// Time: worst case O(n)
    ///
    /// Additional memory: worst case O(n)
    ///
    /// where n = `self.significant_bits()`
    ///
    /// # Examples
    /// ```
    /// use malachite_native::integer::Integer;
    /// use std::str::FromStr;
    ///
    /// assert!(Integer::from(0).twos_complement_limbs_le().is_empty());
    /// assert_eq!(Integer::from(123).twos_complement_limbs_le(), vec![123]);
    /// assert_eq!(Integer::from(-123).twos_complement_limbs_le(), vec![4294967173]);
    /// // 10^12 = 232 * 2^32 + 3567587328
    /// assert_eq!(Integer::from_str("1000000000000").unwrap().twos_complement_limbs_le(),
    ///     vec![3567587328, 232]);
    /// assert_eq!(Integer::from_str("-1000000000000").unwrap().twos_complement_limbs_le(),
    ///     vec![727379968, 4294967063]);
    /// ```
    pub fn twos_complement_limbs_le(&self) -> Vec<u32> {
        let mut limbs = self.natural_abs_ref().limbs_le();
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

    /// Returns the limbs, or base-2^(32) digits, of an `Integer`, in big-endian order, so that less
    /// significant limbs have higher indices in the output vector. The limbs are in two's
    /// complement, and the most significant bit of the limbs indicates the sign; if the bit is
    /// zero, the `Integer` is positive, and if the bit is one it is negative. There are no
    /// leading zero limbs if the `Integer` is non-negative or leading !0 limbs if `Integer` is
    /// negative, except as necessary to include the correct sign bit. Zero is a special case: it
    /// contains no limbs. Although GMP may use 32- or 64-bit limbs internally, this method always
    /// returns 32-bit limbs.
    ///
    /// This is similar to how BigIntegers in Java are represented.
    ///
    /// This method is less efficient than `twos_complement_limbs_le`.
    ///
    /// Time: worst case O(n)
    ///
    /// Additional memory: worst case O(n)
    ///
    /// where n = `self.significant_bits()`
    ///
    /// # Examples
    /// ```
    /// use malachite_native::integer::Integer;
    /// use std::str::FromStr;
    ///
    /// assert!(Integer::from(0).twos_complement_limbs_be().is_empty());
    /// assert_eq!(Integer::from(123).twos_complement_limbs_be(), vec![123]);
    /// assert_eq!(Integer::from(-123).twos_complement_limbs_be(), vec![4294967173]);
    /// // 10^12 = 232 * 2^32 + 3567587328
    /// assert_eq!(Integer::from_str("1000000000000").unwrap().twos_complement_limbs_be(),
    ///     vec![232, 3567587328]);
    /// assert_eq!(Integer::from_str("-1000000000000").unwrap().twos_complement_limbs_be(),
    ///     vec![4294967063, 727379968]);
    pub fn twos_complement_limbs_be(&self) -> Vec<u32> {
        self.twos_complement_limbs_le().into_iter().rev().collect()
    }
}
