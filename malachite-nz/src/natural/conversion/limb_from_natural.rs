use malachite_base::misc::{CheckedFrom, WrappingFrom};
use natural::Natural::{self, Large, Small};
use platform::Limb;

impl CheckedFrom<Natural> for Limb {
    /// Converts a `Natural` to a `Limb`, taking the `Natural` by value and returning `None` if the
    /// `Natural` is too large.
    ///
    /// Time: worst case O(1)
    ///
    /// Additional memory: worst case O(1)
    ///
    /// # Example
    /// ```
    /// extern crate malachite_base;
    /// extern crate malachite_nz;
    ///
    /// use malachite_base::misc::CheckedFrom;
    /// use malachite_nz::natural::Natural;
    ///
    /// fn main() {
    ///     assert_eq!(format!("{:?}", u32::checked_from(Natural::from(123u32))), "Some(123)");
    ///     assert_eq!(format!("{:?}", u32::checked_from(Natural::trillion())), "None");
    /// }
    /// ```
    fn checked_from(value: Natural) -> Option<Limb> {
        Limb::checked_from(&value)
    }
}

impl<'a> CheckedFrom<&'a Natural> for Limb {
    /// Converts a `Natural` to a `Limb`, taking the `Natural` by reference and returning `None` if
    /// the `Natural` is too large.
    ///
    /// Time: worst case O(1)
    ///
    /// Additional memory: worst case O(1)
    ///
    /// # Example
    /// ```
    /// extern crate malachite_base;
    /// extern crate malachite_nz;
    ///
    /// use malachite_base::misc::CheckedFrom;
    /// use malachite_nz::natural::Natural;
    ///
    /// fn main() {
    ///     assert_eq!(format!("{:?}", u32::checked_from(&Natural::from(123u32))), "Some(123)");
    ///     assert_eq!(format!("{:?}", u32::checked_from(&Natural::trillion())), "None");
    /// }
    /// ```
    fn checked_from(value: &Natural) -> Option<Limb> {
        match *value {
            Small(small) => Some(small),
            Large(_) => None,
        }
    }
}

impl WrappingFrom<Natural> for Limb {
    /// Converts a `Natural` to a `Limb`, taking the `Natural` by value and wrapping mod
    /// 2<sup>32</sup>.
    ///
    /// Time: worst case O(1)
    ///
    /// Additional memory: worst case O(1)
    ///
    /// # Example
    /// ```
    /// extern crate malachite_base;
    /// extern crate malachite_nz;
    ///
    /// use malachite_base::misc::WrappingFrom;
    /// use malachite_nz::natural::Natural;
    ///
    /// fn main() {
    ///     assert_eq!(u32::wrapping_from(Natural::from(123u32)), 123);
    ///     assert_eq!(u32::wrapping_from(Natural::trillion()), 3567587328);
    /// }
    /// ```
    fn wrapping_from(value: Natural) -> Limb {
        Limb::wrapping_from(&value)
    }
}

impl<'a> WrappingFrom<&'a Natural> for Limb {
    /// Converts a `Natural` to a `Limb`, taking the `Natural` by reference and wrapping mod
    /// 2<sup>32</sup>.
    ///
    /// Time: worst case O(1)
    ///
    /// Additional memory: worst case O(1)
    ///
    /// # Example
    /// ```
    /// extern crate malachite_base;
    /// extern crate malachite_nz;
    ///
    /// use malachite_base::misc::WrappingFrom;
    /// use malachite_nz::natural::Natural;
    ///
    /// fn main() {
    ///     assert_eq!(u32::wrapping_from(&Natural::from(123u32)), 123);
    ///     assert_eq!(u32::wrapping_from(&Natural::trillion()), 3567587328);
    /// }
    /// ```
    fn wrapping_from(value: &Natural) -> Limb {
        match *value {
            Small(small) => small,
            Large(ref limbs) => limbs[0],
        }
    }
}

//TODO test
#[cfg(feature = "64_bit_limbs")]
impl<'a> CheckedFrom<&'a Natural> for u32 {
    fn checked_from(value: &Natural) -> Option<u32> {
        u64::checked_from(value).and_then(u32::checked_from)
    }
}

//TODO test
#[cfg(feature = "64_bit_limbs")]
impl<'a> WrappingFrom<&'a Natural> for u32 {
    fn wrapping_from(value: &Natural) -> u32 {
        u32::wrapping_from(u64::wrapping_from(value))
    }
}
