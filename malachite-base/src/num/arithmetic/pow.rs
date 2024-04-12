// Copyright © 2024 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::num::arithmetic::traits::{Parity, Pow, PowAssign};
use crate::num::conversion::traits::ExactFrom;

macro_rules! impl_pow_unsigned {
    ($t:ident) => {
        impl Pow<u64> for $t {
            type Output = $t;

            /// This is a wrapper over the `pow` functions in the standard library, for example
            /// [this one](u32::pow).
            #[inline]
            fn pow(self, exp: u64) -> $t {
                if exp == 0 {
                    1
                } else if self < 2 {
                    self
                } else {
                    self.pow(u32::exact_from(exp))
                }
            }
        }
    };
}
apply_to_unsigneds!(impl_pow_unsigned);

macro_rules! impl_pow_signed {
    ($t:ident) => {
        impl Pow<u64> for $t {
            type Output = $t;

            /// This is a wrapper over the `pow` functions in the standard library, for example
            /// [this one](i32::pow).
            #[inline]
            fn pow(self, exp: u64) -> $t {
                if exp == 0 {
                    1
                } else if self == 0 || self == 1 {
                    self
                } else if self == -1 {
                    if exp.even() {
                        1
                    } else {
                        -1
                    }
                } else {
                    self.pow(u32::exact_from(exp))
                }
            }
        }
    };
}
apply_to_signeds!(impl_pow_signed);

macro_rules! impl_pow_primitive_int {
    ($t:ident) => {
        impl PowAssign<u64> for $t {
            /// Raises a number to a power, in place.
            ///
            /// # Worst-case complexity
            /// $T(n) = O(n)$
            ///
            /// $M(n) = O(1)$
            ///
            /// where $T$ is time, $M$ is additional memory, and $n$ is `exp.significant_bits()`.
            ///
            /// # Examples
            /// See [here](super::pow#pow_assign).
            #[inline]
            fn pow_assign(&mut self, exp: u64) {
                *self = Pow::pow(*self, exp);
            }
        }
    };
}
apply_to_primitive_ints!(impl_pow_primitive_int);

macro_rules! impl_pow_primitive_float {
    ($t:ident) => {
        impl Pow<i64> for $t {
            type Output = $t;

            /// This is a wrapper over the `powi` functions in the standard library, for example
            /// [this one](f32::powi).
            #[inline]
            fn pow(self, exp: i64) -> $t {
                libm::Libm::<$t>::pow(self, exp as $t)
            }
        }

        impl PowAssign<i64> for $t {
            /// Raises a number to a power, in place.
            ///
            /// # Worst-case complexity
            /// Constant time and additional memory.
            ///
            /// # Examples
            /// See [here](super::pow#pow_assign).
            #[inline]
            fn pow_assign(&mut self, exp: i64) {
                *self = libm::Libm::<$t>::pow(*self, exp as $t);
            }
        }

        impl Pow<$t> for $t {
            type Output = $t;

            /// This is a wrapper over the `powf` functions in the standard library, for example
            /// [this one](f32::powf).
            #[inline]
            fn pow(self, exp: $t) -> $t {
                libm::Libm::<$t>::pow(self, exp)
            }
        }

        impl PowAssign<$t> for $t {
            /// Raises a number to a power, in place.
            ///
            /// # Worst-case complexity
            /// Constant time and additional memory.
            ///
            /// # Examples
            /// See [here](super::pow#pow_assign).
            #[inline]
            fn pow_assign(&mut self, exp: $t) {
                *self = libm::Libm::<$t>::pow(*self, exp);
            }
        }
    };
}
apply_to_primitive_floats!(impl_pow_primitive_float);
