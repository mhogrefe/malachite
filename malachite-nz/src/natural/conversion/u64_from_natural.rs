use malachite_base::misc::{CheckedFrom, WrappingFrom};
use malachite_base::num::JoinHalves;
use natural::Natural::{self, Large, Small};

impl CheckedFrom<Natural> for u64 {
    /// Converts a `Natural` to a `u64`, taking the `Natural` by reference and returning `None` if
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
    /// use std::str::FromStr;
    ///
    /// fn main() {
    ///     assert_eq!(format!("{:?}", u64::checked_from(Natural::from(123u32))), "Some(123)");
    ///     assert_eq!(format!("{:?}",
    ///         u64::checked_from(Natural::from_str("1000000000000000000000").unwrap())), "None");
    /// }
    /// ```
    fn checked_from(value: Natural) -> Option<u64> {
        u64::checked_from(&value)
    }
}

impl<'a> CheckedFrom<&'a Natural> for u64 {
    /// Converts a `Natural` to a `u64`, taking the `Natural` by reference and returning `None` if
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
    /// use std::str::FromStr;
    ///
    /// fn main() {
    ///     assert_eq!(format!("{:?}", u64::checked_from(&Natural::from(123u32))), "Some(123)");
    ///     assert_eq!(format!("{:?}",
    ///         u64::checked_from(&Natural::from_str("1000000000000000000000").unwrap())), "None");
    /// }
    /// ```
    fn checked_from(value: &Natural) -> Option<u64> {
        match *value {
            Small(small) => Some(small.into()),
            Large(ref limbs) if limbs.len() == 2 => Some(u64::join_halves(limbs[1], limbs[0])),
            Large(_) => None,
        }
    }
}

impl WrappingFrom<Natural> for u64 {
    /// Converts a `Natural` to a `u64`, taking the `Natural` by value and wrapping mod
    /// 2<sup>64</sup>.
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
    /// use std::str::FromStr;
    ///
    /// fn main() {
    ///     assert_eq!(u64::wrapping_from(Natural::from(123u32)), 123);
    ///     assert_eq!(u64::wrapping_from(Natural::from_str("1000000000000000000000").unwrap()),
    ///         3875820019684212736);
    /// }
    /// ```
    fn wrapping_from(value: Natural) -> u64 {
        u64::wrapping_from(&value)
    }
}

impl<'a> WrappingFrom<&'a Natural> for u64 {
    /// Converts a `Natural` to a `u64`, taking the `Natural` by reference and wrapping mod
    /// 2<sup>64</sup>.
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
    /// use std::str::FromStr;
    ///
    /// fn main() {
    ///     assert_eq!(u64::wrapping_from(&Natural::from(123u32)), 123);
    ///     assert_eq!(u64::wrapping_from(&Natural::from_str("1000000000000000000000").unwrap()),
    ///         3875820019684212736);
    /// }
    /// ```
    fn wrapping_from(value: &Natural) -> u64 {
        match *value {
            Small(small) => small.into(),
            Large(ref limbs) => u64::join_halves(limbs[1], limbs[0]),
        }
    }
}
