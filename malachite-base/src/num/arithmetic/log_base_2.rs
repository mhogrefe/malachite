// Copyright © 2025 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::num::arithmetic::traits::{CeilingLogBase2, CheckedLogBase2, FloorLogBase2};
use crate::num::basic::integers::PrimitiveInt;
use crate::num::basic::unsigneds::PrimitiveUnsigned;
use crate::num::conversion::traits::SciMantissaAndExponent;
use crate::num::logic::traits::{LeadingZeros, TrailingZeros};

fn floor_log_base_2<T: PrimitiveUnsigned>(x: T) -> u64 {
    assert!(x != T::ZERO, "Cannot take the base-2 logarithm of 0.");
    x.significant_bits() - 1
}

fn ceiling_log_base_2<T: PrimitiveUnsigned>(x: T) -> u64 {
    let floor_log_base_2 = x.floor_log_base_2();
    if x.is_power_of_2() {
        floor_log_base_2
    } else {
        floor_log_base_2 + 1
    }
}

fn checked_log_base_2<T: PrimitiveInt>(x: T) -> Option<u64> {
    assert!(x != T::ZERO, "Cannot take the base-2 logarithm of 0.");
    let leading_zeros = LeadingZeros::leading_zeros(x);
    let trailing_zeros = TrailingZeros::trailing_zeros(x);
    if leading_zeros + trailing_zeros == T::WIDTH - 1 {
        Some(trailing_zeros)
    } else {
        None
    }
}

macro_rules! impl_log_base_2_unsigned {
    ($t:ident) => {
        impl FloorLogBase2 for $t {
            type Output = u64;

            /// Returns the floor of the base-2 logarithm of a positive integer.
            ///
            /// $f(x) = \lfloor\log_2 x\rfloor$.
            ///
            /// # Worst-case complexity
            /// Constant time and additional memory.
            ///
            /// # Panics
            /// Panics if `self` is 0.
            ///
            /// # Examples
            /// See [here](super::log_base_2#floor_log_base_2).
            #[inline]
            fn floor_log_base_2(self) -> u64 {
                // TODO use ilog2 once stabilized
                floor_log_base_2(self)
            }
        }

        impl CeilingLogBase2 for $t {
            type Output = u64;

            /// Returns the ceiling of the base-2 logarithm of a positive integer.
            ///
            /// $f(x) = \lceil\log_2 x\rceil$.
            ///
            /// # Worst-case complexity
            /// Constant time and additional memory.
            ///
            /// # Panics
            /// Panics if `self` is 0.
            ///
            /// # Examples
            /// See [here](super::log_base_2#ceiling_log_base_2).
            #[inline]
            fn ceiling_log_base_2(self) -> u64 {
                ceiling_log_base_2(self)
            }
        }

        impl CheckedLogBase2 for $t {
            type Output = u64;

            /// Returns the base-2 logarithm of a positive integer. If the integer is not a power of
            /// 2, `None` is returned.
            ///
            /// $$
            /// f(x) = \\begin{cases}
            ///     \operatorname{Some}(\log_2 x) & \text{if} \\quad \log_2 x \in \Z, \\\\
            ///     \operatorname{None} & \textrm{otherwise}.
            /// \\end{cases}
            /// $$
            ///
            /// # Worst-case complexity
            /// Constant time and additional memory.
            ///
            /// # Panics
            /// Panics if `self` is 0.
            ///
            /// # Examples
            /// See [here](super::log_base_2#checked_log_base_2).
            #[inline]
            fn checked_log_base_2(self) -> Option<u64> {
                checked_log_base_2(self)
            }
        }
    };
}
apply_to_unsigneds!(impl_log_base_2_unsigned);

macro_rules! impl_log_base_2_primitive_float {
    ($t:ident) => {
        impl FloorLogBase2 for $t {
            type Output = i64;

            /// Returns the floor of the base-2 logarithm of a positive float.
            ///
            /// $f(x) = \lfloor\log_2 x\rfloor$.
            ///
            /// # Worst-case complexity
            /// Constant time and additional memory.
            ///
            /// # Panics
            /// Panics if `self` is infinite, `NaN`, or less than or equal to zero.
            ///
            /// # Examples
            /// See [here](super::log_base_2#floor_log_base_2).
            #[inline]
            fn floor_log_base_2(self) -> i64 {
                assert!(self > 0.0);
                self.sci_exponent()
            }
        }

        impl CeilingLogBase2 for $t {
            type Output = i64;

            /// Returns the ceiling of the base-2 logarithm of a positive float.
            ///
            /// $f(x) = \lceil\log_2 x\rceil$.
            ///
            /// # Worst-case complexity
            /// Constant time and additional memory.
            ///
            /// # Panics
            /// Panics if `self` is infinite, `NaN`, or less than or equal to zero.
            ///
            /// # Examples
            /// See [here](super::log_base_2#ceiling_log_base_2).
            #[inline]
            fn ceiling_log_base_2(self) -> i64 {
                assert!(self > 0.0);
                let (mantissa, exponent) = self.sci_mantissa_and_exponent();
                if mantissa == 1.0 {
                    exponent
                } else {
                    exponent + 1
                }
            }
        }

        impl CheckedLogBase2 for $t {
            type Output = i64;

            /// Returns the base-2 logarithm of a positive float If the float is not a power of 2,
            /// `None` is returned.
            ///
            /// $$
            /// f(x) = \\begin{cases}
            ///     \operatorname{Some}(\log_2 x) & \text{if} \\quad \log_2 x \in \Z, \\\\
            ///     \operatorname{None} & \textrm{otherwise}.
            /// \\end{cases}
            /// $$
            ///
            /// # Worst-case complexity
            /// Constant time and additional memory.
            ///
            /// # Panics
            /// Panics if `self` is infinite, `NaN`, or less than or equal to zero.
            ///
            /// # Examples
            /// See [here](super::log_base_2#checked_log_base_2).
            #[inline]
            fn checked_log_base_2(self) -> Option<i64> {
                assert!(self > 0.0);
                let (mantissa, exponent) = self.sci_mantissa_and_exponent();
                if mantissa == 1.0 {
                    Some(exponent)
                } else {
                    None
                }
            }
        }
    };
}
apply_to_primitive_floats!(impl_log_base_2_primitive_float);
