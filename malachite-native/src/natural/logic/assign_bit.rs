use natural::Natural;

impl Natural {
    /// Sets the `index`th bit of a `Natural`, or the coefficient of 2^(`index`) in its binary
    /// expansion, to 1 if `bit` or to 0 if `!bit`.
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
    ///     let mut x = Natural::ZERO;
    ///     x.assign_bit(2, true);
    ///     x.assign_bit(5, true);
    ///     x.assign_bit(6, true);
    ///     assert_eq!(x.to_string(), "100");
    ///     x.assign_bit(2, false);
    ///     x.assign_bit(5, false);
    ///     x.assign_bit(6, false);
    ///     assert_eq!(x.to_string(), "0");
    /// }
    /// ```
    pub fn assign_bit(&mut self, index: u64, bit: bool) {
        if bit {
            self.set_bit(index);
        } else {
            self.clear_bit(index);
        }
    }
}
