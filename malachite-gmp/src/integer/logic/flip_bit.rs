use integer::Integer;

impl Integer {
    /// Flips the `index`th bit of a `Integer`, or the coefficient of 2^(`index`) in its binary
    /// expansion; sets it to 1 if it was 0 and 0 if it was 1.
    ///
    /// # Examples
    /// ```
    /// use malachite_gmp::integer::Integer;
    ///
    /// let mut x = Integer::new();
    /// x.flip_bit(10);
    /// assert_eq!(x.to_string(), "1024");
    /// x.flip_bit(10);
    /// assert_eq!(x.to_string(), "0");
    ///
    /// let mut x = Integer::from(-1);
    /// x.flip_bit(10);
    /// assert_eq!(x.to_string(), "-1025");
    /// x.flip_bit(10);
    /// assert_eq!(x.to_string(), "-1");
    /// ```
    pub fn flip_bit(&mut self, index: u64) {
        if self.get_bit(index) {
            self.clear_bit(index);
        } else {
            self.set_bit(index);
        }
    }
}
