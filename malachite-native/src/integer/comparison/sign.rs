use integer::Integer;
use std::cmp::Ordering;

impl Integer {
    /// Returns the sign of `self`. Interpret the result as the result of a comparison to zero, so
    /// that `Equal` means zero, `Greater` means positive, and `Less` means negative.
    ///
    /// # Examples
    /// ```
    /// use malachite_native::integer::Integer;
    /// use std::cmp::Ordering;
    ///
    /// assert_eq!(Integer::from(0).sign(), Ordering::Equal);
    /// assert_eq!(Integer::from(123).sign(), Ordering::Greater);
    /// assert_eq!(Integer::from(-123).sign(), Ordering::Less);
    /// ```
    pub fn sign(&self) -> Ordering {
        if self.sign {
            if self.abs == 0 {
                Ordering::Equal
            } else {
                Ordering::Greater
            }
        } else {
            Ordering::Less
        }
    }
}
