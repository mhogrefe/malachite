// Copyright © 2026 Mikhail Hogrefe
//
// Uses code adopted from the FLINT Library.
//
//      Copyright © 2024 Fredrik Johansson
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::integer::Integer;
use crate::integer_polynomial::arithmetic::vec::small_value;
use crate::natural::Natural;
use crate::natural::arithmetic::add::{
    limbs_add_to_out_aliased, limbs_slice_add_greater_in_place_left, limbs_slice_add_limb_in_place,
};
use crate::natural::arithmetic::add_mul::limbs_slice_add_mul_limb_same_length_in_place_left;
use crate::natural::arithmetic::mul::limb::limbs_mul_limb_to_out;
use crate::natural::arithmetic::mul::{
    limbs_mul_greater_to_out, limbs_mul_greater_to_out_scratch_len,
};
use crate::natural::arithmetic::square::{limbs_square_to_out, limbs_square_to_out_scratch_len};
use crate::natural::arithmetic::sub::limbs_sub_greater_to_out;
use crate::natural::comparison::cmp::limbs_cmp_same_length;
use crate::platform::{DoubleLimb, Limb, SignedDoubleLimb};
use alloc::vec;
use alloc::vec::Vec;
use core::cmp::Ordering::*;
use core::mem::swap;
use core::ptr;
use malachite_base::num::arithmetic::traits::{
    AddMulAssign, SubMulAssign, WrappingAddAssign, XXXAddYYYToZZZ, XXXSubYYYToZZZ,
};
use malachite_base::num::basic::integers::PrimitiveInt;
use malachite_base::num::basic::traits::Zero;
use malachite_base::num::conversion::traits::{SplitInHalf, WrappingFrom};

// Sets `(s, sn)` to `(s, sn) + (b, bn)`, where `s` has room for one more limb than the longer of
// the two. Allows `sn` to be 0, in which case the buffers are swapped rather than copied.
//
// This is equivalent to `MPN_ADD` from `fmpz_vec/dot.c`, FLINT 3.6.0, with the output the same as
// the first input.
fn mpn_add(s: &mut Vec<Limb>, sn: &mut usize, b: &mut Vec<Limb>, bn: usize) {
    if *sn == 0 {
        swap(s, b);
        *sn = bn;
    } else if *sn >= bn {
        if s.len() <= *sn {
            s.resize(*sn + 1, 0);
        }
        let carry = limbs_slice_add_greater_in_place_left(&mut s[..*sn], &b[..bn]);
        s[*sn] = Limb::from(carry);
        *sn += usize::from(carry);
    } else {
        if s.len() <= bn {
            s.resize(bn + 1, 0);
        }
        let carry = limbs_add_to_out_aliased(&mut s[..bn], *sn, &b[..bn]);
        s[bn] = Limb::from(carry);
        *sn = bn + usize::from(carry);
    }
}

// Sets `(s, sn)` to `(s, sn) + a * b`, where `a` is nonempty. Allows `sn` to be 0.
//
// This is equivalent to `MPN_ADDMUL_1` from `fmpz_vec/dot.c`, FLINT 3.6.0.
fn mpn_addmul_1(s: &mut Vec<Limb>, sn: &mut usize, a: &[Limb], b: Limb) {
    let an = a.len();
    if s.len() <= (*sn).max(an) {
        s.resize((*sn).max(an) + 1, 0);
    }
    if *sn >= an {
        let mut carry = limbs_slice_add_mul_limb_same_length_in_place_left(&mut s[..an], a, b);
        if *sn > an {
            carry = Limb::from(limbs_slice_add_limb_in_place(&mut s[an..*sn], carry));
        }
        s[*sn] = carry;
        *sn += usize::from(carry != 0);
    } else {
        s[an] = limbs_mul_limb_to_out::<DoubleLimb, Limb>(&mut s[*sn..an], &a[*sn..], b);
        if *sn != 0 {
            let carry =
                limbs_slice_add_mul_limb_same_length_in_place_left(&mut s[..*sn], &a[..*sn], b);
            let carry = limbs_slice_add_limb_in_place(&mut s[*sn..an], carry);
            s[an].wrapping_add_assign(Limb::from(carry));
        }
        *sn = an + usize::from(s[an] != 0);
    }
}

// Strips the zero limbs from the top of `(xs, xn)`.
//
// This is equivalent to `MPN_NORM` from `gmp-impl.h`.
const fn mpn_norm(xs: &[Limb], xn: &mut usize) {
    while *xn != 0 && xs[*xn - 1] == 0 {
        *xn -= 1;
    }
}

// The `Integer` whose two's complement representation is the three limbs `(x2, x1, x0)`.
//
// This is equivalent to `fmpz_set_signed_uiuiui` from `fmpz.h`, FLINT 3.6.0.
fn integer_from_signed_triple(x2: Limb, x1: Limb, x0: Limb) -> Integer {
    if x2.get_highest_bit() {
        let (y2, y1, y0) = Limb::xxx_sub_yyy_to_zzz(0, 0, 0, x2, x1, x0);
        -Integer::from(Natural::from_owned_limbs_asc(vec![y0, y1, y2]))
    } else {
        Integer::from(Natural::from_owned_limbs_asc(vec![x0, x1, x2]))
    }
}

// The `Integer` with absolute value `(xs, xn)`, negated if `negative`.
//
// This is equivalent to `_fmpz_set_mpn` from `fmpz_vec/dot.c`, FLINT 3.6.0.
fn integer_from_limbs(xs: &[Limb], xn: usize, negative: bool) -> Integer {
    Integer::from_sign_and_abs(!negative, Natural::from_limbs_asc(&xs[..xn]))
}

// Returns `initial` plus, or if `subtract` minus, the dot product of `xs` and `ys`, which must have
// the same length; if `reverse`, `ys` is read backwards. Products of two small elements are summed
// in a three-limb signed accumulator, and the other products are summed separately by sign, so that
// the main loop never subtracts; the three sums are combined at the end.
//
// # Worst-case complexity
// $T(n, m) = O(nm \log m \log\log m)$
//
// $M(m) = O(m \log m)$
//
// where $T$ is time, $M$ is additional memory, $n$ is `xs.len()`, and $m$ is the largest number of
// significant bits of `initial` and of any element of `xs` or `ys`.
//
// This is equivalent to `_fmpz_vec_dot_general` from `fmpz_vec/dot.c`, FLINT 3.6.0, where `initial`
// being `None` corresponds to a null `initial`.
crate_test_fn! {vec_dot_general(
    initial: Option<&Integer>,
    subtract: bool,
    xs: &[Integer],
    ys: &[Integer],
    reverse: bool,
) -> Integer {
    let len = xs.len();
    assert_eq!(ys.len(), len);
    if len <= 1 {
        return if let Some(initial) = initial {
            let mut result = initial.clone();
            if len == 1 {
                if subtract {
                    result.sub_mul_assign(&xs[0], &ys[0]);
                } else {
                    result.add_mul_assign(&xs[0], &ys[0]);
                }
            }
            result
        } else if len == 1 {
            let product = &xs[0] * &ys[0];
            if subtract { -product } else { product }
        } else {
            Integer::ZERO
        };
    }
    let mut s0: Limb = 0;
    let mut s1: Limb = 0;
    let mut s2: Limb = 0;
    let mut neg: Vec<Limb> = Vec::new();
    let mut pos: Vec<Limb> = Vec::new();
    let mut posn = 0;
    let mut negn = 0;
    let mut t: Vec<Limb> = Vec::new();
    // Scratch space for the products, reused and grown as needed.
    let mut scratch: Vec<Limb> = Vec::new();
    if let Some(initial) = initial {
        let ap = initial.abs.as_limbs_asc();
        let an = ap.len();
        let aneg = !initial.sign;
        if an <= 2 {
            s0 = ap.first().copied().unwrap_or(0);
            if an == 2 {
                s1 = ap[1];
            }
            if aneg ^ subtract {
                (s2, s1, s0) = Limb::xxx_sub_yyy_to_zzz(0, 0, 0, 0, s1, s0);
            }
        } else if aneg ^ subtract {
            neg = ap.to_vec();
            negn = an;
        } else {
            pos = ap.to_vec();
            posn = an;
        }
    }
    for i in 0..len {
        let ca = &xs[i];
        if *ca == 0u32 {
            continue;
        }
        let cb = if reverse { &ys[len - 1 - i] } else { &ys[i] };
        if *cb == 0u32 {
            continue;
        }
        if let (Some(a), Some(b)) = (small_value(ca), small_value(cb)) {
            let product = SignedDoubleLimb::from(a) * SignedDoubleLimb::from(b);
            let (hi, lo) = DoubleLimb::wrapping_from(product).split_in_half();
            let extension = if product < 0 { Limb::MAX } else { 0 };
            (s2, s1, s0) = Limb::xxx_add_yyy_to_zzz(s2, s1, s0, extension, hi, lo);
            continue;
        }
        let mut ap = ca.abs.as_limbs_asc();
        let mut aneg = !ca.sign;
        let mut bp = cb.abs.as_limbs_asc();
        let mut bneg = !cb.sign;
        if ap.len() < bp.len() {
            swap(&mut ap, &mut bp);
            swap(&mut aneg, &mut bneg);
        }
        let an = ap.len();
        let bn = bp.len();
        if bn == 1 {
            let b0 = bp[0];
            if aneg ^ bneg {
                mpn_addmul_1(&mut neg, &mut negn, ap, b0);
            } else {
                mpn_addmul_1(&mut pos, &mut posn, ap, b0);
            }
            continue;
        }
        let mut tn = an + bn;
        if t.len() < tn {
            t.resize(tn, 0);
        }
        let carry = if ptr::eq(ap.as_ptr(), bp.as_ptr()) && an == bn {
            let scratch_len = limbs_square_to_out_scratch_len(an);
            if scratch.len() < scratch_len {
                scratch.resize(scratch_len, 0);
            }
            limbs_square_to_out(&mut t[..tn], ap, &mut scratch[..scratch_len]);
            t[tn - 1]
        } else {
            let scratch_len = limbs_mul_greater_to_out_scratch_len(an, bn);
            if scratch.len() < scratch_len {
                scratch.resize(scratch_len, 0);
            }
            limbs_mul_greater_to_out(&mut t[..tn], ap, bp, &mut scratch[..scratch_len])
        };
        tn -= usize::from(carry == 0);
        if aneg ^ bneg {
            mpn_add(&mut neg, &mut negn, &mut t, tn);
        } else {
            mpn_add(&mut pos, &mut posn, &mut t, tn);
        }
    }
    // There are only small terms.
    if posn == 0 && negn == 0 {
        if subtract {
            (s2, s1, s0) = Limb::xxx_sub_yyy_to_zzz(0, 0, 0, s2, s1, s0);
        }
        return integer_from_signed_triple(s2, s1, s0);
    }
    // Add the small terms to the large ones.
    if s2.get_highest_bit() {
        let (d2, d1, d0) = Limb::xxx_sub_yyy_to_zzz(0, 0, 0, s2, s1, s0);
        let mut small = vec![d0, d1, d2];
        mpn_add(&mut neg, &mut negn, &mut small, 3);
    } else {
        let mut small = vec![s0, s1, s2];
        mpn_add(&mut pos, &mut posn, &mut small, 3);
    }

    mpn_norm(&pos, &mut posn);
    mpn_norm(&neg, &mut negn);
    if negn == 0 {
        integer_from_limbs(&pos, posn, subtract)
    } else if posn == 0 {
        integer_from_limbs(&neg, negn, !subtract)
    } else {
        // Do the subtraction.
        let mut tneg = false;
        let mut tn = posn;
        if posn > negn {
            tn = posn;
        } else if negn > posn {
            tn = negn;
            tneg = true;
        } else if limbs_cmp_same_length(&pos[..posn], &neg[..negn]) == Less {
            tneg = true;
        }
        if t.len() < tn {
            t.resize(tn, 0);
        }
        if tneg {
            limbs_sub_greater_to_out(&mut t[..tn], &neg[..negn], &pos[..posn]);
        } else {
            limbs_sub_greater_to_out(&mut t[..tn], &pos[..posn], &neg[..negn]);
        }
        mpn_norm(&t, &mut tn);
        integer_from_limbs(&t, tn, tneg ^ subtract)
    }
}}
