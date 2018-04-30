use malachite_base::misc::WrappingFrom;
use natural::Natural::{self, Small};
use std::ops::{BitAnd, BitAndAssign};

/// Takes the bitwise and of a `Natural` and a `u32`, taking the `Natural` by reference. The output
/// is a `u32`.
///
/// Time: worst case O(1)
///
/// Additional memory: worst case O(1)
///
/// ```
/// extern crate malachite_base;
/// extern crate malachite_nz;
///
/// use malachite_base::num::Zero;
/// use malachite_nz::natural::Natural;
///
/// fn main() {
///     assert_eq!(&Natural::ZERO & 123, 0);
///     assert_eq!(&Natural::from(123u32) & 0, 0);
///     assert_eq!(&Natural::from(123u32) & 456, 72);
///     assert_eq!(&(Natural::trillion() + 1) & 123, 1);
/// }
/// ```
impl<'a> BitAnd<u32> for &'a Natural {
    type Output = u32;

    fn bitand(self, other: u32) -> u32 {
        u32::wrapping_from(self) & other
    }
}

/// Takes the bitwise and of a `u32` and a `Natural`, taking the `Natural` by reference. The output
/// is a `u32`.
///
/// Time: worst case O(1)
///
/// Additional memory: worst case O(1)
///
/// ```
/// extern crate malachite_base;
/// extern crate malachite_nz;
///
/// use malachite_base::num::Zero;
/// use malachite_nz::natural::Natural;
///
/// fn main() {
///     assert_eq!(123 & &Natural::ZERO, 0);
///     assert_eq!(0 & &Natural::from(123u32), 0);
///     assert_eq!(456 & &Natural::from(123u32), 72);
///     assert_eq!(123 & &(Natural::trillion() + 1), 1);
/// }
/// ```
impl<'a> BitAnd<&'a Natural> for u32 {
    type Output = u32;

    fn bitand(self, other: &'a Natural) -> u32 {
        other & self
    }
}

/// Bitwise-ands a `u32` with a `Natural` in place.
///
/// Time: worst case O(1)
///
/// Additional memory: worst case O(1)
///
/// # Examples
/// ```
/// use malachite_nz::natural::Natural;
///
/// let mut x = Natural::from(0xffff_ffffu32);
/// x &= 0xf0ff_ffff;
/// x &= 0xfff0_ffff;
/// x &= 0xffff_f0ff;
/// x &= 0xffff_fff0;
/// assert_eq!(x, 0xf0f0_f0f0);
/// ```
impl BitAndAssign<u32> for Natural {
    fn bitand_assign(&mut self, other: u32) {
        *self = Small(&*self & other);
    }
}
