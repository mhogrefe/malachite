use comparison::traits::{Max, Min};
use num::arithmetic::traits::{
    CheckedMul, SaturatingAdd, SaturatingAddAssign, SaturatingAddMul, SaturatingAddMulAssign,
    SaturatingMul, UnsignedAbs, WrappingSub,
};
use num::basic::traits::Zero;
use num::conversion::traits::WrappingFrom;

fn saturating_add_mul_unsigned<T: SaturatingAdd<T, Output = T> + SaturatingMul<T, Output = T>>(
    x: T,
    y: T,
    z: T,
) -> T {
    x.saturating_add(y.saturating_mul(z))
}

fn saturating_add_mul_assign_unsigned<T: SaturatingAddAssign<T> + SaturatingMul<T, Output = T>>(
    x: &mut T,
    y: T,
    z: T,
) {
    x.saturating_add_assign(y.saturating_mul(z));
}

macro_rules! impl_saturating_add_mul_unsigned {
    ($t:ident) => {
        impl SaturatingAddMul<$t> for $t {
            type Output = $t;

            /// Computes $x + yz$, saturating at the numeric bounds instead of overflowing.
            ///
            /// $$
            /// f(x, y, z) = \\begin{cases}
            ///     x + yz & m \leq x + yz \leq M \\\\
            ///     M & x + yz > M \\\\
            ///     m & x + yz < m,
            /// \\end{cases}
            /// $$
            /// where $m$ is `$t::MIN` and $M$ is `$t::MAX`.
            ///
            /// # Worst-case complexity
            /// Constant time and additional memory.
            ///
            /// # Examples
            /// See the documentation of the `num::arithmetic::saturating_add_mul` module.
            #[inline]
            fn saturating_add_mul(self, y: $t, z: $t) -> $t {
                saturating_add_mul_unsigned(self, y, z)
            }
        }

        impl SaturatingAddMulAssign<$t> for $t {
            /// Replaces $x$ with $x + yz$, saturating at the numeric bounds instead of
            /// overflowing.
            ///
            /// $$
            /// x \gets \\begin{cases}
            ///     x + yz & m \leq x + yz \leq M \\\\
            ///     M & x + yz > M \\\\
            ///     m & x + yz < m,
            /// \\end{cases}
            /// $$
            /// where $m$ is `$t::MIN` and $M$ is `$t::MAX`.
            ///
            /// # Worst-case complexity
            /// Constant time and additional memory.
            ///
            /// # Examples
            /// See the documentation of the `num::arithmetic::saturating_add_mul` module.
            #[inline]
            fn saturating_add_mul_assign(&mut self, y: $t, z: $t) {
                saturating_add_mul_assign_unsigned(self, y, z);
            }
        }
    };
}
apply_to_unsigneds!(impl_saturating_add_mul_unsigned);

fn saturating_add_mul_signed<
    U: CheckedMul<U, Output = U> + Copy + Ord + WrappingSub<U, Output = U>,
    S: Copy
        + Max
        + Min
        + Ord
        + SaturatingAdd<S, Output = S>
        + SaturatingMul<S, Output = S>
        + UnsignedAbs<Output = U>
        + WrappingFrom<U>
        + Zero,
>(
    x: S,
    y: S,
    z: S,
) -> S {
    if y == S::ZERO || z == S::ZERO {
        return x;
    }
    let x_sign = x >= S::ZERO;
    if x_sign == ((y >= S::ZERO) == (z >= S::ZERO)) {
        x.saturating_add(y.saturating_mul(z))
    } else {
        let x = x.unsigned_abs();
        let product = if let Some(product) = y.unsigned_abs().checked_mul(z.unsigned_abs()) {
            product
        } else {
            return if x_sign { S::MIN } else { S::MAX };
        };
        let result = S::wrapping_from(if x_sign {
            x.wrapping_sub(product)
        } else {
            product.wrapping_sub(x)
        });
        if x >= product || (x_sign == (result < S::ZERO)) {
            result
        } else if x_sign {
            S::MIN
        } else {
            S::MAX
        }
    }
}

macro_rules! impl_saturating_add_mul_signed {
    ($t:ident) => {
        impl SaturatingAddMul<$t> for $t {
            type Output = $t;

            /// Computes $x + yz$, saturating at the numeric bounds instead of overflowing.
            ///
            /// $$
            /// f(x, y, z) = \\begin{cases}
            ///     x + yz & m \leq x + yz \leq M \\\\
            ///     M & x + yz > M \\\\
            ///     m & x + yz < m,
            /// \\end{cases}
            /// $$
            /// where $m$ is `$t::MIN` and $M$ is `$t::MAX`.
            ///
            /// # Worst-case complexity
            /// Constant time and additional memory.
            ///
            /// # Examples
            /// See the documentation of the `num::arithmetic::saturating_add_mul` module.
            #[inline]
            fn saturating_add_mul(self, y: $t, z: $t) -> $t {
                saturating_add_mul_signed(self, y, z)
            }
        }

        impl SaturatingAddMulAssign<$t> for $t {
            /// Replaces $x$ with $x + yz$, saturating at the numeric bounds instead of
            /// overflowing.
            ///
            /// $$
            /// x \gets \\begin{cases}
            ///     x + yz & m \leq x + yz \leq M \\\\
            ///     M & x + yz > M \\\\
            ///     m & x + yz < m,
            /// \\end{cases}
            /// $$
            /// where $m$ is `$t::MIN` and $M$ is `$t::MAX`.
            ///
            /// # Worst-case complexity
            /// Constant time and additional memory.
            ///
            /// # Examples
            /// See the documentation of the `num::arithmetic::saturating_add_mul` module.
            #[inline]
            fn saturating_add_mul_assign(&mut self, y: $t, z: $t) {
                *self = self.saturating_add_mul(y, z);
            }
        }
    };
}
apply_to_signeds!(impl_saturating_add_mul_signed);
