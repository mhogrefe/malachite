// Copyright © 2025 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::InnerFloat::Finite;
use crate::{significand_bits, Float};
use core::cmp::Ordering::{self, *};
use malachite_base::num::arithmetic::traits::{
    NegAssign, RoundToMultipleOfPowerOf2, RoundToMultipleOfPowerOf2Assign,
};
use malachite_base::num::basic::integers::PrimitiveInt;
use malachite_base::num::basic::traits::{Infinity, NegativeInfinity};
use malachite_base::num::conversion::traits::ExactFrom;
use malachite_base::num::logic::traits::SignificantBits;
use malachite_base::rounding_modes::RoundingMode::{self, *};
use malachite_nz::natural::Natural;
use malachite_nz::platform::Limb;

const PREC_ROUND_THRESHOLD: u64 = 1500;

impl Float {
    /// Gets the significand of a [`Float`], taking the [`Float`] by value.
    ///
    /// The significand is the smallest positive integer which is some power of 2 times the
    /// [`Float`], and whose number of significant bits is a multiple of the limb width. If the
    /// [`Float`] is NaN, infinite, or zero, then `None` is returned.
    ///
    /// # Worst-case complexity
    /// Constant time and additional memory.
    ///
    /// # Examples
    /// ```
    /// #[cfg(not(feature = "32_bit_limbs"))]
    /// use malachite_base::num::arithmetic::traits::PowerOf2;
    /// #[cfg(not(feature = "32_bit_limbs"))]
    /// use malachite_base::num::basic::traits::One;
    /// use malachite_base::num::basic::traits::{Infinity, NaN, Zero};
    /// use malachite_float::Float;
    /// #[cfg(not(feature = "32_bit_limbs"))]
    /// use malachite_nz::natural::Natural;
    ///
    /// assert_eq!(Float::NAN.to_significand(), None);
    /// assert_eq!(Float::INFINITY.to_significand(), None);
    /// assert_eq!(Float::ZERO.to_significand(), None);
    ///
    /// #[cfg(not(feature = "32_bit_limbs"))]
    /// {
    ///     assert_eq!(Float::ONE.to_significand(), Some(Natural::power_of_2(63)));
    ///     assert_eq!(
    ///         Float::from(std::f64::consts::PI).to_significand().unwrap(),
    ///         14488038916154245120u64
    ///     );
    /// }
    /// ```
    #[inline]
    pub fn to_significand(&self) -> Option<Natural> {
        match self {
            Float(Finite { significand, .. }) => Some(significand.clone()),
            _ => None,
        }
    }

    /// Gets the significand of a [`Float`], taking the [`Float`] by reference.
    ///
    /// The significand is the smallest positive integer which is some power of 2 times the
    /// [`Float`], and whose number of significant bits is a multiple of the limb width. If the
    /// [`Float`] is NaN, infinite, or zero, then `None` is returned.
    ///
    /// # Worst-case complexity
    /// Constant time and additional memory.
    ///
    /// # Examples
    /// ```
    /// #[cfg(not(feature = "32_bit_limbs"))]
    /// use malachite_base::num::arithmetic::traits::PowerOf2;
    /// #[cfg(not(feature = "32_bit_limbs"))]
    /// use malachite_base::num::basic::traits::One;
    /// use malachite_base::num::basic::traits::{Infinity, NaN, Zero};
    /// use malachite_float::Float;
    /// #[cfg(not(feature = "32_bit_limbs"))]
    /// use malachite_nz::natural::Natural;
    ///
    /// assert_eq!(Float::NAN.into_significand(), None);
    /// assert_eq!(Float::INFINITY.into_significand(), None);
    /// assert_eq!(Float::ZERO.into_significand(), None);
    ///
    /// #[cfg(not(feature = "32_bit_limbs"))]
    /// {
    ///     assert_eq!(Float::ONE.into_significand(), Some(Natural::power_of_2(63)));
    ///     assert_eq!(
    ///         Float::from(std::f64::consts::PI)
    ///             .into_significand()
    ///             .unwrap(),
    ///         14488038916154245120u64
    ///     );
    /// }
    /// ```
    #[allow(clippy::missing_const_for_fn)] // destructor doesn't work with const
    #[inline]
    pub fn into_significand(self) -> Option<Natural> {
        match self {
            Float(Finite { significand, .. }) => Some(significand),
            _ => None,
        }
    }

    /// Returns a reference to the significand of a [`Float`].
    ///
    /// The significand is the smallest positive integer which is some power of 2 times the
    /// [`Float`], and whose number of significant bits is a multiple of the limb width. If the
    /// [`Float`] is NaN, infinite, or zero, then `None` is returned.
    ///
    /// # Worst-case complexity
    /// Constant time and additional memory.
    ///
    /// # Examples
    /// ```
    /// #[cfg(not(feature = "32_bit_limbs"))]
    /// use malachite_base::num::arithmetic::traits::PowerOf2;
    /// #[cfg(not(feature = "32_bit_limbs"))]
    /// use malachite_base::num::basic::traits::One;
    /// use malachite_base::num::basic::traits::{Infinity, NaN, Zero};
    /// use malachite_float::Float;
    /// #[cfg(not(feature = "32_bit_limbs"))]
    /// use malachite_nz::natural::Natural;
    ///
    /// assert_eq!(Float::NAN.significand_ref(), None);
    /// assert_eq!(Float::INFINITY.significand_ref(), None);
    /// assert_eq!(Float::ZERO.significand_ref(), None);
    ///
    /// #[cfg(not(feature = "32_bit_limbs"))]
    /// {
    ///     assert_eq!(
    ///         *Float::ONE.significand_ref().unwrap(),
    ///         Natural::power_of_2(63)
    ///     );
    ///     assert_eq!(
    ///         *Float::from(std::f64::consts::PI).significand_ref().unwrap(),
    ///         14488038916154245120u64
    ///     );
    /// }
    /// ```
    #[inline]
    pub const fn significand_ref(&self) -> Option<&Natural> {
        match self {
            Float(Finite { significand, .. }) => Some(significand),
            _ => None,
        }
    }

    /// Returns a [`Float`]'s exponent.
    ///
    /// $$
    /// f(\text{NaN}) = f(\pm\infty) = f(\pm 0.0) = \text{None},
    /// $$
    ///
    /// and, if $x$ is finite and nonzero,
    ///
    /// $$
    /// f(x) = \operatorname{Some}(\lfloor \log_2 x \rfloor + 1).
    /// $$
    ///
    /// The output is in the range $[-(2^{30}-1), 2^{30}-1]$.
    ///
    /// # Worst-case complexity
    /// Constant time and additional memory.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::PowerOf2;
    /// use malachite_base::num::basic::traits::{Infinity, NaN, One, Zero};
    /// use malachite_float::Float;
    ///
    /// assert_eq!(Float::NAN.get_exponent(), None);
    /// assert_eq!(Float::INFINITY.get_exponent(), None);
    /// assert_eq!(Float::ZERO.get_exponent(), None);
    ///
    /// assert_eq!(Float::ONE.get_exponent(), Some(1));
    /// assert_eq!(Float::from(std::f64::consts::PI).get_exponent(), Some(2));
    /// assert_eq!(Float::power_of_2(100u64).get_exponent(), Some(101));
    /// assert_eq!(Float::power_of_2(-100i64).get_exponent(), Some(-99));
    /// ```
    #[inline]
    pub const fn get_exponent(&self) -> Option<i32> {
        match self {
            Float(Finite { exponent, .. }) => Some(*exponent),
            _ => None,
        }
    }

    /// Returns a [`Float`]'s precision. The precision is a positive integer denoting how many of
    /// the [`Float`]'s bits are significant.
    ///
    /// Only [`Float`]s that are finite and nonzero have a precision. For other [`Float`]s, `None`
    /// is returned.
    ///
    /// # Worst-case complexity
    /// Constant time and additional memory.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::basic::traits::{Infinity, NaN, One, Zero};
    /// use malachite_float::Float;
    ///
    /// assert_eq!(Float::NAN.get_prec(), None);
    /// assert_eq!(Float::INFINITY.get_prec(), None);
    /// assert_eq!(Float::ZERO.get_prec(), None);
    ///
    /// assert_eq!(Float::ONE.get_prec(), Some(1));
    /// assert_eq!(Float::one_prec(100).get_prec(), Some(100));
    /// assert_eq!(Float::from(std::f64::consts::PI).get_prec(), Some(50));
    /// ```
    #[inline]
    pub const fn get_prec(&self) -> Option<u64> {
        match self {
            Float(Finite { precision, .. }) => Some(*precision),
            _ => None,
        }
    }

    /// Returns the minimum precision necessary to represent the given [`Float`]'s value.
    ///
    /// For example, `Float:one_prec(100)` has a precision of 100, but its minimum precision is 1,
    /// because that's all that's necessary to represent the value 1.
    ///
    /// The minimum precision is always less than or equal to the actual precision.
    ///
    /// Only [`Float`]s that are finite and nonzero have a minimum precision. For other [`Float`]s,
    /// `None` is returned.
    ///
    /// # Worst-case complexity
    /// Constant time and additional memory.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::basic::traits::{Infinity, NaN, One, Zero};
    /// use malachite_float::Float;
    ///
    /// assert_eq!(Float::NAN.get_min_prec(), None);
    /// assert_eq!(Float::INFINITY.get_min_prec(), None);
    /// assert_eq!(Float::ZERO.get_min_prec(), None);
    ///
    /// assert_eq!(Float::ONE.get_min_prec(), Some(1));
    /// assert_eq!(Float::one_prec(100).get_min_prec(), Some(1));
    /// assert_eq!(Float::from(std::f64::consts::PI).get_min_prec(), Some(50));
    /// ```
    pub fn get_min_prec(&self) -> Option<u64> {
        match self {
            Float(Finite { significand, .. }) => {
                Some(significand_bits(significand) - significand.trailing_zeros().unwrap())
            }
            _ => None,
        }
    }

    /// Changes a [`Float`]'s precision. If the precision decreases, rounding may be necessary, and
    /// will use the provided [`RoundingMode`].
    ///
    /// Returns an [`Ordering`], indicating whether the final value is less than, greater than, or
    /// equal to the original value.
    ///
    /// If the [`Float`] originally had the maximum exponent, it is possible for this function to
    /// overflow. This is even possible if `rm` is `Nearest`, even though infinity is never nearer
    /// to the exact result than any finite [`Float`] is. This is to match the behavior of MPFR.
    ///
    /// This function never underflows.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `prec`.
    ///
    /// # Panics
    /// Panics if `prec` is zero or if `rm` is [`Exact`] but setting the desired precision requires
    /// rounding.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// let original_x = Float::from(1.0f64 / 3.0);
    /// assert_eq!(original_x.to_string(), "0.33333333333333331");
    /// assert_eq!(original_x.get_prec(), Some(53));
    ///
    /// let mut x = original_x.clone();
    /// assert_eq!(x.set_prec_round(100, Exact), Equal);
    /// assert_eq!(x.to_string(), "0.3333333333333333148296162562474");
    /// assert_eq!(x.get_prec(), Some(100));
    ///
    /// let mut x = original_x.clone();
    /// assert_eq!(x.set_prec_round(10, Floor), Less);
    /// assert_eq!(x.to_string(), "0.333");
    /// assert_eq!(x.get_prec(), Some(10));
    ///
    /// let mut x = original_x.clone();
    /// assert_eq!(x.set_prec_round(10, Ceiling), Greater);
    /// assert_eq!(x.to_string(), "0.3335");
    /// assert_eq!(x.get_prec(), Some(10));
    /// ```
    pub fn set_prec_round(&mut self, prec: u64, rm: RoundingMode) -> Ordering {
        assert_ne!(prec, 0);
        match self {
            Float(Finite {
                sign,
                exponent,
                precision,
                significand,
            }) => {
                let target_bits = prec
                    .round_to_multiple_of_power_of_2(Limb::LOG_WIDTH, Ceiling)
                    .0;
                let significant_bits = significand_bits(significand);
                let o;
                if target_bits > significant_bits {
                    *significand <<= target_bits - significant_bits;
                    o = Equal;
                } else {
                    let limb_count = significand.limb_count();
                    let abs_rm = if *sign { rm } else { -rm };
                    o = significand
                        .round_to_multiple_of_power_of_2_assign(significant_bits - prec, abs_rm);
                    if significand.limb_count() > limb_count {
                        if *exponent == Float::MAX_EXPONENT {
                            return if *sign {
                                *self = Float::INFINITY;
                                Greater
                            } else {
                                *self = Float::NEGATIVE_INFINITY;
                                Less
                            };
                        }
                        *significand >>= 1;
                        *exponent += 1;
                    }
                    *significand >>= significant_bits - target_bits;
                }
                *precision = prec;
                if *sign {
                    o
                } else {
                    o.reverse()
                }
            }
            _ => Equal,
        }
    }

    /// Changes a [`Float`]'s precision. If the precision decreases, rounding may be necessary, and
    /// [`Nearest`] will be used.
    ///
    /// Returns an [`Ordering`], indicating whether the final value is less than, greater than, or
    /// equal to the original value.
    ///
    /// If the [`Float`] originally had the maximum exponent, it is possible for this function to
    /// overflow, even though infinity is never nearer to the exact result than any finite [`Float`]
    /// is. This is to match the behavior of MPFR.
    ///
    /// This function never underflows.
    ///
    /// To use a different rounding mode, try [`Float::set_prec_round`].
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `prec`.
    ///
    /// # Examples
    /// ```
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// let original_x = Float::from(1.0f64 / 3.0);
    /// assert_eq!(original_x.to_string(), "0.33333333333333331");
    /// assert_eq!(original_x.get_prec(), Some(53));
    ///
    /// let mut x = original_x.clone();
    /// assert_eq!(x.set_prec(100), Equal);
    /// assert_eq!(x.to_string(), "0.3333333333333333148296162562474");
    /// assert_eq!(x.get_prec(), Some(100));
    ///
    /// let mut x = original_x.clone();
    /// assert_eq!(x.set_prec(10), Greater);
    /// assert_eq!(x.to_string(), "0.3335");
    /// assert_eq!(x.get_prec(), Some(10));
    /// ```
    #[inline]
    pub fn set_prec(&mut self, p: u64) -> Ordering {
        self.set_prec_round(p, Nearest)
    }

    /// Creates a [`Float`] from another [`Float`], possibly with a different precision. If the
    /// precision decreases, rounding may be necessary, and will use the provided [`RoundingMode`].
    /// The input [`Float`] is taken by value.
    ///
    /// Returns an [`Ordering`], indicating whether the final value is less than, greater than, or
    /// equal to the original value.
    ///
    /// If the input [`Float`] has the maximum exponent, it is possible for this function to
    /// overflow. This is even possible if `rm` is `Nearest`, even though infinity is never nearer
    /// to the exact result than any finite [`Float`] is. This is to match the behavior of MPFR.
    ///
    /// This function never underflows.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `prec`.
    ///
    /// # Panics
    /// Panics if `prec` is zero or if `rm` is [`Exact`] but setting the desired precision requires
    /// rounding.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// let original_x = Float::from(1.0f64 / 3.0);
    /// assert_eq!(original_x.to_string(), "0.33333333333333331");
    /// assert_eq!(original_x.get_prec(), Some(53));
    ///
    /// let (x, o) = Float::from_float_prec_round(original_x.clone(), 100, Exact);
    /// assert_eq!(x.to_string(), "0.3333333333333333148296162562474");
    /// assert_eq!(x.get_prec(), Some(100));
    /// assert_eq!(o, Equal);
    ///
    /// let (x, o) = Float::from_float_prec_round(original_x.clone(), 10, Floor);
    /// assert_eq!(x.to_string(), "0.333");
    /// assert_eq!(x.get_prec(), Some(10));
    /// assert_eq!(o, Less);
    ///
    /// let (x, o) = Float::from_float_prec_round(original_x.clone(), 10, Ceiling);
    /// assert_eq!(x.to_string(), "0.3335");
    /// assert_eq!(x.get_prec(), Some(10));
    /// assert_eq!(o, Greater);
    /// ```
    #[inline]
    pub fn from_float_prec_round(mut x: Float, prec: u64, rm: RoundingMode) -> (Float, Ordering) {
        let o = x.set_prec_round(prec, rm);
        (x, o)
    }

    /// Creates a [`Float`] from another [`Float`], possibly with a different precision. If the
    /// precision decreases, rounding may be necessary, and will use the provided [`RoundingMode`].
    /// The input [`Float`] is taken by reference.
    ///
    /// Returns an [`Ordering`], indicating whether the final value is less than, greater than, or
    /// equal to the original value.
    ///
    /// If the input [`Float`] has the maximum exponent, it is possible for this function to
    /// overflow. This is even possible if `rm` is `Nearest`, even though infinity is never nearer
    /// to the exact result than any finite [`Float`] is. This is to match the behavior of MPFR.
    ///
    /// This function never underflows.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `prec`.
    ///
    /// # Panics
    /// Panics if `prec` is zero or if `rm` is [`Exact`] but setting the desired precision requires
    /// rounding.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// let original_x = Float::from(1.0f64 / 3.0);
    /// assert_eq!(original_x.to_string(), "0.33333333333333331");
    /// assert_eq!(original_x.get_prec(), Some(53));
    ///
    /// let (x, o) = Float::from_float_prec_round_ref(&original_x, 100, Exact);
    /// assert_eq!(x.to_string(), "0.3333333333333333148296162562474");
    /// assert_eq!(x.get_prec(), Some(100));
    /// assert_eq!(o, Equal);
    ///
    /// let (x, o) = Float::from_float_prec_round_ref(&original_x, 10, Floor);
    /// assert_eq!(x.to_string(), "0.333");
    /// assert_eq!(x.get_prec(), Some(10));
    /// assert_eq!(o, Less);
    ///
    /// let (x, o) = Float::from_float_prec_round_ref(&original_x, 10, Ceiling);
    /// assert_eq!(x.to_string(), "0.3335");
    /// assert_eq!(x.get_prec(), Some(10));
    /// assert_eq!(o, Greater);
    /// ```
    pub fn from_float_prec_round_ref(x: &Float, prec: u64, rm: RoundingMode) -> (Float, Ordering) {
        if x.significant_bits() < PREC_ROUND_THRESHOLD {
            let mut x = x.clone();
            let o = x.set_prec_round(prec, rm);
            return (x, o);
        }
        match x {
            Float(Finite {
                sign,
                exponent,
                significand,
                ..
            }) => {
                let (mut y, mut o) = Float::from_natural_prec_round_ref(
                    significand,
                    prec,
                    if *sign { rm } else { -rm },
                );
                if !sign {
                    y.neg_assign();
                    o = o.reverse();
                }
                (
                    y >> (i32::exact_from(significand_bits(significand)) - exponent),
                    o,
                )
            }
            _ => (x.clone(), Equal),
        }
    }

    /// Creates a [`Float`] from another [`Float`], possibly with a different precision. If the
    /// precision decreases, rounding may be necessary, and will use [`Nearest`]. The input
    /// [`Float`] is taken by value.
    ///
    /// Returns an [`Ordering`], indicating whether the final value is less than, greater than, or
    /// equal to the original value.
    ///
    /// If the [`Float`] originally had the maximum exponent, it is possible for this function to
    /// overflow, even though infinity is never nearer to the exact result than any finite [`Float`]
    /// is. This is to match the behavior of MPFR.
    ///
    /// This function never underflows.
    ///
    /// To use a different rounding mode, try [`Float::from_float_prec_round`].
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `prec`.
    ///
    /// # Panics
    /// Panics if `prec` is zero.
    ///
    /// # Examples
    /// ```
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// let original_x = Float::from(1.0f64 / 3.0);
    /// assert_eq!(original_x.to_string(), "0.33333333333333331");
    /// assert_eq!(original_x.get_prec(), Some(53));
    ///
    /// let (x, o) = Float::from_float_prec(original_x.clone(), 100);
    /// assert_eq!(x.to_string(), "0.3333333333333333148296162562474");
    /// assert_eq!(x.get_prec(), Some(100));
    /// assert_eq!(o, Equal);
    ///
    /// let (x, o) = Float::from_float_prec(original_x.clone(), 10);
    /// assert_eq!(x.to_string(), "0.3335");
    /// assert_eq!(x.get_prec(), Some(10));
    /// assert_eq!(o, Greater);
    /// ```
    #[inline]
    pub fn from_float_prec(mut x: Float, prec: u64) -> (Float, Ordering) {
        let o = x.set_prec(prec);
        (x, o)
    }

    /// Creates a [`Float`] from another [`Float`], possibly with a different precision. If the
    /// precision decreases, rounding may be necessary, and will use [`Nearest`]. The input
    /// [`Float`] is taken by reference.
    ///
    /// Returns an [`Ordering`], indicating whether the final value is less than, greater than, or
    /// equal to the original value.
    ///
    /// If the [`Float`] originally had the maximum exponent, it is possible for this function to
    /// overflow, even though infinity is never nearer to the exact result than any finite [`Float`]
    /// is. This is to match the behavior of MPFR.
    ///
    /// This function never underflows.
    ///
    /// To use a different rounding mode, try [`Float::from_float_prec_round_ref`].
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `prec`.
    ///
    /// # Panics
    /// Panics if `prec` is zero.
    ///
    /// # Examples
    /// ```
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// let original_x = Float::from(1.0f64 / 3.0);
    /// assert_eq!(original_x.to_string(), "0.33333333333333331");
    /// assert_eq!(original_x.get_prec(), Some(53));
    ///
    /// let (x, o) = Float::from_float_prec_ref(&original_x, 100);
    /// assert_eq!(x.to_string(), "0.3333333333333333148296162562474");
    /// assert_eq!(x.get_prec(), Some(100));
    /// assert_eq!(o, Equal);
    ///
    /// let (x, o) = Float::from_float_prec_ref(&original_x, 10);
    /// assert_eq!(x.to_string(), "0.3335");
    /// assert_eq!(x.get_prec(), Some(10));
    /// assert_eq!(o, Greater);
    /// ```
    #[inline]
    pub fn from_float_prec_ref(x: &Float, prec: u64) -> (Float, Ordering) {
        Float::from_float_prec_round_ref(x, prec, Nearest)
    }
}
