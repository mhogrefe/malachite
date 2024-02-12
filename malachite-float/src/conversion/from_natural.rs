use crate::Float;
use crate::InnerFloat::Finite;
use core::cmp::Ordering;
use malachite_base::num::arithmetic::traits::NegModPowerOf2;
use malachite_base::num::basic::integers::PrimitiveInt;
use malachite_base::num::basic::traits::Zero;
use malachite_base::num::conversion::traits::ExactFrom;
use malachite_base::num::logic::traits::SignificantBits;
use malachite_base::rounding_modes::RoundingMode;
use malachite_nz::natural::Natural;
use malachite_nz::platform::Limb;

impl Float {
    #[doc(hidden)]
    pub fn from_natural_times_power_of_2(x: Natural, pow: i64) -> Float {
        if x == 0u32 {
            return Float::ZERO;
        }
        let bits = x.significant_bits();
        Float(Finite {
            sign: true,
            exponent: i64::exact_from(bits) + pow,
            precision: bits,
            significand: x << bits.neg_mod_power_of_2(Limb::LOG_WIDTH),
        })
    }

    #[doc(hidden)]
    pub fn from_natural_times_power_of_2_prec_round(
        x: Natural,
        pow: i64,
        prec: u64,
        rm: RoundingMode,
    ) -> (Float, Ordering) {
        assert_ne!(prec, 0);
        if x == 0u32 {
            return (Float::ZERO, Ordering::Equal);
        }
        let bits = x.significant_bits();
        let mut f = Float(Finite {
            sign: true,
            exponent: i64::exact_from(bits) + pow,
            precision: bits,
            significand: x << bits.neg_mod_power_of_2(Limb::LOG_WIDTH),
        });
        let o = f.set_prec_round(prec, rm);
        (f, o)
    }

    #[doc(hidden)]
    #[inline]
    pub fn from_natural_times_power_of_2_prec(
        x: Natural,
        pow: i64,
        prec: u64,
    ) -> (Float, Ordering) {
        Float::from_natural_times_power_of_2_prec_round(x, pow, prec, RoundingMode::Nearest)
    }

    /// Converts a [`Natural`] to a [`Float`], taking the [`Natural`] by value. If the [`Float`] is
    /// nonzero, it has the specified precision. If rounding is needed, the specified rounding mode
    /// is used. An [`Ordering`] is also returned, indicating whether the returned value is less
    /// than, equal to, or greater than the original value.
    ///
    /// If you're only using [`RoundingMode::Nearest`], try using [`Float::from_natural_prec`]
    /// instead.
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
    /// use malachite_base::rounding_modes::RoundingMode;
    /// use malachite_base::num::basic::traits::Zero;
    /// use malachite_float::Float;
    /// use malachite_nz::natural::Natural;
    /// use std::cmp::Ordering;
    ///
    /// let (x, o) = Float::from_natural_prec_round(Natural::ZERO, 10, RoundingMode::Exact);
    /// assert_eq!(x.to_string(), "0.0");
    /// assert_eq!(o, Ordering::Equal);
    ///
    /// let (x, o) = Float::from_natural_prec_round(
    ///     Natural::from(123u32),
    ///     20,
    ///     RoundingMode::Exact
    /// );
    /// assert_eq!(x.to_string(), "123.0");
    /// assert_eq!(x.get_prec(), Some(20));
    /// assert_eq!(o, Ordering::Equal);
    ///
    /// let (x, o) = Float::from_natural_prec_round(Natural::from(123u32), 4, RoundingMode::Floor);
    /// assert_eq!(x.to_string(), "1.2e2");
    /// assert_eq!(x.get_prec(), Some(4));
    /// assert_eq!(o, Ordering::Less);
    ///
    /// let (x, o) = Float::from_natural_prec_round(
    ///     Natural::from(123u32),
    ///     4,
    ///     RoundingMode::Ceiling
    /// );
    /// assert_eq!(x.to_string(), "1.3e2");
    /// assert_eq!(x.get_prec(), Some(4));
    /// assert_eq!(o, Ordering::Greater);
    /// ```
    #[inline]
    pub fn from_natural_prec_round(x: Natural, prec: u64, rm: RoundingMode) -> (Float, Ordering) {
        Float::from_natural_times_power_of_2_prec_round(x, 0, prec, rm)
    }

    /// Converts a [`Natural`] to a [`Float`], taking the [`Natural`] by value. If the [`Float`] is
    /// nonzero, it has the specified precision. An [`Ordering`] is also returned, indicating
    /// whether the returned value is less than, equal to, or greater than the original value.
    ///
    /// If you want the [`Float`]'s precision to be equal to the [`Natural`]'s number of significant
    /// bits, try just using `Float::from` instead.
    ///
    /// Rounding may occur, in which case [`RoundingMode::Nearest`] is used by default. To specify a
    /// rounding mode as well as a precision, try [`Float::from_natural_prec_round`].
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
    /// use std::cmp::Ordering;
    ///
    /// let (x, o) = Float::from_natural_prec(Natural::ZERO, 10);
    /// assert_eq!(x.to_string(), "0.0");
    /// assert_eq!(o, Ordering::Equal);
    ///
    /// let (x, o) = Float::from_natural_prec(Natural::from(123u32), 20);
    /// assert_eq!(x.to_string(), "123.0");
    /// assert_eq!(x.get_prec(), Some(20));
    /// assert_eq!(o, Ordering::Equal);
    ///
    /// let (x, o) = Float::from_natural_prec(Natural::from(123u32), 4);
    /// assert_eq!(x.to_string(), "1.2e2");
    /// assert_eq!(x.get_prec(), Some(4));
    /// assert_eq!(o, Ordering::Less);
    /// ```
    #[inline]
    pub fn from_natural_prec(x: Natural, prec: u64) -> (Float, Ordering) {
        Float::from_natural_times_power_of_2_prec_round(x, 0, prec, RoundingMode::Nearest)
    }

    #[doc(hidden)]
    pub fn from_natural_times_power_of_2_ref(x: &Natural, pow: i64) -> Float {
        if *x == 0u32 {
            return Float::ZERO;
        }
        let bits = x.significant_bits();
        Float(Finite {
            sign: true,
            exponent: i64::exact_from(bits) + pow,
            precision: bits,
            significand: x << bits.neg_mod_power_of_2(Limb::LOG_WIDTH),
        })
    }

    #[doc(hidden)]
    pub fn from_natural_times_power_of_2_prec_round_ref(
        x: &Natural,
        pow: i64,
        prec: u64,
        rm: RoundingMode,
    ) -> (Float, Ordering) {
        // TODO be more efficient when x is large and prec is small
        assert_ne!(prec, 0);
        if *x == 0u32 {
            return (Float::ZERO, Ordering::Equal);
        }
        let bits = x.significant_bits();
        let mut f = Float(Finite {
            sign: true,
            exponent: i64::exact_from(bits) + pow,
            precision: bits,
            significand: x << bits.neg_mod_power_of_2(Limb::LOG_WIDTH),
        });
        let o = f.set_prec_round(prec, rm);
        (f, o)
    }

    #[doc(hidden)]
    #[inline]
    pub fn from_natural_times_power_of_2_prec_ref(
        x: &Natural,
        pow: i64,
        prec: u64,
    ) -> (Float, Ordering) {
        Float::from_natural_times_power_of_2_prec_round_ref(x, pow, prec, RoundingMode::Nearest)
    }

    /// Converts a [`Natural`] to a [`Float`], taking the [`Natural`] by reference. If the [`Float`]
    /// is nonzero, it has the specified precision. If rounding is needed, the specified rounding
    /// mode is used. An [`Ordering`] is also returned, indicating whether the returned value is
    /// less than, equal to, or greater than the original value.
    ///
    /// If you're only using [`RoundingMode::Nearest`], try using [`Float::from_natural_prec_ref`]
    /// instead.
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
    /// use malachite_base::rounding_modes::RoundingMode;
    /// use malachite_base::num::basic::traits::Zero;
    /// use malachite_float::Float;
    /// use malachite_nz::natural::Natural;
    /// use std::cmp::Ordering;
    ///
    /// let (x, o) = Float::from_natural_prec_round_ref(&Natural::ZERO, 10, RoundingMode::Exact);
    /// assert_eq!(x.to_string(), "0.0");
    /// assert_eq!(o, Ordering::Equal);
    ///
    /// let (x, o) = Float::from_natural_prec_round_ref(
    ///     &Natural::from(123u32),
    ///     20,
    ///     RoundingMode::Exact
    /// );
    /// assert_eq!(x.to_string(), "123.0");
    /// assert_eq!(x.get_prec(), Some(20));
    /// assert_eq!(o, Ordering::Equal);
    ///
    /// let (x, o) = Float::from_natural_prec_round_ref(
    ///     &Natural::from(123u32),
    ///     4,
    ///     RoundingMode::Floor
    /// );
    /// assert_eq!(x.to_string(), "1.2e2");
    /// assert_eq!(x.get_prec(), Some(4));
    /// assert_eq!(o, Ordering::Less);
    ///
    /// let (x, o) = Float::from_natural_prec_round_ref(
    ///     &Natural::from(123u32),
    ///     4,
    ///     RoundingMode::Ceiling
    /// );
    /// assert_eq!(x.to_string(), "1.3e2");
    /// assert_eq!(x.get_prec(), Some(4));
    /// assert_eq!(o, Ordering::Greater);
    /// ```
    #[inline]
    pub fn from_natural_prec_round_ref(
        x: &Natural,
        prec: u64,
        rm: RoundingMode,
    ) -> (Float, Ordering) {
        Float::from_natural_times_power_of_2_prec_round_ref(x, 0, prec, rm)
    }

    /// Converts a [`Natural`] to a [`Float`], taking the [`Natural`] by reference. If the [`Float`]
    /// is nonzero, it has the specified precision. An [`Ordering`] is also returned, indicating
    /// whether the returned value is less than, equal to, or greater than the original value.
    ///
    /// If you want the [`Float`]'s precision to be equal to the [`Natural`]'s number of significant
    /// bits, try just using `Float::from` instead.
    ///
    /// Rounding may occur, in which case [`RoundingMode::Nearest`] is used by default. To specify a
    /// rounding mode as well as a precision, try [`Float::from_natural_prec_round_ref`].
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
    /// use std::cmp::Ordering;
    ///
    /// let (x, o) = Float::from_natural_prec_ref(&Natural::ZERO, 10);
    /// assert_eq!(x.to_string(), "0.0");
    /// assert_eq!(o, Ordering::Equal);
    ///
    /// let (x, o) = Float::from_natural_prec_ref(&Natural::from(123u32), 20);
    /// assert_eq!(x.to_string(), "123.0");
    /// assert_eq!(x.get_prec(), Some(20));
    /// assert_eq!(o, Ordering::Equal);
    ///
    /// let (x, o) = Float::from_natural_prec_ref(&Natural::from(123u32), 4);
    /// assert_eq!(x.to_string(), "1.2e2");
    /// assert_eq!(x.get_prec(), Some(4));
    /// assert_eq!(o, Ordering::Less);
    /// ```
    #[inline]
    pub fn from_natural_prec_ref(x: &Natural, prec: u64) -> (Float, Ordering) {
        Float::from_natural_times_power_of_2_prec_round_ref(x, 0, prec, RoundingMode::Nearest)
    }
}

impl From<Natural> for Float {
    /// Converts a [`Natural`] to a [`Float`], taking the [`Natural`] by value.
    ///
    /// If the [`Natural`] is nonzero, the precision of the [`Float`] is equal to the [`Natural`]'s
    /// number of significant bits. If you want to specify a different precision, try
    /// [`Float::from_natural_prec`]. This may require rounding, which uses
    /// [`RoundingMode::Nearest`] by default. To specify a rounding mode as well as a precision, try
    /// [`Float::from_natural_prec_round`].
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
    #[inline]
    fn from(n: Natural) -> Float {
        Float::from_natural_times_power_of_2(n, 0)
    }
}

impl<'a> From<&'a Natural> for Float {
    /// Converts a [`Natural`] to a [`Float`], taking the [`Natural`] by reference.
    ///
    /// If the [`Natural`] is nonzero, the precision of the [`Float`] is equal to the [`Natural`]'s
    /// number of significant bits. If you want to specify a different precision, try
    /// [`Float::from_natural_prec_ref`]. This may require rounding, which uses
    /// [`RoundingMode::Nearest`] by default. To specify a rounding mode as well as a precision, try
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
        Float::from_natural_times_power_of_2_ref(n, 0)
    }
}
