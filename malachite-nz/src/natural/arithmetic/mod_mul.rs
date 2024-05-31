// Copyright © 2024 Mikhail Hogrefe
//
// Uses code adopted from the FLINT Library.
//
//      Copyright © 2019 Daniel Schultz
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::natural::arithmetic::div_mod::limbs_div_mod_by_two_limb_normalized;
use crate::natural::InnerNatural::{Large, Small};
use crate::natural::Natural;
use crate::platform::{DoubleLimb, Limb};
use malachite_base::num::arithmetic::traits::{
    ModMul, ModMulAssign, ModMulPrecomputed, ModMulPrecomputedAssign, ModPowerOf2Mul,
    ModPowerOf2MulAssign, PowerOf2, XMulYToZZ, XXXAddYYYToZZZ, XXXSubYYYToZZZ, XXXXAddYYYYToZZZZ,
};
use malachite_base::num::basic::integers::PrimitiveInt;
use malachite_base::num::basic::traits::{One, Zero};
use malachite_base::num::conversion::traits::{JoinHalves, SplitInHalf};
use malachite_base::num::logic::traits::LeadingZeros;

// m_1 cannot be zero, and we cannot have m_1 == 1 and m_0 == 0.
//
// # Worst-case complexity
// Constant time and additional memory.
//
// This is equivalent to part of `fmpz_mod_ctx_init` from `fmpz_mod/ctx_init.c`, FLINT 2.7.1.
pub_test! {limbs_precompute_mod_mul_two_limbs(m_1: Limb, m_0: Limb) -> (Limb, Limb, Limb) {
    let xs = &mut [0; 5];
    let out = &mut [0; 3];
    let bits = LeadingZeros::leading_zeros(m_1);
    if bits == 0 {
        xs[4] = 1;
        assert!(!limbs_div_mod_by_two_limb_normalized(out, xs, &[m_0, m_1]));
    } else {
        xs[4] = Limb::power_of_2(bits);
        assert!(!limbs_div_mod_by_two_limb_normalized(
            out,
            xs,
            &[m_0 << bits, (m_1 << bits) | (m_0 >> (Limb::WIDTH - bits))]
        ));
    }
    assert_ne!(out[2], 0);
    (out[2], out[1], out[0])
}}

// Standard Barrett reduction: (set r = `Limb::WIDTH`)
//
// We have m fits into 2 words and 2 ^ r < m < 2 ^ (2 * r). Therefore 2 ^ (3 * r) > 2 ^ (4 * r) / m
// > 2 ^ (2 * r) and the precomputed number inv = floor(2 ^ (4 * r) / m) fits into 3 words. The
// inputs x and y are < m and therefore fit into 2 words.
//
// The computation of a = x*y mod m is:
// ```
// w = x * y               x < m ^ 2 and therefore fits into 4 words
// z = (w >> r) * inv      z <= m * 2 ^ (3 * r) and therefore fits into 5 words
// q = (z >> (3 * r)) * n  q fits into 4 words
// w = w - q               w fits into 3 words after the subtraction
// ```
//
// at this point the canonical reduction in the range [0, m) is one of a = w, a = w - n, or a = w -
// 2 * m
//
// # Worst-case complexity
// Constant time and additional memory.
//
// This is equivalent to `_fmpz_mod_mul2` from `fmpz_mod/mul.c`, FLINT 2.7.1.
pub_test! {limbs_mod_mul_two_limbs(
    x_1: Limb,
    x_0: Limb,
    y_1: Limb,
    y_0: Limb,
    m_1: Limb,
    m_0: Limb,
    inv_2: Limb,
    inv_1: Limb,
    inv_0: Limb,
) -> (Limb, Limb) {
    // w[3:0] = x[1:0] * y[1:0]
    let (w_3, w_2) = Limb::x_mul_y_to_zz(x_1, y_1);
    let (w_1, w_0) = Limb::x_mul_y_to_zz(x_0, y_0);
    let (t, carry) = (DoubleLimb::from(x_1) * DoubleLimb::from(y_0))
        .overflowing_add(DoubleLimb::from(x_0) * DoubleLimb::from(y_1));
    let (t_2, t_1) = t.split_in_half();
    let (w_3, w_2, w_1) = Limb::xxx_add_yyy_to_zzz(w_3, w_2, w_1, Limb::from(carry), t_2, t_1);

    // z[5:0] = w[3:1] * ninv[2:0], z[5] should end up zero
    let (z_3, z_2) = Limb::x_mul_y_to_zz(w_2, inv_1);
    let (t, carry) = (DoubleLimb::from(w_1) * DoubleLimb::from(inv_2))
        .overflowing_add(DoubleLimb::from(w_3) * DoubleLimb::from(inv_0));
    let (t_3, t_2) = t.split_in_half();
    let (u_2, u_1) = Limb::x_mul_y_to_zz(w_2, inv_0);
    let (u_4, u_3) = Limb::x_mul_y_to_zz(w_3, inv_1);
    let (z_4, z_3, z_2) = Limb::xxx_add_yyy_to_zzz(
        w_3.wrapping_mul(inv_2),
        z_3,
        z_2,
        Limb::from(carry),
        t_3,
        t_2,
    );
    let (v_2, v_1) = Limb::x_mul_y_to_zz(w_1, inv_1);
    let (v_4, v_3) = Limb::x_mul_y_to_zz(w_2, inv_2);
    let (z_4, z_3, z_2, z_1) = Limb::xxxx_add_yyyy_to_zzzz(
        z_4,
        z_3,
        z_2,
        (DoubleLimb::from(w_1) * DoubleLimb::from(inv_0)).upper_half(),
        u_4,
        u_3,
        u_2,
        u_1,
    );
    let (z_4, z_3, _, _) = Limb::xxxx_add_yyyy_to_zzzz(z_4, z_3, z_2, z_1, v_4, v_3, v_2, v_1);

    // - q[3:0] = z[4:3] * n[1:0], q[3] is not needed
    // - x[3:0] -= q[3:0], w[3] should end up zero
    let (q_1, q_0) = Limb::x_mul_y_to_zz(z_3, m_0);
    let (w_2, w_1) = DoubleLimb::join_halves(w_2, w_1)
        .wrapping_sub(DoubleLimb::from(z_4) * DoubleLimb::from(m_0))
        .wrapping_sub(DoubleLimb::from(z_3) * DoubleLimb::from(m_1))
        .split_in_half();
    let (w_2, w_1, w_0) = Limb::xxx_sub_yyy_to_zzz(w_2, w_1, w_0, z_4.wrapping_mul(m_1), q_1, q_0);

    // at most two subtractions of n, use q as temp space
    let (q_2, q_1, q_0) = Limb::xxx_sub_yyy_to_zzz(w_2, w_1, w_0, 0, m_1, m_0);
    if q_2.get_highest_bit() {
        (w_1, w_0)
    } else {
        let (w_2, w_1, w_0) = Limb::xxx_sub_yyy_to_zzz(q_2, q_1, q_0, 0, m_1, m_0);
        if w_2.get_highest_bit() {
            (q_1, q_0)
        } else {
            (w_1, w_0)
        }
    }
}}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
#[doc(hidden)]
pub enum ModMulData {
    OneLimb(Limb),
    MinTwoLimbs,
    TwoLimbs(Limb, Limb, Limb),
    MoreThanTwoLimbs,
}

fn precompute_mod_mul_data_helper(m: &Natural) -> ModMulData {
    match *m {
        Natural::ZERO => panic!("division by zero"),
        Natural(Small(ref x)) => ModMulData::OneLimb(Limb::precompute_mod_mul_data(x)),
        Natural(Large(ref xs)) => match xs[..] {
            [0, 1] => ModMulData::MinTwoLimbs,
            [m_0, m_1] => {
                let (inv_2, inv_1, inv_0) = limbs_precompute_mod_mul_two_limbs(m_1, m_0);
                ModMulData::TwoLimbs(inv_2, inv_1, inv_0)
            }
            _ => ModMulData::MoreThanTwoLimbs,
        },
    }
}

impl Natural {
    fn mod_mul_precomputed_two_limbs(
        &self,
        y: &Natural,
        m: &Natural,
        inv_2: Limb,
        inv_1: Limb,
        inv_0: Limb,
    ) -> Natural {
        let (r_1, r_0) = match (self, y, m) {
            (&Natural(Small(x)), &Natural(Small(y)), &Natural(Large(ref ms))) => {
                limbs_mod_mul_two_limbs(0, x, 0, y, ms[1], ms[0], inv_2, inv_1, inv_0)
            }
            (&Natural(Large(ref xs)), &Natural(Small(y)), &Natural(Large(ref ms)))
            | (&Natural(Small(y)), &Natural(Large(ref xs)), &Natural(Large(ref ms))) => {
                limbs_mod_mul_two_limbs(xs[1], xs[0], 0, y, ms[1], ms[0], inv_2, inv_1, inv_0)
            }
            (&Natural(Large(ref xs)), &Natural(Large(ref ys)), &Natural(Large(ref ms))) => {
                limbs_mod_mul_two_limbs(
                    xs[1], xs[0], ys[1], ys[0], ms[1], ms[0], inv_2, inv_1, inv_0,
                )
            }
            _ => unreachable!(),
        };
        Natural::from_owned_limbs_asc(vec![r_0, r_1])
    }

    fn mod_mul_precomputed_two_limbs_assign(
        &mut self,
        y: &Natural,
        m: &Natural,
        inv_2: Limb,
        inv_1: Limb,
        inv_0: Limb,
    ) {
        match (&mut *self, y, m) {
            (&mut Natural(Small(x)), &Natural(Small(y)), &Natural(Large(ref ms))) => {
                let (r_1, r_0) =
                    limbs_mod_mul_two_limbs(0, x, 0, y, ms[1], ms[0], inv_2, inv_1, inv_0);
                *self = Natural::from_owned_limbs_asc(vec![r_0, r_1]);
            }
            (&mut Natural(Small(x)), &Natural(Large(ref ys)), &Natural(Large(ref ms))) => {
                let (r_1, r_0) =
                    limbs_mod_mul_two_limbs(0, x, ys[1], ys[0], ms[1], ms[0], inv_2, inv_1, inv_0);
                *self = Natural::from_owned_limbs_asc(vec![r_0, r_1]);
            }
            (&mut Natural(Large(ref mut xs)), &Natural(Small(y)), &Natural(Large(ref ms))) => {
                let (r_1, r_0) =
                    limbs_mod_mul_two_limbs(xs[1], xs[0], 0, y, ms[1], ms[0], inv_2, inv_1, inv_0);
                *xs = vec![r_0, r_1];
                self.trim();
            }
            (&mut Natural(Large(ref mut xs)), &Natural(Large(ref ys)), &Natural(Large(ref ms))) => {
                let (r_1, r_0) = limbs_mod_mul_two_limbs(
                    xs[1], xs[0], ys[1], ys[0], ms[1], ms[0], inv_2, inv_1, inv_0,
                );
                *xs = vec![r_0, r_1];
                self.trim();
            }
            _ => unreachable!(),
        }
    }
}

impl ModMulPrecomputed<Natural, Natural> for Natural {
    type Output = Natural;
    type Data = ModMulData;

    /// Precomputes data for modular multiplication. See `mod_mul_precomputed` and
    /// [`mod_mul_precomputed_assign`](ModMulPrecomputedAssign).
    ///
    /// # Worst-case complexity
    /// Constant time and additional memory.
    ///
    /// This is equivalent to part of `fmpz_mod_ctx_init` from `fmpz_mod/ctx_init.c`, FLINT 2.7.1.
    #[inline]
    fn precompute_mod_mul_data(m: &Natural) -> ModMulData {
        precompute_mod_mul_data_helper(m)
    }

    /// Multiplies two [`Natural`] modulo a third [`Natural`] $m$. The inputs must be already
    /// reduced modulo $m$. All three [`Natural`]s are taken by value.
    ///
    /// Some precomputed data is provided; this speeds up computations involving several modular
    /// multiplications with the same modulus. The precomputed data should be obtained using
    /// [`precompute_mod_mul_data`](ModMulPrecomputed::precompute_mod_mul_data).
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `m.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `self` or `other` are greater than or equal to `m`.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::ModMulPrecomputed;
    /// use malachite_nz::natural::Natural;
    ///
    /// let data = ModMulPrecomputed::<Natural>::precompute_mod_mul_data(&Natural::from(10u32));
    /// assert_eq!(
    ///     Natural::from(6u8).mod_mul_precomputed(
    ///         Natural::from(7u32),
    ///         Natural::from(10u32),
    ///         &data
    ///     ),
    ///     2
    /// );
    /// assert_eq!(
    ///     Natural::from(9u8).mod_mul_precomputed(
    ///         Natural::from(9u32),
    ///         Natural::from(10u32),
    ///         &data
    ///     ),
    ///     1
    /// );
    /// assert_eq!(
    ///     Natural::from(4u8).mod_mul_precomputed(
    ///         Natural::from(7u32),
    ///         Natural::from(10u32),
    ///         &data
    ///     ),
    ///     8
    /// );
    /// ```
    ///
    /// This is equivalent to `_fmpz_mod_mulN` from `fmpz_mod/mul.c`, FLINT 2.7.1, where `b`, `c`,
    /// and `m` are taken by value.
    fn mod_mul_precomputed(mut self, other: Natural, m: Natural, data: &ModMulData) -> Natural {
        self.mod_mul_precomputed_assign(other, m, data);
        self
    }
}

impl<'a> ModMulPrecomputed<Natural, &'a Natural> for Natural {
    type Output = Natural;
    type Data = ModMulData;

    /// Precomputes data for modular multiplication. See `mod_mul_precomputed` and
    /// [`mod_mul_precomputed_assign`](ModMulPrecomputedAssign).
    ///
    /// # Worst-case complexity
    /// Constant time and additional memory.
    ///
    /// This is equivalent to part of `fmpz_mod_ctx_init` from `fmpz_mod/ctx_init.c`, FLINT 2.7.1.
    #[inline]
    fn precompute_mod_mul_data(m: &&Natural) -> ModMulData {
        precompute_mod_mul_data_helper(m)
    }

    /// Multiplies two [`Natural`] modulo a third [`Natural`] $m$. The inputs must be already
    /// reduced modulo $m$. The first two [`Natural`]s are taken by value and the third by
    /// reference.
    ///
    /// Some precomputed data is provided; this speeds up computations involving several modular
    /// multiplications with the same modulus. The precomputed data should be obtained using
    /// [`precompute_mod_mul_data`](ModMulPrecomputed::precompute_mod_mul_data).
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `m.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `self` or `other` are greater than or equal to `m`.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::ModMulPrecomputed;
    /// use malachite_nz::natural::Natural;
    ///
    /// let data = ModMulPrecomputed::<Natural>::precompute_mod_mul_data(&Natural::from(10u32));
    /// assert_eq!(
    ///     Natural::from(6u8).mod_mul_precomputed(
    ///         Natural::from(7u32),
    ///         &Natural::from(10u32),
    ///         &data
    ///     ),
    ///     2
    /// );
    /// assert_eq!(
    ///     Natural::from(9u8).mod_mul_precomputed(
    ///         Natural::from(9u32),
    ///         &Natural::from(10u32),
    ///         &data
    ///     ),
    ///     1
    /// );
    /// assert_eq!(
    ///     Natural::from(4u8).mod_mul_precomputed(
    ///         Natural::from(7u32),
    ///         &Natural::from(10u32),
    ///         &data
    ///     ),
    ///     8
    /// );
    /// ```
    ///
    /// This is equivalent to `_fmpz_mod_mulN` from `fmpz_mod/mul.c`, FLINT 2.7.1, where `b` and `c`
    /// are taken by value and `m` is taken by reference.
    fn mod_mul_precomputed(mut self, other: Natural, m: &'a Natural, data: &ModMulData) -> Natural {
        self.mod_mul_precomputed_assign(other, m, data);
        self
    }
}

impl<'a> ModMulPrecomputed<&'a Natural, Natural> for Natural {
    type Output = Natural;
    type Data = ModMulData;

    /// Precomputes data for modular multiplication. See `mod_mul_precomputed` and
    /// [`mod_mul_precomputed_assign`](ModMulPrecomputedAssign).
    ///
    /// # Worst-case complexity
    /// Constant time and additional memory.
    ///
    /// This is equivalent to part of `fmpz_mod_ctx_init` from `fmpz_mod/ctx_init.c`, FLINT 2.7.1.
    #[inline]
    fn precompute_mod_mul_data(m: &Natural) -> ModMulData {
        precompute_mod_mul_data_helper(m)
    }

    /// Multiplies two [`Natural`] modulo a third [`Natural`] $m$. The inputs must be already
    /// reduced modulo $m$. The first and third [`Natural`]s are taken by value and the second by
    /// reference.
    ///
    /// Some precomputed data is provided; this speeds up computations involving several modular
    /// multiplications with the same modulus. The precomputed data should be obtained using
    /// [`precompute_mod_mul_data`](ModMulPrecomputed::precompute_mod_mul_data).
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `m.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `self` or `other` are greater than or equal to `m`.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::ModMulPrecomputed;
    /// use malachite_nz::natural::Natural;
    ///
    /// let data = ModMulPrecomputed::<Natural>::precompute_mod_mul_data(&Natural::from(10u32));
    /// assert_eq!(
    ///     Natural::from(6u8).mod_mul_precomputed(
    ///         &Natural::from(7u32),
    ///         Natural::from(10u32),
    ///         &data
    ///     ),
    ///     2
    /// );
    /// assert_eq!(
    ///     Natural::from(9u8).mod_mul_precomputed(
    ///         &Natural::from(9u32),
    ///         Natural::from(10u32),
    ///         &data
    ///     ),
    ///     1
    /// );
    /// assert_eq!(
    ///     Natural::from(4u8).mod_mul_precomputed(
    ///         &Natural::from(7u32),
    ///         Natural::from(10u32),
    ///         &data
    ///     ),
    ///     8
    /// );
    /// ```
    ///
    /// This is equivalent to `_fmpz_mod_mulN` from `fmpz_mod/mul.c`, FLINT 2.7.1, where `b` and `m`
    /// are taken by value and `c` is taken by reference.
    fn mod_mul_precomputed(mut self, other: &'a Natural, m: Natural, data: &ModMulData) -> Natural {
        self.mod_mul_precomputed_assign(other, m, data);
        self
    }
}

impl<'a, 'b> ModMulPrecomputed<&'a Natural, &'b Natural> for Natural {
    type Output = Natural;
    type Data = ModMulData;

    /// Precomputes data for modular multiplication. See `mod_mul_precomputed` and
    /// [`mod_mul_precomputed_assign`](ModMulPrecomputedAssign).
    ///
    /// # Worst-case complexity
    /// Constant time and additional memory.
    ///
    /// This is equivalent to part of `fmpz_mod_ctx_init` from `fmpz_mod/ctx_init.c`, FLINT 2.7.1.
    #[inline]
    fn precompute_mod_mul_data(m: &&Natural) -> ModMulData {
        precompute_mod_mul_data_helper(m)
    }

    /// Multiplies two [`Natural`] modulo a third [`Natural`] $m$. The inputs must be already
    /// reduced modulo $m$. The first [`Natural`] is taken by value and the second and third by
    /// reference.
    ///
    /// Some precomputed data is provided; this speeds up computations involving several modular
    /// multiplications with the same modulus. The precomputed data should be obtained using
    /// [`precompute_mod_mul_data`](ModMulPrecomputed::precompute_mod_mul_data).
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `m.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `self` or `other` are greater than or equal to `m`.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::ModMulPrecomputed;
    /// use malachite_nz::natural::Natural;
    ///
    /// let data = ModMulPrecomputed::<Natural>::precompute_mod_mul_data(&Natural::from(10u32));
    /// assert_eq!(
    ///     Natural::from(6u8).mod_mul_precomputed(
    ///         &Natural::from(7u32),
    ///         &Natural::from(10u32),
    ///         &data
    ///     ),
    ///     2
    /// );
    /// assert_eq!(
    ///     Natural::from(9u8).mod_mul_precomputed(
    ///         &Natural::from(9u32),
    ///         &Natural::from(10u32),
    ///         &data
    ///     ),
    ///     1
    /// );
    /// assert_eq!(
    ///     Natural::from(4u8).mod_mul_precomputed(
    ///         &Natural::from(7u32),
    ///         &Natural::from(10u32),
    ///         &data
    ///     ),
    ///     8
    /// );
    /// ```
    ///
    /// This is equivalent to `_fmpz_mod_mulN` from `fmpz_mod/mul.c`, FLINT 2.7.1, where `b` is
    /// taken by value and `c` and `m` are taken by reference.
    fn mod_mul_precomputed(
        mut self,
        other: &'a Natural,
        m: &'b Natural,
        data: &ModMulData,
    ) -> Natural {
        self.mod_mul_precomputed_assign(other, m, data);
        self
    }
}

impl<'a> ModMulPrecomputed<Natural, Natural> for &'a Natural {
    type Output = Natural;
    type Data = ModMulData;

    /// Precomputes data for modular multiplication. See `mod_mul_precomputed` and
    /// [`mod_mul_precomputed_assign`](ModMulPrecomputedAssign).
    ///
    /// # Worst-case complexity
    /// Constant time and additional memory.
    ///
    /// This is equivalent to part of `fmpz_mod_ctx_init` from `fmpz_mod/ctx_init.c`, FLINT 2.7.1.
    #[inline]
    fn precompute_mod_mul_data(m: &Natural) -> ModMulData {
        precompute_mod_mul_data_helper(m)
    }

    /// Multiplies two [`Natural`] modulo a third [`Natural`] $m$. The inputs must be already
    /// reduced modulo $m$. The first [`Natural`] is taken by reference and the second and third by
    /// value.
    ///
    /// Some precomputed data is provided; this speeds up computations involving several modular
    /// multiplications with the same modulus. The precomputed data should be obtained using
    /// [`precompute_mod_mul_data`](ModMulPrecomputed::precompute_mod_mul_data).
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `m.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `self` or `other` are greater than or equal to `m`.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::ModMulPrecomputed;
    /// use malachite_nz::natural::Natural;
    ///
    /// let data = ModMulPrecomputed::<Natural>::precompute_mod_mul_data(&Natural::from(10u32));
    /// assert_eq!(
    ///     (&Natural::from(6u8)).mod_mul_precomputed(
    ///         Natural::from(7u32),
    ///         Natural::from(10u32),
    ///         &data
    ///     ),
    ///     2
    /// );
    /// assert_eq!(
    ///     (&Natural::from(9u8)).mod_mul_precomputed(
    ///         Natural::from(9u32),
    ///         Natural::from(10u32),
    ///         &data
    ///     ),
    ///     1
    /// );
    /// assert_eq!(
    ///     (&Natural::from(4u8)).mod_mul_precomputed(
    ///         Natural::from(7u32),
    ///         Natural::from(10u32),
    ///         &data
    ///     ),
    ///     8
    /// );
    /// ```
    ///
    /// This is equivalent to `_fmpz_mod_mulN` from `fmpz_mod/mul.c`, FLINT 2.7.1, where `b` is
    /// taken by reference and `c` and `m` are taken by value.
    fn mod_mul_precomputed(self, other: Natural, m: Natural, data: &ModMulData) -> Natural {
        assert!(*self < m, "self must be reduced mod m, but {self} >= {m}");
        assert!(other < m, "other must be reduced mod m, but {other} >= {m}");
        match (self, other, m, data) {
            (&Natural::ZERO, _, _, _) | (_, Natural::ZERO, _, _) => Natural::ZERO,
            (x, Natural::ONE, _, _) => x.clone(),
            (&Natural::ONE, y, _, _) => y,
            (
                &Natural(Small(x)),
                Natural(Small(y)),
                Natural(Small(m)),
                &ModMulData::OneLimb(inv),
            ) => Natural::from(x.mod_mul_precomputed(y, m, &inv)),
            (x, y, _, &ModMulData::MinTwoLimbs) => x.mod_power_of_2_mul(y, Limb::WIDTH),
            (x, y, m, &ModMulData::TwoLimbs(inv_2, inv_1, inv_0)) => {
                x.mod_mul_precomputed_two_limbs(&y, &m, inv_2, inv_1, inv_0)
            }
            (x, y, m, _) => x * y % m,
        }
    }
}

impl<'a, 'b> ModMulPrecomputed<Natural, &'b Natural> for &'a Natural {
    type Output = Natural;
    type Data = ModMulData;

    /// Precomputes data for modular multiplication. See `mod_mul_precomputed` and
    /// [`mod_mul_precomputed_assign`](ModMulPrecomputedAssign).
    ///
    /// # Worst-case complexity
    /// Constant time and additional memory.
    ///
    /// This is equivalent to part of `fmpz_mod_ctx_init` from `fmpz_mod/ctx_init.c`, FLINT 2.7.1.
    #[inline]
    fn precompute_mod_mul_data(m: &&Natural) -> ModMulData {
        precompute_mod_mul_data_helper(m)
    }

    /// Multiplies two [`Natural`] modulo a third [`Natural`] $m$. The inputs must be already
    /// reduced modulo $m$. The first and third [`Natural`]s are taken by reference and the second
    /// by value.
    ///
    /// Some precomputed data is provided; this speeds up computations involving several modular
    /// multiplications with the same modulus. The precomputed data should be obtained using
    /// [`precompute_mod_mul_data`](ModMulPrecomputed::precompute_mod_mul_data).
    ///
    /// # Panics
    /// Panics if `self` or `other` are greater than or equal to `m`.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `m.significant_bits()`.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::ModMulPrecomputed;
    /// use malachite_nz::natural::Natural;
    ///
    /// let data = ModMulPrecomputed::<Natural>::precompute_mod_mul_data(&Natural::from(10u32));
    /// assert_eq!(
    ///     (&Natural::from(6u8)).mod_mul_precomputed(
    ///         Natural::from(7u32),
    ///         &Natural::from(10u32),
    ///         &data
    ///     ),
    ///     2
    /// );
    /// assert_eq!(
    ///     (&Natural::from(9u8)).mod_mul_precomputed(
    ///         Natural::from(9u32),
    ///         &Natural::from(10u32),
    ///         &data
    ///     ),
    ///     1
    /// );
    /// assert_eq!(
    ///     (&Natural::from(4u8)).mod_mul_precomputed(
    ///         Natural::from(7u32),
    ///         &Natural::from(10u32),
    ///         &data
    ///     ),
    ///     8
    /// );
    /// ```
    ///
    /// This is equivalent to `_fmpz_mod_mulN` from `fmpz_mod/mul.c`, FLINT 2.7.1, where `b` and `m`
    /// are taken by reference and `c` is taken by value.
    #[inline]
    fn mod_mul_precomputed(self, other: Natural, m: &'b Natural, data: &ModMulData) -> Natural {
        other.mod_mul_precomputed(self, m, data)
    }
}

impl<'a, 'b> ModMulPrecomputed<&'b Natural, Natural> for &'a Natural {
    type Output = Natural;
    type Data = ModMulData;

    /// Precomputes data for modular multiplication. See `mod_mul_precomputed` and
    /// [`mod_mul_precomputed_assign`](ModMulPrecomputedAssign).
    ///
    /// # Worst-case complexity
    /// Constant time and additional memory.
    ///
    /// This is equivalent to part of `fmpz_mod_ctx_init` from `fmpz_mod/ctx_init.c`, FLINT 2.7.1.
    #[inline]
    fn precompute_mod_mul_data(m: &Natural) -> ModMulData {
        precompute_mod_mul_data_helper(m)
    }

    /// Multiplies two [`Natural`] modulo a third [`Natural`] $m$. The inputs must be already
    /// reduced modulo $m$. The first two [`Natural`]s are taken by reference and the third by
    /// value.
    ///
    /// Some precomputed data is provided; this speeds up computations involving several modular
    /// multiplications with the same modulus. The precomputed data should be obtained using
    /// [`precompute_mod_mul_data`](ModMulPrecomputed::precompute_mod_mul_data).
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `m.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `self` or `other` are greater than or equal to `m`.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::ModMulPrecomputed;
    /// use malachite_nz::natural::Natural;
    ///
    /// let data = ModMulPrecomputed::<Natural>::precompute_mod_mul_data(&Natural::from(10u32));
    /// assert_eq!(
    ///     (&Natural::from(6u8)).mod_mul_precomputed(
    ///         &Natural::from(7u32),
    ///         Natural::from(10u32),
    ///         &data
    ///     ),
    ///     2
    /// );
    /// assert_eq!(
    ///     (&Natural::from(9u8)).mod_mul_precomputed(
    ///         &Natural::from(9u32),
    ///         Natural::from(10u32),
    ///         &data
    ///     ),
    ///     1
    /// );
    /// assert_eq!(
    ///     (&Natural::from(4u8)).mod_mul_precomputed(
    ///         &Natural::from(7u32),
    ///         Natural::from(10u32),
    ///         &data
    ///     ),
    ///     8
    /// );
    /// ```
    ///
    /// This is equivalent to `_fmpz_mod_mulN` from `fmpz_mod/mul.c`, FLINT 2.7.1, where `b` and `c`
    /// are taken by reference and `m` is taken by value.
    fn mod_mul_precomputed(self, other: &'b Natural, m: Natural, data: &ModMulData) -> Natural {
        assert!(*self < m, "self must be reduced mod m, but {self} >= {m}");
        assert!(
            *other < m,
            "other must be reduced mod m, but {other} >= {m}"
        );
        match (self, other, m, data) {
            (&Natural::ZERO, _, _, _) | (_, &Natural::ZERO, _, _) => Natural::ZERO,
            (x, &Natural::ONE, _, _) => x.clone(),
            (&Natural::ONE, y, _, _) => y.clone(),
            (
                &Natural(Small(x)),
                &Natural(Small(y)),
                Natural(Small(m)),
                &ModMulData::OneLimb(inv),
            ) => Natural::from(x.mod_mul_precomputed(y, m, &inv)),
            (x, y, _, &ModMulData::MinTwoLimbs) => x.mod_power_of_2_mul(y, Limb::WIDTH),
            (x, y, m, &ModMulData::TwoLimbs(inv_2, inv_1, inv_0)) => {
                x.mod_mul_precomputed_two_limbs(y, &m, inv_2, inv_1, inv_0)
            }
            (x, y, m, _) => x * y % m,
        }
    }
}

impl<'a, 'b, 'c> ModMulPrecomputed<&'b Natural, &'c Natural> for &'a Natural {
    type Output = Natural;
    type Data = ModMulData;

    /// Precomputes data for modular multiplication. See `mod_mul_precomputed` and
    /// [`mod_mul_precomputed_assign`](ModMulPrecomputedAssign).
    ///
    /// # Worst-case complexity
    /// Constant time and additional memory.
    ///
    /// This is equivalent to part of `fmpz_mod_ctx_init` from `fmpz_mod/ctx_init.c`, FLINT 2.7.1.
    #[inline]
    fn precompute_mod_mul_data(m: &&Natural) -> ModMulData {
        precompute_mod_mul_data_helper(m)
    }

    /// Multiplies two [`Natural`] modulo a third [`Natural`] $m$. The inputs must be already
    /// reduced modulo $m$. All three [`Natural`]s are taken by reference.
    ///
    /// Some precomputed data is provided; this speeds up computations involving several modular
    /// multiplications with the same modulus. The precomputed data should be obtained using
    /// [`precompute_mod_mul_data`](ModMulPrecomputed::precompute_mod_mul_data).
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `m.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `self` or `other` are greater than or equal to `m`.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::ModMulPrecomputed;
    /// use malachite_nz::natural::Natural;
    ///
    /// let data = ModMulPrecomputed::<Natural>::precompute_mod_mul_data(&Natural::from(10u32));
    /// assert_eq!(
    ///     (&Natural::from(6u8)).mod_mul_precomputed(
    ///         &Natural::from(7u32),
    ///         &Natural::from(10u32),
    ///         &data
    ///     ),
    ///     2
    /// );
    /// assert_eq!(
    ///     (&Natural::from(9u8)).mod_mul_precomputed(
    ///         &Natural::from(9u32),
    ///         &Natural::from(10u32),
    ///         &data
    ///     ),
    ///     1
    /// );
    /// assert_eq!(
    ///     (&Natural::from(4u8)).mod_mul_precomputed(
    ///         &Natural::from(7u32),
    ///         &Natural::from(10u32),
    ///         &data
    ///     ),
    ///     8
    /// );
    /// ```
    ///
    /// This is equivalent to `_fmpz_mod_mulN` from fmpz_mod/mul.c, FLINT 2.7.1, where `b`, `c`, and
    /// `m` are taken by reference.
    fn mod_mul_precomputed(self, other: &'b Natural, m: &'c Natural, data: &ModMulData) -> Natural {
        assert!(self < m, "self must be reduced mod m, but {self} >= {m}");
        assert!(other < m, "other must be reduced mod m, but {other} >= {m}");
        match (self, other, m, data) {
            (&Natural::ZERO, _, _, _) | (_, &Natural::ZERO, _, _) => Natural::ZERO,
            (x, &Natural::ONE, _, _) => x.clone(),
            (&Natural::ONE, y, _, _) => y.clone(),
            (
                &Natural(Small(x)),
                &Natural(Small(y)),
                &Natural(Small(m)),
                &ModMulData::OneLimb(inv),
            ) => Natural::from(x.mod_mul_precomputed(y, m, &inv)),
            (x, y, _, &ModMulData::MinTwoLimbs) => x.mod_power_of_2_mul(y, Limb::WIDTH),
            (x, y, m, &ModMulData::TwoLimbs(inv_2, inv_1, inv_0)) => {
                x.mod_mul_precomputed_two_limbs(y, m, inv_2, inv_1, inv_0)
            }
            (x, y, m, _) => x * y % m,
        }
    }
}

impl ModMulPrecomputedAssign<Natural, Natural> for Natural {
    /// Multiplies two [`Natural`]s modulo a third [`Natural`] $m$, in place. The inputs must be
    /// already reduced modulo $m$. Both [`Natural`]s on the right-hand side are taken by value.
    ///
    /// Some precomputed data is provided; this speeds up computations involving several modular
    /// multiplications with the same modulus. The precomputed data should be obtained using
    /// [`precompute_mod_mul_data`](ModMulPrecomputed::precompute_mod_mul_data).
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `m.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `self` or `other` are greater than or equal to `m`.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::{ModMulPrecomputed, ModMulPrecomputedAssign};
    /// use malachite_nz::natural::Natural;
    ///
    /// let data = ModMulPrecomputed::<Natural>::precompute_mod_mul_data(&Natural::from(10u32));
    ///
    /// let mut x = Natural::from(6u8);
    /// x.mod_mul_precomputed_assign(Natural::from(7u32), Natural::from(10u32), &data);
    /// assert_eq!(x, 2);
    ///
    /// let mut x = Natural::from(9u8);
    /// x.mod_mul_precomputed_assign(Natural::from(9u32), Natural::from(10u32), &data);
    /// assert_eq!(x, 1);
    ///
    /// let mut x = Natural::from(4u8);
    /// x.mod_mul_precomputed_assign(Natural::from(7u32), Natural::from(10u32), &data);
    /// assert_eq!(x, 8);
    /// ```
    ///
    /// This is equivalent to `_fmpz_mod_mulN` from `fmpz_mod/mul.c`, FLINT 2.7.1, where `b`, `c`,
    /// and `m` are taken by value and `a == b`.
    fn mod_mul_precomputed_assign(&mut self, other: Natural, m: Natural, data: &ModMulData) {
        assert!(*self < m, "self must be reduced mod m, but {self} >= {m}");
        assert!(other < m, "other must be reduced mod m, but {other} >= {m}");
        match (&mut *self, other, m, data) {
            (&mut Natural::ZERO, _, _, _) | (_, Natural::ONE, _, _) => {}
            (x, Natural::ZERO, _, _) => *x = Natural::ZERO,
            (&mut Natural::ONE, y, _, _) => *self = y,
            (
                &mut Natural(Small(x)),
                Natural(Small(y)),
                Natural(Small(m)),
                &ModMulData::OneLimb(inv),
            ) => *self = Natural::from(x.mod_mul_precomputed(y, m, &inv)),
            (x, y, _, &ModMulData::MinTwoLimbs) => x.mod_power_of_2_mul_assign(y, Limb::WIDTH),
            (x, y, m, &ModMulData::TwoLimbs(inv_2, inv_1, inv_0)) => {
                x.mod_mul_precomputed_two_limbs_assign(&y, &m, inv_2, inv_1, inv_0);
            }
            (x, y, m, _) => {
                *x *= y;
                *x %= m;
            }
        }
    }
}

impl<'a> ModMulPrecomputedAssign<Natural, &'a Natural> for Natural {
    /// Multiplies two [`Natural`]s modulo a third [`Natural`] $m$, in place. The inputs must be
    /// already reduced modulo $m$. The first [`Natural`] on the right-hand side is taken by value
    /// and the second by reference.
    ///
    /// Some precomputed data is provided; this speeds up computations involving several modular
    /// multiplications with the same modulus. The precomputed data should be obtained using
    /// [`precompute_mod_mul_data`](ModMulPrecomputed::precompute_mod_mul_data).
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `m.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `self` or `other` are greater than or equal to `m`.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::{ModMulPrecomputed, ModMulPrecomputedAssign};
    /// use malachite_nz::natural::Natural;
    ///
    /// let data = ModMulPrecomputed::<Natural>::precompute_mod_mul_data(&Natural::from(10u32));
    ///
    /// let mut x = Natural::from(6u8);
    /// x.mod_mul_precomputed_assign(Natural::from(7u32), &Natural::from(10u32), &data);
    /// assert_eq!(x, 2);
    ///
    /// let mut x = Natural::from(9u8);
    /// x.mod_mul_precomputed_assign(Natural::from(9u32), &Natural::from(10u32), &data);
    /// assert_eq!(x, 1);
    ///
    /// let mut x = Natural::from(4u8);
    /// x.mod_mul_precomputed_assign(Natural::from(7u32), &Natural::from(10u32), &data);
    /// assert_eq!(x, 8);
    /// ```
    ///
    /// This is equivalent to `_fmpz_mod_mulN` from `fmpz_mod/mul.c`, FLINT 2.7.1, where `b` and `c`
    /// are taken by value, `m` is taken by reference, and `a == b`.
    fn mod_mul_precomputed_assign(&mut self, other: Natural, m: &'a Natural, data: &ModMulData) {
        assert!(&*self < m, "self must be reduced mod m, but {self} >= {m}");
        assert!(
            other < *m,
            "other must be reduced mod m, but {other} >= {m}"
        );
        match (&mut *self, other, m, data) {
            (&mut Natural::ZERO, _, _, _) | (_, Natural::ONE, _, _) => {}
            (x, Natural::ZERO, _, _) => *x = Natural::ZERO,
            (&mut Natural::ONE, y, _, _) => *self = y,
            (
                &mut Natural(Small(x)),
                Natural(Small(y)),
                &Natural(Small(m)),
                &ModMulData::OneLimb(inv),
            ) => *self = Natural::from(x.mod_mul_precomputed(y, m, &inv)),
            (x, y, _, &ModMulData::MinTwoLimbs) => x.mod_power_of_2_mul_assign(y, Limb::WIDTH),
            (x, y, m, &ModMulData::TwoLimbs(inv_2, inv_1, inv_0)) => {
                x.mod_mul_precomputed_two_limbs_assign(&y, m, inv_2, inv_1, inv_0);
            }
            (x, y, m, _) => {
                *x *= y;
                *x %= m;
            }
        }
    }
}

impl<'a> ModMulPrecomputedAssign<&'a Natural, Natural> for Natural {
    /// Multiplies two [`Natural`]s modulo a third [`Natural`] $m$, in place. The inputs must be
    /// already reduced modulo $m$. The first [`Natural`] on the right-hand side is taken by
    /// reference and the second by value.
    ///
    /// Some precomputed data is provided; this speeds up computations involving several modular
    /// multiplications with the same modulus. The precomputed data should be obtained using
    /// [`precompute_mod_mul_data`](ModMulPrecomputed::precompute_mod_mul_data).
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `m.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `self` or `other` are greater than or equal to `m`.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::{ModMulPrecomputed, ModMulPrecomputedAssign};
    /// use malachite_nz::natural::Natural;
    ///
    /// let data = ModMulPrecomputed::<Natural>::precompute_mod_mul_data(&Natural::from(10u32));
    ///
    /// let mut x = Natural::from(6u8);
    /// x.mod_mul_precomputed_assign(&Natural::from(7u32), Natural::from(10u32), &data);
    /// assert_eq!(x, 2);
    ///
    /// let mut x = Natural::from(9u8);
    /// x.mod_mul_precomputed_assign(&Natural::from(9u32), Natural::from(10u32), &data);
    /// assert_eq!(x, 1);
    ///
    /// let mut x = Natural::from(4u8);
    /// x.mod_mul_precomputed_assign(&Natural::from(7u32), Natural::from(10u32), &data);
    /// assert_eq!(x, 8);
    /// ```
    ///
    /// This is equivalent to `_fmpz_mod_mulN` from `fmpz_mod/mul.c`, FLINT 2.7.1, where `b` and `m`
    /// are taken by value, `c` is taken by reference, and `a == b`.
    fn mod_mul_precomputed_assign(&mut self, other: &'a Natural, m: Natural, data: &ModMulData) {
        assert!(*self < m, "self must be reduced mod m, but {self} >= {m}");
        assert!(
            *other < m,
            "other must be reduced mod m, but {other} >= {m}"
        );
        match (&mut *self, other, m, data) {
            (&mut Natural::ZERO, _, _, _) | (_, &Natural::ONE, _, _) => {}
            (x, &Natural::ZERO, _, _) => *x = Natural::ZERO,
            (&mut Natural::ONE, y, _, _) => *self = y.clone(),
            (
                &mut Natural(Small(x)),
                &Natural(Small(y)),
                Natural(Small(m)),
                &ModMulData::OneLimb(inv),
            ) => *self = Natural::from(x.mod_mul_precomputed(y, m, &inv)),
            (x, y, _, &ModMulData::MinTwoLimbs) => x.mod_power_of_2_mul_assign(y, Limb::WIDTH),
            (x, y, m, &ModMulData::TwoLimbs(inv_2, inv_1, inv_0)) => {
                x.mod_mul_precomputed_two_limbs_assign(y, &m, inv_2, inv_1, inv_0);
            }
            (x, y, m, _) => {
                *x *= y;
                *x %= m;
            }
        }
    }
}

impl<'a, 'b> ModMulPrecomputedAssign<&'a Natural, &'b Natural> for Natural {
    /// Multiplies two [`Natural`]s modulo a third [`Natural`] $m$, in place. The inputs must be
    /// already reduced modulo $m$. Both [`Natural`]s on the right-hand side are taken by reference.
    ///
    /// Some precomputed data is provided; this speeds up computations involving several modular
    /// multiplications with the same modulus. The precomputed data should be obtained using
    /// [`precompute_mod_mul_data`](ModMulPrecomputed::precompute_mod_mul_data).
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `m.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `self` or `other` are greater than or equal to `m`.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::{ModMulPrecomputed, ModMulPrecomputedAssign};
    /// use malachite_nz::natural::Natural;
    ///
    /// let data = ModMulPrecomputed::<Natural>::precompute_mod_mul_data(&Natural::from(10u32));
    ///
    /// let mut x = Natural::from(6u8);
    /// x.mod_mul_precomputed_assign(&Natural::from(7u32), &Natural::from(10u32), &data);
    /// assert_eq!(x, 2);
    ///
    /// let mut x = Natural::from(9u8);
    /// x.mod_mul_precomputed_assign(&Natural::from(9u32), &Natural::from(10u32), &data);
    /// assert_eq!(x, 1);
    ///
    /// let mut x = Natural::from(4u8);
    /// x.mod_mul_precomputed_assign(&Natural::from(7u32), &Natural::from(10u32), &data);
    /// assert_eq!(x, 8);
    /// ```
    ///
    /// This is equivalent to `_fmpz_mod_mulN` from `fmpz_mod/mul.c`, FLINT 2.7.1, where `b` is
    /// taken by value, `c` and `m` are taken by reference, and `a == b`.
    fn mod_mul_precomputed_assign(
        &mut self,
        other: &'a Natural,
        m: &'b Natural,
        data: &ModMulData,
    ) {
        assert!(&*self < m, "self must be reduced mod m, but {self} >= {m}");
        assert!(other < m, "other must be reduced mod m, but {other} >= {m}");
        match (&mut *self, other, m, data) {
            (&mut Natural::ZERO, _, _, _) | (_, &Natural::ONE, _, _) => {}
            (x, &Natural::ZERO, _, _) => *x = Natural::ZERO,
            (&mut Natural::ONE, y, _, _) => *self = y.clone(),
            (
                &mut Natural(Small(x)),
                &Natural(Small(y)),
                &Natural(Small(m)),
                &ModMulData::OneLimb(inv),
            ) => *self = Natural::from(x.mod_mul_precomputed(y, m, &inv)),
            (x, y, _, &ModMulData::MinTwoLimbs) => x.mod_power_of_2_mul_assign(y, Limb::WIDTH),
            (x, y, m, &ModMulData::TwoLimbs(inv_2, inv_1, inv_0)) => {
                x.mod_mul_precomputed_two_limbs_assign(y, m, inv_2, inv_1, inv_0);
            }
            (x, y, m, _) => {
                *x *= y;
                *x %= m;
            }
        }
    }
}

impl ModMul<Natural, Natural> for Natural {
    type Output = Natural;

    /// Multiplies two [`Natural`]s modulo a third [`Natural`] $m$. The inputs must be already
    /// reduced modulo $m$. All three [`Natural`]s are taken by value.
    ///
    /// $f(x, y, m) = z$, where $x, y, z < m$ and $xy \equiv z \mod m$.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `m.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `self` or `other` are greater than or equal to `m`.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::ModMul;
    /// use malachite_nz::natural::Natural;
    ///
    /// assert_eq!(
    ///     Natural::from(3u32).mod_mul(Natural::from(4u32), Natural::from(15u32)),
    ///     12
    /// );
    /// assert_eq!(
    ///     Natural::from(7u32).mod_mul(Natural::from(6u32), Natural::from(10u32)),
    ///     2
    /// );
    /// ```
    ///
    /// This is equivalent to `_fmpz_mod_mulN` from `fmpz_mod/mul.c`, FLINT 2.7.1, where `b`, `c`,
    /// and `m` are taken by value.
    #[inline]
    fn mod_mul(self, other: Natural, m: Natural) -> Natural {
        let data = precompute_mod_mul_data_helper(&m);
        self.mod_mul_precomputed(other, m, &data)
    }
}

impl<'a> ModMul<Natural, &'a Natural> for Natural {
    type Output = Natural;

    /// Multiplies two [`Natural`]s modulo a third [`Natural`] $m$. The inputs must be already
    /// reduced modulo $m$. The first two [`Natural`]s are taken by value and the third by
    /// reference.
    ///
    /// $f(x, y, m) = z$, where $x, y, z < m$ and $xy \equiv z \mod m$.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `m.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `self` or `other` are greater than or equal to `m`.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::ModMul;
    /// use malachite_nz::natural::Natural;
    ///
    /// assert_eq!(
    ///     Natural::from(3u32).mod_mul(Natural::from(4u32), &Natural::from(15u32)),
    ///     12
    /// );
    /// assert_eq!(
    ///     Natural::from(7u32).mod_mul(Natural::from(6u32), &Natural::from(10u32)),
    ///     2
    /// );
    /// ```
    ///
    /// This is equivalent to `_fmpz_mod_mulN` from `fmpz_mod/mul.c`, FLINT 2.7.1, where `b` and `c`
    /// are taken by value and `m` is taken by reference.
    #[inline]
    fn mod_mul(self, other: Natural, m: &'a Natural) -> Natural {
        self.mod_mul_precomputed(other, m, &precompute_mod_mul_data_helper(m))
    }
}

impl<'a> ModMul<&'a Natural, Natural> for Natural {
    type Output = Natural;

    /// Multiplies two [`Natural`]s modulo a third [`Natural`] $m$. The inputs must be already
    /// reduced modulo $m$. The first and third [`Natural`]s are taken by value and the second by
    /// reference.
    ///
    /// $f(x, y, m) = z$, where $x, y, z < m$ and $xy \equiv z \mod m$.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `m.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `self` or `other` are greater than or equal to `m`.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::ModMul;
    /// use malachite_nz::natural::Natural;
    ///
    /// assert_eq!(
    ///     Natural::from(3u32).mod_mul(&Natural::from(4u32), Natural::from(15u32)),
    ///     12
    /// );
    /// assert_eq!(
    ///     Natural::from(7u32).mod_mul(&Natural::from(6u32), Natural::from(10u32)),
    ///     2
    /// );
    /// ```
    ///
    /// This is equivalent to `_fmpz_mod_mulN` from `fmpz_mod/mul.c`, FLINT 2.7.1, where `b` and `m`
    /// are taken by value and `c` is taken by reference.
    #[inline]
    fn mod_mul(self, other: &'a Natural, m: Natural) -> Natural {
        let data = precompute_mod_mul_data_helper(&m);
        self.mod_mul_precomputed(other, m, &data)
    }
}

impl<'a, 'b> ModMul<&'a Natural, &'b Natural> for Natural {
    type Output = Natural;

    /// Multiplies two [`Natural`]s modulo a third [`Natural`] $m$. The inputs must be already
    /// reduced modulo $m$. The first [`Natural`] is taken by value and the second and third by
    /// reference.
    ///
    /// $f(x, y, m) = z$, where $x, y, z < m$ and $xy \equiv z \mod m$.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `m.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `self` or `other` are greater than or equal to `m`.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::ModMul;
    /// use malachite_nz::natural::Natural;
    ///
    /// assert_eq!(
    ///     Natural::from(3u32).mod_mul(&Natural::from(4u32), &Natural::from(15u32)),
    ///     12
    /// );
    /// assert_eq!(
    ///     Natural::from(7u32).mod_mul(&Natural::from(6u32), &Natural::from(10u32)),
    ///     2
    /// );
    /// ```
    ///
    /// This is equivalent to `_fmpz_mod_mulN` from `fmpz_mod/mul.c`, FLINT 2.7.1, where `b` is
    /// taken by value and `c` and `m` are taken by reference.
    #[inline]
    fn mod_mul(self, other: &'a Natural, m: &'b Natural) -> Natural {
        self.mod_mul_precomputed(other, m, &precompute_mod_mul_data_helper(m))
    }
}

impl<'a> ModMul<Natural, Natural> for &'a Natural {
    type Output = Natural;

    /// Multiplies two [`Natural`]s modulo a third [`Natural`] $m$. The inputs must be already
    /// reduced modulo $m$. The first [`Natural`] is taken by reference and the second and third by
    /// value.
    ///
    /// $f(x, y, m) = z$, where $x, y, z < m$ and $xy \equiv z \mod m$.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `m.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `self` or `other` are greater than or equal to `m`.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::ModMul;
    /// use malachite_nz::natural::Natural;
    ///
    /// assert_eq!(
    ///     (&Natural::from(3u32)).mod_mul(Natural::from(4u32), Natural::from(15u32)),
    ///     12
    /// );
    /// assert_eq!(
    ///     (&Natural::from(7u32)).mod_mul(Natural::from(6u32), Natural::from(10u32)),
    ///     2
    /// );
    /// ```
    ///
    /// This is equivalent to `_fmpz_mod_mulN` from `fmpz_mod/mul.c`, FLINT 2.7.1, where `b` is
    /// taken by reference and `c` and `m` are taken by value.
    #[inline]
    fn mod_mul(self, other: Natural, m: Natural) -> Natural {
        let data = precompute_mod_mul_data_helper(&m);
        self.mod_mul_precomputed(other, m, &data)
    }
}

impl<'a, 'b> ModMul<Natural, &'b Natural> for &'a Natural {
    type Output = Natural;

    /// Multiplies two [`Natural`]s modulo a third [`Natural`] $m$. The inputs must be already
    /// reduced modulo $m$. The first and third [`Natural`]s are taken by reference and the second
    /// by value.
    ///
    /// $f(x, y, m) = z$, where $x, y, z < m$ and $xy \equiv z \mod m$.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `m.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `self` or `other` are greater than or equal to `m`.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::ModMul;
    /// use malachite_nz::natural::Natural;
    ///
    /// assert_eq!(
    ///     (&Natural::from(3u32)).mod_mul(Natural::from(4u32), &Natural::from(15u32)),
    ///     12
    /// );
    /// assert_eq!(
    ///     (&Natural::from(7u32)).mod_mul(Natural::from(6u32), &Natural::from(10u32)),
    ///     2
    /// );
    /// ```
    ///
    /// This is equivalent to `_fmpz_mod_mulN` from `fmpz_mod/mul.c`, FLINT 2.7.1, where `b` and `m`
    /// are taken by reference and `c` is taken by value.
    #[inline]
    fn mod_mul(self, other: Natural, m: &'b Natural) -> Natural {
        self.mod_mul_precomputed(other, m, &precompute_mod_mul_data_helper(m))
    }
}

impl<'a, 'b> ModMul<&'b Natural, Natural> for &'a Natural {
    type Output = Natural;

    /// Multiplies two [`Natural`]s modulo a third [`Natural`] $m$. The inputs must be already
    /// reduced modulo $m$. The first two [`Natural`]s are taken by reference and the third by
    /// value.
    ///
    /// $f(x, y, m) = z$, where $x, y, z < m$ and $xy \equiv z \mod m$.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `m.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `self` or `other` are greater than or equal to `m`.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::ModMul;
    /// use malachite_nz::natural::Natural;
    ///
    /// assert_eq!(
    ///     (&Natural::from(3u32)).mod_mul(&Natural::from(4u32), Natural::from(15u32)),
    ///     12
    /// );
    /// assert_eq!(
    ///     (&Natural::from(7u32)).mod_mul(&Natural::from(6u32), Natural::from(10u32)),
    ///     2
    /// );
    /// ```
    ///
    /// This is equivalent to `_fmpz_mod_mulN` from `fmpz_mod/mul.c`, FLINT 2.7.1, where `b` and `c`
    /// are taken by reference and `m` is taken by value.
    #[inline]
    fn mod_mul(self, other: &'b Natural, m: Natural) -> Natural {
        let data = precompute_mod_mul_data_helper(&m);
        self.mod_mul_precomputed(other, m, &data)
    }
}

impl<'a, 'b, 'c> ModMul<&'b Natural, &'c Natural> for &'a Natural {
    type Output = Natural;

    /// Multiplies two [`Natural`]s modulo a third [`Natural`] $m$. The inputs must be already
    /// reduced modulo $m$. All three [`Natural`]s are taken by reference.
    ///
    /// $f(x, y, m) = z$, where $x, y, z < m$ and $xy \equiv z \mod m$.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `m.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `self` or `other` are greater than or equal to `m`.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::ModMul;
    /// use malachite_nz::natural::Natural;
    ///
    /// assert_eq!(
    ///     (&Natural::from(3u32)).mod_mul(&Natural::from(4u32), &Natural::from(15u32)),
    ///     12
    /// );
    /// assert_eq!(
    ///     (&Natural::from(7u32)).mod_mul(&Natural::from(6u32), &Natural::from(10u32)),
    ///     2
    /// );
    /// ```
    ///
    /// This is equivalent to `_fmpz_mod_mulN` from `fmpz_mod/mul.c`, FLINT 2.7.1, where `b`, `c`,
    /// and `m` are taken by reference.
    #[inline]
    fn mod_mul(self, other: &'b Natural, m: &'c Natural) -> Natural {
        self.mod_mul_precomputed(other, m, &precompute_mod_mul_data_helper(m))
    }
}

impl ModMulAssign<Natural, Natural> for Natural {
    /// Multiplies two [`Natural`]s modulo a third [`Natural`] $m$, in place. The inputs must be
    /// already reduced modulo $m$. Both [`Natural`]s on the right-hand side are taken by value.
    ///
    /// $x \gets z$, where $x, y, z < m$ and $xy \equiv z \mod m$.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `m.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `self` or `other` are greater than or equal to `m`.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::ModMulAssign;
    /// use malachite_nz::natural::Natural;
    ///
    /// let mut x = Natural::from(3u32);
    /// x.mod_mul_assign(Natural::from(4u32), Natural::from(15u32));
    /// assert_eq!(x, 12);
    ///
    /// let mut x = Natural::from(7u32);
    /// x.mod_mul_assign(Natural::from(6u32), Natural::from(10u32));
    /// assert_eq!(x, 2);
    /// ```
    ///
    /// This is equivalent to `_fmpz_mod_mulN` from `fmpz_mod/mul.c`, FLINT 2.7.1, where `b`, `c`,
    /// and `m` are taken by value and `a == b`.
    #[inline]
    fn mod_mul_assign(&mut self, other: Natural, m: Natural) {
        let data = precompute_mod_mul_data_helper(&m);
        self.mod_mul_precomputed_assign(other, m, &data);
    }
}

impl<'a> ModMulAssign<Natural, &'a Natural> for Natural {
    /// Multiplies two [`Natural`]s modulo a third [`Natural`] $m$, in place. The inputs must be
    /// already reduced modulo $m$. The first [`Natural`] on the right-hand side is taken by value
    /// and the second by reference.
    ///
    /// $x \gets z$, where $x, y, z < m$ and $xy \equiv z \mod m$.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `m.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `self` or `other` are greater than or equal to `m`.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::ModMulAssign;
    /// use malachite_nz::natural::Natural;
    ///
    /// let mut x = Natural::from(3u32);
    /// x.mod_mul_assign(Natural::from(4u32), &Natural::from(15u32));
    /// assert_eq!(x, 12);
    ///
    /// let mut x = Natural::from(7u32);
    /// x.mod_mul_assign(Natural::from(6u32), &Natural::from(10u32));
    /// assert_eq!(x, 2);
    /// ```
    ///
    /// This is equivalent to `_fmpz_mod_mulN` from `fmpz_mod/mul.c`, FLINT 2.7.1, where `b` and `c`
    /// are taken by value, `m` is taken by reference, and `a == b`.
    #[inline]
    fn mod_mul_assign(&mut self, other: Natural, m: &'a Natural) {
        self.mod_mul_precomputed_assign(other, m, &precompute_mod_mul_data_helper(m));
    }
}

impl<'a> ModMulAssign<&'a Natural, Natural> for Natural {
    /// Multiplies two [`Natural`]s modulo a third [`Natural`] $m$, in place. The inputs must be
    /// already reduced modulo $m$. The first [`Natural`] on the right-hand side is taken by
    /// reference and the second by value.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `m.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `self` or `other` are greater than or equal to `m`.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::ModMulAssign;
    /// use malachite_nz::natural::Natural;
    ///
    /// let mut x = Natural::from(3u32);
    /// x.mod_mul_assign(&Natural::from(4u32), Natural::from(15u32));
    /// assert_eq!(x, 12);
    ///
    /// let mut x = Natural::from(7u32);
    /// x.mod_mul_assign(&Natural::from(6u32), Natural::from(10u32));
    /// assert_eq!(x, 2);
    /// ```
    ///
    /// This is equivalent to `_fmpz_mod_mulN` from `fmpz_mod/mul.c`, FLINT 2.7.1, where `b` and `m`
    /// are taken by value, `c` is taken by reference, and `a == b`.
    #[inline]
    fn mod_mul_assign(&mut self, other: &'a Natural, m: Natural) {
        let data = precompute_mod_mul_data_helper(&m);
        self.mod_mul_precomputed_assign(other, m, &data);
    }
}

impl<'a, 'b> ModMulAssign<&'a Natural, &'b Natural> for Natural {
    /// Multiplies two [`Natural`]s modulo a third [`Natural`] $m$, in place. The inputs must be
    /// already reduced modulo $m$. Both [`Natural`]s on the right-hand side are taken by reference.
    ///
    /// $x \gets z$, where $x, y, z < m$ and $xy \equiv z \mod m$.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `m.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `self` or `other` are greater than or equal to `m`.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::ModMulAssign;
    /// use malachite_nz::natural::Natural;
    ///
    /// let mut x = Natural::from(3u32);
    /// x.mod_mul_assign(&Natural::from(4u32), &Natural::from(15u32));
    /// assert_eq!(x, 12);
    ///
    /// let mut x = Natural::from(7u32);
    /// x.mod_mul_assign(&Natural::from(6u32), &Natural::from(10u32));
    /// assert_eq!(x, 2);
    /// ```
    ///
    /// This is equivalent to `_fmpz_mod_mulN` from `fmpz_mod/mul.c`, FLINT 2.7.1, where `b` is
    /// taken by value, `c` and `m` are taken by reference, and `a == b`.
    #[inline]
    fn mod_mul_assign(&mut self, other: &'a Natural, m: &'b Natural) {
        self.mod_mul_precomputed_assign(other, m, &precompute_mod_mul_data_helper(m));
    }
}
