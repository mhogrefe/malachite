use crate::integer::Integer;
use crate::natural::Natural;
use malachite_base::num::basic::traits::Zero;
use malachite_base::num::conversion::traits::{CheckedFrom, ConvertibleFrom, SaturatingFrom};

impl CheckedFrom<Integer> for Natural {
    /// Converts an [`Integer`] to a [`Natural`], taking the [`Natural`] by value. If the
    /// [`Integer`] is negative, `None` is returned.
    ///
    /// # Worst-case complexity
    /// Constant time and additional memory.
    ///
    /// # Examples
    /// ```
    /// extern crate malachite_base;
    ///
    /// use malachite_base::num::arithmetic::traits::Pow;
    /// use malachite_base::num::conversion::traits::CheckedFrom;
    /// use malachite_base::strings::ToDebugString;
    /// use malachite_nz::integer::Integer;
    /// use malachite_nz::natural::Natural;
    ///
    /// assert_eq!(Natural::checked_from(Integer::from(123)).to_debug_string(), "Some(123)");
    /// assert_eq!(Natural::checked_from(Integer::from(-123)).to_debug_string(), "None");
    /// assert_eq!(
    ///     Natural::checked_from(Integer::from(10u32).pow(12)).to_debug_string(),
    ///     "Some(1000000000000)"
    /// );
    /// assert_eq!(Natural::checked_from(-Integer::from(10u32).pow(12)).to_debug_string(), "None");
    /// ```
    fn checked_from(value: Integer) -> Option<Natural> {
        match value {
            Integer { sign: false, .. } => None,
            Integer { sign: true, abs } => Some(abs),
        }
    }
}

impl<'a> CheckedFrom<&'a Integer> for Natural {
    /// Converts an [`Integer`] to a [`Natural`], taking the [`Natural`] by reference. If the
    /// [`Integer`] is negative, `None` is returned.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `value.significant_bits()`.
    ///
    /// # Examples
    /// ```
    /// extern crate malachite_base;
    ///
    /// use malachite_base::num::arithmetic::traits::Pow;
    /// use malachite_base::num::conversion::traits::CheckedFrom;
    /// use malachite_base::strings::ToDebugString;
    /// use malachite_nz::integer::Integer;
    /// use malachite_nz::natural::Natural;
    ///
    /// assert_eq!(Natural::checked_from(&Integer::from(123)).to_debug_string(), "Some(123)");
    /// assert_eq!(Natural::checked_from(&Integer::from(-123)).to_debug_string(), "None");
    /// assert_eq!(
    ///     Natural::checked_from(&Integer::from(10u32).pow(12)).to_debug_string(),
    ///     "Some(1000000000000)"
    /// );
    /// assert_eq!(
    ///     Natural::checked_from(&(-Integer::from(10u32).pow(12))).to_debug_string(),
    ///     "None"
    /// );
    /// ```
    fn checked_from(value: &'a Integer) -> Option<Natural> {
        match *value {
            Integer { sign: false, .. } => None,
            Integer {
                sign: true,
                ref abs,
            } => Some(abs.clone()),
        }
    }
}

impl SaturatingFrom<Integer> for Natural {
    /// Converts an [`Integer`] to a [`Natural`], taking the [`Natural`] by value. If the
    /// [`Integer`] is negative, 0 is returned.
    ///
    /// # Worst-case complexity
    /// Constant time and additional memory.
    ///
    /// # Examples
    /// ```
    /// extern crate malachite_base;
    ///
    /// use malachite_base::num::arithmetic::traits::Pow;
    /// use malachite_base::num::conversion::traits::SaturatingFrom;
    /// use malachite_nz::integer::Integer;
    /// use malachite_nz::natural::Natural;
    ///
    /// assert_eq!(Natural::saturating_from(Integer::from(123)), 123);
    /// assert_eq!(Natural::saturating_from(Integer::from(-123)), 0);
    /// assert_eq!(Natural::saturating_from(Integer::from(10u32).pow(12)), 1000000000000u64);
    /// assert_eq!(Natural::saturating_from(-Integer::from(10u32).pow(12)), 0);
    /// ```
    fn saturating_from(value: Integer) -> Natural {
        match value {
            Integer { sign: false, .. } => Natural::ZERO,
            Integer { sign: true, abs } => abs,
        }
    }
}

impl<'a> SaturatingFrom<&'a Integer> for Natural {
    /// Converts an [`Integer`] to a [`Natural`], taking the [`Natural`] by reference. If the
    /// [`Integer`] is negative, 0 is returned.
    ///
    /// # Worst-case complexity
    /// Constant time and additional memory.
    ///
    /// # Examples
    /// ```
    /// extern crate malachite_base;
    ///
    /// use malachite_base::num::arithmetic::traits::Pow;
    /// use malachite_base::num::conversion::traits::SaturatingFrom;
    /// use malachite_nz::integer::Integer;
    /// use malachite_nz::natural::Natural;
    ///
    /// assert_eq!(Natural::saturating_from(&Integer::from(123)), 123);
    /// assert_eq!(Natural::saturating_from(&Integer::from(-123)), 0);
    /// assert_eq!(Natural::saturating_from(&Integer::from(10u32).pow(12)), 1000000000000u64);
    /// assert_eq!(Natural::saturating_from(&-Integer::from(10u32).pow(12)), 0);
    /// ```
    fn saturating_from(value: &'a Integer) -> Natural {
        match *value {
            Integer { sign: false, .. } => Natural::ZERO,
            Integer {
                sign: true,
                ref abs,
            } => abs.clone(),
        }
    }
}

impl ConvertibleFrom<Integer> for Natural {
    /// Determines whether an [`Integer`] can be converted to a [`Natural`] (when the [`Integer`]
    /// is non-negative). Takes the [`Integer`] by value.
    ///
    /// # Worst-case complexity
    /// Constant time and additional memory.
    ///
    /// # Examples
    /// ```
    /// extern crate malachite_base;
    ///
    /// use malachite_base::num::arithmetic::traits::Pow;
    /// use malachite_base::num::conversion::traits::ConvertibleFrom;
    /// use malachite_nz::integer::Integer;
    /// use malachite_nz::natural::Natural;
    ///
    /// assert_eq!(Natural::convertible_from(Integer::from(123)), true);
    /// assert_eq!(Natural::convertible_from(Integer::from(-123)), false);
    /// assert_eq!(Natural::convertible_from(Integer::from(10u32).pow(12)), true);
    /// assert_eq!(Natural::convertible_from(-Integer::from(10u32).pow(12)), false);
    /// ```
    #[inline]
    fn convertible_from(value: Integer) -> bool {
        value.sign
    }
}

impl<'a> ConvertibleFrom<&'a Integer> for Natural {
    /// Determines whether an [`Integer`] can be converted to a [`Natural`] (when the [`Integer`]
    /// is non-negative). Takes the [`Integer`] by reference.
    ///
    /// # Worst-case complexity
    /// Constant time and additional memory.
    ///
    /// # Examples
    /// ```
    /// extern crate malachite_base;
    ///
    /// use malachite_base::num::arithmetic::traits::Pow;
    /// use malachite_base::num::conversion::traits::ConvertibleFrom;
    /// use malachite_nz::integer::Integer;
    /// use malachite_nz::natural::Natural;
    ///
    /// assert_eq!(Natural::convertible_from(&Integer::from(123)), true);
    /// assert_eq!(Natural::convertible_from(&Integer::from(-123)), false);
    /// assert_eq!(Natural::convertible_from(&Integer::from(10u32).pow(12)), true);
    /// assert_eq!(Natural::convertible_from(&-Integer::from(10u32).pow(12)), false);
    /// ```
    #[inline]
    fn convertible_from(value: &'a Integer) -> bool {
        value.sign
    }
}
