use malachite_base::misc::{CheckedFrom, WrappingFrom};
use natural::Natural::{self, Large, Small};

//TODO is this necessary?
impl CheckedFrom<Natural> for u32 {
    fn checked_from(value: Natural) -> Option<u32> {
        u32::checked_from(&value)
    }
}

//TODO is this necessary?
impl WrappingFrom<Natural> for u32 {
    fn wrapping_from(value: Natural) -> u32 {
        u32::wrapping_from(&value)
    }
}

impl<'a> CheckedFrom<&'a Natural> for u32 {
    /// Converts a `Natural` to a `u32`, returning `None` if the `Natural` is too large.
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
    fn checked_from(value: &Natural) -> Option<u32> {
        match *value {
            Small(small) => Some(small),
            Large(_) => None,
        }
    }
}

impl<'a> WrappingFrom<&'a Natural> for u32 {
    /// Converts a `Natural` to a `u32`, wrapping mod 2<sup>32</sup>.
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
    fn wrapping_from(value: &Natural) -> u32 {
        match *value {
            Small(small) => small,
            Large(ref limbs) => limbs[0],
        }
    }
}
