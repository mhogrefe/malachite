use malachite_base::num::arithmetic::traits::{DivRound, DivisibleBy};
use malachite_base::num::basic::floats::PrimitiveFloat;
use malachite_base::num::conversion::traits::{
    ExactFrom, IntegerMantissaAndExponent, SciMantissaAndExponent, WrappingFrom,
};
use malachite_base::num::logic::traits::{BitAccess, SignificantBits};
use malachite_base::rounding_modes::RoundingMode;
use std::cmp::Ordering;
use Rational;

impl Rational {
    /// Returns the scientific mantissa and exponent, taking `self` by reference.
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
    /// TODO
    ///
    /// # Examples
    /// ```
    /// extern crate malachite_base;
    /// extern crate malachite_q;
    ///
    /// use malachite_base::num::arithmetic::traits::Pow;
    /// use malachite_base::num::conversion::traits::SciMantissaAndExponent;
    /// use malachite_base::num::float::NiceFloat;
    /// use malachite_base::rounding_modes::RoundingMode;
    /// use malachite_q::Rational;
    ///
    /// let test = |n: Rational, rm: RoundingMode, out: Option<(f32, i64)>| {
    ///     assert_eq!(
    ///         n.sci_mantissa_and_exponent_with_rounding(rm)
    ///             .map(|(m, e)| (NiceFloat(m), e)),
    ///         out.map(|(m, e)| (NiceFloat(m), e))
    ///     );
    /// };
    /// test(Rational::from(3u32), RoundingMode::Down, Some((1.5, 1)));
    /// test(Rational::from(3u32), RoundingMode::Ceiling, Some((1.5, 1)));
    /// test(Rational::from(3u32), RoundingMode::Up, Some((1.5, 1)));
    /// test(Rational::from(3u32), RoundingMode::Nearest, Some((1.5, 1)));
    /// test(Rational::from(3u32), RoundingMode::Exact, Some((1.5, 1)));
    ///
    /// test(Rational::from_signeds(1, 3), RoundingMode::Floor, Some((1.3333333, -2)));
    /// test(Rational::from_signeds(1, 3), RoundingMode::Down, Some((1.3333333, -2)));
    /// test(Rational::from_signeds(1, 3), RoundingMode::Ceiling, Some((1.3333334, -2)));
    /// test(Rational::from_signeds(1, 3), RoundingMode::Up, Some((1.3333334, -2)));
    /// test(Rational::from_signeds(1, 3), RoundingMode::Nearest, Some((1.3333334, -2)));
    /// test(Rational::from_signeds(1, 3), RoundingMode::Exact, None);
    /// ```
    pub fn sci_mantissa_and_exponent_with_rounding<T: PrimitiveFloat>(
        mut self,
        rm: RoundingMode,
    ) -> Option<(T, i64)> {
        assert!(self != 0);
        let mut exponent = i64::exact_from(self.numerator.significant_bits())
            - i64::exact_from(self.denominator.significant_bits());
        if self.numerator.cmp_normalized(&self.denominator) == Ordering::Less {
            exponent -= 1;
        }
        self >>= exponent - i64::wrapping_from(T::MANTISSA_WIDTH);
        let (n, d) = self.into_numerator_and_denominator();
        if rm == RoundingMode::Exact && !(&n).divisible_by(&d) {
            return None;
        }
        let mut mantissa = n.div_round(d, rm);
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
        ))
    }

    /// Returns the scientific mantissa and exponent, taking `self` by reference.
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
    /// TODO
    ///
    /// # Examples
    /// ```
    /// extern crate malachite_base;
    /// extern crate malachite_q;
    ///
    /// use malachite_base::num::arithmetic::traits::Pow;
    /// use malachite_base::num::conversion::traits::SciMantissaAndExponent;
    /// use malachite_base::num::float::NiceFloat;
    /// use malachite_base::rounding_modes::RoundingMode;
    /// use malachite_q::Rational;
    ///
    /// let test = |n: Rational, rm: RoundingMode, out: Option<(f32, i64)>| {
    ///     assert_eq!(
    ///         (&n).sci_mantissa_and_exponent_with_rounding_ref(rm)
    ///             .map(|(m, e)| (NiceFloat(m), e)),
    ///         out.map(|(m, e)| (NiceFloat(m), e))
    ///     );
    /// };
    /// test(Rational::from(3u32), RoundingMode::Down, Some((1.5, 1)));
    /// test(Rational::from(3u32), RoundingMode::Ceiling, Some((1.5, 1)));
    /// test(Rational::from(3u32), RoundingMode::Up, Some((1.5, 1)));
    /// test(Rational::from(3u32), RoundingMode::Nearest, Some((1.5, 1)));
    /// test(Rational::from(3u32), RoundingMode::Exact, Some((1.5, 1)));
    ///
    /// test(Rational::from_signeds(1, 3), RoundingMode::Floor, Some((1.3333333, -2)));
    /// test(Rational::from_signeds(1, 3), RoundingMode::Down, Some((1.3333333, -2)));
    /// test(Rational::from_signeds(1, 3), RoundingMode::Ceiling, Some((1.3333334, -2)));
    /// test(Rational::from_signeds(1, 3), RoundingMode::Up, Some((1.3333334, -2)));
    /// test(Rational::from_signeds(1, 3), RoundingMode::Nearest, Some((1.3333334, -2)));
    /// test(Rational::from_signeds(1, 3), RoundingMode::Exact, None);
    /// ```
    pub fn sci_mantissa_and_exponent_with_rounding_ref<T: PrimitiveFloat>(
        &self,
        rm: RoundingMode,
    ) -> Option<(T, i64)> {
        assert!(*self != 0);
        let mut exponent = i64::exact_from(self.numerator.significant_bits())
            - i64::exact_from(self.denominator.significant_bits());
        if self.numerator.cmp_normalized(&self.denominator) == Ordering::Less {
            exponent -= 1;
        }
        let x = self >> (exponent - i64::wrapping_from(T::MANTISSA_WIDTH));
        let (n, d) = x.into_numerator_and_denominator();
        if rm == RoundingMode::Exact && !(&n).divisible_by(&d) {
            return None;
        }
        let mut mantissa = n.div_round(d, rm);
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
        ))
    }
}

macro_rules! impl_mantissa_and_exponent {
    ($t:ident) => {
        impl SciMantissaAndExponent<$t, i64> for Rational {
            /// Returns the scientific mantissa and exponent, taking `self` by value
            ///
            /// When $x$ is positive, we can write $x = 2^{e_s}m_s$, where $e_s$ is an integer and
            /// $m_s$ is a rational number with $1 \leq m_s < 2$. We represent the rational
            /// mantissa as a float. The conversion might not be exact, so we round to the nearest
            /// float using the `Nearest` rounding mode. To use other rounding modes, use
            /// `sci_mantissa_and_exponent_with_rounding`.
            /// $$
            /// f(x) \approx (\frac{x}{2^{\lfloor \log_2 x \rfloor}}, \lfloor \log_2 x \rfloor).
            /// $$
            ///
            /// # Worst-case complexity
            /// TODO
            ///
            /// # Examples
            /// See the documentation of the `conversion::mantissa_and_exponent` module.
            #[inline]
            fn sci_mantissa_and_exponent(self) -> ($t, i64) {
                self.sci_mantissa_and_exponent_with_rounding(RoundingMode::Nearest)
                    .unwrap()
            }

            /// Returns the scientific exponent, taking `self` by value
            ///
            /// When $x$ is positive, we can write $x = 2^{e_s}m_s$, where $e_s$ is an integer and
            /// $m_s$ is a rational number with $1 \leq m_s < 2$. We represent the rational
            /// mantissa as a float. The conversion might not be exact, so we round to the nearest
            /// float using the `Nearest` rounding mode. To use other rounding modes, use
            /// `sci_mantissa_and_exponent_with_rounding`.
            /// $$
            /// f(x) \approx \lfloor \log_2 x \rfloor.
            /// $$
            ///
            /// # Worst-case complexity
            /// TODO
            ///
            /// # Examples
            /// See the documentation of the `conversion::mantissa_and_exponent` module.
            fn sci_exponent(mut self) -> i64 {
                assert!(self != 0);
                let mut exponent = i64::exact_from(self.numerator.significant_bits())
                    - i64::exact_from(self.denominator.significant_bits());
                if self.numerator.cmp_normalized(&self.denominator) == Ordering::Less {
                    exponent -= 1;
                }
                self >>= exponent - i64::wrapping_from($t::MANTISSA_WIDTH);
                let (n, d) = self.into_numerator_and_denominator();
                if n.div_round(d, RoundingMode::Nearest).significant_bits() > $t::MANTISSA_WIDTH + 1
                {
                    exponent + 1
                } else {
                    exponent
                }
            }

            /// Constructs a `Rational` from its scientific mantissa and exponent.
            ///
            /// When $x$ is positive, we can write $x = 2^{e_s}m_s$, where $e_s$ is an integer and
            /// $m_s$ is a rational number with $1 \leq m_s < 2$. Here, the rational mantissa is
            /// provided as a float. If the mantissa is outside the range $[1, 2)$, `None` is
            /// returned.
            ///
            /// All finite floats can be represented using `Rational`s, so no rounding is needed.
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
            /// # Examples
            /// See the documentation of the `conversion::mantissa_and_exponent` module.
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

        impl<'a> SciMantissaAndExponent<$t, i64, Rational> for &'a Rational {
            /// Returns the scientific mantissa and exponent, taking `self` by reference.
            ///
            /// When $x$ is positive, we can write $x = 2^{e_s}m_s$, where $e_s$ is an integer and
            /// $m_s$ is a rational number with $1 \leq m_s < 2$. We represent the rational
            /// mantissa as a float. The conversion might not be exact, so we round to the nearest
            /// float using the `Nearest` rounding mode. To use other rounding modes, use
            /// `sci_mantissa_and_exponent_with_rounding`.
            /// $$
            /// f(x) \approx (\frac{x}{2^{\lfloor \log_2 x \rfloor}}, \lfloor \log_2 x \rfloor).
            /// $$
            ///
            /// # Worst-case complexity
            /// TODO
            ///
            /// # Examples
            /// See the documentation of the `conversion::mantissa_and_exponent` module.
            #[inline]
            fn sci_mantissa_and_exponent(self) -> ($t, i64) {
                self.sci_mantissa_and_exponent_with_rounding_ref(RoundingMode::Nearest)
                    .unwrap()
            }

            /// Returns the scientific exponent, taking `self` by reference.
            ///
            /// When $x$ is positive, we can write $x = 2^{e_s}m_s$, where $e_s$ is an integer and
            /// $m_s$ is a rational number with $1 \leq m_s < 2$. We represent the rational
            /// mantissa as a float. The conversion might not be exact, so we round to the nearest
            /// float using the `Nearest` rounding mode. To use other rounding modes, use
            /// `sci_mantissa_and_exponent_with_rounding`.
            /// $$
            /// f(x) \approx \lfloor \log_2 x \rfloor.
            /// $$
            ///
            /// # Worst-case complexity
            /// TODO
            ///
            /// # Examples
            /// See the documentation of the `conversion::mantissa_and_exponent` module.
            fn sci_exponent(self) -> i64 {
                assert!(*self != 0);
                let mut exponent = i64::exact_from(self.numerator.significant_bits())
                    - i64::exact_from(self.denominator.significant_bits());
                if self.numerator.cmp_normalized(&self.denominator) == Ordering::Less {
                    exponent -= 1;
                }
                let x = self >> exponent - i64::wrapping_from($t::MANTISSA_WIDTH);
                let (n, d) = x.into_numerator_and_denominator();
                if n.div_round(d, RoundingMode::Nearest).significant_bits() > $t::MANTISSA_WIDTH + 1
                {
                    exponent + 1
                } else {
                    exponent
                }
            }

            /// Constructs a `Rational` from its scientific mantissa and exponent.
            ///
            /// See the identical implementation on `&Rational`.
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
