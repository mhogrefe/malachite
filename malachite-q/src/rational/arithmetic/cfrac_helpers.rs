// Copyright © 2026 Mikhail Hogrefe
//
// Uses code adopted from the FLINT Library.
//
//      Copyright © 2019, 2021 Daniel Schultz
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

// The continued-fraction and half-gcd machinery of fmpq/get_cfrac_helpers.c, FLINT 3.6.0, shared by
// the functions that need a continued-fraction expansion faster than one term at a time: rational
// reconstruction and the simplest fraction in an interval. The word-level kernels and the `Mat22`
// matrix of accumulated quotients are here too, since both consumers use them.

use core::cmp::Ordering;
use core::mem::{replace, swap};
use malachite_base::num::arithmetic::traits::{
    AddMul, AddMulAssign, CheckedSub, DivMod, ModPowerOf2, ModPowerOf2Assign, ModPowerOf2SubAssign,
    SubMul, SubMulAssign, WrappingAddMul,
};
use malachite_base::num::basic::integers::PrimitiveInt;
use malachite_base::num::basic::traits::{One, Zero};
use malachite_base::num::conversion::traits::{ExactFrom, JoinHalves, WrappingFrom};
use malachite_base::num::logic::traits::{NotAssign, SignificantBits};
use malachite_nz::integer::Integer;
use malachite_nz::natural::Natural;
use malachite_nz::natural::arithmetic::mul::limb::limbs_mul_limb_to_out;
use malachite_nz::natural::arithmetic::sub_mul::limbs_sub_mul_limb_same_length_in_place_left;
use malachite_nz::platform::{DoubleLimb, Limb};

// This is _hgcd_uiui_no_write from fmpq/reconstruct_fmpz_2.c, FLINT 3.6.0: a half-gcd on two-limb
// values, accumulating quotients into a word matrix whose determinant sign is tracked as `det_pos`.
// Returns the number of quotients applied; zero means no progress and the matrix is meaningless.
pub(super) struct WordMat {
    pub(super) m11: Limb,
    pub(super) m12: Limb,
    pub(super) m21: Limb,
    pub(super) m22: Limb,
    pub(super) det_pos: bool,
}

pub(super) fn hgcd_word(mut a: DoubleLimb, mut b: DoubleLimb) -> (usize, WordMat) {
    let mut m = WordMat {
        m11: 1,
        m12: 0,
        m21: 0,
        m22: 1,
        det_pos: true,
    };
    let mut written = 0;
    let mut last_written: Limb = 0;
    debug_assert!(a >> Limb::WIDTH != 0);
    debug_assert!(b <= a);
    if b >> Limb::WIDTH == 0 || b >= a {
        return (0, m);
    }
    loop {
        let (q, r) = a.div_mod(b);
        let q = Limb::wrapping_from(q);
        // The candidate entries may wrap; they are discarded when the remainder loses its high
        // limb, and entries that are stored are bounded by the original a over the current b, which
        // fits a limb.
        let t1 = m.m12.wrapping_add_mul(q, m.m11);
        let t2 = m.m22.wrapping_add_mul(q, m.m21);
        if r >> Limb::WIDTH == 0 {
            break;
        }
        a = b;
        b = r;
        m.m12 = m.m11;
        m.m22 = m.m21;
        m.m11 = t1;
        m.m21 = t2;
        m.det_pos.not_assign();
        last_written = q;
        written += 1;
    }
    // The last quotient is only trustworthy if the next remainder could not have been absorbed: a -
    // b must be at least the relevant column sum, and b must exceed the entry that bounds the
    // smallest representable remainder. Otherwise pop the last quotient.
    let d = a - b;
    let (small_entry, column_sum) = if m.det_pos {
        (m.m21, DoubleLimb::from(m.m11) + DoubleLimb::from(m.m12))
    } else {
        (m.m11, DoubleLimb::from(m.m21) + DoubleLimb::from(m.m22))
    };
    if b <= DoubleLimb::from(small_entry) || d < column_sum {
        debug_assert!(written >= 1);
        debug_assert_ne!(last_written, 0);
        written -= 1;
        let q = last_written;
        let t1 = m.m11 - q * m.m12;
        let t2 = m.m21 - q * m.m22;
        m.m11 = m.m12;
        m.m21 = m.m22;
        m.m12 = t1;
        m.m22 = t2;
        m.det_pos.not_assign();
    }
    (written, m)
}

// res = x * a - y * b over n limbs. The result must be nonnegative; returns its normalized length.
// This is flint_mpn_fmms1 from mpn_extras.h, FLINT 3.6.0.
pub(super) fn fmms1(res: &mut [Limb], x: Limb, a: &[Limb], y: Limb, b: &[Limb], n: usize) -> usize {
    fmms1_checked(res, x, a, y, b, n).expect("fmms1 result must be nonnegative")
}

// `fmms1` where the result is not known in advance to be nonnegative, which is the case whenever a
// window taken from one endpoint of a ball is applied to the other. `None` means the difference
// would have been negative, and leaves `res` clobbered; FLINT signals the same thing with a
// nonpositive length.
fn fmms1_checked(
    res: &mut [Limb],
    x: Limb,
    a: &[Limb],
    y: Limb,
    b: &[Limb],
    n: usize,
) -> Option<usize> {
    let carry = limbs_mul_limb_to_out::<DoubleLimb, Limb>(&mut res[..n], &a[..n], x);
    let borrow = limbs_sub_mul_limb_same_length_in_place_left(&mut res[..n], &b[..n], y);
    if carry != borrow {
        return None;
    }
    let mut len = n;
    while len > 0 && res[len - 1] == 0 {
        len -= 1;
    }
    Some(len)
}

// Whether the number in the normalized ascending limb slice `x` is at most the one in `bound`.
pub(super) fn limbs_at_most(x: &[Limb], bound: &[Limb]) -> bool {
    limbs_cmp(x, bound) != Ordering::Greater
}

// Compares two normalized ascending limb slices as numbers.
pub(super) fn limbs_cmp(a: &[Limb], b: &[Limb]) -> Ordering {
    a.len()
        .cmp(&b.len())
        .then_with(|| a.iter().rev().cmp(b.iter().rev()))
}

// (hi << shift) | (lo >> (W - shift)), with the zero-shift case guarded. This is MPN_LEFT_SHIFT_HI
// from mpn_extras.h, FLINT 3.6.0.
pub(super) const fn left_shift_hi(hi: Limb, lo: Limb, shift: u32) -> Limb {
    if shift == 0 {
        hi
    } else {
        (hi << shift) | (lo >> (const { Limb::WIDTH as u32 } - shift))
    }
}

// This is FMPQ_RECONSTRUCT_HGCD_CUTOFF from fmpq.h, FLINT 3.6.0: the limb gap between the operand
// and the bound above which the subquadratic splitter is used.
pub(super) const RECONSTRUCT_HGCD_CUTOFF: u64 = 500;

// The same cutoff in bits, which is how both engines compare it.
const HGCD_CUTOFF_BITS: u64 = RECONSTRUCT_HGCD_CUTOFF << Limb::LOG_WIDTH;

// The floor below which a word window cannot pay for itself, FLINT's `4*FLINT_BITS`.
const LEHMER_FLOOR_BITS: u64 = Limb::WIDTH << 2;

// This is _fmpz_mat22_t and its operations from fmpq/mat22.c, FLINT 3.6.0. Throughout the half-gcd
// the entries are nonnegative, so they are [`Natural`]s, and the subtractions below all
// reconstitute earlier nonnegative values. The determinant, always 1 or -1, is tracked as
// `det_pos`.
pub(super) struct Mat22 {
    pub(super) m11: Natural,
    pub(super) m12: Natural,
    pub(super) m21: Natural,
    pub(super) m22: Natural,
    pub(super) det_pos: bool,
}

impl Mat22 {
    pub(super) const fn one() -> Self {
        Self {
            m11: Natural::ONE,
            m12: Natural::ZERO,
            m21: Natural::ZERO,
            m22: Natural::ONE,
            det_pos: true,
        }
    }

    pub(super) fn is_one(&self) -> bool {
        self.m11 == 1u32 && self.m12 == 0u32 && self.m21 == 0u32 && self.m22 == 1u32
    }

    pub(super) fn bits(&self) -> u64 {
        self.m11
            .significant_bits()
            .max(self.m12.significant_bits())
            .max(self.m21.significant_bits())
            .max(self.m22.significant_bits())
    }

    // M = M * N
    pub(super) fn rmul(&mut self, n: &Self) {
        let a = (&self.m11 * &n.m11).add_mul(&self.m12, &n.m21);
        let b = (&self.m11 * &n.m12).add_mul(&self.m12, &n.m22);
        let c = (&self.m21 * &n.m11).add_mul(&self.m22, &n.m21);
        let d = (&self.m21 * &n.m12).add_mul(&self.m22, &n.m22);
        self.m11 = a;
        self.m12 = b;
        self.m21 = c;
        self.m22 = d;
        if !n.det_pos {
            self.det_pos.not_assign();
        }
    }

    // M = M * n, where n is a word matrix
    pub(super) fn rmul_word(&mut self, n: &WordMat) {
        let a = (&self.m11 * Natural::from(n.m11)).add_mul(&self.m12, Natural::from(n.m21));
        self.m12 *= Natural::from(n.m22);
        self.m12.add_mul_assign(&self.m11, Natural::from(n.m12));
        self.m11 = a;
        let a = (&self.m21 * Natural::from(n.m11)).add_mul(&self.m22, Natural::from(n.m21));
        self.m22 *= Natural::from(n.m22);
        self.m22.add_mul_assign(&self.m21, Natural::from(n.m12));
        self.m21 = a;
        if !n.det_pos {
            self.det_pos.not_assign();
        }
    }

    // M = M * n^-1, undoing an rmul_word; every difference is an entry that existed before that
    // multiplication, so none goes negative
    pub(super) fn rmul_inv_word(&mut self, n: &WordMat) {
        let (a, b) = if n.det_pos {
            (
                (&self.m11 * Natural::from(n.m22)).sub_mul(&self.m12, &Natural::from(n.m21)),
                (&self.m12 * Natural::from(n.m11)).sub_mul(&self.m11, &Natural::from(n.m12)),
            )
        } else {
            (
                (&self.m12 * Natural::from(n.m21)).sub_mul(&self.m11, &Natural::from(n.m22)),
                (&self.m11 * Natural::from(n.m12)).sub_mul(&self.m12, &Natural::from(n.m11)),
            )
        };
        self.m11 = a;
        self.m12 = b;
        let (a, b) = if n.det_pos {
            (
                (&self.m21 * Natural::from(n.m22)).sub_mul(&self.m22, &Natural::from(n.m21)),
                (&self.m22 * Natural::from(n.m11)).sub_mul(&self.m21, &Natural::from(n.m12)),
            )
        } else {
            (
                (&self.m22 * Natural::from(n.m21)).sub_mul(&self.m21, &Natural::from(n.m22)),
                (&self.m21 * Natural::from(n.m12)).sub_mul(&self.m22, &Natural::from(n.m11)),
            )
        };
        self.m21 = a;
        self.m22 = b;
        if !n.det_pos {
            self.det_pos.not_assign();
        }
    }

    // M = M * [q 1; 1 0]
    pub(super) fn rmul_elem(&mut self, q: &Natural) {
        self.m12.add_mul_assign(&self.m11, q);
        self.m22.add_mul_assign(&self.m21, q);
        swap(&mut self.m11, &mut self.m12);
        swap(&mut self.m21, &mut self.m22);
        self.det_pos.not_assign();
    }

    // M = M * [q 1; 1 0]^-1 = M * [0 1; 1 -q], undoing an rmul_elem
    pub(super) fn rmul_inv_elem(&mut self, q: &Natural) {
        self.m11.sub_mul_assign(&self.m12, q);
        self.m21.sub_mul_assign(&self.m22, q);
        swap(&mut self.m11, &mut self.m12);
        swap(&mut self.m21, &mut self.m22);
        self.det_pos.not_assign();
    }

    // (ya, yb) += N^-1 * (xa, xb); the results are remainders in the half-gcd, so the subtractions
    // do not go negative when performed after the additions
    pub(super) fn addmul_inv_vec(
        &self,
        ya: &mut Natural,
        yb: &mut Natural,
        xa: &Natural,
        xb: &Natural,
    ) {
        if self.det_pos {
            ya.add_mul_assign(&self.m22, xa);
            ya.sub_mul_assign(&self.m12, xb);
            yb.add_mul_assign(&self.m11, xb);
            yb.sub_mul_assign(&self.m21, xa);
        } else {
            ya.add_mul_assign(&self.m12, xb);
            ya.sub_mul_assign(&self.m22, xa);
            yb.add_mul_assign(&self.m21, xa);
            yb.sub_mul_assign(&self.m11, xb);
        }
    }

    // (ya, yb) += N^-1 * (xa, xb) where the right-hand side may be negative, as it is in the split
    // reassembly. The results are ball endpoints and so are nonnegative.
    pub(super) fn addmul_inv_vec_signed(
        &self,
        ya: &mut Natural,
        yb: &mut Natural,
        xa: &Integer,
        xb: &Integer,
    ) {
        let mut a = Integer::from(&*ya);
        let mut b = Integer::from(&*yb);
        if self.det_pos {
            a.add_mul_assign(Integer::from(&self.m22), xa);
            a.sub_mul_assign(Integer::from(&self.m12), xb);
            b.add_mul_assign(Integer::from(&self.m11), xb);
            b.sub_mul_assign(Integer::from(&self.m21), xa);
        } else {
            a.add_mul_assign(Integer::from(&self.m12), xb);
            a.sub_mul_assign(Integer::from(&self.m22), xa);
            b.add_mul_assign(Integer::from(&self.m21), xa);
            b.sub_mul_assign(Integer::from(&self.m11), xb);
        }
        *ya = Natural::exact_from(a);
        *yb = Natural::exact_from(b);
    }
}

// This is _hgcd_ok from fmpq/get_cfrac_helpers.c, FLINT 3.6.0: whether the matrix M and the pair a
// > b > 0 still describe an open interval of reals greater than one. Unlike the asserts elsewhere,
// this is control flow: the half-gcd uses it to decide when to stop.
fn hgcd_ok(m: &Mat22, a: &Natural, b: &Natural) -> bool {
    if *a <= *b || *b == 0u32 {
        return false;
    }
    let ok = if m.det_pos {
        *a > m.m12 && *b > m.m21
    } else {
        *a > m.m22 && *b > m.m11
    };
    if !ok {
        return false;
    }
    let column_sum = if m.det_pos {
        &m.m11 + &m.m12
    } else {
        &m.m21 + &m.m22
    };
    a - b >= column_sum
}

// This is _hgcd_split from fmpq/get_cfrac_helpers.c, FLINT 3.6.0: truncate the ball described by
// (M, ya/yb) to a shifted pair on which the half-gcd can recurse. Returns the adjusted shift, or
// zero if no useful truncation exists.
fn hgcd_split(
    xa: &mut Natural,
    xb: &mut Natural,
    ya: &Natural,
    yb: &Natural,
    m: &Mat22,
    mut shift: u64,
) -> u64 {
    let (mut ta, mut tb);
    if m.det_pos {
        *xa = ya - &m.m12;
        *xb = yb - &m.m21;
        ta = ya + &m.m22;
        tb = yb + &m.m11;
    } else {
        *xa = ya - &m.m22;
        *xb = yb - &m.m11;
        ta = ya + &m.m12;
        tb = yb + &m.m21;
    }
    *xa >>= shift;
    ta >>= shift;
    *xb >>= shift;
    tb >>= shift;
    if *xb == 0u32 || *xa <= *xb {
        return 0;
    }
    while *xa != ta || *xb != tb {
        shift += 1;
        *xa >>= 1u32;
        ta >>= 1u32;
        *xb >>= 1u32;
        tb >>= 1u32;
        if *xb == 0u32 || *xa <= *xb {
            return 0;
        }
    }
    shift
}

// This is _uiui_hgcd from fmpq/get_cfrac_helpers.c, FLINT 3.6.0: hgcd_word, but writing the
// quotients out for the continued-fraction list.
fn hgcd_word_write(
    quotients: &mut [Limb; (Limb::WIDTH as usize) << 1],
    mut a: DoubleLimb,
    mut b: DoubleLimb,
) -> (usize, WordMat) {
    let mut m = WordMat {
        m11: 1,
        m12: 0,
        m21: 0,
        m22: 1,
        det_pos: true,
    };
    let mut written = 0;
    debug_assert!(a >> Limb::WIDTH != 0);
    debug_assert!(b <= a);
    if b >> Limb::WIDTH == 0 || b >= a {
        return (0, m);
    }
    loop {
        let (q, r) = a.div_mod(b);
        let q = Limb::wrapping_from(q);
        let t1 = m.m12.wrapping_add_mul(q, m.m11);
        let t2 = m.m22.wrapping_add_mul(q, m.m21);
        if r >> Limb::WIDTH == 0 {
            break;
        }
        a = b;
        b = r;
        m.m12 = m.m11;
        m.m22 = m.m21;
        m.m11 = t1;
        m.m21 = t2;
        m.det_pos.not_assign();
        quotients[written] = q;
        written += 1;
    }
    let d = a - b;
    let (small_entry, column_sum) = if m.det_pos {
        (m.m21, DoubleLimb::from(m.m11) + DoubleLimb::from(m.m12))
    } else {
        (m.m11, DoubleLimb::from(m.m21) + DoubleLimb::from(m.m22))
    };
    if b <= DoubleLimb::from(small_entry) || d < column_sum {
        debug_assert!(written >= 1);
        written -= 1;
        let q = quotients[written];
        let t1 = m.m11 - q * m.m12;
        let t2 = m.m21 - q * m.m22;
        m.m11 = m.m12;
        m.m21 = m.m22;
        m.m12 = t1;
        m.m22 = t2;
        m.det_pos.not_assign();
    }
    (written, m)
}

// This is _lehmer_exact from fmpq/get_cfrac_helpers.c, FLINT 3.6.0, specialized to the
// CFRAC_NEED_MATRIX | CFRAC_NEED_HGCD mode, the only one the half-gcd uses: word windows are only
// kept when an over-strict fast version of hgcd_ok holds, and are undone otherwise. The
// quotient-list limit is unlimited here, so FLINT's limit checks drop out.
#[cfg_attr(dylint_lib = "malachite_lints", allow(adjacent_vec_allocations))]
fn lehmer_exact(
    mut s: Option<&mut Vec<Natural>>,
    m: &mut Mat22,
    xa: &mut Natural,
    xb: &mut Natural,
) {
    let mut xn = xa.to_limbs_asc();
    let mut xn_len = xn.len();
    if xn_len < 3 {
        return;
    }
    let capacity = xn_len;
    let mut xd = xb.to_limbs_asc();
    let mut xd_len = xd.len();
    xd.resize(capacity, 0);
    // The scratch buffers are swapped with xn and xd as the windows apply.
    let mut yn = vec![0 as Limb; capacity];
    let mut yd = vec![0 as Limb; capacity];
    let mut quotients = [0 as Limb; (Limb::WIDTH as usize) << 1];
    loop {
        let n = xn_len;
        if n < 3
            || xd_len <= 3 + usize::wrapping_from(m.bits() >> Limb::LOG_WIDTH)
            || (n != xd_len && n != xd_len + 1)
        {
            break;
        }
        if n == xd_len + 1 {
            xd[n - 1] = 0;
        }
        let x_lz = xn[n - 1].leading_zeros();
        let a1 = left_shift_hi(xn[n - 1], xn[n - 2], x_lz);
        let a0 = left_shift_hi(xn[n - 2], xn[n - 3], x_lz);
        let b1 = left_shift_hi(xd[n - 1], xd[n - 2], x_lz);
        let b0 = left_shift_hi(xd[n - 2], xd[n - 3], x_lz);
        let (written, wm) = hgcd_word_write(
            &mut quotients,
            DoubleLimb::join_halves(a1, a0),
            DoubleLimb::join_halves(b1, b0),
        );
        if written == 0 {
            break;
        }
        let (yn_len, yd_len) = if wm.det_pos {
            (
                fmms1(&mut yn, wm.m22, &xn, wm.m12, &xd, n),
                fmms1(&mut yd, wm.m11, &xd, wm.m21, &xn, n),
            )
        } else {
            (
                fmms1(&mut yn, wm.m12, &xd, wm.m22, &xn, n),
                fmms1(&mut yd, wm.m21, &xn, wm.m11, &xd, n),
            )
        };
        if yn_len == 0 || yd_len == 0 {
            // defensive, unobserved: a window annihilating an operand entirely
            break;
        }
        // the over-strict but fast hgcd_ok(M, yn, yd)
        debug_assert!(yn_len >= yd_len);
        m.rmul_word(&wm);
        let mut its_ok = false;
        for j in 2 + usize::wrapping_from(m.bits() >> Limb::LOG_WIDTH)..yn_len {
            let aa = yn[j];
            let bb = if j < yd_len { yd[j] } else { 0 };
            if aa > bb && aa - bb > 1 {
                its_ok = true;
                break;
            }
        }
        if !its_ok {
            m.rmul_inv_word(&wm);
            break;
        }
        if let Some(s) = s.as_deref_mut() {
            for &q in &quotients[..written] {
                s.push(Natural::from(q));
            }
        }
        swap(&mut xn, &mut yn);
        swap(&mut xd, &mut yd);
        xn_len = yn_len;
        xd_len = yd_len;
    }
    xn.truncate(xn_len);
    *xa = Natural::from_owned_limbs_asc(xn);
    xd.truncate(xd_len);
    *xb = Natural::from_owned_limbs_asc(xd);
}

// This is _hgcd_step from fmpq/get_cfrac_helpers.c, FLINT 3.6.0: transport the truncated operands'
// half-gcd result N back to the full operands. The wrap-around subtraction under mod 2^shift goes
// through Integer, which is what fdiv_r_2exp of a negative value computes.
fn hgcd_step(
    m: &mut Mat22,
    xa: &mut Natural,
    xb: &mut Natural,
    shift: u64,
    n: &Mat22,
    ya: &mut Natural,
    yb: &mut Natural,
) {
    let (ca, cb) = if m.det_pos {
        (&m.m12, &m.m21)
    } else {
        (&m.m22, &m.m11)
    };
    // xa = ((xa - ca) mod 2^shift) + ca, and likewise for xb. FLINT reaches the same value by
    // letting the subtraction go negative and taking fdiv_r_2exp of it; subtracting modularly
    // instead keeps everything in the Naturals. Reducing ca first is what the modular subtraction
    // requires of its operands, and does not change the result, since ca and ca mod 2^shift are
    // congruent.
    xa.mod_power_of_2_assign(shift);
    xa.mod_power_of_2_sub_assign(ca.mod_power_of_2(shift), shift);
    *xa += ca;
    xb.mod_power_of_2_assign(shift);
    xb.mod_power_of_2_sub_assign(cb.mod_power_of_2(shift), shift);
    *xb += cb;
    *ya <<= shift;
    *yb <<= shift;
    n.addmul_inv_vec(ya, yb, xa, xb);
    swap(xa, ya);
    swap(xb, yb);
    m.rmul(n);
}

// This is _fmpq_hgcd from fmpq/get_cfrac_helpers.c, FLINT 3.6.0: for a > b > 0, generate
// continued-fraction terms valid for every real in the open interval M^-1(a/(b+1), (a+1)/b),
// appending the terms to s and multiplying M on the right. Subquadratic: large operands are
// truncated, recursed on, and stitched back with hgcd_step.
pub(super) fn fmpq_hgcd(
    mut s: Option<&mut Vec<Natural>>,
    m: &mut Mat22,
    xa: &mut Natural,
    xb: &mut Natural,
) {
    let mut ya = Natural::ZERO;
    let mut yb = Natural::ZERO;
    loop {
        debug_assert!(hgcd_ok(m, xa, xb));
        let k = xa.significant_bits() - m.bits();
        if k > HGCD_CUTOFF_BITS {
            let km = m.bits();
            let shift = hgcd_split(&mut ya, &mut yb, xa, xb, m, km + (k >> 1));
            if shift != 0 {
                let mut n = Mat22::one();
                fmpq_hgcd(s.as_deref_mut(), &mut n, &mut ya, &mut yb);
                if !n.is_one() {
                    hgcd_step(m, xa, xb, shift, &n, &mut ya, &mut yb);
                    debug_assert!(hgcd_ok(m, xa, xb));
                    let km = m.bits();
                    let shift = hgcd_split(&mut ya, &mut yb, xa, xb, m, km + 1);
                    if shift != 0 {
                        let mut n = Mat22::one();
                        fmpq_hgcd(s.as_deref_mut(), &mut n, &mut ya, &mut yb);
                        if !n.is_one() {
                            hgcd_step(m, xa, xb, shift, &n, &mut ya, &mut yb);
                            debug_assert!(hgcd_ok(m, xa, xb));
                            continue;
                        }
                    }
                }
            }
        } else if k > LEHMER_FLOOR_BITS {
            lehmer_exact(s.as_deref_mut(), m, xa, xb);
        }
        // one exact Euclidean step; stop when the interval is no longer greater than one
        let (q, r) = (&*xa).div_mod(&*xb);
        m.rmul_elem(&q);
        if !hgcd_ok(m, xb, &r) {
            m.rmul_inv_elem(&q);
            return;
        }
        *xa = replace(xb, r);
        if let Some(s) = s.as_deref_mut() {
            s.push(q);
        }
    }
}

// This is _fmpq_ball_t from fmpq.h, FLINT 3.6.0, restricted to the inexact case: a closed interval
// of rationals with both endpoints greater than one, so all four components are positive. FLINT's
// `exact` flag distinguishes a degenerate ball, which is what fmpq_get_cfrac needs; nothing here
// does.
pub(super) struct Ball {
    pub(super) left_num: Natural,
    pub(super) left_den: Natural,
    pub(super) right_num: Natural,
    pub(super) right_den: Natural,
}

impl Ball {
    // This is _fmpq_ball_gt_one from fmpq/get_cfrac_helpers.c, FLINT 3.6.0: whether the ball is
    // canonical and bounded away from one, so that another term can be taken from it.
    fn gt_one(&self) -> bool {
        self.left_den != 0u32
            && self.left_den < self.left_num
            && self.right_den != 0u32
            && self.right_den < self.right_num
    }

    // This is _fmpq_ball_apply_mat22_inv_elem2 from fmpq/get_cfrac_helpers.c, FLINT 3.6.0: y = [q
    // 1; 1 0]^-1(x), reusing the left endpoint's remainder. FLINT lets the new left denominator go
    // negative here and rejects it a moment later through `gt_one`; a [`Natural`] cannot hold that,
    // so the subtraction is checked and `None` stands for the rejection.
    //
    // The rejection is believed unreachable: q is the floor of the left endpoint and the ball is
    // ordered, so the right endpoint is at least q and the difference cannot go negative. An
    // instrumented run observed zero hits. It is kept because it is what makes the [`Natural`]
    // arithmetic total, and because FLINT admits the case.
    fn apply_inv_elem2(&self, q: &Natural, r: Natural) -> Option<Self> {
        Some(Self {
            left_num: self.right_den.clone(),
            left_den: (&self.right_num).checked_sub(&self.right_den * q)?,
            right_num: self.left_den.clone(),
            right_den: r,
        })
    }
}

// This is _fmpz_tail_bits from fmpq/get_cfrac_helpers.c, FLINT 3.6.0: for a >= b > 0, the smallest
// k with floor(a / 2^k) = floor(b / 2^k), which is where their leading bits stop agreeing. When
// they never differ FLINT returns a's bit count, and so does this.
fn tail_bits(a: &Natural, b: &Natural) -> u64 {
    let d = a ^ b;
    if d == 0u32 {
        a.significant_bits()
    } else {
        d.significant_bits()
    }
}

// This is _fmpq_ball_get_cfrac from fmpq/get_cfrac_helpers.c, FLINT 3.6.0, in the no-write mode
// (FLINT's `s->length = -1`) that wants only the matrix: generate the continued-fraction terms
// valid for every rational in the ball, accumulating them into `m`, and reduce the ball by them.
// The Lehmer tier is not yet ported, so sizes between the gauss and split thresholds take the gauss
// loop.
#[allow(dead_code)]
pub(super) fn ball_get_cfrac(m: &mut Mat22, x: &mut Ball) {
    ball_get_cfrac_with_cutoff(m, x, HGCD_CUTOFF_BITS);
}

// The engine proper. The split threshold is a parameter so that tests can drive the subquadratic
// path at sizes small enough to check against a naive expansion.
fn ball_get_cfrac_with_cutoff(m: &mut Mat22, x: &mut Ball, cutoff: u64) {
    *m = Mat22::one();
    // When one component is shared by the endpoints, the terms agree until the others' leading bits
    // diverge, and that whole prefix comes from one exact half-gcd on the truncated pair.
    if x.left_num == x.right_num {
        chop(m, x, tail_bits(&x.left_den, &x.right_den));
    } else if x.left_den == x.right_den {
        chop(m, x, tail_bits(&x.right_num, &x.left_num));
    }
    loop {
        debug_assert!(x.gt_one());
        if x.left_num.significant_bits() > cutoff {
            let mut n = Mat22::one();
            if split(&mut n, x, x.left_num.significant_bits() >> 1, cutoff) {
                m.rmul(&n);
                let mut n = Mat22::one();
                ball_get_cfrac_with_cutoff(&mut n, x, cutoff);
                m.rmul(&n);
                return;
            }
        } else if x.left_num.significant_bits() > LEHMER_FLOOR_BITS {
            lehmer_inexact(m, x);
        }
        // The gauss step: take one term, and keep it only if the whole ball still admits another.
        let (q, r) = (&x.left_num).div_mod(&x.left_den);
        let Some(y) = x.apply_inv_elem2(&q, r) else {
            return;
        };
        if !y.gt_one() {
            return;
        }
        *x = y;
        m.rmul_elem(&q);
    }
}

// This is _lehmer_inexact from fmpq/get_cfrac_helpers.c, FLINT 3.6.0: batches of terms taken from a
// two-limb window on the ball's left endpoint, applied to both endpoints at once. The window's
// matrix is derived from the left endpoint only, so applying it to the right one can underflow;
// that is what the fallible `fmms1_checked` is for, and it ends the batch exactly where FLINT's
// nonpositive-length check does.
#[cfg_attr(dylint_lib = "malachite_lints", allow(adjacent_vec_allocations))]
fn lehmer_inexact(m: &mut Mat22, x: &mut Ball) {
    // The size check must precede the conversions: this function is attempted once per gauss step,
    // and in the regime where one endpoint's representation has already shrunk below the window
    // size, paying four limb-vector conversions per step just to bail is what FLINT's free
    // COEFF_IS_MPZ check avoids.
    if x.left_num.limb_count() < 3
        || x.left_den.limb_count() < 3
        || x.right_num.limb_count() < 3
        || x.right_den.limb_count() < 3
    {
        return;
    }
    let mut xln = x.left_num.to_limbs_asc();
    let mut xld = x.left_den.to_limbs_asc();
    let mut xrn = x.right_num.to_limbs_asc();
    let mut xrd = x.right_den.to_limbs_asc();
    let (mut nl, mut ldl) = (xln.len(), xld.len());
    let (mut nr, mut rdl) = (xrn.len(), xrd.len());
    // Every array is padded to the longest numerator, so that a window may read one limb past a
    // shorter denominator; the loop below zeroes that limb whenever it does.
    let capacity = nl.max(nr);
    for v in [&mut xln, &mut xld, &mut xrn, &mut xrd] {
        v.resize(capacity, 0);
    }
    let mut yln = vec![0 as Limb; capacity];
    let mut yld = vec![0 as Limb; capacity];
    let mut yrn = vec![0 as Limb; capacity];
    let mut yrd = vec![0 as Limb; capacity];
    loop {
        if nl < 3 || nr < 3 || (nl != ldl && nl != ldl + 1) || (nr != rdl && nr != rdl + 1) {
            break;
        }
        if nl == ldl + 1 {
            xld[nl - 1] = 0;
        }
        if nr == rdl + 1 {
            xrd[nr - 1] = 0;
        }
        let lz = xln[nl - 1].leading_zeros();
        let a1 = left_shift_hi(xln[nl - 1], xln[nl - 2], lz);
        let a0 = left_shift_hi(xln[nl - 2], xln[nl - 3], lz);
        let b1 = left_shift_hi(xld[nl - 1], xld[nl - 2], lz);
        let b0 = left_shift_hi(xld[nl - 2], xld[nl - 3], lz);
        let (written, h) = hgcd_word(
            DoubleLimb::join_halves(a1, a0),
            DoubleLimb::join_halves(b1, b0),
        );
        if written == 0 {
            break;
        }
        // A determinant of -1 reverses the interval, so the transformed endpoints trade places.
        let lens = if h.det_pos {
            (
                fmms1_checked(&mut yln, h.m22, &xln, h.m12, &xld, nl),
                fmms1_checked(&mut yld, h.m11, &xld, h.m21, &xln, nl),
                fmms1_checked(&mut yrn, h.m22, &xrn, h.m12, &xrd, nr),
                fmms1_checked(&mut yrd, h.m11, &xrd, h.m21, &xrn, nr),
            )
        } else {
            (
                fmms1_checked(&mut yln, h.m12, &xrd, h.m22, &xrn, nr),
                fmms1_checked(&mut yld, h.m21, &xrn, h.m11, &xrd, nr),
                fmms1_checked(&mut yrn, h.m12, &xld, h.m22, &xln, nl),
                fmms1_checked(&mut yrd, h.m21, &xln, h.m11, &xld, nl),
            )
        };
        let (Some(new_nl), Some(new_ldl), Some(new_nr), Some(new_rdl)) = lens else {
            break;
        };
        // No endpoint may vanish, and the transformed left one must still be greater than one.
        // FLINT folds these into a single nonpositive-length test; separated, only the ordering
        // half was ever observed to fire, a vanishing endpoint having been seen zero times.
        if new_nl == 0
            || new_ldl == 0
            || new_nr == 0
            || new_rdl == 0
            || limbs_cmp(&yln[..new_nl], &yld[..new_ldl]) != Ordering::Greater
        {
            break;
        }
        nl = new_nl;
        ldl = new_ldl;
        nr = new_nr;
        rdl = new_rdl;
        m.rmul_word(&h);
        swap(&mut xln, &mut yln);
        swap(&mut xld, &mut yld);
        swap(&mut xrn, &mut yrn);
        swap(&mut xrd, &mut yrd);
    }
    x.left_num = Natural::from_limbs_asc(&xln[..nl]);
    x.left_den = Natural::from_limbs_asc(&xld[..ldl]);
    x.right_num = Natural::from_limbs_asc(&xrn[..nr]);
    x.right_den = Natural::from_limbs_asc(&xrd[..rdl]);
}

// The `chop` path of _fmpq_ball_get_cfrac: run the exact half-gcd on the endpoints' shared leading
// bits, then rebuild the ball around the reduced pair.
fn chop(m: &mut Mat22, x: &mut Ball, k: u64) {
    let mut q = &x.left_num >> k;
    let mut r = &x.left_den >> k;
    if r == 0u32 || q <= r {
        return;
    }
    fmpq_hgcd(None, m, &mut q, &mut r);
    if m.is_one() {
        return;
    }
    let low_ln = (&x.left_num).mod_power_of_2(k);
    let low_ld = (&x.left_den).mod_power_of_2(k);
    let low_rn = (&x.right_num).mod_power_of_2(k);
    let low_rd = (&x.right_den).mod_power_of_2(k);
    x.left_num = &q << k;
    x.left_den = &r << k;
    x.right_num = q << k;
    x.right_den = r << k;
    // A determinant of -1 reverses the interval, so each endpoint takes the other's low part.
    let ((a, b), (c, d)) = if m.det_pos {
        ((&low_ln, &low_ld), (&low_rn, &low_rd))
    } else {
        ((&low_rn, &low_rd), (&low_ln, &low_ld))
    };
    m.addmul_inv_vec(&mut x.left_num, &mut x.left_den, a, b);
    m.addmul_inv_vec(&mut x.right_num, &mut x.right_den, c, d);
}

// The `split` path of _fmpq_ball_get_cfrac, which makes the whole engine subquadratic: recurse on a
// ball containing the truncated operands, then transport the result back to the full ones. Returns
// whether any progress was made; if not, the caller falls through to a gauss step.
fn split(n: &mut Mat22, x: &mut Ball, k: u64, cutoff: u64) -> bool {
    let mut y = Ball {
        left_num: &x.left_num >> k,
        left_den: (&x.left_den >> k) + Natural::ONE,
        right_num: (&x.right_num >> k) + Natural::ONE,
        right_den: &x.right_den >> k,
    };
    if !y.gt_one() {
        return false;
    }
    ball_get_cfrac_with_cutoff(n, &mut y, cutoff);
    if n.is_one() {
        return false;
    }
    // The low parts of the original endpoints. The two that were rounded up above owe a borrow of
    // 2^k, which makes them negative; the reassembled endpoints are nonnegative again.
    let two_k = Integer::from(Natural::ONE << k);
    let low_ln = Integer::from((&x.left_num).mod_power_of_2(k));
    let low_ld = Integer::from((&x.left_den).mod_power_of_2(k)) - &two_k;
    let low_rn = Integer::from((&x.right_num).mod_power_of_2(k)) - two_k;
    let low_rd = Integer::from((&x.right_den).mod_power_of_2(k));
    let ((a, b), (c, d)) = if n.det_pos {
        ((&low_ln, &low_ld), (&low_rn, &low_rd))
    } else {
        ((&low_rn, &low_rd), (&low_ln, &low_ld))
    };
    x.left_num = y.left_num << k;
    x.left_den = y.left_den << k;
    x.right_num = y.right_num << k;
    x.right_den = y.right_den << k;
    n.addmul_inv_vec_signed(&mut x.left_num, &mut x.left_den, a, b);
    n.addmul_inv_vec_signed(&mut x.right_num, &mut x.right_den, c, d);
    true
}

// Runs the ball engine on the interval [x_num/x_den, y_num/y_den], which must be greater than one,
// and returns the accumulated matrix followed by the reduced ball. This exists so that tests can
// check the engine against a naive term-by-term expansion.
#[cfg(feature = "test_build")]
pub fn ball_get_cfrac_for_testing(
    x_num: Natural,
    x_den: Natural,
    y_num: Natural,
    y_den: Natural,
    cutoff: u64,
) -> (
    (Natural, Natural, Natural, Natural, bool),
    (Natural, Natural, Natural, Natural),
) {
    let mut ball = Ball {
        left_num: x_num,
        left_den: x_den,
        right_num: y_num,
        right_den: y_den,
    };
    assert!(ball.gt_one());
    let mut m = Mat22::one();
    ball_get_cfrac_with_cutoff(&mut m, &mut ball, cutoff);
    (
        (m.m11, m.m12, m.m21, m.m22, m.det_pos),
        (ball.left_num, ball.left_den, ball.right_num, ball.right_den),
    )
}
