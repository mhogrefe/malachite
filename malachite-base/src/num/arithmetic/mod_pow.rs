// Copyright © 2025 Mikhail Hogrefe
//
// Uses code adopted from the FLINT Library.
//
//      Copyright © 2009, 2010, 2012, 2016 William Hart
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::num::arithmetic::mod_mul::{limbs_invert_limb_u32, limbs_invert_limb_u64};
use crate::num::arithmetic::traits::{
    ModPow, ModPowAssign, ModPowPrecomputed, ModPowPrecomputedAssign,
};
use crate::num::basic::integers::USIZE_IS_U32;
use crate::num::basic::unsigneds::PrimitiveUnsigned;
use crate::num::conversion::traits::WrappingFrom;
use crate::num::conversion::traits::{HasHalf, JoinHalves, SplitInHalf};
use crate::num::logic::traits::{BitIterable, LeadingZeros};

pub_test! {simple_binary_mod_pow<T: PrimitiveUnsigned>(x: T, exp: u64, m: T) -> T {
    assert!(x < m, "x must be reduced mod m, but {x} >= {m}");
    if m == T::ONE {
        return T::ZERO;
    }
    let data = T::precompute_mod_mul_data(&m);
    let mut out = T::ONE;
    for bit in exp.bits().rev() {
        out.mod_mul_precomputed_assign(out, m, &data);
        if bit {
            out.mod_mul_precomputed_assign(x, m, &data);
        }
    }
    out
}}

// m.get_highest_bit(), x < m, y < m
//
// This is equivalent to `n_mulmod_preinv` from `ulong_extras/mulmod_preinv.c`, FLINT 2.7.1.
pub(crate) fn mul_mod_helper<
    T: PrimitiveUnsigned,
    DT: From<T> + HasHalf<Half = T> + JoinHalves + PrimitiveUnsigned + SplitInHalf,
>(
    mut x: T,
    y: T,
    m: T,
    inverse: T,
    shift: u64,
) -> T {
    x >>= shift;
    let p = DT::from(x) * DT::from(y);
    let (p_hi, p_lo) = p.split_in_half();
    let (q_1, q_0) = (DT::from(p_hi) * DT::from(inverse))
        .wrapping_add(p)
        .split_in_half();
    let mut r = p_lo.wrapping_sub(q_1.wrapping_add(T::ONE).wrapping_mul(m));
    if r > q_0 {
        r.wrapping_add_assign(m);
    }
    if r < m {
        r
    } else {
        r.wrapping_sub(m)
    }
}

// m.get_highest_bit(), x < m
//
// This is equivalent to `n_powmod_ui_preinv` from ulong_extras/powmod_ui_preinv.c, FLINT 2.7.1.
pub(crate) fn fast_mod_pow<
    T: PrimitiveUnsigned,
    DT: From<T> + HasHalf<Half = T> + JoinHalves + PrimitiveUnsigned + SplitInHalf,
>(
    mut x: T,
    exp: u64,
    m: T,
    inverse: T,
    shift: u64,
) -> T {
    assert!(x < m, "x must be reduced mod m, but {x} >= {m}");
    if exp == 0 {
        let x = T::power_of_2(shift);
        if x == m {
            T::ZERO
        } else {
            x
        }
    } else if x == T::ZERO {
        T::ZERO
    } else {
        let mut bits = exp.bits();
        let mut out = if bits.next().unwrap() {
            x
        } else {
            T::power_of_2(shift)
        };
        for bit in bits {
            x = mul_mod_helper::<T, DT>(x, x, m, inverse, shift);
            if bit {
                out = mul_mod_helper::<T, DT>(out, x, m, inverse, shift);
            }
        }
        out
    }
}

macro_rules! impl_mod_pow_precomputed_fast {
    ($t:ident, $dt:ident, $invert_limb:ident) => {
        impl ModPowPrecomputed<u64, $t> for $t {
            type Output = $t;
            type Data = ($t, u64);

            /// Precomputes data for modular exponentiation.
            ///
            /// See `mod_pow_precomputed` and
            /// [`mod_pow_precomputed_assign`](super::traits::ModPowPrecomputedAssign).
            ///
            /// # Worst-case complexity
            /// Constant time and additional memory.
            fn precompute_mod_pow_data(&m: &$t) -> ($t, u64) {
                let leading_zeros = LeadingZeros::leading_zeros(m);
                ($invert_limb(m << leading_zeros), leading_zeros)
            }

            /// Raises a number to a power modulo another number $m$. The base must be already
            /// reduced modulo $m$.
            ///
            /// Some precomputed data is provided; this speeds up computations involving several
            /// modular exponentiations with the same modulus. The precomputed data should be
            /// obtained using
            /// [`precompute_mod_pow_data`](ModPowPrecomputed::precompute_mod_pow_data).
            ///
            /// # Worst-case complexity
            /// $T(n) = O(n)$
            ///
            /// $M(n) = O(1)$
            ///
            /// where $T$ is time, $M$ is additional memory, and $n$ is `exp.significant_bits()`.
            ///
            /// # Panics
            /// Panics if `self` is greater than or equal to `m`.
            ///
            /// # Examples
            /// See [here](super::mod_pow#mod_pow_precomputed).
            fn mod_pow_precomputed(self, exp: u64, m: $t, data: &($t, u64)) -> $t {
                let (inverse, shift) = *data;
                fast_mod_pow::<$t, $dt>(self << shift, exp, m << shift, inverse, shift) >> shift
            }
        }
    };
}
impl_mod_pow_precomputed_fast!(u32, u64, limbs_invert_limb_u32);
impl_mod_pow_precomputed_fast!(u64, u128, limbs_invert_limb_u64);

macro_rules! impl_mod_pow_precomputed_promoted {
    ($t:ident) => {
        impl ModPowPrecomputed<u64, $t> for $t {
            type Output = $t;
            type Data = (u32, u64);

            /// Precomputes data for modular exponentiation.
            ///
            /// See `mod_pow_precomputed` and
            /// [`mod_pow_precomputed_assign`](super::traits::ModPowPrecomputedAssign).
            ///
            /// # Worst-case complexity
            /// Constant time and additional memory.
            fn precompute_mod_pow_data(&m: &$t) -> (u32, u64) {
                u32::precompute_mod_pow_data(&u32::from(m))
            }

            /// Raises a number to a power modulo another number $m$. The base must be already
            /// reduced modulo $m$.
            ///
            /// Some precomputed data is provided; this speeds up computations involving several
            /// modular exponentiations with the same modulus. The precomputed data should be
            /// obtained using
            /// [`precompute_mod_pow_data`](ModPowPrecomputed::precompute_mod_pow_data).
            ///
            /// # Worst-case complexity
            /// $T(n) = O(n)$
            ///
            /// $M(n) = O(1)$
            ///
            /// where $T$ is time, $M$ is additional memory, and $n$ is `exp.significant_bits()`.
            ///
            /// # Panics
            /// Panics if `self` is greater than or equal to `m`.
            ///
            /// # Examples
            /// See [here](super::mod_pow#mod_pow_precomputed).
            fn mod_pow_precomputed(self, exp: u64, m: $t, data: &(u32, u64)) -> $t {
                $t::wrapping_from(u32::from(self).mod_pow_precomputed(exp, u32::from(m), data))
            }
        }
    };
}
impl_mod_pow_precomputed_promoted!(u8);
impl_mod_pow_precomputed_promoted!(u16);

impl ModPowPrecomputed<u64, u128> for u128 {
    type Output = u128;
    type Data = ();

    /// Precomputes data for modular exponentiation.
    ///
    /// See `mod_pow_precomputed` and
    /// [`mod_pow_precomputed_assign`](super::traits::ModPowPrecomputedAssign).
    ///
    /// # Worst-case complexity
    /// Constant time and additional memory.
    fn precompute_mod_pow_data(_m: &u128) {}

    /// Raises a number to a power modulo another number $m$. The base must be already reduced
    /// modulo $m$.
    ///
    /// Some precomputed data is provided; this speeds up computations involving several modular
    /// exponentiations with the same modulus. The precomputed data should be obtained using
    /// [`precompute_mod_pow_data`](ModPowPrecomputed::precompute_mod_pow_data).
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(1)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `exp.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `self` is greater than or equal to `m`.
    ///
    /// # Examples
    /// See [here](super::mod_pow#mod_pow_precomputed).
    #[inline]
    fn mod_pow_precomputed(self, exp: u64, m: u128, _data: &()) -> u128 {
        simple_binary_mod_pow(self, exp, m)
    }
}

impl ModPowPrecomputed<u64, usize> for usize {
    type Output = usize;
    type Data = (usize, u64);

    /// Precomputes data for modular exponentiation.
    ///
    /// See `mod_pow_precomputed` and
    /// [`mod_pow_precomputed_assign`](super::traits::ModPowPrecomputedAssign).
    ///
    /// # Worst-case complexity
    /// Constant time and additional memory.
    fn precompute_mod_pow_data(&m: &usize) -> (usize, u64) {
        if USIZE_IS_U32 {
            let (inverse, shift) = u32::precompute_mod_pow_data(&u32::wrapping_from(m));
            (usize::wrapping_from(inverse), shift)
        } else {
            let (inverse, shift) = u64::precompute_mod_pow_data(&u64::wrapping_from(m));
            (usize::wrapping_from(inverse), shift)
        }
    }

    /// Raises a number to a power modulo another number $m$. The base must be already reduced
    /// modulo $m$.
    ///
    /// Some precomputed data is provided; this speeds up computations involving several modular
    /// exponentiations with the same modulus. The precomputed data should be obtained using
    /// [`precompute_mod_pow_data`](ModPowPrecomputed::precompute_mod_pow_data).
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(1)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `exp.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `self` is greater than or equal to `m`.
    ///
    /// # Examples
    /// See [here](super::mod_pow#mod_pow_precomputed).
    fn mod_pow_precomputed(self, exp: u64, m: usize, data: &(usize, u64)) -> usize {
        let (inverse, shift) = *data;
        if USIZE_IS_U32 {
            usize::wrapping_from(u32::wrapping_from(self).mod_pow_precomputed(
                exp,
                u32::wrapping_from(m),
                &(u32::wrapping_from(inverse), shift),
            ))
        } else {
            usize::wrapping_from(u64::wrapping_from(self).mod_pow_precomputed(
                exp,
                u64::wrapping_from(m),
                &(u64::wrapping_from(inverse), shift),
            ))
        }
    }
}

macro_rules! impl_mod_pow {
    ($t:ident) => {
        impl ModPowPrecomputedAssign<u64, $t> for $t {
            /// Raises a number to a power modulo another number $m$, in place. The base must be
            /// already reduced modulo $m$.
            ///
            /// Some precomputed data is provided; this speeds up computations involving several
            /// modular exponentiations with the same modulus. The precomputed data should be
            /// obtained using
            /// [`precompute_mod_pow_data`](ModPowPrecomputed::precompute_mod_pow_data).
            ///
            /// # Worst-case complexity
            /// $T(n) = O(n)$
            ///
            /// $M(n) = O(1)$
            ///
            /// where $T$ is time, $M$ is additional memory, and $n$ is `exp.significant_bits()`.
            ///
            /// # Examples
            /// See [here](super::mod_pow#mod_pow_precomputed_assign).
            #[inline]
            fn mod_pow_precomputed_assign(&mut self, exp: u64, m: $t, data: &Self::Data) {
                *self = self.mod_pow_precomputed(exp, m, data);
            }
        }

        impl ModPow<u64> for $t {
            type Output = $t;

            /// Raises a number to a power modulo another number $m$. The base must be already
            /// reduced modulo $m$.
            ///
            /// $f(x, n, m) = y$, where $x, y < m$ and $x^n \equiv y \mod m$.
            ///
            /// # Worst-case complexity
            /// $T(n) = O(n)$
            ///
            /// $M(n) = O(1)$
            ///
            /// where $T$ is time, $M$ is additional memory, and $n$ is `exp.significant_bits()`.
            ///
            /// # Panics
            /// Panics if `self` is greater than or equal to `m`.
            ///
            /// # Examples
            /// See [here](super::mod_pow#mod_pow).
            #[inline]
            fn mod_pow(self, exp: u64, m: $t) -> $t {
                simple_binary_mod_pow(self, exp, m)
            }
        }

        impl ModPowAssign<u64> for $t {
            /// Raises a number to a power modulo another number $m$, in place. The base must be
            /// already reduced modulo $m$.
            ///
            /// $x \gets y$, where $x, y < m$ and $x^n \equiv y \mod m$.
            ///
            /// # Worst-case complexity
            /// $T(n) = O(n)$
            ///
            /// $M(n) = O(1)$
            ///
            /// where $T$ is time, $M$ is additional memory, and $n$ is `exp.significant_bits()`.
            ///
            /// # Panics
            /// Panics if `self` is greater than or equal to `m`.
            ///
            /// # Examples
            /// See [here](super::mod_pow#mod_pow_assign).
            #[inline]
            fn mod_pow_assign(&mut self, exp: u64, m: $t) {
                *self = simple_binary_mod_pow(*self, exp, m);
            }
        }
    };
}
apply_to_unsigneds!(impl_mod_pow);
