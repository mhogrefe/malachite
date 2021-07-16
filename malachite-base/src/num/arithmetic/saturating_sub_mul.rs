use comparison::traits::{Max, Min};
use num::arithmetic::traits::{
    CheckedMul, SaturatingMul, SaturatingSub, SaturatingSubAssign, SaturatingSubMul,
    SaturatingSubMulAssign, UnsignedAbs, WrappingSub,
};
use num::basic::traits::Zero;
use num::conversion::traits::WrappingFrom;

fn _saturating_sub_mul_unsigned<T: SaturatingMul<T, Output = T> + SaturatingSub<T, Output = T>>(
    x: T,
    y: T,
    z: T,
) -> T {
    x.saturating_sub(y.saturating_mul(z))
}

fn _saturating_sub_mul_assign_unsigned<T: SaturatingMul<T, Output = T> + SaturatingSubAssign<T>>(
    x: &mut T,
    y: T,
    z: T,
) {
    x.saturating_sub_assign(y.saturating_mul(z));
}

macro_rules! impl_saturating_sub_mul_unsigned {
    ($t:ident) => {
        impl SaturatingSubMul<$t> for $t {
            type Output = $t;

            /// Computes $x - yz$, saturating at the numeric bounds instead of overflowing.
            ///
            /// $$
            /// f(x, y, z) = \\begin{cases}
            ///     x - yz & m \leq x - yz \leq M \\\\
            ///     M & x - yz > M \\\\
            ///     m & x - yz < m,
            /// \\end{cases}
            /// $$
            /// where $m$ is `$t::MIN` and $M$ is `$t::MAX`.
            ///
            /// # Worst-case complexity
            /// Constant time and additional memory.
            ///
            /// # Examples
            /// See the documentation of the `num::arithmetic::saturating_sub_mul` module.
            #[inline]
            fn saturating_sub_mul(self, y: $t, z: $t) -> $t {
                _saturating_sub_mul_unsigned(self, y, z)
            }
        }

        impl SaturatingSubMulAssign<$t> for $t {
            /// Replaces $x$ with $x - yz$, saturating at the numeric bounds instead of
            /// overflowing.
            ///
            /// $$
            /// x \gets \\begin{cases}
            ///     x - yz & m \leq x - yz \leq M \\\\
            ///     M & x - yz > M \\\\
            ///     m & x - yz < m,
            /// \\end{cases}
            /// $$
            /// where $m$ is `$t::MIN` and $M$ is `$t::MAX`.
            ///
            /// # Worst-case complexity
            /// Constant time and additional memory.
            ///
            /// # Examples
            /// See the documentation of the `num::arithmetic::saturating_sub_mul` module.
            #[inline]
            fn saturating_sub_mul_assign(&mut self, y: $t, z: $t) {
                self.saturating_sub_assign(y.saturating_mul(z));
            }
        }
    };
}
apply_to_unsigneds!(impl_saturating_sub_mul_unsigned);

fn _saturating_sub_mul_signed<
    U: CheckedMul<U, Output = U> + Copy + Ord + WrappingSub<U, Output = U>,
    S: Copy
        + Eq
        + Max
        + Min
        + Ord
        + SaturatingMul<S, Output = S>
        + SaturatingSub<S, Output = S>
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
    if x_sign == ((y >= S::ZERO) != (z >= S::ZERO)) {
        x.saturating_sub(y.saturating_mul(z))
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

macro_rules! impl_saturating_sub_mul_signed {
    ($t:ident) => {
        impl SaturatingSubMul<$t> for $t {
            type Output = $t;

            /// Computes $x - yz$, saturating at the numeric bounds instead of overflowing.
            ///
            /// $$
            /// f(x, y, z) = \\begin{cases}
            ///     x - yz & m \leq x - yz \leq M \\\\
            ///     M & x - yz > M \\\\
            ///     m & x - yz < m,
            /// \\end{cases}
            /// $$
            /// where $m$ is `$t::MIN` and $M$ is `$t::MAX`.
            ///
            /// # Worst-case complexity
            /// Constant time and additional memory.
            ///
            /// # Examples
            /// See the documentation of the `num::arithmetic::saturating_sub_mul` module.
            #[inline]
            fn saturating_sub_mul(self, y: $t, z: $t) -> $t {
                _saturating_sub_mul_signed(self, y, z)
            }
        }

        impl SaturatingSubMulAssign<$t> for $t {
            /// Replaces $x$ with $x - yz$, saturating at the numeric bounds instead of
            /// overflowing.
            ///
            /// $$
            /// x \gets \\begin{cases}
            ///     x - yz & m \leq x - yz \leq M \\\\
            ///     M & x - yz > M \\\\
            ///     m & x - yz < m,
            /// \\end{cases}
            /// $$
            /// where $m$ is `$t::MIN` and $M$ is `$t::MAX`.
            ///
            /// # Worst-case complexity
            /// Constant time and additional memory.
            ///
            /// # Examples
            /// See the documentation of the `num::arithmetic::saturating_sub_mul` module.
            #[inline]
            fn saturating_sub_mul_assign(&mut self, y: $t, z: $t) {
                *self = self.saturating_sub_mul(y, z);
            }
        }
    };
}
apply_to_signeds!(impl_saturating_sub_mul_signed);
