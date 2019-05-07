use malachite_base::conversion::{CheckedFrom, WrappingFrom};
use malachite_base::num::integers::PrimitiveInteger;

use integer::Integer;
use natural::Natural::Small;
use platform::{Limb, SignedLimb};

fn integer_fits_in_signed_limb(x: &Integer) -> bool {
    match *x {
        Integer {
            sign: true,
            abs: Small(x),
        } => !x.get_highest_bit(),
        Integer {
            sign: false,
            abs: Small(x),
        } => !x.get_highest_bit() || x == 1 << (Limb::WIDTH - 1),
        _ => false,
    }
}

impl CheckedFrom<Integer> for SignedLimb {
    /// Converts an `Integer` to a `SignedLimb`, taking the `Integer` by value and returning `None`
    /// if the `Integer` is outside the range of a `SignedLimb`.
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
    /// use malachite_nz::integer::Integer;
    ///
    /// fn main() {
    ///     assert_eq!(format!("{:?}", i32::checked_from(Integer::from(123))), "Some(123)");
    ///     assert_eq!(format!("{:?}", i32::checked_from(Integer::from(-123))), "Some(-123)");
    ///     assert_eq!(format!("{:?}", i32::checked_from(Integer::trillion())), "None");
    ///     assert_eq!(format!("{:?}", i32::checked_from(-Integer::trillion())), "None");
    /// }
    /// ```
    #[inline]
    fn checked_from(value: Integer) -> Option<SignedLimb> {
        SignedLimb::checked_from(&value)
    }
}

impl<'a> CheckedFrom<&'a Integer> for SignedLimb {
    /// Converts an `Integer` to a `SignedLimb`, taking the `Integer` by reference and returning
    /// `None` if the `Integer` is outside the range of a `SignedLimb`.
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
    /// use malachite_nz::integer::Integer;
    ///
    /// fn main() {
    ///     assert_eq!(format!("{:?}", i32::checked_from(&Integer::from(123))), "Some(123)");
    ///     assert_eq!(format!("{:?}", i32::checked_from(&Integer::from(-123))), "Some(-123)");
    ///     assert_eq!(format!("{:?}", i32::checked_from(&Integer::trillion())), "None");
    ///     assert_eq!(format!("{:?}", i32::checked_from(&-Integer::trillion())), "None");
    /// }
    /// ```
    fn checked_from(value: &Integer) -> Option<SignedLimb> {
        if integer_fits_in_signed_limb(value) {
            Some(SignedLimb::wrapping_from(value))
        } else {
            None
        }
    }
}

#[cfg(feature = "64_bit_limbs")]
impl<'a> CheckedFrom<&'a Integer> for i32 {
    #[inline]
    fn checked_from(value: &Integer) -> Option<i32> {
        SignedLimb::checked_from(value).and_then(i32::checked_from)
    }
}

impl WrappingFrom<Integer> for SignedLimb {
    /// Converts an `Integer` to a `SignedLimb`, taking the `Integer` by reference and wrapping mod
    /// 2<sup>`Limb::WIDTH`</sup>.
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
    /// use malachite_nz::integer::Integer;
    ///
    /// fn main() {
    ///     assert_eq!(i32::wrapping_from(Integer::from(123)).to_string(), "123");
    ///     assert_eq!(i32::wrapping_from(Integer::from(-123)).to_string(), "-123");
    ///     assert_eq!(i32::wrapping_from(Integer::trillion()).to_string(), "-727379968");
    ///     assert_eq!(i32::wrapping_from(-Integer::trillion()).to_string(), "727379968");
    /// }
    /// ```
    #[inline]
    fn wrapping_from(value: Integer) -> SignedLimb {
        SignedLimb::wrapping_from(&value)
    }
}

impl<'a> WrappingFrom<&'a Integer> for SignedLimb {
    /// Converts an `Integer` to a `SignedLimb`, taking the `Integer` by reference and wrapping mod
    /// 2<sup>`Limb::WIDTH`</sup>.
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
    /// use malachite_nz::integer::Integer;
    ///
    /// fn main() {
    ///     assert_eq!(i32::wrapping_from(&Integer::from(123)).to_string(), "123");
    ///     assert_eq!(i32::wrapping_from(&Integer::from(-123)).to_string(), "-123");
    ///     assert_eq!(i32::wrapping_from(&Integer::trillion()).to_string(), "-727379968");
    ///     assert_eq!(i32::wrapping_from(&-Integer::trillion()).to_string(), "727379968");
    /// }
    /// ```
    fn wrapping_from(value: &Integer) -> SignedLimb {
        Limb::wrapping_from(value) as SignedLimb
    }
}

#[cfg(feature = "64_bit_limbs")]
impl<'a> WrappingFrom<&'a Integer> for i32 {
    #[inline]
    fn wrapping_from(value: &Integer) -> i32 {
        i32::wrapping_from(SignedLimb::wrapping_from(value))
    }
}
