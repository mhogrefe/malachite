// Copyright © 2024 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::num::arithmetic::traits::{
    BinomialCoefficient, CheckedBinomialCoefficient, UnsignedAbs,
};
use crate::num::basic::signeds::PrimitiveSigned;
use crate::num::basic::unsigneds::PrimitiveUnsigned;
use crate::num::conversion::traits::OverflowingFrom;
use crate::num::exhaustive::primitive_int_increasing_inclusive_range;
use core::cmp::min;

fn checked_binomial_coefficient_unsigned<T: PrimitiveUnsigned>(n: T, mut k: T) -> Option<T> {
    if k > n {
        return Some(T::ZERO);
    }
    k = min(k, n - k);
    if k == T::ZERO {
        Some(T::ONE)
    } else if k == T::ONE {
        Some(n)
    } else if k == T::TWO {
        (if n.even() { n - T::ONE } else { n }).checked_mul(n >> 1)
    } else {
        // Some binomial coefficient algorithms have intermediate results greater than the final
        // result, risking overflow. This one does not.
        let mut product = n - k + T::ONE;
        let mut numerator = product;
        for i in primitive_int_increasing_inclusive_range(T::TWO, k) {
            numerator += T::ONE;
            let gcd = numerator.gcd(i);
            product /= i / gcd;
            product = product.checked_mul(numerator / gcd)?;
        }
        Some(product)
    }
}

fn checked_binomial_coefficient_signed<
    U: PrimitiveUnsigned,
    S: OverflowingFrom<U> + PrimitiveSigned + TryFrom<U> + UnsignedAbs<Output = U>,
>(
    n: S,
    k: S,
) -> Option<S> {
    if k < S::ZERO {
        return None;
    }
    if n >= S::ZERO {
        S::try_from(U::checked_binomial_coefficient(
            n.unsigned_abs(),
            k.unsigned_abs(),
        )?)
        .ok()
    } else {
        let k = k.unsigned_abs();
        let b = U::checked_binomial_coefficient(n.unsigned_abs() + k - U::ONE, k)?;
        if k.even() {
            S::try_from(b).ok()
        } else {
            let (b, overflow) = S::overflowing_from(b);
            if overflow {
                if b == S::MIN {
                    Some(S::MIN)
                } else {
                    None
                }
            } else {
                Some(-b)
            }
        }
    }
}

macro_rules! impl_binomial_coefficient_unsigned {
    ($t:ident) => {
        impl CheckedBinomialCoefficient for $t {
            /// Computes the binomial coefficient of two numbers. If the inputs are too large, the
            /// function returns `None`.
            ///
            /// $$
            /// f(n, k) = \\begin{cases}
            ///     \operatorname{Some}(\binom{n}{k}) & \text{if} \\quad \binom{n}{k} < 2^W, \\\\
            ///     \operatorname{None} & \text{if} \\quad \binom{n}{k} \geq 2^W,
            /// \\end{cases}
            /// $$
            /// where $W$ is `Self::WIDTH`.
            ///
            /// # Worst-case complexity
            /// $T(k) = O(k)$
            ///
            /// $M(k) = O(1)$
            ///
            /// where $T$ is time, $M$ is additional memory, and $k$ is `k`.
            ///
            /// # Examples
            /// See [here](super::binomial_coefficient#checked_binomial_coefficient).
            #[inline]
            fn checked_binomial_coefficient(n: $t, k: $t) -> Option<$t> {
                checked_binomial_coefficient_unsigned(n, k)
            }
        }
    };
}
apply_to_unsigneds!(impl_binomial_coefficient_unsigned);

macro_rules! impl_binomial_coefficient_signed {
    ($t:ident) => {
        impl CheckedBinomialCoefficient for $t {
            /// Computes the binomial coefficient of two numbers. If the inputs are too large, the
            /// function returns `None`.
            ///
            /// The second argument must be non-negative, but the first may be negative. If it is,
            /// the identity $\binom{-n}{k} = (-1)^k \binom{n+k-1}{k}$ is used.
            ///
            /// $$
            /// f(n, k) = \\begin{cases}
            ///     \operatorname{Some}(\binom{n}{k}) & \text{if} \\quad n \geq 0 \\ \text{and}
            ///         \\ -2^{W-1} \leq \binom{n}{k} < 2^{W-1}, \\\\
            ///     \operatorname{Some}((-1)^k \binom{-n+k-1}{k}) & \text{if} \\quad n < 0
            ///         \\ \text{and} \\ -2^{W-1} \leq \binom{n}{k} < 2^{W-1}, \\\\
            ///     \operatorname{None} & \\quad \\text{otherwise},
            /// \\end{cases}
            /// $$
            /// where $W$ is `Self::WIDTH`.
            ///
            /// # Worst-case complexity
            /// $T(k) = O(k)$
            ///
            /// $M(k) = O(1)$
            ///
            /// where $T$ is time, $M$ is additional memory, and $k$ is `k.abs()`.
            ///
            /// # Examples
            /// See [here](super::binomial_coefficient#checked_binomial_coefficient).
            #[inline]
            fn checked_binomial_coefficient(n: $t, k: $t) -> Option<$t> {
                checked_binomial_coefficient_signed(n, k)
            }
        }
    };
}
apply_to_signeds!(impl_binomial_coefficient_signed);

macro_rules! impl_binomial_coefficient_primitive_int {
    ($t:ident) => {
        impl BinomialCoefficient for $t {
            /// Computes the binomial coefficient of two numbers. If the inputs are too large, the
            /// function panics.
            ///
            /// The second argument must be non-negative, but the first may be negative. If it is,
            /// the identity $\binom{-n}{k} = (-1)^k \binom{n+k-1}{k}$ is used.
            ///
            /// $$
            /// f(n, k) = \\begin{cases}
            ///     \binom{n}{k} & \text{if} \\quad n \geq 0, \\\\
            ///     (-1)^k \binom{-n+k-1}{k} & \text{if} \\quad n < 0.
            /// \\end{cases}
            /// $$
            ///
            /// # Worst-case complexity
            /// $T(k) = O(k)$
            ///
            /// $M(k) = O(1)$
            ///
            /// where $T$ is time, $M$ is additional memory, and $k$ is `k.abs()`.
            ///
            /// # Panics
            /// Panics if the result is not representable by this type, or if $k$ is negative.
            ///
            /// # Examples
            /// See [here](super::binomial_coefficient#binomial_coefficient).
            #[inline]
            fn binomial_coefficient(n: $t, k: $t) -> $t {
                $t::checked_binomial_coefficient(n, k).unwrap()
            }
        }
    };
}
apply_to_primitive_ints!(impl_binomial_coefficient_primitive_int);
