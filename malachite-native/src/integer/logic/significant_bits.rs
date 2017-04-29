use integer::Integer;

impl Integer {
    /// Returns the smallest number of bits necessary to represent the absolute value of `self`. 0
    /// has zero significant bits.
    ///
    /// # Example
    /// ```
    /// use malachite_native::integer::Integer;
    ///
    /// assert_eq!(Integer::from(0).significant_bits(), 0);
    /// assert_eq!(Integer::from(100).significant_bits(), 7);
    /// assert_eq!(Integer::from(-100).significant_bits(), 7);
    /// ```
    pub fn significant_bits(&self) -> u64 {
        self.abs.significant_bits()
    }
}
