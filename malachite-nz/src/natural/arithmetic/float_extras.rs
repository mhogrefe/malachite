// Copyright © 2026 Mikhail Hogrefe
//
// Uses code adopted from the GNU MPFR Library.
//
//      Copyright © 1999-2022 Free Software Foundation, Inc.
//
//      Contributed by the AriC and Caramba projects, INRIA.
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::natural::InnerNatural::{Large, Small};
use crate::natural::arithmetic::add::{limbs_add_limb_to_out, limbs_slice_add_limb_in_place};
use crate::natural::arithmetic::mul::limb::limbs_mul_limb_to_out;
use crate::natural::arithmetic::shl::limbs_shl_to_out;
use crate::natural::arithmetic::square::{limbs_square_to_out, limbs_square_to_out_scratch_len};
use crate::natural::{LIMB_HIGH_BIT, Natural, bit_to_limb_count_floor, limb_to_bit_count};
use crate::platform::{DoubleLimb, Limb};
use alloc::vec::Vec;
use core::cmp::min;
use malachite_base::fail_on_untested_path;
use malachite_base::num::arithmetic::traits::{NegModPowerOf2, PowerOf2, WrappingSubAssign};
use malachite_base::num::basic::integers::PrimitiveInt;
use malachite_base::num::conversion::traits::ExactFrom;
use malachite_base::num::logic::traits::LowMask;
use malachite_base::rounding_modes::RoundingMode::{self, *};
use malachite_base::slices::slice_test_zero;

// This is MPFR_CAN_ROUND from mpfr-impl.h, MPFR 4.2.0.
pub fn float_can_round(x: &Natural, err0: u64, prec: u64, rm: RoundingMode) -> bool {
    match x {
        Natural(Small(small)) => limb_float_can_round(*small, err0, prec, rm),
        Natural(Large(xs)) => limbs_float_can_round(xs, err0, prec, rm),
    }
}

pub(crate) fn limb_float_can_round(x: Limb, err0: u64, mut prec: u64, rm: RoundingMode) -> bool {
    if rm == Nearest {
        prec += 1;
    }
    assert!(x.get_highest_bit());
    let err = min(err0, u64::power_of_2(Limb::LOG_WIDTH));
    if err <= prec {
        return false;
    }
    let mut s = Limb::WIDTH - (prec & Limb::WIDTH_MASK);
    let n = bit_to_limb_count_floor(err);
    // Check first limb
    let mask = Limb::low_mask(s);
    let mut tmp = x & mask;
    s = Limb::WIDTH - (err & Limb::WIDTH_MASK);
    if n == 0 {
        // prec and error are in the same limb
        assert!(s < Limb::WIDTH);
        tmp >>= s;
        tmp != 0 && tmp != mask >> s
    } else if tmp == 0 {
        // Check if error limb is 0
        s != Limb::WIDTH && x >> s != 0
    } else if tmp == mask {
        // Check if error limb is 0
        s != Limb::WIDTH && x >> s != Limb::MAX >> s
    } else {
        // limb is different from 000000 or 1111111
        true
    }
}

pub fn limbs_float_can_round(xs: &[Limb], err0: u64, mut prec: u64, rm: RoundingMode) -> bool {
    if rm == Nearest {
        prec += 1;
    }
    let len = xs.len();
    assert!(xs[len - 1].get_highest_bit());
    let err = min(err0, limb_to_bit_count(len));
    if err <= prec {
        return false;
    }
    let k = bit_to_limb_count_floor(prec);
    let mut s = Limb::WIDTH - (prec & Limb::WIDTH_MASK);
    let n = bit_to_limb_count_floor(err) - k;
    assert!(len > k);
    // Check first limb
    let mut i = len - k - 1;
    let mask = Limb::low_mask(s);
    let mut tmp = xs[i] & mask;
    i.wrapping_sub_assign(1);
    if n == 0 {
        // prec and error are in the same limb
        s = Limb::WIDTH - (err & Limb::WIDTH_MASK);
        assert!(s < Limb::WIDTH);
        tmp >>= s;
        tmp != 0 && tmp != mask >> s
    } else if tmp == 0 {
        // Check if all (n - 1) limbs are 0
        let j = i.wrapping_add(2) - n;
        if n > 1 && xs[j..=i].iter().any(|&x| x != 0) {
            return true;
        }
        // Check if final error limb is 0
        s = Limb::WIDTH - (err & Limb::WIDTH_MASK);
        s != Limb::WIDTH && xs[j - 1] >> s != 0
    } else if tmp == mask {
        // Check if all (n - 1) limbs are 11111111111111111
        let j = i.wrapping_add(2) - n;
        if n > 1 && xs[j..=i].iter().any(|&x| x != Limb::MAX) {
            return true;
        }
        // Check if final error limb is 0
        s = Limb::WIDTH - (err & Limb::WIDTH_MASK);
        s != Limb::WIDTH && xs[j - 1] >> s != Limb::MAX >> s
    } else {
        // First limb is different from 000000 or 1111111
        true
    }
}

// Given the significand `xs` of a nonzero finite `Float` (little-endian limbs, with the most
// significant bit of the most significant limb set), returns `Some(j)` if the significand's bits
// form a run of `j` ones followed by all zeros (that is, the mantissa equals $2^j - 1$), and `None`
// otherwise.
//
// This detects inputs `x` for which $1+x$ is an exact power of 2: combined with the exponent, a
// significand of the form $2^j - 1$ means the value is $2^e - 2^{e-j}$, which equals $2^k - 1$ (for
// `x` positive, when $e = j$, giving $k = j$) or $1 - 2^{-j}$ (for `x` in $(-1, 0)$, when $e = 0$,
// giving $k = -j$).
pub fn limbs_float_significand_leading_ones(xs: &[Limb]) -> Option<u64> {
    let mut i = xs.len();
    let mut count = 0;
    // Skip the all-ones limbs at the top.
    while i > 0 && xs[i - 1] == Limb::MAX {
        count += Limb::WIDTH;
        i -= 1;
    }
    if i == 0 {
        return Some(count);
    }
    // The transition limb (not all ones): it must be a run of ones followed by zeros.
    let m = xs[i - 1];
    let j = m.leading_ones();
    if m << j != 0 {
        // A one-bit appears below the leading run of ones.
        return None;
    }
    count += u64::from(j);
    // Every remaining lower limb must be zero.
    if slice_test_zero(&xs[..i - 1]) {
        Some(count)
    } else {
        None
    }
}

// Given the significand `x` of a nonzero finite `Float`, returns `Some(j)` if the mantissa equals
// $2^j - 1$ (a run of ones followed by all zeros), and `None` otherwise. See
// [`limbs_float_significand_leading_ones`].
pub fn float_significand_leading_ones(x: &Natural) -> Option<u64> {
    match x {
        Natural(Small(small)) => limbs_float_significand_leading_ones(core::slice::from_ref(small)),
        Natural(Large(xs)) => limbs_float_significand_leading_ones(xs),
    }
}

const WIDTH_M1_MASK: Limb = Limb::MAX >> 1;
pub(crate) const MPFR_EVEN_INEX: i8 = 2;

// This is MPFR_RNDRAW_EVEN from mpfr-impl.h, MPFR 4.2.0, returning `inexact` and a `bool`
// signifying whether the returned exponent should be incremented.
pub(crate) fn round_helper_even(
    out: &mut [Limb],
    out_prec: u64,
    xs: &[Limb],
    x_prec: u64,
    rm: RoundingMode,
) -> (i8, bool) {
    round_helper(out, out_prec, xs, x_prec, rm, |out, xs_hi, ulp| {
        let ulp_mask = !(ulp - 1);
        if xs_hi[0] & ulp == 0 {
            out.copy_from_slice(xs_hi);
            out[0] &= ulp_mask;
            (-MPFR_EVEN_INEX, false)
        } else {
            let increment = limbs_add_limb_to_out(out, xs_hi, ulp);
            if increment {
                *out.last_mut().unwrap() = LIMB_HIGH_BIT;
            }
            out[0] &= ulp_mask;
            (MPFR_EVEN_INEX, increment)
        }
    })
}

// This is MPFR_RNDRAW and mpfr_round_raw from mpfr-impl.h, MPFR 4.2.0, returning `inexact` and a
// `bool` signifying whether the returned exponent should be incremented.
pub fn round_helper_raw(
    out: &mut [Limb],
    out_prec: u64,
    xs: &[Limb],
    x_prec: u64,
    rm: RoundingMode,
) -> (i8, bool) {
    round_helper(out, out_prec, xs, x_prec, rm, |out, xs_hi, ulp| {
        let ulp_mask = !(ulp - 1);
        if xs_hi[0] & ulp == 0 {
            out.copy_from_slice(xs_hi);
            out[0] &= ulp_mask;
            (-1, false)
        } else {
            let increment = limbs_add_limb_to_out(out, xs_hi, ulp);
            if increment {
                *out.last_mut().unwrap() = LIMB_HIGH_BIT;
            }
            out[0] &= ulp_mask;
            (1, increment)
        }
    })
}

// This is MPFR_RNDRAW_GEN from mpfr-impl.h, MPFR 4.2.0, returning `inexact` and a `bool` signifying
// whether the returned exponent should be incremented.
fn round_helper<F: Fn(&mut [Limb], &[Limb], Limb) -> (i8, bool)>(
    out: &mut [Limb],
    out_prec: u64,
    xs: &[Limb],
    x_prec: u64,
    rm: RoundingMode,
    middle_handler: F,
) -> (i8, bool) {
    let xs_len = xs.len();
    let out_len = out.len();
    // Check trivial case when out mantissa has more bits than source
    if out_prec >= x_prec {
        out[out_len - xs_len..].copy_from_slice(xs);
        (0, false)
    } else {
        // - Nontrivial case: rounding needed
        // - Compute position and shift
        let shift = out_prec.neg_mod_power_of_2(Limb::LOG_WIDTH);
        let i = xs_len.checked_sub(out_len).unwrap();
        let mut sticky_bit;
        let round_bit;
        // General case when prec % Limb::WIDTH != 0
        let ulp = if shift != 0 {
            // Compute rounding bit and sticky bit
            //
            // Note: in directed rounding modes, if the rounding bit is 1, the behavior does not
            // depend on the sticky bit; thus we will not try to compute it in this case (this can
            // be much faster and avoids reading uninitialized data in the current mpfr_mul
            // implementation). We just make sure that sticky_bit is initialized.
            let mask = Limb::power_of_2(shift - 1);
            let x = xs[i];
            round_bit = x & mask;
            sticky_bit = x & (mask - 1);
            if rm == Nearest || round_bit == 0 {
                let mut to = i;
                let mut n = xs_len - out_len;
                while n != 0 && sticky_bit == 0 {
                    to -= 1;
                    sticky_bit = xs[to];
                    n -= 1;
                }
            }
            mask << 1
        } else {
            assert!(out_len < xs_len);
            // Compute rounding bit and sticky bit - see note above
            let x = xs[i - 1];
            round_bit = x & LIMB_HIGH_BIT;
            sticky_bit = x & WIDTH_M1_MASK;
            if rm == Nearest || round_bit == 0 {
                let mut to = i - 1;
                let mut n = xs_len - out_len - 1;
                while n != 0 && sticky_bit == 0 {
                    to -= 1;
                    sticky_bit = xs[to];
                    n -= 1;
                }
            }
            1
        };
        let xs_hi = &xs[i..];
        let ulp_mask = !(ulp - 1);
        match rm {
            Floor | Down | Exact => {
                out.copy_from_slice(xs_hi);
                out[0] &= ulp_mask;
                (if sticky_bit | round_bit != 0 { -1 } else { 0 }, false)
            }
            Ceiling | Up => {
                if sticky_bit | round_bit == 0 {
                    out.copy_from_slice(xs_hi);
                    out[0] &= ulp_mask;
                    (0, false)
                } else {
                    let increment = limbs_add_limb_to_out(out, xs_hi, ulp);
                    if increment {
                        out[out_len - 1] = LIMB_HIGH_BIT;
                    }
                    out[0] &= ulp_mask;
                    (1, increment)
                }
            }
            Nearest => {
                if round_bit == 0 {
                    out.copy_from_slice(xs_hi);
                    out[0] &= ulp_mask;
                    (if (sticky_bit | round_bit) != 0 { -1 } else { 0 }, false)
                } else if sticky_bit == 0 {
                    middle_handler(out, xs_hi, ulp)
                } else {
                    let increment = limbs_add_limb_to_out(out, xs_hi, ulp);
                    if increment {
                        out[out_len - 1] = LIMB_HIGH_BIT;
                    }
                    out[0] &= ulp_mask;
                    (1, increment)
                }
            }
        }
    }
}

// Assuming xs is an approximation of a non-singular number with error at most equal to 2 ^ (EXP(x)
// - err0) (`err0` bits of x are known) of direction unknown, check if we can round x toward zero
// with precision prec.
//
// This is mpfr_round_p from round_p.c, MPFR 4.2.0.
pub(crate) fn round_helper_2(xs: &[Limb], err0: i32, prec: u64) -> bool {
    let len = xs.len();
    assert!(xs.last().unwrap().get_highest_bit());
    let mut err = limb_to_bit_count(len);
    if err0 <= 0 {
        return false;
    }
    let err0 = u64::from(err0.unsigned_abs());
    if err0 <= prec || prec >= err {
        return false;
    }
    err = min(err, err0);
    let k = bit_to_limb_count_floor(prec);
    let n = bit_to_limb_count_floor(err) - k;
    assert!(len > k);
    // Check first limb
    let xs = &xs[len - k - n - 1..];
    let (xs_last, xs_init) = xs[..=n].split_last().unwrap();
    let mut tmp = *xs_last;
    let mask = Limb::MAX >> (prec & Limb::WIDTH_MASK);
    tmp &= mask;
    if n == 0 {
        // prec and error are in the same limb
        let s = Limb::WIDTH - (err & Limb::WIDTH_MASK);
        assert!(s < Limb::WIDTH);
        tmp >>= s;
        tmp != 0 && tmp != mask >> s
    } else if tmp == 0 {
        let (xs_head, xs_tail) = xs_init.split_first().unwrap();
        // Check if all (n - 1) limbs are 0
        if !slice_test_zero(xs_tail) {
            return true;
        }
        // Check if final error limb is 0
        let s = Limb::WIDTH - (err & Limb::WIDTH_MASK);
        s != Limb::WIDTH && *xs_head >> s != 0
    } else if tmp == mask {
        let (xs_head, xs_tail) = xs_init.split_first().unwrap();
        // Check if all (n - 1) limbs are 11111111111111111
        if xs_tail.iter().any(|&x| x != Limb::MAX) {
            return true;
        }
        // Check if final error limb is 0
        let s = Limb::WIDTH - (err & Limb::WIDTH_MASK);
        s != Limb::WIDTH && *xs_head >> s != Limb::MAX >> s
    } else {
        // First limb is different from 000000 or 1111111
        true
    }
}

#[inline]
pub fn limbs_significand_slice_add_limb_in_place(xs: &mut [Limb], y: Limb) -> bool {
    limbs_slice_add_limb_in_place(xs, y)
}

// Computes an approximation to `b ^ e` in `{a, n}`, where `n` is `a.len()`, returning the pair
// `(exp, err)`. The computed value is rounded toward zero (truncated), and `a * 2 ^ exp` represents
// it, where `a` is the integer `a[0] + a[1] * B + ... + a[n - 1] * B ^ (n - 1)` with
// `B = 2 ^ Limb::WIDTH`.
//
// `err` is an integer `f` such that the final error is bounded by `2 ^ f` ulps; that is,
// `a * 2 ^ exp <= b ^ e <= 2 ^ exp * (a + 2 ^ f)`. `err` is -1 if the result is exact, or -2 if an
// overflow occurred while computing `exp`.
//
// `n` must be positive, `e` must be positive, and `b` must be between 2 and 62, inclusive.
//
// This is equivalent to `mpfr_mpn_exp` from `mpn_exp.c`, MPFR 4.x.
pub fn limbs_float_exp(a: &mut [Limb], b: u64, e: i64) -> (i64, i32) {
    // Shifts the `n`-limb value in `c[n..2 * n]` left by one bit into `a`, bringing in the top bit
    // of `c[n - 1]`, and decrements the exponent `f` to match.
    fn shift_a_left_one_bit(a: &mut [Limb], c: &[Limb], n: usize, f: &mut i64) {
        limbs_shl_to_out(a, &c[n..2 * n], 1);
        a[0] |= Limb::from(c[n - 1].get_highest_bit());
        *f -= 1;
    }

    let n = a.len();
    assert!(n > 0);
    assert!(e > 0);
    assert!((2..=62).contains(&b));
    let width = i64::exact_from(Limb::WIDTH);
    let n_width = i64::exact_from(n) * width;
    // Normalize the base.
    let mut big_b = Limb::exact_from(b);
    let mut h = i64::from(big_b.leading_zeros());
    big_b <<= h;
    h = -h;
    // Allocate space for the running square or product (and a scratch buffer large enough for any of
    // the squarings below), and set A to B.
    let mut c: Vec<Limb> = vec![0; 2 * n];
    let mut square_scratch = vec![0; limbs_square_to_out_scratch_len(n)];
    a[n - 1] = big_b;
    a[..n - 1].fill(0);
    // The initial exponent for A; the invariant is A = {a, n} * 2 ^ f.
    let mut f = h - i64::exact_from(n - 1) * width;
    // The number of bits in e.
    let t = i32::exact_from(i64::WIDTH - u64::from(e.leading_zeros()));
    // `error == t` means that the result is still exact.
    let mut error = t;
    let mut err_s_a2: i32 = 0; // number of left shifts when squaring after the first inexact loop
    let mut err_s_ab: i32 = 0; // number of left shifts when multiplying after the first inexact loop
    for i in (0..=t - 2).rev() {
        // n1 is the number of zero low limbs of {a, n} (that is, mpn_scan1(a, 0) / Limb::WIDTH).
        let n1 = a.iter().take_while(|&&x| x == 0).count();
        // Square of A: {c + 2 * n1, 2 * (n - n1)} = {a + n1, n - n1} ^ 2.
        limbs_square_to_out(
            &mut c[2 * n1..2 * n],
            &a[n1..n],
            &mut square_scratch[..limbs_square_to_out_scratch_len(n - n1)],
        );
        // Check for overflow on f.
        if !(i64::MIN / 2..=i64::MAX / 2).contains(&f) {
            return (f, -2);
        }
        f *= 2;
        if let Some(g) = f.checked_add(n_width) {
            f = g;
        } else {
            // Reachable only when `f` lands within `Limb::WIDTH / 2` below `i64::MAX / 2`, so that
            // doubling and adding `n * Limb::WIDTH` overflows without the check above catching it
            // first. Every overflow found by testing is caught by that check instead, so this arm
            // is untested.
            fail_on_untested_path("limbs_float_exp, f overflow in checked_add");
            return (f, -2);
        }
        if c[2 * n - 1].get_highest_bit() {
            a.copy_from_slice(&c[n..2 * n]);
        } else {
            shift_a_left_one_bit(a, &c, n, &mut f);
            if error != t {
                err_s_a2 += 1;
            }
        }
        if error == t && 2 * n1 <= n && c[2 * n1..n].iter().any(|&x| x != 0) {
            error = i;
        }
        if (e >> i) & 1 == 1 {
            // Multiply A by B.
            let carry =
                limbs_mul_limb_to_out::<DoubleLimb, Limb>(&mut c[n - 1..2 * n - 1], a, big_b);
            c[2 * n - 1] = carry;
            f += h + width;
            if c[2 * n - 1].get_highest_bit() {
                a.copy_from_slice(&c[n..2 * n]);
                if error != t {
                    err_s_ab += 1;
                }
            } else {
                shift_a_left_one_bit(a, &c, n, &mut f);
            }
            if error == t && c[n - 1] != 0 {
                error = i;
            }
        }
    }
    if error == t {
        (f, -1) // the result is exact
    } else {
        (f, error + err_s_ab + err_s_a2 / 2 + 3)
    }
}
