use malachite_base::num::arithmetic::traits::{
    ModPowerOf2, ModPowerOf2Assign, Parity, PowerOf2, ShrRound, Sign,
};
use malachite_base::num::basic::floats::PrimitiveFloat;
use malachite_base::num::basic::integers::PrimitiveInt;
use malachite_base::num::conversion::traits::{
    ExactFrom, FromOtherTypeSlice, IntegerMantissaAndExponent, SciMantissaAndExponent, WrappingFrom,
};
use malachite_base::num::logic::traits::SignificantBits;
use malachite_base::rounding_modes::RoundingMode;
use malachite_base::slices::{slice_set_zero, slice_test_zero};
use natural::arithmetic::shl::limbs_slice_shl_in_place;
use natural::arithmetic::shr::limbs_slice_shr_in_place;
use natural::logic::bit_access::limbs_get_bit;
use natural::logic::bit_scan::limbs_index_of_next_true_bit;
use natural::InnerNatural::{Large, Small};
use natural::Natural;
use platform::Limb;
use std::cmp::Ordering;

impl Natural {
    /// Returns the scientific mantissa and exponent.
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
    /// extern crate malachite_base;
    /// extern crate malachite_nz;
    ///
    /// use malachite_base::num::arithmetic::traits::Pow;
    /// use malachite_base::num::conversion::traits::SciMantissaAndExponent;
    /// use malachite_base::num::float::NiceFloat;
    /// use malachite_base::rounding_modes::RoundingMode;
    /// use malachite_nz::natural::Natural;
    ///
    /// let test = |n: Natural, rm: RoundingMode, out: Option<(f32, u64)>| {
    ///     assert_eq!(
    ///         n.sci_mantissa_and_exponent_with_rounding(rm)
    ///             .map(|(m, e)| (NiceFloat(m), e)),
    ///         out.map(|(m, e)| (NiceFloat(m), e))
    ///     );
    /// };
    /// test(Natural::from(3u32), RoundingMode::Down, Some((1.5, 1)));
    /// test(Natural::from(3u32), RoundingMode::Ceiling, Some((1.5, 1)));
    /// test(Natural::from(3u32), RoundingMode::Up, Some((1.5, 1)));
    /// test(Natural::from(3u32), RoundingMode::Nearest, Some((1.5, 1)));
    /// test(Natural::from(3u32), RoundingMode::Exact, Some((1.5, 1)));
    ///
    /// test(
    ///     Natural::from(123u32),
    ///     RoundingMode::Floor,
    ///     Some((1.921875, 6)),
    /// );
    /// test(
    ///     Natural::from(123u32),
    ///     RoundingMode::Down,
    ///     Some((1.921875, 6)),
    /// );
    /// test(
    ///     Natural::from(123u32),
    ///     RoundingMode::Ceiling,
    ///     Some((1.921875, 6)),
    /// );
    /// test(Natural::from(123u32), RoundingMode::Up, Some((1.921875, 6)));
    /// test(
    ///     Natural::from(123u32),
    ///     RoundingMode::Nearest,
    ///     Some((1.921875, 6)),
    /// );
    /// test(
    ///     Natural::from(123u32),
    ///     RoundingMode::Exact,
    ///     Some((1.921875, 6)),
    /// );
    ///
    /// test(
    ///     Natural::from(1000000000u32),
    ///     RoundingMode::Nearest,
    ///     Some((1.8626451, 29)),
    /// );
    /// test(
    ///     Natural::from(10u32).pow(52),
    ///     RoundingMode::Nearest,
    ///     Some((1.670478, 172)),
    /// );
    ///
    /// test(Natural::from(10u32).pow(52), RoundingMode::Exact, None);
    /// ```
    pub fn sci_mantissa_and_exponent_with_rounding<T: PrimitiveFloat>(
        &self,
        rm: RoundingMode,
    ) -> Option<(T, u64)> {
        assert_ne!(*self, 0);
        // Worst case: 32-bit limbs, 64-bit float output, most-significant limb is 1. In this
        // case, the 3 most significant limbs are needed.
        let mut most_significant_limbs = [0; 3];
        let mut exponent = T::MANTISSA_WIDTH;
        let significant_bits;
        let mut exact = true;
        let mut half_compare = Ordering::Less; // (mantissa - floor(mantissa)).cmp(&0.5)
        let care_about_exactness = rm != RoundingMode::Floor && rm != RoundingMode::Down;
        let mut highest_discarded_limb = 0;
        match self {
            Natural(Small(x)) => {
                most_significant_limbs[0] = *x;
                significant_bits = x.significant_bits();
            }
            Natural(Large(ref xs)) => {
                let len = xs.len();
                if len == 2 {
                    most_significant_limbs[0] = xs[0];
                    most_significant_limbs[1] = xs[1];
                    significant_bits = xs[1].significant_bits() + Limb::WIDTH;
                } else {
                    most_significant_limbs[2] = xs[len - 1];
                    most_significant_limbs[1] = xs[len - 2];
                    most_significant_limbs[0] = xs[len - 3];
                    exponent += u64::exact_from(len - 3) << Limb::LOG_WIDTH;
                    if care_about_exactness && !slice_test_zero(&xs[..len - 3]) {
                        if rm == RoundingMode::Exact {
                            return None;
                        }
                        exact = false;
                        highest_discarded_limb = xs[len - 4];
                    }
                    significant_bits =
                        most_significant_limbs[2].significant_bits() + (Limb::WIDTH << 1);
                }
            }
        }
        let shift =
            i128::wrapping_from(T::MANTISSA_WIDTH + 1) - i128::wrapping_from(significant_bits);
        match shift.sign() {
            Ordering::Greater => {
                let mut shift = u64::exact_from(shift);
                exponent -= shift;
                let limbs_to_shift = shift >> Limb::LOG_WIDTH;
                if limbs_to_shift != 0 {
                    shift.mod_power_of_2_assign(Limb::LOG_WIDTH);
                    let limbs_to_shift = usize::wrapping_from(limbs_to_shift);
                    most_significant_limbs.copy_within(..3 - limbs_to_shift, limbs_to_shift);
                    slice_set_zero(&mut most_significant_limbs[..limbs_to_shift])
                }
                if shift != 0 {
                    limbs_slice_shl_in_place(&mut most_significant_limbs, shift);
                }
            }
            Ordering::Less => {
                let mut shift = u64::exact_from(-shift);
                if care_about_exactness {
                    let one_index =
                        limbs_index_of_next_true_bit(&most_significant_limbs, 0).unwrap();
                    if one_index < shift {
                        if rm == RoundingMode::Exact {
                            return None;
                        }
                        if rm == RoundingMode::Nearest {
                            // If `exact` is true here, that means all lower limbs are 0
                            half_compare = if exact && one_index == shift - 1 {
                                Ordering::Equal
                            } else if limbs_get_bit(&most_significant_limbs, shift - 1) {
                                Ordering::Greater
                            } else {
                                Ordering::Less
                            };
                        }
                        exact = false;
                    }
                }
                exponent += shift;
                let limbs_to_shift = shift >> Limb::LOG_WIDTH;
                if limbs_to_shift != 0 {
                    shift.mod_power_of_2_assign(Limb::LOG_WIDTH);
                    most_significant_limbs.copy_within(usize::wrapping_from(limbs_to_shift).., 0);
                }
                if shift != 0 {
                    limbs_slice_shr_in_place(&mut most_significant_limbs, shift);
                }
            }
            Ordering::Equal => {
                if !exact && rm == RoundingMode::Nearest {
                    // len is at least 4, since the only way `exact` is false at this point is if
                    // xs[..len - 3] is nonzero
                    half_compare = highest_discarded_limb.cmp(&Limb::power_of_2(Limb::WIDTH - 1));
                }
            }
        }
        let raw_mantissa =
            u64::from_other_type_slice(&most_significant_limbs).mod_power_of_2(T::MANTISSA_WIDTH);
        let mantissa =
            T::from_raw_mantissa_and_exponent(raw_mantissa, u64::wrapping_from(T::MAX_EXPONENT));
        let increment = !exact
            && (rm == RoundingMode::Up
                || rm == RoundingMode::Ceiling
                || rm == RoundingMode::Nearest
                    && (half_compare == Ordering::Greater
                        || half_compare == Ordering::Equal && raw_mantissa.odd()));
        if increment {
            let next_mantissa = mantissa.next_higher();
            if next_mantissa == T::TWO {
                Some((T::ONE, exponent + 1))
            } else {
                Some((next_mantissa, exponent))
            }
        } else {
            Some((mantissa, exponent))
        }
    }

    /// Constructs a `Natural` from its scientific mantissa and exponent.
    ///
    /// When $x$ is positive, we can write $x = 2^{e_s}m_s$, where $e_s$ is an integer and $m_s$ is
    /// a rational number with $1 \leq m_s < 2$. Here, the rational mantissa is provided as a
    /// float. If the mantissa is outside the range $[1, 2)$, `None` is returned.
    ///
    /// Some combinations of mantissas and exponents do not specify an integer, in which case the
    /// resulting value is rounded to an integer using the specified rounding mode. If the rounding
    /// mode is `Exact` but the input does not exactly specify an integer, `None` is returned.
    ///
    /// $$
    /// f(x, r) \approx 2^{e_s}m_s.
    /// $$
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `sci_exponent`.
    ///
    /// # Panics
    /// Panics if `sci_mantissa` is zero.
    ///
    /// # Examples
    /// ```
    /// extern crate malachite_base;
    /// extern crate malachite_nz;
    ///
    /// use malachite_base::num::conversion::traits::SciMantissaAndExponent;
    /// use malachite_base::rounding_modes::RoundingMode;
    /// use malachite_nz::natural::Natural;
    /// use std::str::FromStr;
    ///
    /// let test = |mantissa: f32, exponent: u64, rm: RoundingMode, out: Option<Natural>| {
    ///     assert_eq!(
    ///         Natural::from_sci_mantissa_and_exponent_with_rounding(mantissa, exponent, rm),
    ///         out
    ///     );
    /// };
    /// test(1.5, 1, RoundingMode::Floor, Some(Natural::from(3u32)));
    /// test(1.5, 1, RoundingMode::Down, Some(Natural::from(3u32)));
    /// test(1.5, 1, RoundingMode::Ceiling, Some(Natural::from(3u32)));
    /// test(1.5, 1, RoundingMode::Up, Some(Natural::from(3u32)));
    /// test(1.5, 1, RoundingMode::Nearest, Some(Natural::from(3u32)));
    /// test(1.5, 1, RoundingMode::Exact, Some(Natural::from(3u32)));
    ///
    /// test(1.51, 1, RoundingMode::Floor, Some(Natural::from(3u32)));
    /// test(1.51, 1, RoundingMode::Down, Some(Natural::from(3u32)));
    /// test(1.51, 1, RoundingMode::Ceiling, Some(Natural::from(4u32)));
    /// test(1.51, 1, RoundingMode::Up, Some(Natural::from(4u32)));
    /// test(1.51, 1, RoundingMode::Nearest, Some(Natural::from(3u32)));
    /// test(1.51, 1, RoundingMode::Exact, None);
    ///
    /// test(
    ///     1.670478,
    ///     172,
    ///     RoundingMode::Nearest,
    ///     Some(
    ///         Natural::from_str("10000000254586612611935772707803116801852191350456320").unwrap()
    ///     ),
    /// );
    ///
    /// test(2.0, 1, RoundingMode::Floor, None);
    /// test(10.0, 1, RoundingMode::Floor, None);
    /// test(0.5, 1, RoundingMode::Floor, None);
    /// ```
    #[inline]
    pub fn from_sci_mantissa_and_exponent_with_rounding<T: PrimitiveFloat>(
        sci_mantissa: T,
        sci_exponent: u64,
        rm: RoundingMode,
    ) -> Option<Natural> {
        assert_ne!(sci_mantissa, T::ZERO);
        if sci_mantissa < T::ONE || sci_mantissa >= T::TWO {
            return None;
        }
        let (integer_mantissa, integer_exponent) = sci_mantissa.integer_mantissa_and_exponent();
        if integer_exponent > 0 {
            Some(
                Natural::from(integer_mantissa)
                    << (sci_exponent + u64::exact_from(integer_exponent)),
            )
        } else {
            let integer_exponent = u64::exact_from(-integer_exponent);
            if integer_exponent <= sci_exponent {
                Some(Natural::from(integer_mantissa) << (sci_exponent - integer_exponent))
            } else if rm == RoundingMode::Exact {
                None
            } else {
                Some(Natural::from(integer_mantissa).shr_round(integer_exponent - sci_exponent, rm))
            }
        }
    }
}

impl<'a> IntegerMantissaAndExponent<Natural, u64, Natural> for &'a Natural {
    /// Returns the integer mantissa and exponent.
    ///
    /// When $x$ is nonzero, we can write $x = 2^{e_i}m_i$, where $e_i$ is an integer and
    /// $m_i$ is an odd integer.
    /// $$
    /// f(x) = (\frac{|x|}{2^{e_i}}, e_i),
    /// $$
    /// where $e_i$ is the unique integer such that $x/2^{e_i}$ is an odd integer.
    ///
    /// The inverse operation is `from_integer_mantissa_and_exponent`.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(1)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `self.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `self` is zero.
    ///
    /// # Examples
    /// ```
    /// extern crate malachite_base;
    /// extern crate malachite_nz;
    ///
    /// use malachite_base::num::conversion::traits::IntegerMantissaAndExponent;
    /// use malachite_nz::natural::Natural;
    ///
    /// assert_eq!(
    ///     Natural::from(123u32).integer_mantissa_and_exponent(),
    ///     (Natural::from(123u32), 0)
    /// );
    /// assert_eq!(
    ///     Natural::from(100u32).integer_mantissa_and_exponent(),
    ///     (Natural::from(25u32), 2)
    /// );
    /// ```
    #[inline]
    fn integer_mantissa_and_exponent(self) -> (Natural, u64) {
        let trailing_zeros = self.trailing_zeros().unwrap();
        (self >> trailing_zeros, trailing_zeros)
    }

    /// Returns the integer mantissa.
    ///
    /// When $x$ is nonzero, we can write $x = 2^{e_i}m_i$, where $e_i$ is an integer and
    /// $m_i$ is an odd integer.
    /// $$
    /// f(x) = \frac{|x|}{2^{e_i}},
    /// $$
    /// where $e_i$ is the unique integer such that $x/2^{e_i}$ is an odd integer.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(1)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `self.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `self` is zero.
    ///
    /// # Examples
    /// ```
    /// extern crate malachite_base;
    /// extern crate malachite_nz;
    ///
    /// use malachite_base::num::conversion::traits::IntegerMantissaAndExponent;
    /// use malachite_nz::natural::Natural;
    ///
    /// assert_eq!(Natural::from(123u32).integer_mantissa(), 123);
    /// assert_eq!(Natural::from(100u32).integer_mantissa(), 25);
    /// ```
    #[inline]
    fn integer_mantissa(self) -> Natural {
        self >> self.trailing_zeros().unwrap()
    }

    /// Returns the integer exponent.
    ///
    /// When $x$ is nonzero, we can write $x = 2^{e_i}m_i$, where $e_i$ is an integer and
    /// $m_i$ is an odd integer.
    /// $$
    /// f(x) = e_i,
    /// $$
    /// where $e_i$ is the unique integer such that $x/2^{e_i}$ is an odd integer.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(1)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `self.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `self` is zero.
    ///
    /// # Examples
    /// ```
    /// extern crate malachite_base;
    /// extern crate malachite_nz;
    ///
    /// use malachite_base::num::conversion::traits::IntegerMantissaAndExponent;
    /// use malachite_nz::natural::Natural;
    ///
    /// assert_eq!(Natural::from(123u32).integer_exponent(), 0);
    /// assert_eq!(Natural::from(100u32).integer_exponent(), 2);
    /// ```
    #[inline]
    fn integer_exponent(self) -> u64 {
        self.trailing_zeros().unwrap()
    }

    /// Constructs a `Natural` from its integer mantissa and exponent.
    ///
    /// When $x$ is nonzero, we can write $x = 2^{e_i}m_i$, where $e_i$ is an integer and $m_i$ is
    /// an odd integer.
    ///
    /// $$
    /// f(x) = 2^{e_i}m_i.
    /// $$
    ///
    /// The input does not have to be reduced; that is, the mantissa does not have to be odd.
    ///
    /// The result is an `Option`, but for this trait implementation the result is always `Some`.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is
    /// `integer_mantissa.significant_bits() + integer_exponent`.
    ///
    /// # Examples
    /// ```
    /// extern crate malachite_base;
    /// extern crate malachite_nz;
    ///
    /// use malachite_base::num::conversion::traits::IntegerMantissaAndExponent;
    /// use malachite_nz::natural::Natural;
    ///
    /// let n = <&Natural as IntegerMantissaAndExponent<_, _, _>>
    ///     ::from_integer_mantissa_and_exponent(Natural::from(123u32), 0).unwrap();
    /// assert_eq!(n, 123);
    /// let n = <&Natural as IntegerMantissaAndExponent<_, _, _>>
    ///     ::from_integer_mantissa_and_exponent(Natural::from(25u32), 2).unwrap();
    /// assert_eq!(n, 100);
    /// ```
    #[inline]
    fn from_integer_mantissa_and_exponent(
        integer_mantissa: Natural,
        integer_exponent: u64,
    ) -> Option<Natural> {
        Some(integer_mantissa << integer_exponent)
    }
}

macro_rules! impl_mantissa_and_exponent {
    ($t:ident) => {
        impl<'a> SciMantissaAndExponent<$t, u64, Natural> for &'a Natural {
            /// Returns the scientific mantissa and exponent.
            ///
            /// When $x$ is positive, we can write $x = 2^{e_s}m_s$, where $e_s$ is an integer and
            /// $m_s$ is a rational number with $1 \leq m_s < 2$. We represent the rational
            /// mantissa as a float. The conversion might not be exact, so we round to the nearest
            /// float using the `Nearest` rounding mode. To use other rounding modes, use
            /// `sci_mantissa_and_exponent`.
            /// $$
            /// f(x) \approx (\frac{x}{2^{\lfloor \log_2 x \rfloor}}, \lfloor \log_2 x \rfloor).
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
            /// See the documentation of the `natural::conversion::mantissa_and_exponent` module.
            #[inline]
            fn sci_mantissa_and_exponent(self) -> ($t, u64) {
                self.sci_mantissa_and_exponent_with_rounding(RoundingMode::Nearest)
                    .unwrap()
            }

            /// Constructs a `Natural` from its scientific mantissa and exponent.
            ///
            /// When $x$ is positive, we can write $x = 2^{e_s}m_s$, where $e_s$ is an integer and
            /// $m_s$ is a rational number with $1 \leq m_s < 2$. Here, the rational mantissa is
            /// provided as a float. If the mantissa is outside the range $[1, 2)$, `None` is
            /// returned.
            ///
            /// Some combinations of mantissas and exponents do not specify an integer, in which
            /// case the resulting value is rounded to an integer using the `Nearest` rounding
            /// mode. To specify other rounding modes, use
            /// `from_sci_mantissa_and_exponent_with_rounding`.
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
            /// See the documentation of the `natural::conversion::mantissa_and_exponent` module.
            #[inline]
            fn from_sci_mantissa_and_exponent(
                sci_mantissa: $t,
                sci_exponent: u64,
            ) -> Option<Natural> {
                Natural::from_sci_mantissa_and_exponent_with_rounding(
                    sci_mantissa,
                    sci_exponent,
                    RoundingMode::Nearest,
                )
            }
        }
    };
}
apply_to_primitive_floats!(impl_mantissa_and_exponent);
