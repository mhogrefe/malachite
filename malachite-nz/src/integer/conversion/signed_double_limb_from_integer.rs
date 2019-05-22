use malachite_base::comparison::Min;
use malachite_base::conversion::{
    CheckedFrom, ConvertibleFrom, OverflowingFrom, SaturatingFrom, WrappingFrom,
};
use malachite_base::num::integers::PrimitiveInteger;

use integer::Integer;
use natural::Natural::{Large, Small};
use platform::{DoubleLimb, Limb, SignedDoubleLimb};

impl CheckedFrom<Integer> for SignedDoubleLimb {
    /// Converts an `Integer` to a `SignedDoubleLimb`, taking the `Integer` by value and returning
    /// `None` if the `Integer` is outside the range of a `SignedDoubleLimb`.
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
    ///     assert_eq!(format!("{:?}", i64::checked_from(Integer::from(123))), "Some(123)");
    ///     assert_eq!(format!("{:?}", i64::checked_from(Integer::from(-123))), "Some(-123)");
    ///     assert_eq!(
    ///         format!("{:?}",
    ///         i64::checked_from(Integer::from_str("1000000000000000000000000").unwrap())),
    ///         "None");
    ///     assert_eq!(format!("{:?}",
    ///         i64::checked_from(Integer::from_str("-1000000000000000000000000").unwrap())),
    ///         "None");
    /// }
    /// ```
    #[inline]
    fn checked_from(value: Integer) -> Option<SignedDoubleLimb> {
        SignedDoubleLimb::checked_from(&value)
    }
}

impl<'a> CheckedFrom<&'a Integer> for SignedDoubleLimb {
    /// Converts an `Integer` to a `SignedDoubleLimb`, taking the `Integer` by reference and
    /// returning `None` if the `Integer` is outside the range of a `SignedDoubleLimb`.
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
    ///     assert_eq!(format!("{:?}", i64::checked_from(&Integer::from(123))), "Some(123)");
    ///     assert_eq!(format!("{:?}", i64::checked_from(&Integer::from(-123))), "Some(-123)");
    ///     assert_eq!(
    ///         format!("{:?}",
    ///         i64::checked_from(&Integer::from_str("1000000000000000000000000").unwrap())),
    ///         "None");
    ///     assert_eq!(format!("{:?}",
    ///         i64::checked_from(&Integer::from_str("-1000000000000000000000000").unwrap())),
    ///         "None");
    /// }
    /// ```
    fn checked_from(value: &Integer) -> Option<SignedDoubleLimb> {
        if SignedDoubleLimb::convertible_from(value) {
            Some(SignedDoubleLimb::wrapping_from(value))
        } else {
            None
        }
    }
}

impl WrappingFrom<Integer> for SignedDoubleLimb {
    /// Converts an `Integer` to a `SignedDoubleLimb`, taking the `Integer` by value and wrapping
    /// mod 2<sup>`DoubleLimb::WIDTH`</sup>.
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
    ///     assert_eq!(i64::wrapping_from(Integer::from(123)), 123);
    ///     assert_eq!(i64::wrapping_from(Integer::from(-123)), -123);
    ///     assert_eq!(
    ///         i64::wrapping_from(Integer::from_str("1000000000000000000000000").unwrap()),
    ///         2003764205206896640);
    ///     assert_eq!(
    ///         i64::wrapping_from(Integer::from_str("-1000000000000000000000000").unwrap()),
    ///         -2003764205206896640);
    /// }
    /// ```
    #[inline]
    fn wrapping_from(value: Integer) -> SignedDoubleLimb {
        SignedDoubleLimb::wrapping_from(&value)
    }
}

impl<'a> WrappingFrom<&'a Integer> for SignedDoubleLimb {
    /// Converts an `Integer` to a `SignedDoubleLimb`, taking the `Integer` by reference and
    /// wrapping mod 2<sup>`DoubleLimb::WIDTH`</sup>.
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
    ///     assert_eq!(i64::wrapping_from(&Integer::from(123)), 123);
    ///     assert_eq!(i64::wrapping_from(&Integer::from(-123)), -123);
    ///     assert_eq!(
    ///         i64::wrapping_from(&Integer::from_str("1000000000000000000000000").unwrap()),
    ///         2003764205206896640);
    ///     assert_eq!(
    ///         i64::wrapping_from(&Integer::from_str("-1000000000000000000000000").unwrap()),
    ///         -2003764205206896640);
    /// }
    /// ```
    fn wrapping_from(value: &Integer) -> SignedDoubleLimb {
        SignedDoubleLimb::wrapping_from(DoubleLimb::wrapping_from(value))
    }
}

impl SaturatingFrom<Integer> for SignedDoubleLimb {
    /// Converts an `Integer` to a `SignedDoubleLimb`, taking the `Integer` by value. If the
    /// `Integer` is larger than `SignedDoubleLimb::MAX`, `SignedDoubleLimb::MAX` is returned. If it
    /// is smaller than `SignedDoubleLimb::MIN`, `SignedDoubleLimb::MIN` is returned.
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
    /// use malachite_nz::integer::Integer;
    /// use std::str::FromStr;
    ///
    /// fn main() {
    ///     assert_eq!(i64::saturating_from(Integer::from(123)), 123);
    ///     assert_eq!(i64::saturating_from(Integer::from(-123)), -123);
    ///     assert_eq!(
    ///         i64::saturating_from(Integer::from_str("1000000000000000000000000").unwrap()),
    ///         9223372036854775807);
    ///     assert_eq!(
    ///         i64::saturating_from(Integer::from_str("-1000000000000000000000000").unwrap()),
    ///         -9223372036854775808);
    /// }
    /// ```
    fn saturating_from(value: Integer) -> SignedDoubleLimb {
        SignedDoubleLimb::saturating_from(&value)
    }
}

impl<'a> SaturatingFrom<&'a Integer> for SignedDoubleLimb {
    /// Converts an `Integer` to a `SignedDoubleLimb`, taking the `Integer` by reference. If the
    /// `Integer` is larger than `SignedDoubleLimb::MAX`, `SignedDoubleLimb::MAX` is returned. If it
    /// is smaller than `SignedDoubleLimb::MIN`, `SignedDoubleLimb::MIN` is returned.
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
    /// use malachite_nz::integer::Integer;
    /// use std::str::FromStr;
    ///
    /// fn main() {
    ///     assert_eq!(i64::saturating_from(&Integer::from(123)), 123);
    ///     assert_eq!(i64::saturating_from(&Integer::from(-123)), -123);
    ///     assert_eq!(
    ///         i64::saturating_from(&Integer::from_str("1000000000000000000000000").unwrap()),
    ///         9223372036854775807);
    ///     assert_eq!(
    ///         i64::saturating_from(&Integer::from_str("-1000000000000000000000000").unwrap()),
    ///         -9223372036854775808);
    /// }
    /// ```
    fn saturating_from(value: &Integer) -> SignedDoubleLimb {
        match *value {
            Integer {
                sign: true,
                ref abs,
            } => SignedDoubleLimb::saturating_from(DoubleLimb::saturating_from(abs)),
            Integer {
                sign: false,
                ref abs,
            } => {
                let abs = DoubleLimb::saturating_from(abs);
                if abs.get_highest_bit() {
                    SignedDoubleLimb::MIN
                } else {
                    -SignedDoubleLimb::wrapping_from(abs)
                }
            }
        }
    }
}

impl OverflowingFrom<Integer> for SignedDoubleLimb {
    /// Converts an `Integer` to a `SignedDoubleLimb`, taking the `Integer` by value and wrapping
    /// mod 2<sup>`DoubleLimb::WIDTH`</sup>. The returned boolean value indicates whether wrapping
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
    /// use malachite_nz::integer::Integer;
    /// use std::str::FromStr;
    ///
    /// fn main() {
    ///     assert_eq!(i64::overflowing_from(Integer::from(123)), (123, false));
    ///     assert_eq!(i64::overflowing_from(Integer::from(-123)), (-123, false));
    ///     assert_eq!(
    ///         i64::overflowing_from(Integer::from_str("1000000000000000000000000").unwrap()),
    ///         (2003764205206896640, true));
    ///     assert_eq!(
    ///         i64::overflowing_from(Integer::from_str("-1000000000000000000000000").unwrap()),
    ///         (-2003764205206896640, true));
    /// }
    /// ```
    fn overflowing_from(value: Integer) -> (SignedDoubleLimb, bool) {
        SignedDoubleLimb::overflowing_from(&value)
    }
}

impl<'a> OverflowingFrom<&'a Integer> for SignedDoubleLimb {
    /// Converts an `Integer` to a `SignedDoubleLimb`, taking the `Integer` by reference and
    /// wrapping mod 2<sup>`DoubleLimb::WIDTH`</sup>. The returned boolean value indicates whether
    /// wrapping occurred.
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
    /// use malachite_nz::integer::Integer;
    /// use std::str::FromStr;
    ///
    /// fn main() {
    ///     assert_eq!(i64::overflowing_from(&Integer::from(123)), (123, false));
    ///     assert_eq!(i64::overflowing_from(&Integer::from(-123)), (-123, false));
    ///     assert_eq!(
    ///         i64::overflowing_from(&Integer::from_str("1000000000000000000000000").unwrap()),
    ///         (2003764205206896640, true));
    ///     assert_eq!(
    ///         i64::overflowing_from(&Integer::from_str("-1000000000000000000000000").unwrap()),
    ///         (-2003764205206896640, true));
    /// }
    /// ```
    fn overflowing_from(value: &Integer) -> (SignedDoubleLimb, bool) {
        (
            SignedDoubleLimb::wrapping_from(value),
            !SignedDoubleLimb::convertible_from(value),
        )
    }
}

impl ConvertibleFrom<Integer> for SignedDoubleLimb {
    /// Determines whether an `Integer` can be converted to a `SignedDoubleLimb`. Takes the
    /// `Integer` by value.
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
    /// use malachite_nz::integer::Integer;
    /// use std::str::FromStr;
    ///
    /// fn main() {
    ///     assert_eq!(i64::convertible_from(Integer::from(123)), true);
    ///     assert_eq!(i64::convertible_from(Integer::from(-123)), true);
    ///     assert_eq!(
    ///         i64::convertible_from(Integer::from_str("1000000000000000000000000").unwrap()),
    ///         false);
    ///     assert_eq!(
    ///         i64::convertible_from(Integer::from_str("-1000000000000000000000000").unwrap()),
    ///         false);
    /// }
    /// ```
    #[inline]
    fn convertible_from(value: Integer) -> bool {
        SignedDoubleLimb::convertible_from(&value)
    }
}

impl<'a> ConvertibleFrom<&'a Integer> for SignedDoubleLimb {
    /// Determines whether an `Integer` can be converted to a `SignedDoubleLimb`. Takes the
    /// `Integer` by reference.
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
    /// use malachite_nz::integer::Integer;
    /// use std::str::FromStr;
    ///
    /// fn main() {
    ///     assert_eq!(i64::convertible_from(&Integer::from(123)), true);
    ///     assert_eq!(i64::convertible_from(&Integer::from(-123)), true);
    ///     assert_eq!(
    ///         i64::convertible_from(&Integer::from_str("1000000000000000000000000").unwrap()),
    ///         false);
    ///     assert_eq!(
    ///         i64::convertible_from(&Integer::from_str("-1000000000000000000000000").unwrap()),
    ///         false);
    /// }
    /// ```
    fn convertible_from(value: &Integer) -> bool {
        match *value {
            Integer { abs: Small(_), .. } => true,
            Integer {
                sign,
                abs: Large(ref limbs),
            } => {
                if limbs.len() > 2 {
                    false
                } else if sign {
                    !limbs[1].get_highest_bit()
                } else {
                    // include check for x == SignedDoubleLimb::MIN
                    !limbs[1].get_highest_bit()
                        || limbs[0] == 0 && limbs[1] == 1 << (Limb::WIDTH - 1)
                }
            }
        }
    }
}
