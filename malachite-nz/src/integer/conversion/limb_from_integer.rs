use integer::Integer;
use malachite_base::misc::{CheckedFrom, WrappingFrom};
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
    /// use malachite_base::misc::CheckedFrom;
    /// use malachite_nz::integer::Integer;
    ///
    /// fn main() {
    ///     assert_eq!(format!("{:?}", u32::checked_from(Integer::from(123))), "Some(123)");
    ///     assert_eq!(format!("{:?}", u32::checked_from(Integer::from(-123))), "None");
    ///     assert_eq!(format!("{:?}", u32::checked_from(Integer::trillion())), "None");
    ///     assert_eq!(format!("{:?}", u32::checked_from(-Integer::trillion())), "None");
    /// }
    /// ```
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
    /// use malachite_base::misc::CheckedFrom;
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
    /// use malachite_base::misc::WrappingFrom;
    /// use malachite_nz::integer::Integer;
    ///
    /// fn main() {
    ///     assert_eq!(u32::wrapping_from(Integer::from(123)), 123);
    ///     assert_eq!(u32::wrapping_from(Integer::from(-123)), 4294967173);
    ///     assert_eq!(u32::wrapping_from(Integer::trillion()), 3567587328);
    ///     assert_eq!(u32::wrapping_from(-Integer::trillion()), 727379968);
    /// }
    /// ```
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
    /// use malachite_base::misc::WrappingFrom;
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

//TODO test
#[cfg(feature = "64_bit_limbs")]
impl<'a> CheckedFrom<&'a Integer> for u32 {
    fn checked_from(value: &Integer) -> Option<u32> {
        match *value {
            Integer { sign: false, .. } => None,
            Integer {
                sign: true,
                ref abs,
            } => u32::checked_from(abs),
        }
    }
}

//TODO test
#[cfg(feature = "64_bit_limbs")]
impl<'a> WrappingFrom<&'a Integer> for u32 {
    fn wrapping_from(value: &Integer) -> u32 {
        match *value {
            Integer {
                sign: true,
                ref abs,
            } => u32::wrapping_from(abs),
            Integer {
                sign: false,
                ref abs,
            } => u32::wrapping_from(abs).wrapping_neg(),
        }
    }
}
