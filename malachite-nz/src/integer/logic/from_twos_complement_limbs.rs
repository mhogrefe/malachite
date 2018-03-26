use integer::Integer;
use malachite_base::num::{BitAccess, PrimitiveInteger, Zero};
use natural::Natural;

impl Integer {
    /// Converts a slice of limbs, or base-2<sup>32</sup> digits, to an `Integer`, in ascending
    /// order, so that less significant limbs have lower indices in the input slice. The limbs are
    /// in two's complement, and the most significant bit of the limbs indicates the sign; if the
    /// bit is zero, the `Integer` is non-negative, and if the bit is one it is negative. If `limbs`
    /// is empty, zero is returned.
    ///
    /// This method is more efficient than `from_twos_complement_limbs_desc`.
    ///
    /// Time: worst case O(n)
    ///
    /// Additional memory: worst case O(n)
    ///
    /// where n = `limbs.len()`
    ///
    /// # Examples
    /// ```
    /// use malachite_nz::integer::Integer;
    ///
    /// assert_eq!(Integer::from_twos_complement_limbs_asc(&[]).to_string(), "0");
    /// assert_eq!(Integer::from_twos_complement_limbs_asc(&[123]).to_string(), "123");
    /// assert_eq!(Integer::from_twos_complement_limbs_asc(&[4294967173]).to_string(), "-123");
    /// // 10^12 = 232 * 2^32 + 3567587328
    /// assert_eq!(Integer::from_twos_complement_limbs_asc(&[3567587328, 232]).to_string(),
    ///     "1000000000000");
    /// assert_eq!(Integer::from_twos_complement_limbs_asc(&[727379968, 4294967063]).to_string(),
    ///     "-1000000000000");
    /// ```
    pub fn from_twos_complement_limbs_asc(limbs: &[u32]) -> Integer {
        if limbs.is_empty() {
            return Integer::ZERO;
        }
        if !limbs.last().unwrap().get_bit(u64::from(u32::WIDTH) - 1) {
            Natural::from_limbs_asc(limbs).into()
        } else {
            let mut limbs = limbs.to_vec();
            let mut borrow = true;
            for limb in &mut limbs {
                if borrow {
                    let (difference, overflow) = limb.overflowing_sub(1);
                    *limb = !difference;
                    if !overflow {
                        borrow = false;
                    }
                } else {
                    *limb = !*limb;
                }
            }
            // At this point borrow must be false, because limbs has some nonzero elements in this
            // branch
            -Natural::from_limbs_asc(&limbs)
        }
    }

    /// Converts a slice of limbs, or base-2<sup>32</sup> digits, to an `Integer`, in descending
    /// order, so that less significant limbs have higher indices in the input slice. The limbs are
    /// in two's complement, and the most significant bit of the limbs indicates the sign; if the
    /// bit is zero, the `Integer` is non-negative, and if the bit is one it is negative. If `limbs`
    /// is empty, zero is returned.
    ///
    /// This method is less efficient than `from_twos_complement_limbs_asc`.
    ///
    /// Time: worst case O(n)
    ///
    /// Additional memory: worst case O(n)
    ///
    /// where n = `limbs.len()`
    ///
    /// # Examples
    /// ```
    /// use malachite_nz::integer::Integer;
    ///
    /// assert_eq!(Integer::from_twos_complement_limbs_desc(&[]).to_string(), "0");
    /// assert_eq!(Integer::from_twos_complement_limbs_desc(&[123]).to_string(), "123");
    /// assert_eq!(Integer::from_twos_complement_limbs_desc(&[4294967173]).to_string(), "-123");
    /// // 10^12 = 232 * 2^32 + 3567587328
    /// assert_eq!(Integer::from_twos_complement_limbs_desc(&[232, 3567587328]).to_string(),
    ///     "1000000000000");
    /// assert_eq!(Integer::from_twos_complement_limbs_desc(&[4294967063, 727379968]).to_string(),
    ///     "-1000000000000");
    /// ```
    pub fn from_twos_complement_limbs_desc(limbs: &[u32]) -> Integer {
        Integer::from_twos_complement_limbs_asc(&limbs.iter().cloned().rev().collect::<Vec<u32>>())
    }
}
