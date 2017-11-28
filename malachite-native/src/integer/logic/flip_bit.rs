use integer::Integer;

impl Integer {
    /// Flips the `index`th bit of a `Integer`, or the coefficient of 2^(`index`) in its binary
    /// expansion; sets it to 1 if it was 0 and 0 if it was 1.
    ///
    /// Negative integers are treated as though they are represented in two's complement.
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
    /// use malachite_base::traits::{NegativeOne, Zero};
    /// use malachite_native::integer::Integer;
    ///
    /// fn main() {
    ///     let mut x = Integer::ZERO;
    ///     x.flip_bit(10);
    ///     assert_eq!(x.to_string(), "1024");
    ///     x.flip_bit(10);
    ///     assert_eq!(x.to_string(), "0");
    ///
    ///     let mut x = Integer::NEGATIVE_ONE;
    ///     x.flip_bit(10);
    ///     assert_eq!(x.to_string(), "-1025");
    ///     x.flip_bit(10);
    ///     assert_eq!(x.to_string(), "-1");
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
