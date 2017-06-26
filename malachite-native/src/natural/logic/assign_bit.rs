use natural::Natural;

impl Natural {
    /// Set the `index`th bit of `self`, or the coefficient of 2^(`index`) in the binary expansion
    /// of `self`, to 1 if `bit` or to 0 if `!bit`.
    ///
    /// # Examples
    /// ```
    /// use malachite_native::natural::Natural;
    ///
    /// let mut x = Natural::new();
    /// x.assign_bit(2, true);
    /// x.assign_bit(5, true);
    /// x.assign_bit(6, true);
    /// assert_eq!(x.to_string(), "100");
    /// x.assign_bit(2, false);
    /// x.assign_bit(5, false);
    /// x.assign_bit(6, false);
    /// assert_eq!(x.to_string(), "0");
    /// ```
    pub fn assign_bit(&mut self, index: u64, bit: bool) {
        if bit {
            self.set_bit(index);
        } else {
            self.clear_bit(index);
        }
    }
}
