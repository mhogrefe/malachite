// Copyright © 2026 Mikhail Hogrefe
//
// Uses code adopted from the FLINT Library.
//
//      Copyright © 2011 Fredrik Johansson
//
//      Copyright © 2020 Daniel Schultz
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::Rational;
use core::cmp::Ordering;
use core::mem::{replace, swap, take};
use malachite_base::num::arithmetic::traits::{
    AddMul, AddMulAssign, CoprimeWith, DivMod, FloorSqrt, ModPowerOf2, ModPowerOf2Assign,
    ModPowerOf2SubAssign, Parity, SubMul, SubMulAssign, WrappingAddMul,
};
use malachite_base::num::basic::integers::PrimitiveInt;
use malachite_base::num::basic::traits::{One, Zero};
use malachite_base::num::conversion::traits::{JoinHalves, SplitInHalf, WrappingFrom};
use malachite_base::num::logic::traits::{NotAssign, SignificantBits};
use malachite_nz::natural::Natural;
use malachite_nz::platform::{DoubleLimb, Limb};

// The kernels below follow _fmpq_reconstruct_fmpz_2's size-dispatched structure completely: word
// kernels, the array kernel, the Lehmer windows, and the subquadratic HGCD splitter.
//
// This is _fmpq_reconstruct_fmpz_2_ui from fmpq/reconstruct_fmpz_2.c, FLINT 3.6.0. The eudiv
// division primitive is a plain hardware division here, and coprime_ui collapses to the primitive
// coprimality check. The denominator bound is compared at full precision, as fmpz_cmp_ui does. The
// B == 0 sign guard mirrors reconstruct_helper's; see the comment there.
fn reconstruct_limb(a: Limb, m: Limb, n_bound: Limb, d_bound: &Natural) -> Option<Rational> {
    let mut big_a = m;
    let mut big_b = a;
    let mut m11: Limb = 1;
    let mut m12: Limb = 0;
    let mut mdet_pos = true;
    loop {
        debug_assert!(big_a > big_b && big_b > n_bound);
        // The matrix entries are bounded by m, so nothing here overflows.
        let (q, r) = big_a.div_mod(big_b);
        m12.add_mul_assign(m11, q);
        swap(&mut m11, &mut m12);
        mdet_pos.not_assign();
        big_a = big_b;
        big_b = r;
        if big_b <= n_bound {
            break;
        }
    }
    if *d_bound < m11 || !big_b.coprime_with(m11) {
        return None;
    }
    Some(Rational {
        sign: mdet_pos || big_b == 0,
        numerator: Natural::from(big_b),
        denominator: Natural::from(m11),
    })
}

// This is _fmpq_reconstruct_fmpz_2_uiui from fmpq/reconstruct_fmpz_2.c, FLINT 3.6.0. The three
// eudiv division shapes collapse to a Limb division when the dividend fits one Limb and a
// DoubleLimb division otherwise, and coprime_uiui collapses to the primitive coprimality check. The
// denominator bound is compared at full precision, avoiding fmpz_get_uiui's misread of bounds wider
// than two limbs, which is reachable only outside the documented 2ND < m domain; a unit test pins
// that input. The B == 0 sign guard mirrors reconstruct_helper's; see the comment there.
fn reconstruct_double_limb(
    a: DoubleLimb,
    m: DoubleLimb,
    n_bound: DoubleLimb,
    d_bound: &Natural,
) -> Option<Rational> {
    let mut big_a = m;
    let mut big_b = a;
    let mut m11: DoubleLimb = 1;
    let mut m12: DoubleLimb = 0;
    let mut mdet_pos = true;
    loop {
        debug_assert!(big_a > big_b && big_b > n_bound);
        // The matrix entries are bounded by m, so nothing here overflows.
        let (q, r) = if big_a >> Limb::WIDTH == 0 {
            let (q, r) = Limb::wrapping_from(big_a).div_mod(Limb::wrapping_from(big_b));
            (DoubleLimb::from(q), DoubleLimb::from(r))
        } else {
            big_a.div_mod(big_b)
        };
        m12.add_mul_assign(m11, q);
        swap(&mut m11, &mut m12);
        mdet_pos.not_assign();
        big_a = big_b;
        big_b = r;
        if big_b <= n_bound {
            break;
        }
    }
    if *d_bound < m11 || !big_b.coprime_with(m11) {
        return None;
    }
    Some(Rational {
        sign: mdet_pos || big_b == 0,
        numerator: Natural::from(big_b),
        denominator: Natural::from(m11),
    })
}

// This is _fmpq_reconstruct_fmpz_2 from fmpq/reconstruct_fmpz_2.c, FLINT 3.6.0: the size dispatch
// over the kernels. Moduli of one or two limbs go to the word kernels above, moduli of at most
// ARRAY_LIMIT limbs to the fixed-size array kernel, and the rest to the half-gcd accelerations
// below, with the plain Euclidean ("gauss") loop finishing whatever they leave.
fn reconstruct_helper(
    a: Natural,
    m: Natural,
    n_bound: &Natural,
    d_bound: &Natural,
) -> Option<Rational> {
    assert!(a < m, "a must be reduced mod m");
    assert_ne!(*n_bound, 0u32, "n_bound must be positive");
    assert_ne!(*d_bound, 0u32, "d_bound must be positive");
    // Quickly identify small integers: n = a and n = a - m, with d = 1.
    if a <= *n_bound {
        return Some(Rational::from(a));
    }
    let diff = &m - &a;
    if diff <= *n_bound {
        return Some(-Rational::from(diff));
    }
    // Dispatch small moduli to the fixed-size kernels. The fast paths have already handled a <= N,
    // so N < a < m: all three fit in m's size class, and the conversions do not need checking. The
    // denominator bound is passed at full precision.
    let m_limbs = m.limb_count();
    if m_limbs == 1 {
        return reconstruct_limb(
            Limb::wrapping_from(&a),
            Limb::wrapping_from(&m),
            Limb::wrapping_from(n_bound),
            d_bound,
        );
    }
    if m_limbs == 2 {
        return reconstruct_double_limb(
            DoubleLimb::wrapping_from(&a),
            DoubleLimb::wrapping_from(&m),
            DoubleLimb::wrapping_from(n_bound),
            d_bound,
        );
    }
    if m_limbs <= const { ARRAY_LIMIT as u64 } {
        return reconstruct_array(&a, &m, n_bound, d_bound);
    }
    // A > B > N > 0: reduce until A > N >= B. A wide gap between the sizes of m and N earns the
    // subquadratic splitter and then the Lehmer windows; the plain Euclidean ("gauss") loop
    // finishes whatever they leave.
    let mut r = Reduction::new(m, a);
    let gap = m_limbs - n_bound.limb_count();
    if gap >= 3 && !(gap >= RECONSTRUCT_HGCD_CUTOFF && r.split_loop(n_bound)) {
        r.lehmer_loop(n_bound);
    }
    if r.big_b > *n_bound {
        r.gauss_loop(n_bound);
    }
    r.finish(d_bound)
}

// This is FMPQ_RECONSTRUCT_ARRAY_LIMIT from fmpq/reconstruct_fmpz_2.c, FLINT 3.6.0.
const ARRAY_LIMIT: usize = 12;

// This is _hgcd_uiui_no_write from fmpq/reconstruct_fmpz_2.c, FLINT 3.6.0: a half-gcd on two-limb
// values, accumulating quotients into a word matrix whose determinant sign is tracked as `det_pos`.
// Returns the number of quotients applied; zero means no progress and the matrix is meaningless.
struct WordMat {
    m11: Limb,
    m12: Limb,
    m21: Limb,
    m22: Limb,
    det_pos: bool,
}

fn hgcd_word(mut a: DoubleLimb, mut b: DoubleLimb) -> (usize, WordMat) {
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

// out = a * x over a's length; returns the carry limb. This is mpn_mul_1.
fn mul_1(out: &mut [Limb], a: &[Limb], x: Limb) -> Limb {
    let mut carry: Limb = 0;
    for (o, &d) in out.iter_mut().zip(a.iter()) {
        let p = DoubleLimb::from(d) * DoubleLimb::from(x) + DoubleLimb::from(carry);
        (carry, *o) = p.split_in_half();
    }
    carry
}

// out *= x in place; returns the carry limb. This is mpn_mul_1 with aliased operands.
fn mul_1_in_place(out: &mut [Limb], x: Limb) -> Limb {
    let mut carry: Limb = 0;
    for o in out.iter_mut() {
        let p = DoubleLimb::from(*o) * DoubleLimb::from(x) + DoubleLimb::from(carry);
        (carry, *o) = p.split_in_half();
    }
    carry
}

// out += a * x over a's length; returns the carry limb. This is mpn_addmul_1.
fn addmul_1(out: &mut [Limb], a: &[Limb], x: Limb) -> Limb {
    let mut carry: Limb = 0;
    for (o, &d) in out.iter_mut().zip(a.iter()) {
        let p = DoubleLimb::from(d) * DoubleLimb::from(x)
            + DoubleLimb::from(*o)
            + DoubleLimb::from(carry);
        (carry, *o) = p.split_in_half();
    }
    carry
}

// out -= b * y over b's length; returns the borrow limb. This is mpn_submul_1.
fn submul_1(out: &mut [Limb], b: &[Limb], y: Limb) -> Limb {
    let mut borrow: Limb = 0;
    for (o, &d) in out.iter_mut().zip(b.iter()) {
        let p = DoubleLimb::from(d) * DoubleLimb::from(y) + DoubleLimb::from(borrow);
        let (hi, lo) = p.split_in_half();
        let (diff, under) = o.overflowing_sub(lo);
        *o = diff;
        borrow = hi + Limb::from(under);
    }
    borrow
}

// res = x * a - y * b over n limbs. The result must be nonnegative; returns its normalized length.
// This is flint_mpn_fmms1 from mpn_extras.h, FLINT 3.6.0.
fn fmms1(res: &mut [Limb], x: Limb, a: &[Limb], y: Limb, b: &[Limb], n: usize) -> usize {
    let carry = mul_1(&mut res[..n], &a[..n], x);
    let borrow = submul_1(&mut res[..n], &b[..n], y);
    debug_assert_eq!(carry, borrow, "fmms1 result must be nonnegative");
    let mut len = n;
    while len > 0 && res[len - 1] == 0 {
        len -= 1;
    }
    len
}

// Whether the number in the normalized ascending limb slice `x` is at most the one in `bound`.
fn limbs_at_most(x: &[Limb], bound: &[Limb]) -> bool {
    limbs_cmp(x, bound) != Ordering::Greater
}

// Compares two normalized ascending limb slices as numbers.
fn limbs_cmp(a: &[Limb], b: &[Limb]) -> Ordering {
    a.len()
        .cmp(&b.len())
        .then_with(|| a.iter().rev().cmp(b.iter().rev()))
}

// (hi << shift) | (lo >> (W - shift)), with the zero-shift case guarded. This is MPN_LEFT_SHIFT_HI
// from mpn_extras.h, FLINT 3.6.0.
const fn left_shift_hi(hi: Limb, lo: Limb, shift: u32) -> Limb {
    if shift == 0 {
        hi
    } else {
        (hi << shift) | (lo >> (const { Limb::WIDTH as u32 } - shift))
    }
}

// This is _fmpq_reconstruct_fmpz_2_ui_array from fmpq/reconstruct_fmpz_2.c, FLINT 3.6.0: the
// Euclidean loop on fixed-size limb arrays, accelerated by two-limb half-gcd windows. Where FLINT
// calls mpn_tdiv_qr for a difficult quotient, this rounds the operands through Natural division;
// that branch is rare. The denominator bound is compared at full precision at the end, and the
// mpn_gcd coprimality check collapses to coprime_with.
fn reconstruct_array(
    a: &Natural,
    m: &Natural,
    n_bound: &Natural,
    d_bound: &Natural,
) -> Option<Rational> {
    let mut big_a = [0 as Limb; ARRAY_LIMIT + 1];
    let mut big_b = [0 as Limb; ARRAY_LIMIT + 1];
    let mut q_arr = [0 as Limb; ARRAY_LIMIT + 1];
    let mut r_arr = [0 as Limb; ARRAY_LIMIT + 1];
    let mut m11 = [0 as Limb; ARRAY_LIMIT + 2];
    let mut m12 = [0 as Limb; ARRAY_LIMIT + 2];
    let m_limbs = m.to_limbs_asc();
    let mut a_len = m_limbs.len();
    big_a[..a_len].copy_from_slice(&m_limbs);
    let a_limbs = a.to_limbs_asc();
    let mut b_len = a_limbs.len();
    big_b[..b_len].copy_from_slice(&a_limbs);
    let n_limbs = n_bound.to_limbs_asc();
    let n_len = n_limbs.len();
    let n_lz = n_limbs.last().unwrap().leading_zeros();
    let mut m_len = 1;
    m11[0] = 1;
    let mut m_len_12 = 1;
    let mut mdet_pos = true;
    'outer: loop {
        // The half-gcd windows: apply as many two-limb reductions as possible.
        loop {
            debug_assert!(a_len > 0 && big_a[a_len - 1] > 0);
            debug_assert!(b_len > 0 && big_b[b_len - 1] > 0);
            debug_assert!(limbs_cmp(&big_a[..a_len], &big_b[..b_len]) == Ordering::Greater);
            debug_assert!(limbs_cmp(&big_b[..b_len], &n_limbs) == Ordering::Greater);
            if a_len < 3 || b_len <= n_len {
                // too small or too close to the end
                break;
            }
            let a_lz = big_a[a_len - 1].leading_zeros();
            if a_len - 1 > b_len {
                // large quotient
                break;
            }
            if a_len - 1 == n_len && n_lz < a_lz {
                // too small or too close to the end
                break;
            }
            debug_assert!(a_len == b_len || a_len - 1 == b_len);
            // zero-extend B to the length of A in the case Alen - 1 == Blen
            big_b[a_len - 1] &= if a_len - 1 == b_len { 0 } else { Limb::MAX };
            let a1 = left_shift_hi(big_a[a_len - 1], big_a[a_len - 2], a_lz);
            let a0 = left_shift_hi(big_a[a_len - 2], big_a[a_len - 3], a_lz);
            let b1 = left_shift_hi(big_b[a_len - 1], big_b[a_len - 2], a_lz);
            let b0 = left_shift_hi(big_b[a_len - 2], big_b[a_len - 3], a_lz);
            let (written, h) = hgcd_word(
                DoubleLimb::join_halves(a1, a0),
                DoubleLimb::join_halves(b1, b0),
            );
            if written == 0 {
                // difficult quotient
                break;
            }
            // (Q, R) will be the new values for (A, B)
            let (q_len, r_len) = if h.det_pos {
                (
                    fmms1(&mut q_arr, h.m22, &big_a, h.m12, &big_b, a_len),
                    fmms1(&mut r_arr, h.m11, &big_b, h.m21, &big_a, a_len),
                )
            } else {
                (
                    fmms1(&mut q_arr, h.m12, &big_b, h.m22, &big_a, a_len),
                    fmms1(&mut r_arr, h.m21, &big_a, h.m11, &big_b, a_len),
                )
            };
            debug_assert!(q_len >= r_len);
            if limbs_at_most(&q_arr[..q_len], &n_limbs) {
                // Overshot with too many quotients. FLINT annotates this "rare (impossible?)": the
                // leading-zero restriction above keeps a window from pushing A past N. An
                // instrumented run and a 300000-trial randomized search both observed zero hits;
                // the guard is retained for FLINT fidelity.
                break;
            }
            a_len = q_len;
            b_len = r_len;
            big_a = q_arr;
            big_b = r_arr;
            // multiply the first row of m by h, using r_arr as a temp
            if !h.det_pos {
                mdet_pos.not_assign();
            }
            let row_len = m_len.max(m_len_12);
            let ex0 = mul_1(&mut r_arr[..row_len], &m11[..row_len], h.m11);
            let ex1 = addmul_1(&mut r_arr[..row_len], &m12[..row_len], h.m21);
            let ex2 = mul_1_in_place(&mut m12[..row_len], h.m22);
            let ex3 = addmul_1(&mut m12[..row_len], &m11[..row_len], h.m12);
            let sum = DoubleLimb::from(ex2) + DoubleLimb::from(ex3);
            (m12[row_len + 1], m12[row_len]) = sum.split_in_half();
            m11[..row_len].copy_from_slice(&r_arr[..row_len]);
            let sum = DoubleLimb::from(ex0) + DoubleLimb::from(ex1);
            (m11[row_len + 1], m11[row_len]) = sum.split_in_half();
            m_len = row_len
                + if m11[row_len + 1] != 0 {
                    2
                } else {
                    usize::from(m11[row_len] != 0)
                };
            m_len_12 = row_len
                + if m12[row_len + 1] != 0 {
                    2
                } else {
                    usize::from(m12[row_len] != 0)
                };
            // so A > N; see if further A > N >= B
            if limbs_at_most(&big_b[..b_len], &n_limbs) {
                // got lucky
                break 'outer;
            }
        }
        // The gauss step: (A, B) = (B, A mod B) with a full division, and (m11, m12) = (m12 + Q *
        // m11, m11).
        let an = Natural::from_limbs_asc(&big_a[..a_len]);
        let bn = Natural::from_limbs_asc(&big_b[..b_len]);
        let (qn, rn) = an.div_mod(&bn);
        big_a = [0; ARRAY_LIMIT + 1];
        let b_limbs = bn.into_limbs_asc();
        a_len = b_limbs.len();
        big_a[..a_len].copy_from_slice(&b_limbs);
        big_b = [0; ARRAY_LIMIT + 1];
        let r_limbs = rn.into_limbs_asc();
        b_len = r_limbs.len();
        big_b[..b_len].copy_from_slice(&r_limbs);
        let m11n = Natural::from_limbs_asc(&m11[..m_len]);
        let new_m11 = Natural::from_limbs_asc(&m12[..m_len_12]).add_mul(&m11n, qn);
        m12 = [0; ARRAY_LIMIT + 2];
        let m11_limbs = m11n.into_limbs_asc();
        m_len_12 = m11_limbs.len();
        m12[..m_len_12].copy_from_slice(&m11_limbs);
        m11 = [0; ARRAY_LIMIT + 2];
        let new_limbs = new_m11.into_limbs_asc();
        m_len = new_limbs.len();
        m11[..m_len].copy_from_slice(&new_limbs);
        mdet_pos.not_assign();
        // see if further A > N >= B
        if limbs_at_most(&big_b[..b_len], &n_limbs) {
            break;
        }
    }
    finish(
        Natural::from_limbs_asc(&big_b[..b_len]),
        Natural::from_limbs_asc(&m11[..m_len]),
        mdet_pos,
        d_bound,
    )
}

// The outcome of a batch of Lehmer word windows: FLINT's -1, 0, and 1 returns from _lehmer.
enum LehmerOutcome {
    // The word windows can no longer help; the caller finishes with the plain loop.
    Exhausted,
    // A single full Euclidean step should be taken before trying the windows again.
    OneStepNeeded,
    // A > N >= B has been reached.
    Finished,
}

// The state of the Euclidean reduction: the pair (A, B) being reduced toward the numerator bound,
// and the first row (m11, m12) of the matrix M of accumulated quotients, whose determinant is
// always 1 or -1 and is tracked as `det_pos`. The second row of M is never needed. FLINT threads
// these five values through _lehmer, _split, and the gauss loop as separate arguments.
struct Reduction {
    big_a: Natural,
    big_b: Natural,
    m11: Natural,
    m12: Natural,
    det_pos: bool,
}

impl Reduction {
    // Begins reducing (A, B) with M the identity.
    const fn new(big_a: Natural, big_b: Natural) -> Self {
        Self {
            big_a,
            big_b,
            m11: Natural::ONE,
            m12: Natural::ZERO,
            det_pos: true,
        }
    }

    // One step of the Euclidean loop: replace (A, B) with (B, A mod B), folding the quotient into
    // the first row of the matrix M and flipping the sign of its determinant. This is the body
    // shared by the gauss, Lehmer, and split loops.
    fn gauss_step(&mut self) {
        let (q, r) = take(&mut self.big_a).div_mod(&self.big_b);
        self.m12.add_mul_assign(&self.m11, q);
        swap(&mut self.m11, &mut self.m12);
        self.det_pos.not_assign();
        self.big_a = replace(&mut self.big_b, r);
    }

    // The plain Euclidean ("gauss") loop of _fmpq_reconstruct_fmpz_2 from
    // fmpq/reconstruct_fmpz_2.c, FLINT 3.6.0, entered with A > B > N and leaving with A > N >= B.
    fn gauss_loop(&mut self, n_bound: &Natural) {
        loop {
            self.gauss_step();
            if self.big_b <= *n_bound {
                break;
            }
        }
    }

    // This is _lehmer from fmpq/reconstruct_fmpz_2.c, FLINT 3.6.0: word-window reductions of
    // arbitrary-precision operands, accumulating into the Natural matrix row.
    #[cfg_attr(dylint_lib = "malachite_lints", allow(adjacent_vec_allocations))]
    fn lehmer_step(&mut self, n_bound: &Natural) -> LehmerOutcome {
        let n_limbs = n_bound.to_limbs_asc();
        let n_len = n_limbs.len();
        let n_lz = n_limbs.last().unwrap().leading_zeros();
        let mut a = self.big_a.to_limbs_asc();
        let mut b = self.big_b.to_limbs_asc();
        let mut a_len = a.len();
        let mut b_len = b.len();
        if a_len < 3 || b_len <= n_len {
            // don't come back
            return LehmerOutcome::Exhausted;
        }
        // fit everything to A's initial length
        let capacity = a_len;
        b.resize(capacity, 0);
        // The scratch buffers are swapped with a and b as the windows apply, so merging them into
        // one allocation would force copies at every swap.
        let mut s = vec![0 as Limb; capacity];
        let mut t = vec![0 as Limb; capacity];
        let ret = loop {
            debug_assert!(a_len >= b_len && b_len >= n_len && n_len > 0);
            debug_assert!(a[a_len - 1] != 0 && b[b_len - 1] != 0);
            if a_len < 3 || b_len <= n_len {
                // too small or too close to the end
                break LehmerOutcome::Exhausted;
            }
            if a_len - 1 > b_len {
                // large quotient
                break LehmerOutcome::OneStepNeeded;
            }
            let a_lz = a[a_len - 1].leading_zeros();
            if a_len - 1 == n_len && n_lz < a_lz {
                // too small or too close to the end
                break LehmerOutcome::Exhausted;
            }
            debug_assert!(a_len == b_len || a_len - 1 == b_len);
            if a_len - 1 == b_len {
                b[a_len - 1] = 0;
            }
            let a1 = left_shift_hi(a[a_len - 1], a[a_len - 2], a_lz);
            let a0 = left_shift_hi(a[a_len - 2], a[a_len - 3], a_lz);
            let b1 = left_shift_hi(b[a_len - 1], b[a_len - 2], a_lz);
            let b0 = left_shift_hi(b[a_len - 2], b[a_len - 3], a_lz);
            let (written, h) = hgcd_word(
                DoubleLimb::join_halves(a1, a0),
                DoubleLimb::join_halves(b1, b0),
            );
            if written == 0 {
                // difficult quotient
                break LehmerOutcome::OneStepNeeded;
            }
            let (s_len, t_len) = if h.det_pos {
                (
                    fmms1(&mut s, h.m22, &a, h.m12, &b, a_len),
                    fmms1(&mut t, h.m11, &b, h.m21, &a, a_len),
                )
            } else {
                (
                    fmms1(&mut s, h.m12, &b, h.m22, &a, a_len),
                    fmms1(&mut t, h.m21, &a, h.m11, &b, a_len),
                )
            };
            // s > t >= 0
            debug_assert!(s_len >= t_len);
            if limbs_at_most(&s[..s_len], &n_limbs) {
                // Overshot with too many quotients. Believed unreachable for the same reason as the
                // array kernel's overshoot guard; retained for FLINT fidelity.
                break LehmerOutcome::OneStepNeeded;
            }
            // multiply the first row of M by h
            let new_m11 =
                (&self.m11 * Natural::from(h.m11)).add_mul(&self.m12, Natural::from(h.m21));
            self.m12 *= Natural::from(h.m22);
            self.m12.add_mul_assign(&self.m11, Natural::from(h.m12));
            self.m11 = new_m11;
            if !h.det_pos {
                self.det_pos.not_assign();
            }
            // a = s; b = t
            swap(&mut a, &mut s);
            swap(&mut b, &mut t);
            a_len = s_len;
            b_len = t_len;
            // so a > n; see if further a > n >= b
            if limbs_at_most(&b[..b_len], &n_limbs) {
                // lucky finish
                break LehmerOutcome::Finished;
            }
        };
        a.truncate(a_len);
        self.big_a = Natural::from_owned_limbs_asc(a);
        b.truncate(b_len);
        self.big_b = Natural::from_owned_limbs_asc(b);
        ret
    }

    // The `lehmer` loop of _fmpq_reconstruct_fmpz_2 from fmpq/reconstruct_fmpz_2.c, FLINT 3.6.0:
    // alternate word-window reduction batches with single full Euclidean steps. On return, either A
    // > N >= B, or B > N and the word windows can no longer help.
    fn lehmer_loop(&mut self, n_bound: &Natural) {
        loop {
            match self.lehmer_step(n_bound) {
                LehmerOutcome::Exhausted | LehmerOutcome::Finished => return,
                LehmerOutcome::OneStepNeeded => {}
            }
            self.gauss_step();
            if self.big_b <= *n_bound {
                return;
            }
        }
    }

    // This is _split from fmpq/reconstruct_fmpz_2.c, FLINT 3.6.0: reduce (A, B) toward N using the
    // subquadratic half-gcd on truncated operands, accumulating into the first matrix row. Returns
    // whether A > N >= B was reached; false means few quotients remain and the caller should
    // continue with the Lehmer loop.
    fn split_loop(&mut self, n_bound: &Natural) -> bool {
        let n_size = n_bound.limb_count();
        let mut v = Vec::new();
        loop {
            debug_assert!(self.big_a > self.big_b && self.big_b > *n_bound);
            let a_size = self.big_a.limb_count();
            let b_size = self.big_b.limb_count();
            if b_size - n_size < RECONSTRUCT_HGCD_CUTOFF {
                // relatively few remaining quotients
                return false;
            }
            let s_limbs = 1 + (n_size << 1).saturating_sub(a_size);
            // s_limbs >= b_size is unreachable: combined with the cutoff check above it would force
            // a_size <= b_size - 999, impossible for A > B. The branch mirrors FLINT's defensive
            // check.
            if s_limbs < b_size {
                let shift_bits = s_limbs << Limb::LOG_WIDTH;
                let mut a_top = &self.big_a >> shift_bits;
                let mut b_top = &self.big_b >> shift_bits;
                if a_top > b_top {
                    let mut h = Mat22::one();
                    v.clear();
                    fmpq_hgcd(&mut v, &mut h, &mut a_top, &mut b_top);
                    if !h.is_one() {
                        let q = (&self.big_a).mod_power_of_2(shift_bits);
                        let r = (&self.big_b).mod_power_of_2(shift_bits);
                        self.big_a = a_top << shift_bits;
                        self.big_b = b_top << shift_bits;
                        if h.det_pos {
                            self.big_a.add_mul_assign(&q, &h.m22);
                            self.big_a.sub_mul_assign(&r, &h.m12);
                            self.big_b.add_mul_assign(&r, &h.m11);
                            self.big_b.sub_mul_assign(&q, &h.m21);
                        } else {
                            self.big_a.add_mul_assign(&r, &h.m12);
                            self.big_a.sub_mul_assign(&q, &h.m22);
                            self.big_b.add_mul_assign(&q, &h.m21);
                            self.big_b.sub_mul_assign(&r, &h.m11);
                        }
                        // multiply the first row of M by H
                        let new_m11 = (&self.m11 * &h.m11).add_mul(&self.m12, &h.m21);
                        self.m12 *= &h.m22;
                        self.m12.add_mul_assign(&self.m11, &h.m12);
                        self.m11 = new_m11;
                        if !h.det_pos {
                            self.det_pos.not_assign();
                        }
                        while self.big_a <= *n_bound {
                            // FLINT annotates this "unlikely (impossible?)" with the above choice
                            // of the shift; an instrumented run observed zero hits. Pop a quotient.
                            let q = v.pop().unwrap();
                            self.big_b.add_mul_assign(&self.big_a, &q);
                            swap(&mut self.big_a, &mut self.big_b);
                            self.m11.sub_mul_assign(&self.m12, &q);
                            swap(&mut self.m11, &mut self.m12);
                            self.det_pos.not_assign();
                        }
                        // should have used at least one quotient!
                        debug_assert!(!v.is_empty());
                        if self.big_b > *n_bound {
                            continue;
                        }
                        // A window landing B at or below N directly; the covered exit below, one
                        // Euclidean step doing the same, is this exit's observed twin.
                        return true;
                    }
                }
            }
            // we hit a hard quotient: one plain Euclidean step
            self.gauss_step();
            if self.big_b <= *n_bound {
                return true;
            }
        }
    }

    // Checks the candidate n = (-1)^(det M) * B over d = m11 against the denominator bound.
    fn finish(self, d_bound: &Natural) -> Option<Rational> {
        finish(self.big_b, self.m11, self.det_pos, d_bound)
    }
}

// This is FMPQ_RECONSTRUCT_HGCD_CUTOFF from fmpq.h, FLINT 3.6.0: the limb gap between the operand
// and the bound above which the subquadratic splitter is used.
const RECONSTRUCT_HGCD_CUTOFF: u64 = 500;

// This is _fmpz_mat22_t and its operations from fmpq/mat22.c, FLINT 3.6.0. Throughout the half-gcd
// the entries are nonnegative, so they are [`Natural`]s, and the subtractions below all
// reconstitute earlier nonnegative values. The determinant, always 1 or -1, is tracked as
// `det_pos`.
struct Mat22 {
    m11: Natural,
    m12: Natural,
    m21: Natural,
    m22: Natural,
    det_pos: bool,
}

impl Mat22 {
    const fn one() -> Self {
        Self {
            m11: Natural::ONE,
            m12: Natural::ZERO,
            m21: Natural::ZERO,
            m22: Natural::ONE,
            det_pos: true,
        }
    }

    fn is_one(&self) -> bool {
        self.m11 == 1u32 && self.m12 == 0u32 && self.m21 == 0u32 && self.m22 == 1u32
    }

    fn bits(&self) -> u64 {
        self.m11
            .significant_bits()
            .max(self.m12.significant_bits())
            .max(self.m21.significant_bits())
            .max(self.m22.significant_bits())
    }

    // M = M * N
    fn rmul(&mut self, n: &Self) {
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
    fn rmul_word(&mut self, n: &WordMat) {
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
    fn rmul_inv_word(&mut self, n: &WordMat) {
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
    fn rmul_elem(&mut self, q: &Natural) {
        self.m12.add_mul_assign(&self.m11, q);
        self.m22.add_mul_assign(&self.m21, q);
        swap(&mut self.m11, &mut self.m12);
        swap(&mut self.m21, &mut self.m22);
        self.det_pos.not_assign();
    }

    // M = M * [q 1; 1 0]^-1 = M * [0 1; 1 -q], undoing an rmul_elem
    fn rmul_inv_elem(&mut self, q: &Natural) {
        self.m11.sub_mul_assign(&self.m12, q);
        self.m21.sub_mul_assign(&self.m22, q);
        swap(&mut self.m11, &mut self.m12);
        swap(&mut self.m21, &mut self.m22);
        self.det_pos.not_assign();
    }

    // (ya, yb) += N^-1 * (xa, xb); the results are remainders in the half-gcd, so the subtractions
    // do not go negative when performed after the additions
    fn addmul_inv_vec(&self, ya: &mut Natural, yb: &mut Natural, xa: &Natural, xb: &Natural) {
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
fn lehmer_exact(s: &mut Vec<Natural>, m: &mut Mat22, xa: &mut Natural, xb: &mut Natural) {
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
        for &q in &quotients[..written] {
            s.push(Natural::from(q));
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
fn fmpq_hgcd(s: &mut Vec<Natural>, m: &mut Mat22, xa: &mut Natural, xb: &mut Natural) {
    let mut ya = Natural::ZERO;
    let mut yb = Natural::ZERO;
    loop {
        debug_assert!(hgcd_ok(m, xa, xb));
        let k = xa.significant_bits() - m.bits();
        if k > const { RECONSTRUCT_HGCD_CUTOFF << Limb::LOG_WIDTH } {
            let km = m.bits();
            let shift = hgcd_split(&mut ya, &mut yb, xa, xb, m, km + (k >> 1));
            if shift != 0 {
                let mut n = Mat22::one();
                fmpq_hgcd(s, &mut n, &mut ya, &mut yb);
                if !n.is_one() {
                    hgcd_step(m, xa, xb, shift, &n, &mut ya, &mut yb);
                    debug_assert!(hgcd_ok(m, xa, xb));
                    let km = m.bits();
                    let shift = hgcd_split(&mut ya, &mut yb, xa, xb, m, km + 1);
                    if shift != 0 {
                        let mut n = Mat22::one();
                        fmpq_hgcd(s, &mut n, &mut ya, &mut yb);
                        if !n.is_one() {
                            hgcd_step(m, xa, xb, shift, &n, &mut ya, &mut yb);
                            debug_assert!(hgcd_ok(m, xa, xb));
                            continue;
                        }
                    }
                }
            }
        } else if k > const { Limb::WIDTH << 2 } {
            lehmer_exact(s, m, xa, xb);
        }
        // one exact Euclidean step; stop when the interval is no longer greater than one
        let (q, r) = (&*xa).div_mod(&*xb);
        m.rmul_elem(&q);
        if !hgcd_ok(m, xb, &r) {
            m.rmul_inv_elem(&q);
            return;
        }
        *xa = replace(xb, r);
        s.push(q);
    }
}

// The shared write_answer tail of _fmpq_reconstruct_fmpz_2 from fmpq/reconstruct_fmpz_2.c, FLINT
// 3.6.0. The candidate is n = (-1)^(det M) * B and d = m11.
fn finish(big_b: Natural, m11: Natural, mdet_pos: bool, d_bound: &Natural) -> Option<Rational> {
    if m11 > *d_bound || !(&big_b).coprime_with(&m11) {
        return None;
    }
    // The zero-numerator guard keeps the result canonical if B = 0 ever reaches this point. It is
    // believed unreachable: B = 0 requires m11 = 1 to pass the coprimality check, which would mean
    // the loop ran exactly one iteration with quotient 1, and then its remainder m - a would have
    // been at most N, already handled by the second fast path. An instrumented run observed zero
    // hits. It is retained as canonicality insurance, mirroring FLINT's own B == 0 handling.
    Some(Rational {
        sign: mdet_pos || big_b == 0u32,
        numerator: big_b,
        denominator: m11,
    })
}

// The balanced bounds N = D = floor(sqrt((m - 1) / 2)) of fmpq_reconstruct_fmpz: the largest
// symmetric bounds satisfying 2ND < m.
fn balanced_bound(m: &Natural) -> Natural {
    let mut b = m >> 1u32;
    if m.even() {
        b -= Natural::ONE;
    }
    b.floor_sqrt()
}

impl Rational {
    /// Reconstructs a [`Rational`] from its residue $a$ modulo $m$, subject to bounds on the
    /// numerator and denominator. The residue and modulus are taken by value and the bounds by
    /// reference.
    ///
    /// $f(a, m, N, D) = n/d$, where $n \equiv ad \pmod{m}$, $|n| \leq N$, $0 < d \leq D$, and
    /// $\gcd(n, d) = 1$.
    ///
    /// Whenever $2ND < m$, at most one fraction satisfies the constraints, and it is returned if it
    /// exists; `None` means no such fraction exists. If $2ND \geq m$ the fraction is no longer
    /// unique. In either case the function examines a single candidate, the continued-fraction
    /// approximant of $a/m$ whose remainder is the first to reach $N$ or below, and returns it
    /// exactly when it satisfies the constraints.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n (\log n)^2 \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `m.significant_bits()`: the
    /// size-dispatched kernels culminate in a subquadratic half-gcd splitter, giving the gcd-class
    /// bound.
    ///
    /// # Panics
    /// Panics if `a` is greater than or equal to `m`, or if `n_bound` or `d_bound` is zero.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::basic::traits::One;
    /// use malachite_nz::natural::Natural;
    /// use malachite_q::Rational;
    ///
    /// // 2/3 has residue 33 modulo 97, since 33 * 3 = 99 ≡ 2 mod 97
    /// assert_eq!(
    ///     Rational::reconstruct_with_bounds(
    ///         Natural::from(33u32),
    ///         Natural::from(97u32),
    ///         &Natural::from(6u32),
    ///         &Natural::from(6u32),
    ///     )
    ///     .unwrap()
    ///     .to_string(),
    ///     "2/3"
    /// );
    /// // 1/25 has residue 444 modulo 1009; denominator-heavy bounds find it...
    /// assert_eq!(
    ///     Rational::reconstruct_with_bounds(
    ///         Natural::from(444u32),
    ///         Natural::from(1009u32),
    ///         &Natural::ONE,
    ///         &Natural::from(30u32),
    ///     )
    ///     .unwrap()
    ///     .to_string(),
    ///     "1/25"
    /// );
    /// // ...but numerator-heavy bounds do not
    /// assert_eq!(
    ///     Rational::reconstruct_with_bounds(
    ///         Natural::from(444u32),
    ///         Natural::from(1009u32),
    ///         &Natural::from(30u32),
    ///         &Natural::ONE,
    ///     ),
    ///     None
    /// );
    /// ```
    ///
    /// This is fmpq_reconstruct_fmpz_2 from fmpq/reconstruct_fmpz_2.c, FLINT 3.6.0.
    #[inline]
    pub fn reconstruct_with_bounds(
        a: Natural,
        m: Natural,
        n_bound: &Natural,
        d_bound: &Natural,
    ) -> Option<Self> {
        reconstruct_helper(a, m, n_bound, d_bound)
    }

    /// Reconstructs a [`Rational`] from its residue $a$ modulo $m$, subject to bounds on the
    /// numerator and denominator. All four [`Natural`]s are taken by reference.
    ///
    /// $f(a, m, N, D) = n/d$, where $n \equiv ad \pmod{m}$, $|n| \leq N$, $0 < d \leq D$, and
    /// $\gcd(n, d) = 1$.
    ///
    /// Whenever $2ND < m$, at most one fraction satisfies the constraints, and it is returned if it
    /// exists; `None` means no such fraction exists. If $2ND \geq m$ the fraction is no longer
    /// unique. In either case the function examines a single candidate, the continued-fraction
    /// approximant of $a/m$ whose remainder is the first to reach $N$ or below, and returns it
    /// exactly when it satisfies the constraints.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n (\log n)^2 \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `m.significant_bits()`: the
    /// size-dispatched kernels culminate in a subquadratic half-gcd splitter, giving the gcd-class
    /// bound.
    ///
    /// # Panics
    /// Panics if `a` is greater than or equal to `m`, or if `n_bound` or `d_bound` is zero.
    ///
    /// # Examples
    /// ```
    /// use malachite_nz::natural::Natural;
    /// use malachite_q::Rational;
    ///
    /// // 2/3 has residue 33 modulo 97, since 33 * 3 = 99 ≡ 2 mod 97
    /// assert_eq!(
    ///     Rational::reconstruct_with_bounds_ref(
    ///         &Natural::from(33u32),
    ///         &Natural::from(97u32),
    ///         &Natural::from(6u32),
    ///         &Natural::from(6u32),
    ///     )
    ///     .unwrap()
    ///     .to_string(),
    ///     "2/3"
    /// );
    /// // no fraction with numerator and denominator at most 6 has residue 44 modulo 97
    /// assert_eq!(
    ///     Rational::reconstruct_with_bounds_ref(
    ///         &Natural::from(44u32),
    ///         &Natural::from(97u32),
    ///         &Natural::from(6u32),
    ///         &Natural::from(6u32),
    ///     ),
    ///     None
    /// );
    /// ```
    ///
    /// This is fmpq_reconstruct_fmpz_2 from fmpq/reconstruct_fmpz_2.c, FLINT 3.6.0, where all
    /// inputs are taken by reference.
    #[inline]
    pub fn reconstruct_with_bounds_ref(
        a: &Natural,
        m: &Natural,
        n_bound: &Natural,
        d_bound: &Natural,
    ) -> Option<Self> {
        reconstruct_helper(a.clone(), m.clone(), n_bound, d_bound)
    }

    /// Reconstructs a [`Rational`] from its residue $a$ modulo $m$, using balanced bounds on the
    /// numerator and denominator. Both [`Natural`]s are taken by value.
    ///
    /// $f(a, m) = n/d$, where $n \equiv ad \pmod{m}$, $|n| \leq N$, $0 < d \leq N$, and $\gcd(n, d)
    /// = 1$, with $N = \lfloor\sqrt{(m-1)/2}\rfloor$.
    ///
    /// The balanced bounds are the largest symmetric bounds satisfying $2N^2 < m$, so at most one
    /// fraction satisfies the constraints, and it is returned if it exists; `None` means no such
    /// fraction exists. To reconstruct fractions whose numerator and denominator have different
    /// sizes, use [`reconstruct_with_bounds`](Rational::reconstruct_with_bounds).
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n (\log n)^2 \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `m.significant_bits()`: the
    /// size-dispatched kernels culminate in a subquadratic half-gcd splitter, giving the gcd-class
    /// bound.
    ///
    /// # Panics
    /// Panics if `m` is less than 3 or if `a` is greater than or equal to `m`.
    ///
    /// # Examples
    /// ```
    /// use malachite_nz::natural::Natural;
    /// use malachite_q::Rational;
    ///
    /// // 2/3 has residue 33 modulo 97, since 33 * 3 = 99 ≡ 2 mod 97
    /// assert_eq!(
    ///     Rational::reconstruct(Natural::from(33u32), Natural::from(97u32))
    ///         .unwrap()
    ///         .to_string(),
    ///     "2/3"
    /// );
    /// // 1/25 has residue 444 modulo 1009, but the balanced bounds cannot see it: its
    /// // denominator outweighs its numerator
    /// assert_eq!(
    ///     Rational::reconstruct(Natural::from(444u32), Natural::from(1009u32)),
    ///     None
    /// );
    /// ```
    ///
    /// This is fmpq_reconstruct_fmpz from fmpq/reconstruct_fmpz.c, FLINT 3.6.0.
    pub fn reconstruct(a: Natural, m: Natural) -> Option<Self> {
        assert!(m > 2u32, "m must be greater than 2");
        let b = balanced_bound(&m);
        reconstruct_helper(a, m, &b, &b)
    }

    /// Reconstructs a [`Rational`] from its residue $a$ modulo $m$, using balanced bounds on the
    /// numerator and denominator. Both [`Natural`]s are taken by reference.
    ///
    /// $f(a, m) = n/d$, where $n \equiv ad \pmod{m}$, $|n| \leq N$, $0 < d \leq N$, and $\gcd(n, d)
    /// = 1$, with $N = \lfloor\sqrt{(m-1)/2}\rfloor$.
    ///
    /// The balanced bounds are the largest symmetric bounds satisfying $2N^2 < m$, so at most one
    /// fraction satisfies the constraints, and it is returned if it exists; `None` means no such
    /// fraction exists. To reconstruct fractions whose numerator and denominator have different
    /// sizes, use [`reconstruct_with_bounds_ref`](Rational::reconstruct_with_bounds_ref).
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n (\log n)^2 \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `m.significant_bits()`: the
    /// size-dispatched kernels culminate in a subquadratic half-gcd splitter, giving the gcd-class
    /// bound.
    ///
    /// # Panics
    /// Panics if `m` is less than 3 or if `a` is greater than or equal to `m`.
    ///
    /// # Examples
    /// ```
    /// use malachite_nz::natural::Natural;
    /// use malachite_q::Rational;
    /// use std::str::FromStr;
    ///
    /// // 2/3 has residue 33 modulo 97, since 33 * 3 = 99 ≡ 2 mod 97
    /// assert_eq!(
    ///     Rational::reconstruct_ref(&Natural::from(33u32), &Natural::from(97u32))
    ///         .unwrap()
    ///         .to_string(),
    ///     "2/3"
    /// );
    /// // 22/7 has residue 8818342134038800723104056361 modulo 12345678987654321012345678901
    /// assert_eq!(
    ///     Rational::reconstruct_ref(
    ///         &Natural::from_str("8818342134038800723104056361").unwrap(),
    ///         &Natural::from_str("12345678987654321012345678901").unwrap(),
    ///     )
    ///     .unwrap()
    ///     .to_string(),
    ///     "22/7"
    /// );
    /// ```
    ///
    /// This is fmpq_reconstruct_fmpz from fmpq/reconstruct_fmpz.c, FLINT 3.6.0, where all inputs
    /// are taken by reference.
    pub fn reconstruct_ref(a: &Natural, m: &Natural) -> Option<Self> {
        assert!(*m > 2u32, "m must be greater than 2");
        let b = balanced_bound(m);
        reconstruct_helper(a.clone(), m.clone(), &b, &b)
    }
}
