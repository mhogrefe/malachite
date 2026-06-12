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
use malachite_nz::natural::arithmetic::mul::limbs_mul_greater_to_out_basecase;
use malachite_nz::natural::arithmetic::mul::toom::{
    limbs_mul_greater_to_out_toom_22, limbs_mul_greater_to_out_toom_22_input_sizes_valid,
    limbs_mul_greater_to_out_toom_22_scratch_len, limbs_mul_greater_to_out_toom_33,
    limbs_mul_greater_to_out_toom_33_input_sizes_valid,
    limbs_mul_greater_to_out_toom_33_scratch_len, limbs_mul_greater_to_out_toom_44,
    limbs_mul_greater_to_out_toom_44_input_sizes_valid,
    limbs_mul_greater_to_out_toom_44_scratch_len,
};
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
    let mut dat = Vec::new();
    let mut since_change = 0;
    let mut consecutive_upper_wins = 0;
    let mut last_thresh = None;
    let mut size = c.min_size as f64;
    println!(
        "tuning {} ({} -> {})",
        c.threshold_name, c.lower.name, c.upper.name
    );
    while (size as usize) < c.max_size {
        let n = size as usize;
        size = f64::max(size * 1.05, size + 1.0);
        let Some((tl, tu)) = measure_mul_pair(n, &c.lower, &c.upper) else {
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
            "  size {n:>6}  {} {tl:>10.1}ns  {} {tu:>10.1}ns  d {d:>7.4}  -> {}",
            c.lower.name,
            c.upper.name,
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
            if since_change > 40 {
                break;
            }
        } else {
            since_change = 0;
            last_thresh = thresh;
        }
    }
    match analyze(&dat) {
        None => println!(
            "  {}: upper algorithm never wins below {}",
            c.threshold_name, c.max_size
        ),
        Some(t) => println!("pub(crate) const {}: usize = {};", c.threshold_name, t),
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

// MP_BASES_BIG_BASE_10 (10^19), private to the library; redeclared here for the shootout.
const BIG_BASE_10: Limb = 0x8ac7230489e80000;

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
// GET_STR_PRECOMPUTE_THRESHOLD: at what size does building a power table + divide-and-conquer
// beat the O(n^2) basecase? The power-table construction is timed inside the candidate, since
// the real dispatch pays it on every conversion.

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
    let max_size =
        malachite_nz::natural::conversion::digits::general_digits::GET_STR_PRECOMPUTE_THRESHOLD - 1;
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

// Probe for GET_STR_DC_THRESHOLD grid search: times the full powtab+dc conversion at fixed
// sizes. The DC threshold is a compiled-in constant controlling recursion leaf size, so the
// driver (perf/tune.sh or a loop) rebuilds with each candidate value and compares these numbers.
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
            use malachite_nz::natural::conversion::digits::general_digits::limbs_to_digits_small_base_basecase;
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
