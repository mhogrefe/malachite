use integer::Integer;
use std::cmp::Ordering;

impl PartialOrd for Integer {
    /// Compares an `Integer` to another `Integer`.
    ///
    /// See the documentation for the `Ord` implementation.
    #[inline]
    fn partial_cmp(&self, other: &Integer) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Integer {
    /// Compares an `Integer` to another `Integer`.
    ///
    /// Time: worst case O(n)
    ///
    /// Additional memory: worst case O(1)
    ///
    /// where n = min(`self.significant_bits()`, `other.significant_bits()`)
    ///
    /// # Examples
    /// ```
    /// use malachite_nz::integer::Integer;
    ///
    /// assert!(Integer::from(-123) < Integer::from(-122));
    /// assert!(Integer::from(-123) <= Integer::from(-122));
    /// assert!(Integer::from(-123) > Integer::from(-124));
    /// assert!(Integer::from(-123) >= Integer::from(-124));
    /// ```
    fn cmp(&self, other: &Integer) -> Ordering {
        if std::ptr::eq(self, other) {
            Ordering::Equal
        } else {
            match (self.sign, other.sign) {
                (true, false) => Ordering::Greater,
                (false, true) => Ordering::Less,
                (true, true) => self.abs.cmp(&other.abs),
                (false, false) => other.abs.cmp(&self.abs),
            }
        }
    }
}
