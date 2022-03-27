use num::arithmetic::traits::{
    CeilingMod, CeilingModAssign, Mod, ModAssign, NegMod, NegModAssign, UnsignedAbs,
};
use num::basic::signeds::PrimitiveSigned;
use num::basic::unsigneds::PrimitiveUnsigned;
use num::conversion::traits::ExactFrom;

fn neg_mod_unsigned<T: PrimitiveUnsigned>(x: T, other: T) -> T {
    let remainder = x % other;
    if remainder == T::ZERO {
        T::ZERO
    } else {
        other - remainder
    }
}

fn neg_mod_assign_unsigned<T: PrimitiveUnsigned>(x: &mut T, other: T) {
    *x %= other;
    if *x != T::ZERO {
        *x = other - *x;
    }
}

macro_rules! impl_mod_unsigned {
    ($t:ident) => {
        impl Mod<$t> for $t {
            type Output = $t;

            /// Divides a value by another value, returning just the remainder.
            ///
            /// If the quotient were computed, the quotient and remainder would satisfy
            /// $x = qy + r$ and $0 \leq r < y$.
            ///
            /// $$
            /// f(x, y) = x - y\left \lfloor \frac{x}{y} \right \rfloor.
            /// $$
            ///
            /// # Worst-case complexity
            /// Constant time and additional memory.
            ///
            /// # Panics
            /// Panics if `other` is 0.
            ///
            /// # Examples
            /// See the documentation of the `num::arithmetic::mod_op` module.
            #[inline]
            fn mod_op(self, other: $t) -> $t {
                self % other
            }
        }

        impl ModAssign<$t> for $t {
            /// Divides a value by another value, replacing the first value by the remainder.
            ///
            /// If the quotient were computed, he quotient and remainder would satisfy $x = qy + r$
            /// and $0 \leq r < y$.
            ///
            /// $$
            /// x \gets x - y\left \lfloor \frac{x}{y} \right \rfloor.
            /// $$
            ///
            /// # Worst-case complexity
            /// Constant time and additional memory.
            ///
            /// # Panics
            /// Panics if `other` is 0.
            ///
            /// # Examples
            /// See the documentation of the `num::arithmetic::mod_op` module.
            #[inline]
            fn mod_assign(&mut self, other: $t) {
                *self %= other;
            }
        }

        impl NegMod<$t> for $t {
            type Output = $t;

            /// Divides the negative of a value by another value, returning just the remainder.
            ///
            /// If the quotient were computed, the quotient and remainder would satisfy
            /// $x = qy - r$ and $0 \leq r < y$.
            ///
            /// $$
            /// f(x, y) = y\left \lceil \frac{x}{y} \right \rceil - x.
            /// $$
            ///
            /// # Worst-case complexity
            /// Constant time and additional memory.
            ///
            /// # Panics
            /// Panics if `other` is 0.
            ///
            /// # Examples
            /// See the documentation of the `num::arithmetic::mod_op` module.
            #[inline]
            fn neg_mod(self, other: $t) -> $t {
                neg_mod_unsigned(self, other)
            }
        }

        impl NegModAssign<$t> for $t {
            /// Divides the negative of a value by another value, returning just the remainder.
            ///
            /// If the quotient were computed, the quotient and remainder would satisfy
            /// $x = qy - r$ and $0 \leq r < y$.
            ///
            /// $$
            /// x \gets y\left \lceil \frac{x}{y} \right \rceil - x.
            /// $$
            ///
            /// # Worst-case complexity
            /// Constant time and additional memory.
            ///
            /// # Panics
            /// Panics if `other` is 0.
            ///
            /// # Examples
            /// See the documentation of the `num::arithmetic::mod_op` module.
            #[inline]
            fn neg_mod_assign(&mut self, other: $t) {
                neg_mod_assign_unsigned(self, other);
            }
        }
    };
}
apply_to_unsigneds!(impl_mod_unsigned);

fn mod_op_signed<
    U: PrimitiveUnsigned,
    S: ExactFrom<U> + PrimitiveSigned + UnsignedAbs<Output = U>,
>(
    x: S,
    other: S,
) -> S {
    let remainder = if (x >= S::ZERO) == (other >= S::ZERO) {
        x.unsigned_abs() % other.unsigned_abs()
    } else {
        x.unsigned_abs().neg_mod(other.unsigned_abs())
    };
    if other >= S::ZERO {
        S::exact_from(remainder)
    } else {
        -S::exact_from(remainder)
    }
}

fn ceiling_mod_signed<
    U: PrimitiveUnsigned,
    S: ExactFrom<U> + PrimitiveSigned + UnsignedAbs<Output = U>,
>(
    x: S,
    other: S,
) -> S {
    let remainder = if (x >= S::ZERO) == (other >= S::ZERO) {
        x.unsigned_abs().neg_mod(other.unsigned_abs())
    } else {
        x.unsigned_abs() % other.unsigned_abs()
    };
    if other >= S::ZERO {
        -S::exact_from(remainder)
    } else {
        S::exact_from(remainder)
    }
}

macro_rules! impl_mod_signed {
    ($t:ident) => {
        impl Mod<$t> for $t {
            type Output = $t;

            /// Divides a value by another value, returning just the remainder. The remainder has
            /// the same sign as the second value.
            ///
            /// If the quotient were computed, the quotient and remainder would satisfy
            /// $x = qy + r$ and $0 \leq |r| < y$.
            ///
            /// $$
            /// f(x, y) = x - y\left \lfloor \frac{x}{y} \right \rfloor.
            /// $$
            ///
            /// # Worst-case complexity
            /// Constant time and additional memory.
            ///
            /// # Panics
            /// Panics if `other` is 0.
            ///
            /// # Examples
            /// See the documentation of the `num::arithmetic::mod_op` module.
            #[inline]
            fn mod_op(self, other: $t) -> $t {
                mod_op_signed(self, other)
            }
        }

        impl ModAssign<$t> for $t {
            /// Divides a value by another value, replacing the first value by the remainder. The
            /// remainder has the same sign as the second value.
            ///
            /// If the quotient were computed, he quotient and remainder would satisfy $x = qy + r$
            /// and $0 \leq |r| < y$.
            ///
            /// $$
            /// x \gets x - y\left \lfloor \frac{x}{y} \right \rfloor.
            /// $$
            ///
            /// # Worst-case complexity
            /// Constant time and additional memory.
            ///
            /// # Panics
            /// Panics if `other` is 0.
            ///
            /// # Examples
            /// See the documentation of the `num::arithmetic::mod_op` module.
            #[inline]
            fn mod_assign(&mut self, other: $t) {
                *self = self.mod_op(other);
            }
        }

        impl CeilingMod<$t> for $t {
            type Output = $t;

            /// Divides a value by another value, returning just the remainder. The remainder has
            /// the opposite sign of the second value.
            ///
            /// If the quotient were computed, the quotient and remainder would satisfy
            /// $x = qy + r$ and $0 \leq |r| < |y|$.
            ///
            /// $$
            /// f(x, y) =  x - y\left \lceil \frac{x}{y} \right \rceil.
            /// $$
            ///
            /// # Worst-case complexity
            /// Constant time and additional memory.
            ///
            /// # Panics
            /// Panics if `other` is 0.
            ///
            /// # Examples
            /// See the documentation of the `num::arithmetic::mod_op` module.
            #[inline]
            fn ceiling_mod(self, other: $t) -> $t {
                ceiling_mod_signed(self, other)
            }
        }

        impl CeilingModAssign<$t> for $t {
            /// Divides a value by another value, replacing the first value by the remainder. The
            /// remainder has the opposite sign of the second value.
            ///
            /// If the quotient were computed, the quotient and remainder would satisfy
            /// $x = qy + r$ and $0 \leq |r| < |y|$.
            ///
            /// $$
            /// x \gets x - y\left \lceil\frac{x}{y} \right \rceil.
            /// $$
            ///
            /// # Worst-case complexity
            /// Constant time and additional memory.
            ///
            /// # Panics
            /// Panics if `other` is 0.
            ///
            /// # Examples
            /// See the documentation of the `num::arithmetic::mod_op` module.
            #[inline]
            fn ceiling_mod_assign(&mut self, other: $t) {
                *self = self.ceiling_mod(other);
            }
        }
    };
}
apply_to_signeds!(impl_mod_signed);
