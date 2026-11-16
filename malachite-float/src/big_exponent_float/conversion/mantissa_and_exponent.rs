// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.
use crate::big_exponent_float::BigExponentFloat;
use crate::big_exponent_float::InnerBigExponentFloat::Finite;
use crate::big_exponent_float::arithmetic::mul_rational::shift_finite;
use core::cmp::Ordering::{self, *};
use malachite_base::num::basic::floats::PrimitiveFloat;
use malachite_base::num::basic::integers::PrimitiveInt;
use malachite_base::num::basic::traits::{One, Zero};
use malachite_base::num::conversion::traits::{
    IntegerMantissaAndExponent, RawMantissaAndExponent, SciMantissaAndExponent,
};
use malachite_base::num::logic::traits::SignificantBits;
use malachite_base::num::modular::traits::DivisibleByPowerOf2;
use malachite_base::rounding_modes::RoundingMode;
use malachite_nz::integer::Integer;
use malachite_nz::natural::Natural;
use malachite_nz::platform::Limb;

impl BigExponentFloat {
    /// Returns a [`BigExponentFloat`]'s scientific mantissa and exponent, rounding the mantissa to
    /// a primitive float. The sign is ignored.
    ///
    /// The scientific mantissa of a nonzero $x$ is $|x|/2^e$, where $e$ is the scientific exponent
    /// $\lfloor \log_2 |x| \rfloor$; it lies in $[1, 2)$. Rounding the mantissa may carry it to 2,
    /// in which case it becomes 1 and the exponent is incremented. An [`Ordering`] is also
    /// returned, indicating how the rounded mantissa compares with the exact one. `None` is
    /// returned if the [`BigExponentFloat`] is NaN, infinite, or zero, or if `rm` is `Exact` and
    /// the mantissa is not exactly representable.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `self.complexity()`.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::{BigExponentFloat, Float};
    /// use std::cmp::Ordering::*;
    ///
    /// let x = BigExponentFloat::from(Float::from(6.0));
    /// let (m, e, o) = x.sci_mantissa_and_exponent_round::<f32>(Nearest).unwrap();
    /// assert_eq!((m, e.to_string(), o), (1.5, "2".to_string(), Equal));
    /// ```
    pub fn sci_mantissa_and_exponent_round<T: PrimitiveFloat>(
        &self,
        rm: RoundingMode,
    ) -> Option<(T, Integer, Ordering)> {
        match self {
            Self(Finite {
                exponent,
                significand,
                ..
            }) => significand
                .sci_mantissa_and_exponent_round::<T>(rm)
                .map(|(m, _, o)| {
                    (
                        m,
                        if o == Greater && m == T::ONE {
                            exponent.clone()
                        } else {
                            exponent - Integer::ONE
                        },
                        o,
                    )
                }),
            _ => None,
        }
    }
}

// Whether `x` is a valid significand: positive, with a number of bits divisible by the limb width,
// and with a nonzero lowest limb.
fn is_valid_significand(x: &Natural) -> bool {
    let bits = x.significant_bits();
    bits != 0
        && bits.divisible_by_power_of_2(Limb::LOG_WIDTH)
        && !x.divisible_by_power_of_2(Limb::WIDTH)
}

impl RawMantissaAndExponent<Natural, Integer> for BigExponentFloat {
    /// Returns the raw mantissa and exponent of a [`BigExponentFloat`], taking it by value.
    ///
    /// The raw mantissa is the significand, as returned by [`BigExponentFloat::to_significand`],
    /// and the raw exponent is the exponent, as returned by [`BigExponentFloat::get_exponent`]: a
    /// finite nonzero $x$ is $\pm m 2^{e-b}$, where $m$ is the raw mantissa, $b$ its number of
    /// bits, and $e$ the raw exponent. The sign is ignored.
    ///
    /// # Worst-case complexity
    /// Constant time and additional memory.
    ///
    /// # Panics
    /// Panics if the [`BigExponentFloat`] is NaN, infinite, or zero.
    ///
    /// # Examples
    /// See [here](super::mantissa_and_exponent#raw_mantissa_and_exponent).
    fn raw_mantissa_and_exponent(self) -> (Natural, Integer) {
        if let Self(Finite {
            exponent,
            significand,
            ..
        }) = self
        {
            (significand, exponent)
        } else {
            panic!("{self:?} has no raw mantissa and exponent")
        }
    }

    /// Constructs a positive [`BigExponentFloat`] from its raw mantissa and exponent.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(1)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `raw_mantissa.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `raw_mantissa` is zero, if its number of significant bits is not divisible by the
    /// limb width, or if its lowest limb is zero.
    ///
    /// # Examples
    /// See [here](super::mantissa_and_exponent#from_raw_mantissa_and_exponent).
    fn from_raw_mantissa_and_exponent(raw_mantissa: Natural, raw_exponent: Integer) -> Self {
        assert!(
            is_valid_significand(&raw_mantissa),
            "Invalid raw mantissa: {raw_mantissa}"
        );
        Self(Finite {
            sign: true,
            exponent: raw_exponent,
            significand: raw_mantissa,
        })
    }
}

impl RawMantissaAndExponent<Natural, Integer, BigExponentFloat> for &BigExponentFloat {
    /// Returns the raw mantissa and exponent of a [`BigExponentFloat`], taking it by reference.
    ///
    /// See the by-value implementation for details.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `self.complexity()`.
    ///
    /// # Panics
    /// Panics if the [`BigExponentFloat`] is NaN, infinite, or zero.
    ///
    /// # Examples
    /// See [here](super::mantissa_and_exponent#raw_mantissa_and_exponent).
    fn raw_mantissa_and_exponent(self) -> (Natural, Integer) {
        if let BigExponentFloat(Finite {
            exponent,
            significand,
            ..
        }) = self
        {
            (significand.clone(), exponent.clone())
        } else {
            panic!("{self:?} has no raw mantissa and exponent")
        }
    }

    /// Constructs a positive [`BigExponentFloat`] from its raw mantissa and exponent.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(1)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `raw_mantissa.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `raw_mantissa` is zero, if its number of significant bits is not divisible by the
    /// limb width, or if its lowest limb is zero.
    ///
    /// # Examples
    /// See [here](super::mantissa_and_exponent#from_raw_mantissa_and_exponent).
    #[inline]
    fn from_raw_mantissa_and_exponent(
        raw_mantissa: Natural,
        raw_exponent: Integer,
    ) -> BigExponentFloat {
        BigExponentFloat::from_raw_mantissa_and_exponent(raw_mantissa, raw_exponent)
    }
}

impl IntegerMantissaAndExponent<Natural, Integer> for BigExponentFloat {
    /// Returns the integer mantissa and exponent of a [`BigExponentFloat`], taking it by value.
    ///
    /// The integer mantissa is the odd integer $m$ and the integer exponent the integer $e$ such
    /// that the absolute value of the [`BigExponentFloat`] is $m2^e$. The sign is ignored.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(1)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `self.significant_bits()`.
    ///
    /// # Panics
    /// Panics if the [`BigExponentFloat`] is NaN, infinite, or zero.
    ///
    /// # Examples
    /// See [here](super::mantissa_and_exponent#integer_mantissa_and_exponent).
    fn integer_mantissa_and_exponent(self) -> (Natural, Integer) {
        if let Self(Finite {
            exponent,
            significand,
            ..
        }) = self
        {
            let zeros = significand.trailing_zeros().unwrap();
            let shifted = significand >> zeros;
            let bits = shifted.significant_bits();
            (shifted, exponent - Integer::from(bits))
        } else {
            panic!("{self:?} has no integer mantissa and exponent")
        }
    }

    /// Constructs a [`BigExponentFloat`] from an integer mantissa and exponent: the result is
    /// $m2^e$. Since the exponent is unbounded, this always succeeds.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(1)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is
    /// `integer_mantissa.significant_bits()`.
    ///
    /// # Examples
    /// See [here](super::mantissa_and_exponent#from_integer_mantissa_and_exponent).
    fn from_integer_mantissa_and_exponent(
        integer_mantissa: Natural,
        integer_exponent: Integer,
    ) -> Option<Self> {
        Some(if integer_mantissa == 0u32 {
            Self::ZERO
        } else {
            let bits = integer_mantissa.significant_bits();
            Self::from_parts(
                true,
                integer_exponent + Integer::from(bits),
                integer_mantissa,
            )
        })
    }
}

impl IntegerMantissaAndExponent<Natural, Integer, BigExponentFloat> for &BigExponentFloat {
    /// Returns the integer mantissa and exponent of a [`BigExponentFloat`], taking it by reference.
    ///
    /// See the by-value implementation for details.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `self.complexity()`.
    ///
    /// # Panics
    /// Panics if the [`BigExponentFloat`] is NaN, infinite, or zero.
    ///
    /// # Examples
    /// See [here](super::mantissa_and_exponent#integer_mantissa_and_exponent).
    #[inline]
    fn integer_mantissa_and_exponent(self) -> (Natural, Integer) {
        self.clone().integer_mantissa_and_exponent()
    }

    /// Constructs a [`BigExponentFloat`] from an integer mantissa and exponent: the result is
    /// $m2^e$. Since the exponent is unbounded, this always succeeds.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(1)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is
    /// `integer_mantissa.significant_bits()`.
    ///
    /// # Examples
    /// See [here](super::mantissa_and_exponent#from_integer_mantissa_and_exponent).
    #[inline]
    fn from_integer_mantissa_and_exponent(
        integer_mantissa: Natural,
        integer_exponent: Integer,
    ) -> Option<BigExponentFloat> {
        BigExponentFloat::from_integer_mantissa_and_exponent(integer_mantissa, integer_exponent)
    }
}

impl SciMantissaAndExponent<Self, Integer> for BigExponentFloat {
    /// Returns the scientific mantissa and exponent of a [`BigExponentFloat`], taking it by value.
    ///
    /// The scientific mantissa is $|x|/2^e$, which lies in $[1, 2)$, and the scientific exponent
    /// $e$ is $\lfloor \log_2 |x| \rfloor$. The sign is ignored.
    ///
    /// # Worst-case complexity
    /// Constant time and additional memory.
    ///
    /// # Panics
    /// Panics if the [`BigExponentFloat`] is NaN, infinite, or zero.
    ///
    /// # Examples
    /// See [here](super::mantissa_and_exponent#sci_mantissa_and_exponent).
    fn sci_mantissa_and_exponent(mut self) -> (Self, Integer) {
        if let Self(Finite { sign, exponent, .. }) = &mut self {
            let old_exponent = core::mem::replace(exponent, Integer::ONE);
            *sign = true;
            (self, old_exponent - Integer::ONE)
        } else {
            panic!("{self:?} has no scientific mantissa and exponent")
        }
    }

    /// Constructs a [`BigExponentFloat`] from a scientific mantissa and exponent: the result is
    /// $m2^e$. `None` is returned if the mantissa is negative or not in $[1, 2)$.
    ///
    /// # Worst-case complexity
    /// Constant time and additional memory.
    ///
    /// # Panics
    /// Panics if `sci_mantissa` is NaN, infinite, or zero.
    ///
    /// # Examples
    /// See [here](super::mantissa_and_exponent#from_sci_mantissa_and_exponent).
    fn from_sci_mantissa_and_exponent(
        mut sci_mantissa: Self,
        sci_exponent: Integer,
    ) -> Option<Self> {
        assert!(sci_mantissa.is_normal(), "Invalid scientific mantissa");
        if let Self(Finite { sign, exponent, .. }) = &mut sci_mantissa {
            if !*sign || *exponent != 1u32 {
                return None;
            }
            *exponent = sci_exponent + Integer::ONE;
        }
        Some(sci_mantissa)
    }
}

impl SciMantissaAndExponent<BigExponentFloat, Integer, BigExponentFloat> for &BigExponentFloat {
    /// Returns the scientific mantissa and exponent of a [`BigExponentFloat`], taking it by
    /// reference.
    ///
    /// See the by-value implementation for details.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `self.complexity()`.
    ///
    /// # Panics
    /// Panics if the [`BigExponentFloat`] is NaN, infinite, or zero.
    ///
    /// # Examples
    /// See [here](super::mantissa_and_exponent#sci_mantissa_and_exponent).
    #[inline]
    fn sci_mantissa_and_exponent(self) -> (BigExponentFloat, Integer) {
        self.clone().sci_mantissa_and_exponent()
    }

    /// Constructs a [`BigExponentFloat`] from a scientific mantissa and exponent: the result is
    /// $m2^e$. `None` is returned if the mantissa is negative or not in $[1, 2)$.
    ///
    /// # Worst-case complexity
    /// Constant time and additional memory.
    ///
    /// # Panics
    /// Panics if `sci_mantissa` is NaN, infinite, or zero.
    ///
    /// # Examples
    /// See [here](super::mantissa_and_exponent#from_sci_mantissa_and_exponent).
    #[inline]
    fn from_sci_mantissa_and_exponent(
        sci_mantissa: BigExponentFloat,
        sci_exponent: Integer,
    ) -> Option<BigExponentFloat> {
        BigExponentFloat::from_sci_mantissa_and_exponent(sci_mantissa, sci_exponent)
    }
}

macro_rules! impl_mantissa_and_exponent {
    ($t:ident) => {
        impl SciMantissaAndExponent<$t, Integer, BigExponentFloat> for &BigExponentFloat {
            /// Returns a [`BigExponentFloat`]'s scientific mantissa and exponent, taking the
            /// [`BigExponentFloat`] by reference.
            ///
            /// When $x$ is finite and nonzero, we can write $|x| = 2^{e_s}m_s$, where $e_s$ is an
            /// integer and $m_s$ is a rational number with $1 \leq m_s < 2$. We represent the
            /// rational mantissa as a primitive float. The conversion might not be exact, so we
            /// round to the nearest float using the `Nearest` rounding mode. To use other rounding
            /// modes, use [`BigExponentFloat::sci_mantissa_and_exponent_round`].
            ///
            /// $$
            /// f(x) \approx (\frac{|x|}{2^{\lfloor \log_2 |x| \rfloor}},
            /// \lfloor \log_2 |x| \rfloor).
            /// $$
            ///
            /// Unlike the [`Float`](crate::Float) version, the exponent is an [`Integer`] and can
            /// have any size.
            ///
            /// # Worst-case complexity
            /// $T(n) = O(n)$
            ///
            /// $M(n) = O(n)$
            ///
            /// where $T$ is time, $M$ is additional memory, and $n$ is `self.complexity()`.
            ///
            /// # Panics
            /// Panics if `self` is NaN, infinite, or zero.
            ///
            /// # Examples
            /// See [here](super::mantissa_and_exponent#sci_mantissa_and_exponent).
            #[inline]
            fn sci_mantissa_and_exponent(self) -> ($t, Integer) {
                let (m, e, _) = self
                    .sci_mantissa_and_exponent_round(RoundingMode::Nearest)
                    .unwrap();
                (m, e)
            }

            /// Constructs a [`BigExponentFloat`] from its scientific mantissa and exponent: the
            /// result is $m2^e$.
            ///
            /// If the mantissa is zero or not finite, this function panics. If it is finite but not
            /// in the interval $[1, 2)$, `None` is returned. Unlike the [`Float`](crate::Float)
            /// version, `None` is never returned because of the size of the exponent.
            ///
            /// # Worst-case complexity
            /// $T(n) = O(n)$
            ///
            /// $M(n) = O(n)$
            ///
            /// where $T$ is time, $M$ is additional memory, and $n$ is
            /// `sci_exponent.significant_bits()`.
            ///
            /// # Panics
            /// Panics if `sci_mantissa` is zero or not finite.
            ///
            /// # Examples
            /// See [here](super::mantissa_and_exponent#from_sci_mantissa_and_exponent).
            #[allow(clippy::manual_range_contains)]
            #[inline]
            fn from_sci_mantissa_and_exponent(
                sci_mantissa: $t,
                sci_exponent: Integer,
            ) -> Option<BigExponentFloat> {
                assert!(sci_mantissa.is_finite());
                assert_ne!(sci_mantissa, 0.0);
                if sci_mantissa < 1.0 || sci_mantissa >= 2.0 {
                    None
                } else {
                    Some(shift_finite(
                        BigExponentFloat::from(sci_mantissa),
                        &sci_exponent,
                    ))
                }
            }
        }
    };
}
apply_to_primitive_floats!(impl_mantissa_and_exponent);
