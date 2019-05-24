use malachite_base::comparison::Max;
use malachite_base::conversion::{
    CheckedFrom, ConvertibleFrom, OverflowingFrom, SaturatingFrom, WrappingFrom,
};
use natural::Natural::{self, Large, Small};
use platform::Limb;

impl CheckedFrom<Natural> for Limb {
    /// Converts a `Natural` to a `Limb`, taking the `Natural` by value and returning `None` if the
    /// `Natural` is too large.
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
    /// use malachite_nz::natural::Natural;
    ///
    /// fn main() {
    ///     assert_eq!(u32::checked_from(Natural::from(123u32)), Some(123));
    ///     assert_eq!(u32::checked_from(Natural::trillion()), None);
    /// }
    /// ```
    #[inline]
    fn checked_from(value: Natural) -> Option<Limb> {
        Limb::checked_from(&value)
    }
}

#[cfg(feature = "64_bit_limbs")]
impl CheckedFrom<Natural> for u32 {
    #[inline]
    fn checked_from(value: Natural) -> Option<u32> {
        u64::checked_from(value).and_then(u32::checked_from)
    }
}

impl<'a> CheckedFrom<&'a Natural> for Limb {
    /// Converts a `Natural` to a `Limb`, taking the `Natural` by reference and returning `None` if
    /// the `Natural` is too large.
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
    /// use malachite_nz::natural::Natural;
    ///
    /// fn main() {
    ///     assert_eq!(u32::checked_from(&Natural::from(123u32)), Some(123));
    ///     assert_eq!(u32::checked_from(&Natural::trillion()), None);
    /// }
    /// ```
    fn checked_from(value: &Natural) -> Option<Limb> {
        match *value {
            Small(small) => Some(small),
            Large(_) => None,
        }
    }
}

#[cfg(feature = "64_bit_limbs")]
impl<'a> CheckedFrom<&'a Natural> for u32 {
    #[inline]
    fn checked_from(value: &Natural) -> Option<u32> {
        u64::checked_from(value).and_then(u32::checked_from)
    }
}

impl WrappingFrom<Natural> for Limb {
    /// Converts a `Natural` to a `Limb`, taking the `Natural` by value and wrapping mod
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
    /// use malachite_nz::natural::Natural;
    ///
    /// fn main() {
    ///     assert_eq!(u32::wrapping_from(Natural::from(123u32)), 123);
    ///     assert_eq!(u32::wrapping_from(Natural::trillion()), 3_567_587_328);
    /// }
    /// ```
    #[inline]
    fn wrapping_from(value: Natural) -> Limb {
        Limb::wrapping_from(&value)
    }
}

#[cfg(feature = "64_bit_limbs")]
impl WrappingFrom<Natural> for u32 {
    #[inline]
    fn wrapping_from(value: Natural) -> u32 {
        u32::wrapping_from(u64::wrapping_from(value))
    }
}

impl<'a> WrappingFrom<&'a Natural> for Limb {
    /// Converts a `Natural` to a `Limb`, taking the `Natural` by reference and wrapping mod
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
    /// use malachite_nz::natural::Natural;
    ///
    /// fn main() {
    ///     assert_eq!(u32::wrapping_from(&Natural::from(123u32)), 123);
    ///     assert_eq!(u32::wrapping_from(&Natural::trillion()), 3_567_587_328);
    /// }
    /// ```
    fn wrapping_from(value: &Natural) -> Limb {
        match *value {
            Small(small) => small,
            Large(ref limbs) => limbs[0],
        }
    }
}

#[cfg(feature = "64_bit_limbs")]
impl<'a> WrappingFrom<&'a Natural> for u32 {
    #[inline]
    fn wrapping_from(value: &Natural) -> u32 {
        u32::wrapping_from(u64::wrapping_from(value))
    }
}

impl SaturatingFrom<Natural> for Limb {
    /// Converts a `Natural` to a `Limb`, taking the `Natural` by value. If the `Natural` is too
    /// large to fit in a `Limb`, `Limb::MAX` is returned.
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
    /// use malachite_nz::natural::Natural;
    ///
    /// fn main() {
    ///     assert_eq!(u32::saturating_from(Natural::from(123u32)), 123);
    ///     assert_eq!(u32::saturating_from(Natural::trillion()), 4_294_967_295);
    /// }
    /// ```
    #[inline]
    fn saturating_from(value: Natural) -> Limb {
        Limb::saturating_from(&value)
    }
}

#[cfg(feature = "64_bit_limbs")]
impl SaturatingFrom<Natural> for u32 {
    #[inline]
    fn saturating_from(value: Natural) -> u32 {
        u32::saturating_from(u64::saturating_from(value))
    }
}

impl<'a> SaturatingFrom<&'a Natural> for Limb {
    /// Converts a `Natural` to a `Limb`, taking the `Natural` by reference. If the `Natural` is too
    /// large to fit in a `Limb`, `Limb::MAX` is returned.
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
    /// use malachite_nz::natural::Natural;
    ///
    /// fn main() {
    ///     assert_eq!(u32::saturating_from(&Natural::from(123u32)), 123);
    ///     assert_eq!(u32::saturating_from(&Natural::trillion()), 4_294_967_295);
    /// }
    /// ```
    fn saturating_from(value: &Natural) -> Limb {
        match *value {
            Small(small) => small,
            Large(_) => Limb::MAX,
        }
    }
}

#[cfg(feature = "64_bit_limbs")]
impl<'a> SaturatingFrom<&'a Natural> for u32 {
    #[inline]
    fn saturating_from(value: &'a Natural) -> u32 {
        u32::saturating_from(u64::saturating_from(value))
    }
}

impl OverflowingFrom<Natural> for Limb {
    /// Converts a `Natural` to a `Limb`, taking the `Natural` by reference and wrapping mod
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
    /// use malachite_nz::natural::Natural;
    ///
    /// fn main() {
    ///     assert_eq!(u32::overflowing_from(Natural::from(123u32)), (123, false));
    ///     assert_eq!(u32::overflowing_from(Natural::trillion()), (3_567_587_328, true));
    /// }
    /// ```
    fn overflowing_from(value: Natural) -> (Limb, bool) {
        Limb::overflowing_from(&value)
    }
}

#[cfg(feature = "64_bit_limbs")]
impl OverflowingFrom<Natural> for u32 {
    #[inline]
    fn overflowing_from(value: Natural) -> (u32, bool) {
        let (result, overflow_1) = Limb::overflowing_from(value);
        let (result, overflow_2) = u32::overflowing_from(result);
        (result, overflow_1 || overflow_2)
    }
}

impl<'a> OverflowingFrom<&'a Natural> for Limb {
    /// Converts a `Natural` to a `Limb`, taking the `Natural` by reference and wrapping mod
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
    /// use malachite_nz::natural::Natural;
    ///
    /// fn main() {
    ///     assert_eq!(u32::overflowing_from(&Natural::from(123u32)), (123, false));
    ///     assert_eq!(u32::overflowing_from(&Natural::trillion()), (3_567_587_328, true));
    /// }
    /// ```
    fn overflowing_from(value: &Natural) -> (Limb, bool) {
        match *value {
            Small(small) => (small, false),
            Large(ref limbs) => (limbs[0], true),
        }
    }
}

#[cfg(feature = "64_bit_limbs")]
impl<'a> OverflowingFrom<&'a Natural> for u32 {
    #[inline]
    fn overflowing_from(value: &'a Natural) -> (u32, bool) {
        let (result, overflow_1) = Limb::overflowing_from(value);
        let (result, overflow_2) = u32::overflowing_from(result);
        (result, overflow_1 || overflow_2)
    }
}

impl ConvertibleFrom<Natural> for Limb {
    /// Determines whether a `Natural` can be converted to a `Limb`. Takes the `Natural` by value.
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
    /// use malachite_nz::natural::Natural;
    ///
    /// fn main() {
    ///     assert_eq!(u32::convertible_from(Natural::from(123u32)), true);
    ///     assert_eq!(u32::convertible_from(Natural::trillion()), false);
    /// }
    /// ```
    #[inline]
    fn convertible_from(value: Natural) -> bool {
        Limb::convertible_from(&value)
    }
}

#[cfg(feature = "64_bit_limbs")]
impl ConvertibleFrom<Natural> for u32 {
    #[inline]
    fn convertible_from(value: Natural) -> bool {
        u32::convertible_from(&value)
    }
}

impl<'a> ConvertibleFrom<&'a Natural> for Limb {
    /// Determines whether a `Natural` can be converted to a `Limb`. Takes the `Natural` by
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
    /// use malachite_base::conversion::ConvertibleFrom;
    /// use malachite_nz::natural::Natural;
    ///
    /// fn main() {
    ///     assert_eq!(u32::convertible_from(&Natural::from(123u32)), true);
    ///     assert_eq!(u32::convertible_from(&Natural::trillion()), false);
    /// }
    /// ```
    fn convertible_from(value: &Natural) -> bool {
        match *value {
            Small(_) => true,
            Large(_) => false,
        }
    }
}

#[cfg(feature = "64_bit_limbs")]
impl<'a> ConvertibleFrom<&'a Natural> for u32 {
    #[inline]
    fn convertible_from(value: &Natural) -> bool {
        match *value {
            Small(small) => u32::convertible_from(small),
            Large(_) => false,
        }
    }
}
