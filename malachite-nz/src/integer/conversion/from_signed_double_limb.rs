use malachite_base::num::traits::UnsignedAbs;

use integer::Integer;
use natural::Natural;
use platform::SignedDoubleLimb;

/// Converts a `SignedDoubleLimb` to an `Integer`.
///
/// Time: worst case O(1)
///
/// Additional memory: worst case O(1)
///
/// # Example
/// ```
/// use malachite_nz::integer::Integer;
///
/// assert_eq!(Integer::from(123i64).to_string(), "123");
/// assert_eq!(Integer::from(-123i64).to_string(), "-123");
/// ```
impl From<SignedDoubleLimb> for Integer {
    fn from(i: SignedDoubleLimb) -> Integer {
        Integer {
            sign: i >= 0,
            abs: Natural::from(i.unsigned_abs()),
        }
    }
}
