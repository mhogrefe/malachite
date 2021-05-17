use num::arithmetic::traits::{CeilingLogBase2, CheckedLogBase2, FloorLogBase2};
use num::basic::integers::PrimitiveInt;
use num::logic::traits::{LeadingZeros, SignificantBits, TrailingZeros};

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
                if self == 0 {
                    panic!("Cannot take the base-2 logarithm of 0.");
                }
                self.significant_bits() - 1
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
                let floor_log_base_2 = self.floor_log_base_2();
                if self.is_power_of_two() {
                    floor_log_base_2
                } else {
                    floor_log_base_2 + 1
                }
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
                if self == 0 {
                    panic!("Cannot take the base-2 logarithm of 0.");
                }
                let leading_zeros = LeadingZeros::leading_zeros(self);
                let trailing_zeros = TrailingZeros::trailing_zeros(self);
                if leading_zeros + trailing_zeros == $t::WIDTH - 1 {
                    Some(trailing_zeros)
                } else {
                    None
                }
            }
        }
    };
}
apply_to_unsigneds!(impl_arithmetic_traits);
