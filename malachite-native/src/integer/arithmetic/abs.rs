use integer::Integer;
use natural::Natural;

impl Integer {
    /// Takes the absolute value of `self`.
    ///
    /// # Examples
    /// ```
    /// use malachite_native::integer::Integer;
    ///
    /// assert_eq!(Integer::from(0).abs().to_string(), "0");
    /// assert_eq!(Integer::from(123).abs().to_string(), "123");
    /// assert_eq!(Integer::from(-123).abs().to_string(), "123");
    /// ```
    pub fn abs(&mut self) -> &mut Integer {
        self.sign = true;
        self
    }

    /// Takes the absolute value of `self`, converting the result to a `Natural`.
    ///
    /// # Examples
    /// ```
    /// use malachite_native::integer::Integer;
    ///
    /// assert_eq!(Integer::from(0).unsigned_abs().to_string(), "0");
    /// assert_eq!(Integer::from(123).unsigned_abs().to_string(), "123");
    /// assert_eq!(Integer::from(-123).unsigned_abs().to_string(), "123");
    /// ```
    pub fn unsigned_abs(self) -> Natural {
        self.abs
    }
}
