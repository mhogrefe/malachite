// Copyright © 2024 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::num::arithmetic::traits::{
    CeilingLogBasePowerOf2, CheckedLogBasePowerOf2, DivMod, DivRound, FloorLogBasePowerOf2,
};
use crate::num::basic::unsigneds::PrimitiveUnsigned;
use crate::num::conversion::traits::{ExactFrom, SciMantissaAndExponent};
use crate::rounding_modes::RoundingMode::*;

#[cfg(feature = "test_build")]
pub fn ceiling_log_base_power_of_2_naive<T: PrimitiveUnsigned>(x: T, pow: u64) -> u64 {
    assert_ne!(x, T::ZERO);
    assert_ne!(pow, 0);
    if pow >= T::WIDTH {
        return u64::from(x != T::ONE);
    }
    let mut result = 0;
    let mut p = T::ONE;
    while p < x {
        let highest_possible = p.leading_zeros() < pow;
        result += 1;
        if highest_possible {
            break;
        }
        p <<= pow;
    }
    result
}

fn floor_log_base_power_of_2<T: PrimitiveUnsigned>(x: T, pow: u64) -> u64 {
    assert!(x != T::ZERO, "Cannot take the base-2 logarithm of 0.");
    assert_ne!(pow, 0);
    (x.significant_bits() - 1) / pow
}

fn ceiling_log_base_power_of_2<T: PrimitiveUnsigned>(x: T, pow: u64) -> u64 {
    assert!(x != T::ZERO, "Cannot take the base-2 logarithm of 0.");
    assert_ne!(pow, 0);
    let (floor_log, rem) = (x.significant_bits() - 1).div_mod(pow);
    if rem == 0 && T::is_power_of_2(&x) {
        floor_log
    } else {
        floor_log + 1
    }
}

fn checked_log_base_power_of_2<T: PrimitiveUnsigned>(x: T, pow: u64) -> Option<u64> {
    assert!(x != T::ZERO, "Cannot take the base-2 logarithm of 0.");
    assert_ne!(pow, 0);
    let (floor_log, rem) = (x.significant_bits() - 1).div_mod(pow);
    if rem == 0 && T::is_power_of_2(&x) {
        Some(floor_log)
    } else {
        None
    }
}

macro_rules! impl_log_base_power_of_2_unsigned {
    ($t:ident) => {
        impl FloorLogBasePowerOf2<u64> for $t {
            type Output = u64;

            /// Returns the floor of the base-$2^k$ logarithm of a positive integer.
            ///
            /// $f(x, k) = \lfloor\log_{2^k} x\rfloor$.
            ///
            /// # Worst-case complexity
            /// Constant time and additional memory.
            ///
            /// # Panics
            /// Panics if `self` is infinite, `NaN`, or less than or equal to zero, or if `pow` is
            /// zero.
            ///
            /// # Examples
            /// See [here](super::log_base_power_of_2#floor_log_base_power_of_2).
            #[inline]
            fn floor_log_base_power_of_2(self, pow: u64) -> u64 {
                floor_log_base_power_of_2(self, pow)
            }
        }

        impl CeilingLogBasePowerOf2<u64> for $t {
            type Output = u64;

            /// Returns the ceiling of the base-$2^k$ logarithm of a positive integer.
            ///
            /// $f(x, k) = \lceil\log_{2^k} x\rceil$.
            ///
            /// # Worst-case complexity
            /// Constant time and additional memory.
            ///
            /// # Panics
            /// Panics if `self` is infinite, `NaN`, or less than or equal to zero, or if `pow` is
            /// zero.
            ///
            /// # Examples
            /// See [here](super::log_base_power_of_2#ceiling_log_base_power_of_2).
            #[inline]
            fn ceiling_log_base_power_of_2(self, pow: u64) -> u64 {
                ceiling_log_base_power_of_2(self, pow)
            }
        }

        impl CheckedLogBasePowerOf2<u64> for $t {
            type Output = u64;

            /// Returns the base-$2^k$ logarithm of a positive integer. If the integer is not a
            /// power of $2^k$, `None` is returned.
            ///
            /// $$
            /// f(x, k) = \\begin{cases}
            ///     \operatorname{Some}(\log_{2^k} x) & \text{if} \\quad \log_{2^k} x \in \Z, \\\\
            ///     \operatorname{None} & \textrm{otherwise}.
            /// \\end{cases}
            /// $$
            ///
            /// # Worst-case complexity
            /// Constant time and additional memory.
            ///
            /// # Panics
            /// Panics if `self` is infinite, `NaN`, or less than or equal to zero, or if `pow` is
            /// zero.
            ///
            /// # Examples
            /// See [here](super::log_base_power_of_2#ceiling_log_base_power_of_2).
            #[inline]
            fn checked_log_base_power_of_2(self, pow: u64) -> Option<u64> {
                checked_log_base_power_of_2(self, pow)
            }
        }
    };
}
apply_to_unsigneds!(impl_log_base_power_of_2_unsigned);

macro_rules! impl_log_base_power_of_2_primitive_float {
    ($t:ident) => {
        impl FloorLogBasePowerOf2<u64> for $t {
            type Output = i64;

            /// Returns the floor of the base-$2^k$ logarithm of a positive float.
            ///
            /// $f(x, k) = \lfloor\log_{2^k} x\rfloor$.
            ///
            /// # Worst-case complexity
            /// Constant time and additional memory.
            ///
            /// # Panics
            /// Panics if `self` or `pow` are 0.
            ///
            /// # Examples
            /// See [here](super::log_base_power_of_2#floor_log_base_power_of_2).
            #[inline]
            fn floor_log_base_power_of_2(self, pow: u64) -> i64 {
                assert!(self > 0.0);
                self.sci_exponent().div_round(i64::exact_from(pow), Floor).0
            }
        }

        impl CeilingLogBasePowerOf2<u64> for $t {
            type Output = i64;

            /// Returns the ceiling of the base-$2^k$ logarithm of a positive float.
            ///
            /// $f(x, k) = \lceil\log_{2^k} x\rceil$.
            ///
            /// # Worst-case complexity
            /// Constant time and additional memory.
            ///
            /// # Panics
            /// Panics if `self` or `pow` are 0.
            ///
            /// # Examples
            /// See [here](super::log_base_power_of_2#ceiling_log_base_power_of_2).
            #[inline]
            fn ceiling_log_base_power_of_2(self, pow: u64) -> i64 {
                assert!(self > 0.0);
                let (mantissa, exponent) = self.sci_mantissa_and_exponent();
                let exact = mantissa == 1.0;
                let (q, r) = exponent.div_mod(i64::exact_from(pow));
                if exact && r == 0 {
                    q
                } else {
                    q + 1
                }
            }
        }

        impl CheckedLogBasePowerOf2<u64> for $t {
            type Output = i64;

            /// Returns the base-$2^k$ logarithm of a positive float. If the float is not a power of
            /// $2^k$, `None` is returned.
            ///
            /// $$
            /// f(x, k) = \\begin{cases}
            ///     \operatorname{Some}(\log_{2^k} x) & \text{if} \\quad \log_{2^k} x \in \Z, \\\\
            ///     \operatorname{None} & \textrm{otherwise}.
            /// \\end{cases}
            /// $$
            ///
            /// # Worst-case complexity
            /// Constant time and additional memory.
            ///
            /// # Panics
            /// Panics if `self` or `pow` are 0.
            ///
            /// # Examples
            /// See [here](super::log_base_power_of_2#checked_log_base_power_of_2).
            #[inline]
            fn checked_log_base_power_of_2(self, pow: u64) -> Option<i64> {
                assert!(self > 0.0);
                let (mantissa, exponent) = self.sci_mantissa_and_exponent();
                if mantissa != 1.0 {
                    return None;
                }
                let (q, r) = exponent.div_mod(i64::exact_from(pow));
                if r == 0 {
                    Some(q)
                } else {
                    None
                }
            }
        }
    };
}
apply_to_primitive_floats!(impl_log_base_power_of_2_primitive_float);
