use integer::Integer;
use natural::Natural;
use traits::AbsAssign;

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
    pub fn abs(mut self) -> Integer {
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

/// Replaces `self` with its absolute value.
///
/// # Examples
/// ```
/// use malachite_native::integer::Integer;
/// use malachite_native::traits::AbsAssign;
///
/// let mut x = Integer::from(0);
/// x.abs_assign();
/// assert_eq!(x.to_string(), "0");
///
/// let mut x = Integer::from(123);
/// x.abs_assign();
/// assert_eq!(x.to_string(), "123");
///
/// let mut x = Integer::from(-123);
/// x.abs_assign();
/// assert_eq!(x.to_string(), "123");
/// ```
impl AbsAssign for Integer {
    fn abs_assign(&mut self) {
        self.sign = true;
    }
}
