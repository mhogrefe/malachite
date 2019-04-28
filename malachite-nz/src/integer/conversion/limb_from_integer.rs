use malachite_base::conversion::{CheckedFrom, WrappingFrom};

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
    /// use malachite_base::conversion::CheckedFrom;
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
    /// use malachite_base::conversion::CheckedFrom;
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
    /// 2<sup>32</sup>.
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
    /// 2<sup>32</sup>.
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
