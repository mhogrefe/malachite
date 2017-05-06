use integer::Integer;
use std::ops::Neg;

/// Takes the negative of `self`.
///
/// # Examples
/// ```
/// use malachite_native::integer::Integer;
///
/// assert_eq!((-Integer::from(0)).to_string(), "0");
/// assert_eq!((-Integer::from(123)).to_string(), "-123");
/// assert_eq!((-Integer::from(-123)).to_string(), "123");
/// ```
impl Neg for Integer {
    type Output = Integer;

    fn neg(mut self) -> Integer {
        if self.abs != 0 {
            self.sign = !self.sign;
        }
        self
    }
}
