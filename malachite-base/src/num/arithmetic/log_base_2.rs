use num::arithmetic::traits::{CeilingLogBase2, CheckedLogBase2, FloorLogBase2, IsPowerOf2};
use num::basic::integers::PrimitiveInt;
use num::basic::traits::Zero;
use num::logic::traits::{LeadingZeros, SignificantBits, TrailingZeros};

fn _floor_log_base_2<T: Copy + Eq + SignificantBits + Zero>(x: T) -> u64 {
    if x == T::ZERO {
        panic!("Cannot take the base-2 logarithm of 0.");
    }
    x.significant_bits() - 1
}

fn _ceiling_log_base_2<T: Copy + Eq + IsPowerOf2 + SignificantBits + Zero>(x: T) -> u64 {
    let floor_log_base_2 = _floor_log_base_2(x);
    if x.is_power_of_2() {
        floor_log_base_2
    } else {
        floor_log_base_2 + 1
    }
}

fn _checked_log_base_2<T: PrimitiveInt>(x: T) -> Option<u64> {
    if x == T::ZERO {
        panic!("Cannot take the base-2 logarithm of 0.");
    }
    let leading_zeros = LeadingZeros::leading_zeros(x);
    let trailing_zeros = TrailingZeros::trailing_zeros(x);
    if leading_zeros + trailing_zeros == T::WIDTH - 1 {
        Some(trailing_zeros)
    } else {
        None
    }
}

macro_rules! impl_arithmetic_traits {
    ($t:ident) => {
        impl FloorLogBase2 for $t {
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
            /// See the documentation of the `num::arithmetic::log_base_2` module.
            #[inline]
            fn floor_log_base_2(self) -> u64 {
                _floor_log_base_2(self)
            }
        }

        impl CeilingLogBase2 for $t {
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
            /// See the documentation of the `num::arithmetic::log_base_2` module.
            #[inline]
            fn ceiling_log_base_2(self) -> u64 {
                _ceiling_log_base_2(self)
            }
        }

        impl CheckedLogBase2 for $t {
            /// Returns the base-2 logarithm of a positive integer. If the integer is not a power
            /// of 2, `None` is returned.
            ///
            /// $$
            /// f(x) = \\begin{cases}
            ///     \operatorname{Some}(\log_2 x) & \log_2 x \in \Z \\\\
            ///     \operatorname{None} & \textrm{otherwise},
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
            /// See the documentation of the `num::arithmetic::log_base_2` module.
            #[inline]
            fn checked_log_base_2(self) -> Option<u64> {
                _checked_log_base_2(self)
            }
        }
    };
}
apply_to_unsigneds!(impl_arithmetic_traits);
