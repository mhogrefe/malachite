use malachite_base::conversion::{CheckedFrom, ConvertibleFrom, SaturatingFrom};
use malachite_base::num::traits::Zero;

use integer::Integer;
use natural::Natural;

impl CheckedFrom<Integer> for Natural {
    /// Converts an `Integer` to a `Natural`, taking the `Natural` by value. If the `Integer` is
    /// negative, `None` is returned.
    ///
    /// Time: worst case O(1)
    ///
    /// Additional memory: worst case O(1)
    ///
    /// # Examples
    /// ```
    /// extern crate malachite_base;
    /// extern crate malachite_nz;
    ///
    /// use malachite_base::conversion::CheckedFrom;
    /// use malachite_nz::integer::Integer;
    /// use malachite_nz::natural::Natural;
    ///
    /// fn main() {
    ///     assert_eq!(format!("{:?}", Natural::checked_from(Integer::from(123))), "Some(123)");
    ///     assert_eq!(format!("{:?}", Natural::checked_from(Integer::from(-123))), "None");
    ///     assert_eq!(format!("{:?}", Natural::checked_from(Integer::trillion())),
    ///         "Some(1000000000000)");
    ///     assert_eq!(format!("{:?}", Natural::checked_from(-Integer::trillion())), "None");
    /// }
    /// ```
    fn checked_from(value: Integer) -> Option<Natural> {
        match value {
            Integer { sign: false, .. } => None,
            Integer { sign: true, abs } => Some(abs),
        }
    }
}

impl<'a> CheckedFrom<&'a Integer> for Natural {
    /// Converts an `Integer` to a `Natural`, taking the `Natural` by reference. If the `Integer` is
    /// negative, `None` is returned.
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
    /// use malachite_base::conversion::CheckedFrom;
    /// use malachite_nz::integer::Integer;
    /// use malachite_nz::natural::Natural;
    ///
    /// fn main() {
    ///     assert_eq!(format!("{:?}", Natural::checked_from(&Integer::from(123))), "Some(123)");
    ///     assert_eq!(format!("{:?}", Natural::checked_from(&Integer::from(-123))), "None");
    ///     assert_eq!(format!("{:?}", Natural::checked_from(&Integer::trillion())),
    ///         "Some(1000000000000)");
    ///     assert_eq!(format!("{:?}", Natural::checked_from(&(-Integer::trillion()))), "None");
    /// }
    /// ```
    fn checked_from(value: &'a Integer) -> Option<Natural> {
        match *value {
            Integer { sign: false, .. } => None,
            Integer {
                sign: true,
                ref abs,
            } => Some(abs.clone()),
        }
    }
}

impl SaturatingFrom<Integer> for Natural {
    /// Converts an `Integer` to a `Natural`, taking the `Natural` by value. If the `Integer` is
    /// negative, 0 is returned.
    ///
    /// Time: worst case O(1)
    ///
    /// Additional memory: worst case O(1)
    ///
    /// # Examples
    /// ```
    /// extern crate malachite_base;
    /// extern crate malachite_nz;
    ///
    /// use malachite_base::conversion::SaturatingFrom;
    /// use malachite_nz::integer::Integer;
    /// use malachite_nz::natural::Natural;
    ///
    /// fn main() {
    ///     assert_eq!(Natural::saturating_from(Integer::from(123)).to_string(), "123");
    ///     assert_eq!(Natural::saturating_from(Integer::from(-123)).to_string(), "0");
    ///     assert_eq!(Natural::saturating_from(Integer::trillion()).to_string(), "1000000000000");
    ///     assert_eq!(Natural::saturating_from(-Integer::trillion()).to_string(), "0");
    /// }
    /// ```
    fn saturating_from(value: Integer) -> Natural {
        match value {
            Integer { sign: false, .. } => Natural::ZERO,
            Integer { sign: true, abs } => abs,
        }
    }
}

impl<'a> SaturatingFrom<&'a Integer> for Natural {
    /// Converts an `Integer` to a `Natural`, taking the `Natural` by reference. If the `Integer` is
    /// negative, 0 is returned.
    ///
    /// Time: worst case O(1)
    ///
    /// Additional memory: worst case O(1)
    ///
    /// # Examples
    /// ```
    /// extern crate malachite_base;
    /// extern crate malachite_nz;
    ///
    /// use malachite_base::conversion::SaturatingFrom;
    /// use malachite_nz::integer::Integer;
    /// use malachite_nz::natural::Natural;
    ///
    /// fn main() {
    ///     assert_eq!(Natural::saturating_from(&Integer::from(123)).to_string(), "123");
    ///     assert_eq!(Natural::saturating_from(&Integer::from(-123)).to_string(), "0");
    ///     assert_eq!(Natural::saturating_from(&Integer::trillion()).to_string(), "1000000000000");
    ///     assert_eq!(Natural::saturating_from(&-Integer::trillion()).to_string(), "0");
    /// }
    /// ```
    fn saturating_from(value: &'a Integer) -> Natural {
        match *value {
            Integer { sign: false, .. } => Natural::ZERO,
            Integer {
                sign: true,
                ref abs,
            } => abs.clone(),
        }
    }
}

impl ConvertibleFrom<Integer> for Natural {
    /// Determines whether an `Integer` can be converted to a `Natural` (when the `Integer` is
    /// non-negative). Takes the `Integer` by value.
    ///
    /// Time: worst case O(1)
    ///
    /// Additional memory: worst case O(1)
    ///
    /// # Examples
    /// ```
    /// extern crate malachite_base;
    /// extern crate malachite_nz;
    ///
    /// use malachite_base::conversion::ConvertibleFrom;
    /// use malachite_nz::integer::Integer;
    /// use malachite_nz::natural::Natural;
    ///
    /// fn main() {
    ///     assert_eq!(Natural::convertible_from(Integer::from(123)), true);
    ///     assert_eq!(Natural::convertible_from(Integer::from(-123)), false);
    ///     assert_eq!(Natural::convertible_from(Integer::trillion()), true);
    ///     assert_eq!(Natural::convertible_from(-Integer::trillion()), false);
    /// }
    /// ```
    #[inline]
    fn convertible_from(value: Integer) -> bool {
        value.sign
    }
}

impl<'a> ConvertibleFrom<&'a Integer> for Natural {
    /// Determines whether an `Integer` can be converted to a `Natural` (when the `Integer` is
    /// non-negative). Takes the `Integer` by reference.
    ///
    /// Time: worst case O(1)
    ///
    /// Additional memory: worst case O(1)
    ///
    /// # Examples
    /// ```
    /// extern crate malachite_base;
    /// extern crate malachite_nz;
    ///
    /// use malachite_base::conversion::ConvertibleFrom;
    /// use malachite_nz::integer::Integer;
    /// use malachite_nz::natural::Natural;
    ///
    /// fn main() {
    ///     assert_eq!(Natural::convertible_from(&Integer::from(123)), true);
    ///     assert_eq!(Natural::convertible_from(&Integer::from(-123)), false);
    ///     assert_eq!(Natural::convertible_from(&Integer::trillion()), true);
    ///     assert_eq!(Natural::convertible_from(&-Integer::trillion()), false);
    /// }
    /// ```
    #[inline]
    fn convertible_from(value: &'a Integer) -> bool {
        value.sign
    }
}
