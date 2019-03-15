use integer::Integer;
use malachite_base::num::UnsignedAbs;
use natural::Natural;
use platform::SignedLimb;

/// Converts a `SignedLimb` to an `Integer`.
///
/// Time: worst case O(1)
///
/// Additional memory: worst case O(1)
///
/// # Example
/// ```
/// use malachite_nz::integer::Integer;
///
/// assert_eq!(Integer::from(123).to_string(), "123");
/// assert_eq!(Integer::from(-123).to_string(), "-123");
/// ```
impl From<SignedLimb> for Integer {
    fn from(i: SignedLimb) -> Integer {
        Integer {
            sign: i >= 0,
            abs: Natural::from(i.unsigned_abs()),
        }
    }
}

#[cfg(feature = "64_bit_limbs")]
impl From<i32> for Integer {
    #[inline]
    fn from(i: i32) -> Integer {
        Integer::from(SignedLimb::from(i))
    }
}
