// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::num::arithmetic::traits::{MulAddMul, MulAddMulAssign, UnsignedAbs};
use crate::num::basic::signeds::PrimitiveSigned;
use crate::num::basic::unsigneds::PrimitiveUnsigned;
use crate::num::conversion::traits::WrappingFrom;

// Where the exact value of $xy \pm zw$ sits relative to the type's range. The products are formed
// at double width, so no intermediate overflow can hide a result that would have fit.
pub(crate) enum Wide<T> {
    Fits(T),
    Above,
    Below,
}

// The exact value of $xy + zw$, or of $xy - zw$ when `sub` is set, for an unsigned type.
pub(crate) fn mul_add_mul_wide_unsigned<T: PrimitiveUnsigned>(
    x: T,
    y: T,
    z: T,
    w: T,
    sub: bool,
) -> Wide<T> {
    let (p_1, p_0) = T::x_mul_y_to_zz(x, y);
    let (q_1, q_0) = T::x_mul_y_to_zz(z, w);
    let (r_1, r_0) = if sub {
        if (p_1, p_0) < (q_1, q_0) {
            return Wide::Below;
        }
        T::xx_sub_yy_to_zz(p_1, p_0, q_1, q_0)
    } else {
        let (r_1, r_0) = T::xx_add_yy_to_zz(p_1, p_0, q_1, q_0);
        // Two products can carry out of double width, since $(2^W-1)^2 + (2^W-1)^2 \geq 2^{2W}$,
        // and `xx_add_yy_to_zz` wraps without saying so. The sum wrapped iff it is now smaller than
        // one of the addends.
        if (r_1, r_0) < (p_1, p_0) {
            return Wide::Above;
        }
        (r_1, r_0)
    };
    if r_1 == T::ZERO {
        Wide::Fits(r_0)
    } else {
        Wide::Above
    }
}

// The exact value of $xy + zw$, or of $xy - zw$ when `sub` is set, for a signed type. The two
// products are combined as a sign and a double-width magnitude, so the sign of an out-of-range
// result is known and the saturating variants can pick the right bound.
pub(crate) fn mul_add_mul_wide_signed<
    U: PrimitiveUnsigned,
    T: PrimitiveSigned + UnsignedAbs<Output = U> + WrappingFrom<U>,
>(
    x: T,
    y: T,
    z: T,
    w: T,
    sub: bool,
) -> Wide<T> {
    let p_neg = (x < T::ZERO) != (y < T::ZERO);
    let q_neg = ((z < T::ZERO) != (w < T::ZERO)) != sub;
    let (p_1, p_0) = U::x_mul_y_to_zz(x.unsigned_abs(), y.unsigned_abs());
    let (q_1, q_0) = U::x_mul_y_to_zz(z.unsigned_abs(), w.unsigned_abs());
    let (neg, r_1, r_0) = if p_neg == q_neg {
        let (r_1, r_0) = U::xx_add_yy_to_zz(p_1, p_0, q_1, q_0);
        (p_neg, r_1, r_0)
    } else if (p_1, p_0) >= (q_1, q_0) {
        let (r_1, r_0) = U::xx_sub_yy_to_zz(p_1, p_0, q_1, q_0);
        (p_neg, r_1, r_0)
    } else {
        let (r_1, r_0) = U::xx_sub_yy_to_zz(q_1, q_0, p_1, p_0);
        (q_neg, r_1, r_0)
    };
    if r_1 != U::ZERO {
        return if neg { Wide::Below } else { Wide::Above };
    }
    if neg {
        // The negative bound has one more magnitude than the positive one.
        if r_0 <= T::MIN.unsigned_abs() {
            Wide::Fits(T::wrapping_from(r_0).wrapping_neg())
        } else {
            Wide::Below
        }
    } else if r_0 <= T::MAX.unsigned_abs() {
        Wide::Fits(T::wrapping_from(r_0))
    } else {
        Wide::Above
    }
}

macro_rules! impl_mul_add_mul_primitive_int {
    ($t:ident) => {
        impl MulAddMul for $t {
            type Output = $t;

            /// Adds the products of two pairs of numbers.
            ///
            /// $f(x, y, z, w) = xy + zw$.
            ///
            /// Both products and their sum wrap on overflow, as they do for
            /// [`add_mul`](super::traits::AddMul::add_mul).
            ///
            /// # Worst-case complexity
            /// Constant time and additional memory.
            ///
            /// # Examples
            /// See [here](super::mul_add_mul#mul_add_mul).
            #[inline]
            fn mul_add_mul(self, y: $t, z: $t, w: $t) -> $t {
                self.wrapping_mul(y).wrapping_add(z.wrapping_mul(w))
            }
        }

        impl MulAddMulAssign for $t {
            /// Adds the products of two pairs of numbers, in place.
            ///
            /// $x \gets xy + zw$.
            ///
            /// Both products and their sum wrap on overflow, as they do for
            /// [`add_mul`](super::traits::AddMul::add_mul).
            ///
            /// # Worst-case complexity
            /// Constant time and additional memory.
            ///
            /// # Examples
            /// See [here](super::mul_add_mul#mul_add_mul_assign).
            #[inline]
            fn mul_add_mul_assign(&mut self, y: $t, z: $t, w: $t) {
                *self = self.wrapping_mul(y).wrapping_add(z.wrapping_mul(w));
            }
        }
    };
}
apply_to_primitive_ints!(impl_mul_add_mul_primitive_int);
