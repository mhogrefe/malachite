use natural::{Natural, Large};
use traits::Assign;

impl Natural {
    /// Assigns a slice of limbs, or base-2^(32) digits, to `self`, in little-endian order, so that
    /// less significant limbs have lower indices in the input slice.
    ///
    /// # Example
    /// ```
    /// use malachite_native::natural::Natural;
    ///
    /// let mut x = Natural::new();
    /// x.assign_limbs_le(&[]);
    /// assert_eq!(x.to_string(), "0");
    /// x.assign_limbs_le(&[123]);
    /// assert_eq!(x.to_string(), "123");
    /// // 10^12 = 232 * 2^32 + 3567587328
    /// x.assign_limbs_le(&[3567587328, 232]);
    /// assert_eq!(x.to_string(), "1000000000000");
    /// ```
    pub fn assign_limbs_le(&mut self, limbs: &[u32]) {
        let mut sig_size = 0;
        for (i, limb) in limbs.iter().enumerate().rev() {
            if *limb != 0 {
                sig_size = i + 1;
                break;
            }
        }
        let limbs = &limbs[0..sig_size];
        match sig_size {
            0 => self.assign(0),
            1 => self.assign(limbs[0]),
            _ => *self = Large(limbs.to_vec()),
        }
    }
}
