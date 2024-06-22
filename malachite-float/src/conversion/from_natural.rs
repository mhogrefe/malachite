// Copyright © 2024 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::Float;
use crate::InnerFloat::Finite;
use core::cmp::Ordering::{self, *};
use malachite_base::num::arithmetic::traits::NegModPowerOf2;
use malachite_base::num::basic::integers::PrimitiveInt;
use malachite_base::num::basic::traits::Zero;
use malachite_base::num::conversion::traits::ExactFrom;
use malachite_base::num::logic::traits::SignificantBits;
use malachite_base::rounding_modes::RoundingMode::{self, *};
use malachite_nz::natural::Natural;
use malachite_nz::platform::Limb;

impl Float {
    /// Converts a [`Natural`] to a [`Float`], taking the [`Natural`] by value. If the [`Float`] is
    /// nonzero, it has the specified precision. If rounding is needed, the specified rounding mode
    /// is used. An [`Ordering`] is also returned, indicating whether the returned value is less
    /// than, equal to, or greater than the original value.
    ///
    /// If you're only using [`Nearest`], try using [`Float::from_natural_prec`] instead.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(n.significant_bits(), prec)`.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::basic::traits::Zero;
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use malachite_nz::natural::Natural;
    /// use std::cmp::Ordering::*;
    ///
    /// let (x, o) = Float::from_natural_prec_round(Natural::ZERO, 10, Exact);
    /// assert_eq!(x.to_string(), "0.0");
    /// assert_eq!(o, Equal);
    ///
    /// let (x, o) = Float::from_natural_prec_round(Natural::from(123u32), 20, Exact);
    /// assert_eq!(x.to_string(), "123.0");
    /// assert_eq!(x.get_prec(), Some(20));
    /// assert_eq!(o, Equal);
    ///
    /// let (x, o) = Float::from_natural_prec_round(Natural::from(123u32), 4, Floor);
    /// assert_eq!(x.to_string(), "1.2e2");
    /// assert_eq!(x.get_prec(), Some(4));
    /// assert_eq!(o, Less);
    ///
    /// let (x, o) = Float::from_natural_prec_round(Natural::from(123u32), 4, Ceiling);
    /// assert_eq!(x.to_string(), "1.3e2");
    /// assert_eq!(x.get_prec(), Some(4));
    /// assert_eq!(o, Greater);
    /// ```
    #[inline]
    pub fn from_natural_prec_round(x: Natural, prec: u64, rm: RoundingMode) -> (Float, Ordering) {
        assert_ne!(prec, 0);
        if x == 0u32 {
            return (Float::ZERO, Equal);
        }
        let bits = x.significant_bits();
        let mut f = Float(Finite {
            sign: true,
            exponent: i32::exact_from(bits),
            precision: bits,
            significand: x << bits.neg_mod_power_of_2(Limb::LOG_WIDTH),
        });
        let o = f.set_prec_round(prec, rm);
        (f, o)
    }

    /// Converts a [`Natural`] to a [`Float`], taking the [`Natural`] by value. If the [`Float`] is
    /// nonzero, it has the specified precision. An [`Ordering`] is also returned, indicating
    /// whether the returned value is less than, equal to, or greater than the original value.
    ///
    /// If you want the [`Float`]'s precision to be equal to the [`Natural`]'s number of significant
    /// bits, try just using `Float::from` instead.
    ///
    /// Rounding may occur, in which case [`Nearest`] is used by default. To specify a rounding mode
    /// as well as a precision, try [`Float::from_natural_prec_round`].
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(n.significant_bits(), prec)`.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::basic::traits::Zero;
    /// use malachite_float::Float;
    /// use malachite_nz::natural::Natural;
    /// use std::cmp::Ordering::*;
    ///
    /// let (x, o) = Float::from_natural_prec(Natural::ZERO, 10);
    /// assert_eq!(x.to_string(), "0.0");
    /// assert_eq!(o, Equal);
    ///
    /// let (x, o) = Float::from_natural_prec(Natural::from(123u32), 20);
    /// assert_eq!(x.to_string(), "123.0");
    /// assert_eq!(x.get_prec(), Some(20));
    /// assert_eq!(o, Equal);
    ///
    /// let (x, o) = Float::from_natural_prec(Natural::from(123u32), 4);
    /// assert_eq!(x.to_string(), "1.2e2");
    /// assert_eq!(x.get_prec(), Some(4));
    /// assert_eq!(o, Less);
    /// ```
    #[inline]
    pub fn from_natural_prec(x: Natural, prec: u64) -> (Float, Ordering) {
        Float::from_natural_prec_round(x, prec, Nearest)
    }

    /// Converts a [`Natural`] to a [`Float`], taking the [`Natural`] by reference. If the [`Float`]
    /// is nonzero, it has the specified precision. If rounding is needed, the specified rounding
    /// mode is used. An [`Ordering`] is also returned, indicating whether the returned value is
    /// less than, equal to, or greater than the original value.
    ///
    /// If you're only using [`Nearest`], try using [`Float::from_natural_prec_ref`] instead.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(n.significant_bits(), prec)`.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::basic::traits::Zero;
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use malachite_nz::natural::Natural;
    /// use std::cmp::Ordering::*;
    ///
    /// let (x, o) = Float::from_natural_prec_round_ref(&Natural::ZERO, 10, Exact);
    /// assert_eq!(x.to_string(), "0.0");
    /// assert_eq!(o, Equal);
    ///
    /// let (x, o) = Float::from_natural_prec_round_ref(&Natural::from(123u32), 20, Exact);
    /// assert_eq!(x.to_string(), "123.0");
    /// assert_eq!(x.get_prec(), Some(20));
    /// assert_eq!(o, Equal);
    ///
    /// let (x, o) = Float::from_natural_prec_round_ref(&Natural::from(123u32), 4, Floor);
    /// assert_eq!(x.to_string(), "1.2e2");
    /// assert_eq!(x.get_prec(), Some(4));
    /// assert_eq!(o, Less);
    ///
    /// let (x, o) = Float::from_natural_prec_round_ref(&Natural::from(123u32), 4, Ceiling);
    /// assert_eq!(x.to_string(), "1.3e2");
    /// assert_eq!(x.get_prec(), Some(4));
    /// assert_eq!(o, Greater);
    /// ```
    #[inline]
    pub fn from_natural_prec_round_ref(
        x: &Natural,
        prec: u64,
        rm: RoundingMode,
    ) -> (Float, Ordering) {
        // TODO be more efficient when x is large and prec is small
        assert_ne!(prec, 0);
        if *x == 0u32 {
            return (Float::ZERO, Equal);
        }
        let bits = x.significant_bits();
        let mut f = Float(Finite {
            sign: true,
            exponent: i32::exact_from(bits),
            precision: bits,
            significand: x << bits.neg_mod_power_of_2(Limb::LOG_WIDTH),
        });
        let o = f.set_prec_round(prec, rm);
        (f, o)
    }

    /// Converts a [`Natural`] to a [`Float`], taking the [`Natural`] by reference. If the [`Float`]
    /// is nonzero, it has the specified precision. An [`Ordering`] is also returned, indicating
    /// whether the returned value is less than, equal to, or greater than the original value.
    ///
    /// If you want the [`Float`]'s precision to be equal to the [`Natural`]'s number of significant
    /// bits, try just using `Float::from` instead.
    ///
    /// Rounding may occur, in which case [`Nearest`] is used by default. To specify a rounding mode
    /// as well as a precision, try [`Float::from_natural_prec_round_ref`].
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(n.significant_bits(), prec)`.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::basic::traits::Zero;
    /// use malachite_float::Float;
    /// use malachite_nz::natural::Natural;
    /// use std::cmp::Ordering::*;
    ///
    /// let (x, o) = Float::from_natural_prec_ref(&Natural::ZERO, 10);
    /// assert_eq!(x.to_string(), "0.0");
    /// assert_eq!(o, Equal);
    ///
    /// let (x, o) = Float::from_natural_prec_ref(&Natural::from(123u32), 20);
    /// assert_eq!(x.to_string(), "123.0");
    /// assert_eq!(x.get_prec(), Some(20));
    /// assert_eq!(o, Equal);
    ///
    /// let (x, o) = Float::from_natural_prec_ref(&Natural::from(123u32), 4);
    /// assert_eq!(x.to_string(), "1.2e2");
    /// assert_eq!(x.get_prec(), Some(4));
    /// assert_eq!(o, Less);
    /// ```
    #[inline]
    pub fn from_natural_prec_ref(x: &Natural, prec: u64) -> (Float, Ordering) {
        // TODO be more efficient when x is large and prec is small
        assert_ne!(prec, 0);
        if *x == 0u32 {
            return (Float::ZERO, Equal);
        }
        let bits = x.significant_bits();
        let mut f = Float(Finite {
            sign: true,
            exponent: i32::exact_from(bits),
            precision: bits,
            significand: x << bits.neg_mod_power_of_2(Limb::LOG_WIDTH),
        });
        let o = f.set_prec(prec);
        (f, o)
    }
}

impl From<Natural> for Float {
    /// Converts a [`Natural`] to a [`Float`], taking the [`Natural`] by value.
    ///
    /// If the [`Natural`] is nonzero, the precision of the [`Float`] is equal to the [`Natural`]'s
    /// number of significant bits. If you want to specify a different precision, try
    /// [`Float::from_natural_prec`]. This may require rounding, which uses [`Nearest`] by default.
    /// To specify a rounding mode as well as a precision, try [`Float::from_natural_prec_round`].
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(1)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `n.significant_bits()`.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::basic::traits::Zero;
    /// use malachite_float::Float;
    /// use malachite_nz::natural::Natural;
    ///
    /// assert_eq!(Float::from(Natural::ZERO).to_string(), "0.0");
    /// assert_eq!(Float::from(Natural::from(123u32)).to_string(), "123.0");
    /// assert_eq!(Float::from(Natural::from(123u32)).get_prec(), Some(7));
    /// ```
    fn from(n: Natural) -> Float {
        if n == 0u32 {
            return Float::ZERO;
        }
        let bits = n.significant_bits();
        Float(Finite {
            sign: true,
            exponent: i32::exact_from(bits),
            precision: bits,
            significand: n << bits.neg_mod_power_of_2(Limb::LOG_WIDTH),
        })
    }
}

impl<'a> From<&'a Natural> for Float {
    /// Converts a [`Natural`] to a [`Float`], taking the [`Natural`] by reference.
    ///
    /// If the [`Natural`] is nonzero, the precision of the [`Float`] is equal to the [`Natural`]'s
    /// number of significant bits. If you want to specify a different precision, try
    /// [`Float::from_natural_prec_ref`]. This may require rounding, which uses [`Nearest`] by
    /// default. To specify a rounding mode as well as a precision, try
    /// [`Float::from_natural_prec_round_ref`].
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `n.significant_bits()`.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::basic::traits::Zero;
    /// use malachite_float::Float;
    /// use malachite_nz::natural::Natural;
    ///
    /// assert_eq!(Float::from(&Natural::ZERO).to_string(), "0.0");
    /// assert_eq!(Float::from(&Natural::from(123u32)).to_string(), "123.0");
    /// assert_eq!(Float::from(&Natural::from(123u32)).get_prec(), Some(7));
    /// ```
    #[inline]
    fn from(n: &'a Natural) -> Float {
        if *n == 0u32 {
            return Float::ZERO;
        }
        let bits = n.significant_bits();
        Float(Finite {
            sign: true,
            exponent: i32::exact_from(bits),
            precision: bits,
            significand: n << bits.neg_mod_power_of_2(Limb::LOG_WIDTH),
        })
    }
}
