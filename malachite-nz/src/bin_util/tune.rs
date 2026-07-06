// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

//! Threshold tuning, after GMP's `tune/tuneup.c`.
//!
//! For each threshold, two algorithm variants are measured head-to-head at increasing sizes and
//! the crossover is located. Like GMP, this sidesteps the "recursive thresholds must be tuned
//! simultaneously" problem by tuning bottom-up: each threshold is finalized (edited into
//! `platform_64.rs`, library rebuilt) before any threshold above it is measured. Since algorithms
//! near a crossover are nearly equal by definition, modest error in earlier levels barely
//! perturbs later ones; iterate to a fixpoint (2-3 passes) if paranoid.
//!
//! Measurement notes:
//! - Batched best-of-k timing: noise is strictly additive, so the minimum of many batch
//!   measurements converges on the true cost, unlike means or medians.
//! - Each call rotates through several distinct random input sets. With a single input set the
//!   branch predictor memorizes the operands' carry patterns and flatters whichever algorithm is
//!   branchier — at small sizes this distorted crossovers badly.
//! - A warmup pass runs before timing to fault in pages and let the core settle.
//!
//! Usage: `cargo run --release --features bin_build -p malachite-nz -- -g tune_mul`
//! (acquire perf/bench-lock.sh first; results are garbage on a busy machine)

use malachite_base::num::basic::integers::PrimitiveInt;
use malachite_base::num::random::random_primitive_ints;
use malachite_base::random::EXAMPLE_SEED;
use malachite_nz::natural::arithmetic::add_mul::limbs_slice_add_mul_limb_same_length_in_place_left;
use malachite_nz::natural::arithmetic::div::{
    limbs_div_barrett_approx, limbs_div_barrett_approx_scratch_len,
    limbs_div_divide_and_conquer_approx, limbs_div_schoolbook_approx,
};
use malachite_nz::natural::arithmetic::div_exact::{
    limbs_modular_div_barrett, limbs_modular_div_barrett_scratch_len,
    limbs_modular_div_divide_and_conquer, limbs_modular_div_mod_barrett,
    limbs_modular_div_mod_barrett_scratch_len, limbs_modular_div_mod_divide_and_conquer,
    limbs_modular_div_mod_schoolbook, limbs_modular_div_schoolbook, limbs_modular_invert,
    limbs_modular_invert_limb, limbs_modular_invert_scratch_len,
};
use malachite_nz::natural::arithmetic::div_mod::limbs_div_mod_to_out;
use malachite_nz::natural::arithmetic::div_mod::{
    limbs_div_mod_barrett, limbs_div_mod_barrett_scratch_len, limbs_div_mod_divide_and_conquer,
    limbs_div_mod_schoolbook, limbs_invert_approx, limbs_invert_approx_scratch_len,
    limbs_invert_basecase_approx, limbs_invert_newton_approx, limbs_two_limb_inverse_helper,
};
use malachite_nz::natural::arithmetic::mod_power_of_2_square::{
    limbs_square_low_basecase, limbs_square_low_divide_and_conquer, limbs_square_low_scratch_len,
};
use malachite_nz::natural::arithmetic::mul::fft::{
    mpn_mul_fft_for_tuning, mpn_square_fft_for_tuning,
};
use malachite_nz::natural::arithmetic::mul::limbs_mul_greater_to_out_basecase;
use malachite_nz::natural::arithmetic::mul::mul_low::{
    limbs_mul_low_same_length_basecase, limbs_mul_low_same_length_divide_and_conquer,
    limbs_mul_low_same_length_divide_and_conquer_scratch_len,
};
use malachite_nz::natural::arithmetic::mul::toom::{
    limbs_mul_greater_to_out_toom_6h, limbs_mul_greater_to_out_toom_6h_input_sizes_valid,
    limbs_mul_greater_to_out_toom_6h_scratch_len, limbs_mul_greater_to_out_toom_8h,
    limbs_mul_greater_to_out_toom_8h_input_sizes_valid,
    limbs_mul_greater_to_out_toom_8h_scratch_len, limbs_mul_greater_to_out_toom_22,
    limbs_mul_greater_to_out_toom_22_input_sizes_valid,
    limbs_mul_greater_to_out_toom_22_scratch_len, limbs_mul_greater_to_out_toom_32,
    limbs_mul_greater_to_out_toom_32_input_sizes_valid,
    limbs_mul_greater_to_out_toom_32_scratch_len, limbs_mul_greater_to_out_toom_33,
    limbs_mul_greater_to_out_toom_33_input_sizes_valid,
    limbs_mul_greater_to_out_toom_33_scratch_len, limbs_mul_greater_to_out_toom_42,
    limbs_mul_greater_to_out_toom_42_input_sizes_valid,
    limbs_mul_greater_to_out_toom_42_scratch_len, limbs_mul_greater_to_out_toom_43,
    limbs_mul_greater_to_out_toom_43_input_sizes_valid,
    limbs_mul_greater_to_out_toom_43_scratch_len, limbs_mul_greater_to_out_toom_44,
    limbs_mul_greater_to_out_toom_44_input_sizes_valid,
    limbs_mul_greater_to_out_toom_44_scratch_len, limbs_mul_greater_to_out_toom_53,
    limbs_mul_greater_to_out_toom_53_input_sizes_valid,
    limbs_mul_greater_to_out_toom_53_scratch_len, limbs_mul_greater_to_out_toom_63,
    limbs_mul_greater_to_out_toom_63_input_sizes_valid,
    limbs_mul_greater_to_out_toom_63_scratch_len,
};
use malachite_nz::natural::arithmetic::mul::{
    limbs_mul_greater_to_out, limbs_mul_greater_to_out_scratch_len,
};
use malachite_nz::natural::arithmetic::square::{
    limbs_square_to_out_basecase, limbs_square_to_out_toom_2,
    limbs_square_to_out_toom_2_scratch_len, limbs_square_to_out_toom_3,
    limbs_square_to_out_toom_3_scratch_len, limbs_square_to_out_toom_4,
    limbs_square_to_out_toom_4_scratch_len, limbs_square_to_out_toom_6,
    limbs_square_to_out_toom_6_scratch_len, limbs_square_to_out_toom_8,
    limbs_square_to_out_toom_8_scratch_len,
};
use malachite_nz::natural::conversion::digits::general_digits::limbs_to_digits_small_base_basecase;
use malachite_nz::platform::Limb;
use std::hint::black_box;
use std::time::Instant;

// Number of distinct input sets rotated through during measurement (power of 2).
const INPUT_SETS: usize = 8;
const RUNS: usize = 9;
const MIN_BATCH_NS: u128 = 50_000;

fn time_batch(f: &mut dyn FnMut(), iters: u64) -> f64 {
    let start = Instant::now();
    for _ in 0..iters {
        f();
    }
    start.elapsed().as_nanos() as f64 / iters as f64
}

// Calibrate the batch size so that one batch takes >= MIN_BATCH_NS.
fn calibrate(f: &mut dyn FnMut()) -> u64 {
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
fn interleaved_min_pair(fa: &mut dyn FnMut(), fb: &mut dyn FnMut()) -> (f64, f64) {
    let ia = calibrate(fa);
    let ib = calibrate(fb);
    let (mut best_a, mut best_b) = (f64::INFINITY, f64::INFINITY);
    for _ in 0..RUNS {
        best_a = best_a.min(time_batch(fa, ia));
        best_b = best_b.min(time_batch(fb, ib));
    }
    (best_a, best_b)
}

type MulFn<'a> = &'a dyn Fn(&mut [Limb], &[Limb], &[Limb], &mut [Limb]);

// A mul-shaped algorithm: validity predicate, scratch size, and the routine itself.
struct Algo<'a> {
    name: &'a str,
    valid: &'a dyn Fn(usize) -> bool,
    scratch_len: &'a dyn Fn(usize) -> usize,
    run: MulFn<'a>,
}

// Measure two mul-shaped algorithms at balanced size n x n on identical, rotating input sets.
fn measure_mul_pair(n: usize, a: &Algo, b: &Algo) -> Option<(f64, f64)> {
    if !(a.valid)(n) || !(b.valid)(n) {
        return None;
    }
    let inputs: Vec<(Vec<Limb>, Vec<Limb>)> = (0..INPUT_SETS)
        .map(|k| {
            let xs = random_primitive_ints(EXAMPLE_SEED.fork(&format!("x{k}")))
                .take(n)
                .collect();
            let ys = random_primitive_ints(EXAMPLE_SEED.fork(&format!("y{k}")))
                .take(n)
                .collect();
            (xs, ys)
        })
        .collect();
    let mut out_a = vec![0; n << 1];
    let mut out_b = vec![0; n << 1];
    let mut scratch_a = vec![0; (a.scratch_len)(n)];
    let mut scratch_b = vec![0; (b.scratch_len)(n)];
    // Warmup: fault in pages, settle the core.
    for (xs, ys) in &inputs {
        (a.run)(&mut out_a, xs, ys, &mut scratch_a);
        (b.run)(&mut out_b, xs, ys, &mut scratch_b);
    }
    let (mut i, mut j) = (0usize, 0usize);
    let (ta, tb) = interleaved_min_pair(
        &mut || {
            let (xs, ys) = &inputs[i & (INPUT_SETS - 1)];
            i += 1;
            (a.run)(black_box(&mut out_a), xs, ys, &mut scratch_a);
        },
        &mut || {
            let (xs, ys) = &inputs[j & (INPUT_SETS - 1)];
            j += 1;
            (b.run)(black_box(&mut out_b), xs, ys, &mut scratch_b);
        },
    );
    Some((ta, tb))
}

// GMP's analyze_dat: given (size, d) where d > 0 means the lower algorithm was faster at that size,
// pick the cut index minimizing the total relative time lost to mispredictions.
fn analyze(dat: &[(usize, f64)]) -> Option<usize> {
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

struct Level<'a> {
    threshold_name: &'a str,
    min_size: usize,
    max_size: usize,
    lower: Algo<'a>,
    upper: Algo<'a>,
}

fn find_crossover(c: &Level) {
    find_crossover_spec(
        c.threshold_name,
        c.lower.name,
        c.upper.name,
        c.min_size,
        c.max_size,
        &|n| measure_mul_pair(n, &c.lower, &c.upper),
    );
}

// The generic crossover loop; `measure` returns (lower time, upper time) at a size, or `None` if
// the size is invalid for either algorithm.
fn find_crossover_spec(
    threshold_name: &str,
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
            if since_change > 40 && !glued {
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
        Some(t) => println!("pub(crate) const {threshold_name}: usize = {t};"),
    }
}

fn basecase_algo<'a>() -> Algo<'a> {
    Algo {
        name: "basecase",
        valid: &|_| true,
        scratch_len: &|_| 0,
        run: &|out, xs, ys, _| limbs_mul_greater_to_out_basecase(out, xs, ys),
    }
}

fn toom22_algo<'a>() -> Algo<'a> {
    Algo {
        name: "toom22",
        valid: &|n| limbs_mul_greater_to_out_toom_22_input_sizes_valid(n, n),
        scratch_len: &|n| limbs_mul_greater_to_out_toom_22_scratch_len(n, n),
        run: &|out, xs, ys, scratch| limbs_mul_greater_to_out_toom_22(out, xs, ys, scratch),
    }
}

fn toom33_algo<'a>() -> Algo<'a> {
    Algo {
        name: "toom33",
        valid: &|n| limbs_mul_greater_to_out_toom_33_input_sizes_valid(n, n),
        scratch_len: &|n| limbs_mul_greater_to_out_toom_33_scratch_len(n, n),
        run: &|out, xs, ys, scratch| limbs_mul_greater_to_out_toom_33(out, xs, ys, scratch),
    }
}

fn toom44_algo<'a>() -> Algo<'a> {
    Algo {
        name: "toom44",
        valid: &|n| limbs_mul_greater_to_out_toom_44_input_sizes_valid(n, n),
        scratch_len: &|n| limbs_mul_greater_to_out_toom_44_scratch_len(n, n),
        run: &|out, xs, ys, scratch| limbs_mul_greater_to_out_toom_44(out, xs, ys, scratch),
    }
}

fn tune_mul_toom22() {
    find_crossover(&Level {
        threshold_name: "MUL_TOOM22_THRESHOLD",
        min_size: 4,
        max_size: 1000,
        lower: basecase_algo(),
        upper: toom22_algo(),
    });
}

fn tune_mul_toom33() {
    find_crossover(&Level {
        threshold_name: "MUL_TOOM33_THRESHOLD",
        min_size: 20,
        max_size: 2000,
        lower: toom22_algo(),
        upper: toom33_algo(),
    });
}

fn tune_mul_toom44() {
    find_crossover(&Level {
        threshold_name: "MUL_TOOM44_THRESHOLD",
        min_size: 60,
        max_size: 4000,
        lower: toom33_algo(),
        upper: toom44_algo(),
    });
}

// The squaring algorithms reuse the mul-shaped `Algo` plumbing, ignoring `ys`. Validity predicates
// replicate each function's split asserts (the sqr functions have no `_input_sizes_valid` helpers).

fn sqr_basecase_algo<'a>() -> Algo<'a> {
    Algo {
        name: "sqr_basecase",
        // The basecase's stack buffer is sized by the compiled-in threshold, so it cannot be
        // measured above it; the crossover scan is capped accordingly. To scan higher, raise
        // SQR_TOOM2_THRESHOLD in platform_64.rs and rebuild.
        valid: &|n| n <= malachite_nz::natural::arithmetic::square::SQR_TOOM2_THRESHOLD,
        scratch_len: &|_| 0,
        run: &|out, xs, _, _| limbs_square_to_out_basecase(out, xs),
    }
}

fn sqr_toom2_algo<'a>() -> Algo<'a> {
    Algo {
        name: "sqr_toom2",
        valid: &|n| n > 1,
        scratch_len: &limbs_square_to_out_toom_2_scratch_len,
        run: &|out, xs, _, scratch| limbs_square_to_out_toom_2(out, xs, scratch),
    }
}

fn sqr_toom3_algo<'a>() -> Algo<'a> {
    Algo {
        name: "sqr_toom3",
        // n = ceil(len / 3), s = len - 2n; s must be in 1..=n.
        valid: &|len| {
            let n = len.div_ceil(3);
            len > n << 1 && len <= 3 * n
        },
        scratch_len: &limbs_square_to_out_toom_3_scratch_len,
        run: &|out, xs, _, scratch| limbs_square_to_out_toom_3(out, xs, scratch),
    }
}

fn sqr_toom4_algo<'a>() -> Algo<'a> {
    Algo {
        name: "sqr_toom4",
        // n = ceil(len / 4), s = len - 3n; s must be in 1..=n.
        valid: &|len| {
            let n = (len + 3) >> 2;
            len > 3 * n && len <= n << 2
        },
        scratch_len: &limbs_square_to_out_toom_4_scratch_len,
        run: &|out, xs, _, scratch| limbs_square_to_out_toom_4(out, xs, scratch),
    }
}

fn tune_sqr_toom2() {
    find_crossover(&Level {
        threshold_name: "SQR_TOOM2_THRESHOLD",
        min_size: 4,
        max_size: malachite_nz::natural::arithmetic::square::SQR_TOOM2_THRESHOLD,
        lower: sqr_basecase_algo(),
        upper: sqr_toom2_algo(),
    });
}

fn tune_sqr_toom3() {
    find_crossover(&Level {
        threshold_name: "SQR_TOOM3_THRESHOLD",
        min_size: 20,
        max_size: 2000,
        lower: sqr_toom2_algo(),
        upper: sqr_toom3_algo(),
    });
}

fn sqr_toom6_algo<'a>() -> Algo<'a> {
    Algo {
        name: "sqr_toom6",
        // n = 1 + (len - 1) / 6, s = len - 5n; needs len >= 18, s in 1..=n, and 10n + 3 <= 2 * len.
        valid: &|len| {
            if len < 18 {
                return false;
            }
            let n = 1 + (len - 1) / 6;
            len > 5 * n && len - 5 * n <= n && 10 * n + 3 <= len << 1
        },
        scratch_len: &limbs_square_to_out_toom_6_scratch_len,
        run: &|out, xs, _, scratch| limbs_square_to_out_toom_6(out, xs, scratch),
    }
}

fn sqr_toom8_algo<'a>() -> Algo<'a> {
    Algo {
        name: "sqr_toom8",
        // n = ceil(len / 8), s = len - 7n; needs len >= 40 and s in 2..=n.
        valid: &|len| {
            if len < 40 {
                return false;
            }
            let n = len.div_ceil(8);
            len > 7 * n + 1 && len - 7 * n <= n
        },
        scratch_len: &limbs_square_to_out_toom_8_scratch_len,
        run: &|out, xs, _, scratch| limbs_square_to_out_toom_8(out, xs, scratch),
    }
}

fn tune_sqr_toom4() {
    find_crossover(&Level {
        threshold_name: "SQR_TOOM4_THRESHOLD",
        min_size: 60,
        max_size: 4000,
        lower: sqr_toom3_algo(),
        upper: sqr_toom4_algo(),
    });
}

fn tune_sqr_toom6() {
    find_crossover(&Level {
        threshold_name: "SQR_TOOM6_THRESHOLD",
        min_size: 200,
        max_size: 6000,
        lower: sqr_toom4_algo(),
        upper: sqr_toom6_algo(),
    });
}

fn toom6h_algo<'a>() -> Algo<'a> {
    Algo {
        name: "toom6h",
        valid: &|n| limbs_mul_greater_to_out_toom_6h_input_sizes_valid(n, n),
        scratch_len: &|n| limbs_mul_greater_to_out_toom_6h_scratch_len(n, n),
        run: &|out, xs, ys, scratch| limbs_mul_greater_to_out_toom_6h(out, xs, ys, scratch),
    }
}

fn toom8h_algo<'a>() -> Algo<'a> {
    Algo {
        name: "toom8h",
        valid: &|n| limbs_mul_greater_to_out_toom_8h_input_sizes_valid(n, n),
        scratch_len: &|n| limbs_mul_greater_to_out_toom_8h_scratch_len(n, n),
        run: &|out, xs, ys, scratch| limbs_mul_greater_to_out_toom_8h(out, xs, ys, scratch),
    }
}

fn tune_mul_toom6h() {
    find_crossover(&Level {
        threshold_name: "MUL_TOOM6H_THRESHOLD",
        min_size: 100,
        max_size: 4000,
        lower: toom44_algo(),
        upper: toom6h_algo(),
    });
}

// toom6h's measured crossover vs toom44 (229) is below toom44's own threshold (a ~315-465 plateau
// vs toom33), suggesting toom44 may have no winning range for balanced mul, as toom4 has none for
// squaring; this measures toom6h against the real incumbent directly.
fn tune_mul_toom6h_vs_toom33() {
    find_crossover(&Level {
        threshold_name: "MUL_TOOM6H_THRESHOLD",
        min_size: 60,
        max_size: 4000,
        lower: toom33_algo(),
        upper: toom6h_algo(),
    });
}

fn tune_mul_toom8h() {
    find_crossover(&Level {
        threshold_name: "MUL_TOOM8H_THRESHOLD",
        min_size: 200,
        max_size: 8000,
        lower: toom6h_algo(),
        upper: toom8h_algo(),
    });
}

// Division algorithms all share the `mpn_sbpi1_div_qr` shape: quotient out, dividend mutated in
// place (its low limbs become the remainder), normalized divisor, precomputed two-limb inverse.
type DivAlgoFn = fn(&mut [Limb], &mut [Limb], &[Limb], Limb) -> bool;

// Measure two division algorithms dividing 2n limbs by n limbs on identical, rotating input sets.
// The dividend is refreshed from a pristine copy before each call (the copy cost is incurred
// identically by both sides).
fn measure_div_pair(n: usize, min_d: usize, a: DivAlgoFn, b: DivAlgoFn) -> Option<(f64, f64)> {
    if n < min_d {
        return None;
    }
    let inputs: Vec<(Vec<Limb>, Vec<Limb>, Limb)> = (0..INPUT_SETS)
        .map(|k| {
            let ns: Vec<Limb> = random_primitive_ints(EXAMPLE_SEED.fork(&format!("dn{k}")))
                .take(n << 1)
                .collect();
            let mut ds: Vec<Limb> = random_primitive_ints(EXAMPLE_SEED.fork(&format!("dd{k}")))
                .take(n)
                .collect();
            // The divisor must be normalized (highest bit set).
            ds[n - 1] |= 1 << (Limb::WIDTH - 1);
            let d_inv = limbs_two_limb_inverse_helper(ds[n - 1], ds[n - 2]);
            (ns, ds, d_inv)
        })
        .collect();
    let mut ns_a = vec![0; n << 1];
    let mut ns_b = vec![0; n << 1];
    let mut qs_a = vec![0; n];
    let mut qs_b = vec![0; n];
    // Warmup: fault in pages, settle the core.
    for (ns, ds, d_inv) in &inputs {
        ns_a.copy_from_slice(ns);
        a(&mut qs_a, &mut ns_a, ds, *d_inv);
        ns_b.copy_from_slice(ns);
        b(&mut qs_b, &mut ns_b, ds, *d_inv);
    }
    let (mut i, mut j) = (0usize, 0usize);
    let (ta, tb) = interleaved_min_pair(
        &mut || {
            let (ns, ds, d_inv) = &inputs[i & (INPUT_SETS - 1)];
            i += 1;
            ns_a.copy_from_slice(ns);
            a(black_box(&mut qs_a), &mut ns_a, ds, *d_inv);
        },
        &mut || {
            let (ns, ds, d_inv) = &inputs[j & (INPUT_SETS - 1)];
            j += 1;
            ns_b.copy_from_slice(ns);
            b(black_box(&mut qs_b), &mut ns_b, ds, *d_inv);
        },
    );
    Some((ta, tb))
}

// Newton inversion vs the basecase approximate inversion, at divisor length n (normalized).
fn tune_inv_newton() {
    find_crossover_spec(
        "INV_NEWTON_THRESHOLD",
        "invert_basecase",
        "invert_newton",
        5,
        4000,
        &|n| {
            if n < 5 {
                return None;
            }
            let inputs: Vec<Vec<Limb>> = (0..INPUT_SETS)
                .map(|k| {
                    let mut ds: Vec<Limb> =
                        random_primitive_ints(EXAMPLE_SEED.fork(&format!("inv{k}")))
                            .take(n)
                            .collect();
                    ds[n - 1] |= 1 << (Limb::WIDTH - 1);
                    ds
                })
                .collect();
            let mut is_a = vec![0; n];
            let mut is_b = vec![0; n];
            let mut scratch_a = vec![0; limbs_invert_approx_scratch_len(n)];
            let mut scratch_b = vec![0; limbs_invert_approx_scratch_len(n)];
            for ds in &inputs {
                limbs_invert_basecase_approx(&mut is_a, ds, &mut scratch_a);
                limbs_invert_newton_approx(&mut is_b, ds, &mut scratch_b);
            }
            let (mut i, mut j) = (0usize, 0usize);
            Some(interleaved_min_pair(
                &mut || {
                    let ds = &inputs[i & (INPUT_SETS - 1)];
                    i += 1;
                    black_box(limbs_invert_basecase_approx(
                        black_box(&mut is_a),
                        ds,
                        &mut scratch_a,
                    ));
                },
                &mut || {
                    let ds = &inputs[j & (INPUT_SETS - 1)];
                    j += 1;
                    black_box(limbs_invert_newton_approx(
                        black_box(&mut is_b),
                        ds,
                        &mut scratch_b,
                    ));
                },
            ))
        },
    );
}

// Divide-and-conquer division vs Barrett (MU) division, dividing 2n limbs by n limbs. The DC side
// consumes its dividend, so its per-call refresh copy is included (as any dividend-preserving
// caller would pay it); Barrett reads the dividend by reference.
fn measure_mu_pair(
    n: usize,
    dc: DivAlgoFn,
    barrett: fn(&mut [Limb], &mut [Limb], &[Limb], &[Limb], &mut [Limb]) -> bool,
    barrett_scratch: fn(usize, usize) -> usize,
) -> Option<(f64, f64)> {
    if n < 6 {
        return None;
    }
    let inputs: Vec<(Vec<Limb>, Vec<Limb>, Limb)> = (0..INPUT_SETS)
        .map(|k| {
            let ns: Vec<Limb> = random_primitive_ints(EXAMPLE_SEED.fork(&format!("mn{k}")))
                .take(n << 1)
                .collect();
            let mut ds: Vec<Limb> = random_primitive_ints(EXAMPLE_SEED.fork(&format!("md{k}")))
                .take(n)
                .collect();
            ds[n - 1] |= 1 << (Limb::WIDTH - 1);
            let d_inv = limbs_two_limb_inverse_helper(ds[n - 1], ds[n - 2]);
            (ns, ds, d_inv)
        })
        .collect();
    let mut ns_work = vec![0; n << 1];
    let mut qs_a = vec![0; n + 1];
    let mut qs_b = vec![0; n + 1];
    let mut rs = vec![0; n];
    let mut scratch = vec![0; barrett_scratch(n << 1, n)];
    for (ns, ds, d_inv) in &inputs {
        ns_work.copy_from_slice(ns);
        dc(&mut qs_a[..n], &mut ns_work, ds, *d_inv);
        barrett(&mut qs_b[..n], &mut rs, ns, ds, &mut scratch);
    }
    let (mut i, mut j) = (0usize, 0usize);
    Some(interleaved_min_pair(
        &mut || {
            let (ns, ds, d_inv) = &inputs[i & (INPUT_SETS - 1)];
            i += 1;
            ns_work.copy_from_slice(ns);
            dc(black_box(&mut qs_a[..n]), &mut ns_work, ds, *d_inv);
        },
        &mut || {
            let (ns, ds, _) = &inputs[j & (INPUT_SETS - 1)];
            j += 1;
            barrett(black_box(&mut qs_b[..n]), &mut rs, ns, ds, &mut scratch);
        },
    ))
}

fn tune_mu_div_qr() {
    find_crossover_spec(
        "MU_DIV_QR_THRESHOLD",
        "dc_div_qr",
        "barrett_div_qr",
        61,
        8000,
        &|n| {
            measure_mu_pair(
                n,
                limbs_div_mod_divide_and_conquer,
                limbs_div_mod_barrett,
                limbs_div_mod_barrett_scratch_len,
            )
        },
    );
}

fn tune_mu_divappr_q() {
    find_crossover_spec(
        "MU_DIVAPPR_Q_THRESHOLD",
        "dc_divappr_q",
        "barrett_divappr_q",
        61,
        10000,
        &|n| {
            measure_mu_pair(
                n,
                limbs_div_divide_and_conquer_approx,
                |qs, _rs, ns, ds, scratch| {
                    // The approx variant takes no remainder buffer; adapt to the common shape.
                    let mut ns_plus = ns.to_vec();
                    let _ = &mut ns_plus;
                    limbs_div_barrett_approx(qs, ns, ds, scratch)
                },
                limbs_div_barrett_approx_scratch_len,
            )
        },
    );
}

// The Malachite side of the FFT-region mul comparison; the C sides are perf/scratch/{mul_gmp.c,
// mul_flint.c} (make mul-gmp / mul-gmp-noasm / mul-flint). Inputs use the same LCG so all four
// harnesses multiply identical operands. Times go through the full dispatch, so sizes >=
// MUL_FFT_THRESHOLD exercise the fft_small port.
fn tune_mul_fft_probe() {
    let mut n = 1024;
    while n <= 131072 {
        let mut xs = vec![0; n];
        let mut ys = vec![0; n];
        lcg_fill(&mut xs, 1);
        lcg_fill(&mut ys, 2);
        let mut out = vec![0; n << 1];
        let mut scratch = vec![0; limbs_mul_greater_to_out_scratch_len(n, n)];
        limbs_mul_greater_to_out(&mut out, &xs, &ys, &mut scratch); // warmup
        let mut f = || {
            black_box(limbs_mul_greater_to_out(
                black_box(&mut out),
                &xs,
                &ys,
                &mut scratch,
            ));
        };
        let iters = 1 + ((1u64 << 22) / n as u64);
        let mut best = f64::INFINITY;
        for _ in 0..7 {
            let t = time_batch(&mut f, iters);
            if t < best {
                best = t;
            }
        }
        println!("limbs_mul_greater_to_out n={n:<7} {best:>14.1} ns");
        n <<= 1;
    }
}

// The Malachite side of the small-kernel gap analysis; the C side is perf/scratch/small_gmp.c (make
// small-gmp / small-gmp-noasm). Sizes, seeds, and the best-of-batches loop mirror the C harness
// exactly, so the tables compare directly.
fn tune_small_kernel_probe() {
    fn best_of(f: &mut dyn FnMut(), iters: u64) -> f64 {
        let mut best = f64::INFINITY;
        for _ in 0..9 {
            let t = time_batch(f, iters);
            if t < best {
                best = t;
            }
        }
        best
    }
    for n in [1usize, 2, 3, 4, 6, 8, 12, 16, 24, 32, 48, 64] {
        let mut xs = vec![0; n];
        let mut ys = vec![0; n];
        lcg_fill(&mut xs, 1);
        lcg_fill(&mut ys, 2);
        let mut out = vec![0; n << 1];
        let mut scratch = vec![0; limbs_mul_greater_to_out_scratch_len(n, n)];
        let iters = 1 + ((1u64 << 22) / (n * n + 16) as u64);
        let t_dispatch = best_of(
            &mut || {
                black_box(limbs_mul_greater_to_out(
                    black_box(&mut out),
                    &xs,
                    &ys,
                    &mut scratch,
                ));
            },
            iters,
        );
        let t_basecase = best_of(
            &mut || {
                limbs_mul_greater_to_out_basecase(black_box(&mut out), &xs, &ys);
            },
            iters,
        );
        println!("mul n={n:<3} dispatch {t_dispatch:>10.1} ns   basecase {t_basecase:>10.1} ns");
    }
    for n in [2usize, 4, 8, 16, 32, 64] {
        let mut ns = vec![0; n << 1];
        let mut ds = vec![0; n];
        lcg_fill(&mut ns, 3);
        lcg_fill(&mut ds, 4);
        ds[n - 1] |= 1 << (Limb::WIDTH - 1);
        let mut qs = vec![0; n + 1];
        let mut rs = vec![0; n];
        let iters = 1 + ((1u64 << 22) / (n * n + 16) as u64);
        let t = best_of(
            &mut || {
                limbs_div_mod_to_out(black_box(&mut qs), &mut rs, &ns, &ds);
            },
            iters,
        );
        println!("div 2n/n n={n:<3} {t:>10.1} ns");
    }
    for n in [16usize, 64, 256] {
        let mut xs = vec![0; n];
        let mut ys = vec![0; n];
        lcg_fill(&mut xs, 5);
        lcg_fill(&mut ys, 6);
        let z = 0x9E3779B97F4A7C15 as Limb;
        let iters = 1 + ((1u64 << 22) / n as u64);
        let t = best_of(
            &mut || {
                black_box(limbs_slice_add_mul_limb_same_length_in_place_left(
                    black_box(&mut xs),
                    &ys,
                    z,
                ));
            },
            iters,
        );
        println!("addmul_1 n={n:<3} {t:>10.1} ns");
    }
}

// The Malachite side of the gcd comparison; the C side is perf/scratch/gcd_gmp.c (make gcd-gmp /
// gcd-gmp-noasm). Identical operands via the shared LCG.
fn tune_gcd_probe() {
    use malachite_base::num::arithmetic::traits::Gcd;
    use malachite_nz::natural::Natural;
    for n in [10usize, 25, 50, 100, 200, 400, 800, 1600, 3200, 6400] {
        let mut xs = vec![0; n];
        let mut ys = vec![0; n];
        lcg_fill(&mut xs, 1);
        lcg_fill(&mut ys, 2);
        let x = Natural::from_owned_limbs_asc(xs);
        let y = Natural::from_owned_limbs_asc(ys);
        let mut best = f64::INFINITY;
        let iters = 1 + ((1u64 << 24) / (n * n) as u64);
        for _ in 0..7 {
            let mut f = || {
                black_box((&x).gcd(&y));
            };
            let t = time_batch(&mut f, iters);
            if t < best {
                best = t;
            }
        }
        println!("gcd n={n:<5} {best:>12.1} ns");
    }
}

// Extended-gcd timing at n-limb operand sizes, for the GCDEXT_DC / MATRIX22_STRASSEN sweeps.
fn tune_xgcd_probe() {
    use malachite_base::num::arithmetic::traits::ExtendedGcd;
    use malachite_nz::natural::Natural;
    for n in [25usize, 50, 100, 200, 400, 800, 1600, 3200] {
        let mut xs = vec![0; n];
        let mut ys = vec![0; n];
        lcg_fill(&mut xs, 1);
        lcg_fill(&mut ys, 2);
        let x = Natural::from_owned_limbs_asc(xs);
        let y = Natural::from_owned_limbs_asc(ys);
        let mut best = f64::INFINITY;
        let iters = 1 + ((1u64 << 23) / (n * n) as u64);
        for _ in 0..7 {
            let mut f = || {
                black_box((&x).extended_gcd(&y));
            };
            let t = time_batch(&mut f, iters);
            if t < best {
                best = t;
            }
        }
        println!("xgcd n={n:<5} {best:>12.1} ns");
    }
}

// Matches the `lcg_fill` in the C harnesses, so all sides see identical operands.
fn lcg_fill(p: &mut [Limb], seed: u64) {
    let mut s = 0x9E3779B97F4A7C15u64 ^ seed;
    for x in p.iter_mut() {
        s = s
            .wrapping_mul(6364136223846793005)
            .wrapping_add(1442695040888963407);
        *x = s as Limb;
    }
}

fn fft_mul_algo<'a>() -> Algo<'a> {
    Algo {
        name: "fft",
        valid: &|_| true,
        scratch_len: &|_| 0,
        run: &|out, xs, ys, _| mpn_mul_fft_for_tuning(out, xs, ys),
    }
}

fn tune_mul_fft() {
    find_crossover(&Level {
        threshold_name: "MUL_FFT_THRESHOLD",
        min_size: 400,
        max_size: 8000,
        lower: toom8h_algo(),
        upper: fft_mul_algo(),
    });
}

fn fft_sqr_algo<'a>() -> Algo<'a> {
    Algo {
        name: "fft_sqr",
        valid: &|_| true,
        scratch_len: &|_| 0,
        run: &|out, xs, _, _| mpn_square_fft_for_tuning(out, xs),
    }
}

// SQR_FFT_THRESHOLD is currently derived (SQR_FFT_MODF_THRESHOLD * 10, frozen at 11700 in
// square.rs); this measures where the FFT square actually overtakes toom8, to inform replacing the
// derivation with a measured constant.
fn tune_sqr_fft() {
    find_crossover(&Level {
        threshold_name: "SQR_FFT_THRESHOLD",
        min_size: 400,
        max_size: 16000,
        lower: sqr_toom8_algo(),
        upper: fft_sqr_algo(),
    });
}

// Both FFT crossovers landed below the toom8 thresholds, so toom8h/toom8 may be squeezed out; these
// measure the FFT against the real incumbents at those sizes.
fn tune_mul_fft_vs_toom6h() {
    find_crossover(&Level {
        threshold_name: "MUL_FFT_THRESHOLD",
        min_size: 64,
        max_size: 8000,
        lower: toom6h_algo(),
        upper: fft_mul_algo(),
    });
}

fn tune_sqr_fft_vs_toom6() {
    find_crossover(&Level {
        threshold_name: "SQR_FFT_THRESHOLD",
        min_size: 64,
        max_size: 16000,
        lower: sqr_toom6_algo(),
        upper: fft_sqr_algo(),
    });
}

// Correctness sweep for the FFT at small sizes (the `l <= LG_BLK_SZ` small-transform branch is only
// reachable below ~400 limbs, which production never did while MUL_FFT_THRESHOLD was 1500):
// compares the FFT against the standard dispatch on many random inputs.
fn tune_fft_small_check() {
    let mut mismatches = 0;
    for n in 64..=1024 {
        for k in 0..4u32 {
            let xs: Vec<Limb> = random_primitive_ints(EXAMPLE_SEED.fork(&format!("fx{n}_{k}")))
                .take(n)
                .collect();
            let ys: Vec<Limb> = random_primitive_ints(EXAMPLE_SEED.fork(&format!("fy{n}_{k}")))
                .take(n)
                .collect();
            let mut out_fft = vec![0; n << 1];
            let mut out_ref = vec![0; n << 1];
            let mut scratch = vec![0; limbs_mul_greater_to_out_scratch_len(n, n)];
            let xs2 = xs.clone();
            let ys2 = ys.clone();
            let mul_ok = std::panic::catch_unwind(move || {
                let mut out = vec![0; xs2.len() << 1];
                mpn_mul_fft_for_tuning(&mut out, &xs2, &ys2);
                out
            });
            limbs_mul_greater_to_out(&mut out_ref, &xs, &ys, &mut scratch);
            match mul_ok {
                Err(_) => {
                    println!("MUL PANIC at n={n} set {k}");
                    mismatches += 1;
                }
                Ok(out) if out != out_ref => {
                    println!("MUL MISMATCH at n={n} set {k}");
                    mismatches += 1;
                }
                _ => {}
            }
            let xs2 = xs.clone();
            let sqr_ok = std::panic::catch_unwind(move || {
                let mut out = vec![0; xs2.len() << 1];
                mpn_square_fft_for_tuning(&mut out, &xs2);
                out
            });
            let mut sq_ref = vec![0; n << 1];
            limbs_mul_greater_to_out(&mut sq_ref, &xs, &xs, &mut scratch);
            match sqr_ok {
                Err(_) => {
                    println!("SQR PANIC at n={n} set {k}");
                    mismatches += 1;
                }
                Ok(out) if out != sq_ref => {
                    println!("SQR MISMATCH at n={n} set {k}");
                    mismatches += 1;
                }
                _ => {}
            }
            let _ = &out_fft;
        }
    }
    println!("fft small-size check: {mismatches} failures over sizes 64..=1024 x4 input sets");
}

// An algorithm entry for the unbalanced-multiplication crossovers, parameterized over
// (xs_len, ys_len) rather than a single balanced size.
struct UnbalancedAlgo<'a> {
    name: &'a str,
    valid: &'a dyn Fn(usize, usize) -> bool,
    scratch_len: &'a dyn Fn(usize, usize) -> usize,
    run: &'a dyn Fn(&mut [Limb], &[Limb], &[Limb], &mut [Limb]),
}

// Like measure_mul_pair, but for distinct operand lengths.
fn measure_unbalanced_pair(
    xs_len: usize,
    ys_len: usize,
    a: &UnbalancedAlgo,
    b: &UnbalancedAlgo,
) -> Option<(f64, f64)> {
    if xs_len < ys_len || !(a.valid)(xs_len, ys_len) || !(b.valid)(xs_len, ys_len) {
        return None;
    }
    let inputs: Vec<(Vec<Limb>, Vec<Limb>)> = (0..INPUT_SETS)
        .map(|k| {
            let xs = random_primitive_ints(EXAMPLE_SEED.fork(&format!("ux{k}")))
                .take(xs_len)
                .collect();
            let ys = random_primitive_ints(EXAMPLE_SEED.fork(&format!("uy{k}")))
                .take(ys_len)
                .collect();
            (xs, ys)
        })
        .collect();
    let out_len = xs_len + ys_len;
    let mut out_a = vec![0; out_len];
    let mut out_b = vec![0; out_len];
    let mut scratch_a = vec![0; (a.scratch_len)(xs_len, ys_len)];
    let mut scratch_b = vec![0; (b.scratch_len)(xs_len, ys_len)];
    for (xs, ys) in &inputs {
        (a.run)(&mut out_a, xs, ys, &mut scratch_a);
        (b.run)(&mut out_b, xs, ys, &mut scratch_b);
    }
    assert_eq!(out_a, out_b);
    let (mut i, mut j) = (0usize, 0usize);
    Some(interleaved_min_pair(
        &mut || {
            let (xs, ys) = &inputs[i & (INPUT_SETS - 1)];
            i += 1;
            (a.run)(black_box(&mut out_a), xs, ys, &mut scratch_a);
        },
        &mut || {
            let (xs, ys) = &inputs[j & (INPUT_SETS - 1)];
            j += 1;
            (b.run)(black_box(&mut out_b), xs, ys, &mut scratch_b);
        },
    ))
}

macro_rules! unbalanced_algo {
    ($name: literal, $valid: ident, $scratch: ident, $run: ident) => {
        UnbalancedAlgo {
            name: $name,
            valid: &|x, y| $valid(x, y),
            scratch_len: &|x, y| $scratch(x, y),
            run: &|out, xs, ys, scratch| $run(out, xs, ys, scratch),
        }
    };
}

// Each interior threshold selects between two Toom variants within an aspect-ratio band of the
// unbalanced dispatch (see limbs_mul_greater_to_out); the scan sweeps ys_len with xs_len at a
// representative aspect ratio inside that band.
fn tune_unbalanced_interior(
    threshold_name: &str,
    lower: &UnbalancedAlgo,
    upper: &UnbalancedAlgo,
    aspect: &dyn Fn(usize) -> usize,
    max_size: usize,
) {
    find_crossover_spec(threshold_name, lower.name, upper.name, 15, max_size, &|y| {
        measure_unbalanced_pair(aspect(y), y, lower, upper)
    });
}

fn tune_mul_toom32_to_toom43() {
    // dispatch band: 7 * y / 6 <= x < 3 * y / 2; representative aspect 4 / 3
    tune_unbalanced_interior(
        "MUL_TOOM32_TO_TOOM43_THRESHOLD",
        &unbalanced_algo!(
            "toom32",
            limbs_mul_greater_to_out_toom_32_input_sizes_valid,
            limbs_mul_greater_to_out_toom_32_scratch_len,
            limbs_mul_greater_to_out_toom_32
        ),
        &unbalanced_algo!(
            "toom43",
            limbs_mul_greater_to_out_toom_43_input_sizes_valid,
            limbs_mul_greater_to_out_toom_43_scratch_len,
            limbs_mul_greater_to_out_toom_43
        ),
        &|y| y * 4 / 3,
        1200,
    );
}

fn tune_mul_toom32_to_toom53() {
    // dispatch band: 3 * y / 2 <= x < 7 * y / 4; toom32's validity (2x < 3(y + 1)) only
    // overlaps the band's bottom edge, so the comparison runs at x = 3y/2 + 1
    tune_unbalanced_interior(
        "MUL_TOOM32_TO_TOOM53_THRESHOLD",
        &unbalanced_algo!(
            "toom32",
            limbs_mul_greater_to_out_toom_32_input_sizes_valid,
            limbs_mul_greater_to_out_toom_32_scratch_len,
            limbs_mul_greater_to_out_toom_32
        ),
        &unbalanced_algo!(
            "toom53",
            limbs_mul_greater_to_out_toom_53_input_sizes_valid,
            limbs_mul_greater_to_out_toom_53_scratch_len,
            limbs_mul_greater_to_out_toom_53
        ),
        &|y| y * 3 / 2 + 1,
        2000,
    );
}

fn tune_mul_toom42_to_toom53() {
    // dispatch band: 7 * y / 4 <= x < 11 * y / 6; representative aspect 9 / 5
    tune_unbalanced_interior(
        "MUL_TOOM42_TO_TOOM53_THRESHOLD",
        &unbalanced_algo!(
            "toom42",
            limbs_mul_greater_to_out_toom_42_input_sizes_valid,
            limbs_mul_greater_to_out_toom_42_scratch_len,
            limbs_mul_greater_to_out_toom_42
        ),
        &unbalanced_algo!(
            "toom53",
            limbs_mul_greater_to_out_toom_53_input_sizes_valid,
            limbs_mul_greater_to_out_toom_53_scratch_len,
            limbs_mul_greater_to_out_toom_53
        ),
        &|y| y * 9 / 5,
        3000,
    );
}

fn tune_mul_toom42_to_toom63() {
    // dispatch band: 11 * y / 6 <= x < 5 * y / 2; representative aspect 21 / 10
    tune_unbalanced_interior(
        "MUL_TOOM42_TO_TOOM63_THRESHOLD",
        &unbalanced_algo!(
            "toom42",
            limbs_mul_greater_to_out_toom_42_input_sizes_valid,
            limbs_mul_greater_to_out_toom_42_scratch_len,
            limbs_mul_greater_to_out_toom_42
        ),
        &unbalanced_algo!(
            "toom63",
            limbs_mul_greater_to_out_toom_63_input_sizes_valid,
            limbs_mul_greater_to_out_toom_63_scratch_len,
            limbs_mul_greater_to_out_toom_63
        ),
        &|y| y * 21 / 10,
        3000,
    );
}

fn mullo_basecase_algo<'a>() -> Algo<'a> {
    Algo {
        name: "mullo_basecase",
        valid: &|_| true,
        scratch_len: &|_| 0,
        run: &|out, xs, ys, _| limbs_mul_low_same_length_basecase(out, xs, ys),
    }
}

fn mullo_dc_algo<'a>() -> Algo<'a> {
    Algo {
        name: "mullo_dc",
        valid: &|n| n >= 8,
        scratch_len: &|n| limbs_mul_low_same_length_divide_and_conquer_scratch_len(n),
        run: &|out, xs, ys, scratch| {
            limbs_mul_low_same_length_divide_and_conquer(out, xs, ys, scratch);
        },
    }
}

fn tune_mullo_dc() {
    find_crossover(&Level {
        threshold_name: "MULLO_DC_THRESHOLD",
        min_size: 8,
        max_size: 500,
        lower: mullo_basecase_algo(),
        upper: mullo_dc_algo(),
    });
}

// Above MULLO_MUL_N_THRESHOLD, the low product is computed as a full multiplication (whose high
// half is discarded); the harness's output buffer has 2n limbs, so both run in place. (The
// balanced-measure harness does not compare outputs, so the differing high halves are fine.)
fn tune_mullo_mul_n() {
    find_crossover(&Level {
        threshold_name: "MULLO_MUL_N_THRESHOLD",
        min_size: 300,
        max_size: 30000,
        lower: mullo_dc_algo(),
        upper: Algo {
            name: "full_mul",
            valid: &|_| true,
            scratch_len: &|n| limbs_mul_greater_to_out_scratch_len(n, n),
            run: &|out, xs, ys, scratch| {
                black_box(limbs_mul_greater_to_out(out, xs, ys, scratch));
            },
        },
    });
}

// SQRLO_DC_THRESHOLD is capped by SQRLO_DC_THRESHOLD_LIMIT (500); the scan respects the limit.
fn tune_sqrlo_dc() {
    find_crossover(&Level {
        threshold_name: "SQRLO_DC_THRESHOLD",
        min_size: 15,
        max_size: 499,
        lower: Algo {
            name: "sqrlo_basecase",
            valid: &|_| true,
            scratch_len: &|_| 0,
            run: &|out, xs, _, _| limbs_square_low_basecase(out, xs),
        },
        upper: Algo {
            name: "sqrlo_dc",
            valid: &|n| n >= 8,
            scratch_len: &|n| limbs_square_low_scratch_len(n),
            run: &|out, xs, _, scratch| limbs_square_low_divide_and_conquer(out, xs, scratch),
        },
    });
}

// Measure two Hensel (modular/bdiv) division algorithms dividing 2n limbs by an odd n-limb
// divisor. `full_q` selects the Q-only shape, whose quotient has the dividend's full length.
fn measure_bdiv_pair(
    n: usize,
    full_q: bool,
    a: &dyn Fn(&mut [Limb], &mut [Limb], &[Limb], Limb),
    b: &dyn Fn(&mut [Limb], &mut [Limb], &[Limb], Limb),
) -> Option<(f64, f64)> {
    if n < 2 {
        return None;
    }
    let inputs: Vec<(Vec<Limb>, Vec<Limb>, Limb)> = (0..INPUT_SETS)
        .map(|k| {
            let ns: Vec<Limb> = random_primitive_ints(EXAMPLE_SEED.fork(&format!("bn{k}")))
                .take(n << 1)
                .collect();
            let mut ds: Vec<Limb> = random_primitive_ints(EXAMPLE_SEED.fork(&format!("bd{k}")))
                .take(n)
                .collect();
            ds[0] |= 1;
            let d_inv = limbs_modular_invert_limb(ds[0]).wrapping_neg();
            (ns, ds, d_inv)
        })
        .collect();
    let qs_len = if full_q { n << 1 } else { n };
    let mut ns_a = vec![0; n << 1];
    let mut ns_b = vec![0; n << 1];
    let mut qs_a = vec![0; qs_len];
    let mut qs_b = vec![0; qs_len];
    for (ns, ds, d_inv) in &inputs {
        ns_a.copy_from_slice(ns);
        a(&mut qs_a, &mut ns_a, ds, *d_inv);
        ns_b.copy_from_slice(ns);
        b(&mut qs_b, &mut ns_b, ds, *d_inv);
    }
    let (mut i, mut j) = (0usize, 0usize);
    Some(interleaved_min_pair(
        &mut || {
            let (ns, ds, d_inv) = &inputs[i & (INPUT_SETS - 1)];
            i += 1;
            ns_a.copy_from_slice(ns);
            a(black_box(&mut qs_a), &mut ns_a, ds, *d_inv);
        },
        &mut || {
            let (ns, ds, d_inv) = &inputs[j & (INPUT_SETS - 1)];
            j += 1;
            ns_b.copy_from_slice(ns);
            b(black_box(&mut qs_b), &mut ns_b, ds, *d_inv);
        },
    ))
}

fn tune_dc_bdiv_qr() {
    find_crossover_spec(
        "DC_BDIV_QR_THRESHOLD",
        "bdiv_qr_schoolbook",
        "bdiv_qr_dc",
        4,
        1500,
        &|n| {
            measure_bdiv_pair(
                n,
                false,
                &|qs, ns, ds, d_inv| {
                    limbs_modular_div_mod_schoolbook(qs, ns, ds, d_inv);
                },
                &|qs, ns, ds, d_inv| {
                    limbs_modular_div_mod_divide_and_conquer(qs, ns, ds, d_inv);
                },
            )
        },
    );
}

fn tune_dc_bdiv_q() {
    find_crossover_spec(
        "DC_BDIV_Q_THRESHOLD",
        "bdiv_q_schoolbook",
        "bdiv_q_dc",
        4,
        1500,
        &|n| {
            measure_bdiv_pair(
                n,
                true,
                &|qs, ns, ds, d_inv| limbs_modular_div_schoolbook(qs, ns, ds, d_inv),
                &|qs, ns, ds, d_inv| limbs_modular_div_divide_and_conquer(qs, ns, ds, d_inv),
            )
        },
    );
}

// Measure DC-vs-Barrett Hensel division at 2n / n (odd divisor). The DC side consumes its
// dividend (refresh copy included); the Barrett side reads it directly and manages its own
// inverse internally.
fn measure_mu_bdiv_pair(
    n: usize,
    full_q: bool,
    dc: &dyn Fn(&mut [Limb], &mut [Limb], &[Limb], Limb),
    barrett: &dyn Fn(&mut [Limb], &mut [Limb], &[Limb], &[Limb], &mut [Limb]),
    barrett_scratch: fn(usize, usize) -> usize,
) -> Option<(f64, f64)> {
    if n < 4 {
        return None;
    }
    let inputs: Vec<(Vec<Limb>, Vec<Limb>, Limb)> = (0..INPUT_SETS)
        .map(|k| {
            let ns: Vec<Limb> = random_primitive_ints(EXAMPLE_SEED.fork(&format!("mbn{k}")))
                .take(n << 1)
                .collect();
            let mut ds: Vec<Limb> = random_primitive_ints(EXAMPLE_SEED.fork(&format!("mbd{k}")))
                .take(n)
                .collect();
            ds[0] |= 1;
            let d_inv = limbs_modular_invert_limb(ds[0]).wrapping_neg();
            (ns, ds, d_inv)
        })
        .collect();
    let qs_len = if full_q { n << 1 } else { n };
    let mut ns_work = vec![0; n << 1];
    let mut qs_a = vec![0; qs_len];
    let mut qs_b = vec![0; qs_len];
    let mut rs = vec![0; n];
    let mut scratch = vec![0; barrett_scratch(n << 1, n)];
    for (ns, ds, d_inv) in &inputs {
        ns_work.copy_from_slice(ns);
        dc(&mut qs_a, &mut ns_work, ds, *d_inv);
        barrett(&mut qs_b, &mut rs, ns, ds, &mut scratch);
    }
    let (mut i, mut j) = (0usize, 0usize);
    Some(interleaved_min_pair(
        &mut || {
            let (ns, ds, d_inv) = &inputs[i & (INPUT_SETS - 1)];
            i += 1;
            ns_work.copy_from_slice(ns);
            dc(black_box(&mut qs_a), &mut ns_work, ds, *d_inv);
        },
        &mut || {
            let (ns, ds, _) = &inputs[j & (INPUT_SETS - 1)];
            j += 1;
            barrett(black_box(&mut qs_b), &mut rs, ns, ds, &mut scratch);
        },
    ))
}

fn tune_mu_bdiv_qr() {
    find_crossover_spec(
        "MU_BDIV_QR_THRESHOLD",
        "dc_bdiv_qr",
        "barrett_bdiv_qr",
        50,
        20000,
        &|n| {
            measure_mu_bdiv_pair(
                n,
                false,
                &|qs, ns, ds, d_inv| {
                    limbs_modular_div_mod_divide_and_conquer(qs, ns, ds, d_inv);
                },
                &|qs, rs, ns, ds, scratch| {
                    limbs_modular_div_mod_barrett(qs, rs, ns, ds, scratch);
                },
                limbs_modular_div_mod_barrett_scratch_len,
            )
        },
    );
}

fn tune_mu_bdiv_q() {
    find_crossover_spec(
        "MU_BDIV_Q_THRESHOLD",
        "dc_bdiv_q",
        "barrett_bdiv_q",
        50,
        8000,
        &|n| {
            measure_mu_bdiv_pair(
                n,
                true,
                &|qs, ns, ds, d_inv| limbs_modular_div_divide_and_conquer(qs, ns, ds, d_inv),
                &|qs, _rs, ns, ds, scratch| limbs_modular_div_barrett(qs, ns, ds, scratch),
                limbs_modular_div_barrett_scratch_len,
            )
        },
    );
}

// Times limbs_modular_invert across sizes, for the BINV_NEWTON_THRESHOLD rebuild-per-candidate
// sweep (the threshold is compiled into the Newton recursion).
fn tune_binv_probe() {
    for n in [400usize, 800, 1600, 3200, 6400] {
        let mut ds = vec![0; n];
        lcg_fill(&mut ds, 9);
        ds[0] |= 1;
        let mut is = vec![0; n];
        let mut scratch = vec![0; limbs_modular_invert_scratch_len(n)];
        let mut best = f64::INFINITY;
        let iters = 1 + ((1u64 << 21) / n as u64);
        for _ in 0..7 {
            let mut f = || {
                limbs_modular_invert(black_box(&mut is), &ds, &mut scratch);
            };
            let t = time_batch(&mut f, iters);
            if t < best {
                best = t;
            }
        }
        println!("binv n={n:<5} {best:>12.1} ns");
    }
}

// Basecase vs divide-and-conquer digit parsing, sweeping the digit count (the threshold is in
// digits, not limbs).
fn tune_from_digits_dc() {
    use malachite_nz::natural::conversion::digits::general_digits::{
        limbs_compute_power_table, limbs_digits_power_table_scratch_len_for_tuning,
        limbs_from_digits_small_base_basecase, limbs_from_digits_small_base_divide_and_conquer,
    };
    find_crossover_spec(
        "FROM_DIGITS_DIVIDE_AND_CONQUER_THRESHOLD",
        "parse_basecase",
        "parse_dc",
        500,
        40000,
        &|digit_count| {
            let ds: Vec<u8> = random_primitive_ints(EXAMPLE_SEED.fork("fd"))
                .take(digit_count)
                .map(|b: u8| b % 10)
                .collect();
            let len = digit_count / 19 + 2;
            let mut power_table_memory =
                vec![0; limbs_digits_power_table_scratch_len_for_tuning(len)];
            let (power_len, powers) =
                limbs_compute_power_table(&mut power_table_memory, len, 10, None);
            let mut out_a = vec![0; len + 1];
            let mut out_b = vec![0; len + 1];
            let mut scratch = vec![0; len + 1 + (Limb::WIDTH as usize)];
            let mut best_a = f64::INFINITY;
            let mut best_b = f64::INFINITY;
            let iters = 1 + ((1u64 << 22) / (digit_count * 12) as u64);
            for _ in 0..7 {
                let mut f = || {
                    black_box(limbs_from_digits_small_base_basecase(
                        black_box(&mut out_a),
                        &ds,
                        10,
                    ));
                };
                let t = time_batch(&mut f, iters);
                if t < best_a {
                    best_a = t;
                }
            }
            for _ in 0..7 {
                let mut f = || {
                    black_box(limbs_from_digits_small_base_divide_and_conquer(
                        black_box(&mut out_b),
                        &ds,
                        10,
                        &powers,
                        power_len,
                        &mut scratch,
                    ));
                };
                let t = time_batch(&mut f, iters);
                if t < best_b {
                    best_b = t;
                }
            }
            Some((best_a, best_b))
        },
    );
}

// The measured Barrett-bdiv crossovers vs DC (120/139) fall below DC_BDIV_*_THRESHOLD (218),
// so DC may have no winning range; these measure Barrett against the real incumbent there.
fn tune_mu_bdiv_qr_vs_schoolbook() {
    find_crossover_spec(
        "MU_BDIV_QR_THRESHOLD",
        "bdiv_qr_schoolbook",
        "barrett_bdiv_qr",
        10,
        2000,
        &|n| {
            measure_mu_bdiv_pair(
                n,
                false,
                &|qs, ns, ds, d_inv| {
                    limbs_modular_div_mod_schoolbook(qs, ns, ds, d_inv);
                },
                &|qs, rs, ns, ds, scratch| {
                    limbs_modular_div_mod_barrett(qs, rs, ns, ds, scratch);
                },
                limbs_modular_div_mod_barrett_scratch_len,
            )
        },
    );
}

fn tune_mu_bdiv_q_vs_schoolbook() {
    find_crossover_spec(
        "MU_BDIV_Q_THRESHOLD",
        "bdiv_q_schoolbook",
        "barrett_bdiv_q",
        10,
        2000,
        &|n| {
            measure_mu_bdiv_pair(
                n,
                true,
                &|qs, ns, ds, d_inv| limbs_modular_div_schoolbook(qs, ns, ds, d_inv),
                &|qs, _rs, ns, ds, scratch| limbs_modular_div_barrett(qs, ns, ds, scratch),
                limbs_modular_div_barrett_scratch_len,
            )
        },
    );
}

// A shootout table over the mod-by-limb kernels: times each applicable kernel at each size for
// normalized and unnormalized (< 2^(W - 2)) divisors, from which the MOD_1* thresholds and
// MOD_1_1P_METHOD are read off manually (the dispatch conditions are multi-way).
fn tune_mod_1_shootout() {
    use malachite_nz::natural::arithmetic::mod_op::{
        limbs_mod_limb_any_leading_zeros_1, limbs_mod_limb_any_leading_zeros_2,
        limbs_mod_limb_at_least_1_leading_zero, limbs_mod_limb_at_least_2_leading_zeros,
        limbs_mod_limb_small_normalized, limbs_mod_limb_small_unnormalized,
    };
    type ModFn = fn(&[Limb], Limb) -> Limb;
    let norm_kernels: [(&str, ModFn); 3] = [
        ("small_norm", |ns, d| {
            limbs_mod_limb_small_normalized::<DoubleLimb, Limb>(ns, d)
        }),
        ("any_lz_1", |ns, d| {
            limbs_mod_limb_any_leading_zeros_1::<DoubleLimb, Limb>(ns, d)
        }),
        ("any_lz_2", |ns, d| {
            limbs_mod_limb_any_leading_zeros_2::<DoubleLimb, Limb>(ns, d)
        }),
    ];
    let unnorm_kernels: [(&str, ModFn); 5] = [
        ("small_unnorm", |ns, d| {
            limbs_mod_limb_small_unnormalized::<DoubleLimb, Limb>(ns, d)
        }),
        ("any_lz_1", |ns, d| {
            limbs_mod_limb_any_leading_zeros_1::<DoubleLimb, Limb>(ns, d)
        }),
        ("any_lz_2", |ns, d| {
            limbs_mod_limb_any_leading_zeros_2::<DoubleLimb, Limb>(ns, d)
        }),
        ("one_lz", |ns, d| {
            limbs_mod_limb_at_least_1_leading_zero::<DoubleLimb, Limb>(ns, d)
        }),
        ("two_lz", |ns, d| {
            limbs_mod_limb_at_least_2_leading_zeros::<DoubleLimb, Limb>(ns, d)
        }),
    ];
    for (label, d_mask, kernels) in [
        ("normalized", !(Limb::MAX >> 1), &norm_kernels[..]),
        ("unnormalized (2 leading zeros)", 0, &unnorm_kernels[..]),
    ] {
        println!("### {label} divisors");
        for n in [2usize, 3, 4, 6, 8, 12, 16, 24, 32, 48, 64, 128] {
            let mut line = format!("n={n:<4}");
            for (name, f) in kernels {
                let inputs: Vec<(Vec<Limb>, Limb)> = (0..INPUT_SETS)
                    .map(|k| {
                        let ns: Vec<Limb> =
                            random_primitive_ints(EXAMPLE_SEED.fork(&format!("m1n{k}")))
                                .take(n)
                                .collect();
                        let mut d: Limb =
                            random_primitive_ints(EXAMPLE_SEED.fork(&format!("m1d{k}")))
                                .next()
                                .unwrap();
                        d >>= 2;
                        d |= d_mask | 1;
                        (ns, d)
                    })
                    .collect();
                let mut best = f64::INFINITY;
                let iters = 1 + ((1u64 << 20) / n as u64);
                for _ in 0..7 {
                    let mut i = 0;
                    let mut g = || {
                        let (ns, d) = &inputs[i & (INPUT_SETS - 1)];
                        i += 1;
                        black_box(f(ns, *d));
                    };
                    let t = time_batch(&mut g, iters);
                    if t < best {
                        best = t;
                    }
                }
                line.push_str(&format!("  {name} {best:>7.1}"));
            }
            println!("{line}");
        }
    }
}

// Times limbs_invert_approx across sizes, for the INV_MULMOD_BNM1_THRESHOLD
// rebuild-per-candidate sweep.
fn tune_invert_probe() {
    for n in [50usize, 100, 200, 400, 800, 1600] {
        let mut ds = vec![0; n];
        lcg_fill(&mut ds, 11);
        ds[n - 1] |= 1 << (Limb::WIDTH - 1);
        let mut is = vec![0; n];
        let mut scratch = vec![0; limbs_invert_approx_scratch_len(n)];
        let mut best = f64::INFINITY;
        let iters = 1 + ((1u64 << 20) / n as u64);
        for _ in 0..7 {
            let mut f = || {
                black_box(limbs_invert_approx(black_box(&mut is), &ds, &mut scratch));
            };
            let t = time_batch(&mut f, iters);
            if t < best {
                best = t;
            }
        }
        println!("invert n={n:<5} {best:>12.1} ns");
    }
}

fn tune_dc_div_qr() {
    find_crossover_spec(
        "DC_DIV_QR_THRESHOLD",
        "schoolbook",
        "divide_and_conquer",
        6,
        500,
        &|n| {
            measure_div_pair(
                n,
                6,
                limbs_div_mod_schoolbook,
                limbs_div_mod_divide_and_conquer,
            )
        },
    );
}

fn tune_dc_divappr_q() {
    find_crossover_spec(
        "DC_DIVAPPR_Q_THRESHOLD",
        "schoolbook_approx",
        "divide_and_conquer_approx",
        6,
        1000,
        &|n| {
            measure_div_pair(
                n,
                6,
                limbs_div_schoolbook_approx,
                limbs_div_divide_and_conquer_approx,
            )
        },
    );
}

// toom4 appears to have no winning range on this machine (toom6 overtakes it below the toom3/toom4
// crossover), so the effective ladder is toom3 -> toom6; this measures that crossover directly.
fn tune_sqr_toom6_vs_toom3() {
    find_crossover(&Level {
        threshold_name: "SQR_TOOM6_THRESHOLD",
        min_size: 60,
        max_size: 6000,
        lower: sqr_toom3_algo(),
        upper: sqr_toom6_algo(),
    });
}

fn tune_sqr_toom8() {
    find_crossover(&Level {
        threshold_name: "SQR_TOOM8_THRESHOLD",
        min_size: 400,
        max_size: 10000,
        lower: sqr_toom6_algo(),
        upper: sqr_toom8_algo(),
    });
}

// ---------------------------------------------------------------------------------------------
// Experimental carry-propagation kernels for limbs_add_same_length_to_out (mpn_add_n analog). These
// exist to compare codegen idioms; the winner gets promoted into natural/arithmetic/add.rs. All are
// #[inline(never)] so they're separately measurable and visible to `cargo asm`.

use malachite_nz::platform::DoubleLimb;

// Variant A: the current library idiom (wrapping_add + comparisons).
#[inline(never)]
fn add_n_current(out: &mut [Limb], xs: &[Limb], ys: &[Limb]) -> bool {
    let mut carry = 0;
    for (out, (&x, &y)) in out.iter_mut().zip(xs.iter().zip(ys.iter())) {
        let result_no_carry = x.wrapping_add(y);
        let result = result_no_carry.wrapping_add(carry);
        carry = Limb::from((result_no_carry < x) || (result < result_no_carry));
        *out = result;
    }
    carry != 0
}

// Variant B: DoubleLimb (u128) accumulator.
#[inline(never)]
fn add_n_double_limb(out: &mut [Limb], xs: &[Limb], ys: &[Limb]) -> bool {
    let mut carry = 0;
    for (out, (&x, &y)) in out.iter_mut().zip(xs.iter().zip(ys.iter())) {
        let sum = DoubleLimb::from(x) + DoubleLimb::from(y) + DoubleLimb::from(carry);
        *out = sum as Limb;
        carry = (sum >> Limb::WIDTH) as Limb;
    }
    carry != 0
}

// Variant C: overflowing_add pair (LLVM's uaddo idiom).
#[inline(never)]
fn add_n_overflowing(out: &mut [Limb], xs: &[Limb], ys: &[Limb]) -> bool {
    let mut carry = false;
    for (out, (&x, &y)) in out.iter_mut().zip(xs.iter().zip(ys.iter())) {
        let (sum, c1) = x.overflowing_add(y);
        let (sum, c2) = sum.overflowing_add(Limb::from(carry));
        carry = c1 | c2;
        *out = sum;
    }
    carry
}

// Variant D: overflowing_add pair, 4x unrolled via chunks_exact.
#[inline(never)]
fn add_n_overflowing_x4(out: &mut [Limb], xs: &[Limb], ys: &[Limb]) -> bool {
    let mut carry = false;
    let mut out_it = out.chunks_exact_mut(4);
    let mut xs_it = xs.chunks_exact(4);
    let mut ys_it = ys.chunks_exact(4);
    for ((o, x), y) in (&mut out_it).zip(&mut xs_it).zip(&mut ys_it) {
        for i in 0..4 {
            let (sum, c1) = x[i].overflowing_add(y[i]);
            let (sum, c2) = sum.overflowing_add(Limb::from(carry));
            carry = c1 | c2;
            o[i] = sum;
        }
    }
    for ((o, &x), &y) in out_it
        .into_remainder()
        .iter_mut()
        .zip(xs_it.remainder().iter())
        .zip(ys_it.remainder().iter())
    {
        let (sum, c1) = x.overflowing_add(y);
        let (sum, c2) = sum.overflowing_add(Limb::from(carry));
        carry = c1 | c2;
        *o = sum;
    }
    carry
}

// Variant E: DoubleLimb accumulator, 4x unrolled.
#[inline(never)]
fn add_n_double_limb_x4(out: &mut [Limb], xs: &[Limb], ys: &[Limb]) -> bool {
    let mut carry = 0;
    let mut out_it = out.chunks_exact_mut(4);
    let mut xs_it = xs.chunks_exact(4);
    let mut ys_it = ys.chunks_exact(4);
    for ((o, x), y) in (&mut out_it).zip(&mut xs_it).zip(&mut ys_it) {
        for i in 0..4 {
            let sum = DoubleLimb::from(x[i]) + DoubleLimb::from(y[i]) + DoubleLimb::from(carry);
            o[i] = sum as Limb;
            carry = (sum >> Limb::WIDTH) as Limb;
        }
    }
    for ((o, &x), &y) in out_it
        .into_remainder()
        .iter_mut()
        .zip(xs_it.remainder().iter())
        .zip(ys_it.remainder().iter())
    {
        let sum = DoubleLimb::from(x) + DoubleLimb::from(y) + DoubleLimb::from(carry);
        *o = sum as Limb;
        carry = (sum >> Limb::WIDTH) as Limb;
    }
    carry != 0
}

// ---------------------------------------------------------------------------------------------
// Experimental shift kernels for limbs_shl_to_out (mpn_lshift analog).

// Variant A: the current library idiom — remaining_bits is loop-carried, serializing iterations.
#[inline(never)]
fn shl_to_out_current(out: &mut [Limb], xs: &[Limb], bits: u64) -> Limb {
    let cobits = Limb::WIDTH - bits;
    let mut remaining_bits = 0;
    for (out, x) in out[..xs.len()].iter_mut().zip(xs.iter()) {
        *out = (x << bits) | remaining_bits;
        remaining_bits = x >> cobits;
    }
    remaining_bits
}

// Variant B: windows form — each output limb depends only on two input limbs, so iterations are
// independent and LLVM is free to unroll/vectorize.
#[inline(never)]
fn shl_to_out_windows(out: &mut [Limb], xs: &[Limb], bits: u64) -> Limb {
    let len = xs.len();
    let cobits = Limb::WIDTH - bits;
    out[0] = xs[0] << bits;
    for (o, w) in out[1..len].iter_mut().zip(xs.windows(2)) {
        *o = (w[1] << bits) | (w[0] >> cobits);
    }
    xs[len - 1] >> cobits
}

// Variant C: windows form, manually 4x unrolled.
#[inline(never)]
fn shl_to_out_windows_x4(out: &mut [Limb], xs: &[Limb], bits: u64) -> Limb {
    let len = xs.len();
    let cobits = Limb::WIDTH - bits;
    out[0] = xs[0] << bits;
    let mut o_chunks = out[1..len].chunks_exact_mut(4);
    let mut i = 0;
    for o in &mut o_chunks {
        for j in 0..4 {
            o[j] = (xs[i + j + 1] << bits) | (xs[i + j] >> cobits);
        }
        i += 4;
    }
    for o in o_chunks.into_remainder() {
        *o = (xs[i + 1] << bits) | (xs[i] >> cobits);
        i += 1;
    }
    xs[len - 1] >> cobits
}

fn tune_shl() {
    type ShlFn = fn(&mut [Limb], &[Limb], u64) -> Limb;
    let variants: [(&str, ShlFn); 3] = [
        ("current", shl_to_out_current),
        ("windows", shl_to_out_windows),
        ("windows_x4", shl_to_out_windows_x4),
    ];
    // Correctness cross-check before timing.
    for n in [1, 2, 5, 17, 100] {
        for bits in [1, 7, 31, 63] {
            let xs: Vec<Limb> = random_primitive_ints(EXAMPLE_SEED.fork("cs"))
                .take(n)
                .collect();
            let mut reference = vec![0; n];
            let ref_carry = variants[0].1(&mut reference, &xs, bits);
            for (name, f) in &variants[1..] {
                let mut out = vec![0; n];
                let carry = f(&mut out, &xs, bits);
                assert_eq!(
                    (out, carry),
                    (reference.clone(), ref_carry),
                    "variant {name} disagrees at n={n}, bits={bits}"
                );
            }
        }
    }
    println!("all variants agree; timing (ns/call, ns/limb):");
    for n in [16usize, 64, 256, 1024, 4096] {
        println!("n = {n}:");
        for (name, f) in &variants {
            let inputs: Vec<Vec<Limb>> = (0..INPUT_SETS)
                .map(|k| {
                    random_primitive_ints(EXAMPLE_SEED.fork(&format!("s{k}")))
                        .take(n)
                        .collect()
                })
                .collect();
            let mut out_a = vec![0; n];
            let mut out_b = vec![0; n];
            for xs in &inputs {
                shl_to_out_current(&mut out_a, xs, 13);
                f(&mut out_b, xs, 13);
            }
            let (mut i, mut j) = (0usize, 0usize);
            let (t_base, t) = interleaved_min_pair(
                &mut || {
                    let xs = &inputs[i & (INPUT_SETS - 1)];
                    i += 1;
                    black_box(shl_to_out_current(black_box(&mut out_a), xs, 13));
                },
                &mut || {
                    let xs = &inputs[j & (INPUT_SETS - 1)];
                    j += 1;
                    black_box(f(black_box(&mut out_b), xs, 13));
                },
            );
            println!(
                "  {name:>12}: {t:>9.1} ns  {:>6.3} ns/limb  (vs current {:>5.2}x)",
                t / n as f64,
                t_base / t,
            );
        }
    }
}

// ---------------------------------------------------------------------------------------------
// div_mod_by_preinversion correction-step shootout. GMP's asm divrem_1 keeps the rare second
// quotient correction off the critical path as a cold branch (~13 insns/limb); LLVM if-converts
// Malachite's version into a long branchless csel chain (~22 insns/limb) on a loop that is
// inherently serial. These variants test whether restructuring recovers the difference.

use malachite_base::num::conversion::traits::{JoinHalves, SplitInHalf};
use malachite_nz::natural::arithmetic::div_mod::{div_mod_by_preinversion, limbs_invert_limb};

// MP_BASES_BIG_BASE_10, the largest power of 10 fitting in a `Limb` (private to the library;
// redeclared here for the shootout): 10^19 for 64-bit limbs, 10^9 for 32-bit limbs.
#[cfg(not(feature = "32_bit_limbs"))]
const BIG_BASE_10: Limb = 0x8ac7230489e80000;
#[cfg(feature = "32_bit_limbs")]
const BIG_BASE_10: Limb = 0x3b9aca00;

// Variant B: GMP-shaped straight-line corrections (first adjustment unconditional on r > q_low,
// second as plain if), letting LLVM choose the lowering.
#[inline]
fn div_mod_preinv_gmp_shape(n_high: Limb, n_low: Limb, d: Limb, d_inv: Limb) -> (Limb, Limb) {
    let (mut q_high, q_low) = (DoubleLimb::from(n_high) * DoubleLimb::from(d_inv))
        .wrapping_add(DoubleLimb::join_halves(n_high.wrapping_add(1), n_low))
        .split_in_half();
    let mut r = n_low.wrapping_sub(q_high.wrapping_mul(d));
    if r > q_low {
        q_high = q_high.wrapping_sub(1);
        r = r.wrapping_add(d);
    }
    if r >= d {
        q_high = q_high.wrapping_add(1);
        r -= d;
    }
    (q_high, r)
}

#[cold]
#[inline(never)]
const fn divrem_second_fixup(q_high: Limb, r: Limb, d: Limb) -> (Limb, Limb) {
    (q_high.wrapping_add(1), r - d)
}

// Variant C: like B, but the rare second correction is outlined into a cold function, forcing a
// real branch and keeping the hot dependency chain short.
#[inline]
fn div_mod_preinv_cold_fixup(n_high: Limb, n_low: Limb, d: Limb, d_inv: Limb) -> (Limb, Limb) {
    let (mut q_high, q_low) = (DoubleLimb::from(n_high) * DoubleLimb::from(d_inv))
        .wrapping_add(DoubleLimb::join_halves(n_high.wrapping_add(1), n_low))
        .split_in_half();
    let mut r = n_low.wrapping_sub(q_high.wrapping_mul(d));
    if r > q_low {
        q_high = q_high.wrapping_sub(1);
        r = r.wrapping_add(d);
    }
    if r >= d {
        return divrem_second_fixup(q_high, r, d);
    }
    (q_high, r)
}

// Variant D: current's nested first adjustment (which beat the GMP shape) plus the cold-outlined
// second correction.
#[inline]
fn div_mod_preinv_hybrid(n_high: Limb, n_low: Limb, d: Limb, d_inv: Limb) -> (Limb, Limb) {
    let (mut q_high, q_low) = (DoubleLimb::from(n_high) * DoubleLimb::from(d_inv))
        .wrapping_add(DoubleLimb::join_halves(n_high.wrapping_add(1), n_low))
        .split_in_half();
    let mut r = n_low.wrapping_sub(q_high.wrapping_mul(d));
    if r > q_low {
        let (r_plus_d, overflow) = r.overflowing_add(d);
        if overflow {
            q_high = q_high.wrapping_sub(1);
            r = r_plus_d;
        }
    } else if r >= d {
        return divrem_second_fixup(q_high, r, d);
    }
    (q_high, r)
}

#[inline(never)]
fn divrem_loop_hybrid(qs: &mut [Limb], ns: &[Limb], d: Limb, d_inv: Limb) -> Limb {
    let mut r = 0;
    for (q, &n) in qs.iter_mut().zip(ns.iter()).rev() {
        (*q, r) = div_mod_preinv_hybrid(r, n, d, d_inv);
    }
    r
}

#[inline(never)]
fn divrem_loop_current(qs: &mut [Limb], ns: &[Limb], d: Limb, d_inv: Limb) -> Limb {
    let mut r = 0;
    for (q, &n) in qs.iter_mut().zip(ns.iter()).rev() {
        (*q, r) = div_mod_by_preinversion(r, n, d, d_inv);
    }
    r
}

#[inline(never)]
fn divrem_loop_gmp_shape(qs: &mut [Limb], ns: &[Limb], d: Limb, d_inv: Limb) -> Limb {
    let mut r = 0;
    for (q, &n) in qs.iter_mut().zip(ns.iter()).rev() {
        (*q, r) = div_mod_preinv_gmp_shape(r, n, d, d_inv);
    }
    r
}

#[inline(never)]
fn divrem_loop_cold_fixup(qs: &mut [Limb], ns: &[Limb], d: Limb, d_inv: Limb) -> Limb {
    let mut r = 0;
    for (q, &n) in qs.iter_mut().zip(ns.iter()).rev() {
        (*q, r) = div_mod_preinv_cold_fixup(r, n, d, d_inv);
    }
    r
}

fn tune_divrem() {
    type DivremFn = fn(&mut [Limb], &[Limb], Limb, Limb) -> Limb;
    let variants: [(&str, DivremFn); 4] = [
        ("current", divrem_loop_current),
        ("gmp_shape", divrem_loop_gmp_shape),
        ("cold_fixup", divrem_loop_cold_fixup),
        ("hybrid", divrem_loop_hybrid),
    ];
    let d = BIG_BASE_10;
    let d_inv = limbs_invert_limb::<DoubleLimb, Limb>(d);
    // Correctness cross-check on many random inputs before timing.
    for k in 0..200 {
        let ns: Vec<Limb> = random_primitive_ints(EXAMPLE_SEED.fork(&format!("dr{k}")))
            .take(17)
            .collect();
        let mut q_ref = vec![0; 17];
        let r_ref = variants[0].1(&mut q_ref, &ns, d, d_inv);
        for (name, f) in &variants[1..] {
            let mut q = vec![0; 17];
            let r = f(&mut q, &ns, d, d_inv);
            assert_eq!(
                (q, r),
                (q_ref.clone(), r_ref),
                "variant {name} disagrees, seed {k}"
            );
        }
    }
    println!("all variants agree; timing (ns/call, ns/limb):");
    for n in [16usize, 64, 256, 1024] {
        println!("n = {n}:");
        for (name, f) in &variants {
            let inputs: Vec<Vec<Limb>> = (0..INPUT_SETS)
                .map(|k| {
                    random_primitive_ints(EXAMPLE_SEED.fork(&format!("dv{k}")))
                        .take(n)
                        .collect()
                })
                .collect();
            let mut qs_a = vec![0; n];
            let mut qs_b = vec![0; n];
            for xs in &inputs {
                divrem_loop_current(&mut qs_a, xs, d, d_inv);
                f(&mut qs_b, xs, d, d_inv);
            }
            let (mut i, mut j) = (0usize, 0usize);
            let (t_base, t) = interleaved_min_pair(
                &mut || {
                    let xs = &inputs[i & (INPUT_SETS - 1)];
                    i += 1;
                    black_box(divrem_loop_current(black_box(&mut qs_a), xs, d, d_inv));
                },
                &mut || {
                    let xs = &inputs[j & (INPUT_SETS - 1)];
                    j += 1;
                    black_box(f(black_box(&mut qs_b), xs, d, d_inv));
                },
            );
            println!(
                "  {name:>12}: {t:>9.1} ns  {:>6.3} ns/limb  (vs current {:>5.2}x)",
                t / n as f64,
                t_base / t,
            );
        }
    }
}

// ---------------------------------------------------------------------------------------------
// GET_STR_PRECOMPUTE_THRESHOLD: at what size does building a power table + divide-and-conquer beat
// the O(n^2) basecase? The power-table construction is timed inside the candidate, since the real
// dispatch pays it on every conversion.

fn tune_get_str_precompute() {
    use malachite_nz::natural::conversion::digits::general_digits::{
        digits_in_base_per_limb_for_tuning, get_chars_per_limb, limbs_compute_power_table,
        limbs_digits_power_table_scratch_len_for_tuning, limbs_to_digits_small_base_basecase,
        limbs_to_digits_small_base_divide_and_conquer_for_tuning,
        limbs_to_digits_small_base_divide_and_conquer_scratch_len_for_tuning,
    };
    const BASE: u64 = 10;
    // The basecase asserts xs_len < GET_STR_PRECOMPUTE_THRESHOLD (its stack buffers are sized by
    // it), so the scan is capped just below the compiled-in value; lower the constant and rebuild
    // to scan higher.
    let max_size = if malachite_nz::natural::arithmetic::mul::toom::TUNE_PROGRAM_BUILD {
        // GET_STR_THRESHOLD_LIMIT: the basecase's lifted buffer bound under TUNE_PROGRAM_BUILD.
        150
    } else {
        malachite_nz::natural::conversion::digits::general_digits::GET_STR_PRECOMPUTE_THRESHOLD - 1
    };
    find_crossover(&Level {
        threshold_name: "GET_STR_PRECOMPUTE_THRESHOLD",
        min_size: 4,
        max_size,
        lower: Algo {
            name: "basecase",
            valid: &|_| true,
            scratch_len: &|_| 0,
            run: &|out_limbs, xs, _, _| {
                // out is digit bytes; reuse the limb out buffer as raw space via a local. The
                // basecase writes u8 digits; we keep a thread-local-free local buffer per call
                // shape by transmuting sizes — simplest is a fixed buffer.
                let mut digits = [0u8; 64 * 20];
                let mut xs_copy = [0; 64];
                let n = xs.len();
                xs_copy[..n].copy_from_slice(xs);
                black_box(limbs_to_digits_small_base_basecase(
                    &mut digits[..n * 20],
                    0,
                    &xs_copy[..n],
                    BASE,
                ));
                // Touch out_limbs so the Algo signature stays uniform.
                black_box(&out_limbs[0]);
            },
        },
        upper: Algo {
            name: "powtab+dc",
            valid: &|_| true,
            scratch_len: &|_| 0,
            run: &|out_limbs, xs, _, _| {
                let n = xs.len();
                let mut digits = [0u8; 64 * 20];
                let mut xs_copy = [0; 64];
                xs_copy[..n].copy_from_slice(xs);
                let mut power_table_memory =
                    vec![0; limbs_digits_power_table_scratch_len_for_tuning(n)];
                let digits_len = digits_in_base_per_limb_for_tuning(n, BASE);
                let len = 1 + usize::try_from(digits_len).unwrap() / get_chars_per_limb(BASE);
                let (power_len, powers) =
                    limbs_compute_power_table(&mut power_table_memory, len, BASE, None);
                let mut scratch = vec![
                        0;
                        limbs_to_digits_small_base_divide_and_conquer_scratch_len_for_tuning(n)
                    ];
                black_box(limbs_to_digits_small_base_divide_and_conquer_for_tuning(
                    &mut digits[..n * 20],
                    &mut xs_copy[..n],
                    BASE,
                    &powers,
                    power_len,
                    &mut scratch,
                ));
                black_box(&out_limbs[0]);
            },
        },
    });
}

// Probe for GET_STR_DC_THRESHOLD grid search: times the full powtab+dc conversion at fixed sizes.
// The DC threshold is a compiled-in constant controlling recursion leaf size, so the driver
// (perf/tune.sh or a loop) rebuilds with each candidate value and compares these numbers.
fn tune_get_str_dc_probe() {
    use malachite_nz::natural::conversion::digits::general_digits::{
        digits_in_base_per_limb_for_tuning, get_chars_per_limb, limbs_compute_power_table,
        limbs_digits_power_table_scratch_len_for_tuning,
        limbs_to_digits_small_base_divide_and_conquer_for_tuning,
        limbs_to_digits_small_base_divide_and_conquer_scratch_len_for_tuning,
    };
    const BASE: u64 = 10;
    for n in [32usize, 48, 63] {
        let inputs: Vec<Vec<Limb>> = (0..INPUT_SETS)
            .map(|k| {
                random_primitive_ints(EXAMPLE_SEED.fork(&format!("g{k}")))
                    .take(n)
                    .collect()
            })
            .collect();
        let run = |xs: &[Limb]| {
            let mut digits = [0u8; 64 * 20];
            let mut xs_copy = [0; 64];
            xs_copy[..n].copy_from_slice(xs);
            let mut power_table_memory =
                vec![0; limbs_digits_power_table_scratch_len_for_tuning(n)];
            let digits_len = digits_in_base_per_limb_for_tuning(n, BASE);
            let len = 1 + usize::try_from(digits_len).unwrap() / get_chars_per_limb(BASE);
            let (power_len, powers) =
                limbs_compute_power_table(&mut power_table_memory, len, BASE, None);
            let mut scratch =
                vec![0; limbs_to_digits_small_base_divide_and_conquer_scratch_len_for_tuning(n)];
            black_box(limbs_to_digits_small_base_divide_and_conquer_for_tuning(
                &mut digits[..n * 20],
                &mut xs_copy[..n],
                BASE,
                &powers,
                power_len,
                &mut scratch,
            ));
        };
        // Control series: the basecase at a fixed size, which does not depend on
        // GET_STR_DC_THRESHOLD. Reporting the ratio dc/control makes numbers comparable across
        // rebuilds and runs, canceling out core-scheduling and frequency effects.
        let control_inputs: Vec<Vec<Limb>> = (0..INPUT_SETS)
            .map(|k| {
                random_primitive_ints(EXAMPLE_SEED.fork(&format!("c{k}")))
                    .take(20)
                    .collect()
            })
            .collect();
        let control = |xs: &[Limb]| {
            let mut digits = [0u8; 20 * 20];
            black_box(limbs_to_digits_small_base_basecase(
                &mut digits,
                0,
                xs,
                BASE,
            ));
        };
        for xs in &inputs {
            run(xs);
        }
        for xs in &control_inputs {
            control(xs);
        }
        let mut i = 0usize;
        let mut j = 0usize;
        let (t, t_control) = interleaved_min_pair(
            &mut || {
                let xs = &inputs[i & (INPUT_SETS - 1)];
                i += 1;
                run(xs);
            },
            &mut || {
                let xs = &control_inputs[j & (INPUT_SETS - 1)];
                j += 1;
                control(xs);
            },
        );
        println!(
            "dc_probe n={n}: {t:.1} ns, control {t_control:.1} ns, ratio {:.3}",
            t / t_control
        );
    }
}

// ---------------------------------------------------------------------------------------------
// Allocating-shl shootout: does avoiding the vec![0; n] zero-init pass via Vec::extend pay off, or
// does the iterator plumbing defeat vectorization / the TrustedLen specialization?

// Variant A: zero-init then overwrite (current limbs_shl shape). One "wasted" memset pass, but the
// fill loop is a plain slice loop with reliable codegen.
#[inline(never)]
fn shl_vec_zeroinit(xs: &[Limb], bits: u64) -> Vec<Limb> {
    let len = xs.len();
    let cobits = Limb::WIDTH - bits;
    let mut out = vec![0; len + 1];
    out[0] = xs[0] << bits;
    for (o, w) in out[1..len].iter_mut().zip(xs.windows(2)) {
        *o = (w[1] << bits) | (w[0] >> cobits);
    }
    let carry = xs[len - 1] >> cobits;
    if carry == 0 {
        out.pop();
    } else {
        *out.last_mut().unwrap() = carry;
    }
    out
}

// Variant B: no zero-init; elements are written exactly once via extend. Relies on the TrustedLen
// specialization and on LLVM dissolving the Windows/Map iterator chain.
#[inline(never)]
fn shl_vec_extend(xs: &[Limb], bits: u64) -> Vec<Limb> {
    let len = xs.len();
    let cobits = Limb::WIDTH - bits;
    let mut out = Vec::with_capacity(len + 1);
    out.push(xs[0] << bits);
    out.extend(xs.windows(2).map(|w| (w[1] << bits) | (w[0] >> cobits)));
    let carry = xs[len - 1] >> cobits;
    if carry != 0 {
        out.push(carry);
    }
    out
}

fn tune_shl_alloc() {
    type ShlVecFn = fn(&[Limb], u64) -> Vec<Limb>;
    let variants: [(&str, ShlVecFn); 2] =
        [("zeroinit", shl_vec_zeroinit), ("extend", shl_vec_extend)];
    // Correctness cross-check before timing.
    for n in [1, 2, 5, 17, 100] {
        for bits in [1, 7, 31, 63] {
            let xs: Vec<Limb> = random_primitive_ints(EXAMPLE_SEED.fork("ca"))
                .take(n)
                .collect();
            let reference = variants[0].1(&xs, bits);
            for (name, f) in &variants[1..] {
                assert_eq!(
                    f(&xs, bits),
                    reference,
                    "variant {name} disagrees at n={n}, bits={bits}"
                );
            }
        }
    }
    println!("all variants agree; timing includes alloc+drop (ns/call, ns/limb):");
    for n in [16usize, 64, 256, 1024, 4096] {
        println!("n = {n}:");
        for (name, f) in &variants {
            let inputs: Vec<Vec<Limb>> = (0..INPUT_SETS)
                .map(|k| {
                    random_primitive_ints(EXAMPLE_SEED.fork(&format!("a{k}")))
                        .take(n)
                        .collect()
                })
                .collect();
            for xs in &inputs {
                black_box(shl_vec_zeroinit(xs, 13));
                black_box(f(xs, 13));
            }
            let (mut i, mut j) = (0usize, 0usize);
            let (t_base, t) = interleaved_min_pair(
                &mut || {
                    let xs = &inputs[i & (INPUT_SETS - 1)];
                    i += 1;
                    black_box(shl_vec_zeroinit(black_box(xs), 13));
                },
                &mut || {
                    let xs = &inputs[j & (INPUT_SETS - 1)];
                    j += 1;
                    black_box(f(black_box(xs), 13));
                },
            );
            println!(
                "  {name:>10}: {t:>9.1} ns  {:>6.3} ns/limb  (vs zeroinit {:>5.2}x)",
                t / n as f64,
                t_base / t,
            );
        }
    }
}

fn tune_add() {
    type AddFn = fn(&mut [Limb], &[Limb], &[Limb]) -> bool;
    let variants: [(&str, AddFn); 5] = [
        ("current", add_n_current),
        ("double_limb", add_n_double_limb),
        ("overflowing", add_n_overflowing),
        ("overflowing_x4", add_n_overflowing_x4),
        ("double_limb_x4", add_n_double_limb_x4),
    ];
    // Correctness cross-check before timing.
    for n in [1, 3, 4, 7, 17, 100] {
        let xs: Vec<Limb> = random_primitive_ints(EXAMPLE_SEED.fork("cx"))
            .take(n)
            .collect();
        let ys: Vec<Limb> = random_primitive_ints(EXAMPLE_SEED.fork("cy"))
            .take(n)
            .collect();
        let mut reference = vec![0; n];
        let ref_carry = variants[0].1(&mut reference, &xs, &ys);
        for (name, f) in &variants[1..] {
            let mut out = vec![0; n];
            let carry = f(&mut out, &xs, &ys);
            assert_eq!(
                (out, carry),
                (reference.clone(), ref_carry),
                "variant {name} disagrees at n={n}"
            );
        }
    }
    println!("all variants agree; timing (ns/call, ns/limb):");
    for n in [16usize, 64, 256, 1024, 4096] {
        println!("n = {n}:");
        // Measure each variant interleaved with the current idiom as the common baseline.
        for (name, f) in &variants {
            let inputs: Vec<(Vec<Limb>, Vec<Limb>)> = (0..INPUT_SETS)
                .map(|k| {
                    let xs = random_primitive_ints(EXAMPLE_SEED.fork(&format!("x{k}")))
                        .take(n)
                        .collect();
                    let ys = random_primitive_ints(EXAMPLE_SEED.fork(&format!("y{k}")))
                        .take(n)
                        .collect();
                    (xs, ys)
                })
                .collect();
            let mut out_a = vec![0; n];
            let mut out_b = vec![0; n];
            for (xs, ys) in &inputs {
                add_n_current(&mut out_a, xs, ys);
                f(&mut out_b, xs, ys);
            }
            let (mut i, mut j) = (0usize, 0usize);
            let (t_base, t) = interleaved_min_pair(
                &mut || {
                    let (xs, ys) = &inputs[i & (INPUT_SETS - 1)];
                    i += 1;
                    black_box(add_n_current(black_box(&mut out_a), xs, ys));
                },
                &mut || {
                    let (xs, ys) = &inputs[j & (INPUT_SETS - 1)];
                    j += 1;
                    black_box(f(black_box(&mut out_b), xs, ys));
                },
            );
            println!(
                "  {name:>16}: {t:>9.1} ns  {:>6.3} ns/limb  (vs current {:>5.2}x)",
                t / n as f64,
                t_base / t,
            );
        }
    }
}

/// Dispatch a tuning run by key. Keys mirror the bottom-up tuning order; after each level, write
/// the suggested value into platform_64.rs and rebuild before tuning the next level (perf/tune.sh
/// automates this).
pub fn tune(key: &str) {
    match key {
        "add" => tune_add(),
        "shl" => tune_shl(),
        "shl_alloc" => tune_shl_alloc(),
        "get_str_precompute" => tune_get_str_precompute(),
        "get_str_dc_probe" => tune_get_str_dc_probe(),
        "divrem" => tune_divrem(),
        "mul_toom22" => tune_mul_toom22(),
        "mul_toom33" => tune_mul_toom33(),
        "mul_toom44" => tune_mul_toom44(),
        "mul_toom6h" => tune_mul_toom6h(),
        "mul_toom6h_vs_toom33" => tune_mul_toom6h_vs_toom33(),
        "mul_toom8h" => tune_mul_toom8h(),
        "mul_toom32_to_toom43" => tune_mul_toom32_to_toom43(),
        "mul_toom32_to_toom53" => tune_mul_toom32_to_toom53(),
        "mul_toom42_to_toom53" => tune_mul_toom42_to_toom53(),
        "mul_toom42_to_toom63" => tune_mul_toom42_to_toom63(),
        "dc_div_qr" => tune_dc_div_qr(),
        "dc_divappr_q" => tune_dc_divappr_q(),
        "mullo_dc" => tune_mullo_dc(),
        "mullo_mul_n" => tune_mullo_mul_n(),
        "sqrlo_dc" => tune_sqrlo_dc(),
        "dc_bdiv_qr" => tune_dc_bdiv_qr(),
        "dc_bdiv_q" => tune_dc_bdiv_q(),
        "mu_bdiv_qr" => tune_mu_bdiv_qr(),
        "mu_bdiv_q" => tune_mu_bdiv_q(),
        "binv_probe" => tune_binv_probe(),
        "from_digits_dc" => tune_from_digits_dc(),
        "mu_bdiv_qr_vs_schoolbook" => tune_mu_bdiv_qr_vs_schoolbook(),
        "mu_bdiv_q_vs_schoolbook" => tune_mu_bdiv_q_vs_schoolbook(),
        "mod_1_shootout" => tune_mod_1_shootout(),
        "invert_probe" => tune_invert_probe(),
        "mul_fft_probe" => tune_mul_fft_probe(),
        "small_kernel_probe" => tune_small_kernel_probe(),
        "gcd_probe" => tune_gcd_probe(),
        "xgcd_probe" => tune_xgcd_probe(),
        "mul_fft" => tune_mul_fft(),
        "sqr_fft" => tune_sqr_fft(),
        "mul_fft_vs_toom6h" => tune_mul_fft_vs_toom6h(),
        "sqr_fft_vs_toom6" => tune_sqr_fft_vs_toom6(),
        "fft_small_check" => tune_fft_small_check(),
        "inv_newton" => tune_inv_newton(),
        "mu_div_qr" => tune_mu_div_qr(),
        "mu_divappr_q" => tune_mu_divappr_q(),
        "sqr_toom2" => tune_sqr_toom2(),
        "sqr_toom3" => tune_sqr_toom3(),
        "sqr_toom4" => tune_sqr_toom4(),
        "sqr_toom6" => tune_sqr_toom6(),
        "sqr_toom6_vs_toom3" => tune_sqr_toom6_vs_toom3(),
        "sqr_toom8" => tune_sqr_toom8(),
        "mul" => {
            tune_mul_toom22();
            tune_mul_toom33();
            tune_mul_toom44();
            println!();
            println!("NOTE: levels above toom22 were measured with the COMPILED-IN lower");
            println!("thresholds. Apply the suggestions to platform_64.rs bottom-up, rebuild,");
            println!("and re-run until stable (perf/tune.sh does this).");
        }
        _ => panic!("Invalid tune key: {key}"),
    }
}
