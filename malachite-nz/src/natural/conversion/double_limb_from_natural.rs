use malachite_base::comparison::Max;
use malachite_base::conversion::{
    CheckedFrom, ConvertibleFrom, OverflowingFrom, SaturatingFrom, WrappingFrom,
};
use malachite_base::num::traits::JoinHalves;
use natural::Natural::{self, Large, Small};
use platform::DoubleLimb;

impl CheckedFrom<Natural> for DoubleLimb {
    /// Converts a `Natural` to a `DoubleLimb`, taking the `Natural` by reference and returning
    /// `None` if the `Natural` is too large.
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
    /// use malachite_base::conversion::CheckedFrom;
    /// use malachite_nz::natural::Natural;
    /// use std::str::FromStr;
    ///
    /// fn main() {
    ///     assert_eq!(format!("{:?}", u64::checked_from(Natural::from(123u32))), "Some(123)");
    ///     assert_eq!(format!("{:?}",
    ///         u64::checked_from(Natural::from_str("1000000000000000000000").unwrap())), "None");
    /// }
    /// ```
    #[inline]
    fn checked_from(value: Natural) -> Option<DoubleLimb> {
        DoubleLimb::checked_from(&value)
    }
}

impl<'a> CheckedFrom<&'a Natural> for DoubleLimb {
    /// Converts a `Natural` to a `DoubleLimb`, taking the `Natural` by reference and returning
    /// `None` if the `Natural` is too large.
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
    /// use malachite_base::conversion::CheckedFrom;
    /// use malachite_nz::natural::Natural;
    /// use std::str::FromStr;
    ///
    /// fn main() {
    ///     assert_eq!(format!("{:?}", u64::checked_from(&Natural::from(123u32))), "Some(123)");
    ///     assert_eq!(format!("{:?}",
    ///         u64::checked_from(&Natural::from_str("1000000000000000000000").unwrap())), "None");
    /// }
    /// ```
    fn checked_from(value: &Natural) -> Option<DoubleLimb> {
        match *value {
            Small(small) => Some(small.into()),
            Large(ref limbs) if limbs.len() == 2 => {
                Some(DoubleLimb::join_halves(limbs[1], limbs[0]))
            }
            Large(_) => None,
        }
    }
}

impl WrappingFrom<Natural> for DoubleLimb {
    /// Converts a `Natural` to a `DoubleLimb`, taking the `Natural` by value and wrapping mod
    /// 2<sup>`DoubleLimb::WIDTH`</sup>.
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
    /// use malachite_base::conversion::WrappingFrom;
    /// use malachite_nz::natural::Natural;
    /// use std::str::FromStr;
    ///
    /// fn main() {
    ///     assert_eq!(u64::wrapping_from(Natural::from(123u32)), 123);
    ///     assert_eq!(u64::wrapping_from(Natural::from_str("1000000000000000000000").unwrap()),
    ///         3875820019684212736);
    /// }
    /// ```
    #[inline]
    fn wrapping_from(value: Natural) -> DoubleLimb {
        DoubleLimb::wrapping_from(&value)
    }
}

impl<'a> WrappingFrom<&'a Natural> for DoubleLimb {
    /// Converts a `Natural` to a `DoubleLimb`, taking the `Natural` by reference and wrapping mod
    /// 2<sup>`DoubleLimb::WIDTH`</sup>.
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
    /// use malachite_base::conversion::WrappingFrom;
    /// use malachite_nz::natural::Natural;
    /// use std::str::FromStr;
    ///
    /// fn main() {
    ///     assert_eq!(u64::wrapping_from(&Natural::from(123u32)), 123);
    ///     assert_eq!(u64::wrapping_from(&Natural::from_str("1000000000000000000000").unwrap()),
    ///         3875820019684212736);
    /// }
    /// ```
    fn wrapping_from(value: &Natural) -> DoubleLimb {
        match *value {
            Small(small) => small.into(),
            Large(ref limbs) => DoubleLimb::join_halves(limbs[1], limbs[0]),
        }
    }
}

impl SaturatingFrom<Natural> for DoubleLimb {
    /// Converts a `Natural` to a `DoubleLimb`, taking the `Natural` by value. If the `Natural` is
    /// too large to fit in a `DoubleLimb`, `DoubleLimb::MAX` is returned.
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
    /// use malachite_base::conversion::SaturatingFrom;
    /// use malachite_nz::natural::Natural;
    /// use std::str::FromStr;
    ///
    /// fn main() {
    ///     assert_eq!(u64::saturating_from(Natural::from(123u32)), 123);
    ///     assert_eq!(u64::saturating_from(Natural::from_str("1000000000000000000000").unwrap()),
    ///         18_446_744_073_709_551_615);
    /// }
    /// ```
    #[inline]
    fn saturating_from(value: Natural) -> DoubleLimb {
        DoubleLimb::saturating_from(&value)
    }
}

impl<'a> SaturatingFrom<&'a Natural> for DoubleLimb {
    /// Converts a `Natural` to a `DoubleLimb`, taking the `Natural` by reference. If the `Natural`
    /// is too large to fit in a `DoubleLimb`, `DoubleLimb::MAX` is returned.
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
    /// use malachite_base::conversion::SaturatingFrom;
    /// use malachite_nz::natural::Natural;
    /// use std::str::FromStr;
    ///
    /// fn main() {
    ///     assert_eq!(u64::saturating_from(&Natural::from(123u32)), 123);
    ///     assert_eq!(u64::saturating_from(&Natural::from_str("1000000000000000000000").unwrap()),
    ///         18_446_744_073_709_551_615);
    /// }
    /// ```
    fn saturating_from(value: &Natural) -> DoubleLimb {
        match *value {
            Small(small) => small.into(),
            Large(ref limbs) if limbs.len() == 2 => DoubleLimb::join_halves(limbs[1], limbs[0]),
            Large(_) => DoubleLimb::MAX,
        }
    }
}

impl OverflowingFrom<Natural> for DoubleLimb {
    /// Converts a `Natural` to a `DoubleLimb`, taking the `Natural` by reference and wrapping mod
    /// 2<sup>`DoubleLimb::WIDTH`</sup>. The returned boolean value indicates whether wrapping
    /// occurred.
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
    /// use malachite_base::conversion::OverflowingFrom;
    /// use malachite_nz::natural::Natural;
    /// use std::str::FromStr;
    ///
    /// fn main() {
    ///     assert_eq!(u64::overflowing_from(Natural::from(123u32)), (123, false));
    ///     assert_eq!(u64::overflowing_from(Natural::from_str("1000000000000000000000").unwrap()),
    ///         (3_875_820_019_684_212_736, true));
    /// }
    /// ```
    fn overflowing_from(value: Natural) -> (DoubleLimb, bool) {
        DoubleLimb::overflowing_from(&value)
    }
}

impl<'a> OverflowingFrom<&'a Natural> for DoubleLimb {
    /// Converts a `Natural` to a `DoubleLimb`, taking the `Natural` by reference and wrapping mod
    /// 2<sup>`DoubleLimb::WIDTH`</sup>. The returned boolean value indicates whether wrapping
    /// occurred.
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
    /// use malachite_base::conversion::OverflowingFrom;
    /// use malachite_nz::natural::Natural;
    /// use std::str::FromStr;
    ///
    /// fn main() {
    ///     assert_eq!(u64::overflowing_from(&Natural::from(123u32)), (123, false));
    ///     assert_eq!(u64::overflowing_from(&Natural::from_str("1000000000000000000000").unwrap()),
    ///         (3_875_820_019_684_212_736, true));
    /// }
    /// ```
    fn overflowing_from(value: &Natural) -> (DoubleLimb, bool) {
        match *value {
            Small(small) => (small.into(), false),
            Large(ref limbs) => (
                DoubleLimb::join_halves(limbs[1], limbs[0]),
                limbs.len() != 2,
            ),
        }
    }
}

impl ConvertibleFrom<Natural> for DoubleLimb {
    /// Determines whether a `Natural` can be converted to a `DoubleLimb`. Takes the `Natural` by
    /// value.
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
    /// use malachite_base::conversion::ConvertibleFrom;
    /// use malachite_nz::natural::Natural;
    /// use std::str::FromStr;
    ///
    /// fn main() {
    ///     assert_eq!(u64::convertible_from(Natural::from(123u32)), true);
    ///     assert_eq!(u64::convertible_from(Natural::from_str("1000000000000000000000").unwrap()),
    ///         false);
    /// }
    /// ```
    #[inline]
    fn convertible_from(value: Natural) -> bool {
        DoubleLimb::convertible_from(&value)
    }
}

impl<'a> ConvertibleFrom<&'a Natural> for DoubleLimb {
    /// Determines whether a `Natural` can be converted to a `DoubleLimb`. Takes the `Natural` by
    /// reference.
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
    /// use malachite_base::conversion::ConvertibleFrom;
    /// use malachite_nz::natural::Natural;
    /// use std::str::FromStr;
    ///
    /// fn main() {
    ///     assert_eq!(u64::convertible_from(&Natural::from(123u32)), true);
    ///     assert_eq!(u64::convertible_from(&Natural::from_str("1000000000000000000000").unwrap()),
    ///         false);
    /// }
    /// ```
    fn convertible_from(value: &Natural) -> bool {
        match *value {
            Small(_) => true,
            Large(ref limbs) => limbs.len() == 2,
        }
    }
}
