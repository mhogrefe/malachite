use integer::Integer;
use natural::Natural;
use std::ops::Not;

/// Returns the bitwise complement of a `Natural`, as if it were represented in two's complement,
/// taking the `Natural` by value and returning an `Integer`.
///
/// Time: worst case O(n)
///
/// Additional memory: worst case O(1)
///
/// where n = `self.significant_bits()`
///
/// # Examples
/// ```
/// use malachite_native::natural::Natural;
///
/// assert_eq!((!Natural::from(0u32)).to_string(), "-1");
/// assert_eq!((!Natural::from(123u32)).to_string(), "-124");
/// ```
impl Not for Natural {
    type Output = Integer;

    fn not(self) -> Integer {
        Integer {
            sign: false,
            abs: self + 1,
        }
    }
}

/// Returns the bitwise complement of a `Natural`, as if it were represented in two's complement,
/// taking the `Natural` by reference and returning an `Integer`.
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
/// assert_eq!((!&Natural::from(0u32)).to_string(), "-1");
/// assert_eq!((!&Natural::from(123u32)).to_string(), "-124");
/// ```
impl<'a> Not for &'a Natural {
    type Output = Integer;

    fn not(self) -> Integer {
        Integer {
            sign: false,
            abs: self + 1,
        }
    }
}
