use natural::Natural;

impl Natural {
    /// Flips the `index`th bit of a `Natural`, or the coefficient of 2^(`index`) in its binary
    /// expansion; sets it to 1 if it was 0 and 0 if it was 1.
    ///
    /// Time: worst case O(`index`)
    ///
    /// Additional memory: worst case O(`index`)
    ///
    /// # Examples
    /// ```
    /// extern crate malachite_base;
    /// extern crate malachite_native;
    ///
    /// use malachite_base::traits::Zero;
    /// use malachite_native::natural::Natural;
    ///
    /// fn main() {
    ///     let mut x = Natural::zero();
    ///     x.flip_bit(10);
    ///     assert_eq!(x.to_string(), "1024");
    ///     x.flip_bit(10);
    ///     assert_eq!(x.to_string(), "0");
    /// }
    /// ```
    pub fn flip_bit(&mut self, index: u64) {
        if self.get_bit(index) {
            self.clear_bit(index);
        } else {
            self.set_bit(index);
        }
    }
}
