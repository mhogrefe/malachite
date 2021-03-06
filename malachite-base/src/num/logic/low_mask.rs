use num::basic::integers::PrimitiveInt;
use num::basic::traits::NegativeOne;
use num::logic::traits::LowMask;

fn _low_mask_unsigned<T: PrimitiveInt>(bits: u64) -> T {
    assert!(bits <= T::WIDTH);
    if bits == T::WIDTH {
        T::MAX
    } else {
        T::power_of_two(bits) - T::ONE
    }
}

macro_rules! impl_low_mask_unsigned {
    ($t:ident) => {
        impl LowMask for $t {
            /// Returns a value with the least significant `bits` bits on and the remaining bits
            /// off.
            ///
            /// $f(b) = 2^b - 1$.
            ///
            /// # Worst-case complexity
            /// Constant time and additional memory.
            ///
            /// # Panics
            /// Panics if `pow` is greater than the width of `$t`.
            ///
            /// # Examples
            /// See the documentation of the `num::logic::low_mask` module.
            #[inline]
            fn low_mask(bits: u64) -> $t {
                _low_mask_unsigned(bits)
            }
        }
    };
}
apply_to_unsigneds!(impl_low_mask_unsigned);

fn _low_mask_signed<T: NegativeOne + PrimitiveInt>(bits: u64) -> T {
    assert!(bits <= T::WIDTH);
    if bits == T::WIDTH {
        T::NEGATIVE_ONE
    } else if bits == T::WIDTH - 1 {
        T::MAX
    } else {
        T::power_of_two(bits) - T::ONE
    }
}

macro_rules! impl_low_mask_signed {
    ($t:ident) => {
        impl LowMask for $t {
            /// Returns a value with the least significant `bits` bits on and the remaining bits
            /// off.
            ///
            /// $$
            /// f(b) = \\begin{cases}
            ///     2^b - 1 & 0 \leq n < W \\\\
            ///     -1 & n = W,
            /// \\end{cases}
            /// $$
            /// where $W$ is `$t::WIDTH`.
            ///
            /// # Worst-case complexity
            /// Constant time and additional memory.
            ///
            /// # Panics
            /// Panics if `pow` is greater than the width of `$t`.
            ///
            /// # Examples
            /// See the documentation of the `num::logic::low_mask` module.
            #[inline]
            fn low_mask(bits: u64) -> $t {
                _low_mask_signed(bits)
            }
        }
    };
}
apply_to_signeds!(impl_low_mask_signed);
