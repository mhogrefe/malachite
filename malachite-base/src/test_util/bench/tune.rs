// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

//! The generic machinery of GMP-`tuneup`-style threshold tuning, shared by the per-crate tuners
//! (`malachite-nz`'s `bin_util/tune.rs` for limb-level thresholds, `malachite-float`'s for
//! precision thresholds): batched best-of-k timing of two algorithms on identical inputs, and the
//! scan over sizes that locates their crossover.
//!
//! Measurement notes:
//! - Batched best-of-k timing: noise is strictly additive, so the minimum of many batch
//!   measurements converges on the true cost, unlike means or medians.
//! - Each call should rotate through several distinct random input sets (see `INPUT_SETS`). With
//!   a single input set the branch predictor memorizes the operands' carry patterns and flatters
//!   whichever algorithm is branchier — at small sizes this distorted crossovers badly.
//! - A warmup pass should run before timing to fault in pages and let the core settle.
//!
//! Results are garbage on a busy machine.

use std::time::Instant;

// Number of distinct input sets rotated through during measurement (power of 2).
pub const INPUT_SETS: usize = 8;
pub const RUNS: usize = 9;
pub const MIN_BATCH_NS: u128 = 50_000;

pub fn time_batch(f: &mut dyn FnMut(), iters: u64) -> f64 {
    let start = Instant::now();
    for _ in 0..iters {
        f();
    }
    start.elapsed().as_nanos() as f64 / iters as f64
}

// Calibrate the batch size so that one batch takes >= MIN_BATCH_NS.
pub fn calibrate(f: &mut dyn FnMut()) -> u64 {
    let mut iters = 1u64;
    loop {
        let start = Instant::now();
        for _ in 0..iters {
            f();
        }
        let ns = start.elapsed().as_nanos();
        if ns >= MIN_BATCH_NS {
            return iters;
        }
        iters = if ns == 0 { iters * 100 } else { iters * 2 };
    }
}

// Measure best-case ns per call of two routines, INTERLEAVED: each round times one batch of A then
// one batch of B, and the minimum over rounds is kept for each. Interleaving matters on asymmetric
// cores (Apple Silicon): if A and B were measured in separate blocks, a P/E-core migration or
// frequency shift between blocks would skew one side, producing impossible discontinuities. With
// interleaving plus best-of-k, both sides see the same best environment.
pub fn interleaved_min_pair(fa: &mut dyn FnMut(), fb: &mut dyn FnMut()) -> (f64, f64) {
    let ia = calibrate(fa);
    let ib = calibrate(fb);
    let (mut best_a, mut best_b) = (f64::INFINITY, f64::INFINITY);
    for _ in 0..RUNS {
        best_a = best_a.min(time_batch(fa, ia));
        best_b = best_b.min(time_batch(fb, ib));
    }
    (best_a, best_b)
}

// GMP's analyze_dat: given (size, d) where d > 0 means the lower algorithm was faster at that size,
// pick the cut index minimizing the total relative time lost to mispredictions.
pub fn analyze(dat: &[(usize, f64)]) -> Option<usize> {
    let mut best_i = 0;
    let mut best_badness = f64::INFINITY;
    for i in 0..=dat.len() {
        let mut badness = 0.0;
        for (j, &(_, d)) in dat.iter().enumerate() {
            if j < i {
                // below the cut we'd use the lower algorithm; cost if the upper was faster
                if d < 0.0 {
                    badness -= d;
                }
            } else if d > 0.0 {
                badness += d;
            }
        }
        if badness < best_badness {
            best_badness = badness;
            best_i = i;
        }
    }
    (best_i < dat.len()).then(|| dat[best_i].0)
}

// The generic crossover loop: scans sizes from `min_size` up (geometrically, by 5% steps) and
// reports the cut that minimizes the total relative time lost to mispredictions (GMP's
// `analyze_dat`), as a `const` declaration of type `threshold_type`. `measure` returns (lower time,
// upper time) at a size, or `None` if the size is invalid for either algorithm.
#[allow(clippy::print_stdout)]
pub fn find_crossover_spec(
    threshold_name: &str,
    threshold_type: &str,
    lower_name: &str,
    upper_name: &str,
    min_size: usize,
    max_size: usize,
    measure: &dyn Fn(usize) -> Option<(f64, f64)>,
) {
    let mut dat = Vec::new();
    let mut since_change = 0;
    let mut consecutive_upper_wins = 0;
    let mut last_thresh = None;
    let mut last_size = min_size;
    let mut size = min_size as f64;
    println!("tuning {threshold_name} ({lower_name} -> {upper_name})");
    while (size as usize) < max_size {
        let n = size as usize;
        size = f64::max(size * 1.05, size + 1.0);
        let Some((tl, tu)) = measure(n) else {
            continue;
        };
        // d > 0: lower algorithm faster here
        let d = if tu >= tl {
            (tu - tl) / tu
        } else {
            (tu - tl) / tl
        };
        dat.push((n, d));
        let thresh = analyze(&dat);
        println!(
            "  size {n:>6}  {lower_name} {tl:>10.1}ns  {upper_name} {tu:>10.1}ns  d {d:>7.4}  \
            -> {}",
            thresh.map_or_else(|| "-".to_string(), |t| t.to_string()),
        );
        // Stop when the upper algorithm has clearly won several sizes in a row; a single outlier
        // (e.g. a stray core migration) must not end the scan.
        consecutive_upper_wins = if d < 0.0 {
            consecutive_upper_wins + 1
        } else {
            0
        };
        if consecutive_upper_wins >= 3 && tl >= tu * 1.2 {
            break;
        }
        if thresh == last_thresh {
            since_change += 1;
            // Give up after a long stretch without progress -- but not while the two algorithms are
            // running nearly glued together (|d| small): such plateaus can persist for dozens of
            // sizes before the upper algorithm finally pulls ahead, and quitting inside one reports
            // a bogus "never wins".
            let glued = dat.iter().rev().take(10).any(|&(_, d)| d.abs() < 0.02);
            // Nor while the upper algorithm is still steadily closing the gap (the mean d of the
            // last ten sizes below that of the ten before, so that a single outlier does not
            // decide): a precision threshold's crossover can lie a hundred sizes past the start of
            // the scan.
            let mean_d = |w: &[(usize, f64)]| w.iter().map(|&(_, d)| d).sum::<f64>() / 10.0;
            let closing = dat.len() >= 20
                && mean_d(&dat[dat.len() - 10..])
                    < mean_d(&dat[dat.len() - 20..dat.len() - 10]) - 0.01;
            if since_change > 40 && !glued && !closing {
                break;
            }
        } else {
            since_change = 0;
            last_thresh = thresh;
        }
        last_size = n;
    }
    match analyze(&dat) {
        None => println!(
            "  {threshold_name}: upper algorithm never wins below {last_size} (scan limit \
            {max_size})"
        ),
        Some(t) => println!("pub(crate) const {threshold_name}: {threshold_type} = {t};"),
    }
}
