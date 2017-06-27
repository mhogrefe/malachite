use natural::Natural;

impl Natural {
    /// Flips the `index`th bit of `self`, or the coefficient of 2^(`index`) in the binary expansion
    /// of `self`; sets it to 1 if it was 0 and 0 if it was 1.
    ///
    /// Time: worst case O(`index`)
    ///
    /// Additional memory: worst case O(`index`)
    ///
    /// # Examples
    /// ```
    /// use malachite_native::natural::Natural;
    ///
    /// let mut x = Natural::new();
    /// x.flip_bit(10);
    /// assert_eq!(x.to_string(), "1024");
    /// x.flip_bit(10);
    /// assert_eq!(x.to_string(), "0");
    /// ```
    pub fn flip_bit(&mut self, index: u64) {
        if self.get_bit(index) {
            self.clear_bit(index);
        } else {
            self.set_bit(index);
        }
    }
}
