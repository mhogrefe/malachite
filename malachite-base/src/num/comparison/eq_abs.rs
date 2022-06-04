use num::arithmetic::traits::UnsignedAbs;
use num::comparison::traits::EqAbs;

macro_rules! impl_eq_abs_unsigned {
    ($t:ident) => {
        impl EqAbs<$t> for $t {
            /// Compares the absolute values of two numbers for equality, taking both by reference.
            ///
            /// For unsigned values, this is the same as ordinary equality.
            ///
            /// # Worst-case complexity
            /// Constant time and additional memory.
            ///
            /// # Examples
            /// See [here](super::eq_abs#eq_abs).
            #[inline]
            fn eq_abs(&self, other: &Self) -> bool {
                self == other
            }
        }
    };
}
apply_to_unsigneds!(impl_eq_abs_unsigned);

fn eq_abs_signed<U: Eq, S: Copy + UnsignedAbs<Output = U>>(x: &S, y: &S) -> bool {
    x.unsigned_abs() == y.unsigned_abs()
}

macro_rules! impl_eq_abs_signed {
    ($t:ident) => {
        impl EqAbs<$t> for $t {
            /// Compares the absolute values of two numbers for equality, taking both by reference.
            ///
            /// # Worst-case complexity
            /// Constant time and additional memory.
            ///
            /// # Examples
            /// See [here](super::eq_abs#eq_abs).
            #[inline]
            fn eq_abs(&self, other: &Self) -> bool {
                eq_abs_signed(self, other)
            }
        }
    };
}
apply_to_signeds!(impl_eq_abs_signed);
