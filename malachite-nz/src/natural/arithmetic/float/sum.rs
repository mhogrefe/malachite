// Copyright © 2026 Mikhail Hogrefe
//
// Uses code adopted from the GNU MPFR Library.
//
//      Copyright 2014-2025 Free Software Foundation, Inc.
//
//      Contributed by the AriC and Caramba projects, INRIA.
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::natural::InnerNatural::{Large, Small};
use crate::natural::Natural;
use crate::natural::arithmetic::add::{
    limbs_slice_add_limb_in_place, limbs_slice_add_same_length_in_place_left,
};
use crate::natural::arithmetic::shl::{limbs_shl_to_out, limbs_slice_shl_in_place};
use crate::natural::arithmetic::shr::limbs_shr_to_out;
use crate::natural::arithmetic::sub::{
    limbs_sub_limb_in_place, limbs_sub_same_length_in_place_left,
};
use crate::natural::logic::not::limbs_not_in_place;
use crate::platform::Limb;
use alloc::vec;
use core::cmp::Ordering;
use malachite_base::fail_on_untested_path;
use malachite_base::num::arithmetic::traits::{
    CeilingLogBase2, NegAssign, PowerOf2, Sign, WrappingNegAssign,
};
use malachite_base::num::basic::integers::PrimitiveInt;
use malachite_base::num::conversion::traits::ExactFrom;
use malachite_base::num::logic::traits::LeadingZeros;
use malachite_base::rounding_modes::RoundingMode::{self, *};
use malachite_base::slices::{slice_set_zero, slice_test_zero};

const WIDTH: u64 = Limb::WIDTH;
const WIDTH_I64: i64 = WIDTH as i64;
const WIDTH_USIZE: usize = WIDTH as usize;
const WIDTH_M1: u64 = WIDTH - 1;
const WIDTH_P1: u64 = WIDTH + 1;

// A sentinel standing in for MPFR_EXP_MIN: an exponent smaller than that of any representable
// value, used as max(Empty) when no bits have been ignored.
const EXP_MIN: i64 = i64::MIN;

// A regular (finite, nonzero) input to the sum: its sign (true if nonnegative), its exponent (of
// the most significant bit, plus one, in the 0.m * 2^e convention), its precision, and its
// significand.
pub struct FloatSumInput<'a> {
    pub sign: bool,
    pub exp: i64,
    pub prec: u64,
    pub significand: &'a Natural,
}

impl FloatSumInput<'_> {
    fn limbs(&self) -> &[Limb] {
        match self.significand {
            Natural(Small(x)) => core::slice::from_ref(x),
            Natural(Large(xs)) => xs,
        }
    }
}

// The mask consisting of the `k` lowest bits, where `k < Limb::WIDTH`. This is MPFR_LIMB_MASK.
const fn limb_mask(k: u64) -> Limb {
    if k == 0 { 0 } else { Limb::MAX >> (WIDTH - k) }
}

// This is SAFE_SUB from sum.c, MPFR 4.2.2.
fn safe_sub(e: i64, sh: i64) -> i64 {
    assert!(e >= i64::MIN + sh);
    e - sh
}

// Accumulate a new [minexp, maxexp[ block of the inputs into the two's-complement accumulator
// `wp` (least significant limb first, `wq` bits in total). If, due to cancellation, the exponent
// of the computed result minus the exponent of the error bound is less than `prec`, shift the
// accumulator and reiterate.
//
// Returns 0 if the accumulator is 0, which implies that the exact sum for this invocation is 0;
// otherwise the number of cancelled bits (>= 1), defined as the number of identical bits on the
// most significant part of the accumulator, along with (e, minexp, maxexp): the exponent of the
// computed result, the last value of the window's least significant exponent, and the next
// iteration's block exponent. (When 0 is returned, the other three values are meaningless.)
//
// This is sum_raw from sum.c, MPFR 4.2.2, where the input array has been pre-filtered to the
// regular inputs, and the values passed back through pointers are returned instead.
#[allow(clippy::too_many_arguments)]
pub(crate) fn sum_raw(
    wp: &mut [Limb],
    wq: u64,
    xs: &[FloatSumInput],
    mut minexp: i64,
    mut maxexp: i64,
    tp: &mut [Limb],
    logn: u64,
    prec: u64,
) -> (u64, i64, i64, i64) {
    let ws = wp.len();
    assert!(prec >= 1);
    assert_eq!(wq, u64::exact_from(ws) * WIDTH);
    assert!(wq >= logn + prec + 2);
    loop {
        let mut maxexp2 = EXP_MIN;
        assert!(maxexp > minexp);
        for x in xs {
            // Step 1 (see sum_raw in sum.txt): singular inputs have been filtered out already.
            let xe = x.exp;
            let xq = x.prec;
            let x_limbs = x.limbs();
            let mut vs = x_limbs.len();
            let mut vd = xe - i64::exact_from(u64::exact_from(vs) * WIDTH) - minexp;
            // vd is the exponent of the least significant represented bit of x (including the
            // trailing bits, whose value is 0) minus the exponent of the least significant bit of
            // the accumulator. The trailing bits of x are not filtered out.
            let mut tr;
            let vp_shifted;
            let mut vp_offset = 0;
            let dp_offset;
            // Steps 2, 3, 4 (see sum_raw in sum.txt)
            if vd < 0 {
                // This covers the cases where x extends below the accumulator's least
                // significant bit.
                //
                // Step 2 for subcase vd < 0
                if xe <= minexp {
                    // x is entirely after the LSB of the accumulator, so that it will be ignored
                    // at this iteration.
                    if xe > maxexp2 {
                        maxexp2 = xe;
                    }
                    continue;
                }
                // Step 3 for subcase vd < 0: if some significant bits of x are after the LSB of
                // the accumulator, then maxexp2 will necessarily be minexp.
                if xe - i64::exact_from(xq) < minexp {
                    maxexp2 = minexp;
                }
                // Step 4 for subcase vd < 0: ignore the least |vd| significant bits of x; first,
                // whole limbs.
                vd.neg_assign();
                let vds = usize::exact_from(vd) / WIDTH_USIZE;
                vs -= vds;
                assert!(vs > 0);
                vp_offset += vds;
                vd -= i64::exact_from(u64::exact_from(vds) * WIDTH);
                assert!((0..WIDTH_I64).contains(&vd));
                tr = if xe > maxexp {
                    vs -= usize::exact_from(xe - maxexp) / WIDTH_USIZE;
                    assert!(vs > 0);
                    (xe - maxexp) % WIDTH_I64
                } else {
                    0
                };
                if vd != 0 {
                    assert!(vs <= tp.len());
                    limbs_shr_to_out(tp, &x_limbs[vp_offset..vp_offset + vs], u64::exact_from(vd));
                    vp_shifted = true;
                    vp_offset = 0;
                    tr += vd;
                    if tr >= WIDTH_I64 {
                        vs -= 1;
                        tr -= WIDTH_I64;
                    }
                    assert!(vs >= 1);
                    assert!((0..WIDTH_I64).contains(&tr));
                    if tr != 0 {
                        tp[vs - 1] &= limb_mask(WIDTH - u64::exact_from(tr));
                        tr = 0;
                    }
                } else {
                    vp_shifted = false;
                }
                dp_offset = 0;
            } else {
                // vd >= 0: this covers the cases where x is entirely within (or above) the
                // accumulator's range.
                //
                // Steps 2 and 3 for subcase vd >= 0: nothing to do.
                //
                // Step 4 for subcase vd >= 0: ignore the least vd significant bits of the
                // accumulator; first, whole limbs.
                let vds = usize::exact_from(vd) / WIDTH_USIZE;
                if vds >= ws {
                    continue;
                }
                dp_offset = vds;
                vd -= i64::exact_from(u64::exact_from(vds) * WIDTH);
                assert!((0..WIDTH_I64).contains(&vd));
                // The low part of x will have to be shifted vd bits to the left if vd != 0.
                tr = if xe > maxexp {
                    let skip = usize::exact_from(xe - maxexp) / WIDTH_USIZE;
                    if skip >= vs {
                        continue;
                    }
                    vs -= skip;
                    (xe - maxexp) % WIDTH_I64
                } else {
                    0
                };
                assert!((0..WIDTH_I64).contains(&tr) && vs > 0);
                // We need to consider the least significant vs limbs of x except the most
                // significant tr bits.
                if vd != 0 {
                    assert!(vs <= tp.len());
                    let carry = limbs_shl_to_out(
                        tp,
                        &x_limbs[vp_offset..vp_offset + vs],
                        u64::exact_from(vd),
                    );
                    tr -= vd;
                    if tr < 0 {
                        tr += WIDTH_I64;
                        assert!(vs < tp.len());
                        tp[vs] = carry;
                        vs += 1;
                    }
                    assert!((0..WIDTH_I64).contains(&tr));
                    vp_shifted = true;
                    vp_offset = 0;
                } else {
                    vp_shifted = false;
                }
            }
            let ds = ws - dp_offset;
            assert!(vs > 0 && vs <= ds);
            // We can't truncate the most significant limb of the input (in case it hasn't been
            // shifted to the temporary area). So, let's ignore it now. It will be taken into
            // account via carry propagation after the addition.
            if tr != 0 {
                vs -= 1;
            }
            let vp: &[Limb] = if vp_shifted {
                tp
            } else {
                &x_limbs[vp_offset..]
            };
            // Step 5 (see sum_raw in sum.txt)
            let dp = &mut wp[dp_offset..];
            if x.sign {
                let mut carry = Limb::from(
                    vs > 0 && limbs_slice_add_same_length_in_place_left(&mut dp[..vs], &vp[..vs]),
                );
                if tr != 0 {
                    carry += vp[vs] & limb_mask(WIDTH - u64::exact_from(tr));
                }
                if ds > vs {
                    limbs_slice_add_limb_in_place(&mut dp[vs..], carry);
                }
            } else {
                let mut borrow = Limb::from(
                    vs > 0 && limbs_sub_same_length_in_place_left(&mut dp[..vs], &vp[..vs]),
                );
                if tr != 0 {
                    borrow += vp[vs] & limb_mask(WIDTH - u64::exact_from(tr));
                }
                if ds > vs {
                    limbs_sub_limb_in_place(&mut dp[vs..], borrow);
                }
            }
        }
        // Determine the number of cancelled bits: identical bits on the most significant part of
        // the accumulator.
        let a = if wp[ws - 1] >> WIDTH_M1 != 0 {
            Limb::MAX
        } else {
            0
        };
        let mut cancel = 0;
        let mut wi = ws;
        while wi > 0 {
            let b = wp[wi - 1];
            if b == a {
                cancel += WIDTH;
                wi -= 1;
            } else {
                cancel += LeadingZeros::leading_zeros(b ^ a);
                break;
            }
        }
        if wi > 0 || a != 0 {
            // accumulator != 0
            assert!(cancel > 0);
            let e = minexp + i64::exact_from(wq - cancel);
            assert!(e >= minexp);
            let err = maxexp2.saturating_add(i64::exact_from(logn));
            // The absolute value of the truncated sum is in the binade [2^(e-1),2^e] (closed on
            // both ends due to two's complement). The error is strictly less than 2^err (and is 0
            // if maxexp2 == EXP_MIN).
            if maxexp2 == EXP_MIN || (err <= e && u64::exact_from(e - err) >= prec) {
                return (cancel, e, minexp, maxexp2);
            }
            let diffexp = if err > e { err - e } else { 0 };
            assert!(u64::exact_from(diffexp) < cancel - 2);
            let shiftq = cancel - 2 - u64::exact_from(diffexp);
            // equivalent to: minexp + wq - 2 - max(e, err)
            assert!(shiftq > 0);
            let shifts = usize::exact_from(shiftq) / WIDTH_USIZE;
            let shiftc = shiftq % WIDTH;
            // In C this is a single overlapping mpn_lshift (or mpn_copyd); here the copy and the
            // in-place shift are separate steps.
            wp.copy_within(0..ws - shifts, shifts);
            if shiftc != 0 {
                limbs_slice_shl_in_place(&mut wp[shifts..], shiftc);
            }
            slice_set_zero(&mut wp[..shifts]);
            minexp = safe_sub(minexp, i64::exact_from(shiftq));
            assert!(minexp < maxexp2);
        } else if maxexp2 == EXP_MIN {
            // accumulator = 0 and no bits have been ignored: the sum is 0
            return (0, 0, minexp, maxexp2);
        } else {
            // accumulator = 0, but some bits have been ignored: reiterate with a window just
            // below the ignored bits (the logn + 1 corresponds to cq in the main code)
            minexp = safe_sub(maxexp2, i64::exact_from(wq - (logn + 1)));
        }
        maxexp = maxexp2;
    }
}

// The result of a sum of at least 3 regular numbers: either an exact zero (whose sign is chosen
// by the caller from the rounding mode), or a regular value. The exponent may be out of range;
// the caller is responsible for the range check.
pub enum FloatSumResult {
    Zero,
    Regular {
        sign: bool,
        exp: i64,
        significand: Natural,
        o: Ordering,
    },
}

// Whether the table maker's dilemma occurs, and if so, on what kind of value. This corresponds
// to the values 0, 1, and 2 of the tmd variable in sum.c.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum Tmd {
    // The rounding can be decided from the accumulator alone.
    None,
    // The TMD occurs on a machine number.
    Machine,
    // The TMD occurs on a midpoint (a round-to-nearest halfway case).
    Midpoint,
}

// Whether `rm` rounds toward negative infinity for a value of the given sign (true if
// nonnegative). This is MPFR_IS_LIKE_RNDD from mpfr-impl.h, MPFR 4.2.2.
const fn is_like_floor(rm: RoundingMode, sign: bool) -> bool {
    match rm {
        Floor => true,
        Down => sign,
        Up => !sign,
        _ => false,
    }
}

// Whether `rm` rounds toward positive infinity for a value of the given sign. This is
// MPFR_IS_LIKE_RNDU from mpfr-impl.h, MPFR 4.2.2.
const fn is_like_ceiling(rm: RoundingMode, sign: bool) -> bool {
    match rm {
        Ceiling => true,
        Down => !sign,
        Up => sign,
        _ => false,
    }
}

// Compute the sum of at least 3 regular numbers, rounded to `prec` bits with rounding mode `rm`
// (which must not be `Exact`; the caller handles that mode). `maxexp` is the maximum exponent of
// the inputs.
//
// This is sum_aux from sum.c, MPFR 4.2.2, where the singular inputs have been filtered out by
// the caller (so rn = n), the output is returned instead of written through a pointer, and the
// final range check is left to the caller. The MPFR_RNDF branch is omitted, since Malachite has
// no faithful-rounding mode.
pub fn sum_float_significands(xs: &[FloatSumInput], prec: u64, rm: RoundingMode) -> FloatSumResult {
    let n = xs.len();
    assert!(n >= 3);
    assert_ne!(rm, Exact);
    let maxexp = xs.iter().map(|x| x.exp).max().unwrap();
    // logn = ceil(log2(rn))
    let logn = u64::exact_from(n).ceiling_log_base_2();
    assert!(logn >= 2);
    let sq = prec;
    let cq = logn + 1;
    // Determine the size of the accumulator.
    let ws = usize::exact_from((cq + sq + logn + 2).div_ceil(WIDTH));
    let wq = u64::exact_from(ws) * WIDTH;
    assert!(wq - cq - sq >= 4);
    let zs = usize::exact_from((wq - sq).div_ceil(WIDTH));
    // An input block will have up to wq - cq bits, and its shifted value (to be correctly
    // aligned) may take Limb::WIDTH - 1 additional bits.
    let ts = usize::exact_from((wq - cq + WIDTH - 1).div_ceil(WIDTH));
    // In C, the temporary area, the accumulator, and the TMD accumulator are a single
    // allocation; here the TMD accumulator is allocated separately when needed.
    let mut buf = vec![0; ts + ws];
    let (tp, wp) = buf.split_at_mut(ts);
    // Compute the first approximation with sum_raw.
    let minexp0 = safe_sub(maxexp, i64::exact_from(wq - cq));
    assert!(wq >= logn + sq + 5);
    let (cancel, mut e, minexp, maxexp) = sum_raw(wp, wq, xs, minexp0, maxexp, tp, logn, sq + 3);
    if cancel == 0 {
        // The exact sum is zero. Since not all inputs are 0, the sum is +0 except in the Floor
        // rounding mode, as specified according to the IEEE 754 rules for the addition of two
        // numbers. (The sign is chosen by the caller.)
        return FloatSumResult::Zero;
    }
    // The absolute value of the truncated sum is in the binade [2^(e-1),2^e] (closed on both
    // ends due to two's complement). The error is strictly less than 2^(maxexp + logn) (and is 0
    // if maxexp == EXP_MIN).
    //
    // u is the exponent of the ulp of the target
    let u = e - i64::exact_from(sq);
    // neg = true iff the sum is negative
    let neg = wp[ws - 1] >> WIDTH_M1 != 0;
    let sign = !neg;
    let lbit;
    let mut rbit;
    let mut inex: i8;
    let tmd;
    if u > minexp {
        // tq is the number of trailing bits
        let tq = u64::exact_from(u - minexp);
        let mut wi = usize::exact_from(tq) / WIDTH_USIZE;
        // Determine the rounding bit, which is represented.
        let td = tq % WIDTH;
        lbit = (wp[wi] >> td) & 1;
        rbit = if td >= 1 {
            (wp[wi] >> (td - 1)) & 1
        } else {
            assert!(wi >= 1);
            wp[wi - 1] >> WIDTH_M1
        };
        assert!(rbit == 0 || rbit == 1);
        (inex, tmd) = if maxexp == EXP_MIN {
            // The sum in the accumulator is exact. Determine inex: inex = 0 if the final sum is
            // exact, else 1, i.e. inex = rounding bit || sticky bit. In round to nearest, also
            // determine the rounding direction: obtained from the rounding bit possibly except
            // in halfway cases. Halfway cases are rounded toward -inf iff the last bit of the
            // truncated significand in two's complement is 0.
            let inex_exact = if rbit == 0 || (rm == Nearest && lbit == 0) {
                // We need to determine the sticky bit, either to set inex (if the rounding bit
                // is 0) or to possibly "correct" rbit (round to nearest, halfway case rounded
                // downward) from which the rounding direction will be determined.
                let mut in_ex = if td >= 2 {
                    wp[wi] & limb_mask(td - 1) != 0
                } else if td == 0 {
                    assert!(wi >= 1);
                    wi -= 1;
                    wp[wi] & limb_mask(WIDTH_M1) != 0
                } else {
                    false
                };
                if !in_ex {
                    in_ex = !slice_test_zero(&wp[..wi]);
                    if !in_ex && rbit != 0 {
                        // sticky bit = 0, rounding bit = 1, i.e. halfway case, which will be
                        // rounded downward.
                        assert_eq!(rm, Nearest);
                        in_ex = true;
                        // even rounding downward
                        rbit = 0;
                    }
                }
                i8::from(in_ex)
            } else {
                1
            };
            // We can round correctly -> no TMD.
            (inex_exact, Tmd::None)
        } else {
            // maxexp > EXP_MIN
            // We do not know whether the sum is exact.
            let d = u - maxexp.saturating_add(i64::exact_from(logn));
            // due to prec = sq + 3 in sum_raw
            assert!(d >= 3);
            let mut d = u64::exact_from(d);
            // Let's see whether the TMD occurs by looking at the d bits following the ulp bit,
            // or the d-1 bits after the rounding bit.
            //
            // First chunk after the rounding bit...
            // nbits: number of bits of the first chunk + 1 (the +1 is for the rounding bit)
            let (mut limb, mut mask, nbits) = if td == 0 {
                assert!(wi >= 1);
                wi -= 1;
                (wp[wi], limb_mask(WIDTH_M1), WIDTH)
            } else if td == 1 {
                let limb = if wi >= 1 {
                    wi -= 1;
                    wp[wi]
                } else {
                    0
                };
                (limb, Limb::MAX, WIDTH_P1)
            } else {
                (wp[wi], limb_mask(td - 1), td)
            };
            if nbits > d {
                // Some low significant bits must be ignored.
                limb >>= nbits - d;
                mask >>= nbits - d;
                d = 0;
            } else {
                d -= nbits;
            }
            limb &= mask;
            let mut t = if limb == 0 {
                if rbit == 0 {
                    Tmd::Machine
                } else if rm == Nearest {
                    Tmd::Midpoint
                } else {
                    Tmd::None
                }
            } else if limb == mask {
                limb = Limb::MAX;
                if rbit != 0 {
                    Tmd::Machine
                } else if rm == Nearest {
                    Tmd::Midpoint
                } else {
                    Tmd::None
                }
            } else {
                Tmd::None
            };
            while t != Tmd::None && d != 0 {
                if wi == 0 {
                    // The non-represented bits are 0's.
                    if limb != 0 {
                        t = Tmd::None;
                    }
                    break;
                }
                wi -= 1;
                let limb2 = wp[wi];
                if d < WIDTH {
                    let c = WIDTH - d;
                    assert!(c > 0 && c < WIDTH);
                    if (limb2 >> c) != (limb >> c) {
                        t = Tmd::None;
                    }
                    break;
                }
                if limb2 != limb {
                    t = Tmd::None;
                }
                d -= WIDTH;
            }
            (1, t)
        };
    } else {
        // u <= minexp: the exact value of the accumulator will be copied. The TMD occurs if and
        // only if there are bits still not taken into account, and if it occurs, this is
        // necessarily on a machine number.
        lbit = if u == minexp { wp[0] & 1 } else { 0 };
        rbit = 0;
        inex = i8::from(maxexp != EXP_MIN);
        tmd = if maxexp == EXP_MIN {
            Tmd::None
        } else {
            Tmd::Machine
        };
    }
    assert!(rbit == 0 || rbit == 1);
    // Here, if the final sum is known to be exact, inex = 0, otherwise inex = 1. We have a
    // truncated significand, a trailing term t such that 0 <= t < 1 ulp, and an error on the
    // trailing term bounded by t' in absolute value. Thus the error e on the truncated
    // significand satisfies -t' <= e < 1 ulp + t'. Thus one has 4 correction cases denoted by a
    // corr value between -1 and 2 depending on e, neg, rbit, and the rounding mode:
    //   -1: equivalent to nextbelow;
    //    0: the truncated significand is not corrected;
    //    1: add 1 ulp;
    //    2: add 1 ulp, then nextabove.
    let corr: i8;
    if tmd == Tmd::None {
        // no TMD
        corr = match rm {
            Floor => 0,
            Ceiling => inex,
            Down => i8::from(inex != 0 && neg),
            Up => i8::from(inex != 0 && !neg),
            Nearest => i8::exact_from(rbit),
            Exact => unreachable!(),
        };
        assert!(corr == 0 || corr == 1);
        if inex != 0 && corr == 0 {
            // two's complement significand decreased
            inex = -1;
        }
    } else {
        // TMD case. A new window, with the same meaning as minexp, is used for the secondary
        // term, as the minexp value is kept for the copy to the destination.
        assert!(maxexp > EXP_MIN);
        let mut zp = vec![0; zs];
        let zq = u64::exact_from(zs) * WIDTH;
        let err = maxexp.saturating_add(i64::exact_from(logn));
        // The d-1 bits from u-2 to u-d (= err) are identical.
        let minexp2 = if err >= minexp {
            // Let's keep the last 2 over the d-1 identical bits and the following bits, i.e. the
            // bits from err+1 to minexp.
            let tq = u64::exact_from(err - minexp) + 2;
            assert!(tq >= 2);
            let mut wi = usize::exact_from(tq) / WIDTH_USIZE;
            let td = tq % WIDTH;
            let (zz, minexp2) = if td != 0 {
                // number of words with represented bits
                wi += 1;
                let td = WIDTH - td;
                let zz = zs - wi;
                assert!(zz < zs);
                limbs_shl_to_out(&mut zp[zz..], &wp[..wi], td);
                (
                    zz,
                    safe_sub(minexp, i64::exact_from(u64::exact_from(zz) * WIDTH + td)),
                )
            } else {
                // Since err <= minexp + logn, tq = err - minexp + 2 <= logn + 2, so a
                // limb-aligned tq would require about 2^(Limb::WIDTH - 2) inputs; this branch is
                // kept for fidelity to sum.c but is unreachable in practice.
                fail_on_untested_path("sum_float_significands, TMD copy with td == 0");
                assert!(wi > 0);
                let zz = zs - wi;
                assert!(zz < zs);
                zp[zz..zz + wi].copy_from_slice(&wp[..wi]);
                (
                    zz,
                    safe_sub(minexp, i64::exact_from(u64::exact_from(zz) * WIDTH)),
                )
            };
            slice_set_zero(&mut zp[..zz]);
            assert_eq!(minexp2, err + 2 - i64::exact_from(zq));
            minexp2
        } else {
            // At least one of the identical bits is not represented, meaning that it is 0 and
            // all these bits are 0's. Thus the accumulator will be 0. The new minexp is
            // determined from maxexp, with cq bits reserved to avoid an overflow (as in the
            // early steps).
            let minexp2 = safe_sub(maxexp, i64::exact_from(zq - cq));
            assert_eq!(minexp2, err + 1 - i64::exact_from(zq));
            minexp2
        };
        // Determine the sign sst of the secondary term. In sum_raw, since the truncated sum
        // corresponding to this secondary term will be in [2^(e-1),2^e] and the error strictly
        // less than 2^err, we can stop the iterations when e - err >= 1.
        let (cancel2, ..) = sum_raw(&mut zp, zq, xs, minexp2, maxexp, tp, logn, 1);
        let sst: i8 = if cancel2 != 0 {
            if zp[zs - 1] >> WIDTH_M1 == 0 { 1 } else { -1 }
        } else if tmd == Tmd::Machine {
            0
        } else {
            // For halfway cases, let's virtually eliminate them by setting a sst equivalent to a
            // non-halfway case, which depends on the last bit of the pre-rounded result.
            assert_eq!(rm, Nearest);
            if lbit != 0 { 1 } else { -1 }
        };
        inex = if is_like_floor(rm, sign) {
            if sst != 0 { -1 } else { 0 }
        } else if is_like_ceiling(rm, sign) {
            if sst != 0 { 1 } else { 0 }
        } else {
            assert_eq!(rm, Nearest);
            if tmd == Tmd::Machine { -sst } else { sst }
        };
        corr = if tmd == Tmd::Midpoint && sst == (if rbit != 0 { -1 } else { 1 }) {
            1 - i8::exact_from(rbit)
        } else if is_like_floor(rm, sign) && sst == -1 {
            i8::exact_from(rbit) - 1
        } else if is_like_ceiling(rm, sign) && sst == 1 {
            i8::exact_from(rbit) + 1
        } else {
            i8::exact_from(rbit)
        };
    }
    // Sign handling (-> absolute value and sign), together with rounding. The most common cases
    // are corr = 0 and corr = 1 as this is necessarily the case when the TMD did not occur.
    assert!((-1..=2).contains(&corr));
    // Copy/shift the bits [max(u,minexp),e) to the most significant part of the destination, and
    // zero the least significant part (there can be one only if u < minexp).
    let sn = usize::exact_from(sq.div_ceil(WIDTH));
    let sd = u64::exact_from(sn) * WIDTH - sq;
    let sh = cancel % WIDTH;
    let mut sump = vec![0; sn];
    assert!(sd < WIDTH);
    if u > minexp {
        // Recompute the initial value of wi.
        let wi = usize::exact_from(u - minexp) / WIDTH_USIZE;
        if sh != 0 {
            let fi = usize::exact_from(e - minexp) / WIDTH_USIZE - (sn - 1);
            assert!(fi == wi || fi == wi + 1);
            limbs_shl_to_out(&mut sump, &wp[fi..fi + sn], sh);
            if fi != wi {
                sump[0] |= wp[wi] >> (WIDTH - sh);
            }
        } else {
            assert_eq!(u64::exact_from(ws - (wi + sn)) * WIDTH, cancel);
            sump.copy_from_slice(&wp[wi..wi + sn]);
        }
    } else {
        // u <= minexp
        let en = usize::exact_from(e - minexp + WIDTH_I64 - 1) / WIDTH_USIZE;
        if sh != 0 {
            limbs_shl_to_out(&mut sump[sn - en..], &wp[..en], sh);
        } else if en > 0 {
            sump[sn - en..].copy_from_slice(&wp[..en]);
        }
        slice_set_zero(&mut sump[..sn - en]);
    }
    // Take the complement if the result is negative, and at the same time, do the rounding and
    // zero the trailing bits. As this is valid only for precisions >= 2, there is special code
    // for precision 1 first.
    const HIGH_BIT: Limb = 1 << (WIDTH - 1);
    if sq == 1 {
        // precision 1
        sump[0] = HIGH_BIT;
        e += i64::from(if neg { 1 - corr } else { corr });
    } else if neg {
        // negative result with sq > 1
        assert_eq!(sump[sn - 1] >> WIDTH_M1, 0);
        // abs(x + corr) = -(x + corr) = com(x) + (1 - corr)
        if corr <= 1 {
            // Just do the correction operation on the least significant limb, then either a
            // complement or a negation on the remaining limbs, depending on the carry.
            //
            // Note: if corr = -1, so that 1 - corr = 2, the shift below can overflow to
            // corr2 = 0 when sd = Limb::WIDTH - 1. This case is taken into account below.
            let corr2 = Limb::exact_from(1 - i64::from(corr)).wrapping_shl(u32::exact_from(sd));
            sump[0] = (!(sump[0] | limb_mask(sd))).wrapping_add(corr2);
            if sump[0] < corr2 || (corr2 == 0 && corr < 0) {
                let all_zero = sn == 1 || slice_test_zero(&sump[1..]);
                if !all_zero {
                    // negate the remaining limbs (two's complement with borrow propagation)
                    let mut i = 1;
                    while sump[i] == 0 {
                        i += 1;
                    }
                    sump[i].wrapping_neg_assign();
                    limbs_not_in_place(&mut sump[i + 1..]);
                }
                if all_zero {
                    sump[sn - 1] |= HIGH_BIT;
                    e += 1;
                }
            } else if sn > 1 {
                limbs_not_in_place(&mut sump[1..]);
            }
        } else {
            // corr == 2: we want to compute com(x) - 1. A sequence of low significant bits 1 is
            // invariant; starting at the first low significant bit 0, we can do the complement.
            let corr2 = Limb::power_of_2(sd);
            let c = !(sump[0] | limb_mask(sd));
            sump[0] = c.wrapping_sub(corr2);
            let mut i = 1;
            if c == 0 {
                i += sump[1..].iter().position(|&l| l != Limb::MAX).unwrap();
                sump[i] = (!sump[i]).wrapping_sub(1);
                i += 1;
            }
            if i < sn {
                limbs_not_in_place(&mut sump[i..]);
            } else if sump[sn - 1] >> WIDTH_M1 == 0 {
                // Happens on 01111...111, whose complement is 10000...000, and com(x) - 1 is
                // 01111...111.
                sump[sn - 1] |= HIGH_BIT;
                e -= 1;
            }
        }
    } else {
        // positive result with sq > 1
        assert!(sump[sn - 1] >> WIDTH_M1 != 0);
        sump[0] &= !limb_mask(sd);
        if corr > 0 {
            // If corr == 2 && sd == WIDTH - 1, this overflows to corr2 = 0. This case is taken
            // into account below.
            let corr2 = Limb::exact_from(u8::exact_from(corr)) << sd;
            let carry_out = if corr2 != 0 {
                limbs_slice_add_limb_in_place(&mut sump, corr2)
            } else {
                assert!(sn > 1);
                limbs_slice_add_limb_in_place(&mut sump[1..], 1)
            };
            assert_eq!(sump[sn - 1] >> WIDTH_M1 != 0, !carry_out);
            if carry_out {
                sump[sn - 1] |= HIGH_BIT;
                e += 1;
            }
        }
        if corr < 0 {
            limbs_sub_limb_in_place(&mut sump, Limb::power_of_2(sd));
            if sump[sn - 1] >> WIDTH_M1 == 0 {
                sump[sn - 1] |= HIGH_BIT;
                e -= 1;
            }
        }
    }
    assert!(sump[sn - 1] >> WIDTH_M1 != 0);
    FloatSumResult::Regular {
        sign,
        exp: e,
        significand: Natural::from_owned_limbs_asc(sump),
        o: inex.sign(),
    }
}
