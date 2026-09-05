// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::InnerFloat::Finite;
use crate::{Float, significand_bits};
use malachite_base::num::arithmetic::traits::{NegAssign, PowerOf2};
use malachite_base::num::basic::traits::{Infinity, Zero};
use malachite_base::num::conversion::traits::WrappingFrom;
use malachite_base::num::logic::traits::{BitAccess, SignificantBits};
use malachite_nz::natural::{Natural, bit_to_limb_count_floor};
use malachite_nz::platform::Limb;

impl Float {
    /// Gets a [`Float`]'s ulp (unit in last place, or unit of least precision).
    ///
    /// If the [`Float`] is positive, its ulp is the distance to the next-largest [`Float`] with the
    /// same precision; if it is negative, the next-smallest. (This definition works even if the
    /// [`Float`] is the largest in its binade. If the [`Float`] is the largest in its binade and
    /// has the maximum exponent, we can define its ulp to be the distance to the next-smallest
    /// [`Float`] with the same precision if positive, and to the next-largest [`Float`] with the
    /// same precision if negative.)
    ///
    /// If the [`Float`] is NaN, infinite, or zero, then `None` is returned.
    ///
    /// This function does not overflow or underflow, technically. But it is possible that a
    /// [`Float`]'s ulp is too small to represent, for example if the [`Float`] has the minimum
    /// exponent and its precision is greater than 1, or if the precision is extremely large in
    /// general. In such cases, `None` is returned.
    ///
    /// $$
    /// f(\text{NaN}) = f(\pm\infty) = f(\pm 0.0) = \text{None},
    /// $$
    ///
    /// and, if $x$ is finite and nonzero,
    ///
    /// $$
    /// f(x) = \operatorname{Some}(2^{\lfloor \log_2 |x| \rfloor-p+1}),
    /// $$
    /// where $p$ is the precision of $x$.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `self.significant_bits()`.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::PowerOf2;
    /// use malachite_base::num::basic::traits::{Infinity, NaN, NegativeOne, One, Zero};
    /// use malachite_float::Float;
    ///
    /// assert_eq!(Float::NAN.ulp(), None);
    /// assert_eq!(Float::INFINITY.ulp(), None);
    /// assert_eq!(Float::ZERO.ulp(), None);
    ///
    /// let s = Float::ONE.ulp().map(|x| x.to_string());
    /// assert_eq!(s.as_ref().map(|s| s.as_str()), Some("1.0"));
    ///
    /// let s = Float::one_prec(100).ulp().map(|x| x.to_string());
    /// assert_eq!(s.as_ref().map(|s| s.as_str()), Some("1.6e-30"));
    ///
    /// let s = Float::from(std::f64::consts::PI)
    ///     .ulp()
    ///     .map(|x| x.to_string());
    /// assert_eq!(s.as_ref().map(|s| s.as_str()), Some("3.6e-15"));
    ///
    /// let s = Float::power_of_2(100u64).ulp().map(|x| x.to_string());
    /// assert_eq!(s.as_ref().map(|s| s.as_str()), Some("1.3e30"));
    ///
    /// let s = Float::power_of_2(-100i64).ulp().map(|x| x.to_string());
    /// assert_eq!(s.as_ref().map(|s| s.as_str()), Some("7.9e-31"));
    ///
    /// let s = Float::NEGATIVE_ONE.ulp().map(|x| x.to_string());
    /// assert_eq!(s.as_ref().map(|s| s.as_str()), Some("1.0"));
    /// ```
    pub fn ulp(&self) -> Option<Self> {
        match self {
            Self(Finite {
                exponent,
                precision,
                ..
            }) => {
                let ulp_exponent =
                    i64::from(*exponent).checked_sub(i64::try_from(*precision).ok()?)?;
                if i32::try_from(ulp_exponent).ok()? >= Self::MIN_EXPONENT_MINUS_1 {
                    Some(Self::power_of_2(ulp_exponent))
                } else {
                    None
                }
            }
            _ => None,
        }
    }

    /// Steps a [`Float`] up to the closest larger [`Float`] with the same precision. This matches
    /// the IEEE 754 `nextUp` operation and MPFR's `mpfr_nextabove`, except that this function
    /// panics on NaN, infinities, and zeros rather than handling them.
    ///
    /// For most values this adds one ulp (see [`Float::ulp`]). If the [`Float`] is positive and is
    /// the largest [`Float`] in its binade with its precision, then
    /// - If its exponent is not the maximum exponent, it will become the power of 2 at the bottom
    ///   of the next-higher binade (still a step of one ulp);
    /// - If its exponent is the maximum exponent, it will become $\infty$.
    ///
    /// If the [`Float`] is negative and is closer to zero than any other [`Float`] in its binade
    /// with its precision (that is, its significand is a power of 2), then
    /// - If its exponent is not the minimum exponent, it will move half an ulp toward zero, to the
    ///   largest-magnitude [`Float`] in the next-lower binade with its precision (at precision 1
    ///   the next power of 2, at higher precisions the value with an all-ones significand);
    /// - If its exponent is the minimum exponent, it will become negative zero.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `self.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `self` is NaN, infinite, or zero.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::PowerOf2;
    /// use malachite_base::num::basic::traits::{NegativeOne, One};
    /// use malachite_float::Float;
    ///
    /// let mut x = Float::ONE;
    /// assert_eq!(x.to_string(), "1.0");
    /// x.increment();
    /// assert_eq!(x.to_string(), "2.0");
    ///
    /// let mut x = Float::one_prec(100);
    /// assert_eq!(x.to_string(), "1.0000000000000000000000000000000");
    /// x.increment();
    /// assert_eq!(x.to_string(), "1.0000000000000000000000000000016");
    ///
    /// let mut x = Float::from(std::f64::consts::PI);
    /// assert_eq!(x.to_string(), "3.1415926535897931");
    /// x.increment();
    /// assert_eq!(x.to_string(), "3.1415926535897967");
    ///
    /// let mut x = Float::power_of_2(100u64);
    /// assert_eq!(x.to_string(), "1.3e30");
    /// x.increment();
    /// assert_eq!(x.to_string(), "2.5e30");
    ///
    /// let mut x = Float::power_of_2(-100i64);
    /// assert_eq!(x.to_string(), "7.9e-31");
    /// x.increment();
    /// assert_eq!(x.to_string(), "1.6e-30");
    ///
    /// let mut x = Float::NEGATIVE_ONE;
    /// assert_eq!(x.to_string(), "-1.0");
    /// x.increment();
    /// assert_eq!(x.to_string(), "-0.50");
    /// ```
    pub fn increment(&mut self) {
        if self.is_sign_negative() {
            self.neg_assign();
            self.decrement();
            self.neg_assign();
        } else if let Self(Finite {
            exponent,
            precision,
            significand,
            ..
        }) = self
        {
            let ulp = Limb::power_of_2(significand_bits(significand) - *precision);
            let limb_count = significand.limb_count();
            significand.add_assign_at_limb(
                usize::wrapping_from(limb_count) - 1 - bit_to_limb_count_floor(*precision - 1),
                ulp,
            );
            if significand.limb_count() > limb_count {
                // The value was the largest in its binade with its precision, so stepping up lands
                // on the power of 2 at the bottom of the next-higher binade, which is representable
                // with the same precision.
                if *exponent == Self::MAX_EXPONENT {
                    *self = Self::INFINITY;
                    return;
                }
                *significand >>= 1u32;
                *exponent += 1;
            }
        } else {
            panic!("Cannot increment float is non-finite or zero");
        }
    }

    /// Steps a [`Float`] down to the closest smaller [`Float`] with the same precision. This
    /// matches the IEEE 754 `nextDown` operation and MPFR's `mpfr_nextbelow`, except that this
    /// function panics on NaN, infinities, and zeros rather than handling them.
    ///
    /// For most values this subtracts one ulp (see [`Float::ulp`]). If the [`Float`] is negative
    /// and is the largest-magnitude [`Float`] in its binade with its precision, then
    /// - If its exponent is not the maximum exponent, it will become the negative power of 2 at the
    ///   bottom of the next-higher binade (still a step of one ulp);
    /// - If its exponent is the maximum exponent, it will become $-\infty$.
    ///
    /// If the [`Float`] is positive and is smaller than any other [`Float`] in its binade with its
    /// precision (that is, its significand is a power of 2), then
    /// - If its exponent is not the minimum exponent, it will move half an ulp toward zero, to the
    ///   largest [`Float`] in the next-lower binade with its precision (at precision 1 the next
    ///   power of 2, at higher precisions the value with an all-ones significand);
    /// - If its exponent is the minimum exponent, it will become positive zero.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `self.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `self` is NaN, infinite, or zero.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::PowerOf2;
    /// use malachite_base::num::basic::traits::{NegativeOne, One};
    /// use malachite_float::Float;
    ///
    /// let mut x = Float::ONE;
    /// assert_eq!(x.to_string(), "1.0");
    /// x.decrement();
    /// assert_eq!(x.to_string(), "0.50");
    ///
    /// let mut x = Float::one_prec(100);
    /// assert_eq!(x.to_string(), "1.0000000000000000000000000000000");
    /// x.decrement();
    /// assert_eq!(x.to_string(), "0.99999999999999999999999999999921");
    ///
    /// let mut x = Float::from(std::f64::consts::PI);
    /// assert_eq!(x.to_string(), "3.1415926535897931");
    /// x.decrement();
    /// assert_eq!(x.to_string(), "3.1415926535897896");
    ///
    /// let mut x = Float::power_of_2(100u64);
    /// assert_eq!(x.to_string(), "1.3e30");
    /// x.decrement();
    /// assert_eq!(x.to_string(), "6.3e29");
    ///
    /// let mut x = Float::power_of_2(-100i64);
    /// assert_eq!(x.to_string(), "7.9e-31");
    /// x.decrement();
    /// assert_eq!(x.to_string(), "3.9e-31");
    ///
    /// let mut x = Float::NEGATIVE_ONE;
    /// assert_eq!(x.to_string(), "-1.0");
    /// x.decrement();
    /// assert_eq!(x.to_string(), "-2.0");
    /// ```
    pub fn decrement(&mut self) {
        if self.is_sign_negative() {
            self.neg_assign();
            self.increment();
            self.neg_assign();
        } else if let Self(Finite {
            exponent,
            precision,
            significand,
            ..
        }) = self
        {
            let bits = significand_bits(significand);
            let ulp = Limb::power_of_2(bits - *precision);
            significand.sub_assign_at_limb(
                usize::wrapping_from(significand.limb_count())
                    - 1
                    - bit_to_limb_count_floor(*precision - 1),
                ulp,
            );
            if *significand == 0u32 {
                // The value was a power of 2 with precision 1, so stepping down lands on the next
                // power of 2, unless that is out of range.
                if *exponent == Self::MIN_EXPONENT {
                    *self = Self::ZERO;
                } else {
                    *significand = Natural::power_of_2(bits - 1);
                    *exponent -= 1;
                }
            } else if significand.significant_bits() < bits {
                // The value was a power of 2 with precision greater than 1, so stepping down
                // crosses into the next-lower binade, where the closest value is half an ulp away
                // and has an all-ones significand with the same precision — unless the lower
                // binade is out of range.
                if *exponent == Self::MIN_EXPONENT {
                    *self = Self::ZERO;
                    return;
                }
                significand.set_bit(bits - 1);
                *exponent -= 1;
            }
        } else {
            panic!("Cannot decrement float that is non-finite or zero");
        }
    }
}
