use integer::Integer;
use malachite_base::num::arithmetic::traits::UnsignedAbs;
use malachite_base::num::basic::traits::Zero;
use natural::Natural;

macro_rules! impl_unsigned {
    ($t: ident) => {
        impl PartialEq<$t> for Integer {
            /// Determines whether an `Integer` is equal to a value of unsigned primitive integer
            /// type.
            ///
            /// # Worst-case complexity
            /// Constant time and additional memory.
            ///
            /// # Examples
            /// See the documentation of the `integer::comparison::partial_eq_primitive_int`
            /// module.
            #[inline]
            fn eq(&self, other: &$t) -> bool {
                self.sign && self.abs == *other
            }
        }

        impl PartialEq<Integer> for $t {
            /// Determines whether a value of unsigned primitive integer type is equal to an
            /// `Integer`.
            ///
            /// # Worst-case complexity
            /// Constant time and additional memory.
            ///
            /// # Examples
            /// See the documentation of the `integer::comparison::partial_eq_primitive_int`
            /// module.
            #[inline]
            fn eq(&self, other: &Integer) -> bool {
                other == self
            }
        }
    };
}
apply_to_unsigneds!(impl_unsigned);

fn eq_signed<U, S: Copy + Ord + UnsignedAbs<Output = U> + Zero>(x: &Integer, other: &S) -> bool
where
    Natural: PartialEq<U>,
{
    x.sign == (*other >= S::ZERO) && x.abs == other.unsigned_abs()
}

macro_rules! impl_signed {
    ($t: ident) => {
        impl PartialEq<$t> for Integer {
            /// Determines whether an `Integer` is equal to a value of signed primitive integer
            /// type.
            ///
            /// # Worst-case complexity
            /// Constant time and additional memory.
            ///
            /// # Examples
            /// See the documentation of the `integer::comparison::partial_eq_primitive_int`
            /// module.
            fn eq(&self, other: &$t) -> bool {
                eq_signed(self, other)
            }
        }

        impl PartialEq<Integer> for $t {
            /// Determines whether a value of signed primitive integer type is equal to an
            /// `Integer`.
            ///
            /// # Worst-case complexity
            /// Constant time and additional memory.
            ///
            /// # Examples
            /// See the documentation of the `integer::comparison::partial_eq_primitive_int`
            /// module.
            #[inline]
            fn eq(&self, other: &Integer) -> bool {
                other == self
            }
        }
    };
}
apply_to_signeds!(impl_signed);
