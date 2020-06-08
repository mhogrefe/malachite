use malachite_base::num::basic::integers::PrimitiveInteger;
use malachite_base::num::conversion::traits::{
    CheckedFrom, ConvertibleFrom, FromOtherTypeSlice, OverflowingFrom, SaturatingFrom, WrappingFrom,
};

use natural::InnerNatural::{Large, Small};
use natural::Natural;
use platform::Limb;

macro_rules! impl_from_limb {
    ($u: ident, $s: ident) => {
        impl CheckedFrom<Natural> for $u {
            /// Converts a `Natural` to a `Limb`, taking the `Natural` by value and returning `None`
            /// if the `Natural` is too large.
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
            /// use malachite_nz::natural::Natural;
            ///
            /// assert_eq!(u32::checked_from(Natural::from(123u32)), Some(123));
            /// assert_eq!(u32::checked_from(Natural::trillion()), None);
            /// ```
            #[inline]
            fn checked_from(value: Natural) -> Option<$u> {
                $u::checked_from(&value)
            }
        }

        impl<'a> CheckedFrom<&'a Natural> for $u {
            /// Converts a `Natural` to a `Limb`, taking the `Natural` by reference and returning
            /// `None` if the `Natural` is too large.
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
            /// use malachite_nz::natural::Natural;
            ///
            /// assert_eq!(u32::checked_from(&Natural::from(123u32)), Some(123));
            /// assert_eq!(u32::checked_from(&Natural::trillion()), None);
            /// ```
            fn checked_from(value: &Natural) -> Option<$u> {
                match *value {
                    Natural(Small(small)) => Some(small),
                    Natural(Large(_)) => None,
                }
            }
        }

        impl WrappingFrom<Natural> for $u {
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
            /// use malachite_base::num::conversion::traits::WrappingFrom;
            /// use malachite_nz::natural::Natural;
            ///
            /// assert_eq!(u32::wrapping_from(Natural::from(123u32)), 123);
            /// assert_eq!(u32::wrapping_from(Natural::trillion()), 3_567_587_328);
            /// ```
            #[inline]
            fn wrapping_from(value: Natural) -> $u {
                $u::wrapping_from(&value)
            }
        }

        impl<'a> WrappingFrom<&'a Natural> for $u {
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
            /// use malachite_base::num::conversion::traits::WrappingFrom;
            /// use malachite_nz::natural::Natural;
            ///
            /// assert_eq!(u32::wrapping_from(&Natural::from(123u32)), 123);
            /// assert_eq!(u32::wrapping_from(&Natural::trillion()), 3_567_587_328);
            /// ```
            fn wrapping_from(value: &Natural) -> $u {
                match *value {
                    Natural(Small(small)) => small,
                    Natural(Large(ref limbs)) => limbs[0],
                }
            }
        }

        impl SaturatingFrom<Natural> for $u {
            /// Converts a `Natural` to a `Limb`, taking the `Natural` by value. If the `Natural` is
            /// too large to fit in a `Limb`, `Limb::MAX` is returned.
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
            /// use malachite_nz::natural::Natural;
            ///
            /// assert_eq!(u32::saturating_from(Natural::from(123u32)), 123);
            /// assert_eq!(u32::saturating_from(Natural::trillion()), u32::MAX);
            /// ```
            #[inline]
            fn saturating_from(value: Natural) -> $u {
                $u::saturating_from(&value)
            }
        }

        impl<'a> SaturatingFrom<&'a Natural> for $u {
            /// Converts a `Natural` to a `Limb`, taking the `Natural` by reference. If the
            /// `Natural` is too large to fit in a `Limb`, `Limb::MAX` is returned.
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
            /// use malachite_nz::natural::Natural;
            ///
            /// assert_eq!(u32::saturating_from(&Natural::from(123u32)), 123);
            /// assert_eq!(u32::saturating_from(&Natural::trillion()), u32::MAX);
            /// ```
            fn saturating_from(value: &Natural) -> $u {
                match *value {
                    Natural(Small(small)) => small,
                    Natural(Large(_)) => $u::MAX,
                }
            }
        }

        impl OverflowingFrom<Natural> for $u {
            /// Converts a `Natural` to a `Limb`, taking the `Natural` by reference and wrapping mod
            /// 2<sup>`Limb::WIDTH`</sup>. The returned boolean value indicates whether wrapping
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
            /// use malachite_base::num::conversion::traits::OverflowingFrom;
            /// use malachite_nz::natural::Natural;
            ///
            /// assert_eq!(u32::overflowing_from(Natural::from(123u32)), (123, false));
            /// assert_eq!(u32::overflowing_from(Natural::trillion()), (3_567_587_328, true));
            /// ```
            #[inline]
            fn overflowing_from(value: Natural) -> ($u, bool) {
                $u::overflowing_from(&value)
            }
        }

        impl<'a> OverflowingFrom<&'a Natural> for $u {
            /// Converts a `Natural` to a `Limb`, taking the `Natural` by reference and wrapping mod
            /// 2<sup>`Limb::WIDTH`</sup>. The returned boolean value indicates whether wrapping
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
            /// use malachite_base::num::conversion::traits::OverflowingFrom;
            /// use malachite_nz::natural::Natural;
            ///
            /// assert_eq!(u32::overflowing_from(&Natural::from(123u32)), (123, false));
            /// assert_eq!(u32::overflowing_from(&Natural::trillion()), (3_567_587_328, true));
            /// ```
            fn overflowing_from(value: &Natural) -> ($u, bool) {
                match *value {
                    Natural(Small(small)) => (small, false),
                    Natural(Large(ref limbs)) => (limbs[0], true),
                }
            }
        }

        impl ConvertibleFrom<Natural> for $u {
            /// Determines whether a `Natural` can be converted to a `Limb`. Takes the `Natural` by
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
            /// use malachite_nz::natural::Natural;
            ///
            /// assert_eq!(u32::convertible_from(Natural::from(123u32)), true);
            /// assert_eq!(u32::convertible_from(Natural::trillion()), false);
            /// ```
            #[inline]
            fn convertible_from(value: Natural) -> bool {
                $u::convertible_from(&value)
            }
        }

        impl<'a> ConvertibleFrom<&'a Natural> for $u {
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
            /// use malachite_base::num::conversion::traits::ConvertibleFrom;
            /// use malachite_nz::natural::Natural;
            ///
            /// assert_eq!(u32::convertible_from(&Natural::from(123u32)), true);
            /// assert_eq!(u32::convertible_from(&Natural::trillion()), false);
            /// ```
            fn convertible_from(value: &Natural) -> bool {
                match *value {
                    Natural(Small(_)) => true,
                    Natural(Large(_)) => false,
                }
            }
        }

        impl CheckedFrom<Natural> for $s {
            /// Converts a `Natural` to a `SignedLimb`, taking the `Natural` by value and returning
            /// `None` if the `Natural` is too large.
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
            /// use malachite_nz::natural::Natural;
            ///
            /// assert_eq!(i32::checked_from(Natural::from(123u32)), Some(123));
            /// assert_eq!(i32::checked_from(Natural::trillion()), None);
            /// ```
            #[inline]
            fn checked_from(value: Natural) -> Option<$s> {
                $s::checked_from(&value)
            }
        }

        impl<'a> CheckedFrom<&'a Natural> for $s {
            /// Converts a `Natural` to a `SignedLimb`, taking the `Natural` by reference and
            /// returning `None` if the `Natural` is too large.
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
            /// use malachite_nz::natural::Natural;
            ///
            /// assert_eq!(i32::checked_from(&Natural::from(123u32)), Some(123));
            /// assert_eq!(i32::checked_from(&Natural::trillion()), None);
            /// ```
            fn checked_from(value: &Natural) -> Option<$s> {
                match *value {
                    Natural(Small(small)) => $s::checked_from(small),
                    Natural(Large(_)) => None,
                }
            }
        }

        impl WrappingFrom<Natural> for $s {
            /// Converts a `Natural` to a `SignedLimb`, taking the `Natural` by value and wrapping
            /// mod 2<sup>`Limb::WIDTH`</sup>.
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
            /// use malachite_nz::natural::Natural;
            ///
            /// assert_eq!(i32::wrapping_from(Natural::from(123u32)), 123);
            /// assert_eq!(i32::wrapping_from(Natural::trillion()), -727_379_968);
            /// ```
            #[inline]
            fn wrapping_from(value: Natural) -> $s {
                $s::wrapping_from($u::wrapping_from(value))
            }
        }

        impl<'a> WrappingFrom<&'a Natural> for $s {
            /// Converts a `Natural` to a `SignedLimb`, taking the `Natural` by reference and
            /// wrapping mod 2<sup>`Limb::WIDTH`</sup>.
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
            /// use malachite_nz::natural::Natural;
            ///
            /// assert_eq!(i32::wrapping_from(&Natural::from(123u32)), 123);
            /// assert_eq!(i32::wrapping_from(&Natural::trillion()), -727_379_968);
            /// ```
            #[inline]
            fn wrapping_from(value: &Natural) -> $s {
                $s::wrapping_from($u::wrapping_from(value))
            }
        }

        impl SaturatingFrom<Natural> for $s {
            /// Converts a `Natural` to a `SignedLimb`, taking the `Natural` by value. If the
            /// `Natural` is too large to fit in a `SignedLimb`, `SignedLimb::MAX` is returned.
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
            /// use malachite_nz::natural::Natural;
            ///
            /// assert_eq!(i32::saturating_from(Natural::from(123u32)), 123);
            /// assert_eq!(i32::saturating_from(Natural::trillion()), 2_147_483_647);
            /// ```
            #[inline]
            fn saturating_from(value: Natural) -> $s {
                $s::saturating_from($u::saturating_from(value))
            }
        }

        impl<'a> SaturatingFrom<&'a Natural> for $s {
            /// Converts a `Natural` to a `SignedLimb`, taking the `Natural` by reference. If the
            /// `Natural` is too large to fit in a `SignedLimb`, `SignedLimb::MAX` is returned.
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
            /// use malachite_nz::natural::Natural;
            ///
            /// assert_eq!(i32::saturating_from(&Natural::from(123u32)), 123);
            /// assert_eq!(i32::saturating_from(&Natural::trillion()), 2_147_483_647);
            /// ```
            #[inline]
            fn saturating_from(value: &Natural) -> $s {
                $s::saturating_from($u::saturating_from(value))
            }
        }

        impl OverflowingFrom<Natural> for $s {
            /// Converts a `Natural` to a `SignedLimb`, taking the `Natural` by reference and
            /// wrapping mod 2<sup>`Limb::WIDTH`</sup>. The returned boolean value indicates whether
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
            /// use malachite_base::num::conversion::traits::OverflowingFrom;
            /// use malachite_nz::natural::Natural;
            ///
            /// assert_eq!(i32::overflowing_from(Natural::from(123u32)), (123, false));
            /// assert_eq!(i32::overflowing_from(Natural::trillion()), (-727_379_968, true));
            /// ```
            #[inline]
            fn overflowing_from(value: Natural) -> ($s, bool) {
                $s::overflowing_from(&value)
            }
        }

        impl<'a> OverflowingFrom<&'a Natural> for $s {
            /// Converts a `Natural` to a `SignedLimb`, taking the `Natural` by reference and
            /// wrapping mod 2<sup>`Limb::WIDTH`</sup>. The returned boolean value indicates whether
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
            /// use malachite_base::num::conversion::traits::OverflowingFrom;
            /// use malachite_nz::natural::Natural;
            ///
            /// assert_eq!(i32::overflowing_from(&Natural::from(123u32)), (123, false));
            /// assert_eq!(i32::overflowing_from(&Natural::trillion()), (-727_379_968, true));
            /// ```
            fn overflowing_from(value: &Natural) -> ($s, bool) {
                let (result, overflow_1) = $u::overflowing_from(value);
                let (result, overflow_2) = $s::overflowing_from(result);
                (result, overflow_1 || overflow_2)
            }
        }

        impl ConvertibleFrom<Natural> for $s {
            /// Determines whether a `Natural` can be converted to a `SignedLimb`. Takes the
            /// `Natural` by value.
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
            /// use malachite_nz::natural::Natural;
            ///
            /// assert_eq!(i32::convertible_from(Natural::from(123u32)), true);
            /// assert_eq!(i32::convertible_from(Natural::trillion()), false);
            /// ```
            #[inline]
            fn convertible_from(value: Natural) -> bool {
                $s::convertible_from(&value)
            }
        }

        impl<'a> ConvertibleFrom<&'a Natural> for $s {
            /// Determines whether a `Natural` can be converted to a `SignedLimb`. Takes the
            /// `Natural` by reference.
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
            /// use malachite_nz::natural::Natural;
            ///
            /// assert_eq!(i32::convertible_from(&Natural::from(123u32)), true);
            /// assert_eq!(i32::convertible_from(&Natural::trillion()), false);
            /// ```
            fn convertible_from(value: &Natural) -> bool {
                match *value {
                    Natural(Small(small)) => $s::convertible_from(small),
                    Natural(Large(_)) => false,
                }
            }
        }
    };
}

macro_rules! impl_from_smaller_than_limb {
    ($u: ident, $s: ident) => {
        impl CheckedFrom<Natural> for $u {
            /// Converts a `Natural` to a value of a primitive unsigned integer type that's smaller
            /// than a `Limb`, taking the `Natural` by value and returning `None` if the `Natural`
            /// is too large.
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
            /// use malachite_nz::natural::Natural;
            ///
            /// assert_eq!(u8::checked_from(Natural::from(123u32)), Some(123));
            /// assert_eq!(u8::checked_from(Natural::trillion()), None);
            /// ```
            #[inline]
            fn checked_from(value: Natural) -> Option<$u> {
                Limb::checked_from(value).and_then($u::checked_from)
            }
        }

        impl<'a> CheckedFrom<&'a Natural> for $u {
            /// Converts a `Natural` to a value of a primitive unsigned integer type that's smaller
            /// than a `Limb`, taking the `Natural` by reference and returning `None` if the
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
            /// use malachite_base::num::conversion::traits::CheckedFrom;
            /// use malachite_nz::natural::Natural;
            ///
            /// assert_eq!(u8::checked_from(&Natural::from(123u32)), Some(123));
            /// assert_eq!(u8::checked_from(&Natural::trillion()), None);
            /// ```
            #[inline]
            fn checked_from(value: &Natural) -> Option<$u> {
                Limb::checked_from(value).and_then($u::checked_from)
            }
        }

        impl WrappingFrom<Natural> for $u {
            /// Converts a `Natural` to a value of a primitive unsigned integer type that's smaller
            /// than a `Limb`, taking the `Natural` by value and wrapping mod
            /// 2<sup>`$u::WIDTH`</sup>.
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
            /// use malachite_nz::natural::Natural;
            ///
            /// assert_eq!(u8::wrapping_from(Natural::from(123u32)), 123);
            /// assert_eq!(u8::wrapping_from(Natural::trillion()), 0);
            /// ```
            #[inline]
            fn wrapping_from(value: Natural) -> $u {
                $u::wrapping_from(Limb::wrapping_from(value))
            }
        }

        impl<'a> WrappingFrom<&'a Natural> for $u {
            /// Converts a `Natural` to a value of a primitive unsigned integer type that's smaller
            /// than a `Limb`, taking the `Natural` by reference and wrapping mod
            /// 2<sup>`$u::WIDTH`</sup>.
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
            /// use malachite_nz::natural::Natural;
            ///
            /// assert_eq!(u8::wrapping_from(&Natural::from(123u32)), 123);
            /// assert_eq!(u8::wrapping_from(&Natural::trillion()), 0);
            /// ```
            #[inline]
            fn wrapping_from(value: &Natural) -> $u {
                $u::wrapping_from(Limb::wrapping_from(value))
            }
        }

        impl SaturatingFrom<Natural> for $u {
            /// Converts a `Natural` to a value of a primitive unsigned integer type that's smaller
            /// than a `Limb`, taking the `Natural` by value. If the `Natural` is too large to fit
            /// in `$u`, `$u::MAX` is returned.
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
            /// use malachite_nz::natural::Natural;
            ///
            /// assert_eq!(u8::saturating_from(Natural::from(123u32)), 123);
            /// assert_eq!(u8::saturating_from(Natural::trillion()), 255);
            /// ```
            #[inline]
            fn saturating_from(value: Natural) -> $u {
                $u::saturating_from(Limb::saturating_from(value))
            }
        }

        impl<'a> SaturatingFrom<&'a Natural> for $u {
            /// Converts a `Natural` to a value of a primitive unsigned integer type that's smaller
            /// than a `Limb`, taking the `Natural` by reference. If the `Natural` is too large to
            /// fit in `$u`, `$u::MAX` is returned.
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
            /// use malachite_nz::natural::Natural;
            ///
            /// assert_eq!(u8::saturating_from(&Natural::from(123u32)), 123);
            /// assert_eq!(u8::saturating_from(&Natural::trillion()), 255);
            /// ```
            #[inline]
            fn saturating_from(value: &Natural) -> $u {
                $u::saturating_from(Limb::saturating_from(value))
            }
        }

        impl OverflowingFrom<Natural> for $u {
            /// Converts a `Natural` to a value of a primitive unsigned integer type that's smaller
            /// than a `Limb`, taking the `Natural` by value and wrapping mod
            /// 2<sup>`$u::WIDTH`</sup>. The returned boolean value indicates whether wrapping
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
            /// use malachite_base::num::conversion::traits::OverflowingFrom;
            /// use malachite_nz::natural::Natural;
            ///
            /// assert_eq!(u8::overflowing_from(Natural::from(123u32)), (123, false));
            /// assert_eq!(u8::overflowing_from(Natural::trillion()), (0, true));
            /// ```
            #[inline]
            fn overflowing_from(value: Natural) -> ($u, bool) {
                let (result, overflow_1) = Limb::overflowing_from(value);
                let (result, overflow_2) = $u::overflowing_from(result);
                (result, overflow_1 || overflow_2)
            }
        }

        impl<'a> OverflowingFrom<&'a Natural> for $u {
            /// Converts a `Natural` to a value of a primitive unsigned integer type that's smaller
            /// than a `Limb`, taking the `Natural` by reference and wrapping mod
            /// 2<sup>`$u::WIDTH`</sup>. The returned boolean value indicates whether wrapping
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
            /// use malachite_base::num::conversion::traits::OverflowingFrom;
            /// use malachite_nz::natural::Natural;
            ///
            /// assert_eq!(u8::overflowing_from(&Natural::from(123u32)), (123, false));
            /// assert_eq!(u8::overflowing_from(&Natural::trillion()), (0, true));
            /// ```
            #[inline]
            fn overflowing_from(value: &Natural) -> ($u, bool) {
                let (result, overflow_1) = Limb::overflowing_from(value);
                let (result, overflow_2) = $u::overflowing_from(result);
                (result, overflow_1 || overflow_2)
            }
        }

        impl ConvertibleFrom<Natural> for $u {
            /// Determines whether a `Natural` can be converted to a value of a primitive unsigned
            /// integer type that's smaller than a `Limb`. Takes the `Natural` by value.
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
            /// use malachite_nz::natural::Natural;
            ///
            /// assert_eq!(u8::convertible_from(Natural::from(123u32)), true);
            /// assert_eq!(u8::convertible_from(Natural::trillion()), false);
            /// ```
            #[inline]
            fn convertible_from(value: Natural) -> bool {
                $u::convertible_from(&value)
            }
        }

        impl<'a> ConvertibleFrom<&'a Natural> for $u {
            /// Determines whether a `Natural` can be converted to a value of a primitive unsigned
            /// integer type that's smaller than a `Limb`. Takes the `Natural` by reference.
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
            /// use malachite_nz::natural::Natural;
            ///
            /// assert_eq!(u8::convertible_from(&Natural::from(123u32)), true);
            /// assert_eq!(u8::convertible_from(&Natural::trillion()), false);
            /// ```
            fn convertible_from(value: &Natural) -> bool {
                match *value {
                    Natural(Small(small)) => $u::convertible_from(small),
                    Natural(Large(_)) => false,
                }
            }
        }

        impl CheckedFrom<Natural> for $s {
            /// Converts a `Natural` to a value of a primitive signed integer type that's smaller
            /// than a `Limb`, taking the `Natural` by value and returning `None` if the `Natural`
            /// is too large.
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
            /// use malachite_nz::natural::Natural;
            ///
            /// assert_eq!(i8::checked_from(Natural::from(123u32)), Some(123));
            /// assert_eq!(i8::checked_from(Natural::trillion()), None);
            /// ```
            #[inline]
            fn checked_from(value: Natural) -> Option<$s> {
                Limb::checked_from(value).and_then($s::checked_from)
            }
        }

        impl<'a> CheckedFrom<&'a Natural> for $s {
            /// Converts a `Natural` to a value of a primitive signed integer type that's smaller
            /// than a `Limb`, taking the `Natural` by reference and returning `None` if the
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
            /// use malachite_base::num::conversion::traits::CheckedFrom;
            /// use malachite_nz::natural::Natural;
            ///
            /// assert_eq!(i8::checked_from(&Natural::from(123u32)), Some(123));
            /// assert_eq!(i8::checked_from(&Natural::trillion()), None);
            /// ```
            #[inline]
            fn checked_from(value: &Natural) -> Option<$s> {
                Limb::checked_from(value).and_then($s::checked_from)
            }
        }

        impl WrappingFrom<Natural> for $s {
            /// Converts a `Natural` to a value of a primitive signed integer type that's smaller
            /// than a `Limb`, taking the `Natural` by value and wrapping mod
            /// 2<sup>`$u::WIDTH`</sup>.
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
            /// use malachite_nz::natural::Natural;
            ///
            /// assert_eq!(i8::wrapping_from(Natural::from(123u32)), 123);
            /// assert_eq!(i8::wrapping_from(Natural::trillion()), 0);
            /// ```
            #[inline]
            fn wrapping_from(value: Natural) -> $s {
                $s::wrapping_from(Limb::wrapping_from(value))
            }
        }

        impl<'a> WrappingFrom<&'a Natural> for $s {
            /// Converts a `Natural` to a value of a primitive signed integer type that's smaller
            /// than a `Limb`, taking the `Natural` by reference and wrapping mod
            /// 2<sup>`$u::WIDTH`</sup>.
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
            /// use malachite_nz::natural::Natural;
            ///
            /// assert_eq!(i8::wrapping_from(&Natural::from(123u32)), 123);
            /// assert_eq!(i8::wrapping_from(&Natural::trillion()), 0);
            /// ```
            #[inline]
            fn wrapping_from(value: &Natural) -> $s {
                $s::wrapping_from(Limb::wrapping_from(value))
            }
        }

        impl SaturatingFrom<Natural> for $s {
            /// Converts a `Natural` to a value of a primitive signed integer type that's smaller
            /// than a `Limb`, taking the `Natural` by value. If the `Natural` is too large to fit
            /// in `$u`, `$u::MAX` is returned.
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
            /// use malachite_nz::natural::Natural;
            ///
            /// assert_eq!(i8::saturating_from(Natural::from(123u32)), 123);
            /// assert_eq!(i8::saturating_from(Natural::trillion()), 127);
            /// ```
            #[inline]
            fn saturating_from(value: Natural) -> $s {
                $s::saturating_from(Limb::saturating_from(value))
            }
        }

        impl<'a> SaturatingFrom<&'a Natural> for $s {
            /// Converts a `Natural` to a value of a primitive unsigned integer type that's smaller
            /// than a `Limb`, taking the `Natural` by reference. If the `Natural` is too large to
            /// fit in `$u`, `$u::MAX` is returned.
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
            /// use malachite_nz::natural::Natural;
            ///
            /// assert_eq!(i8::saturating_from(&Natural::from(123u32)), 123);
            /// assert_eq!(i8::saturating_from(&Natural::trillion()), 127);
            /// ```
            #[inline]
            fn saturating_from(value: &Natural) -> $s {
                $s::saturating_from(Limb::saturating_from(value))
            }
        }

        impl OverflowingFrom<Natural> for $s {
            /// Converts a `Natural` to a value of a primitive signed integer type that's smaller
            /// than a `Limb`, taking the `Natural` by value and wrapping mod
            /// 2<sup>`$u::WIDTH`</sup>. The returned boolean value indicates whether wrapping
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
            /// use malachite_base::num::conversion::traits::OverflowingFrom;
            /// use malachite_nz::natural::Natural;
            ///
            /// assert_eq!(i8::overflowing_from(Natural::from(123u32)), (123, false));
            /// assert_eq!(i8::overflowing_from(Natural::trillion()), (0, true));
            /// ```
            #[inline]
            fn overflowing_from(value: Natural) -> ($s, bool) {
                let (result, overflow_1) = Limb::overflowing_from(value);
                let (result, overflow_2) = $s::overflowing_from(result);
                (result, overflow_1 || overflow_2)
            }
        }

        impl<'a> OverflowingFrom<&'a Natural> for $s {
            /// Converts a `Natural` to a value of a primitive signed integer type that's smaller
            /// than a `Limb`, taking the `Natural` by reference and wrapping mod
            /// 2<sup>`$u::WIDTH`</sup>. The returned boolean value indicates whether wrapping
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
            /// use malachite_base::num::conversion::traits::OverflowingFrom;
            /// use malachite_nz::natural::Natural;
            ///
            /// assert_eq!(i8::overflowing_from(&Natural::from(123u32)), (123, false));
            /// assert_eq!(i8::overflowing_from(&Natural::trillion()), (0, true));
            /// ```
            #[inline]
            fn overflowing_from(value: &Natural) -> ($s, bool) {
                let (result, overflow_1) = Limb::overflowing_from(value);
                let (result, overflow_2) = $s::overflowing_from(result);
                (result, overflow_1 || overflow_2)
            }
        }

        impl ConvertibleFrom<Natural> for $s {
            /// Determines whether a `Natural` can be converted to a value of a primitive signed
            /// integer type that's smaller than a `Limb`. Takes the `Natural` by value.
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
            /// use malachite_nz::natural::Natural;
            ///
            /// assert_eq!(i8::convertible_from(Natural::from(123u32)), true);
            /// assert_eq!(i8::convertible_from(Natural::trillion()), false);
            /// ```
            #[inline]
            fn convertible_from(value: Natural) -> bool {
                $s::convertible_from(&value)
            }
        }

        impl<'a> ConvertibleFrom<&'a Natural> for $s {
            /// Determines whether a `Natural` can be converted to a value of a primitive signed
            /// integer type that's smaller than a `Limb`. Takes the `Natural` by reference.
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
            /// use malachite_nz::natural::Natural;
            ///
            /// assert_eq!(i8::convertible_from(&Natural::from(123u32)), true);
            /// assert_eq!(i8::convertible_from(&Natural::trillion()), false);
            /// ```
            fn convertible_from(value: &Natural) -> bool {
                match *value {
                    Natural(Small(small)) => $s::convertible_from(small),
                    Natural(Large(_)) => false,
                }
            }
        }
    };
}

macro_rules! impl_from_larger_than_limb_or_xsize {
    ($u: ident, $s: ident) => {
        impl CheckedFrom<Natural> for $u {
            /// Converts a `Natural` to a `usize` or a value of a primitive unsigned integer type
            /// that's larger than a `Limb`, taking the `Natural` by value and returning `None` if
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
            /// use malachite_base::num::basic::traits::One;
            /// use malachite_base::num::conversion::traits::CheckedFrom;
            /// use malachite_nz::natural::Natural;
            ///
            /// assert_eq!(u64::checked_from(Natural::from(123u32)), Some(123));
            /// assert_eq!(u64::checked_from(Natural::ONE << 100), None);
            /// ```
            #[inline]
            fn checked_from(value: Natural) -> Option<$u> {
                $u::checked_from(&value)
            }
        }

        impl WrappingFrom<Natural> for $u {
            /// Converts a `Natural` to a `usize` or a value of a primitive unsigned integer type
            /// that's larger than a `Limb`, taking the `Natural` by value and wrapping mod
            /// 2<sup>`$u::WIDTH`</sup>.
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
            /// use malachite_base::num::basic::traits::One;
            /// use malachite_base::num::conversion::traits::WrappingFrom;
            /// use malachite_nz::natural::Natural;
            ///
            /// assert_eq!(u64::wrapping_from(Natural::from(123u32)), 123);
            /// assert_eq!(u64::wrapping_from(Natural::ONE << 100), 0);
            /// ```
            #[inline]
            fn wrapping_from(value: Natural) -> $u {
                $u::wrapping_from(&value)
            }
        }

        impl<'a> WrappingFrom<&'a Natural> for $u {
            /// Converts a `Natural` to a `usize` or a value of a primitive unsigned integer type
            /// that's larger than a `Limb`, taking the `Natural` by reference and wrapping mod
            /// 2<sup>`$u::WIDTH`</sup>.
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
            /// use malachite_base::num::basic::traits::One;
            /// use malachite_base::num::conversion::traits::WrappingFrom;
            /// use malachite_nz::natural::Natural;
            ///
            /// assert_eq!(u64::wrapping_from(&Natural::from(123u32)), 123);
            /// assert_eq!(u64::wrapping_from(&(Natural::ONE << 100)), 0);
            /// ```
            fn wrapping_from(value: &Natural) -> $u {
                match *value {
                    Natural(Small(small)) => $u::wrapping_from(small),
                    Natural(Large(ref limbs)) => $u::from_other_type_slice(limbs),
                }
            }
        }

        impl SaturatingFrom<Natural> for $u {
            /// Converts a `Natural` to a `usize` or a value of a primitive unsigned integer type
            /// that's larger than a `Limb`, taking the `Natural` by value. If the `Natural` is too
            /// large to fit in `$u`, `$u::MAX` is returned.
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
            /// use malachite_base::num::basic::traits::One;
            /// use malachite_base::num::conversion::traits::SaturatingFrom;
            /// use malachite_nz::natural::Natural;
            ///
            /// assert_eq!(u64::saturating_from(Natural::from(123u32)), 123);
            /// assert_eq!(u64::saturating_from(Natural::ONE << 100), 18_446_744_073_709_551_615);
            /// ```
            #[inline]
            fn saturating_from(value: Natural) -> $u {
                $u::saturating_from(&value)
            }
        }

        impl OverflowingFrom<Natural> for $u {
            /// Converts a `Natural` to a `usize` or a value of a primitive unsigned integer type
            /// that's larger than a `Limb`, taking the `Natural` by value and wrapping mod
            /// 2<sup>`$u::WIDTH`</sup>. The returned boolean value indicates whether wrapping
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
            /// use malachite_base::num::basic::traits::One;
            /// use malachite_base::num::conversion::traits::OverflowingFrom;
            /// use malachite_nz::natural::Natural;
            ///
            /// assert_eq!(u64::overflowing_from(Natural::from(123u32)), (123, false));
            /// assert_eq!(u64::overflowing_from(Natural::ONE << 100), (0, true));
            /// ```
            #[inline]
            fn overflowing_from(value: Natural) -> ($u, bool) {
                $u::overflowing_from(&value)
            }
        }

        impl ConvertibleFrom<Natural> for $u {
            /// Determines whether a `Natural` can be converted to a `usize` or a value of a
            /// primitive unsigned integer type that's larger than a `Limb`. Takes the `Natural` by
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
            /// use malachite_base::num::basic::traits::One;
            /// use malachite_base::num::conversion::traits::ConvertibleFrom;
            /// use malachite_nz::natural::Natural;
            ///
            /// assert_eq!(u64::convertible_from(Natural::from(123u32)), true);
            /// assert_eq!(u64::convertible_from(Natural::ONE << 100), false);
            /// ```
            #[inline]
            fn convertible_from(value: Natural) -> bool {
                $u::convertible_from(&value)
            }
        }

        impl CheckedFrom<Natural> for $s {
            /// Converts a `Natural` to an `isize` or a value of a primitive signed integer type
            /// that's larger than a `Limb`, taking the `Natural` by value and returning `None` if
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
            /// use malachite_base::num::basic::traits::One;
            /// use malachite_base::num::conversion::traits::CheckedFrom;
            /// use malachite_nz::natural::Natural;
            ///
            /// assert_eq!(i64::checked_from(Natural::from(123u32)), Some(123));
            /// assert_eq!(i64::checked_from(Natural::ONE << 100), None);
            /// ```
            #[inline]
            fn checked_from(value: Natural) -> Option<$s> {
                $u::checked_from(&value).and_then($s::checked_from)
            }
        }

        impl<'a> CheckedFrom<&'a Natural> for $s {
            /// Converts a `Natural` to an `isize` or value of a primitive signed integer type
            /// that's larger than a `Limb`, taking the `Natural` by reference and returning `None`
            /// if the `Natural` is too large.
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
            /// use malachite_base::num::basic::traits::One;
            /// use malachite_base::num::conversion::traits::CheckedFrom;
            /// use malachite_nz::natural::Natural;
            ///
            /// assert_eq!(i64::checked_from(&Natural::from(123u32)), Some(123));
            /// assert_eq!(i64::checked_from(&(Natural::ONE << 100)), None);
            /// ```
            #[inline]
            fn checked_from(value: &Natural) -> Option<$s> {
                $u::checked_from(value).and_then($s::checked_from)
            }
        }

        impl WrappingFrom<Natural> for $s {
            /// Converts a `Natural` to an `isize` or a value of a primitive signed integer type
            /// that's larger than a `Limb`, taking the `Natural` by value and wrapping mod
            /// 2<sup>`$u::WIDTH`</sup>.
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
            /// use malachite_base::num::basic::traits::One;
            /// use malachite_base::num::conversion::traits::WrappingFrom;
            /// use malachite_nz::natural::Natural;
            ///
            /// assert_eq!(i64::wrapping_from(Natural::from(123u32)), 123);
            /// assert_eq!(i64::wrapping_from(Natural::ONE << 100), 0);
            /// ```
            #[inline]
            fn wrapping_from(value: Natural) -> $s {
                $s::wrapping_from($u::wrapping_from(&value))
            }
        }

        impl<'a> WrappingFrom<&'a Natural> for $s {
            /// Converts a `Natural` to an `isize` or a value of a primitive signed integer type
            /// that's larger than a `Limb`, taking the `Natural` by reference and wrapping mod
            /// 2<sup>`$u::WIDTH`</sup>.
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
            /// use malachite_base::num::basic::traits::One;
            /// use malachite_base::num::conversion::traits::WrappingFrom;
            /// use malachite_nz::natural::Natural;
            ///
            /// assert_eq!(i64::wrapping_from(&Natural::from(123u32)), 123);
            /// assert_eq!(i64::wrapping_from(&(Natural::ONE << 100)), 0);
            /// ```
            #[inline]
            fn wrapping_from(value: &Natural) -> $s {
                $s::wrapping_from($u::wrapping_from(value))
            }
        }

        impl SaturatingFrom<Natural> for $s {
            /// Converts a `Natural` to an `isize` or a value of a primitive signed integer type
            /// that's larger than a `Limb`, taking the `Natural` by value. If the `Natural` is too
            /// large to fit in `$s`, `$s::MAX` is returned.
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
            /// use malachite_base::num::basic::traits::One;
            /// use malachite_base::num::conversion::traits::SaturatingFrom;
            /// use malachite_nz::natural::Natural;
            ///
            /// assert_eq!(i64::saturating_from(Natural::from(123u32)), 123);
            /// assert_eq!(i64::saturating_from(Natural::ONE << 100), 9_223_372_036_854_775_807);
            /// ```
            #[inline]
            fn saturating_from(value: Natural) -> $s {
                $s::saturating_from($u::saturating_from(&value))
            }
        }

        impl<'a> SaturatingFrom<&'a Natural> for $s {
            /// Converts a `Natural` to an `isize` or a value of a primitive signed integer type
            /// that's larger than a `Limb`, taking the `Natural` by reference. If the `Natural` is
            /// too large to fit in `$s`, `$s::MAX` is returned.
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
            /// use malachite_base::num::basic::traits::One;
            /// use malachite_base::num::conversion::traits::SaturatingFrom;
            /// use malachite_nz::natural::Natural;
            ///
            /// assert_eq!(i64::saturating_from(&Natural::from(123u32)), 123);
            /// assert_eq!(i64::saturating_from(&(Natural::ONE << 100)), 9_223_372_036_854_775_807);
            /// ```
            #[inline]
            fn saturating_from(value: &Natural) -> $s {
                $s::saturating_from($u::saturating_from(value))
            }
        }

        impl OverflowingFrom<Natural> for $s {
            /// Converts a `Natural` to an `isize` or a value of a primitive signed integer type
            /// that's larger than a `Limb`, taking the `Natural` by value and wrapping mod
            /// 2<sup>`$u::WIDTH`</sup>. The returned boolean value indicates whether wrapping
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
            /// use malachite_base::num::basic::traits::One;
            /// use malachite_base::num::conversion::traits::OverflowingFrom;
            /// use malachite_nz::natural::Natural;
            ///
            /// assert_eq!(i64::overflowing_from(Natural::from(123u32)), (123, false));
            /// assert_eq!(i64::overflowing_from(Natural::ONE << 100), (0, true));
            /// ```
            #[inline]
            fn overflowing_from(value: Natural) -> ($s, bool) {
                $s::overflowing_from(&value)
            }
        }

        impl<'a> OverflowingFrom<&'a Natural> for $s {
            /// Converts a `Natural` to an `isize` or a value of a primitive signed integer type
            /// that's larger than a `Limb`, taking the `Natural` by reference and wrapping mod
            /// 2<sup>`$u::WIDTH`</sup>. The returned boolean value indicates whether wrapping
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
            /// use malachite_base::num::basic::traits::One;
            /// use malachite_base::num::conversion::traits::OverflowingFrom;
            /// use malachite_nz::natural::Natural;
            ///
            /// assert_eq!(i64::overflowing_from(&Natural::from(123u32)), (123, false));
            /// assert_eq!(i64::overflowing_from(&(Natural::ONE << 100)), (0, true));
            /// ```
            fn overflowing_from(value: &Natural) -> ($s, bool) {
                let (result, overflow_1) = $u::overflowing_from(value);
                let (result, overflow_2) = $s::overflowing_from(result);
                (result, overflow_1 || overflow_2)
            }
        }

        impl ConvertibleFrom<Natural> for $s {
            /// Determines whether a `Natural` can be converted to an `isize` or a value of a
            /// primitive signed integer type that's larger than a `Limb`. Takes the `Natural` by
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
            /// use malachite_base::num::basic::traits::One;
            /// use malachite_base::num::conversion::traits::ConvertibleFrom;
            /// use malachite_nz::natural::Natural;
            ///
            /// assert_eq!(i64::convertible_from(Natural::from(123u32)), true);
            /// assert_eq!(i64::convertible_from(Natural::ONE << 100), false);
            /// ```
            #[inline]
            fn convertible_from(value: Natural) -> bool {
                $s::convertible_from(&value)
            }
        }
    };
}

macro_rules! impl_from_larger_than_limb {
    ($u: ident, $s: ident) => {
        impl_from_larger_than_limb_or_xsize!($u, $s);

        impl<'a> CheckedFrom<&'a Natural> for $u {
            /// Converts a `Natural` to a value of a primitive unsigned integer type that's larger
            /// than a `Limb`, taking the `Natural` by reference and returning `None` if the
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
            /// use malachite_base::num::basic::traits::One;
            /// use malachite_base::num::conversion::traits::CheckedFrom;
            /// use malachite_nz::natural::Natural;
            ///
            /// assert_eq!(u64::checked_from(&Natural::from(123u32)), Some(123));
            /// assert_eq!(u64::checked_from(&(Natural::ONE << 100)), None);
            /// ```
            fn checked_from(value: &Natural) -> Option<$u> {
                const SIZE_RATIO: usize = 1 << ($u::LOG_WIDTH - Limb::LOG_WIDTH);
                match *value {
                    Natural(Small(small)) => Some($u::from(small)),
                    Natural(Large(ref limbs)) if limbs.len() <= SIZE_RATIO => {
                        Some($u::from_other_type_slice(limbs))
                    }
                    Natural(Large(_)) => None,
                }
            }
        }

        impl<'a> SaturatingFrom<&'a Natural> for $u {
            /// Converts a `Natural` to a value of a primitive unsigned integer type that's larger
            /// than a `Limb`, taking the `Natural` by reference. If the `Natural` is too large to
            /// fit in `$u`, `$u::MAX` is returned.
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
            /// use malachite_base::num::basic::traits::One;
            /// use malachite_base::num::conversion::traits::SaturatingFrom;
            /// use malachite_nz::natural::Natural;
            ///
            /// assert_eq!(u64::saturating_from(&Natural::from(123u32)), 123);
            /// assert_eq!(
            ///     u64::saturating_from(&(Natural::ONE << 100)),
            ///     18_446_744_073_709_551_615
            /// );
            /// ```
            fn saturating_from(value: &Natural) -> $u {
                const SIZE_RATIO: usize = 1 << ($u::LOG_WIDTH - Limb::LOG_WIDTH);
                match *value {
                    Natural(Small(small)) => $u::from(small),
                    Natural(Large(ref limbs)) if limbs.len() <= SIZE_RATIO => {
                        $u::from_other_type_slice(limbs)
                    }
                    Natural(Large(_)) => $u::MAX,
                }
            }
        }

        impl<'a> OverflowingFrom<&'a Natural> for $u {
            /// Converts a `Natural` to a value of a primitive unsigned integer type that's larger
            /// than a `Limb`, taking the `Natural` by reference and wrapping mod
            /// 2<sup>`$u::WIDTH`</sup>. The returned boolean value indicates whether wrapping
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
            /// use malachite_base::num::basic::traits::One;
            /// use malachite_base::num::conversion::traits::OverflowingFrom;
            /// use malachite_nz::natural::Natural;
            ///
            /// assert_eq!(u64::overflowing_from(&Natural::from(123u32)), (123, false));
            /// assert_eq!(u64::overflowing_from(&(Natural::ONE << 100)), (0, true));
            /// ```
            fn overflowing_from(value: &Natural) -> ($u, bool) {
                const SIZE_RATIO: usize = 1 << ($u::LOG_WIDTH - Limb::LOG_WIDTH);
                match *value {
                    Natural(Small(small)) => ($u::from(small), false),
                    Natural(Large(ref limbs)) => {
                        ($u::from_other_type_slice(limbs), limbs.len() > SIZE_RATIO)
                    }
                }
            }
        }

        impl<'a> ConvertibleFrom<&'a Natural> for $u {
            /// Determines whether a `Natural` can be converted to a value of a primitive unsigned
            /// integer type that's larger than a `Limb`. Takes the `Natural` by reference.
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
            /// use malachite_base::num::basic::traits::One;
            /// use malachite_base::num::conversion::traits::ConvertibleFrom;
            /// use malachite_nz::natural::Natural;
            ///
            /// assert_eq!(u64::convertible_from(&Natural::from(123u32)), true);
            /// assert_eq!(u64::convertible_from(&(Natural::ONE << 100)), false);
            /// ```
            fn convertible_from(value: &Natural) -> bool {
                const SIZE_RATIO: usize = 1 << ($u::LOG_WIDTH - Limb::LOG_WIDTH);
                match *value {
                    Natural(Small(_)) => true,
                    Natural(Large(ref limbs)) => limbs.len() <= SIZE_RATIO,
                }
            }
        }

        impl<'a> ConvertibleFrom<&'a Natural> for $s {
            /// Determines whether a `Natural` can be converted to a value of a primitive signed
            /// integer type that's larger than a `Limb`. Takes the `Natural` by reference.
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
            /// use malachite_base::num::basic::traits::One;
            /// use malachite_base::num::conversion::traits::ConvertibleFrom;
            /// use malachite_nz::natural::Natural;
            ///
            /// assert_eq!(i64::convertible_from(&Natural::from(123u32)), true);
            /// assert_eq!(i64::convertible_from(&(Natural::ONE << 100)), false);
            /// ```
            fn convertible_from(value: &Natural) -> bool {
                const SIZE_RATIO: usize = 1 << ($u::LOG_WIDTH - Limb::LOG_WIDTH);
                match *value {
                    Natural(Small(_)) => true,
                    Natural(Large(ref limbs)) => {
                        limbs.len() < SIZE_RATIO
                            || limbs.len() == SIZE_RATIO && !limbs[SIZE_RATIO - 1].get_highest_bit()
                    }
                }
            }
        }
    };
}

impl<'a> CheckedFrom<&'a Natural> for usize {
    /// Converts a `Natural` to a `usize`, taking the `Natural` by reference and returning `None` if
    /// the `Natural` is too large.
    ///
    /// Time: worst case O(1)
    ///
    /// Additional memory: worst case O(1)
    fn checked_from(value: &Natural) -> Option<usize> {
        if usize::WIDTH == u32::WIDTH {
            u32::checked_from(value).map(usize::wrapping_from)
        } else {
            assert_eq!(usize::WIDTH, u64::WIDTH);
            u64::checked_from(value).map(usize::wrapping_from)
        }
    }
}

impl<'a> SaturatingFrom<&'a Natural> for usize {
    /// Converts a `Natural` to a `usize`, taking the `Natural` by reference. If the `Natural` is
    /// too large to fit in a `usize`, `usize::MAX` is returned.
    ///
    /// Time: worst case O(1)
    ///
    /// Additional memory: worst case O(1)
    fn saturating_from(value: &Natural) -> usize {
        if usize::WIDTH == u32::WIDTH {
            usize::wrapping_from(u32::saturating_from(value))
        } else {
            assert_eq!(usize::WIDTH, u64::WIDTH);
            usize::wrapping_from(u64::saturating_from(value))
        }
    }
}

impl<'a> OverflowingFrom<&'a Natural> for usize {
    /// Converts a `Natural` to a `usize`, taking the `Natural` by reference and wrapping mod
    /// 2<sup>`usize::WIDTH`</sup>. The returned boolean value indicates whether wrapping occurred.
    ///
    /// Time: worst case O(1)
    ///
    /// Additional memory: worst case O(1)
    fn overflowing_from(value: &Natural) -> (usize, bool) {
        if usize::WIDTH == u32::WIDTH {
            let (result, overflow) = u32::overflowing_from(value);
            (usize::wrapping_from(result), overflow)
        } else {
            assert_eq!(usize::WIDTH, u64::WIDTH);
            let (result, overflow) = u64::overflowing_from(value);
            (usize::wrapping_from(result), overflow)
        }
    }
}

impl<'a> ConvertibleFrom<&'a Natural> for usize {
    /// Determines whether a `Natural` can be converted to a `usize`. Takes the `Natural` by
    /// reference.
    ///
    /// Time: worst case O(1)
    ///
    /// Additional memory: worst case O(1)
    fn convertible_from(value: &Natural) -> bool {
        if usize::WIDTH == u32::WIDTH {
            u32::convertible_from(value)
        } else {
            assert_eq!(usize::WIDTH, u64::WIDTH);
            u64::convertible_from(value)
        }
    }
}

impl<'a> ConvertibleFrom<&'a Natural> for isize {
    /// Determines whether a `Natural` can be converted to an `isize`. Takes the `Natural` by
    /// reference.
    ///
    /// Time: worst case O(1)
    ///
    /// Additional memory: worst case O(1)
    fn convertible_from(value: &Natural) -> bool {
        if usize::WIDTH == u32::WIDTH {
            i32::convertible_from(value)
        } else {
            assert_eq!(usize::WIDTH, u64::WIDTH);
            i64::convertible_from(value)
        }
    }
}

impl_from_smaller_than_limb!(u8, i8);
impl_from_smaller_than_limb!(u16, i16);
#[cfg(feature = "32_bit_limbs")]
impl_from_limb!(u32, i32);
#[cfg(not(feature = "32_bit_limbs"))]
impl_from_smaller_than_limb!(u32, i32);
#[cfg(feature = "32_bit_limbs")]
impl_from_larger_than_limb!(u64, i64);
#[cfg(not(feature = "32_bit_limbs"))]
impl_from_limb!(u64, i64);
impl_from_larger_than_limb!(u128, i128);
impl_from_larger_than_limb_or_xsize!(usize, isize);
