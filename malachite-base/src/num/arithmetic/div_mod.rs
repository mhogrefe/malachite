// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::fail_on_untested_path;
use crate::num::arithmetic::mod_mul::{limbs_invert_limb_u32, limbs_invert_limb_u64};
use crate::num::arithmetic::traits::{
    CeilingDivAssignMod, CeilingDivAssignNegMod, CeilingDivMod, CeilingDivNegMod, DivAssignMod,
    DivAssignModPrecomputed, DivAssignRem, DivMod, DivModPrecomputed, DivRem, UnsignedAbs,
};
use crate::num::basic::integers::USIZE_IS_U32;
use crate::num::basic::signeds::PrimitiveSigned;
use crate::num::basic::unsigneds::PrimitiveUnsigned;
use crate::num::conversion::traits::{ExactFrom, HasHalf, JoinHalves, SplitInHalf, WrappingFrom};
use crate::num::logic::traits::LeadingZeros;

fn div_mod_unsigned<T: PrimitiveUnsigned>(x: T, other: T) -> (T, T) {
    let q = x / other;
    (q, x - q * other)
}

fn div_assign_mod_unsigned<T: PrimitiveUnsigned>(x: &mut T, other: T) -> T {
    let original = *x;
    *x /= other;
    original - *x * other
}

fn ceiling_div_neg_mod_unsigned<T: PrimitiveUnsigned>(x: T, other: T) -> (T, T) {
    let (quotient, remainder) = x.div_mod(other);
    if remainder == T::ZERO {
        (quotient, T::ZERO)
    } else {
        // Here remainder != 0, so other > 1, so quotient < T::MAX.
        (quotient + T::ONE, other - remainder)
    }
}

fn ceiling_div_assign_neg_mod_unsigned<T: PrimitiveUnsigned>(x: &mut T, other: T) -> T {
    let remainder = x.div_assign_mod(other);
    if remainder == T::ZERO {
        T::ZERO
    } else {
        // Here remainder != 0, so other > 1, so self < T::MAX.
        *x += T::ONE;
        other - remainder
    }
}

macro_rules! impl_div_mod_unsigned {
    ($t:ident) => {
        impl DivMod<$t> for $t {
            type DivOutput = $t;
            type ModOutput = $t;

            /// Divides a number by another number, returning the quotient and remainder. The
            /// quotient is rounded towards negative infinity.
            ///
            /// The quotient and remainder satisfy $x = qy + r$ and $0 \leq r < y$.
            ///
            /// $$
            /// f(x, y) = \left ( \left \lfloor \frac{x}{y} \right \rfloor, \space
            /// x - y\left \lfloor \frac{x}{y} \right \rfloor \right ).
            /// $$
            ///
            /// # Worst-case complexity
            /// Constant time and additional memory.
            ///
            /// # Panics
            /// Panics if `other` is 0.
            ///
            /// # Examples
            /// See [here](super::div_mod#div_mod).
            #[inline]
            fn div_mod(self, other: $t) -> ($t, $t) {
                div_mod_unsigned(self, other)
            }
        }

        impl DivAssignMod<$t> for $t {
            type ModOutput = $t;

            /// Divides a number by another number in place, returning the remainder. The quotient
            /// is rounded towards negative infinity.
            ///
            /// The quotient and remainder satisfy $x = qy + r$ and $0 \leq r < y$.
            ///
            /// $$
            /// f(x, y) = x - y\left \lfloor \frac{x}{y} \right \rfloor,
            /// $$
            /// $$
            /// x \gets \left \lfloor \frac{x}{y} \right \rfloor.
            /// $$
            ///
            /// # Worst-case complexity
            /// Constant time and additional memory.
            ///
            /// # Panics
            /// Panics if `other` is 0.
            ///
            /// # Examples
            /// See [here](super::div_mod#div_assign_mod).
            #[inline]
            fn div_assign_mod(&mut self, other: $t) -> $t {
                div_assign_mod_unsigned(self, other)
            }
        }

        impl DivRem<$t> for $t {
            type DivOutput = $t;
            type RemOutput = $t;

            /// Divides a number by another number, returning the quotient and remainder. The
            /// quotient is rounded towards zero.
            ///
            /// The quotient and remainder satisfy $x = qy + r$ and $0 \leq r < y$.
            ///
            /// $$
            /// f(x, y) = \left ( \left \lfloor \frac{x}{y} \right \rfloor, \space
            /// x - y\left \lfloor \frac{x}{y} \right \rfloor \right ).
            /// $$
            ///
            /// For unsigned integers, `div_rem` is equivalent to `div_mod`.
            ///
            /// # Worst-case complexity
            /// Constant time and additional memory.
            ///
            /// # Panics
            /// Panics if `other` is 0.
            ///
            /// # Examples
            /// See [here](super::div_mod#div_rem).
            #[inline]
            fn div_rem(self, other: $t) -> ($t, $t) {
                self.div_mod(other)
            }
        }

        impl DivAssignRem<$t> for $t {
            type RemOutput = $t;

            /// Divides a number by another number in place, returning the remainder. The quotient
            /// is rounded towards zero.
            ///
            /// The quotient and remainder satisfy $x = qy + r$ and $0 \leq r < y$.
            ///
            /// $$
            /// f(x, y) = x - y\left \lfloor \frac{x}{y} \right \rfloor,
            /// $$
            /// $$
            /// x \gets \left \lfloor \frac{x}{y} \right \rfloor.
            /// $$
            ///
            /// For unsigned integers, `div_assign_rem` is equivalent to `div_assign_mod`.
            ///
            /// # Worst-case complexity
            /// Constant time and additional memory.
            ///
            /// # Panics
            /// Panics if `other` is 0.
            ///
            /// # Examples
            /// See [here](super::div_mod#div_assign_rem).
            #[inline]
            fn div_assign_rem(&mut self, other: $t) -> $t {
                self.div_assign_mod(other)
            }
        }

        impl CeilingDivNegMod<$t> for $t {
            type DivOutput = $t;
            type ModOutput = $t;

            /// Divides a number by another number, returning the ceiling of the quotient and the
            /// remainder of the negative of the first number divided by the second.
            ///
            /// The quotient and remainder satisfy $x = qy - r$ and $0 \leq r < y$.
            ///
            /// $$
            /// f(x, y) = \left ( \left \lceil \frac{x}{y} \right \rceil, \space
            /// y\left \lceil \frac{x}{y} \right \rceil - x \right ).
            /// $$
            ///
            /// # Worst-case complexity
            /// Constant time and additional memory.
            ///
            /// # Panics
            /// Panics if `other` is 0.
            ///
            /// # Examples
            /// See [here](super::div_mod#ceiling_div_neg_mod).
            #[inline]
            fn ceiling_div_neg_mod(self, other: $t) -> ($t, $t) {
                ceiling_div_neg_mod_unsigned(self, other)
            }
        }

        impl CeilingDivAssignNegMod<$t> for $t {
            type ModOutput = $t;

            /// Divides a number by another number in place, returning the remainder of the negative
            /// of the first number divided by the second.
            ///
            /// The quotient and remainder satisfy $x = qy - r$ and $0 \leq r < y$.
            ///
            /// $$
            /// f(x, y) = y\left \lceil \frac{x}{y} \right \rceil - x,
            /// $$
            /// $$
            /// x \gets \left \lceil \frac{x}{y} \right \rceil.
            /// $$
            ///
            /// # Worst-case complexity
            /// Constant time and additional memory.
            ///
            /// # Panics
            /// Panics if `other` is 0.
            ///
            /// # Examples
            /// See [here](super::div_mod#ceiling_div_assign_neg_mod).
            #[inline]
            fn ceiling_div_assign_neg_mod(&mut self, other: $t) -> $t {
                ceiling_div_assign_neg_mod_unsigned(self, other)
            }
        }
    };
}
apply_to_unsigneds!(impl_div_mod_unsigned);

fn div_mod_signed<
    U: PrimitiveUnsigned,
    S: PrimitiveSigned + ExactFrom<U> + UnsignedAbs<Output = U> + WrappingFrom<U>,
>(
    x: S,
    other: S,
) -> (S, S) {
    let (quotient, remainder) = if (x >= S::ZERO) == (other >= S::ZERO) {
        let (quotient, remainder) = x.unsigned_abs().div_mod(other.unsigned_abs());
        (S::exact_from(quotient), remainder)
    } else {
        let (quotient, remainder) = x.unsigned_abs().ceiling_div_neg_mod(other.unsigned_abs());
        (S::wrapping_from(quotient).wrapping_neg(), remainder)
    };
    (
        quotient,
        if other >= S::ZERO {
            S::exact_from(remainder)
        } else {
            -S::exact_from(remainder)
        },
    )
}

fn div_rem_signed<T: PrimitiveSigned>(x: T, other: T) -> (T, T) {
    let q = x.checked_div(other).unwrap();
    (q, x - q * other)
}

fn div_assign_rem_signed<T: PrimitiveSigned>(x: &mut T, other: T) -> T {
    let original = *x;
    *x = x.checked_div(other).unwrap();
    original - *x * other
}

fn ceiling_div_mod_signed<
    U: PrimitiveUnsigned,
    T: PrimitiveSigned + ExactFrom<U> + UnsignedAbs<Output = U> + WrappingFrom<U>,
>(
    x: T,
    other: T,
) -> (T, T) {
    let (quotient, remainder) = if (x >= T::ZERO) == (other >= T::ZERO) {
        let (quotient, remainder) = x.unsigned_abs().ceiling_div_neg_mod(other.unsigned_abs());
        (T::exact_from(quotient), remainder)
    } else {
        let (quotient, remainder) = x.unsigned_abs().div_mod(other.unsigned_abs());
        (T::wrapping_from(quotient).wrapping_neg(), remainder)
    };
    (
        quotient,
        if other >= T::ZERO {
            -T::exact_from(remainder)
        } else {
            T::exact_from(remainder)
        },
    )
}

macro_rules! impl_div_mod_signed {
    ($t:ident) => {
        impl DivMod<$t> for $t {
            type DivOutput = $t;
            type ModOutput = $t;

            /// Divides a number by another number, returning the quotient and remainder. The
            /// quotient is rounded towards negative infinity, and the remainder has the same sign
            /// as the second number.
            ///
            /// The quotient and remainder satisfy $x = qy + r$ and $0 \leq |r| < |y|$.
            ///
            /// $$
            /// f(x, y) = \left ( \left \lfloor \frac{x}{y} \right \rfloor, \space
            /// x - y\left \lfloor \frac{x}{y} \right \rfloor \right ).
            /// $$
            ///
            /// # Worst-case complexity
            /// Constant time and additional memory.
            ///
            /// # Panics
            /// Panics if `other` is 0, or if `self` is `$t::MIN` and `other` is -1.
            ///
            /// # Examples
            /// See [here](super::div_mod#div_mod).
            #[inline]
            fn div_mod(self, other: $t) -> ($t, $t) {
                div_mod_signed(self, other)
            }
        }

        impl DivAssignMod<$t> for $t {
            type ModOutput = $t;

            /// Divides a number by another number in place, returning the remainder. The quotient
            /// is rounded towards negative infinity, and the remainder has the same sign as the
            /// second number.
            ///
            /// The quotient and remainder satisfy $x = qy + r$ and $0 \leq |r| < |y|$.
            ///
            /// $$
            /// f(x, y) = x - y\left \lfloor \frac{x}{y} \right \rfloor,
            /// $$
            /// $$
            /// x \gets \left \lfloor \frac{x}{y} \right \rfloor.
            /// $$
            ///
            /// # Worst-case complexity
            /// Constant time and additional memory.
            ///
            /// # Panics
            /// Panics if `other` is 0, or if `self` is `$t::MIN` and `other` is -1.
            ///
            /// # Examples
            /// See [here](super::div_mod#div_assign_mod).
            #[inline]
            fn div_assign_mod(&mut self, other: $t) -> $t {
                let (q, r) = self.div_mod(other);
                *self = q;
                r
            }
        }

        impl DivRem<$t> for $t {
            type DivOutput = $t;
            type RemOutput = $t;

            /// Divides a number by another number, returning the quotient and remainder. The
            /// quotient is rounded towards zero and the remainder has the same sign as the
            /// dividend.
            ///
            /// The quotient and remainder satisfy $x = qy + r$ and $0 \leq |r| < |y|$.
            ///
            /// $$
            /// f(x, y) = \left ( \operatorname{sgn}(xy) \left \lfloor \left | \frac{x}{y} \right |
            /// \right \rfloor, \space
            /// x - y \operatorname{sgn}(xy)
            /// \left \lfloor \left | \frac{x}{y} \right | \right \rfloor \right ).
            /// $$
            ///
            /// # Worst-case complexity
            /// Constant time and additional memory.
            ///
            /// # Panics
            /// Panics if `other` is 0, or if `self` is `$t::MIN` and `other` is -1.
            ///
            /// # Examples
            /// See [here](super::div_mod#div_rem).
            #[inline]
            fn div_rem(self, other: $t) -> ($t, $t) {
                div_rem_signed(self, other)
            }
        }

        impl DivAssignRem<$t> for $t {
            type RemOutput = $t;

            /// Divides a number by another number in place, returning the remainder. The quotient
            /// is rounded towards zero and the remainder has the same sign as the dividend.
            ///
            /// The quotient and remainder satisfy $x = qy + r$ and $0 \leq |r| < |y|$.
            ///
            /// $$
            /// f(x, y) = x - y \operatorname{sgn}(xy)
            /// \left \lfloor \left | \frac{x}{y} \right | \right \rfloor,
            /// $$
            /// $$
            /// x \gets \operatorname{sgn}(xy) \left \lfloor \left | \frac{x}{y} \right |
            /// \right \rfloor.
            /// $$
            ///
            /// # Worst-case complexity
            /// Constant time and additional memory.
            ///
            /// # Panics
            /// Panics if `other` is 0, or if `self` is `$t::MIN` and `other` is -1.
            ///
            /// # Examples
            /// See [here](super::div_mod#div_assign_rem).
            #[inline]
            fn div_assign_rem(&mut self, other: $t) -> $t {
                div_assign_rem_signed(self, other)
            }
        }

        impl CeilingDivMod<$t> for $t {
            type DivOutput = $t;
            type ModOutput = $t;

            /// Divides a number by another number, returning the quotient and remainder. The
            /// quotient is rounded towards positive infinity and the remainder has the opposite
            /// sign as the second number.
            ///
            /// The quotient and remainder satisfy $x = qy + r$ and $0 \leq |r| < |y|$.
            ///
            /// $$
            /// f(x, y) = \left ( \left \lceil \frac{x}{y} \right \rceil, \space
            /// x - y\left \lceil \frac{x}{y} \right \rceil \right ).
            /// $$
            ///
            /// # Worst-case complexity
            /// Constant time and additional memory.
            ///
            /// # Panics
            /// Panics if `other` is 0, or if `self` is `$t::MIN` and `other` is -1.
            ///
            /// # Examples
            /// See [here](super::div_mod#ceiling_div_mod).
            #[inline]
            fn ceiling_div_mod(self, other: $t) -> ($t, $t) {
                ceiling_div_mod_signed(self, other)
            }
        }

        impl CeilingDivAssignMod<$t> for $t {
            type ModOutput = $t;

            /// Divides a number by another number in place, returning the remainder. The quotient
            /// is rounded towards positive infinity and the remainder has the opposite sign as the
            /// second number.
            ///
            /// The quotient and remainder satisfy $x = qy + r$ and $0 \leq |r| < |y|$.
            ///
            /// $$
            /// f(x, y) = x - y\left \lceil\frac{x}{y} \right \rceil,
            /// $$
            /// $$
            /// x \gets \left \lceil \frac{x}{y} \right \rceil.
            /// $$
            ///
            /// # Worst-case complexity
            /// Constant time and additional memory.
            ///
            /// # Panics
            /// Panics if `other` is 0, or if `self` is `$t::MIN` and `other` is -1.
            ///
            /// # Examples
            /// See [here](super::div_mod#ceiling_div_assign_mod).
            #[inline]
            fn ceiling_div_assign_mod(&mut self, other: $t) -> $t {
                let (q, r) = self.ceiling_div_mod(other);
                *self = q;
                r
            }
        }
    };
}
apply_to_signeds!(impl_div_mod_signed);

// Divides `x` by `d`, given `shift`, the number of leading zeros of `d`, and `d_inv`, the inverse
// of `d << shift` computed by `limbs_invert_limb`.
//
// This is equivalent to `udiv_qrnnd_preinv` from `gmp-impl.h`, GMP 6.2.1, where the dividend
// occupies a single limb.
fn div_mod_preinverted<
    T: PrimitiveUnsigned,
    DT: From<T> + HasHalf<Half = T> + JoinHalves + PrimitiveUnsigned + SplitInHalf,
>(
    x: T,
    d: T,
    d_inv: T,
    shift: u64,
) -> (T, T) {
    let d = d << shift;
    let (y_1, y_0) = if shift == 0 {
        (T::ZERO, x)
    } else {
        (x >> (T::WIDTH - shift), x << shift)
    };
    let (q_1, q_0) = (DT::from(d_inv) * DT::from(y_1))
        .wrapping_add(DT::join_halves(y_1, y_0))
        .split_in_half();
    let mut q = q_1.wrapping_add(T::ONE);
    let mut r = y_0.wrapping_sub(q.wrapping_mul(d));
    if r > q_0 {
        q.wrapping_sub_assign(T::ONE);
        r.wrapping_add_assign(d);
    }
    if r >= d {
        // The generic `udiv_qrnnd_preinv` needs this second adjustment, but with a single-limb
        // dividend `y_1 < 2^shift <= d`, and the tighter estimate never seems to require it:
        // exhaustively verified for 8-bit limbs, and never observed in large random 64-bit sweeps.
        fail_on_untested_path("div_mod_preinverted, second adjustment");
        q += T::ONE;
        r -= d;
    }
    (q, r >> shift)
}

macro_rules! impl_div_mod_precomputed_fast {
    ($t:ident, $dt:ident, $invert_limb:ident) => {
        impl DivModPrecomputed<$t> for $t {
            type DivOutput = $t;
            type ModOutput = $t;
            type Data = ($t, u64);

            /// Precomputes data for division: the `limbs_invert_limb`-style inverse of the
            /// normalized divisor, and the normalizing shift. See `div_mod_precomputed` and
            /// [`div_assign_mod_precomputed`](super::traits::DivAssignModPrecomputed).
            ///
            /// # Worst-case complexity
            /// Constant time and additional memory.
            ///
            /// # Panics
            /// Panics if `other` is 0.
            ///
            /// This is equivalent to `n_preinvert_limb` from `ulong_extras.h`, FLINT 2.7.1, with
            /// the normalizing shift retained, as in FLINT's `nmod_t`.
            fn precompute_div_mod_data(&other: &$t) -> ($t, u64) {
                assert_ne!(other, 0, "division by zero");
                let shift = LeadingZeros::leading_zeros(other);
                ($invert_limb(other << shift), shift)
            }

            /// Divides a number by another number, returning the quotient and remainder.
            ///
            /// The quotient and remainder satisfy $x = qy + r$ and $0 \leq r < y$.
            ///
            /// Some precomputed data is provided; this speeds up computations involving several
            /// divisions by the same divisor. The precomputed data should be obtained using
            /// [`precompute_div_mod_data`](DivModPrecomputed::precompute_div_mod_data).
            ///
            /// # Worst-case complexity
            /// Constant time and additional memory.
            ///
            /// This trades the hardware division for a widening multiplication and adjustments,
            /// which pays off on processors whose dividers are slow relative to their multipliers;
            /// on processors with fast, pipelined dividers, plain division may be faster.
            ///
            /// # Examples
            /// See [here](super::div_mod#div_mod_precomputed).
            ///
            /// This is equivalent to `udiv_qrnnd_preinv` from `gmp-impl.h`, GMP 6.2.1, where the
            /// dividend occupies a single limb.
            #[inline]
            fn div_mod_precomputed(self, other: $t, data: &($t, u64)) -> ($t, $t) {
                div_mod_preinverted::<$t, $dt>(self, other, data.0, data.1)
            }
        }
    };
}
impl_div_mod_precomputed_fast!(u32, u64, limbs_invert_limb_u32);
impl_div_mod_precomputed_fast!(u64, u128, limbs_invert_limb_u64);

macro_rules! impl_div_mod_precomputed_promoted {
    ($t:ident) => {
        impl DivModPrecomputed<$t> for $t {
            type DivOutput = $t;
            type ModOutput = $t;
            type Data = (u32, u64);

            /// Precomputes data for division. See `div_mod_precomputed` and
            /// [`div_assign_mod_precomputed`](super::traits::DivAssignModPrecomputed).
            ///
            /// # Worst-case complexity
            /// Constant time and additional memory.
            ///
            /// # Panics
            /// Panics if `other` is 0.
            ///
            /// This is equivalent to `n_preinvert_limb` from `ulong_extras.h`, FLINT 2.7.1.
            fn precompute_div_mod_data(&other: &$t) -> (u32, u64) {
                u32::precompute_div_mod_data(&u32::from(other))
            }

            /// Divides a number by another number, returning the quotient and remainder.
            ///
            /// The quotient and remainder satisfy $x = qy + r$ and $0 \leq r < y$.
            ///
            /// Some precomputed data is provided; this speeds up computations involving several
            /// divisions by the same divisor. The precomputed data should be obtained using
            /// [`precompute_div_mod_data`](DivModPrecomputed::precompute_div_mod_data).
            ///
            /// # Worst-case complexity
            /// Constant time and additional memory.
            ///
            /// # Examples
            /// See [here](super::div_mod#div_mod_precomputed).
            #[inline]
            fn div_mod_precomputed(self, other: $t, data: &(u32, u64)) -> ($t, $t) {
                let (q, r) = u32::from(self).div_mod_precomputed(u32::from(other), data);
                ($t::wrapping_from(q), $t::wrapping_from(r))
            }
        }
    };
}
impl_div_mod_precomputed_promoted!(u8);
impl_div_mod_precomputed_promoted!(u16);

impl DivModPrecomputed<Self> for u128 {
    type DivOutput = Self;
    type ModOutput = Self;
    type Data = ();

    /// Precomputes data for division. See `div_mod_precomputed` and
    /// [`div_assign_mod_precomputed`](super::traits::DivAssignModPrecomputed).
    ///
    /// # Worst-case complexity
    /// Constant time and additional memory.
    ///
    /// # Panics
    /// Panics if `other` is 0.
    fn precompute_div_mod_data(&other: &Self) {
        assert_ne!(other, 0, "division by zero");
    }

    /// Divides a number by another number, returning the quotient and remainder.
    ///
    /// The quotient and remainder satisfy $x = qy + r$ and $0 \leq r < y$.
    ///
    /// Some precomputed data is provided; this speeds up computations involving several divisions
    /// by the same divisor. The precomputed data should be obtained using
    /// [`precompute_div_mod_data`](DivModPrecomputed::precompute_div_mod_data).
    ///
    /// # Worst-case complexity
    /// Constant time and additional memory.
    ///
    /// # Panics
    /// Panics if `other` is 0.
    ///
    /// # Examples
    /// See [here](super::div_mod#div_mod_precomputed).
    #[inline]
    fn div_mod_precomputed(self, other: Self, _data: &()) -> (Self, Self) {
        self.div_mod(other)
    }
}

impl DivModPrecomputed<Self> for usize {
    type DivOutput = Self;
    type ModOutput = Self;
    type Data = (Self, u64);

    /// Precomputes data for division. See `div_mod_precomputed` and
    /// [`div_assign_mod_precomputed`](super::traits::DivAssignModPrecomputed).
    ///
    /// # Worst-case complexity
    /// Constant time and additional memory.
    ///
    /// # Panics
    /// Panics if `other` is 0.
    ///
    /// This is equivalent to `n_preinvert_limb` from `ulong_extras.h`, FLINT 2.7.1.
    fn precompute_div_mod_data(&other: &Self) -> (Self, u64) {
        if USIZE_IS_U32 {
            let (d_inv, shift) = u32::precompute_div_mod_data(&u32::wrapping_from(other));
            (Self::wrapping_from(d_inv), shift)
        } else {
            let (d_inv, shift) = u64::precompute_div_mod_data(&u64::wrapping_from(other));
            (Self::wrapping_from(d_inv), shift)
        }
    }

    /// Divides a number by another number, returning the quotient and remainder.
    ///
    /// The quotient and remainder satisfy $x = qy + r$ and $0 \leq r < y$.
    ///
    /// Some precomputed data is provided; this speeds up computations involving several divisions
    /// by the same divisor. The precomputed data should be obtained using
    /// [`precompute_div_mod_data`](DivModPrecomputed::precompute_div_mod_data).
    ///
    /// # Worst-case complexity
    /// Constant time and additional memory.
    ///
    /// # Examples
    /// See [here](super::div_mod#div_mod_precomputed).
    fn div_mod_precomputed(self, other: Self, data: &(Self, u64)) -> (Self, Self) {
        if USIZE_IS_U32 {
            let (q, r) = u32::wrapping_from(self).div_mod_precomputed(
                u32::wrapping_from(other),
                &(u32::wrapping_from(data.0), data.1),
            );
            (Self::wrapping_from(q), Self::wrapping_from(r))
        } else {
            let (q, r) = u64::wrapping_from(self).div_mod_precomputed(
                u64::wrapping_from(other),
                &(u64::wrapping_from(data.0), data.1),
            );
            (Self::wrapping_from(q), Self::wrapping_from(r))
        }
    }
}

// The remainder is with respect to the ceiling quotient: `x = qy - r` and `0 <= r < y`.
fn ceiling_div_neg_mod_precomputed_unsigned<
    U: PrimitiveUnsigned + DivModPrecomputed<U, DivOutput = U, ModOutput = U>,
>(
    x: U,
    other: U,
    data: &<U as DivModPrecomputed<U>>::Data,
) -> (U, U) {
    let (quotient, remainder) = x.div_mod_precomputed(other, data);
    if remainder == U::ZERO {
        (quotient, U::ZERO)
    } else {
        // Here remainder != 0, so other > 1, so quotient < U::MAX.
        (quotient + U::ONE, other - remainder)
    }
}

fn div_mod_precomputed_signed<
    U: PrimitiveUnsigned + DivModPrecomputed<U, DivOutput = U, ModOutput = U>,
    S: PrimitiveSigned + ExactFrom<U> + UnsignedAbs<Output = U> + WrappingFrom<U>,
>(
    x: S,
    other: S,
    data: &<U as DivModPrecomputed<U>>::Data,
) -> (S, S) {
    let (quotient, remainder) = if (x >= S::ZERO) == (other >= S::ZERO) {
        let (quotient, remainder) = x
            .unsigned_abs()
            .div_mod_precomputed(other.unsigned_abs(), data);
        (S::exact_from(quotient), remainder)
    } else {
        let (quotient, remainder) =
            ceiling_div_neg_mod_precomputed_unsigned(x.unsigned_abs(), other.unsigned_abs(), data);
        (S::wrapping_from(quotient).wrapping_neg(), remainder)
    };
    (
        quotient,
        if other >= S::ZERO {
            S::exact_from(remainder)
        } else {
            -S::exact_from(remainder)
        },
    )
}

macro_rules! impl_div_mod_precomputed_signed {
    ($u:ident, $t:ident) => {
        impl DivModPrecomputed<$t> for $t {
            type DivOutput = $t;
            type ModOutput = $t;
            type Data = <$u as DivModPrecomputed<$u>>::Data;

            /// Precomputes data for division. See `div_mod_precomputed` and
            /// [`div_assign_mod_precomputed`](super::traits::DivAssignModPrecomputed).
            ///
            /// The data depends only on the absolute value of the divisor.
            ///
            /// # Worst-case complexity
            /// Constant time and additional memory.
            ///
            /// # Panics
            /// Panics if `other` is 0.
            #[inline]
            fn precompute_div_mod_data(&other: &$t) -> Self::Data {
                $u::precompute_div_mod_data(&other.unsigned_abs())
            }

            /// Divides a number by another number, returning the quotient and remainder. The
            /// quotient is rounded towards negative infinity, and the remainder has the same sign
            /// as the second number.
            ///
            /// The quotient and remainder satisfy $x = qy + r$ and $0 \leq |r| < |y|$.
            ///
            /// Some precomputed data is provided; this speeds up computations involving several
            /// divisions by the same divisor. The precomputed data should be obtained using
            /// [`precompute_div_mod_data`](DivModPrecomputed::precompute_div_mod_data).
            ///
            /// # Worst-case complexity
            /// Constant time and additional memory.
            ///
            /// # Panics
            /// Panics if `self` is `$t::MIN` and `other` is -1.
            ///
            /// # Examples
            /// See [here](super::div_mod#div_mod_precomputed).
            #[inline]
            fn div_mod_precomputed(self, other: $t, data: &Self::Data) -> ($t, $t) {
                div_mod_precomputed_signed::<$u, $t>(self, other, data)
            }
        }
    };
}
apply_to_unsigned_signed_pairs!(impl_div_mod_precomputed_signed);

macro_rules! impl_div_assign_mod_precomputed {
    ($t:ident) => {
        impl DivAssignModPrecomputed<$t> for $t {
            /// Divides a number by another number in place, returning the remainder. The quotient
            /// is rounded towards negative infinity, and the remainder has the same sign as the
            /// second number.
            ///
            /// The quotient and remainder satisfy $x = qy + r$ and $0 \leq |r| < |y|$.
            ///
            /// Some precomputed data is provided; this speeds up computations involving several
            /// divisions by the same divisor. The precomputed data should be obtained using
            /// [`precompute_div_mod_data`](DivModPrecomputed::precompute_div_mod_data).
            ///
            /// # Worst-case complexity
            /// Constant time and additional memory.
            ///
            /// # Examples
            /// See [here](super::div_mod#div_assign_mod_precomputed).
            #[inline]
            fn div_assign_mod_precomputed(&mut self, other: $t, data: &Self::Data) -> $t {
                let (q, r) = self.div_mod_precomputed(other, data);
                *self = q;
                r
            }
        }
    };
}
apply_to_primitive_ints!(impl_div_assign_mod_precomputed);
