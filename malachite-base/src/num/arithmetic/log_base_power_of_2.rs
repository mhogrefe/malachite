use num::arithmetic::traits::{
    CeilingLogBasePowerOf2, CheckedLogBasePowerOf2, DivMod, DivRound, FloorLogBasePowerOf2,
};
#[cfg(feature = "test_build")]
use num::basic::traits::Iverson;
use num::basic::unsigneds::PrimitiveUnsigned;
use num::conversion::traits::{ExactFrom, SciMantissaAndExponent};
use rounding_modes::RoundingMode;

#[cfg(feature = "test_build")]
pub fn ceiling_log_base_power_of_2_naive<T: PrimitiveUnsigned>(x: T, pow: u64) -> u64 {
    assert_ne!(x, T::ZERO);
    assert_ne!(pow, 0);
    if pow >= T::WIDTH {
        return u64::iverson(x != T::ONE);
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
    if x == T::ZERO {
        panic!("Cannot take the base-2 logarithm of 0.");
    }
    assert_ne!(pow, 0);
    (x.significant_bits() - 1) / pow
}

fn ceiling_log_base_power_of_2<T: PrimitiveUnsigned>(x: T, pow: u64) -> u64 {
    if x == T::ZERO {
        panic!("Cannot take the base-2 logarithm of 0.");
    }
    assert_ne!(pow, 0);
    let (floor_log, rem) = (x.significant_bits() - 1).div_mod(pow);
    if rem == 0 && T::is_power_of_2(&x) {
        floor_log
    } else {
        floor_log + 1
    }
}

fn checked_log_base_power_of_2<T: PrimitiveUnsigned>(x: T, pow: u64) -> Option<u64> {
    if x == T::ZERO {
        panic!("Cannot take the base-2 logarithm of 0.");
    }
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

            /// Returns the floor of the base-$b$ logarithm of a positive float, where $b$ is a
            /// power of 2.
            ///
            /// $f(x, p) = \lfloor\log_{2^p} x\rfloor$.
            ///
            /// # Worst-case complexity
            /// Constant time and additional memory.
            ///
            /// # Panics
            /// Panics if `self` is infinite, `NaN`, or less than or equal to zero, or if `pow` is
            /// zero.
            ///
            /// # Examples
            /// See the documentation of the `num::arithmetic::log_base_power_of_2` module.
            #[inline]
            fn floor_log_base_power_of_2(self, pow: u64) -> u64 {
                floor_log_base_power_of_2(self, pow)
            }
        }

        impl CeilingLogBasePowerOf2<u64> for $t {
            type Output = u64;

            /// Returns the ceiling of the base-$b$ logarithm of a positive float, where $b$ is a
            /// power of 2.
            ///
            /// $f(x, p) = \lceil\log_{2^p} x\rceil$.
            ///
            /// # Worst-case complexity
            /// Constant time and additional memory.
            ///
            /// # Panics
            /// Panics if `self` is infinite, `NaN`, or less than or equal to zero, or if `pow` is
            /// zero.
            ///
            /// # Examples
            /// See the documentation of the `num::arithmetic::log_base_power_of_2` module.
            #[inline]
            fn ceiling_log_base_power_of_2(self, pow: u64) -> u64 {
                ceiling_log_base_power_of_2(self, pow)
            }
        }

        impl CheckedLogBasePowerOf2<u64> for $t {
            type Output = u64;

            /// Returns the base-$b$ logarithm of a positive float, where $b$ is a power of 2. If
            /// the float is not a power of $b$, `None` is returned.
            ///
            /// $$
            /// f(x, p) = \\begin{cases}
            ///     \operatorname{Some}(\log_{2^p} x) & \log_{2^p} x \in \Z \\\\
            ///     \operatorname{None} & \textrm{otherwise},
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
            /// See the documentation of the `num::arithmetic::log_base_power_of_2` module.
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

            /// Returns the floor of the base-$b$ logarithm of a positive integer, where $b$ is a
            /// power of 2.
            ///
            /// $f(x, p) = \lfloor\log_{2^p} x\rfloor$.
            ///
            /// # Worst-case complexity
            /// Constant time and additional memory.
            ///
            /// # Panics
            /// Panics if `self` or `pow` are 0.
            ///
            /// # Examples
            /// See the documentation of the `num::arithmetic::log_base_power_of_2` module.
            #[inline]
            fn floor_log_base_power_of_2(self, pow: u64) -> i64 {
                assert!(self > 0.0);
                self.sci_exponent()
                    .div_round(i64::exact_from(pow), RoundingMode::Floor)
            }
        }

        impl CeilingLogBasePowerOf2<u64> for $t {
            type Output = i64;

            /// Returns the ceiling of the base-$b$ logarithm of a positive integer, where $b$ is a
            /// power of 2.
            ///
            /// $f(x, p) = \lceil\log_{2^p} x\rceil$.
            ///
            /// # Worst-case complexity
            /// Constant time and additional memory.
            ///
            /// # Panics
            /// Panics if `self` or `pow` are 0.
            ///
            /// # Examples
            /// See the documentation of the `num::arithmetic::log_base_power_of_2` module.
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

            /// Returns the base-$b$ logarithm of a positive integer, where $b$ is a power of 2. If
            /// the integer is not a power of $b$, `None` is returned.
            ///
            /// $$
            /// f(x, p) = \\begin{cases}
            ///     \operatorname{Some}(\log_{2^p} x) & \log_{2^p} x \in \Z \\\\
            ///     \operatorname{None} & \textrm{otherwise},
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
            /// See the documentation of the `num::arithmetic::log_base_power_of_2` module.
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
