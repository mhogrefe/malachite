// Copyright © 2025 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::InnerFloat::Finite;
use crate::{significand_bits, Float};
use malachite_base::num::arithmetic::traits::{DivisibleByPowerOf2, NegAssign, PowerOf2};
use malachite_base::num::basic::integers::PrimitiveInt;
use malachite_base::num::basic::traits::{Infinity, Zero};
use malachite_base::num::conversion::traits::{ExactFrom, WrappingFrom};
use malachite_base::num::logic::traits::SignificantBits;
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
    /// f(x) = \operatorname{Some}(2^{\lfloor \log_2 x \rfloor-p+1}),
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
    /// assert_eq!(s.as_ref().map(|s| s.as_str()), Some("2.0e-30"));
    ///
    /// let s = Float::from(std::f64::consts::PI)
    ///     .ulp()
    ///     .map(|x| x.to_string());
    /// assert_eq!(s.as_ref().map(|s| s.as_str()), Some("4.0e-15"));
    ///
    /// let s = Float::power_of_2(100u64).ulp().map(|x| x.to_string());
    /// assert_eq!(s.as_ref().map(|s| s.as_str()), Some("1.0e30"));
    ///
    /// let s = Float::power_of_2(-100i64).ulp().map(|x| x.to_string());
    /// assert_eq!(s.as_ref().map(|s| s.as_str()), Some("8.0e-31"));
    ///
    /// let s = Float::NEGATIVE_ONE.ulp().map(|x| x.to_string());
    /// assert_eq!(s.as_ref().map(|s| s.as_str()), Some("1.0"));
    /// ```
    pub fn ulp(&self) -> Option<Float> {
        match self {
            Float(Finite {
                exponent,
                precision,
                ..
            }) => {
                let ulp_exponent =
                    i64::from(*exponent).checked_sub(i64::try_from(*precision).ok()?)?;
                if i32::try_from(ulp_exponent).ok()? >= Float::MIN_EXPONENT - 1 {
                    Some(Float::power_of_2(ulp_exponent))
                } else {
                    None
                }
            }
            _ => None,
        }
    }

    /// Increments a [`Float`] by its ulp. See [`Float::ulp`] for details.
    ///
    /// If the [`Float`] is positive and is the largest [`Float`] in its binade with its precision,
    /// then
    /// - If its exponent is not the maximum exponent, it will become the smallest [`Float`] in the
    ///   next-higher binade, and its precision will increase by 1 (so that its ulp remains the
    ///   same);
    /// - If its exponent is the maximum exponent, it will become $\infty$.
    ///
    /// If the [`Float`] is negative and is closer to zero than any other [`Float`] in its binade
    /// with its precision, then
    /// - If its precision is 1, it will become negative zero.
    /// - If its precision is greater than 1 and its exponent is not the minimum exponent, it will
    ///   become the farthest-from-zero [`Float`] in the next-lower binade, and its precision will
    ///   decrease by 1 (so that its ulp remains the same).
    /// - If its precision is greater than 1 and its exponent is the minimum exponent, it will
    ///   become negative zero.
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
    /// assert_eq!(x.to_string(), "1.0");
    /// x.increment();
    /// assert_eq!(x.to_string(), "1.000000000000000000000000000002");
    ///
    /// let mut x = Float::from(std::f64::consts::PI);
    /// assert_eq!(x.to_string(), "3.141592653589793");
    /// x.increment();
    /// assert_eq!(x.to_string(), "3.141592653589797");
    ///
    /// let mut x = Float::power_of_2(100u64);
    /// assert_eq!(x.to_string(), "1.0e30");
    /// x.increment();
    /// assert_eq!(x.to_string(), "3.0e30");
    ///
    /// let mut x = Float::power_of_2(-100i64);
    /// assert_eq!(x.to_string(), "8.0e-31");
    /// x.increment();
    /// assert_eq!(x.to_string(), "1.6e-30");
    ///
    /// let mut x = Float::NEGATIVE_ONE;
    /// assert_eq!(x.to_string(), "-1.0");
    /// x.increment();
    /// assert_eq!(x.to_string(), "-0.0");
    /// ```
    pub fn increment(&mut self) {
        if self.is_sign_negative() {
            self.neg_assign();
            self.decrement();
            self.neg_assign();
        } else if let Float(Finite {
            exponent,
            precision,
            significand,
            ..
        }) = self
        {
            let ulp = Limb::power_of_2(significand_bits(significand) - *precision);
            let limb_count = significand.limb_count();
            significand.add_assign_at_limb(
                usize::wrapping_from(limb_count)
                    - 1
                    - usize::exact_from((*precision - 1) >> Limb::LOG_WIDTH),
                ulp,
            );
            if significand.limb_count() > limb_count {
                if *exponent == Float::MAX_EXPONENT {
                    *self = Float::INFINITY;
                    return;
                }
                *significand >>= 1;
                *exponent += 1;
                if precision.divisible_by_power_of_2(Limb::LOG_WIDTH) {
                    *significand <<= Limb::WIDTH;
                }
                *precision = precision.checked_add(1).unwrap();
            }
        } else {
            panic!("Cannot increment float is non-finite or zero");
        }
    }

    /// Decrements a [`Float`] by its ulp. See [`Float::ulp`] for details.
    ///
    /// If the [`Float`] is negative and is the largest [`Float`] in its binade with its precision,
    /// then
    /// - If its exponent is not the maximum exponent, it will become the closest-to-zero [`Float`]
    ///   in the next-higher binade, and its precision will increase by 1 (so that its ulp remains
    ///   the same);
    /// - If its exponent is the maximum exponent, it will become $-\infty$.
    ///
    /// If the [`Float`] is positive and is smaller than any other [`Float`] in its binade with its
    /// precision, then
    /// - If its precision is 1, it will become positive zero.
    /// - If its precision is greater than 1 and its exponent is not the minimum exponent, it will
    ///   become the largest [`Float`] in the next-lower binade, and its precision will decrease by
    ///   1 (so that its ulp remains the same).
    /// - If its precision is greater than 1 and its exponent is the minimum exponent, it will
    ///   become positive zero.
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
    /// assert_eq!(x.to_string(), "0.0");
    ///
    /// let mut x = Float::one_prec(100);
    /// assert_eq!(x.to_string(), "1.0");
    /// x.decrement();
    /// assert_eq!(x.to_string(), "0.999999999999999999999999999998");
    ///
    /// let mut x = Float::from(std::f64::consts::PI);
    /// assert_eq!(x.to_string(), "3.141592653589793");
    /// x.decrement();
    /// assert_eq!(x.to_string(), "3.14159265358979");
    ///
    /// let mut x = Float::power_of_2(100u64);
    /// assert_eq!(x.to_string(), "1.0e30");
    /// x.decrement();
    /// assert_eq!(x.to_string(), "0.0");
    ///
    /// let mut x = Float::power_of_2(-100i64);
    /// assert_eq!(x.to_string(), "8.0e-31");
    /// x.decrement();
    /// assert_eq!(x.to_string(), "0.0");
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
        } else if let Float(Finite {
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
                    - usize::exact_from((*precision - 1) >> Limb::LOG_WIDTH),
                ulp,
            );
            if *significand == 0u32 {
                *self = Float::ZERO;
            } else if significand.significant_bits() < bits {
                if *exponent == Float::MIN_EXPONENT {
                    *self = Float::ZERO;
                    return;
                }
                *significand <<= 1;
                *exponent -= 1;
                *precision = precision.checked_sub(1).unwrap();
                if bits - *precision == Limb::WIDTH {
                    *significand >>= Limb::WIDTH;
                }
            }
        } else {
            panic!("Cannot decrement float that is non-finite or zero");
        }
    }
}
