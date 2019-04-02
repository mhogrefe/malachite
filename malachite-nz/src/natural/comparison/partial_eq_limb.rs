use natural::Natural::{self, Large, Small};
use platform::Limb;

/// Determines whether a `Natural` is equal to a `Limb`.
///
/// Time: worst case O(1)
///
/// Additional memory: worst case O(1)
///
/// # Example
/// ```
/// use malachite_nz::natural::Natural;
///
/// assert!(Natural::from(123u32) == 123);
/// assert!(Natural::from(123u32) != 5);
/// ```
impl PartialEq<Limb> for Natural {
    fn eq(&self, other: &Limb) -> bool {
        match *self {
            Small(x) => x == *other,
            Large(_) => false,
        }
    }
}

#[cfg(feature = "64_bit_limbs")]
impl PartialEq<u32> for Natural {
    #[inline]
    fn eq(&self, other: &u32) -> bool {
        PartialEq::eq(self, &Limb::from(*other))
    }
}

/// Determines whether a `Limb` is equal to a `Natural`.
///
/// Time: worst case O(1)
///
/// Additional memory: worst case O(1)
///
/// # Example
/// ```
/// use malachite_nz::natural::Natural;
///
/// assert!(123 == Natural::from(123u32));
/// assert!(5 != Natural::from(123u32));
/// ```
impl PartialEq<Natural> for Limb {
    fn eq(&self, other: &Natural) -> bool {
        match *other {
            Small(y) => y == *self,
            Large(_) => false,
        }
    }
}

#[cfg(feature = "64_bit_limbs")]
impl PartialEq<Natural> for u32 {
    fn eq(&self, other: &Natural) -> bool {
        PartialEq::eq(&Limb::from(*self), other)
    }
}
