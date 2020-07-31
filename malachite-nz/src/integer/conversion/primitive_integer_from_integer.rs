use malachite_base::num::arithmetic::traits::DivisibleByPowerOfTwo;
use malachite_base::num::basic::integers::PrimitiveInteger;
use malachite_base::num::conversion::traits::{
    CheckedFrom, ConvertibleFrom, OverflowingFrom, SaturatingFrom, WrappingFrom,
};
use malachite_base::num::logic::traits::SignificantBits;

use integer::Integer;

macro_rules! impl_from {
    ($u: ident, $s: ident) => {
        impl CheckedFrom<Integer> for $u {
            /// Converts an `Integer` to a value of a primitive unsigned integer type, taking the
            /// `Integer` by value and returning `None` if the `Integer` is negative or too large.
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
            /// assert_eq!(format!("{:?}", u32::checked_from(Integer::from(123))), "Some(123)");
            /// assert_eq!(format!("{:?}", u32::checked_from(Integer::from(-123))), "None");
            /// assert_eq!(format!("{:?}", u32::checked_from(Integer::trillion())), "None");
            /// assert_eq!(format!("{:?}", u32::checked_from(-Integer::trillion())), "None");
            /// ```
            #[inline]
            fn checked_from(value: Integer) -> Option<$u> {
                $u::checked_from(&value)
            }
        }

        impl<'a> CheckedFrom<&'a Integer> for $u {
            /// Converts an `Integer` to a value of a primitive unsigned integer type, taking the
            /// `Integer` by reference and returning `None` if the `Integer` is negative or too
            /// large.
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
            /// assert_eq!(format!("{:?}", u32::checked_from(&Integer::from(123))), "Some(123)");
            /// assert_eq!(format!("{:?}", u32::checked_from(&Integer::from(-123))), "None");
            /// assert_eq!(format!("{:?}", u32::checked_from(&Integer::trillion())), "None");
            /// assert_eq!(format!("{:?}", u32::checked_from(&-Integer::trillion())), "None");
            /// ```
            fn checked_from(value: &Integer) -> Option<$u> {
                match *value {
                    Integer { sign: false, .. } => None,
                    Integer {
                        sign: true,
                        ref abs,
                    } => $u::checked_from(abs),
                }
            }
        }

        impl WrappingFrom<Integer> for $u {
            /// Converts an `Integer` to a value of a primitive unsigned integer type, taking the
            /// `Integer` by value and wrapping mod 2<sup>`$u::WIDTH`</sup>.
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
            /// assert_eq!(u32::wrapping_from(Integer::from(123)), 123);
            /// assert_eq!(u32::wrapping_from(Integer::from(-123)), 4294967173);
            /// assert_eq!(u32::wrapping_from(Integer::trillion()), 3567587328);
            /// assert_eq!(u32::wrapping_from(-Integer::trillion()), 727379968);
            /// ```
            #[inline]
            fn wrapping_from(value: Integer) -> $u {
                $u::wrapping_from(&value)
            }
        }

        impl<'a> WrappingFrom<&'a Integer> for $u {
            /// Converts an `Integer` to a value of a primitive unsigned integer type, taking the
            /// `Integer` by reference and wrapping mod 2<sup>`$u::WIDTH`</sup>.
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
            /// assert_eq!(u32::wrapping_from(&Integer::from(123)), 123);
            /// assert_eq!(u32::wrapping_from(&Integer::from(-123)), 4294967173);
            /// assert_eq!(u32::wrapping_from(&Integer::trillion()), 3567587328);
            /// assert_eq!(u32::wrapping_from(&-Integer::trillion()), 727379968);
            /// ```
            fn wrapping_from(value: &Integer) -> $u {
                match *value {
                    Integer {
                        sign: true,
                        ref abs,
                    } => $u::wrapping_from(abs),
                    Integer {
                        sign: false,
                        ref abs,
                    } => $u::wrapping_from(abs).wrapping_neg(),
                }
            }
        }

        impl SaturatingFrom<Integer> for $u {
            /// Converts an `Integer` to a value of a primitive unsigned integer type, taking the
            /// `Integer` by value. If the `Integer` is too large to fit in a `$u`, `$u::MAX` is
            /// returned. If it is negative, 0 is returned.
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
            /// assert_eq!(u32::saturating_from(Integer::from(123)), 123);
            /// assert_eq!(u32::saturating_from(Integer::from(-123)), 0);
            /// assert_eq!(u32::saturating_from(Integer::trillion()), u32::MAX);
            /// assert_eq!(u32::saturating_from(-Integer::trillion()), 0);
            /// ```
            fn saturating_from(value: Integer) -> $u {
                $u::saturating_from(&value)
            }
        }

        impl<'a> SaturatingFrom<&'a Integer> for $u {
            /// Converts an `Integer` to a value of a primitive unsigned integer type, taking the
            /// `Integer` by reference. If the `Integer` is too large to fit in a `$u`, `$u::MAX` is
            /// returned. If it is negative, 0 is returned.
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
            /// assert_eq!(u32::saturating_from(&Integer::from(123)), 123);
            /// assert_eq!(u32::saturating_from(&Integer::from(-123)), 0);
            /// assert_eq!(u32::saturating_from(&Integer::trillion()), u32::MAX);
            /// assert_eq!(u32::saturating_from(&-Integer::trillion()), 0);
            /// ```
            fn saturating_from(value: &Integer) -> $u {
                match *value {
                    Integer {
                        sign: true,
                        ref abs,
                    } => $u::saturating_from(abs),
                    _ => 0,
                }
            }
        }

        impl OverflowingFrom<Integer> for $u {
            /// Converts an `Integer` to a value of a primitive unsigned integer type, taking the
            /// `Integer` by value and wrapping mod 2<sup>`$u::WIDTH`</sup>. The returned boolean
            /// value indicates whether wrapping occurred.
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
            /// assert_eq!(u32::overflowing_from(Integer::from(123)), (123, false));
            /// assert_eq!(u32::overflowing_from(Integer::from(-123)), (4294967173, true));
            /// assert_eq!(u32::overflowing_from(Integer::trillion()), (3567587328, true));
            /// assert_eq!(u32::overflowing_from(-Integer::trillion()), (727379968, true));
            /// ```
            fn overflowing_from(value: Integer) -> ($u, bool) {
                $u::overflowing_from(&value)
            }
        }

        impl<'a> OverflowingFrom<&'a Integer> for $u {
            /// Converts an `Integer` to a value of a primitive unsigned integer type, taking the
            /// `Integer` by reference and wrapping mod 2<sup>`$u::WIDTH`</sup>. The returned
            /// boolean value indicates whether wrapping occurred.
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
            /// assert_eq!(u32::overflowing_from(&Integer::from(123)), (123, false));
            /// assert_eq!(u32::overflowing_from(&Integer::from(-123)), (4294967173, true));
            /// assert_eq!(u32::overflowing_from(&Integer::trillion()), (3567587328, true));
            /// assert_eq!(u32::overflowing_from(&-Integer::trillion()), (727379968, true));
            /// ```
            fn overflowing_from(value: &Integer) -> ($u, bool) {
                match *value {
                    Integer {
                        sign: true,
                        ref abs,
                    } => $u::overflowing_from(abs),
                    Integer {
                        sign: false,
                        ref abs,
                    } => ($u::wrapping_from(abs).wrapping_neg(), true),
                }
            }
        }

        impl ConvertibleFrom<Integer> for $u {
            /// Determines whether an `Integer` can be converted to a value of a primitive unsigned
            /// integer type. Takes the `Integer` by value.
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
            /// assert_eq!(u32::convertible_from(Integer::from(123)), true);
            /// assert_eq!(u32::convertible_from(Integer::from(-123)), false);
            /// assert_eq!(u32::convertible_from(Integer::trillion()), false);
            /// assert_eq!(u32::convertible_from(-Integer::trillion()), false);
            /// ```
            #[inline]
            fn convertible_from(value: Integer) -> bool {
                $u::convertible_from(&value)
            }
        }

        impl<'a> ConvertibleFrom<&'a Integer> for $u {
            /// Determines whether an `Integer` can be converted to a value of a primitive unsigned
            /// integer type. Takes the `Integer` by reference.
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
            /// assert_eq!(u32::convertible_from(&Integer::from(123)), true);
            /// assert_eq!(u32::convertible_from(&Integer::from(-123)), false);
            /// assert_eq!(u32::convertible_from(&Integer::trillion()), false);
            /// assert_eq!(u32::convertible_from(&-Integer::trillion()), false);
            /// ```
            fn convertible_from(value: &Integer) -> bool {
                value.sign && $u::convertible_from(&value.abs)
            }
        }

        impl CheckedFrom<Integer> for $s {
            /// Converts an `Integer` to a value of a primitive signed integer type, taking the
            /// `Integer` by value and returning `None` if the `Integer` is outside the range of an
            /// `$s`.
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
            /// assert_eq!(i32::checked_from(Integer::from(123)), Some(123));
            /// assert_eq!(i32::checked_from(Integer::from(-123)), Some(-123));
            /// assert_eq!(i32::checked_from(Integer::trillion()), None);
            /// assert_eq!(i32::checked_from(-Integer::trillion()), None);
            /// ```
            #[inline]
            fn checked_from(value: Integer) -> Option<$s> {
                $s::checked_from(&value)
            }
        }

        impl<'a> CheckedFrom<&'a Integer> for $s {
            /// Converts an `Integer` to a value of a primitive signed integer type, taking the
            /// `Integer` by reference and returning `None` if the `Integer` is outside the range of
            /// an `$s`.
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
            /// assert_eq!(i32::checked_from(&Integer::from(123)), Some(123));
            /// assert_eq!(i32::checked_from(&Integer::from(-123)), Some(-123));
            /// assert_eq!(i32::checked_from(&Integer::trillion()), None);
            /// assert_eq!(i32::checked_from(&-Integer::trillion()), None);
            /// ```
            fn checked_from(value: &Integer) -> Option<$s> {
                if $s::convertible_from(value) {
                    Some($s::wrapping_from(value))
                } else {
                    None
                }
            }
        }

        impl WrappingFrom<Integer> for $s {
            /// Converts an `Integer` to a value of a primitive signed integer type, taking the
            /// `Integer` by reference and wrapping mod 2<sup>`$u::WIDTH`</sup>.
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
            /// assert_eq!(i32::wrapping_from(Integer::from(123)), 123);
            /// assert_eq!(i32::wrapping_from(Integer::from(-123)), -123);
            /// assert_eq!(i32::wrapping_from(Integer::trillion()), -727379968);
            /// assert_eq!(i32::wrapping_from(-Integer::trillion()), 727379968);
            /// ```
            #[inline]
            fn wrapping_from(value: Integer) -> $s {
                $s::wrapping_from(&value)
            }
        }

        impl<'a> WrappingFrom<&'a Integer> for $s {
            /// Converts an `Integer` to a value of a primitive signed integer type, taking the
            /// `Integer` by reference and wrapping mod 2<sup>`$u::WIDTH`</sup>.
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
            /// assert_eq!(i32::wrapping_from(&Integer::from(123)), 123);
            /// assert_eq!(i32::wrapping_from(&Integer::from(-123)), -123);
            /// assert_eq!(i32::wrapping_from(&Integer::trillion()), -727379968);
            /// assert_eq!(i32::wrapping_from(&-Integer::trillion()), 727379968);
            /// ```
            fn wrapping_from(value: &Integer) -> $s {
                $s::wrapping_from($u::wrapping_from(value))
            }
        }

        impl SaturatingFrom<Integer> for $s {
            /// Converts an `Integer` to a value of a primitive signed integer type, taking the
            /// `Integer` by value. If the `Integer` is larger than `$s::MAX`, `$s::MAX` is
            /// returned. If it is smaller than `$s::MIN`, `$s::MIN` is returned.
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
            /// assert_eq!(i32::saturating_from(Integer::from(123)), 123);
            /// assert_eq!(i32::saturating_from(Integer::from(-123)), -123);
            /// assert_eq!(i32::saturating_from(Integer::trillion()), 2147483647);
            /// assert_eq!(i32::saturating_from(-Integer::trillion()), -2147483648);
            /// ```
            fn saturating_from(value: Integer) -> $s {
                $s::saturating_from(&value)
            }
        }

        impl<'a> SaturatingFrom<&'a Integer> for $s {
            /// Converts an `Integer` to a value of a primitive signed integer type, taking the
            /// `Integer` by reference. If the `Integer` is larger than `$s::MAX`, `$s::MAX` is
            /// returned. If it is smaller than `$s::MIN`, `$s::MIN` is returned.
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
            /// assert_eq!(i32::saturating_from(&Integer::from(123)), 123);
            /// assert_eq!(i32::saturating_from(&Integer::from(-123)), -123);
            /// assert_eq!(i32::saturating_from(&Integer::trillion()), 2147483647);
            /// assert_eq!(i32::saturating_from(&-Integer::trillion()), -2147483648);
            /// ```
            fn saturating_from(value: &Integer) -> $s {
                match *value {
                    Integer {
                        sign: true,
                        ref abs,
                    } => $s::saturating_from($u::saturating_from(abs)),
                    Integer {
                        sign: false,
                        ref abs,
                    } => {
                        let abs = $u::saturating_from(abs);
                        if abs.get_highest_bit() {
                            $s::MIN
                        } else {
                            -$s::wrapping_from(abs)
                        }
                    }
                }
            }
        }

        impl OverflowingFrom<Integer> for $s {
            /// Converts an `Integer` to a value of a primitive signed integer type, taking the
            /// `Integer` by value and wrapping mod 2<sup>`$u::WIDTH`</sup>. The returned boolean
            /// value indicates whether wrapping occurred.
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
            /// assert_eq!(i32::overflowing_from(Integer::from(123)), (123, false));
            /// assert_eq!(i32::overflowing_from(Integer::from(-123)), (-123, false));
            /// assert_eq!(i32::overflowing_from(Integer::trillion()), (-727379968, true));
            /// assert_eq!(i32::overflowing_from(-Integer::trillion()), (727379968, true));
            /// ```
            fn overflowing_from(value: Integer) -> ($s, bool) {
                $s::overflowing_from(&value)
            }
        }

        impl<'a> OverflowingFrom<&'a Integer> for $s {
            /// Converts an `Integer` to a value of a primitive signed integer type, taking the
            /// `Integer` by reference and wrapping mod 2<sup>`$u::WIDTH`</sup>. The returned
            /// boolean value indicates whether wrapping occurred.
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
            /// assert_eq!(i32::overflowing_from(&Integer::from(123)), (123, false));
            /// assert_eq!(i32::overflowing_from(&Integer::from(-123)), (-123, false));
            /// assert_eq!(i32::overflowing_from(&Integer::trillion()), (-727379968, true));
            /// assert_eq!(i32::overflowing_from(&-Integer::trillion()), (727379968, true));
            /// ```
            fn overflowing_from(value: &Integer) -> ($s, bool) {
                ($s::wrapping_from(value), !$s::convertible_from(value))
            }
        }

        impl ConvertibleFrom<Integer> for $s {
            /// Determines whether an `Integer` can be converted to a value of a primitive signed
            /// integer type. Takes the `Integer` by value.
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
            /// assert_eq!(i32::convertible_from(Integer::from(123)), true);
            /// assert_eq!(i32::convertible_from(Integer::from(-123)), true);
            /// assert_eq!(i32::convertible_from(Integer::trillion()), false);
            /// assert_eq!(i32::convertible_from(-Integer::trillion()), false);
            /// ```
            #[inline]
            fn convertible_from(value: Integer) -> bool {
                $s::convertible_from(&value)
            }
        }

        impl<'a> ConvertibleFrom<&'a Integer> for $s {
            /// Determines whether an `Integer` can be converted to a value of a primitive signed
            /// integer type. Takes the `Integer` by reference.
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
            /// assert_eq!(i32::convertible_from(&Integer::from(123)), true);
            /// assert_eq!(i32::convertible_from(&Integer::from(-123)), true);
            /// assert_eq!(i32::convertible_from(&Integer::trillion()), false);
            /// assert_eq!(i32::convertible_from(&-Integer::trillion()), false);
            /// ```
            fn convertible_from(value: &Integer) -> bool {
                match *value {
                    Integer {
                        sign: true,
                        ref abs,
                    } => abs.significant_bits() < $u::WIDTH,
                    Integer {
                        sign: false,
                        ref abs,
                    } => {
                        let significant_bits = abs.significant_bits();
                        significant_bits < $u::WIDTH
                            || significant_bits == $u::WIDTH
                                && abs.divisible_by_power_of_two($u::WIDTH - 1)
                    }
                }
            }
        }
    };
}
apply_to_unsigned_signed_pair!(impl_from);
