use integer::Integer;
use std::ops::Not;
use traits::NotAssign;

/// Returns the bitwise complement of an `Integer`, as if it were represented in two's complement,
/// taking the `Integer` by value.
///
/// Time: worst case O(n)
///
/// Additional memory: worst case O(1)
///
/// where n = `self.significant_bits()`
///
/// # Examples
/// ```
/// use malachite_native::integer::Integer;
///
/// assert_eq!((!Integer::from(0)).to_string(), "-1");
/// assert_eq!((!Integer::from(123)).to_string(), "-124");
/// assert_eq!((!Integer::from(-123)).to_string(), "122");
/// ```
impl Not for Integer {
    type Output = Integer;

    fn not(mut self) -> Integer {
        self.not_assign();
        self
    }
}

/// Returns the bitwise complement of an `Integer`, as if it were represented in two's complement,
/// taking the `Integer` by reference.
///
/// Time: worst case O(n)
///
/// Additional memory: worst case O(n)
///
/// where n = `self.significant_bits()`
///
/// # Examples
/// ```
/// use malachite_native::integer::Integer;
///
/// assert_eq!((!&Integer::from(0)).to_string(), "-1");
/// assert_eq!((!&Integer::from(123)).to_string(), "-124");
/// assert_eq!((!&Integer::from(-123)).to_string(), "122");
/// ```
impl<'a> Not for &'a Integer {
    type Output = Integer;

    fn not(self) -> Integer {
        match *self {
            Integer { sign: true, ref abs } => {
                Integer {
                    sign: false,
                    abs: abs + 1,
                }
            }
            Integer { sign: false, ref abs } => {
                Integer {
                    sign: true,
                    abs: (abs - 1).unwrap(),
                }
            }
        }
    }
}

/// Replaces an `Integer` with its bitwise complement, as if it were represented in two's
/// complement.
///
/// Time: worst case O(n)
///
/// Additional memory: worst case O(1)
///
/// where n = `self.significant_bits()`
///
/// # Examples
/// ```
/// use malachite_native::integer::Integer;
/// use malachite_native::traits::NotAssign;
///
/// let mut x = Integer::from(0);
/// x.not_assign();
/// assert_eq!(x.to_string(), "-1");
///
/// let mut x = Integer::from(123);
/// x.not_assign();
/// assert_eq!(x.to_string(), "-124");
///
/// let mut x = Integer::from(-123);
/// x.not_assign();
/// assert_eq!(x.to_string(), "122");
/// ```
impl NotAssign for Integer {
    fn not_assign(&mut self) {
        if self.sign {
            self.sign = false;
            self.abs += 1;
        } else {
            self.sign = true;
            self.abs -= 1;
        }
    }
}
