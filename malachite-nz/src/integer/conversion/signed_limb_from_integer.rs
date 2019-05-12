use malachite_base::comparison::Min;
use malachite_base::conversion::{CheckedFrom, OverflowingFrom, SaturatingFrom, WrappingFrom};
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
    ///     assert_eq!(i32::checked_from(Integer::from(123)), Some(123));
    ///     assert_eq!(i32::checked_from(Integer::from(-123)), Some(-123));
    ///     assert_eq!(i32::checked_from(Integer::trillion()), None);
    ///     assert_eq!(i32::checked_from(-Integer::trillion()), None);
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
    ///     assert_eq!(i32::checked_from(&Integer::from(123)), Some(123));
    ///     assert_eq!(i32::checked_from(&Integer::from(-123)), Some(-123));
    ///     assert_eq!(i32::checked_from(&Integer::trillion()), None);
    ///     assert_eq!(i32::checked_from(&-Integer::trillion()), None);
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
    ///     assert_eq!(i32::wrapping_from(Integer::from(123)), 123);
    ///     assert_eq!(i32::wrapping_from(Integer::from(-123)), -123);
    ///     assert_eq!(i32::wrapping_from(Integer::trillion()), -727379968);
    ///     assert_eq!(i32::wrapping_from(-Integer::trillion()), 727379968);
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
    ///     assert_eq!(i32::wrapping_from(&Integer::from(123)), 123);
    ///     assert_eq!(i32::wrapping_from(&Integer::from(-123)), -123);
    ///     assert_eq!(i32::wrapping_from(&Integer::trillion()), -727379968);
    ///     assert_eq!(i32::wrapping_from(&-Integer::trillion()), 727379968);
    /// }
    /// ```
    fn wrapping_from(value: &Integer) -> SignedLimb {
        SignedLimb::wrapping_from(Limb::wrapping_from(value))
    }
}

#[cfg(feature = "64_bit_limbs")]
impl<'a> WrappingFrom<&'a Integer> for i32 {
    #[inline]
    fn wrapping_from(value: &Integer) -> i32 {
        i32::wrapping_from(SignedLimb::wrapping_from(value))
    }
}

impl SaturatingFrom<Integer> for SignedLimb {
    /// Converts an `Integer` to a `SignedLimb`, taking the `Integer` by value. If the `Integer` is
    /// larger than `SignedLimb::MAX`, `SignedLimb::MAX` is returned. If it is smaller than
    /// `SignedLimb::MIN`, `SignedLimb::MIN` is returned.
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
    ///
    /// fn main() {
    ///     assert_eq!(i32::saturating_from(Integer::from(123)), 123);
    ///     assert_eq!(i32::saturating_from(Integer::from(-123)), -123);
    ///     assert_eq!(i32::saturating_from(Integer::trillion()), 2147483647);
    ///     assert_eq!(i32::saturating_from(-Integer::trillion()), -2147483648);
    /// }
    /// ```
    fn saturating_from(value: Integer) -> SignedLimb {
        SignedLimb::saturating_from(&value)
    }
}

#[cfg(feature = "64_bit_limbs")]
impl SaturatingFrom<Integer> for i32 {
    #[inline]
    fn saturating_from(value: Integer) -> i32 {
        i32::saturating_from(SignedLimb::saturating_from(value))
    }
}

impl<'a> SaturatingFrom<&'a Integer> for SignedLimb {
    /// Converts an `Integer` to a `SignedLimb`, taking the `Integer` by reference. If the `Integer`
    /// is larger than `SignedLimb::MAX`, `SignedLimb::MAX` is returned. If it is smaller than
    /// `SignedLimb::MIN`, `SignedLimb::MIN` is returned.
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
    ///
    /// fn main() {
    ///     assert_eq!(i32::saturating_from(&Integer::from(123)), 123);
    ///     assert_eq!(i32::saturating_from(&Integer::from(-123)), -123);
    ///     assert_eq!(i32::saturating_from(&Integer::trillion()), 2147483647);
    ///     assert_eq!(i32::saturating_from(&-Integer::trillion()), -2147483648);
    /// }
    /// ```
    fn saturating_from(value: &Integer) -> SignedLimb {
        match *value {
            Integer {
                sign: true,
                ref abs,
            } => SignedLimb::saturating_from(Limb::saturating_from(abs)),
            Integer {
                sign: false,
                ref abs,
            } => {
                let abs = Limb::saturating_from(abs);
                if abs.get_highest_bit() {
                    SignedLimb::MIN
                } else {
                    -SignedLimb::wrapping_from(abs)
                }
            }
        }
    }
}

#[cfg(feature = "64_bit_limbs")]
impl<'a> SaturatingFrom<&'a Integer> for i32 {
    #[inline]
    fn saturating_from(value: &Integer) -> i32 {
        i32::saturating_from(SignedLimb::saturating_from(value))
    }
}

impl OverflowingFrom<Integer> for SignedLimb {
    /// Converts an `Integer` to a `SignedLimb`, taking the `Integer` by value and wrapping mod
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
    /// use malachite_base::conversion::OverflowingFrom;
    /// use malachite_nz::integer::Integer;
    ///
    /// fn main() {
    ///     assert_eq!(i32::overflowing_from(Integer::from(123)), (123, false));
    ///     assert_eq!(i32::overflowing_from(Integer::from(-123)), (-123, false));
    ///     assert_eq!(i32::overflowing_from(Integer::trillion()), (-727379968, true));
    ///     assert_eq!(i32::overflowing_from(-Integer::trillion()), (727379968, true));
    /// }
    /// ```
    fn overflowing_from(value: Integer) -> (SignedLimb, bool) {
        SignedLimb::overflowing_from(&value)
    }
}

#[cfg(feature = "64_bit_limbs")]
impl OverflowingFrom<Integer> for i32 {
    #[inline]
    fn overflowing_from(value: Integer) -> (i32, bool) {
        let (result, overflow_1) = SignedLimb::overflowing_from(value);
        let (result, overflow_2) = i32::overflowing_from(result);
        (result, overflow_1 || overflow_2)
    }
}

impl<'a> OverflowingFrom<&'a Integer> for SignedLimb {
    /// Converts an `Integer` to a `SignedLimb`, taking the `Integer` by reference and wrapping mod
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
    /// use malachite_base::conversion::OverflowingFrom;
    /// use malachite_nz::integer::Integer;
    ///
    /// fn main() {
    ///     assert_eq!(i32::overflowing_from(&Integer::from(123)), (123, false));
    ///     assert_eq!(i32::overflowing_from(&Integer::from(-123)), (-123, false));
    ///     assert_eq!(i32::overflowing_from(&Integer::trillion()), (-727379968, true));
    ///     assert_eq!(i32::overflowing_from(&-Integer::trillion()), (727379968, true));
    /// }
    /// ```
    fn overflowing_from(value: &Integer) -> (SignedLimb, bool) {
        (
            SignedLimb::wrapping_from(value),
            !integer_fits_in_signed_limb(value),
        )
    }
}

#[cfg(feature = "64_bit_limbs")]
impl<'a> OverflowingFrom<&'a Integer> for i32 {
    #[inline]
    fn overflowing_from(value: &'a Integer) -> (i32, bool) {
        let (result, overflow_1) = SignedLimb::overflowing_from(value);
        let (result, overflow_2) = i32::overflowing_from(result);
        (result, overflow_1 || overflow_2)
    }
}
