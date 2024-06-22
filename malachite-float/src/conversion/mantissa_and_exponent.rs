// Copyright © 2024 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::Float;
use crate::InnerFloat::Finite;
use core::cmp::{
    min,
    Ordering::{self, *},
};
use malachite_base::num::arithmetic::traits::DivisibleByPowerOf2;
use malachite_base::num::basic::floats::PrimitiveFloat;
use malachite_base::num::basic::integers::PrimitiveInt;
use malachite_base::num::conversion::traits::{
    ExactFrom, IntegerMantissaAndExponent, RawMantissaAndExponent, SciMantissaAndExponent,
};
use malachite_base::num::logic::traits::SignificantBits;
use malachite_base::rounding_modes::RoundingMode::{self, *};
use malachite_nz::natural::Natural;
use malachite_nz::platform::Limb;

impl Float {
    /// Returns a [`Float`]'s scientific mantissa and exponent, rounding according to the specified
    /// rounding mode. An [`Ordering`] is also returned, indicating whether the mantissa and
    /// exponent represent a value that is less than, equal to, or greater than the original value.
    ///
    /// When $x$ is positive, we can write $x = 2^{e_s}m_s$, where $e_s$ is an integer and $m_s$ is
    /// a rational number with $1 \leq m_s < 2$. We represent the rational mantissa as a float. The
    /// conversion might not be exact, so we round to the nearest float using the provided rounding
    /// mode. If the rounding mode is `Exact` but the conversion is not exact, `None` is returned.
    /// $$
    /// f(x, r) \approx \left (\frac{x}{2^{\lfloor \log_2 x \rfloor}},
    ///     \lfloor \log_2 x \rfloor\right ).
    /// $$
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(1)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `self.significant_bits()`.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::Pow;
    /// use malachite_base::num::float::NiceFloat;
    /// use malachite_base::rounding_modes::RoundingMode::{self, *};
    /// use malachite_float::Float;
    /// use malachite_nz::natural::Natural;
    /// use std::cmp::Ordering::{self, *};
    ///
    /// let test = |x: Float, rm: RoundingMode, out: Option<(f32, i32, Ordering)>| {
    ///     assert_eq!(
    ///         x.sci_mantissa_and_exponent_round(rm)
    ///             .map(|(m, e, o)| (NiceFloat(m), e, o)),
    ///         out.map(|(m, e, o)| (NiceFloat(m), e, o))
    ///     );
    /// };
    /// test(Float::from(3u32), Floor, Some((1.5, 1, Equal)));
    /// test(Float::from(3u32), Down, Some((1.5, 1, Equal)));
    /// test(Float::from(3u32), Ceiling, Some((1.5, 1, Equal)));
    /// test(Float::from(3u32), Up, Some((1.5, 1, Equal)));
    /// test(Float::from(3u32), Nearest, Some((1.5, 1, Equal)));
    /// test(Float::from(3u32), Exact, Some((1.5, 1, Equal)));
    ///
    /// let x = Float::from(std::f64::consts::PI);
    /// test(x.clone(), Floor, Some((1.5707963, 1, Less)));
    /// test(x.clone(), Down, Some((1.5707963, 1, Less)));
    /// test(x.clone(), Ceiling, Some((1.5707964, 1, Greater)));
    /// test(x.clone(), Up, Some((1.5707964, 1, Greater)));
    /// test(x.clone(), Nearest, Some((1.5707964, 1, Greater)));
    /// test(x.clone(), Exact, None);
    ///
    /// test(
    ///     Float::from(1000000000u32),
    ///     Nearest,
    ///     Some((1.8626451, 29, Equal)),
    /// );
    /// test(
    ///     Float::from(Natural::from(10u32).pow(52)),
    ///     Nearest,
    ///     Some((1.670478, 172, Greater)),
    /// );
    ///
    /// test(Float::from(Natural::from(10u32).pow(52)), Exact, None);
    /// ```
    pub fn sci_mantissa_and_exponent_round<T: PrimitiveFloat>(
        &self,
        rm: RoundingMode,
    ) -> Option<(T, i32, Ordering)> {
        match self {
            Float(Finite {
                exponent,
                significand,
                ..
            }) => significand
                .sci_mantissa_and_exponent_round::<T>(rm)
                .map(|(m, _, o)| {
                    (
                        m,
                        if o == Greater && m == T::ONE {
                            *exponent
                        } else {
                            exponent - 1
                        },
                        o,
                    )
                }),
            _ => None,
        }
    }
}

impl RawMantissaAndExponent<Natural, i32> for Float {
    /// Returns the raw mantissa and exponent of a [`Float`], taking the [`Float`] by value.
    ///
    /// The raw exponent and raw mantissa are the actual bit patterns used to represent the
    /// components of `self`. When `self` is finite and nonzero, the raw mantissa is an integer
    /// whose number of significant bits is a multiple of the limb width, and which is equal to the
    /// absolute value of `self` multiplied by some integer power of 2. The raw exponent is one more
    /// than the floor of the base-2 logarithm of the absolute value of `self`.
    ///
    /// The inverse operation is [`Self::from_raw_mantissa_and_exponent`].
    ///
    /// # Worst-case complexity
    /// Constant time and additional memory.
    ///
    /// # Panics
    /// Panics if the [`Float`] is not finite or not zero.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::Pow;
    /// use malachite_base::num::basic::integers::PrimitiveInt;
    /// use malachite_base::num::basic::traits::One;
    /// use malachite_base::num::conversion::traits::RawMantissaAndExponent;
    /// use malachite_float::Float;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::platform::Limb;
    /// use malachite_q::Rational;
    ///
    /// if Limb::WIDTH == u64::WIDTH {
    ///     let (m, e) = Float::ONE.raw_mantissa_and_exponent();
    ///     assert_eq!(m.to_string(), "9223372036854775808");
    ///     assert_eq!(e, 1);
    ///
    ///     let (m, e) = Float::from(std::f64::consts::PI).raw_mantissa_and_exponent();
    ///     assert_eq!(m.to_string(), "14488038916154245120");
    ///     assert_eq!(e, 2);
    ///
    ///     let (m, e) = Float::from(Natural::from(3u32).pow(50u64)).raw_mantissa_and_exponent();
    ///     assert_eq!(m.to_string(), "202070319366191015160784900114134073344");
    ///     assert_eq!(e, 80);
    ///
    ///     let (m, e) = Float::from_rational_prec(Rational::from(3u32).pow(-50i64), 100)
    ///         .0
    ///         .raw_mantissa_and_exponent();
    ///     assert_eq!(m.to_string(), "286514342137199872022965541161805021184");
    ///     assert_eq!(e, -79);
    /// }
    /// ```
    fn raw_mantissa_and_exponent(self) -> (Natural, i32) {
        if let Float(Finite {
            exponent,
            significand,
            ..
        }) = self
        {
            (significand, exponent)
        } else {
            panic!()
        }
    }

    /// Returns the raw exponent of a [`Float`], taking the [`Float`] by value.
    ///
    /// The raw exponent is one more than the floor of the base-2 logarithm of the absolute value of
    /// `self`.
    ///
    /// # Worst-case complexity
    /// Constant time and additional memory.
    ///
    /// # Panics
    /// Panics if the [`Float`] is not finite or not zero.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::Pow;
    /// use malachite_base::num::basic::traits::One;
    /// use malachite_base::num::conversion::traits::RawMantissaAndExponent;
    /// use malachite_float::Float;
    /// use malachite_nz::natural::Natural;
    /// use malachite_q::Rational;
    ///
    /// assert_eq!(Float::ONE.raw_exponent(), 1);
    /// assert_eq!(Float::from(std::f64::consts::PI).raw_exponent(), 2);
    /// assert_eq!(
    ///     Float::from(Natural::from(3u32).pow(50u64)).raw_exponent(),
    ///     80
    /// );
    /// assert_eq!(
    ///     Float::from_rational_prec(Rational::from(3u32).pow(-50i64), 100)
    ///         .0
    ///         .raw_exponent(),
    ///     -79
    /// );
    /// ```
    fn raw_exponent(self) -> i32 {
        if let Float(Finite { exponent, .. }) = self {
            exponent
        } else {
            panic!()
        }
    }

    /// Constructs a [`Float`] from its raw mantissa and exponent. The resulting [`Float`] is
    /// positive and has the smallest precision possible.
    ///
    /// # Worst-case complexity
    /// Constant time and additional memory.
    ///
    /// # Panics
    /// Panics if `raw_mantissa` is zero, or if its number of significant bits is not divisible by
    /// the limb width.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::Pow;
    /// use malachite_base::num::basic::integers::PrimitiveInt;
    /// use malachite_base::num::conversion::traits::RawMantissaAndExponent;
    /// use malachite_float::Float;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::platform::Limb;
    /// use malachite_q::Rational;
    /// use std::str::FromStr;
    ///
    /// if Limb::WIDTH == u64::WIDTH {
    ///     assert_eq!(
    ///         Float::from_raw_mantissa_and_exponent(Natural::from(9223372036854775808u64), 1),
    ///         1
    ///     );
    ///     assert_eq!(
    ///         Float::from_raw_mantissa_and_exponent(Natural::from(14488038916154245120u64), 2),
    ///         std::f64::consts::PI
    ///     );
    ///     assert_eq!(
    ///         Float::from_raw_mantissa_and_exponent(
    ///             Natural::from_str("202070319366191015160784900114134073344").unwrap(),
    ///             80
    ///         ),
    ///         Natural::from(3u32).pow(50u64)
    ///     );
    ///     assert_eq!(
    ///         Float::from_raw_mantissa_and_exponent(
    ///             Natural::from_str("286514342137199872022965541161805021184").unwrap(),
    ///             -79
    ///         ),
    ///         Float::from_rational_prec(Rational::from(3u32).pow(-50i64), 100).0
    ///     );
    /// }
    /// ```
    fn from_raw_mantissa_and_exponent(raw_mantissa: Natural, raw_exponent: i32) -> Float {
        let bits = raw_mantissa.significant_bits();
        assert_ne!(bits, 0);
        assert!(bits.divisible_by_power_of_2(Limb::LOG_WIDTH));
        let precision = bits - min(raw_mantissa.trailing_zeros().unwrap(), Limb::WIDTH - 1);
        Float(Finite {
            sign: true,
            exponent: raw_exponent,
            significand: raw_mantissa,
            precision,
        })
    }
}

impl<'a> RawMantissaAndExponent<Natural, i32, Float> for &'a Float {
    /// Returns the raw mantissa and exponent of a [`Float`], taking the [`Float`] by reference.
    ///
    /// The raw exponent and raw mantissa are the actual bit patterns used to represent the
    /// components of `self`. When `self` is finite and nonzero, the raw mantissa is an integer
    /// whose number of significant bits is a multiple of the limb width, and which is equal to the
    /// absolute value of `self` multiplied by some integer power of 2. The raw exponent is one more
    /// than the floor of the base-2 logarithm of the absolute value of `self`.
    ///
    /// The inverse operation is [`Float::from_raw_mantissa_and_exponent`].
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `f.significant_bits()`.
    ///
    /// # Panics
    /// Panics if the [`Float`] is not finite or not zero.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::Pow;
    /// use malachite_base::num::basic::integers::PrimitiveInt;
    /// use malachite_base::num::basic::traits::One;
    /// use malachite_base::num::conversion::traits::RawMantissaAndExponent;
    /// use malachite_float::Float;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::platform::Limb;
    /// use malachite_q::Rational;
    ///
    /// if Limb::WIDTH == u64::WIDTH {
    ///     let (m, e) = (&Float::ONE).raw_mantissa_and_exponent();
    ///     assert_eq!(m.to_string(), "9223372036854775808");
    ///     assert_eq!(e, 1);
    ///
    ///     let (m, e) = (&Float::from(std::f64::consts::PI)).raw_mantissa_and_exponent();
    ///     assert_eq!(m.to_string(), "14488038916154245120");
    ///     assert_eq!(e, 2);
    ///
    ///     let (m, e) = (&Float::from(Natural::from(3u32).pow(50u64))).raw_mantissa_and_exponent();
    ///     assert_eq!(m.to_string(), "202070319366191015160784900114134073344");
    ///     assert_eq!(e, 80);
    ///
    ///     let (m, e) = (&Float::from_rational_prec(Rational::from(3u32).pow(-50i64), 100).0)
    ///         .raw_mantissa_and_exponent();
    ///     assert_eq!(m.to_string(), "286514342137199872022965541161805021184");
    ///     assert_eq!(e, -79);
    /// }
    /// ```
    fn raw_mantissa_and_exponent(self) -> (Natural, i32) {
        if let Float(Finite {
            exponent,
            significand,
            ..
        }) = self
        {
            (significand.clone(), *exponent)
        } else {
            panic!()
        }
    }

    /// Returns the raw exponent of a [`Float`], taking the [`Float`] by reference.
    ///
    /// The raw exponent is one more than the floor of the base-2 logarithm of the absolute value of
    /// `self`.
    ///
    /// # Worst-case complexity
    /// Constant time and additional memory.
    ///
    /// # Panics
    /// Panics if the [`Float`] is not finite or not zero.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::Pow;
    /// use malachite_base::num::basic::traits::One;
    /// use malachite_base::num::conversion::traits::RawMantissaAndExponent;
    /// use malachite_float::Float;
    /// use malachite_nz::natural::Natural;
    /// use malachite_q::Rational;
    ///
    /// assert_eq!((&Float::ONE).raw_exponent(), 1);
    /// assert_eq!((&Float::from(std::f64::consts::PI)).raw_exponent(), 2);
    /// assert_eq!(
    ///     (&Float::from(Natural::from(3u32).pow(50u64))).raw_exponent(),
    ///     80
    /// );
    /// assert_eq!(
    ///     (&Float::from_rational_prec(Rational::from(3u32).pow(-50i64), 100).0).raw_exponent(),
    ///     -79
    /// );
    /// ```
    fn raw_exponent(self) -> i32 {
        if let Float(Finite { exponent, .. }) = self {
            *exponent
        } else {
            panic!()
        }
    }

    /// Constructs a [`Float`] from its raw mantissa and exponent. The resulting [`Float`] is
    /// positive and has the smallest precision possible.
    ///
    /// # Worst-case complexity
    /// Constant time and additional memory.
    ///
    /// # Panics
    /// Panics if `raw_mantissa` is zero, or if its number of significant bits is not divisible by
    /// the limb width.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::Pow;
    /// use malachite_base::num::basic::integers::PrimitiveInt;
    /// use malachite_base::num::conversion::traits::RawMantissaAndExponent;
    /// use malachite_float::Float;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::platform::Limb;
    /// use malachite_q::Rational;
    /// use std::str::FromStr;
    ///
    /// if Limb::WIDTH == u64::WIDTH {
    ///     assert_eq!(
    ///         <&Float as RawMantissaAndExponent<_, _, _>>::from_raw_mantissa_and_exponent(
    ///             Natural::from(9223372036854775808u64),
    ///             1
    ///         ),
    ///         1
    ///     );
    ///     assert_eq!(
    ///         <&Float as RawMantissaAndExponent<_, _, _>>::from_raw_mantissa_and_exponent(
    ///             Natural::from(14488038916154245120u64),
    ///             2
    ///         ),
    ///         std::f64::consts::PI
    ///     );
    ///     assert_eq!(
    ///         <&Float as RawMantissaAndExponent<_, _, _>>::from_raw_mantissa_and_exponent(
    ///             Natural::from_str("202070319366191015160784900114134073344").unwrap(),
    ///             80
    ///         ),
    ///         Natural::from(3u32).pow(50u64)
    ///     );
    ///     assert_eq!(
    ///         <&Float as RawMantissaAndExponent<_, _, _>>::from_raw_mantissa_and_exponent(
    ///             Natural::from_str("286514342137199872022965541161805021184").unwrap(),
    ///             -79
    ///         ),
    ///         Float::from_rational_prec(Rational::from(3u32).pow(-50i64), 100).0
    ///     );
    /// }
    /// ```
    #[inline]
    fn from_raw_mantissa_and_exponent(raw_mantissa: Natural, raw_exponent: i32) -> Float {
        Float::from_raw_mantissa_and_exponent(raw_mantissa, raw_exponent)
    }
}

impl IntegerMantissaAndExponent<Natural, i32> for Float {
    /// Returns a [`Float`]'s integer mantissa and exponent, taking the [`Float`] by value.
    ///
    /// When $x$ is finite and nonzero, we can write $x = 2^{e_i}m_i$, where $e_i$ is an integer and
    /// $m_i$ is an odd integer.
    /// $$
    /// f(x) = (\frac{|x|}{2^{e_i}}, e_i),
    /// $$
    /// where $e_i$ is the unique integer such that $x/2^{e_i}$ is an odd integer.
    ///
    /// The inverse operation is
    /// [`from_integer_mantissa_and_exponent`](IntegerMantissaAndExponent::from_integer_mantissa_and_exponent).
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(1)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `self.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `self` is zero or not finite.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::Pow;
    /// use malachite_base::num::basic::traits::One;
    /// use malachite_base::num::conversion::traits::IntegerMantissaAndExponent;
    /// use malachite_float::Float;
    /// use malachite_nz::natural::Natural;
    /// use malachite_q::Rational;
    /// use std::str::FromStr;
    ///
    /// assert_eq!(
    ///     Float::ONE.integer_mantissa_and_exponent(),
    ///     (Natural::ONE, 0)
    /// );
    /// assert_eq!(
    ///     Float::from(std::f64::consts::PI).integer_mantissa_and_exponent(),
    ///     (Natural::from(884279719003555u64), -48)
    /// );
    /// assert_eq!(
    ///     Float::from(Natural::from(3u32).pow(50u64)).integer_mantissa_and_exponent(),
    ///     (Natural::from_str("717897987691852588770249").unwrap(), 0)
    /// );
    /// assert_eq!(
    ///     Float::from_rational_prec(Rational::from(3u32).pow(-50i64), 100)
    ///         .0
    ///         .integer_mantissa_and_exponent(),
    ///     (
    ///         Natural::from_str("1067349099133908271875104088939").unwrap(),
    ///         -179
    ///     )
    /// );
    /// ```
    fn integer_mantissa_and_exponent(self) -> (Natural, i32) {
        if let Float(Finite {
            exponent,
            significand,
            ..
        }) = self
        {
            let zeros = significand.trailing_zeros().unwrap();
            let shifted = significand >> zeros;
            let bits = shifted.significant_bits();
            (shifted, exponent - i32::exact_from(bits))
        } else {
            panic!()
        }
    }

    /// Returns a [`Float`]'s integer exponent, taking the [`Float`] by value.
    ///
    /// When $x$ is finite and nonzero, we can write $x = 2^{e_i}m_i$, where $e_i$ is an integer and
    /// $m_i$ is an odd integer.
    /// $$
    /// f(x) = e_i,
    /// $$
    /// where $e_i$ is the unique integer such that $x/2^{e_i}$ is an odd integer.
    ///
    /// # Worst-case complexity
    /// Constant time and additional memory.
    ///
    /// # Panics
    /// Panics if `self` is zero or not finite.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::Pow;
    /// use malachite_base::num::basic::traits::One;
    /// use malachite_base::num::conversion::traits::IntegerMantissaAndExponent;
    /// use malachite_float::Float;
    /// use malachite_nz::natural::Natural;
    /// use malachite_q::Rational;
    ///
    /// assert_eq!(Float::ONE.integer_exponent(), 0);
    /// assert_eq!(Float::from(std::f64::consts::PI).integer_exponent(), -48);
    /// assert_eq!(
    ///     Float::from(Natural::from(3u32).pow(50u64)).integer_exponent(),
    ///     0
    /// );
    /// assert_eq!(
    ///     Float::from_rational_prec(Rational::from(3u32).pow(-50i64), 100)
    ///         .0
    ///         .integer_exponent(),
    ///     -179
    /// );
    /// ```
    fn integer_exponent(self) -> i32 {
        if let Float(Finite {
            exponent,
            significand,
            ..
        }) = self
        {
            exponent
                - i32::exact_from(
                    significand.significant_bits() - significand.trailing_zeros().unwrap(),
                )
        } else {
            panic!()
        }
    }

    /// Constructs a [`Float`] from its integer mantissa and exponent.
    ///
    /// When $x$ is finite and nonzero, we can write $x = 2^{e_i}m_i$, where $e_i$ is an integer and
    /// $m_i$ is an odd integer.
    ///
    /// $$
    /// f(x) = 2^{e_i}m_i.
    /// $$
    ///
    /// The input does not have to be reduced; that is, the mantissa does not have to be odd.
    ///
    /// The result is an [`Option`], but for this trait implementation the result is always `Some`.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is
    /// `integer_mantissa.significant_bits()`.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::Pow;
    /// use malachite_base::num::basic::traits::One;
    /// use malachite_base::num::conversion::traits::IntegerMantissaAndExponent;
    /// use malachite_float::Float;
    /// use malachite_nz::natural::Natural;
    /// use malachite_q::Rational;
    /// use std::str::FromStr;
    ///
    /// assert_eq!(
    ///     Float::from_integer_mantissa_and_exponent(Natural::ONE, 0).unwrap(),
    ///     1
    /// );
    /// assert_eq!(
    ///     Float::from_integer_mantissa_and_exponent(Natural::from(884279719003555u64), -48)
    ///         .unwrap(),
    ///     std::f64::consts::PI
    /// );
    /// assert_eq!(
    ///     Float::from_integer_mantissa_and_exponent(
    ///         Natural::from_str("717897987691852588770249").unwrap(),
    ///         0
    ///     )
    ///     .unwrap(),
    ///     Natural::from(3u32).pow(50u64)
    /// );
    /// assert_eq!(
    ///     Float::from_integer_mantissa_and_exponent(
    ///         Natural::from_str("1067349099133908271875104088939").unwrap(),
    ///         -179
    ///     )
    ///     .unwrap(),
    ///     Float::from_rational_prec(Rational::from(3u32).pow(-50i64), 100).0
    /// );
    /// ```
    fn from_integer_mantissa_and_exponent(
        integer_mantissa: Natural,
        integer_exponent: i32,
    ) -> Option<Float> {
        Some(Float::from(integer_mantissa) << integer_exponent)
    }
}

impl<'a> IntegerMantissaAndExponent<Natural, i32, Float> for &'a Float {
    /// Returns a [`Float`]'s integer mantissa and exponent, taking the [`Float`] by reference.
    ///
    /// When $x$ is finite and nonzero, we can write $x = 2^{e_i}m_i$, where $e_i$ is an integer and
    /// $m_i$ is an odd integer.
    /// $$
    /// f(x) = (\frac{|x|}{2^{e_i}}, e_i),
    /// $$
    /// where $e_i$ is the unique integer such that $x/2^{e_i}$ is an odd integer.
    ///
    /// The inverse operation is
    /// [`from_integer_mantissa_and_exponent`](IntegerMantissaAndExponent::from_integer_mantissa_and_exponent).
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `self.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `self` is zero or not finite.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::Pow;
    /// use malachite_base::num::basic::traits::One;
    /// use malachite_base::num::conversion::traits::IntegerMantissaAndExponent;
    /// use malachite_float::Float;
    /// use malachite_nz::natural::Natural;
    /// use malachite_q::Rational;
    /// use std::str::FromStr;
    ///
    /// assert_eq!(
    ///     (&Float::ONE).integer_mantissa_and_exponent(),
    ///     (Natural::ONE, 0)
    /// );
    /// assert_eq!(
    ///     (&Float::from(std::f64::consts::PI)).integer_mantissa_and_exponent(),
    ///     (Natural::from(884279719003555u64), -48)
    /// );
    /// assert_eq!(
    ///     (&Float::from(Natural::from(3u32).pow(50u64))).integer_mantissa_and_exponent(),
    ///     (Natural::from_str("717897987691852588770249").unwrap(), 0)
    /// );
    /// assert_eq!(
    ///     (&Float::from_rational_prec(Rational::from(3u32).pow(-50i64), 100).0)
    ///         .integer_mantissa_and_exponent(),
    ///     (
    ///         Natural::from_str("1067349099133908271875104088939").unwrap(),
    ///         -179
    ///     )
    /// );
    /// ```
    fn integer_mantissa_and_exponent(self) -> (Natural, i32) {
        if let Float(Finite {
            exponent,
            significand,
            ..
        }) = self
        {
            let zeros = significand.trailing_zeros().unwrap();
            let shifted = significand >> zeros;
            let bits = shifted.significant_bits();
            (shifted, exponent - i32::exact_from(bits))
        } else {
            panic!()
        }
    }

    /// Returns a [`Float`]'s integer exponent, taking the [`Float`] by reference.
    ///
    /// When $x$ is finite and nonzero, we can write $x = 2^{e_i}m_i$, where $e_i$ is an integer and
    /// $m_i$ is an odd integer.
    /// $$
    /// f(x) = e_i,
    /// $$
    /// where $e_i$ is the unique integer such that $x/2^{e_i}$ is an odd integer.
    ///
    /// # Worst-case complexity
    /// Constant time and additional memory.
    ///
    /// # Panics
    /// Panics if `self` is zero or not finite.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::Pow;
    /// use malachite_base::num::basic::traits::One;
    /// use malachite_base::num::conversion::traits::IntegerMantissaAndExponent;
    /// use malachite_float::Float;
    /// use malachite_nz::natural::Natural;
    /// use malachite_q::Rational;
    ///
    /// assert_eq!((&Float::ONE).integer_exponent(), 0);
    /// assert_eq!((&Float::from(std::f64::consts::PI)).integer_exponent(), -48);
    /// assert_eq!(
    ///     (&Float::from(Natural::from(3u32).pow(50u64))).integer_exponent(),
    ///     0
    /// );
    /// assert_eq!(
    ///     (&Float::from_rational_prec(Rational::from(3u32).pow(-50i64), 100).0)
    ///         .integer_exponent(),
    ///     -179
    /// );
    /// ```
    fn integer_exponent(self) -> i32 {
        if let Float(Finite {
            exponent,
            significand,
            ..
        }) = self
        {
            exponent
                - i32::exact_from(
                    significand.significant_bits() - significand.trailing_zeros().unwrap(),
                )
        } else {
            panic!()
        }
    }

    /// Constructs a [`Float`] from its integer mantissa and exponent.
    ///
    /// When $x$ is finite and nonzero, we can write $x = 2^{e_i}m_i$, where $e_i$ is an integer and
    /// $m_i$ is an odd integer.
    ///
    /// $$
    /// f(x) = 2^{e_i}m_i.
    /// $$
    ///
    /// The input does not have to be reduced; that is, the mantissa does not have to be odd.
    ///
    /// The result is an [`Option`], but for this trait implementation the result is always `Some`.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is
    /// `integer_mantissa.significant_bits()`.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::Pow;
    /// use malachite_base::num::basic::traits::One;
    /// use malachite_base::num::conversion::traits::IntegerMantissaAndExponent;
    /// use malachite_float::Float;
    /// use malachite_nz::natural::Natural;
    /// use malachite_q::Rational;
    /// use std::str::FromStr;
    ///
    /// assert_eq!(
    ///     <&Float as IntegerMantissaAndExponent<_, _, _>>::from_integer_mantissa_and_exponent(
    ///         Natural::ONE,
    ///         0
    ///     )
    ///     .unwrap(),
    ///     1
    /// );
    /// assert_eq!(
    ///     <&Float as IntegerMantissaAndExponent<_, _, _>>::from_integer_mantissa_and_exponent(
    ///         Natural::from(884279719003555u64),
    ///         -48
    ///     )
    ///     .unwrap(),
    ///     std::f64::consts::PI
    /// );
    /// assert_eq!(
    ///     <&Float as IntegerMantissaAndExponent<_, _, _>>::from_integer_mantissa_and_exponent(
    ///         Natural::from_str("717897987691852588770249").unwrap(),
    ///         0
    ///     )
    ///     .unwrap(),
    ///     Natural::from(3u32).pow(50u64)
    /// );
    /// assert_eq!(
    ///     <&Float as IntegerMantissaAndExponent<_, _, _>>::from_integer_mantissa_and_exponent(
    ///         Natural::from_str("1067349099133908271875104088939").unwrap(),
    ///         -179
    ///     )
    ///     .unwrap(),
    ///     Float::from_rational_prec(Rational::from(3u32).pow(-50i64), 100).0
    /// );
    /// ```
    fn from_integer_mantissa_and_exponent(
        integer_mantissa: Natural,
        integer_exponent: i32,
    ) -> Option<Float> {
        Some(Float::from(integer_mantissa) << integer_exponent)
    }
}

impl SciMantissaAndExponent<Float, i32> for Float {
    /// Returns a [`Float`]'s scientific mantissa and exponent, taking the [`Float`] by value.
    ///
    /// When $x$ is finite and nonzero, we can write $|x| = 2^{e_s}m_s$, where $e_s$ is an integer
    /// and $m_s$ is a rational number with $1 \leq m_s < 2$. We represent the rational mantissa as
    /// a [`Float`].
    /// $$
    /// f(x) = (\frac{|x|}{2^{\lfloor \log_2 |x| \rfloor}}, \lfloor \log_2 |x| \rfloor).
    /// $$
    ///
    /// # Worst-case complexity
    /// Constant time and additional memory.
    ///
    /// # Panics
    /// Panics if `self` is zero or not finite.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::Pow;
    /// use malachite_base::num::basic::traits::One;
    /// use malachite_base::num::conversion::traits::SciMantissaAndExponent;
    /// use malachite_float::Float;
    /// use malachite_nz::natural::Natural;
    /// use malachite_q::Rational;
    ///
    /// assert_eq!(Float::ONE.sci_mantissa_and_exponent(), (Float::ONE, 0));
    ///
    /// let (m, e) = Float::from(std::f64::consts::PI).sci_mantissa_and_exponent();
    /// assert_eq!(m.to_string(), "1.5707963267948966");
    /// assert_eq!(e, 1);
    ///
    /// let (m, e) = Float::from(Natural::from(3u32).pow(50u64)).sci_mantissa_and_exponent();
    /// assert_eq!(m.to_string(), "1.187662594419065093441695");
    /// assert_eq!(e, 79);
    ///
    /// let (m, e) = Float::from_rational_prec(Rational::from(3u32).pow(-50i64), 100)
    ///     .0
    ///     .sci_mantissa_and_exponent();
    /// assert_eq!(m.to_string(), "1.683979953059212693885095551367");
    /// assert_eq!(e, -80);
    /// ```
    #[inline]
    fn sci_mantissa_and_exponent(mut self) -> (Float, i32) {
        if let Float(Finite { sign, exponent, .. }) = &mut self {
            let old_exponent = *exponent;
            *exponent = 1;
            *sign = true;
            (self, old_exponent - 1)
        } else {
            panic!()
        }
    }

    /// Returns a [`Float`]'s scientific exponent, taking the [`Float`] by value.
    ///
    /// When $x$ is finite and nonzero, we can write $|x| = 2^{e_s}m_s$, where $e_s$ is an integer
    /// and $m_s$ is a rational number with $1 \leq m_s < 2$.
    /// $$
    /// f(x) = \lfloor \log_2 |x| \rfloor.
    /// $$
    ///
    /// # Worst-case complexity
    /// Constant time and additional memory.
    ///
    /// # Panics
    /// Panics if `self` is zero or not finite.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::Pow;
    /// use malachite_base::num::basic::traits::One;
    /// use malachite_base::num::conversion::traits::SciMantissaAndExponent;
    /// use malachite_float::Float;
    /// use malachite_nz::natural::Natural;
    /// use malachite_q::Rational;
    ///
    /// assert_eq!(Float::ONE.sci_exponent(), 0);
    /// assert_eq!(Float::from(std::f64::consts::PI).sci_exponent(), 1);
    /// assert_eq!(
    ///     Float::from(Natural::from(3u32).pow(50u64)).sci_exponent(),
    ///     79
    /// );
    /// assert_eq!(
    ///     Float::from_rational_prec(Rational::from(3u32).pow(-50i64), 100)
    ///         .0
    ///         .sci_exponent(),
    ///     -80
    /// );
    /// ```
    #[inline]
    fn sci_exponent(self) -> i32 {
        self.raw_exponent() - 1
    }

    /// Constructs a [`Float`] from its scientific mantissa and exponent.
    ///
    /// When $x$ is finite and nonzero, we can write $|x| = 2^{e_s}m_s$, where $e_s$ is an integer
    /// and $m_s$ is a rational number with $1 \leq m_s < 2$.
    ///
    /// $$
    /// f(x) = 2^{e_i}m_i.
    /// $$
    ///
    /// If the mantissa is zero or not finite, this function panics. If it is finite but not in the
    /// interval $[1, 2)$, this function returns `None`.
    ///
    /// # Worst-case complexity
    /// Constant time and additional memory.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::Pow;
    /// use malachite_base::num::basic::traits::One;
    /// use malachite_base::num::conversion::traits::{FromStringBase, SciMantissaAndExponent};
    /// use malachite_float::Float;
    /// use malachite_nz::natural::Natural;
    /// use malachite_q::Rational;
    ///
    /// assert_eq!(
    ///     Float::from_sci_mantissa_and_exponent(Float::ONE, 0).unwrap(),
    ///     1
    /// );
    /// assert_eq!(
    ///     Float::from_sci_mantissa_and_exponent(
    ///         Float::from_string_base(16, "0x1.921fb54442d18#53").unwrap(),
    ///         1
    ///     )
    ///     .unwrap(),
    ///     std::f64::consts::PI
    /// );
    /// assert_eq!(
    ///     Float::from_sci_mantissa_and_exponent(
    ///         Float::from_string_base(16, "0x1.300aa7e1b65fa13bc792#80").unwrap(),
    ///         79
    ///     )
    ///     .unwrap(),
    ///     Natural::from(3u32).pow(50u64)
    /// );
    /// assert_eq!(
    ///     Float::from_sci_mantissa_and_exponent(
    ///         Float::from_string_base(16, "0x1.af194f6982497a23f9dc546d6#100").unwrap(),
    ///         -80
    ///     )
    ///     .unwrap(),
    ///     Float::from_rational_prec(Rational::from(3u32).pow(-50i64), 100).0
    /// );
    /// ```
    fn from_sci_mantissa_and_exponent(mut sci_mantissa: Float, sci_exponent: i32) -> Option<Float> {
        assert!(sci_mantissa.is_finite());
        assert!(!sci_mantissa.is_zero());
        if sci_mantissa.is_sign_negative() || (&sci_mantissa).raw_exponent() != 1 {
            return None;
        }
        if let Float(Finite { exponent, .. }) = &mut sci_mantissa {
            *exponent = sci_exponent + 1;
        } else {
            panic!()
        }
        Some(sci_mantissa)
    }
}

impl<'a> SciMantissaAndExponent<Float, i32, Float> for &'a Float {
    /// Returns a [`Float`]'s scientific mantissa and exponent, taking the [`Float`] by reference.
    ///
    /// When $x$ is finite and nonzero, we can write $|x| = 2^{e_s}m_s$, where $e_s$ is an integer
    /// and $m_s$ is a rational number with $1 \leq m_s < 2$. We represent the rational mantissa as
    /// a [`Float`].
    /// $$
    /// f(x) = (\frac{|x|}{2^{\lfloor \log_2 |x| \rfloor}}, \lfloor \log_2 |x| \rfloor).
    /// $$
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `self.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `self` is zero or not finite.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::Pow;
    /// use malachite_base::num::basic::traits::One;
    /// use malachite_base::num::conversion::traits::SciMantissaAndExponent;
    /// use malachite_float::Float;
    /// use malachite_nz::natural::Natural;
    /// use malachite_q::Rational;
    ///
    /// assert_eq!((&Float::ONE).sci_mantissa_and_exponent(), (Float::ONE, 0));
    ///
    /// let (m, e): (Float, i32) = (&Float::from(std::f64::consts::PI)).sci_mantissa_and_exponent();
    /// assert_eq!(m.to_string(), "1.5707963267948966");
    /// assert_eq!(e, 1);
    ///
    /// let (m, e): (Float, i32) =
    ///     (&Float::from(Natural::from(3u32).pow(50u64))).sci_mantissa_and_exponent();
    /// assert_eq!(m.to_string(), "1.187662594419065093441695");
    /// assert_eq!(e, 79);
    ///
    /// let (m, e): (Float, i32) =
    ///     (&Float::from_rational_prec(Rational::from(3u32).pow(-50i64), 100).0)
    ///         .sci_mantissa_and_exponent();
    /// assert_eq!(m.to_string(), "1.683979953059212693885095551367");
    /// assert_eq!(e, -80);
    /// ```
    #[inline]
    fn sci_mantissa_and_exponent(self) -> (Float, i32) {
        if let Float(Finite {
            exponent,
            precision,
            significand,
            ..
        }) = self
        {
            (
                Float(Finite {
                    sign: true,
                    exponent: 1,
                    precision: *precision,
                    significand: significand.clone(),
                }),
                exponent - 1,
            )
        } else {
            panic!()
        }
    }

    /// Returns a [`Float`]'s scientific exponent, taking the [`Float`] by reference.
    ///
    /// When $x$ is finite and nonzero, we can write $|x| = 2^{e_s}m_s$, where $e_s$ is an integer
    /// and $m_s$ is a rational number with $1 \leq m_s < 2$.
    /// $$
    /// f(x) = \lfloor \log_2 |x| \rfloor.
    /// $$
    ///
    /// # Worst-case complexity
    /// Constant time and additional memory.
    ///
    /// # Panics
    /// Panics if `self` is zero or not finite.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::Pow;
    /// use malachite_base::num::basic::traits::One;
    /// use malachite_base::num::conversion::traits::SciMantissaAndExponent;
    /// use malachite_float::Float;
    /// use malachite_nz::natural::Natural;
    /// use malachite_q::Rational;
    ///
    /// assert_eq!(
    ///     <&Float as SciMantissaAndExponent<Float, _, _>>::sci_exponent(&Float::ONE),
    ///     0
    /// );
    /// assert_eq!(
    ///     <&Float as SciMantissaAndExponent<Float, _, _>>::sci_exponent(&Float::from(
    ///         std::f64::consts::PI
    ///     )),
    ///     1
    /// );
    /// assert_eq!(
    ///     <&Float as SciMantissaAndExponent<Float, _, _>>::sci_exponent(&Float::from(
    ///         Natural::from(3u32).pow(50u64)
    ///     )),
    ///     79
    /// );
    /// assert_eq!(
    ///     <&Float as SciMantissaAndExponent<Float, _, _>>::sci_exponent(
    ///         &Float::from_rational_prec(Rational::from(3u32).pow(-50i64), 100).0
    ///     ),
    ///     -80
    /// );
    /// ```
    #[inline]
    fn sci_exponent(self) -> i32 {
        self.raw_exponent() - 1
    }

    /// Constructs a [`Float`] from its scientific mantissa and exponent.
    ///
    /// When $x$ is finite and nonzero, we can write $|x| = 2^{e_s}m_s$, where $e_s$ is an integer
    /// and $m_s$ is a rational number with $1 \leq m_s < 2$.
    ///
    /// $$
    /// f(x) = 2^{e_i}m_i.
    /// $$
    ///
    /// If the mantissa is zero or not finite, this function panics. If it is finite but not in the
    /// interval $[1, 2)$, this function returns `None`.
    ///
    /// # Worst-case complexity
    /// Constant time and additional memory.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::Pow;
    /// use malachite_base::num::basic::traits::One;
    /// use malachite_base::num::conversion::traits::{FromStringBase, SciMantissaAndExponent};
    /// use malachite_float::Float;
    /// use malachite_nz::natural::Natural;
    /// use malachite_q::Rational;
    ///
    /// assert_eq!(
    ///     Float::from_sci_mantissa_and_exponent(Float::ONE, 0).unwrap(),
    ///     1
    /// );
    /// assert_eq!(
    ///     <&Float as SciMantissaAndExponent<Float, _, _>>::from_sci_mantissa_and_exponent(
    ///         Float::from_string_base(16, "0x1.921fb54442d18#53").unwrap(),
    ///         1
    ///     )
    ///     .unwrap(),
    ///     std::f64::consts::PI
    /// );
    /// assert_eq!(
    ///     <&Float as SciMantissaAndExponent<Float, _, _>>::from_sci_mantissa_and_exponent(
    ///         Float::from_string_base(16, "0x1.300aa7e1b65fa13bc792#80").unwrap(),
    ///         79
    ///     )
    ///     .unwrap(),
    ///     Natural::from(3u32).pow(50u64)
    /// );
    /// assert_eq!(
    ///     <&Float as SciMantissaAndExponent<Float, _, _>>::from_sci_mantissa_and_exponent(
    ///         Float::from_string_base(16, "0x1.af194f6982497a23f9dc546d6#100").unwrap(),
    ///         -80
    ///     )
    ///     .unwrap(),
    ///     Float::from_rational_prec(Rational::from(3u32).pow(-50i64), 100).0
    /// );
    /// ```
    #[inline]
    fn from_sci_mantissa_and_exponent(sci_mantissa: Float, sci_exponent: i32) -> Option<Float> {
        Float::from_sci_mantissa_and_exponent(sci_mantissa, sci_exponent)
    }
}

macro_rules! impl_mantissa_and_exponent {
    ($t:ident) => {
        impl<'a> SciMantissaAndExponent<$t, i32, Float> for &'a Float {
            /// Returns a [`Float`]'s scientific mantissa and exponent, taking the [`Float`] by
            /// value.
            ///
            /// When $x$ is finite and nonzero, we can write $|x| = 2^{e_s}m_s$, where $e_s$ is an
            /// integer and $m_s$ is a rational number with $1 \leq m_s < 2$. We represent the
            /// rational mantissa as a primitive float. The conversion might not be exact, so we
            /// round to the nearest float using the `Nearest` rounding mode. To use other rounding
            /// modes, use
            /// [`sci_mantissa_and_exponent_round`](Float::sci_mantissa_and_exponent_round).
            /// $$
            /// f(x) \approx (\frac{|x|}{2^{\lfloor \log_2 |x| \rfloor}},
            /// \lfloor \log_2 |x| \rfloor).
            /// $$
            ///
            /// # Worst-case complexity
            /// $T(n) = O(n)$
            ///
            /// $M(n) = O(1)$
            ///
            /// where $T$ is time, $M$ is additional memory, and $n$ is `self.significant_bits()`.
            ///
            /// # Panics
            /// Panics if `self` is zero or not finite.
            ///
            /// # Examples
            /// See [here](super::mantissa_and_exponent#sci_mantissa_and_exponent).
            #[inline]
            fn sci_mantissa_and_exponent(self) -> ($t, i32) {
                let (m, e, _) = self.sci_mantissa_and_exponent_round(Nearest).unwrap();
                (m, e)
            }

            /// Constructs a [`Float`] from its scientific mantissa and exponent.
            ///
            /// When $x$ is finite and nonzero, we can write $|x| = 2^{e_s}m_s$, where $e_s$ is an
            /// integer and $m_s$ is a rational number with $1 \leq m_s < 2$.
            ///
            /// $$
            /// f(x) = 2^{e_i}m_i.
            /// $$
            ///
            /// If the mantissa is zero or not finite, this function panics. If it is finite but not
            /// in the interval $[1, 2)$, this function returns `None`.
            ///
            /// # Worst-case complexity
            /// Constant time and additional memory.
            ///
            /// # Examples
            /// See [here](super::mantissa_and_exponent#from_sci_mantissa_and_exponent).
            #[allow(clippy::manual_range_contains)]
            #[inline]
            fn from_sci_mantissa_and_exponent(
                sci_mantissa: $t,
                sci_exponent: i32,
            ) -> Option<Float> {
                assert!(sci_mantissa.is_finite());
                assert_ne!(sci_mantissa, 0.0);
                if sci_mantissa < 1.0 || sci_mantissa >= 2.0 {
                    None
                } else {
                    let m = sci_mantissa.integer_mantissa();
                    Some(
                        Float::from(m)
                            << (sci_exponent - i32::exact_from(m.significant_bits()) + 1),
                    )
                }
            }
        }
    };
}
apply_to_primitive_floats!(impl_mantissa_and_exponent);
