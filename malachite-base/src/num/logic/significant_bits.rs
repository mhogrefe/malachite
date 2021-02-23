use num::arithmetic::traits::UnsignedAbs;
use num::basic::integers::PrimitiveInt;
use num::logic::traits::{LeadingZeros, SignificantBits};

fn _significant_bits_unsigned<T: PrimitiveInt>(x: T) -> u64 {
    T::WIDTH - LeadingZeros::leading_zeros(x)
}

macro_rules! impl_significant_bits_unsigned {
    ($t:ident) => {
        impl SignificantBits for $t {
            /// Returns the number of significant bits of a primitive unsigned integer.
            ///
            /// This is the integer's width minus the number of leading zeros.
            ///
            /// $$
            /// f(n) = \\begin{cases}
            ///     0 & n = 0 \\\\
            ///     \lfloor \log_2 n \rfloor + 1 & n > 0
            /// \\end{cases}
            /// $$
            ///
            /// # Worst-case complexity
            ///
            /// Constant time and additional memory.
            ///
            /// # Examples
            /// See the documentation of the `num::logic::significant_bits` module.
            #[inline]
            fn significant_bits(self) -> u64 {
                _significant_bits_unsigned(self)
            }
        }
    };
}
apply_to_unsigneds!(impl_significant_bits_unsigned);

fn _significant_bits_signed<U: SignificantBits, S: UnsignedAbs<Output = U>>(x: S) -> u64 {
    x.unsigned_abs().significant_bits()
}

macro_rules! impl_significant_bits_signed {
    ($u:ident, $s:ident) => {
        /// Returns the number of significant bits of a primitive signed integer.
        ///
        /// This is the integer's width minus the number of leading zeros of its absolute value.
        ///
        /// $$
        /// f(n) = \\begin{cases}
        ///     0 & n = 0 \\\\
        ///     \lfloor \log_2 |n| \rfloor + 1 & n \neq 0
        /// \\end{cases}
        /// $$
        ///
        /// # Worst-case complexity
        ///
        /// Constant time and additional memory.
        ///
        /// # Examples
        /// See the documentation of the `num::logic::significant_bits` module.
        impl SignificantBits for $s {
            #[inline]
            fn significant_bits(self) -> u64 {
                _significant_bits_signed(self)
            }
        }
    };
}
apply_to_unsigned_signed_pair!(impl_significant_bits_signed);
