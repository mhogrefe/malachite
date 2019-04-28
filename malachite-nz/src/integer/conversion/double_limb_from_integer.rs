use malachite_base::conversion::{CheckedFrom, WrappingFrom};

use integer::Integer;
use platform::DoubleLimb;

impl CheckedFrom<Integer> for DoubleLimb {
    /// Converts an `Integer` to a `DoubleLimb`, taking the `Integer` by value and returning `None`
    /// if the `Integer` is negative or too large.
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
    /// use std::str::FromStr;
    ///
    /// fn main() {
    ///     assert_eq!(format!("{:?}", u64::checked_from(Integer::from(123))), "Some(123)");
    ///     assert_eq!(format!("{:?}", u64::checked_from(Integer::from(-123))), "None");
    ///     assert_eq!(format!("{:?}",
    ///         u64::checked_from(Integer::from_str("1000000000000000000000").unwrap())), "None");
    ///     assert_eq!(format!("{:?}",
    ///         u64::checked_from(Integer::from_str("-1000000000000000000000").unwrap())), "None");
    /// }
    /// ```
    #[inline]
    fn checked_from(value: Integer) -> Option<DoubleLimb> {
        DoubleLimb::checked_from(&value)
    }
}

impl<'a> CheckedFrom<&'a Integer> for DoubleLimb {
    /// Converts an `Integer` to a `DoubleLimb`, taking the `Integer` by reference and returning
    /// `None` if the `Integer` is negative or too large.
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
    /// use std::str::FromStr;
    ///
    /// fn main() {
    ///     assert_eq!(format!("{:?}", u64::checked_from(&Integer::from(123))), "Some(123)");
    ///     assert_eq!(format!("{:?}", u64::checked_from(&Integer::from(-123))), "None");
    ///     assert_eq!(format!("{:?}",
    ///         u64::checked_from(&Integer::from_str("1000000000000000000000").unwrap())), "None");
    ///     assert_eq!(format!("{:?}",
    ///         u64::checked_from(&Integer::from_str("-1000000000000000000000").unwrap())), "None");
    /// }
    /// ```
    fn checked_from(value: &Integer) -> Option<DoubleLimb> {
        match *value {
            Integer { sign: false, .. } => None,
            Integer {
                sign: true,
                ref abs,
            } => DoubleLimb::checked_from(abs),
        }
    }
}

impl WrappingFrom<Integer> for DoubleLimb {
    /// Converts an `Integer` to a `DoubleLimb`, taking the `Integer` by value and wrapping mod
    /// 2<pow>64</pow>.
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
    /// use malachite_base::conversion::WrappingFrom;
    /// use malachite_nz::integer::Integer;
    /// use std::str::FromStr;
    ///
    /// fn main() {
    ///     assert_eq!(u64::wrapping_from(Integer::from(123)), 123);
    ///     assert_eq!(u64::wrapping_from(Integer::from(-123)), 18446744073709551493);
    ///     assert_eq!(
    ///         u64::wrapping_from(Integer::from_str("1000000000000000000000").unwrap()),
    ///         3875820019684212736);
    ///     assert_eq!(
    ///         u64::wrapping_from(Integer::from_str("-1000000000000000000000").unwrap()),
    ///         14570924054025338880);
    /// }
    /// ```
    #[inline]
    fn wrapping_from(value: Integer) -> DoubleLimb {
        DoubleLimb::wrapping_from(&value)
    }
}

impl<'a> WrappingFrom<&'a Integer> for DoubleLimb {
    /// Converts an `Integer` to a `DoubleLimb`, taking the `Integer` by reference and wrapping mod
    /// 2<pow>64</pow>.
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
    /// use malachite_base::conversion::WrappingFrom;
    /// use malachite_nz::integer::Integer;
    /// use std::str::FromStr;
    ///
    /// fn main() {
    ///     assert_eq!(u64::wrapping_from(&Integer::from(123)), 123);
    ///     assert_eq!(u64::wrapping_from(&Integer::from(-123)), 18446744073709551493);
    ///     assert_eq!(
    ///         u64::wrapping_from(&Integer::from_str("1000000000000000000000").unwrap()),
    ///         3875820019684212736);
    ///     assert_eq!(
    ///         u64::wrapping_from(&Integer::from_str("-1000000000000000000000").unwrap()),
    ///         14570924054025338880);
    /// }
    /// ```
    fn wrapping_from(value: &Integer) -> DoubleLimb {
        match *value {
            Integer {
                sign: true,
                ref abs,
            } => DoubleLimb::wrapping_from(abs),
            Integer {
                sign: false,
                ref abs,
            } => DoubleLimb::wrapping_from(abs).wrapping_neg(),
        }
    }
}
