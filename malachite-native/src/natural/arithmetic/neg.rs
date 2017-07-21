use integer::Integer;
use natural::Natural;
use std::ops::Neg;

/// Returns the negative of a `Natural`, taking the `Natural` by value and returning an `Integer`.
///
/// Time: worst case O(1)
///
/// Additional memory: worst case O(1)
///
/// # Examples
/// ```
/// use malachite_native::natural::Natural;
///
/// assert_eq!((-Natural::from(0u32)).to_string(), "0");
/// assert_eq!((-Natural::from(123u32)).to_string(), "-123");
/// ```
impl Neg for Natural {
    type Output = Integer;

    fn neg(self) -> Integer {
        if self == 0 {
            Integer::from(0)
        } else {
            Integer {
                sign: false,
                abs: self,
            }
        }
    }
}

/// Returns the negative of a `Natural`, taking the `Natural` by reference and returning an
/// `Integer`.
///
/// Time: worst case O(n)
///
/// Additional memory: worst case O(n)
///
/// where n = `self.significant_bits()`
///
/// # Examples
/// ```
/// use malachite_native::natural::Natural;
///
/// assert_eq!((-&Natural::from(0u32)).to_string(), "0");
/// assert_eq!((-&Natural::from(123u32)).to_string(), "-123");
/// ```
impl<'a> Neg for &'a Natural {
    type Output = Integer;

    fn neg(self) -> Integer {
        if *self == 0 {
            Integer::from(0)
        } else {
            Integer {
                sign: false,
                abs: self.clone(),
            }
        }
    }
}
