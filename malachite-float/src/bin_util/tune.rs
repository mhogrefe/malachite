// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

//! Precision-threshold tuning for `Float` functions with an asymptotically fast tier, after
//! MPFR's `tune/tuneup.c`. The measurement machinery is shared with `malachite-nz`'s tuner: see
//! `malachite_base::test_util::bench::tune`.
//!
//! Each tuner times the basic and fast tiers of a function head-to-head at increasing precisions
//! on identical inputs, and reports the crossover as a `const` declaration to paste into the
//! source. Inputs follow MPFR's `tuneup`: random mantissas of the target precision with exponent
//! 0, i.e. values in [1/2, 1).
//!
//! Usage: `cargo run --release --features bin_build -p malachite-float -- -g tune_sincos` (also
//! `tune_sin` and `tune_cos`, which share `SINCOS_THRESHOLD` with `sin_cos` as in MPFR, so their
//! crossovers are for information). Results are garbage on a busy machine.

use malachite_base::num::basic::integers::PrimitiveInt;
use malachite_base::num::random::random_primitive_ints;
use malachite_base::random::EXAMPLE_SEED;
use malachite_base::rounding_modes::RoundingMode::Nearest;
use malachite_base::test_util::bench::tune::{
    INPUT_SETS, find_crossover_spec, interleaved_min_pair,
};
use malachite_float::Float;
use malachite_float::test_util::float::arithmetic::sin_cos::{
    cos_basic_for_tuning, cos_fast_for_tuning, sin_basic_for_tuning, sin_cos_basic_for_tuning,
    sin_cos_fast_for_tuning, sin_fast_for_tuning,
};
use malachite_nz::natural::Natural;
use malachite_nz::platform::Limb;
use std::hint::black_box;

// `INPUT_SETS` random `Float`s of precision `prec` in [1/2, 1).
fn random_inputs(prec: u64) -> Vec<Float> {
    (0..INPUT_SETS)
        .map(|k| {
            let limbs = random_primitive_ints::<Limb>(EXAMPLE_SEED.fork(&format!("x{k}")))
                .take(usize::try_from(prec.div_ceil(Limb::WIDTH)).unwrap())
                .collect::<Vec<_>>();
            let n = Natural::from_owned_limbs_asc(limbs)
                >> (prec.div_ceil(Limb::WIDTH) * Limb::WIDTH - prec);
            // the top bit set, then scaled to [1/2, 1)
            let n = n | (Natural::from(1u32) << (prec - 1));
            Float::from_natural_prec(n, prec).0 >> prec
        })
        .collect()
}

// Measures the basic and fast tiers of a unary function at precision `prec`.
fn measure_pair<T>(
    prec: u64,
    basic: &dyn Fn(&Float, u64) -> T,
    fast: &dyn Fn(&Float, u64) -> T,
) -> (f64, f64) {
    let inputs = random_inputs(prec);
    // warmup
    for x in &inputs {
        black_box(basic(x, prec));
        black_box(fast(x, prec));
    }
    let (mut i, mut j) = (0usize, 0usize);
    interleaved_min_pair(
        &mut || {
            let x = &inputs[i & (INPUT_SETS - 1)];
            i += 1;
            black_box(basic(black_box(x), prec));
        },
        &mut || {
            let x = &inputs[j & (INPUT_SETS - 1)];
            j += 1;
            black_box(fast(black_box(x), prec));
        },
    )
}

fn tune_sincos() {
    find_crossover_spec(
        "SINCOS_THRESHOLD",
        "u64",
        "sin_cos basic",
        "sin_cos fast",
        2000,
        300000,
        &|p| {
            Some(measure_pair(
                u64::try_from(p).unwrap(),
                &|x, prec| sin_cos_basic_for_tuning(x, prec, Nearest),
                &|x, prec| sin_cos_fast_for_tuning(x, prec, Nearest),
            ))
        },
    );
}

fn tune_sin() {
    find_crossover_spec(
        "SIN_THRESHOLD",
        "u64",
        "sin basic",
        "sin fast",
        2000,
        300000,
        &|p| {
            Some(measure_pair(
                u64::try_from(p).unwrap(),
                &|x, prec| sin_basic_for_tuning(x, prec, Nearest),
                &|x, prec| sin_fast_for_tuning(x, prec, Nearest),
            ))
        },
    );
}

fn tune_cos() {
    find_crossover_spec(
        "COS_THRESHOLD",
        "u64",
        "cos basic",
        "cos fast",
        2000,
        300000,
        &|p| {
            Some(measure_pair(
                u64::try_from(p).unwrap(),
                &|x, prec| cos_basic_for_tuning(x, prec, Nearest),
                &|x, prec| cos_fast_for_tuning(x, prec, Nearest),
            ))
        },
    );
}

pub fn tune(key: &str) {
    match key {
        "sincos" => tune_sincos(),
        "sin" => tune_sin(),
        "cos" => tune_cos(),
        _ => panic!("Unrecognized tune key: {key}"),
    }
}
