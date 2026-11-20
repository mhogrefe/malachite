// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

// Enclosures of real numbers by pairs of `BigExponentFloat`s, and Ziv's loop on them: what the
// functions ported from Azurite share.

use crate::big_exponent_float::BigExponentFloat;
use crate::big_exponent_float::exponential::exp::power_of_2_integer;
use core::cmp::Ordering;
use malachite_base::num::basic::traits::Zero;
use malachite_base::num::logic::traits::SignificantBits;
use malachite_base::rounding_modes::RoundingMode::{self, *};
use malachite_nz::integer::Integer;
use malachite_q::Rational;

// The guard bits of the first working precision of the Ziv loops.
//
// This is equivalent to `zivGuardBits` from `AzFloat/Ziv.lean`, Azurite.
pub(crate) const ZIV_GUARD_BITS: u64 = 64;

// An enclosure [lo, hi] of a real number.
pub(crate) type Bracket = (BigExponentFloat, BigExponentFloat);

// The enclosure [lo, lo + ulp] of a value whose rounding down to `prec` bits is `lo`. A zero `lo`
// is taken to be exact: a `BigExponentFloat` does not underflow.
pub(crate) fn floor_bracket(lo: BigExponentFloat, prec: u64) -> Bracket {
    if lo == 0u32 {
        return (BigExponentFloat::ZERO, BigExponentFloat::ZERO);
    }
    let ulp = power_of_2_integer(lo.get_exponent().unwrap() - Integer::from(prec));
    let hi = lo.add_prec_round_ref_val(ulp, prec, Ceiling).0;
    (lo, hi)
}

// The product of two enclosures of nonnegative numbers, rounded outward at precision `prec`.
//
// This is equivalent to `Encl.mul` from `AzFloat/Interval.lean`, Azurite.
pub(crate) fn encl_mul(prec: u64, a: &Bracket, b: &Bracket) -> Bracket {
    (
        a.0.mul_prec_round_ref_ref(&b.0, prec, Floor).0,
        a.1.mul_prec_round_ref_ref(&b.1, prec, Ceiling).0,
    )
}

// The sum of two enclosures, rounded outward at precision `prec`.
//
// This is equivalent to `Encl.add` from `AzFloat/Interval.lean`, Azurite.
pub(crate) fn encl_add(prec: u64, a: &Bracket, b: &Bracket) -> Bracket {
    (
        a.0.add_prec_round_ref_ref(&b.0, prec, Floor).0,
        a.1.add_prec_round_ref_ref(&b.1, prec, Ceiling).0,
    )
}

// The difference of two enclosures, rounded outward at precision `prec`.
//
// This is equivalent to `Encl.sub` from `AzFloat/Interval.lean`, Azurite.
pub(crate) fn encl_sub(prec: u64, a: &Bracket, b: &Bracket) -> Bracket {
    (
        a.0.sub_prec_round_ref_ref(&b.1, prec, Floor).0,
        a.1.sub_prec_round_ref_ref(&b.0, prec, Ceiling).0,
    )
}

// The quotient of an enclosure of a nonnegative number by an enclosure of a positive number,
// rounded outward at precision `prec`.
//
// This is equivalent to `Encl.div` from `AzFloat/Interval.lean`, Azurite.
pub(crate) fn encl_div(prec: u64, a: &Bracket, b: &Bracket) -> Bracket {
    (
        a.0.div_prec_round_ref_ref(&b.1, prec, Floor).0,
        a.1.div_prec_round_ref_ref(&b.0, prec, Ceiling).0,
    )
}

// When the exact value lies in (l, h] (or is l = h), its rounding to precision `prec` is determined
// as soon as the two ends have the same (`prec` + 1)-bit truncation; the rounding is then that of
// h.
//
// This is equivalent to `roundingPossible` from `AzFloat/Ziv.lean`, Azurite.
pub(crate) fn rounding_possible<
    L: Fn(u64, RoundingMode) -> (BigExponentFloat, Ordering),
    H: Fn(u64, RoundingMode) -> (BigExponentFloat, Ordering),
>(
    l: &L,
    h: &H,
    prec: u64,
    rm: RoundingMode,
) -> Option<(BigExponentFloat, Ordering)> {
    if l(prec + 1, Floor).0 == h(prec + 1, Floor).0 {
        Some(h(prec, rm))
    } else {
        None
    }
}

// Ziv's loop: `approx(w)` brackets the exact value at working precision `w`, which starts at `prec`
// + `ZIV_GUARD_BITS` and doubles until rounding is possible.
//
// This is equivalent to `zivLoopCertified` from `AzFloat/Ziv.lean`, Azurite.
pub(crate) fn ziv_loop_certified<F: Fn(u64) -> Bracket>(
    approx: &F,
    prec: u64,
    rm: RoundingMode,
) -> (BigExponentFloat, Ordering) {
    ziv_loop_certified_from(approx, prec, rm, prec + ZIV_GUARD_BITS)
}

// Ziv's loop with the working precision starting at `start`.
//
// This is equivalent to `zivLoopCertified` from `AzFloat/Ziv.lean`, Azurite.
pub(crate) fn ziv_loop_certified_from<F: Fn(u64) -> Bracket>(
    approx: &F,
    prec: u64,
    rm: RoundingMode,
    start: u64,
) -> (BigExponentFloat, Ordering) {
    let mut w = start;
    loop {
        let (lo, hi) = approx(w);
        if let Some(r) = rounding_possible(
            &|p, m| lo.round_to_prec_round_ref(p, m),
            &|p, m| hi.round_to_prec_round_ref(p, m),
            prec,
            rm,
        ) {
            return r;
        }
        w <<= 1;
    }
}

// An exponent bound for |q|: |q| < 2^(bits(numerator) - bits(denominator) + 1), taken with one bit
// of margin.
//
// This is equivalent to `ratExpBound` from `AzFloat/PowRatBracket.lean`, Azurite.
pub(crate) fn rational_exponent_bound(q: &Rational) -> Integer {
    Integer::from(q.numerator_ref().significant_bits() + 2)
        - Integer::from(q.denominator_ref().significant_bits())
}
