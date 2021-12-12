use num::arithmetic::traits::{CheckedLcm, CheckedMul, Gcd, Lcm, LcmAssign};
use num::basic::traits::Zero;
use std::ops::Div;

#[inline]
fn lcm<
    T: CheckedMul<T, Output = T> + Copy + Div<T, Output = T> + Eq + Gcd<T, Output = T> + Zero,
>(
    x: T,
    y: T,
) -> T {
    checked_lcm(x, y).unwrap()
}

fn checked_lcm<
    T: CheckedMul<T, Output = T> + Copy + Div<T, Output = T> + Eq + Gcd<T, Output = T> + Zero,
>(
    x: T,
    y: T,
) -> Option<T> {
    if x == T::ZERO && y == T::ZERO {
        Some(T::ZERO)
    } else {
        (x / x.gcd(y)).checked_mul(y)
    }
}

macro_rules! impl_lcm {
    ($t:ident) => {
        impl Lcm<$t> for $t {
            type Output = $t;

            /// Computes the LCM (least common multiple) of two numbers.
            ///
            /// $$
            /// f(x, y) = \operatorname{lcm}(x, y).
            /// $$
            ///
            /// # Worst-case complexity
            /// Constant time and additional memory.
            ///
            /// # Panics
            /// Panics if the result is too large to fit in the type.
            ///
            /// # Examples
            /// See the documentation of the `num::arithmetic::lcm` module.
            #[inline]
            fn lcm(self, other: $t) -> $t {
                lcm(self, other)
            }
        }

        impl LcmAssign<$t> for $t {
            /// Replaces `self` with the LCM (least common multiple) of `self` and another number.
            ///
            /// $$
            /// x \gets \operatorname{lcm}(x, y).
            /// $$
            ///
            /// # Worst-case complexity
            /// Constant time and additional memory.
            ///
            /// # Panics
            /// Panics if the result is too large to fit in the type.
            ///
            /// # Examples
            /// See the documentation of the `num::arithmetic::lcm` module.
            #[inline]
            fn lcm_assign(&mut self, other: $t) {
                *self = lcm(*self, other);
            }
        }

        impl CheckedLcm<$t> for $t {
            type Output = $t;

            /// Computes the LCM (least common multiple) of two numbers, returning `None` if the
            /// result is too large to represent.
            ///
            /// $$
            /// f(x, y) = \\begin{cases}
            ///     \operatorname{Some}(\operatorname{lcm}(x, y)) &
            ///         \operatorname{lcm}(x, y) < 2^W \\\\
            ///     \operatorname{None} & \operatorname{lcm}(x, y) \geq 2^W,
            /// \\end{cases}
            /// $$
            /// where $W$ is `$t::WIDTH`.
            ///
            /// # Worst-case complexity
            /// Constant time and additional memory.
            ///
            /// # Panics
            /// Panics if the result is too large to fit in the type.
            ///
            /// # Examples
            /// See the documentation of the `num::arithmetic::lcm` module.
            #[inline]
            fn checked_lcm(self, other: $t) -> Option<$t> {
                checked_lcm(self, other)
            }
        }
    };
}
apply_to_unsigneds!(impl_lcm);
