use num::arithmetic::traits::{DivisibleByPowerOf2, ModPowerOf2};
use num::basic::integers::PrimitiveInt;
use num::conversion::traits::{
    ExactFrom, ExactInto, IntegerMantissaAndExponent, RawMantissaAndExponent,
    SciMantissaAndExponent, WrappingFrom,
};
use num::float::PrimitiveFloat;
use num::logic::traits::{BitAccess, LeadingZeros, LowMask, SignificantBits, TrailingZeros};

fn _raw_mantissa_and_exponent<T: PrimitiveFloat>(x: T) -> (u64, u64) {
    let bits = x.to_bits();
    let mantissa = bits.mod_power_of_2(T::MANTISSA_WIDTH);
    let exponent: u64 = (bits >> T::MANTISSA_WIDTH).exact_into();
    let exponent = exponent.mod_power_of_2(T::EXPONENT_WIDTH);
    (mantissa, exponent)
}

#[inline]
fn _raw_mantissa<T: PrimitiveFloat>(x: T) -> u64 {
    x.to_bits().mod_power_of_2(T::MANTISSA_WIDTH)
}

#[inline]
fn _raw_exponent<T: PrimitiveFloat>(x: T) -> u64 {
    let exponent: u64 = (x.to_bits() >> T::MANTISSA_WIDTH).exact_into();
    exponent.mod_power_of_2(T::EXPONENT_WIDTH)
}

fn _from_raw_mantissa_and_exponent<T: PrimitiveFloat>(raw_mantissa: u64, raw_exponent: u64) -> T {
    assert!(raw_mantissa.significant_bits() <= T::MANTISSA_WIDTH);
    assert!(raw_exponent.significant_bits() <= T::EXPONENT_WIDTH);
    let x = T::from_bits((raw_exponent << T::MANTISSA_WIDTH) | raw_mantissa);
    // Only output the canonical NaN
    if x.is_nan() {
        T::NAN
    } else {
        x
    }
}

fn _integer_mantissa_and_exponent<T: PrimitiveFloat>(x: T) -> (u64, i64) {
    assert!(x.is_finite());
    assert!(x != T::ZERO);
    let (mut raw_mantissa, raw_exponent) = x.raw_mantissa_and_exponent();
    if raw_exponent == 0 {
        let trailing_zeros = raw_mantissa.trailing_zeros();
        (
            raw_mantissa >> trailing_zeros,
            i64::wrapping_from(trailing_zeros) + T::MIN_EXPONENT,
        )
    } else {
        raw_mantissa.set_bit(T::MANTISSA_WIDTH);
        let trailing_zeros = TrailingZeros::trailing_zeros(raw_mantissa);
        (
            raw_mantissa >> trailing_zeros,
            i64::wrapping_from(raw_exponent + trailing_zeros) + T::MIN_EXPONENT - 1,
        )
    }
}

fn _integer_mantissa<T: PrimitiveFloat>(x: T) -> u64 {
    assert!(x.is_finite());
    assert!(x != T::ZERO);
    let (mut raw_mantissa, raw_exponent) = x.raw_mantissa_and_exponent();
    if raw_exponent != 0 {
        raw_mantissa.set_bit(T::MANTISSA_WIDTH);
    }
    raw_mantissa >> raw_mantissa.trailing_zeros()
}

fn _integer_exponent<T: PrimitiveFloat>(x: T) -> i64 {
    assert!(x.is_finite());
    assert!(x != T::ZERO);
    let (raw_mantissa, raw_exponent) = x.raw_mantissa_and_exponent();
    if raw_exponent == 0 {
        i64::wrapping_from(raw_mantissa.trailing_zeros()) + T::MIN_EXPONENT
    } else {
        i64::wrapping_from(
            raw_exponent
                + if raw_mantissa == 0 {
                    T::MANTISSA_WIDTH
                } else {
                    TrailingZeros::trailing_zeros(raw_mantissa)
                },
        ) + T::MIN_EXPONENT
            - 1
    }
}

fn _from_integer_mantissa_and_exponent<T: PrimitiveFloat>(
    integer_mantissa: u64,
    integer_exponent: i64,
) -> Option<T> {
    if integer_mantissa == 0 {
        return Some(T::ZERO);
    }
    let trailing_zeros = integer_mantissa.trailing_zeros();
    let (integer_mantissa, adjusted_exponent) = (
        integer_mantissa >> trailing_zeros,
        integer_exponent + i64::wrapping_from(trailing_zeros),
    );
    let mantissa_bits = integer_mantissa.significant_bits();
    let sci_exponent = adjusted_exponent.checked_add(i64::exact_from(mantissa_bits))? - 1;
    let mut raw_mantissa;
    let raw_exponent;
    if sci_exponent < T::MIN_EXPONENT || sci_exponent > T::MAX_EXPONENT {
        return None;
    } else if sci_exponent < T::MIN_NORMAL_EXPONENT {
        if adjusted_exponent < T::MIN_EXPONENT {
            return None;
        } else {
            raw_exponent = 0;
            raw_mantissa = integer_mantissa << (adjusted_exponent - T::MIN_EXPONENT);
        }
    } else if mantissa_bits > T::MANTISSA_WIDTH + 1 {
        return None;
    } else {
        raw_exponent = u64::exact_from(sci_exponent + i64::low_mask(T::EXPONENT_WIDTH - 1));
        raw_mantissa = integer_mantissa << (T::MANTISSA_WIDTH + 1 - mantissa_bits);
        raw_mantissa.clear_bit(T::MANTISSA_WIDTH);
    }
    Some(T::from_raw_mantissa_and_exponent(
        raw_mantissa,
        raw_exponent,
    ))
}

fn _sci_mantissa_and_exponent<T: PrimitiveFloat>(x: T) -> (T, i64) {
    assert!(x.is_finite());
    assert!(x != T::ZERO);
    let (raw_mantissa, raw_exponent) = x.raw_mantissa_and_exponent();
    // Note that Self::MAX_EXPONENT is also the raw exponent of 1.0.
    if raw_exponent == 0 {
        let leading_zeros =
            LeadingZeros::leading_zeros(raw_mantissa) - (u64::WIDTH - T::MANTISSA_WIDTH);
        let mut mantissa = raw_mantissa << (leading_zeros + 1);
        mantissa.clear_bit(T::MANTISSA_WIDTH);
        (
            T::from_raw_mantissa_and_exponent(mantissa, u64::wrapping_from(T::MAX_EXPONENT)),
            i64::wrapping_from(T::MANTISSA_WIDTH - leading_zeros - 1) + T::MIN_EXPONENT,
        )
    } else {
        (
            T::from_raw_mantissa_and_exponent(raw_mantissa, u64::wrapping_from(T::MAX_EXPONENT)),
            i64::wrapping_from(raw_exponent) - T::MAX_EXPONENT,
        )
    }
}

fn _sci_mantissa<T: PrimitiveFloat>(x: T) -> T {
    assert!(x.is_finite());
    assert!(x != T::ZERO);
    let (mut mantissa, raw_exponent) = x.raw_mantissa_and_exponent();
    // Note that Self::MAX_EXPONENT is also the raw exponent of 1.0.
    if raw_exponent == 0 {
        mantissa <<= LeadingZeros::leading_zeros(mantissa) - (u64::WIDTH - T::MANTISSA_WIDTH) + 1;
        mantissa.clear_bit(T::MANTISSA_WIDTH);
    }
    T::from_raw_mantissa_and_exponent(mantissa, u64::wrapping_from(T::MAX_EXPONENT))
}

fn _sci_exponent<T: PrimitiveFloat>(x: T) -> i64 {
    assert!(x.is_finite());
    assert!(x != T::ZERO);
    let (raw_mantissa, raw_exponent) = x.raw_mantissa_and_exponent();
    // Note that Self::MAX_EXPONENT is also the raw exponent of 1.0.
    if raw_exponent == 0 {
        i64::wrapping_from(u64::WIDTH - LeadingZeros::leading_zeros(raw_mantissa) - 1)
            + T::MIN_EXPONENT
    } else {
        i64::wrapping_from(raw_exponent) - T::MAX_EXPONENT
    }
}

#[allow(clippy::wrong_self_convention)]
fn _from_sci_mantissa_and_exponent<T: PrimitiveFloat>(
    sci_mantissa: T,
    sci_exponent: i64,
) -> Option<T> {
    assert!(sci_mantissa.is_finite());
    assert!(sci_mantissa > T::ZERO);
    if sci_exponent < T::MIN_EXPONENT || sci_exponent > T::MAX_EXPONENT {
        return None;
    }
    let (mut orig_mantissa, orig_exponent) = sci_mantissa.raw_mantissa_and_exponent();
    // Note that Self::MAX_EXPONENT is also the raw exponent of 1.0.
    if orig_exponent != u64::wrapping_from(T::MAX_EXPONENT) {
        return None;
    }
    if sci_exponent < T::MIN_NORMAL_EXPONENT {
        let shift = T::MIN_NORMAL_EXPONENT - sci_exponent;
        if orig_mantissa.divisible_by_power_of_2(u64::wrapping_from(shift)) {
            orig_mantissa.set_bit(T::MANTISSA_WIDTH);
            Some(T::from_raw_mantissa_and_exponent(orig_mantissa >> shift, 0))
        } else {
            None
        }
    } else {
        Some(T::from_raw_mantissa_and_exponent(
            orig_mantissa,
            u64::wrapping_from(sci_exponent + T::MAX_EXPONENT),
        ))
    }
}

macro_rules! impl_mantissa_and_exponent {
    ($t:ident) => {
        impl RawMantissaAndExponent<u64, u64> for $t {
            /// Returns the raw mantissa and exponent.
            ///
            /// The raw exponent and raw mantissa are the actual bit patterns used to represent the
            /// components of `self`. When `self` is nonzero and finite, the raw exponent $e_r$ is
            /// an integer in $[0, 2^E-2]$ and the raw mantissa $m_r$ is an integer in
            /// $[0, 2^M-1]$.
            ///
            /// When `self` is nonzero and finite, $f(x) = (m_r, e_r)$, where
            /// $$
            /// m_r = \\begin{cases}
            ///     2^{M+2^{E-1}-2}|x| & |x| < 2^{2-2^{E-1}} \\\\
            ///     2^M \left ( \frac{|x|}{2^{\lfloor \log_2 |x| \rfloor}}-1\right ) &
            ///     \textrm{otherwise}
            /// \\end{cases}
            /// $$
            /// and
            /// $$
            /// e_r = \\begin{cases}
            ///     0 & |x| < 2^{2-2^{E-1}} \\\\
            ///     \lfloor \log_2 |x| \rfloor + 2^{E-1} - 1 & \textrm{otherwise}.
            /// \\end{cases}
            /// $$
            /// and $M$ and $E$ are the mantissa width and exponent width, respectively.
            ///
            /// For zeros, infinities, or `NaN`, refer to IEEE 754 or look at the examples below.
            ///
            /// The inverse operation is `from_raw_mantissa_and_exponent`.
            ///
            /// # Worst-case complexity
            /// Constant time and additional memory.
            ///
            /// # Examples
            /// See the documentation of the `num::conversion::mantissa_and_exponent` module.
            #[inline]
            fn raw_mantissa_and_exponent(self) -> (u64, u64) {
                _raw_mantissa_and_exponent(self)
            }

            /// Returns the raw mantissa.
            ///
            /// The raw mantissa is the actual bit pattern used to represent the mantissa of
            /// `self`. When `self` is nonzero and finite, it is an integer in $[0, 2^M-1]$.
            ///
            /// When `self` is nonzero and finite,
            /// $$
            /// f(x) = \\begin{cases}
            ///     2^{M+2^{E-1}-2}|x| & |x| < 2^{2-2^{E-1}} \\\\
            ///     2^M \left ( \frac{|x|}{2^{\lfloor \log_2 |x| \rfloor}}-1\right )
            ///     & \textrm{otherwise},
            /// \\end{cases}
            /// $$
            /// where $M$ and $E$ are the mantissa width and exponent width, respectively.
            ///
            /// For zeros, infinities, or `NaN`, refer to IEEE 754 or look at the examples below.
            ///
            /// # Worst-case complexity
            /// Constant time and additional memory.
            ///
            /// # Examples
            /// See the documentation of the `num::conversion::mantissa_and_exponent` module.
            #[inline]
            fn raw_mantissa(self) -> u64 {
                _raw_mantissa(self)
            }

            /// Returns the raw exponent.
            ///
            /// The raw exponent is the actual bit pattern used to represent the exponent of
            /// `self`. When `self` is nonzero and finite, it is an integer in $[0, 2^E-2]$.
            ///
            /// When `self` is nonzero and finite,
            /// $$
            /// f(x) = \\begin{cases}
            ///     0 & |x| < 2^{2-2^{E-1}} \\\\
            ///     \lfloor \log_2 |x| \rfloor + 2^{E-1} - 1 & \textrm{otherwise},
            /// \\end{cases}
            /// $$
            /// where $M$ and $E$ are the mantissa width and exponent width, respectively.
            ///
            /// For zeros, infinities, or `NaN`, refer to IEEE 754 or look at the examples below.
            ///
            /// # Worst-case complexity
            /// Constant time and additional memory.
            ///
            /// # Examples
            /// See the documentation of the `num::conversion::mantissa_and_exponent` module.
            #[inline]
            fn raw_exponent(self) -> u64 {
                _raw_exponent(self)
            }

            /// Constructs a float from its raw mantissa and exponent.
            ///
            /// The raw exponent and raw mantissa are the actual bit patterns used to represent the
            /// components of a float. When the float is nonzero and finite, the raw exponent $e_r$
            /// is an integer in $[0, 2^E-2]$ and the raw mantissa $m_r$ is an integer in
            /// $[0, 2^M-1]$.
            ///
            /// When the exponent is not `2^E-1`,
            /// $$
            /// f(m_r, e_r) = \\begin{cases}
            ///     2^{2-2^{E-1}-M}m_r & e_r = 0 \\\\
            ///     2^{e_r-2^{E-1}+1}(2^{-M}m_r+1) & \textrm{otherwise},
            /// \\end{cases}
            /// $$
            /// where $M$ and $E$ are the mantissa width and exponent width, respectively.
            ///
            /// For zeros, infinities, or `NaN`, refer to IEEE 754 or look at the examples below.
            ///
            /// This function only outputs a single, canonical `NaN`.
            ///
            /// # Worst-case complexity
            /// Constant time and additional memory.
            ///
            /// # Examples
            /// See the documentation of the `num::conversion::mantissa_and_exponent` module.
            #[inline]
            fn from_raw_mantissa_and_exponent(raw_mantissa: u64, raw_exponent: u64) -> $t {
                _from_raw_mantissa_and_exponent(raw_mantissa, raw_exponent)
            }
        }

        impl IntegerMantissaAndExponent<u64, i64> for $t {
            /// Returns the integer mantissa and exponent.
            ///
            /// When $x$ is positive, nonzero, and finite, we can write $x = 2^{e_i}m_i$, where
            /// $e_i$ is an integer and $m_i$ is an odd integer.
            /// $$
            /// f(x) = (\frac{|x|}{2^{e_i}}, e_i),
            /// $$
            /// where $e_i$ is the unique integer such that $x/2^{e_i}$ is an odd integer.
            ///
            /// The inverse operation is `from_integer_mantissa_and_exponent`.
            ///
            /// # Worst-case complexity
            /// Constant time and additional memory.
            ///
            /// # Panics
            /// Panics if `self` is zero, infinite, or `NaN`.
            ///
            /// # Examples
            /// See the documentation of the `num::conversion::mantissa_and_exponent` module.
            #[inline]
            fn integer_mantissa_and_exponent(self) -> (u64, i64) {
                _integer_mantissa_and_exponent(self)
            }

            /// Returns the integer mantissa.
            ///
            /// When $x$ is positive, nonzero, and finite, we can write $x = 2^{e_i}m_i$, where
            /// $e_i$ is an integer and $m_i$ is an odd integer.
            /// $$
            /// f(x) = \frac{|x|}{2^{e_i}},
            /// $$
            /// where $e_i$ is the unique integer such that $x/2^{e_i}$ is an odd integer.
            ///
            /// # Worst-case complexity
            /// Constant time and additional memory.
            ///
            /// # Panics
            /// Panics if `self` is zero, infinite, or `NaN`.
            ///
            /// # Examples
            /// See the documentation of the `num::conversion::mantissa_and_exponent` module.
            #[inline]
            fn integer_mantissa(self) -> u64 {
                _integer_mantissa(self)
            }

            /// Returns the integer exponent.
            ///
            /// When $x$ is positive, nonzero, and finite, we can write $x = 2^{e_i}m_i$, where
            /// $e_i$ is an integer and $m_i$ is an odd integer.
            /// $$
            /// f(x) = e_i,
            /// $$
            /// where $e_i$ is the unique integer such that $x/2^{e_i}$ is an odd integer.
            ///
            /// # Worst-case complexity
            /// Constant time and additional memory.
            ///
            /// # Panics
            /// Panics if `self` is zero, infinite, or `NaN`.
            ///
            /// # Examples
            /// See the documentation of the `num::conversion::mantissa_and_exponent` module.
            #[inline]
            fn integer_exponent(self) -> i64 {
                _integer_exponent(self)
            }

            /// Constructs a float from its integer mantissa and exponent.
            ///
            /// When $x$ is positive, nonzero, and finite, we can write $x = 2^{e_i}m_i$, where
            /// $e_i$ is an integer and $m_i$ is an odd integer.
            ///
            /// $$
            /// f(x) = 2^{e_i}m_i,
            /// $$
            /// or `None` if the result cannot be exactly represented as a float of the desired
            /// type (this happens if the exponent is too large or too small, or if the mantissa's
            /// precision is too high for the exponent).
            ///
            /// The input does not have to be reduced; that is, the mantissa does not have to be
            /// odd.
            ///
            /// # Worst-case complexity
            /// Constant time and additional memory.
            ///
            /// # Examples
            /// See the documentation of the `num::conversion::mantissa_and_exponent` module.
            #[inline]
            fn from_integer_mantissa_and_exponent(
                integer_mantissa: u64,
                integer_exponent: i64,
            ) -> Option<$t> {
                _from_integer_mantissa_and_exponent(integer_mantissa, integer_exponent)
            }
        }

        impl SciMantissaAndExponent<$t, i64> for $t {
            /// Returns the scientific mantissa and exponent.
            ///
            /// When $x$ is positive, nonzero, and finite, we can write $x = 2^{e_s}m_s$, where
            /// $e_s$ is an integer and $m_s$ is a rational number with $1 \leq m_s < 2$. If $x$ is
            /// a valid float, the scientific mantissa $m_s$ is always exactly representable as a
            /// float of the same type. We have
            /// $$
            /// f(x) = (\frac{x}{2^{\lfloor \log_2 x \rfloor}}, \lfloor \log_2 x \rfloor).
            /// $$
            ///
            /// The inverse operation is `from_sci_mantissa_and_exponent`.
            ///
            /// # Worst-case complexity
            /// Constant time and additional memory.
            ///
            /// # Panics
            /// Panics if `self` is zero, infinite, or `NaN`.
            ///
            /// # Examples
            /// See the documentation of the `num::conversion::mantissa_and_exponent` module.
            #[inline]
            fn sci_mantissa_and_exponent(self) -> ($t, i64) {
                _sci_mantissa_and_exponent(self)
            }

            /// Returns the scientific mantissa.
            ///
            /// When $x$ is positive, nonzero, and finite, we can write $x = 2^{e_s}m_s$, where
            /// $e_s$ is an integer and $m_s$ is a rational number with $1 \leq m_s < 2$. If $x$
            /// is a valid float, the scientific mantissa $m_s$ is always exactly representable as
            /// a float of the same type. We have
            /// $$
            /// f(x) = \frac{x}{2^{\lfloor \log_2 x \rfloor}}.
            /// $$
            ///
            /// # Worst-case complexity
            /// Constant time and additional memory.
            ///
            /// # Panics
            /// Panics if `self` is zero, infinite, or `NaN`.
            ///
            /// # Examples
            /// See the documentation of the `num::conversion::mantissa_and_exponent` module.
            #[inline]
            fn sci_mantissa(self) -> $t {
                _sci_mantissa(self)
            }

            /// Returns the scientific exponent.
            ///
            /// When $x$ is positive, nonzero, and finite, we can write $x = 2^{e_s}m_s$, where
            /// $e_s$ is an integer and $m_s$ is a rational number with $1 \leq m_s < 2$. We have
            /// $$
            /// f(x) = \lfloor \log_2 x \rfloor.
            /// $$
            ///
            /// # Worst-case complexity
            /// Constant time and additional memory.
            ///
            /// # Panics
            /// Panics if `self` is zero, infinite, or `NaN`.
            ///
            /// # Examples
            /// See the documentation of the `num::conversion::mantissa_and_exponent` module.
            #[inline]
            fn sci_exponent(self) -> i64 {
                _sci_exponent(self)
            }

            /// Constructs a float from its scientific mantissa and exponent.
            ///
            /// When $x$ is positive, nonzero, and finite, we can write $x = 2^{e_s}m_s$, where
            /// $e_s$ is an integer and $m_s$ is a rational number with $1 \leq m_s < 2$.
            ///
            /// $$
            /// f(x) = 2^{e_s}m_s,
            /// $$
            /// or `None` if the result cannot be exactly represented as a float of the desired
            /// type (this happens if the exponent is too large or too small, if the mantissa is
            /// not in the range $[1, 2)$, or if the mantissa's precision is too high for the
            /// exponent).
            ///
            /// # Worst-case complexity
            /// Constant time and additional memory.
            ///
            /// # Panics
            /// Panics if `mantissa` is zero, infinite, or `NaN`.
            ///
            /// # Examples
            /// See the documentation of the `num::conversion::mantissa_and_exponent` module.
            #[inline]
            fn from_sci_mantissa_and_exponent(sci_mantissa: $t, sci_exponent: i64) -> Option<$t> {
                _from_sci_mantissa_and_exponent(sci_mantissa, sci_exponent)
            }
        }
    };
}
apply_to_primitive_floats!(impl_mantissa_and_exponent);
