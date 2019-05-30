use malachite_base::num::conversion::traits::{
    CheckedFrom, ConvertibleFrom, OverflowingFrom, SaturatingFrom, WrappingFrom,
};

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
    /// use malachite_base::num::conversion::traits::CheckedFrom;
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
    /// use malachite_base::num::conversion::traits::CheckedFrom;
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
    /// 2<sup>`DoubleLimb::WIDTH`</sup>.
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
    /// use malachite_base::num::conversion::traits::WrappingFrom;
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
    /// 2<sup>`DoubleLimb::WIDTH`</sup>.
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
    /// use malachite_base::num::conversion::traits::WrappingFrom;
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

impl SaturatingFrom<Integer> for DoubleLimb {
    /// Converts an `Integer` to a `DoubleLimb`, taking the `Integer` by value. If the `Integer` is
    /// too large to fit in a `DoubleLimb`, `DoubleLimb::MAX` is returned. If it is negative, 0 is
    /// returned.
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
    /// use malachite_base::num::conversion::traits::SaturatingFrom;
    /// use malachite_nz::integer::Integer;
    /// use std::str::FromStr;
    ///
    /// fn main() {
    ///     assert_eq!(u64::saturating_from(Integer::from(123)), 123);
    ///     assert_eq!(u64::saturating_from(Integer::from(-123)), 0);
    ///     assert_eq!(
    ///         u64::saturating_from(Integer::from_str("1000000000000000000000").unwrap()),
    ///         18446744073709551615);
    ///     assert_eq!(u64::saturating_from(Integer::from_str("-1000000000000000000000").unwrap()),
    ///         0);
    /// }
    /// ```
    fn saturating_from(value: Integer) -> DoubleLimb {
        DoubleLimb::saturating_from(&value)
    }
}

impl<'a> SaturatingFrom<&'a Integer> for DoubleLimb {
    /// Converts an `Integer` to a `DoubleLimb`, taking the `Integer` by reference. If the `Integer`
    /// is too large to fit in a `DoubleLimb`, `DoubleLimb::MAX` is returned. If it is negative, 0
    /// is returned.
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
    /// use malachite_base::num::conversion::traits::SaturatingFrom;
    /// use malachite_nz::integer::Integer;
    /// use std::str::FromStr;
    ///
    /// fn main() {
    ///     assert_eq!(u64::saturating_from(&Integer::from(123)), 123);
    ///     assert_eq!(u64::saturating_from(&Integer::from(-123)), 0);
    ///     assert_eq!(
    ///         u64::saturating_from(&Integer::from_str("1000000000000000000000").unwrap()),
    ///         18446744073709551615);
    ///     assert_eq!(u64::saturating_from(&Integer::from_str("-1000000000000000000000").unwrap()),
    ///         0);
    /// }
    /// ```
    fn saturating_from(value: &Integer) -> DoubleLimb {
        match *value {
            Integer {
                sign: true,
                ref abs,
            } => DoubleLimb::saturating_from(abs),
            _ => 0,
        }
    }
}

impl OverflowingFrom<Integer> for DoubleLimb {
    /// Converts an `Integer` to a `DoubleLimb`, taking the `Integer` by value and wrapping mod
    /// 2<sup>`Limb::WIDTH`</sup>. The returned boolean value indicates whether wrapping occurred.
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
    /// use malachite_base::num::conversion::traits::OverflowingFrom;
    /// use malachite_nz::integer::Integer;
    /// use std::str::FromStr;
    ///
    /// fn main() {
    ///     assert_eq!(u64::overflowing_from(Integer::from(123)), (123, false));
    ///     assert_eq!(u64::overflowing_from(Integer::from(-123)), (18446744073709551493, true));
    ///     assert_eq!(
    ///         u64::overflowing_from(Integer::from_str("1000000000000000000000").unwrap()),
    ///         (3875820019684212736, true));
    ///     assert_eq!(
    ///         u64::overflowing_from(Integer::from_str("-1000000000000000000000").unwrap()),
    ///         (14570924054025338880, true));
    /// }
    /// ```
    fn overflowing_from(value: Integer) -> (DoubleLimb, bool) {
        DoubleLimb::overflowing_from(&value)
    }
}

impl<'a> OverflowingFrom<&'a Integer> for DoubleLimb {
    /// Converts an `Integer` to a `DoubleLimb`, taking the `Integer` by reference and wrapping mod
    /// 2<sup>`Limb::WIDTH`</sup>. The returned boolean value indicates whether wrapping occurred.
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
    /// use malachite_base::num::conversion::traits::OverflowingFrom;
    /// use malachite_nz::integer::Integer;
    /// use std::str::FromStr;
    ///
    /// fn main() {
    ///     assert_eq!(u64::overflowing_from(&Integer::from(123)), (123, false));
    ///     assert_eq!(u64::overflowing_from(&Integer::from(-123)), (18446744073709551493, true));
    ///     assert_eq!(
    ///         u64::overflowing_from(&Integer::from_str("1000000000000000000000").unwrap()),
    ///         (3875820019684212736, true));
    ///     assert_eq!(
    ///         u64::overflowing_from(&Integer::from_str("-1000000000000000000000").unwrap()),
    ///         (14570924054025338880, true));
    /// }
    /// ```
    fn overflowing_from(value: &Integer) -> (DoubleLimb, bool) {
        match *value {
            Integer {
                sign: true,
                ref abs,
            } => DoubleLimb::overflowing_from(abs),
            Integer {
                sign: false,
                ref abs,
            } => (DoubleLimb::wrapping_from(abs).wrapping_neg(), true),
        }
    }
}

impl ConvertibleFrom<Integer> for DoubleLimb {
    /// Determines whether an `Integer` can be converted to a `DoubleLimb`. Takes the `Integer` by
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
    /// use malachite_base::num::conversion::traits::ConvertibleFrom;
    /// use malachite_nz::integer::Integer;
    /// use std::str::FromStr;
    ///
    /// fn main() {
    ///     assert_eq!(u64::convertible_from(Integer::from(123)), true);
    ///     assert_eq!(u64::convertible_from(Integer::from(-123)), false);
    ///     assert_eq!(
    ///         u64::convertible_from(Integer::from_str("1000000000000000000000").unwrap()), false);
    ///     assert_eq!(
    ///         u64::convertible_from(Integer::from_str("-1000000000000000000000").unwrap()),
    ///         false);
    /// }
    /// ```
    #[inline]
    fn convertible_from(value: Integer) -> bool {
        DoubleLimb::convertible_from(&value)
    }
}

impl<'a> ConvertibleFrom<&'a Integer> for DoubleLimb {
    /// Determines whether an `Integer` can be converted to a `DoubleLimb`. Takes the `Integer` by
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
    /// use malachite_base::num::conversion::traits::ConvertibleFrom;
    /// use malachite_nz::integer::Integer;
    /// use std::str::FromStr;
    ///
    /// fn main() {
    ///     assert_eq!(u64::convertible_from(&Integer::from(123)), true);
    ///     assert_eq!(u64::convertible_from(&Integer::from(-123)), false);
    ///     assert_eq!(
    ///         u64::convertible_from(&Integer::from_str("1000000000000000000000").unwrap()),
    ///         false);
    ///     assert_eq!(
    ///         u64::convertible_from(&Integer::from_str("-1000000000000000000000").unwrap()),
    ///         false);
    /// }
    /// ```
    fn convertible_from(value: &Integer) -> bool {
        value.sign && DoubleLimb::convertible_from(&value.abs)
    }
}
