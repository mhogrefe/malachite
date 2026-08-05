// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

// Helpers for verifying the scratch-length contracts of `limbs_*` functions (see
// DOC-CONVENTIONS.md).
//
// In safe Rust an under-sized scratch slice makes the function panic on a slice operation, so the
// primary check is simply running the function with a scratch of exactly the advertised length,
// swept densely across every algorithm-threshold boundary — that is where an incorrect formula
// hides, since the formulas are threshold-dependent and the bug precedent
// (`limbs_mul_high_same_length_scratch_len`, fixed 2026-08-02) lived exactly in a band between two
// algorithms. The sentinel fill measures the other direction: how much of the advertised length the
// function actually touches.

use crate::platform::Limb;
use malachite_base::num::random::random_primitive_ints;
use malachite_base::random::EXAMPLE_SEED;

// A limb pattern that arithmetic kernels are overwhelmingly unlikely to write. 0xa5a5... at any
// limb width.
pub const SCRATCH_SENTINEL: Limb = Limb::MAX / 0xff * 0xa5;

// Runs `f` on a scratch slice of exactly `scratch_len` limbs pre-filled with `SCRATCH_SENTINEL` and
// returns the high-water mark: the number of limbs up to and including the highest one that `f`
// overwrote. If `scratch_len` is smaller than `f` actually needs, `f` is expected to panic. A limb
// that `f` overwrites with the sentinel value itself goes undetected, which only shifts the
// measurement if it is the topmost touched limb (probability about $2^{-W}$ per call).
pub fn scratch_high_water(scratch_len: usize, f: impl FnOnce(&mut [Limb])) -> usize {
    let mut scratch = vec![SCRATCH_SENTINEL; scratch_len];
    f(&mut scratch);
    scratch
        .iter()
        .rposition(|&x| x != SCRATCH_SENTINEL)
        .map_or(0, |i| i + 1)
}

// Lengths that exercise every algorithm band: for each threshold, all lengths within `radius` of
// it, plus geometric filler (about 8 lengths per octave) so the interiors of the bands are sampled
// too. All returned lengths are within `min..=max`.
pub fn threshold_straddling_lengths(
    thresholds: &[usize],
    min: usize,
    max: usize,
    radius: usize,
) -> Vec<usize> {
    let mut lens = Vec::new();
    for &t in thresholds {
        for len in t.saturating_sub(radius)..=t + radius {
            if len >= min && len <= max {
                lens.push(len);
            }
        }
    }
    let mut len = min;
    while len <= max {
        lens.push(len);
        len += core::cmp::max(1, len >> 3);
    }
    lens.push(max);
    lens.sort_unstable();
    lens.dedup();
    lens
}

// Deterministic pseudorandom limbs for canary inputs. The values only need to vary enough to
// exercise value-dependent branches (evaluation signs, carry chains); `name` varies the sequence
// between call sites.
pub fn canary_limbs(name: &str, len: usize) -> Vec<Limb> {
    random_primitive_ints(EXAMPLE_SEED.fork(name))
        .take(len)
        .collect()
}
