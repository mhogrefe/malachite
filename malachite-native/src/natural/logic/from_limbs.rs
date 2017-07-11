use natural::{Natural, Large, Small};

impl Natural {
    /// Converts a slice of limbs, or base-2^(32) digits, to a `Natural`, in little-endian order, so
    /// that less significant limbs have lower indices in the input slice.
    ///
    /// Time: worst case O(n)
    ///
    /// Additional memory: worst case O(n)
    ///
    /// where n = `limbs.len()`
    ///
    /// This method is more efficient than `Natural::from_limbs_be`.
    ///
    /// # Example
    /// ```
    /// use malachite_native::natural::Natural;
    ///
    /// assert_eq!(Natural::from_limbs_le(&[]).to_string(), "0");
    /// assert_eq!(Natural::from_limbs_le(&[123]).to_string(), "123");
    /// // 10^12 = 232 * 2^32 + 3567587328
    /// assert_eq!(Natural::from_limbs_le(&[3567587328, 232]).to_string(), "1000000000000");
    /// ```
    pub fn from_limbs_le(limbs: &[u32]) -> Natural {
        let mut sig_size = 0;
        for (i, limb) in limbs.iter().enumerate().rev() {
            if *limb != 0 {
                sig_size = i + 1;
                break;
            }
        }
        let limbs = &limbs[0..sig_size];
        match sig_size {
            0 => Small(0u32),
            1 => Small(limbs[0]),
            _ => Large(limbs.to_vec()),
        }
    }

    /// Converts a slice of limbs, or base-2^(32) digits, to a `Natural`, in big-endian order, so
    /// that less significant limbs have higher indices in the input slice.
    ///
    /// Time: worst case O(n)
    ///
    /// Additional memory: worst case O(n)
    ///
    /// where n = `limbs.len()`
    ///
    /// This method is less efficient than `Natural::from_limbs_le`.
    ///
    /// # Example
    /// ```
    /// use malachite_native::natural::Natural;
    ///
    /// assert_eq!(Natural::from_limbs_be(&[]).to_string(), "0");
    /// assert_eq!(Natural::from_limbs_be(&[123]).to_string(), "123");
    /// // 10^12 = 232 * 2^32 + 3567587328
    /// assert_eq!(Natural::from_limbs_be(&[232, 3567587328]).to_string(), "1000000000000");
    /// ```
    pub fn from_limbs_be(limbs: &[u32]) -> Natural {
        Natural::from_limbs_le(&limbs.iter()
                                    .cloned()
                                    .rev()
                                    .collect::<Vec<u32>>())
    }
}
