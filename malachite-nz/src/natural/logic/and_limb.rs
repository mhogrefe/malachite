use malachite_base::misc::WrappingFrom;
use natural::Natural::{self, Small};
use platform::Limb;
use std::ops::{BitAnd, BitAndAssign};

/// Interpreting a slice of `u32`s as the limbs (in ascending order) of a `Natural`, returns the
/// bitwise and of the `Natural` and a `u32`. The slice cannot be empty.
///
/// Time: worst case O(1)
///
/// Additional memory: worst case O(1)
///
/// # Panics
/// Panics if `limbs` is empty.
///
/// # Example
/// ```
/// use malachite_nz::natural::logic::and_limb::limbs_and_limb;
///
/// assert_eq!(limbs_and_limb(&[6, 7], 2), 2);
/// assert_eq!(limbs_and_limb(&[100, 101, 102], 10), 0);
/// ```
pub fn limbs_and_limb(limbs: &[Limb], limb: Limb) -> Limb {
    limbs[0] & limb
}

/// Takes the bitwise and of a `Natural` and a `Limb`, taking the `Natural` by value. The output is a
/// `Limb`.
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
///     assert_eq!(Natural::ZERO & 123, 0);
///     assert_eq!(Natural::from(123u32) & 0, 0);
///     assert_eq!(Natural::from(123u32) & 456, 72);
///     assert_eq!((Natural::trillion() + 1) & 123, 1);
/// }
/// ```
impl BitAnd<Limb> for Natural {
    type Output = Limb;

    fn bitand(self, other: Limb) -> Limb {
        Limb::wrapping_from(self) & other
    }
}

/// Takes the bitwise and of a `Natural` and a `Limb`, taking the `Natural` by reference. The output
/// is a `Limb`.
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
impl<'a> BitAnd<Limb> for &'a Natural {
    type Output = Limb;

    fn bitand(self, other: Limb) -> Limb {
        Limb::wrapping_from(self) & other
    }
}

/// Takes the bitwise and of a `Limb` and a `Natural`, taking the `Natural` by value. The output is a
/// `Limb`.
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
///     assert_eq!(123 & Natural::ZERO, 0);
///     assert_eq!(0 & Natural::from(123u32), 0);
///     assert_eq!(456 & Natural::from(123u32), 72);
///     assert_eq!(123 & (Natural::trillion() + 1), 1);
/// }
/// ```
impl BitAnd<Natural> for Limb {
    type Output = Limb;

    fn bitand(self, other: Natural) -> Limb {
        other & self
    }
}

/// Takes the bitwise and of a `Limb` and a `Natural`, taking the `Natural` by reference. The output
/// is a `Limb`.
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
impl<'a> BitAnd<&'a Natural> for Limb {
    type Output = Limb;

    fn bitand(self, other: &'a Natural) -> Limb {
        other & self
    }
}

/// Bitwise-ands a `Natural` with a `Limb` in place.
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
impl BitAndAssign<Limb> for Natural {
    fn bitand_assign(&mut self, other: Limb) {
        *self = Small(&*self & other);
    }
}

/// Bitwise-ands a `Limb` with a `Natural` in place, taking the `Natural` by value.
///
/// Time: worst case O(1)
///
/// Additional memory: worst case O(1)
///
/// # Examples
/// ```
/// use malachite_nz::natural::Natural;
///
/// let mut x = 0xffff_ffffu32;
/// x &= Natural::from(0xf0ff_ffffu32);
/// x &= Natural::from(0xfff0_ffffu32);
/// x &= Natural::from(0xffff_f0ffu32);
/// x &= Natural::from(0xffff_fff0u32);
/// assert_eq!(x, 0xf0f0_f0f0);
/// ```
impl BitAndAssign<Natural> for Limb {
    fn bitand_assign(&mut self, other: Natural) {
        *self = *self & other;
    }
}

/// Bitwise-ands a `Limb` with a `Natural` in place, taking the `Natural` by reference.
///
/// Time: worst case O(1)
///
/// Additional memory: worst case O(1)
///
/// # Examples
/// ```
/// use malachite_nz::natural::Natural;
///
/// let mut x = 0xffff_ffffu32;
/// x &= &Natural::from(0xf0ff_ffffu32);
/// x &= &Natural::from(0xfff0_ffffu32);
/// x &= &Natural::from(0xffff_f0ffu32);
/// x &= &Natural::from(0xffff_fff0u32);
/// assert_eq!(x, 0xf0f0_f0f0);
/// ```
impl<'a> BitAndAssign<&'a Natural> for Limb {
    fn bitand_assign(&mut self, other: &'a Natural) {
        *self = *self & other;
    }
}
