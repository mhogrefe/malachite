use malachite_base::conversion::{CheckedFrom, WrappingFrom};
use malachite_base::num::integers::PrimitiveInteger;

use integer::Integer;
use natural::Natural::{Large, Small};
use platform::{DoubleLimb, Limb, SignedDoubleLimb};

fn integer_fits_in_signed_double_limb(x: &Integer) -> bool {
    match *x {
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
                !limbs[1].get_highest_bit() || limbs[0] == 0 && limbs[1] == 1 << (Limb::WIDTH - 1)
            }
        }
    }
}

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
        if integer_fits_in_signed_double_limb(value) {
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
        DoubleLimb::wrapping_from(value) as SignedDoubleLimb
    }
}
