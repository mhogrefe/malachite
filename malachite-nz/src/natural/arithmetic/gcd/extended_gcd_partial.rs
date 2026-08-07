// Copyright © 2026 Mikhail Hogrefe
//
// Uses code adopted from the FLINT Library.
//
//      Copyright © 2012 William Hart
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::integer::Integer;
use crate::natural::{Natural, WIDTH_MINUS_1};
use crate::platform::{Limb, SignedLimb};
use core::cmp::max;
use core::mem::swap;
use malachite_base::num::arithmetic::traits::{
    AddMul, DivMod, Parity, SubMulAssign, UnsignedAbs, WrappingSubMul,
};
use malachite_base::num::basic::traits::{NegativeOne, Zero};
use malachite_base::num::conversion::traits::ExactFrom;
use malachite_base::num::logic::traits::SignificantBits;

// Lehmer extended GCD with early termination: `r1` and `r2` are successive Euclidean remainders,
// and the algorithm runs the remainder sequence until `r1` is at most `l` (or zero), returning
// `(co2, co1, r2, r1)`, the last two cofactors and remainders. The cofactors satisfy `co2 * r1 -
// co1 * r2 == ±r2_orig`, and the GCD of the remainders is preserved. Each round approximates the
// remainders by their top word and runs a word-level extended Euclidean loop with the classic
// alternating termination test, falling back to a single big Euclidean step when no word step is
// possible.
//
// This is fmpz_xgcd_partial from fmpz/xgcd_partial.c, FLINT 3.6.0, with the in-place arguments as
// owned inputs and returned outputs. The remainders here are nonnegative `Natural`s, which FLINT's
// sign fixups maintain anyway, so its final renormalization of a negative `r2` is unreachable and
// omitted. The word-level candidate updates are computed with wrapping arithmetic before the
// termination test, exactly reproducing the binary behavior of FLINT's signed words.
pub fn extended_gcd_partial(
    mut r2: Natural,
    mut r1: Natural,
    l: &Natural,
) -> (Integer, Integer, Natural, Natural) {
    let mut co2 = Integer::ZERO;
    let mut co1 = Integer::NEGATIVE_ONE;
    while r1 != 0u32 && r1 > *l {
        // Shift so that the larger remainder occupies at most a full word less one bit.
        let bits = max(r2.significant_bits(), r1.significant_bits())
            .saturating_sub(WIDTH_MINUS_1);
        let mut rr2 = SignedLimb::exact_from(Limb::exact_from(&(&r2 >> bits)));
        let mut rr1 = SignedLimb::exact_from(Limb::exact_from(&(&r1 >> bits)));
        // l < r1 here, so the shifted bound fits as well.
        let bb = SignedLimb::exact_from(Limb::exact_from(&(l >> bits)));
        let mut aa2: SignedLimb = 0;
        let mut aa1: SignedLimb = 1;
        let mut bb2: SignedLimb = 1;
        let mut bb1: SignedLimb = 0;
        let mut i = 0u64;
        while rr1 != 0 && rr1 > bb {
            let qq = rr2 / rr1;
            let t1 = rr2.wrapping_sub_mul(qq, rr1);
            let t2 = aa2.wrapping_sub_mul(qq, aa1);
            let t3 = bb2.wrapping_sub_mul(qq, bb1);
            let stop = if i.odd() {
                t1 < t3.wrapping_neg() || rr1.wrapping_sub(t1) < t2.wrapping_sub(aa1)
            } else {
                t1 < t2.wrapping_neg() || rr1.wrapping_sub(t1) < t3.wrapping_sub(bb1)
            };
            if stop {
                break;
            }
            rr2 = rr1;
            rr1 = t1;
            aa2 = aa1;
            aa1 = t2;
            bb2 = bb1;
            bb1 = t3;
            i += 1;
        }
        if i == 0 {
            // No word step was possible; take a single big Euclidean step.
            let (q, r) = (&r2).div_mod(&r1);
            r2 = r;
            swap(&mut r2, &mut r1);
            co2.sub_mul_assign(&co1, Integer::from(q));
            swap(&mut co2, &mut co1);
        } else {
            // Apply the accumulated 2-by-2 word matrix to the remainders and cofactors, moving any
            // temporary negative signs from the remainders onto the cofactors. The coverage sweep
            // never observed either sign fixup firing, over thirty thousand rounds including
            // adversarial near-multiple pairs, consistent with the termination test being
            // Jebelean's exactness criterion, which keeps the true remainders nonnegative; the
            // fixups are retained for fidelity with FLINT, which has them.
            let new_r2 = (Integer::from(&r2) * Integer::from(bb2))
                .add_mul(Integer::from(&r1), Integer::from(aa2));
            let new_r1 = (Integer::from(&r1) * Integer::from(aa1))
                .add_mul(Integer::from(&r2), Integer::from(bb1));
            let new_co2 =
                (&co2 * Integer::from(bb2)).add_mul(co1.clone(), Integer::from(aa2));
            let new_co1 = (&co1 * Integer::from(aa1)).add_mul(co2, Integer::from(bb1));
            co1 = if new_r1 < 0u32 { -new_co1 } else { new_co1 };
            r1 = new_r1.unsigned_abs();
            co2 = if new_r2 < 0u32 { -new_co2 } else { new_co2 };
            r2 = new_r2.unsigned_abs();
        }
    }
    (co2, co1, r2, r1)
}
