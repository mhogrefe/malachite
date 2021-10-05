use integer::Integer;
use malachite_base::comparison::traits::Min;
use malachite_base::num::arithmetic::traits::{DivisibleByPowerOf2, WrappingNeg};
use malachite_base::num::basic::integers::PrimitiveInt;
use malachite_base::num::basic::traits::Zero;
use malachite_base::num::conversion::traits::{
    CheckedFrom, ConvertibleFrom, OverflowingFrom, SaturatingFrom, WrappingFrom,
};
use malachite_base::num::logic::traits::SignificantBits;
use natural::Natural;
use std::ops::Neg;

fn checked_from_unsigned<'a, T: CheckedFrom<&'a Natural>>(value: &'a Integer) -> Option<T> {
    match *value {
        Integer { sign: false, .. } => None,
        Integer {
            sign: true,
            ref abs,
        } => T::checked_from(abs),
    }
}

fn wrapping_from_unsigned<'a, T: WrappingFrom<&'a Natural> + WrappingNeg<Output = T>>(
    value: &'a Integer,
) -> T {
    match *value {
        Integer {
            sign: true,
            ref abs,
        } => T::wrapping_from(abs),
        Integer {
            sign: false,
            ref abs,
        } => T::wrapping_from(abs).wrapping_neg(),
    }
}

fn saturating_from_unsigned<'a, T: Copy + SaturatingFrom<&'a Natural> + Zero>(
    value: &'a Integer,
) -> T {
    match *value {
        Integer {
            sign: true,
            ref abs,
        } => T::saturating_from(abs),
        _ => T::ZERO,
    }
}

fn overflowing_from_unsigned<
    'a,
    T: OverflowingFrom<&'a Natural> + WrappingFrom<&'a Natural> + WrappingNeg<Output = T>,
>(
    value: &'a Integer,
) -> (T, bool) {
    match *value {
        Integer {
            sign: true,
            ref abs,
        } => T::overflowing_from(abs),
        Integer {
            sign: false,
            ref abs,
        } => (T::wrapping_from(abs).wrapping_neg(), true),
    }
}

fn checked_from_signed<'a, T: ConvertibleFrom<&'a Integer> + WrappingFrom<&'a Integer>>(
    value: &'a Integer,
) -> Option<T> {
    if T::convertible_from(value) {
        Some(T::wrapping_from(value))
    } else {
        None
    }
}

fn saturating_from_signed<
    'a,
    U: PrimitiveInt + SaturatingFrom<&'a Natural>,
    S: Min + Neg<Output = S> + SaturatingFrom<U> + WrappingFrom<U>,
>(
    value: &'a Integer,
) -> S {
    match *value {
        Integer {
            sign: true,
            ref abs,
        } => S::saturating_from(U::saturating_from(abs)),
        Integer {
            sign: false,
            ref abs,
        } => {
            let abs = U::saturating_from(abs);
            if abs.get_highest_bit() {
                S::MIN
            } else {
                -S::wrapping_from(abs)
            }
        }
    }
}

fn convertible_from_signed<T: PrimitiveInt>(value: &Integer) -> bool {
    match *value {
        Integer {
            sign: true,
            ref abs,
        } => abs.significant_bits() < T::WIDTH,
        Integer {
            sign: false,
            ref abs,
        } => {
            let significant_bits = abs.significant_bits();
            significant_bits < T::WIDTH
                || significant_bits == T::WIDTH && abs.divisible_by_power_of_2(T::WIDTH - 1)
        }
    }
}

macro_rules! impl_from {
    ($u: ident, $s: ident) => {
        impl<'a> CheckedFrom<&'a Integer> for $u {
            /// Converts an `Integer` to a value of a primitive unsigned integer type, taking the
            /// `Integer` by reference and returning `None` if the `Integer` is negative or too
            /// large.
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
            /// use malachite_base::num::conversion::traits::CheckedFrom;
            /// use malachite_nz::integer::Integer;
            ///
            /// assert_eq!(
            ///     format!("{:?}", u32::checked_from(&Integer::from(123))),
            ///     "Some(123)"
            /// );
            /// assert_eq!(
            ///     format!("{:?}", u32::checked_from(&Integer::from(-123))),
            ///     "None"
            /// );
            /// assert_eq!(
            ///     format!("{:?}", u32::checked_from(&Integer::trillion())),
            ///     "None"
            /// );
            /// assert_eq!(
            ///     format!("{:?}", u32::checked_from(&-Integer::trillion())),
            ///     "None"
            /// );
            /// ```
            #[inline]
            fn checked_from(value: &Integer) -> Option<$u> {
                checked_from_unsigned(value)
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
            /// # Examples
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
            #[inline]
            fn wrapping_from(value: &Integer) -> $u {
                wrapping_from_unsigned(value)
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
            /// # Examples
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
            #[inline]
            fn saturating_from(value: &Integer) -> $u {
                saturating_from_unsigned(value)
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
            /// # Examples
            /// ```
            /// extern crate malachite_base;
            /// extern crate malachite_nz;
            ///
            /// use malachite_base::num::conversion::traits::OverflowingFrom;
            /// use malachite_nz::integer::Integer;
            ///
            /// assert_eq!(u32::overflowing_from(&Integer::from(123)), (123, false));
            /// assert_eq!(
            ///     u32::overflowing_from(&Integer::from(-123)),
            ///     (4294967173, true)
            /// );
            /// assert_eq!(
            ///     u32::overflowing_from(&Integer::trillion()),
            ///     (3567587328, true)
            /// );
            /// assert_eq!(
            ///     u32::overflowing_from(&-Integer::trillion()),
            ///     (727379968, true)
            /// );
            /// ```
            #[inline]
            fn overflowing_from(value: &Integer) -> ($u, bool) {
                overflowing_from_unsigned(value)
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
            /// # Examples
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
            #[inline]
            fn convertible_from(value: &Integer) -> bool {
                value.sign && $u::convertible_from(&value.abs)
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
            /// # Examples
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
            #[inline]
            fn checked_from(value: &Integer) -> Option<$s> {
                checked_from_signed(value)
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
            /// # Examples
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
            #[inline]
            fn wrapping_from(value: &Integer) -> $s {
                $s::wrapping_from($u::wrapping_from(value))
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
            /// # Examples
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
            #[inline]
            fn saturating_from(value: &Integer) -> $s {
                saturating_from_signed::<$u, $s>(value)
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
            /// # Examples
            /// ```
            /// extern crate malachite_base;
            /// extern crate malachite_nz;
            ///
            /// use malachite_base::num::conversion::traits::OverflowingFrom;
            /// use malachite_nz::integer::Integer;
            ///
            /// assert_eq!(i32::overflowing_from(&Integer::from(123)), (123, false));
            /// assert_eq!(i32::overflowing_from(&Integer::from(-123)), (-123, false));
            /// assert_eq!(
            ///     i32::overflowing_from(&Integer::trillion()),
            ///     (-727379968, true)
            /// );
            /// assert_eq!(
            ///     i32::overflowing_from(&-Integer::trillion()),
            ///     (727379968, true)
            /// );
            /// ```
            #[inline]
            fn overflowing_from(value: &Integer) -> ($s, bool) {
                ($s::wrapping_from(value), !$s::convertible_from(value))
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
            /// # Examples
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
            #[inline]
            fn convertible_from(value: &Integer) -> bool {
                convertible_from_signed::<$u>(value)
            }
        }
    };
}
apply_to_unsigned_signed_pairs!(impl_from);
