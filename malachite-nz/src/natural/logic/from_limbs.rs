use natural::{Large, Natural, Small};

impl Natural {
    /// Converts a slice of limbs, or base-2<sup>32</sup> digits, to a `Natural`, in ascending
    /// order, so that less significant limbs have lower indices in the input slice.
    ///
    /// Time: worst case O(n)
    ///
    /// Additional memory: worst case O(n)
    ///
    /// where n = `limbs.len()`
    ///
    /// This method is more efficient than `Natural::from_limbs_desc`.
    ///
    /// # Example
    /// ```
    /// use malachite_nz::natural::Natural;
    ///
    /// assert_eq!(Natural::from_limbs_asc(&[]).to_string(), "0");
    /// assert_eq!(Natural::from_limbs_asc(&[123]).to_string(), "123");
    /// // 10^12 = 232 * 2^32 + 3567587328
    /// assert_eq!(Natural::from_limbs_asc(&[3567587328, 232]).to_string(), "1000000000000");
    /// ```
    pub fn from_limbs_asc(limbs: &[u32]) -> Natural {
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

    /// Converts a slice of limbs, or base-2<sup>32</sup> digits, to a `Natural`, in descending
    /// order, so that less significant limbs have higher indices in the input slice.
    ///
    /// Time: worst case O(n)
    ///
    /// Additional memory: worst case O(n)
    ///
    /// where n = `limbs.len()`
    ///
    /// This method is less efficient than `Natural::from_limbs_asc`.
    ///
    /// # Example
    /// ```
    /// use malachite_nz::natural::Natural;
    ///
    /// assert_eq!(Natural::from_limbs_desc(&[]).to_string(), "0");
    /// assert_eq!(Natural::from_limbs_desc(&[123]).to_string(), "123");
    /// // 10^12 = 232 * 2^32 + 3567587328
    /// assert_eq!(Natural::from_limbs_desc(&[232, 3567587328]).to_string(), "1000000000000");
    /// ```
    pub fn from_limbs_desc(limbs: &[u32]) -> Natural {
        Natural::from_limbs_asc(&limbs.iter().cloned().rev().collect::<Vec<u32>>())
    }
}
