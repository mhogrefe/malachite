use malachite_base::num::conversion::traits::{
    CheckedFrom, ConvertibleFrom, OverflowingFrom, SaturatingFrom, WrappingFrom,
};

use integer::Integer;
use platform::Limb;

impl CheckedFrom<Integer> for Limb {
    /// Converts an `Integer` to a `Limb`, taking the `Integer` by value and returning `None` if
    /// the `Integer` is negative or too large.
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
    /// use malachite_base::num::conversion::traits::CheckedFrom;
    /// use malachite_nz::integer::Integer;
    ///
    /// fn main() {
    ///     assert_eq!(format!("{:?}", u32::checked_from(Integer::from(123))), "Some(123)");
    ///     assert_eq!(format!("{:?}", u32::checked_from(Integer::from(-123))), "None");
    ///     assert_eq!(format!("{:?}", u32::checked_from(Integer::trillion())), "None");
    ///     assert_eq!(format!("{:?}", u32::checked_from(-Integer::trillion())), "None");
    /// }
    /// ```
    #[inline]
    fn checked_from(value: Integer) -> Option<Limb> {
        Limb::checked_from(&value)
    }
}

impl<'a> CheckedFrom<&'a Integer> for Limb {
    /// Converts an `Integer` to a `Limb`, taking the `Integer` by reference and returning `None` if
    /// the `Integer` is negative or too large.
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
    /// use malachite_base::num::conversion::traits::CheckedFrom;
    /// use malachite_nz::integer::Integer;
    ///
    /// fn main() {
    ///     assert_eq!(format!("{:?}", u32::checked_from(&Integer::from(123))), "Some(123)");
    ///     assert_eq!(format!("{:?}", u32::checked_from(&Integer::from(-123))), "None");
    ///     assert_eq!(format!("{:?}", u32::checked_from(&Integer::trillion())), "None");
    ///     assert_eq!(format!("{:?}", u32::checked_from(&-Integer::trillion())), "None");
    /// }
    /// ```
    fn checked_from(value: &Integer) -> Option<Limb> {
        match *value {
            Integer { sign: false, .. } => None,
            Integer {
                sign: true,
                ref abs,
            } => Limb::checked_from(abs),
        }
    }
}

#[cfg(feature = "64_bit_limbs")]
impl<'a> CheckedFrom<&'a Integer> for u32 {
    #[inline]
    fn checked_from(value: &Integer) -> Option<u32> {
        Limb::checked_from(value).and_then(u32::checked_from)
    }
}

impl WrappingFrom<Integer> for Limb {
    /// Converts an `Integer` to a `Limb`, taking the `Integer` by value and wrapping mod
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
    /// use malachite_base::num::conversion::traits::WrappingFrom;
    /// use malachite_nz::integer::Integer;
    ///
    /// fn main() {
    ///     assert_eq!(u32::wrapping_from(Integer::from(123)), 123);
    ///     assert_eq!(u32::wrapping_from(Integer::from(-123)), 4294967173);
    ///     assert_eq!(u32::wrapping_from(Integer::trillion()), 3567587328);
    ///     assert_eq!(u32::wrapping_from(-Integer::trillion()), 727379968);
    /// }
    /// ```
    #[inline]
    fn wrapping_from(value: Integer) -> Limb {
        Limb::wrapping_from(&value)
    }
}

impl<'a> WrappingFrom<&'a Integer> for Limb {
    /// Converts an `Integer` to a `Limb`, taking the `Integer` by reference and wrapping mod
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
    /// use malachite_base::num::conversion::traits::WrappingFrom;
    /// use malachite_nz::integer::Integer;
    ///
    /// fn main() {
    ///     assert_eq!(u32::wrapping_from(&Integer::from(123)), 123);
    ///     assert_eq!(u32::wrapping_from(&Integer::from(-123)), 4294967173);
    ///     assert_eq!(u32::wrapping_from(&Integer::trillion()), 3567587328);
    ///     assert_eq!(u32::wrapping_from(&-Integer::trillion()), 727379968);
    /// }
    /// ```
    fn wrapping_from(value: &Integer) -> Limb {
        match *value {
            Integer {
                sign: true,
                ref abs,
            } => Limb::wrapping_from(abs),
            Integer {
                sign: false,
                ref abs,
            } => Limb::wrapping_from(abs).wrapping_neg(),
        }
    }
}

#[cfg(feature = "64_bit_limbs")]
impl<'a> WrappingFrom<&'a Integer> for u32 {
    #[inline]
    fn wrapping_from(value: &Integer) -> u32 {
        u32::wrapping_from(Limb::wrapping_from(value))
    }
}

impl SaturatingFrom<Integer> for Limb {
    /// Converts an `Integer` to a `Limb`, taking the `Integer` by value. If the `Integer` is too
    /// large to fit in a `Limb`, `Limb::MAX` is returned. If it is negative, 0 is returned.
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
    ///
    /// fn main() {
    ///     assert_eq!(u32::saturating_from(Integer::from(123)), 123);
    ///     assert_eq!(u32::saturating_from(Integer::from(-123)), 0);
    ///     assert_eq!(u32::saturating_from(Integer::trillion()), 4294967295);
    ///     assert_eq!(u32::saturating_from(-Integer::trillion()), 0);
    /// }
    /// ```
    fn saturating_from(value: Integer) -> Limb {
        Limb::saturating_from(&value)
    }
}

#[cfg(feature = "64_bit_limbs")]
impl SaturatingFrom<Integer> for u32 {
    #[inline]
    fn saturating_from(value: Integer) -> u32 {
        u32::saturating_from(Limb::saturating_from(value))
    }
}

impl<'a> SaturatingFrom<&'a Integer> for Limb {
    /// Converts an `Integer` to a `Limb`, taking the `Integer` by reference. If the `Integer` is
    /// too large to fit in a `Limb`, `Limb::MAX` is returned. If it is negative, 0 is returned.
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
    ///
    /// fn main() {
    ///     assert_eq!(u32::saturating_from(&Integer::from(123)), 123);
    ///     assert_eq!(u32::saturating_from(&Integer::from(-123)), 0);
    ///     assert_eq!(u32::saturating_from(&Integer::trillion()), 4294967295);
    ///     assert_eq!(u32::saturating_from(&-Integer::trillion()), 0);
    /// }
    /// ```
    fn saturating_from(value: &Integer) -> Limb {
        match *value {
            Integer {
                sign: true,
                ref abs,
            } => Limb::saturating_from(abs),
            _ => 0,
        }
    }
}

#[cfg(feature = "64_bit_limbs")]
impl<'a> SaturatingFrom<&'a Integer> for u32 {
    #[inline]
    fn saturating_from(value: &Integer) -> u32 {
        u32::saturating_from(Limb::saturating_from(value))
    }
}

impl OverflowingFrom<Integer> for Limb {
    /// Converts an `Integer` to a `Limb`, taking the `Integer` by value and wrapping mod
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
    ///
    /// fn main() {
    ///     assert_eq!(u32::overflowing_from(Integer::from(123)), (123, false));
    ///     assert_eq!(u32::overflowing_from(Integer::from(-123)), (4294967173, true));
    ///     assert_eq!(u32::overflowing_from(Integer::trillion()), (3567587328, true));
    ///     assert_eq!(u32::overflowing_from(-Integer::trillion()), (727379968, true));
    /// }
    /// ```
    fn overflowing_from(value: Integer) -> (Limb, bool) {
        Limb::overflowing_from(&value)
    }
}

#[cfg(feature = "64_bit_limbs")]
impl OverflowingFrom<Integer> for u32 {
    #[inline]
    fn overflowing_from(value: Integer) -> (u32, bool) {
        let (result, overflow_1) = Limb::overflowing_from(value);
        let (result, overflow_2) = u32::overflowing_from(result);
        (result, overflow_1 || overflow_2)
    }
}

impl<'a> OverflowingFrom<&'a Integer> for Limb {
    /// Converts an `Integer` to a `Limb`, taking the `Integer` by reference and wrapping mod
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
    ///
    /// fn main() {
    ///     assert_eq!(u32::overflowing_from(&Integer::from(123)), (123, false));
    ///     assert_eq!(u32::overflowing_from(&Integer::from(-123)), (4294967173, true));
    ///     assert_eq!(u32::overflowing_from(&Integer::trillion()), (3567587328, true));
    ///     assert_eq!(u32::overflowing_from(&-Integer::trillion()), (727379968, true));
    /// }
    /// ```
    fn overflowing_from(value: &Integer) -> (Limb, bool) {
        match *value {
            Integer {
                sign: true,
                ref abs,
            } => Limb::overflowing_from(abs),
            Integer {
                sign: false,
                ref abs,
            } => (Limb::wrapping_from(abs).wrapping_neg(), true),
        }
    }
}

#[cfg(feature = "64_bit_limbs")]
impl<'a> OverflowingFrom<&'a Integer> for u32 {
    #[inline]
    fn overflowing_from(value: &'a Integer) -> (u32, bool) {
        let (result, overflow_1) = Limb::overflowing_from(value);
        let (result, overflow_2) = u32::overflowing_from(result);
        (result, overflow_1 || overflow_2)
    }
}

impl ConvertibleFrom<Integer> for Limb {
    /// Determines whether an `Integer` can be converted to a `Limb`. Takes the `Integer` by value.
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
    ///
    /// fn main() {
    ///     assert_eq!(u32::convertible_from(Integer::from(123)), true);
    ///     assert_eq!(u32::convertible_from(Integer::from(-123)), false);
    ///     assert_eq!(u32::convertible_from(Integer::trillion()), false);
    ///     assert_eq!(u32::convertible_from(-Integer::trillion()), false);
    /// }
    /// ```
    #[inline]
    fn convertible_from(value: Integer) -> bool {
        Limb::convertible_from(&value)
    }
}

#[cfg(feature = "64_bit_limbs")]
impl ConvertibleFrom<Integer> for u32 {
    #[inline]
    fn convertible_from(value: Integer) -> bool {
        u32::convertible_from(&value)
    }
}

impl<'a> ConvertibleFrom<&'a Integer> for Limb {
    /// Determines whether an `Integer` can be converted to a `Limb`. Takes the `Integer` by
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
    ///
    /// fn main() {
    ///     assert_eq!(u32::convertible_from(&Integer::from(123)), true);
    ///     assert_eq!(u32::convertible_from(&Integer::from(-123)), false);
    ///     assert_eq!(u32::convertible_from(&Integer::trillion()), false);
    ///     assert_eq!(u32::convertible_from(&-Integer::trillion()), false);
    /// }
    /// ```
    fn convertible_from(value: &Integer) -> bool {
        value.sign && Limb::convertible_from(&value.abs)
    }
}

#[cfg(feature = "64_bit_limbs")]
impl<'a> ConvertibleFrom<&'a Integer> for u32 {
    #[inline]
    fn convertible_from(value: &Integer) -> bool {
        value.sign && u32::convertible_from(&value.abs)
    }
}
