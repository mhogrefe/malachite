// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::arithmetic::traits::Abs;
use malachite_base::num::comparison::traits::EqAbs;
use malachite_base::test_util::bench::{BenchmarkType, run_benchmark};
use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::runner::Runner;
use malachite_float::ComparableFloatRef;
use malachite_float::test_util::bench::bucketers::*;
use malachite_float::test_util::generators::{
    float_rational_pair_gen, float_rational_pair_gen_var_2,
};

pub(crate) fn register(runner: &mut Runner) {
    register_demo!(runner, demo_float_eq_abs_rational);
    register_demo!(runner, demo_float_eq_abs_rational_debug);
    register_demo!(runner, demo_float_eq_abs_rational_extreme);
    register_demo!(runner, demo_float_eq_abs_rational_extreme_debug);
    register_demo!(runner, demo_rational_eq_abs_float);
    register_demo!(runner, demo_rational_eq_abs_float_debug);
    register_demo!(runner, demo_rational_eq_abs_float_extreme);
    register_demo!(runner, demo_rational_eq_abs_float_extreme_debug);

    register_bench!(runner, benchmark_float_eq_abs_rational_algorithms);
    register_bench!(runner, benchmark_rational_eq_abs_float_algorithms);
}

fn demo_float_eq_abs_rational(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, y) in float_rational_pair_gen().get(gm, config).take(limit) {
        if x.eq_abs(&y) {
            println!("|{x}| = |{y}|");
        } else {
            println!("|{x}| ≠ |{y}|");
        }
    }
}

fn demo_float_eq_abs_rational_debug(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, y) in float_rational_pair_gen().get(gm, config).take(limit) {
        let cx = ComparableFloatRef(&x);
        if x.eq_abs(&y) {
            println!("|{cx:#x}| = |{y}|");
        } else {
            println!("|{cx:#x}| ≠ |{y}|");
        }
    }
}

fn demo_float_eq_abs_rational_extreme(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, y) in float_rational_pair_gen_var_2().get(gm, config).take(limit) {
        if x.eq_abs(&y) {
            println!("|{x}| = |{y}|");
        } else {
            println!("|{x}| ≠ |{y}|");
        }
    }
}

fn demo_float_eq_abs_rational_extreme_debug(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, y) in float_rational_pair_gen_var_2().get(gm, config).take(limit) {
        let cx = ComparableFloatRef(&x);
        if x.eq_abs(&y) {
            println!("|{cx:#x}| = |{y}|");
        } else {
            println!("|{cx:#x}| ≠ |{y}|");
        }
    }
}

fn demo_rational_eq_abs_float(gm: GenMode, config: &GenConfig, limit: usize) {
    for (y, x) in float_rational_pair_gen().get(gm, config).take(limit) {
        if x.eq_abs(&y) {
            println!("|{x}| = |{y}|");
        } else {
            println!("|{x}| ≠ |{y}|");
        }
    }
}

fn demo_rational_eq_abs_float_debug(gm: GenMode, config: &GenConfig, limit: usize) {
    for (y, x) in float_rational_pair_gen().get(gm, config).take(limit) {
        let cy = ComparableFloatRef(&y);
        if x.eq_abs(&y) {
            println!("|{x}| = |{cy:#x}|");
        } else {
            println!("|{x}| ≠ |{cy:#x}|");
        }
    }
}

fn demo_rational_eq_abs_float_extreme(gm: GenMode, config: &GenConfig, limit: usize) {
    for (y, x) in float_rational_pair_gen_var_2().get(gm, config).take(limit) {
        if x.eq_abs(&y) {
            println!("|{x}| = |{y}|");
        } else {
            println!("|{x}| ≠ |{y}|");
        }
    }
}

fn demo_rational_eq_abs_float_extreme_debug(gm: GenMode, config: &GenConfig, limit: usize) {
    for (y, x) in float_rational_pair_gen_var_2().get(gm, config).take(limit) {
        let cy = ComparableFloatRef(&y);
        if x.eq_abs(&y) {
            println!("|{x}| = |{cy:#x}|");
        } else {
            println!("|{x}| ≠ |{cy:#x}|");
        }
    }
}

#[allow(clippy::no_effect, unused_must_use)]
fn benchmark_float_eq_abs_rational_algorithms(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Float.eq_abs(&Rational)",
        BenchmarkType::Algorithms,
        float_rational_pair_gen().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_float_rational_max_complexity_bucketer("x", "y"),
        &mut [
            ("default", &mut |(x, y)| no_out!(x.eq_abs(&y))),
            ("using abs", &mut |(x, y)| no_out!(x.abs() == y.abs())),
        ],
    );
}

#[allow(clippy::no_effect, unused_must_use)]
fn benchmark_rational_eq_abs_float_algorithms(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Rational.eq_abs(&Float)",
        BenchmarkType::Algorithms,
        float_rational_pair_gen().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_float_rational_max_complexity_bucketer("y", "x"),
        &mut [
            ("default", &mut |(y, x)| no_out!(x.eq_abs(&y))),
            ("using abs", &mut |(y, x)| no_out!(x.abs() == y.abs())),
        ],
    );
}
