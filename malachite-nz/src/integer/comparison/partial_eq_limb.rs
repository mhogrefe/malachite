use integer::Integer;
use platform::Limb;

/// Determines whether an `Integer` is equal to a `Limb`.
///
/// Time: worst case O(1)
///
/// Additional memory: worst case O(1)
///
/// # Example
/// ```
/// use malachite_nz::integer::Integer;
///
/// assert!(Integer::from(123) == 123);
/// assert!(Integer::from(123) != 5);
/// ```
impl PartialEq<Limb> for Integer {
    fn eq(&self, other: &Limb) -> bool {
        self.sign && self.abs == *other
    }
}

#[cfg(feature = "64_bit_limbs")]
impl PartialEq<u32> for Integer {
    #[inline]
    fn eq(&self, other: &u32) -> bool {
        *self == Limb::from(*other)
    }
}

/// Determines whether a `Limb` is equal to an `Integer`.
///
/// Time: worst case O(1)
///
/// Additional memory: worst case O(1)
///
/// # Example
/// ```
/// use malachite_nz::integer::Integer;
///
/// assert!(123 == Integer::from(123));
/// assert!(5 != Integer::from(123));
/// ```
impl PartialEq<Integer> for Limb {
    fn eq(&self, other: &Integer) -> bool {
        other.sign && other.abs == *self
    }
}

#[cfg(feature = "64_bit_limbs")]
impl PartialEq<Integer> for u32 {
    #[inline]
    fn eq(&self, other: &Integer) -> bool {
        Limb::from(*self) == *other
    }
}
