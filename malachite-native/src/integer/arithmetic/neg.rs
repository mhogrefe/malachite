use integer::Integer;
use std::ops::Neg;
use traits::NegAssign;

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

impl<'a> Neg for &'a Integer {
    type Output = Integer;

    fn neg(self) -> Integer {
        let mut negative = self.clone();
        negative.neg_assign();
        negative
    }
}

/// Negates `self`.
///
/// # Examples
/// ```
/// use malachite_native::integer::Integer;
/// use malachite_native::traits::NegAssign;
///
/// let mut x = Integer::from(0);
/// x.neg_assign();
/// assert_eq!(x.to_string(), "0");
///
/// let mut x = Integer::from(123);
/// x.neg_assign();
/// assert_eq!(x.to_string(), "-123");
///
/// let mut x = Integer::from(-123);
/// x.neg_assign();
/// assert_eq!(x.to_string(), "123");
/// ```
impl NegAssign for Integer {
    fn neg_assign(&mut self) {
        if self.abs != 0 {
            self.sign = !self.sign;
        }
    }
}
