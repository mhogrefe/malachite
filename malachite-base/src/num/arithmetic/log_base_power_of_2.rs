use num::arithmetic::traits::{
    CeilingLogBasePowerOf2, CheckedLogBasePowerOf2, DivMod, FloorLogBasePowerOf2, IsPowerOf2,
};
use num::basic::integers::PrimitiveInt;
use num::basic::traits::{Iverson, Zero};
use num::logic::traits::SignificantBits;

#[doc(hidden)]
pub fn _ceiling_log_base_power_of_2_naive<T: PrimitiveInt>(x: T, pow: u64) -> u64 {
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

fn _floor_log_base_power_of_2<T: Copy + Eq + SignificantBits + Zero>(x: T, pow: u64) -> u64 {
    if x == T::ZERO {
        panic!("Cannot take the base-2 logarithm of 0.");
    }
    assert_ne!(pow, 0);
    (x.significant_bits() - 1) / pow
}

fn _ceiling_log_base_power_of_2<T: Copy + Eq + IsPowerOf2 + SignificantBits + Zero>(
    x: T,
    pow: u64,
) -> u64 {
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

fn _checked_log_base_power_of_2<T: Copy + Eq + IsPowerOf2 + SignificantBits + Zero>(
    x: T,
    pow: u64,
) -> Option<u64> {
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

macro_rules! impl_arithmetic_traits {
    ($t:ident) => {
        impl FloorLogBasePowerOf2 for $t {
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
            fn floor_log_base_power_of_2(self, pow: u64) -> u64 {
                _floor_log_base_power_of_2(self, pow)
            }
        }

        impl CeilingLogBasePowerOf2 for $t {
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
            fn ceiling_log_base_power_of_2(self, pow: u64) -> u64 {
                _ceiling_log_base_power_of_2(self, pow)
            }
        }

        impl CheckedLogBasePowerOf2 for $t {
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
            fn checked_log_base_power_of_2(self, pow: u64) -> Option<u64> {
                _checked_log_base_power_of_2(self, pow)
            }
        }
    };
}
apply_to_unsigneds!(impl_arithmetic_traits);
