// Copyright © 2025 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::Rational;
use core::cmp::Ordering::{self, *};
use malachite_base::num::arithmetic::traits::DivRound;
use malachite_base::num::basic::floats::PrimitiveFloat;
use malachite_base::num::conversion::traits::{
    ExactFrom, IntegerMantissaAndExponent, SciMantissaAndExponent, WrappingFrom,
};
use malachite_base::num::logic::traits::{BitAccess, SignificantBits};
use malachite_base::rounding_modes::RoundingMode::{self, *};

impl Rational {
    /// Returns a [`Rational`]'s scientific mantissa and exponent, taking the [`Rational`] by value.
    /// An [`Ordering`] is also returned, indicating whether the returned mantissa and exponent
    /// represent a value that is less than, equal to, or greater than the absolute value of the
    /// [`Rational`].
    ///
    /// The [`Rational`]'s sign is ignored. This means that, for example, that rounding using
    /// `Floor` is  equivalent to rounding using `Down`, even if the [`Rational`] is negative.
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
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `self.significant_bits()`.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::float::NiceFloat;
    /// use malachite_base::rounding_modes::RoundingMode::{self, *};
    /// use malachite_q::Rational;
    /// use std::cmp::Ordering::{self, *};
    ///
    /// let test = |n: Rational, rm: RoundingMode, out: Option<(f32, i64, Ordering)>| {
    ///     assert_eq!(
    ///         n.sci_mantissa_and_exponent_round(rm)
    ///             .map(|(m, e, o)| (NiceFloat(m), e, o)),
    ///         out.map(|(m, e, o)| (NiceFloat(m), e, o))
    ///     );
    /// };
    /// test(Rational::from(3u32), Down, Some((1.5, 1, Equal)));
    /// test(Rational::from(3u32), Ceiling, Some((1.5, 1, Equal)));
    /// test(Rational::from(3u32), Up, Some((1.5, 1, Equal)));
    /// test(Rational::from(3u32), Nearest, Some((1.5, 1, Equal)));
    /// test(Rational::from(3u32), Exact, Some((1.5, 1, Equal)));
    ///
    /// test(
    ///     Rational::from_signeds(1, 3),
    ///     Floor,
    ///     Some((1.3333333, -2, Less)),
    /// );
    /// test(
    ///     Rational::from_signeds(1, 3),
    ///     Down,
    ///     Some((1.3333333, -2, Less)),
    /// );
    /// test(
    ///     Rational::from_signeds(1, 3),
    ///     Ceiling,
    ///     Some((1.3333334, -2, Greater)),
    /// );
    /// test(
    ///     Rational::from_signeds(1, 3),
    ///     Up,
    ///     Some((1.3333334, -2, Greater)),
    /// );
    /// test(
    ///     Rational::from_signeds(1, 3),
    ///     Nearest,
    ///     Some((1.3333334, -2, Greater)),
    /// );
    /// test(Rational::from_signeds(1, 3), Exact, None);
    ///
    /// test(
    ///     Rational::from_signeds(-1, 3),
    ///     Floor,
    ///     Some((1.3333333, -2, Less)),
    /// );
    /// test(
    ///     Rational::from_signeds(-1, 3),
    ///     Down,
    ///     Some((1.3333333, -2, Less)),
    /// );
    /// test(
    ///     Rational::from_signeds(-1, 3),
    ///     Ceiling,
    ///     Some((1.3333334, -2, Greater)),
    /// );
    /// test(
    ///     Rational::from_signeds(-1, 3),
    ///     Up,
    ///     Some((1.3333334, -2, Greater)),
    /// );
    /// test(
    ///     Rational::from_signeds(-1, 3),
    ///     Nearest,
    ///     Some((1.3333334, -2, Greater)),
    /// );
    /// test(Rational::from_signeds(-1, 3), Exact, None);
    /// ```
    pub fn sci_mantissa_and_exponent_round<T: PrimitiveFloat>(
        mut self,
        rm: RoundingMode,
    ) -> Option<(T, i64, Ordering)> {
        assert!(self != 0);
        let mut exponent = i64::exact_from(self.numerator.significant_bits())
            - i64::exact_from(self.denominator.significant_bits());
        if self.numerator.cmp_normalized(&self.denominator) == Less {
            exponent -= 1;
        }
        self >>= exponent - i64::wrapping_from(T::MANTISSA_WIDTH);
        let (n, d) = self.into_numerator_and_denominator();
        if rm == Exact && d != 1u32 {
            return None;
        }
        let (mut mantissa, o) = n.div_round(d, rm);
        let mut bits = mantissa.significant_bits();
        if bits > T::MANTISSA_WIDTH + 1 {
            bits -= 1;
            mantissa >>= 1;
            exponent += 1;
        }
        assert_eq!(bits, T::MANTISSA_WIDTH + 1);
        mantissa.clear_bit(T::MANTISSA_WIDTH);
        Some((
            T::from_raw_mantissa_and_exponent(
                u64::exact_from(&mantissa),
                u64::wrapping_from(T::MAX_EXPONENT),
            ),
            exponent,
            o,
        ))
    }

    /// Returns a [`Rational`]'s scientific mantissa and exponent, taking the [`Rational`] by
    /// reference. An [`Ordering`] is also returned, indicating whether the returned mantissa and
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
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `self.significant_bits()`.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::float::NiceFloat;
    /// use malachite_base::rounding_modes::RoundingMode::{self, *};
    /// use malachite_q::Rational;
    /// use std::cmp::Ordering::{self, *};
    ///
    /// let test = |n: Rational, rm: RoundingMode, out: Option<(f32, i64, Ordering)>| {
    ///     assert_eq!(
    ///         n.sci_mantissa_and_exponent_round_ref(rm)
    ///             .map(|(m, e, o)| (NiceFloat(m), e, o)),
    ///         out.map(|(m, e, o)| (NiceFloat(m), e, o))
    ///     );
    /// };
    /// test(Rational::from(3u32), Down, Some((1.5, 1, Equal)));
    /// test(Rational::from(3u32), Ceiling, Some((1.5, 1, Equal)));
    /// test(Rational::from(3u32), Up, Some((1.5, 1, Equal)));
    /// test(Rational::from(3u32), Nearest, Some((1.5, 1, Equal)));
    /// test(Rational::from(3u32), Exact, Some((1.5, 1, Equal)));
    ///
    /// test(
    ///     Rational::from_signeds(1, 3),
    ///     Floor,
    ///     Some((1.3333333, -2, Less)),
    /// );
    /// test(
    ///     Rational::from_signeds(1, 3),
    ///     Down,
    ///     Some((1.3333333, -2, Less)),
    /// );
    /// test(
    ///     Rational::from_signeds(1, 3),
    ///     Ceiling,
    ///     Some((1.3333334, -2, Greater)),
    /// );
    /// test(
    ///     Rational::from_signeds(1, 3),
    ///     Up,
    ///     Some((1.3333334, -2, Greater)),
    /// );
    /// test(
    ///     Rational::from_signeds(1, 3),
    ///     Nearest,
    ///     Some((1.3333334, -2, Greater)),
    /// );
    /// test(Rational::from_signeds(1, 3), Exact, None);
    /// ```
    pub fn sci_mantissa_and_exponent_round_ref<T: PrimitiveFloat>(
        &self,
        rm: RoundingMode,
    ) -> Option<(T, i64, Ordering)> {
        assert!(*self != 0);
        let mut exponent = i64::exact_from(self.numerator.significant_bits())
            - i64::exact_from(self.denominator.significant_bits());
        if self.numerator.cmp_normalized(&self.denominator) == Less {
            exponent -= 1;
        }
        let x = self >> (exponent - i64::wrapping_from(T::MANTISSA_WIDTH));
        let (n, d) = x.into_numerator_and_denominator();
        if rm == Exact && d != 1u32 {
            return None;
        }
        let (mut mantissa, o) = n.div_round(d, rm);
        let mut bits = mantissa.significant_bits();
        if bits > T::MANTISSA_WIDTH + 1 {
            bits -= 1;
            mantissa >>= 1;
            exponent += 1;
        }
        assert_eq!(bits, T::MANTISSA_WIDTH + 1);
        mantissa.clear_bit(T::MANTISSA_WIDTH);
        Some((
            T::from_raw_mantissa_and_exponent(
                u64::exact_from(&mantissa),
                u64::wrapping_from(T::MAX_EXPONENT),
            ),
            exponent,
            o,
        ))
    }
}

macro_rules! impl_mantissa_and_exponent {
    ($t:ident) => {
        impl SciMantissaAndExponent<$t, i64> for Rational {
            /// Returns a [`Rational`]'s scientific mantissa and exponent, taking the [`Rational`]
            /// by value.
            ///
            /// When $x$ is positive, we can write $x = 2^{e_s}m_s$, where $e_s$ is an integer and
            /// $m_s$ is a rational number with $1 \leq m_s < 2$. We represent the rational mantissa
            /// as a float. The conversion might not be exact, so we round to the nearest float
            /// using the `Nearest` rounding mode. To use other rounding modes, use
            /// [`sci_mantissa_and_exponent_round`](Rational::sci_mantissa_and_exponent_round).
            /// $$
            /// f(x) \approx (\frac{x}{2^{\lfloor \log_2 x \rfloor}}, \lfloor \log_2 x \rfloor).
            /// $$
            ///
            /// # Worst-case complexity
            /// $T(n) = O(n \log n \log\log n)$
            ///
            /// $M(n) = O(n \log n)$
            ///
            /// where $T$ is time, $M$ is additional memory, and $n$ is `self.significant_bits()`.
            ///
            /// # Examples
            /// See [here](super::mantissa_and_exponent#sci_mantissa_and_exponent).
            #[inline]
            fn sci_mantissa_and_exponent(self) -> ($t, i64) {
                let (m, e, _) = self.sci_mantissa_and_exponent_round(Nearest).unwrap();
                (m, e)
            }

            /// Returns a [`Rational`]'s scientific exponent, taking the [`Rational`] by value.
            ///
            /// When $x$ is positive, we can write $x = 2^{e_s}m_s$, where $e_s$ is an integer and
            /// $m_s$ is a rational number with $1 \leq m_s < 2$. We represent the rational mantissa
            /// as a float. The conversion might not be exact, so we round to the nearest float
            /// using the `Nearest` rounding mode. To use other rounding modes, use
            /// [`sci_mantissa_and_exponent_round`](Rational::sci_mantissa_and_exponent_round).
            /// $$
            /// f(x) \approx \lfloor \log_2 x \rfloor.
            /// $$
            ///
            /// # Worst-case complexity
            /// $T(n) = O(n \log n \log\log n)$
            ///
            /// $M(n) = O(n \log n)$
            ///
            /// where $T$ is time, $M$ is additional memory, and $n$ is `self.significant_bits()`.
            ///
            /// # Examples
            /// See [here](super::mantissa_and_exponent#sci_exponent).
            fn sci_exponent(mut self) -> i64 {
                assert!(self != 0);
                let mut exponent = i64::exact_from(self.numerator.significant_bits())
                    - i64::exact_from(self.denominator.significant_bits());
                if self.numerator.cmp_normalized(&self.denominator) == Less {
                    exponent -= 1;
                }
                self >>= exponent - i64::wrapping_from($t::MANTISSA_WIDTH);
                let (n, d) = self.into_numerator_and_denominator();
                if n.div_round(d, Nearest).0.significant_bits() > $t::MANTISSA_WIDTH + 1 {
                    exponent + 1
                } else {
                    exponent
                }
            }

            /// Constructs a [`Rational`] from its scientific mantissa and exponent.
            ///
            /// When $x$ is positive, we can write $x = 2^{e_s}m_s$, where $e_s$ is an integer and
            /// $m_s$ is a rational number with $1 \leq m_s < 2$. Here, the rational mantissa is
            /// provided as a float. If the mantissa is outside the range $[1, 2)$, `None` is
            /// returned.
            ///
            /// All finite floats can be represented using [`Rational`]s, so no rounding is needed.
            ///
            /// $$
            /// f(x) \approx 2^{e_s}m_s.
            /// $$
            ///
            /// # Worst-case complexity
            /// $T(n) = O(n)$
            ///
            /// $M(n) = O(n)$
            ///
            /// where $T$ is time, $M$ is additional memory, and $n$ is `sci_exponent`.
            #[allow(clippy::manual_range_contains)]
            #[inline]
            fn from_sci_mantissa_and_exponent(
                sci_mantissa: $t,
                sci_exponent: i64,
            ) -> Option<Rational> {
                assert_ne!(sci_mantissa, 0.0);
                if sci_mantissa < 1.0 || sci_mantissa >= 2.0 {
                    None
                } else {
                    let m = sci_mantissa.integer_mantissa();
                    Some(
                        Rational::from(m)
                            << (sci_exponent - i64::exact_from(m.significant_bits()) + 1),
                    )
                }
            }
        }

        impl SciMantissaAndExponent<$t, i64, Rational> for &Rational {
            /// Returns a [`Rational`]'s scientific mantissa and exponent, taking the [`Rational`]
            /// by reference.
            ///
            /// When $x$ is positive, we can write $x = 2^{e_s}m_s$, where $e_s$ is an integer and
            /// $m_s$ is a rational number with $1 \leq m_s < 2$. We represent the rational mantissa
            /// as a float. The conversion might not be exact, so we round to the nearest float
            /// using the `Nearest` rounding mode. To use other rounding modes, use
            /// [`sci_mantissa_and_exponent_round`](Rational::sci_mantissa_and_exponent_round).
            /// $$
            /// f(x) \approx (\frac{x}{2^{\lfloor \log_2 x \rfloor}}, \lfloor \log_2 x \rfloor).
            /// $$
            ///
            /// # Worst-case complexity
            /// $T(n) = O(n \log n \log\log n)$
            ///
            /// $M(n) = O(n \log n)$
            ///
            /// where $T$ is time, $M$ is additional memory, and $n$ is `self.significant_bits()`.
            ///
            /// # Examples
            /// See [here](super::mantissa_and_exponent#sci_mantissa_and_exponent).
            #[inline]
            fn sci_mantissa_and_exponent(self) -> ($t, i64) {
                let (m, e, _) = self.sci_mantissa_and_exponent_round_ref(Nearest).unwrap();
                (m, e)
            }

            /// Returns a [`Rational`]'s scientific exponent, taking the [`Rational`] by reference.
            ///
            /// When $x$ is positive, we can write $x = 2^{e_s}m_s$, where $e_s$ is an integer and
            /// $m_s$ is a rational number with $1 \leq m_s < 2$. We represent the rational mantissa
            /// as a float. The conversion might not be exact, so we round to the nearest float
            /// using the `Nearest` rounding mode. To use other rounding modes, use
            /// [`sci_mantissa_and_exponent_round`](Rational::sci_mantissa_and_exponent_round).
            /// $$
            /// f(x) \approx \lfloor \log_2 x \rfloor.
            /// $$
            ///
            /// # Worst-case complexity
            /// $T(n) = O(n \log n \log\log n)$
            ///
            /// $M(n) = O(n \log n)$
            ///
            /// where $T$ is time, $M$ is additional memory, and $n$ is `self.significant_bits()`.
            fn sci_exponent(self) -> i64 {
                assert!(*self != 0);
                let mut exponent = i64::exact_from(self.numerator.significant_bits())
                    - i64::exact_from(self.denominator.significant_bits());
                if self.numerator.cmp_normalized(&self.denominator) == Less {
                    exponent -= 1;
                }
                let x = self >> exponent - i64::wrapping_from($t::MANTISSA_WIDTH);
                let (n, d) = x.into_numerator_and_denominator();
                if n.div_round(d, Nearest).0.significant_bits() > $t::MANTISSA_WIDTH + 1 {
                    exponent + 1
                } else {
                    exponent
                }
            }

            /// Constructs a [`Rational`] from its scientific mantissa and exponent.
            ///
            /// When $x$ is positive, we can write $x = 2^{e_s}m_s$, where $e_s$ is an integer and
            /// $m_s$ is a rational number with $1 \leq m_s < 2$. Here, the rational mantissa is
            /// provided as a float. If the mantissa is outside the range $[1, 2)$, `None` is
            /// returned.
            ///
            /// All finite floats can be represented using [`Rational`]s, so no rounding is needed.
            ///
            /// $$
            /// f(x) \approx 2^{e_s}m_s.
            /// $$
            ///
            /// # Worst-case complexity
            /// $T(n) = O(n)$
            ///
            /// $M(n) = O(n)$
            ///
            /// where $T$ is time, $M$ is additional memory, and $n$ is `sci_exponent`.
            ///
            /// See [here](super::mantissa_and_exponent#from_sci_mantissa_and_exponent).
            #[inline]
            fn from_sci_mantissa_and_exponent(
                sci_mantissa: $t,
                sci_exponent: i64,
            ) -> Option<Rational> {
                Rational::from_sci_mantissa_and_exponent(sci_mantissa, sci_exponent)
            }
        }
    };
}
apply_to_primitive_floats!(impl_mantissa_and_exponent);
