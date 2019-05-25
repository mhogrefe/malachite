use malachite_base::num::traits::{CheckedSub, SaturatingSub, SaturatingSubAssign, Zero};

use natural::Natural;

/// Subtracts a `Natural` from a `Natural`, taking both `Natural`s by value. If the second `Natural`
/// is greater than the first, returns zero.
///
/// Time: worst case O(n)
///
/// Additional memory: worst case O(1)
///
/// where n = `self.significant_bits()`
///
/// # Examples
/// ```
/// extern crate malachite_base;
/// extern crate malachite_nz;
///
/// use malachite_base::num::traits::{SaturatingSub, Zero};
/// use malachite_nz::natural::Natural;
///
/// fn main() {
///     assert_eq!(Natural::ZERO.saturating_sub(Natural::from(123u32)).to_string(), "0");
///     assert_eq!(Natural::from(123u32).saturating_sub(Natural::ZERO).to_string(), "123");
///     assert_eq!(Natural::from(456u32).saturating_sub(Natural::from(123u32)).to_string(), "333");
///     assert_eq!((Natural::trillion() * 3).saturating_sub(Natural::trillion()).to_string(),
///         "2000000000000");
/// }
/// ```
impl SaturatingSub<Natural> for Natural {
    type Output = Natural;

    fn saturating_sub(self, other: Natural) -> Natural {
        CheckedSub::checked_sub(self, other).unwrap_or(Natural::ZERO)
    }
}

/// Subtracts a `Natural` from a `Natural`, taking the left `Natural` by value and the right
/// `Natural` by reference. If the second `Natural` is greater than the first, returns zero.
///
/// Time: worst case O(n)
///
/// Additional memory: worst case O(n)
///
/// where n = `self.significant_bits()`
///
/// # Examples
/// ```
/// extern crate malachite_base;
/// extern crate malachite_nz;
///
/// use malachite_base::num::traits::{SaturatingSub, Zero};
/// use malachite_nz::natural::Natural;
///
/// fn main() {
///     assert_eq!(Natural::ZERO.saturating_sub(&Natural::from(123u32)).to_string(), "0");
///     assert_eq!(Natural::from(123u32).saturating_sub(&Natural::ZERO).to_string(), "123");
///     assert_eq!(Natural::from(456u32).saturating_sub(&Natural::from(123u32)).to_string(), "333");
///     assert_eq!((Natural::trillion() * 3).saturating_sub(&Natural::trillion()).to_string(),
///         "2000000000000");
/// }
/// ```
impl<'a> SaturatingSub<&'a Natural> for Natural {
    type Output = Natural;

    fn saturating_sub(self, other: &'a Natural) -> Natural {
        CheckedSub::checked_sub(self, other).unwrap_or(Natural::ZERO)
    }
}

/// Subtracts a `Natural` from a `Natural`, taking the left `Natural` by reference and the right
/// `Natural` by value. If the second `Natural` is greater than the first, returns zero.
///
/// Time: worst case O(n)
///
/// Additional memory: worst case O(n)
///
/// where n = `self.significant_bits()`
///
/// # Examples
/// ```
/// extern crate malachite_base;
/// extern crate malachite_nz;
///
/// use malachite_base::num::traits::{SaturatingSub, Zero};
/// use malachite_nz::natural::Natural;
///
/// fn main() {
///     assert_eq!((&Natural::ZERO).saturating_sub(Natural::from(123u32)).to_string(), "0");
///     assert_eq!((&Natural::from(123u32)).saturating_sub(Natural::ZERO).to_string(), "123");
///     assert_eq!((&Natural::from(456u32)).saturating_sub(Natural::from(123u32)).to_string(),
///         "333");
///     assert_eq!((&(Natural::trillion() * 3)).saturating_sub(Natural::trillion()).to_string(),
///         "2000000000000");
/// }
/// ```
impl<'a> SaturatingSub<Natural> for &'a Natural {
    type Output = Natural;

    fn saturating_sub(self, other: Natural) -> Natural {
        CheckedSub::checked_sub(self, other).unwrap_or(Natural::ZERO)
    }
}

/// Subtracts a `Natural` from a `Natural`, taking both `Natural`s by reference. If the second
/// `Natural` is greater than the first, returns zero.
///
/// Time: worst case O(n)
///
/// Additional memory: worst case O(n)
///
/// where n = `self.significant_bits()`
///
/// # Examples
/// ```
/// extern crate malachite_base;
/// extern crate malachite_nz;
///
/// use malachite_base::num::traits::{SaturatingSub, Zero};
/// use malachite_nz::natural::Natural;
///
/// fn main() {
///     assert_eq!((&Natural::ZERO).saturating_sub(&Natural::from(123u32)).to_string(), "0");
///     assert_eq!((&Natural::from(123u32)).saturating_sub(&Natural::ZERO).to_string(), "123");
///     assert_eq!((&Natural::from(456u32)).saturating_sub(&Natural::from(123u32)).to_string(),
///         "333");
///     assert_eq!((&(Natural::trillion() * 3)).saturating_sub(&Natural::trillion()).to_string(),
///         "2000000000000");
/// }
/// ```
impl<'a, 'b> SaturatingSub<&'a Natural> for &'b Natural {
    type Output = Natural;

    fn saturating_sub(self, other: &'a Natural) -> Natural {
        CheckedSub::checked_sub(self, other).unwrap_or(Natural::ZERO)
    }
}

/// Subtracts a `Natural` from another `Natural` in place, taking the `Natural` by value, returning
/// zero if the second `Natural` is greater than the first.
///
/// Time: worst case O(n)
///
/// Additional memory: worst case O(1)
///
/// where n = `self.significant_bits()`
///
/// # Panics
/// Panics if `other` is greater than `self`.
///
/// # Examples
/// ```
/// extern crate malachite_base;
/// extern crate malachite_nz;
///
/// use malachite_base::num::traits::{SaturatingSubAssign, Zero};
/// use malachite_nz::natural::Natural;
///
/// fn main() {
///     let mut x = Natural::from(123u32);
///     x.saturating_sub_assign(Natural::from(123u32));
///     assert_eq!(x.to_string(), "0");
///
///     let mut x = Natural::from(123u32);
///     x.saturating_sub_assign(Natural::ZERO);
///     assert_eq!(x.to_string(), "123");
///
///     let mut x = Natural::from(456u32);
///     x.saturating_sub_assign(Natural::from(123u32));
///     assert_eq!(x.to_string(), "333");
///
///     let mut x = Natural::from(123u32);
///     x.saturating_sub_assign(Natural::from(456u32));
///     assert_eq!(x.to_string(), "0");
/// }
/// ```
impl SaturatingSubAssign<Natural> for Natural {
    fn saturating_sub_assign(&mut self, other: Natural) {
        if self.sub_assign_ref_no_panic(&other) {
            *self = Natural::ZERO;
        }
    }
}

/// Subtracts a `Natural` from another `Natural` in place, taking the `Natural` by reference,
/// returning zero if the second `Natural` is greater than the first.
///
/// Time: worst case O(n)
///
/// Additional memory: worst case O(n)
///
/// where n = `self.significant_bits()`
///
/// # Panics
/// Panics if `other` is greater than `self`.
///
/// # Examples
/// ```
/// extern crate malachite_base;
/// extern crate malachite_nz;
///
/// use malachite_base::num::traits::{SaturatingSubAssign, Zero};
/// use malachite_nz::natural::Natural;
///
/// fn main() {
///     let mut x = Natural::from(123u32);
///     x.saturating_sub_assign(&Natural::from(123u32));
///     assert_eq!(x.to_string(), "0");
///
///     let mut x = Natural::from(123u32);
///     x.saturating_sub_assign(&Natural::ZERO);
///     assert_eq!(x.to_string(), "123");
///
///     let mut x = Natural::from(456u32);
///     x.saturating_sub_assign(&Natural::from(123u32));
///     assert_eq!(x.to_string(), "333");
///
///     let mut x = Natural::from(123u32);
///     x.saturating_sub_assign(&Natural::from(456u32));
///     assert_eq!(x.to_string(), "0");
/// }
/// ```
impl<'a> SaturatingSubAssign<&'a Natural> for Natural {
    fn saturating_sub_assign(&mut self, other: &'a Natural) {
        if self.sub_assign_ref_no_panic(other) {
            *self = Natural::ZERO;
        }
    }
}
