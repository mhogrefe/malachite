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
use malachite_float::test_util::bench::bucketers::pair_float_max_complexity_bucketer;
use malachite_float::test_util::generators::{float_pair_gen, float_pair_gen_var_10};
use malachite_float::{ComparableFloat, ComparableFloatRef};

pub(crate) fn register(runner: &mut Runner) {
    register_demo!(runner, demo_float_eq_abs);
    register_demo!(runner, demo_float_eq_abs_debug);
    register_demo!(runner, demo_float_eq_abs_extreme);
    register_demo!(runner, demo_float_eq_abs_extreme_debug);
    register_demo!(runner, demo_comparable_float_eq_abs);
    register_demo!(runner, demo_comparable_float_eq_abs_debug);
    register_demo!(runner, demo_comparable_float_eq_abs_extreme);
    register_demo!(runner, demo_comparable_float_eq_abs_extreme_debug);
    register_demo!(runner, demo_comparable_float_ref_eq_abs);
    register_demo!(runner, demo_comparable_float_ref_eq_abs_debug);

    register_bench!(runner, benchmark_float_eq_abs_algorithms);
    register_bench!(runner, benchmark_comparable_float_eq_abs_algorithms);
    register_bench!(runner, benchmark_comparable_float_ref_eq_abs_algorithms);
}

fn demo_float_eq_abs(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, y) in float_pair_gen().get(gm, config).take(limit) {
        if x.eq_abs(&y) {
            println!("|{x}| = |{y}|");
        } else {
            println!("|{x}| ≠ |{y}|");
        }
    }
}

fn demo_float_eq_abs_debug(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, y) in float_pair_gen().get(gm, config).take(limit) {
        let cx = ComparableFloatRef(&x);
        let cy = ComparableFloatRef(&y);
        if x.eq_abs(&y) {
            println!("|{cx:#x}| = |{cy:#x}|");
        } else {
            println!("|{cx:#x}| ≠ |{cy:#x}|");
        }
    }
}

fn demo_float_eq_abs_extreme(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, y) in float_pair_gen_var_10().get(gm, config).take(limit) {
        if x.eq_abs(&y) {
            println!("|{x}| = |{y}|");
        } else {
            println!("|{x}| ≠ |{y}|");
        }
    }
}

fn demo_float_eq_abs_extreme_debug(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, y) in float_pair_gen_var_10().get(gm, config).take(limit) {
        let cx = ComparableFloatRef(&x);
        let cy = ComparableFloatRef(&y);
        if x.eq_abs(&y) {
            println!("|{cx:#x}| = |{cy:#x}|");
        } else {
            println!("|{cx:#x}| ≠ |{cy:#x}|");
        }
    }
}

fn demo_comparable_float_eq_abs(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, y) in float_pair_gen().get(gm, config).take(limit) {
        let cx = ComparableFloat(x.clone());
        let cy = ComparableFloat(y.clone());
        if cx.eq_abs(&cy) {
            println!("|{x}| = |{y}|");
        } else {
            println!("|{x}| ≠ |{y}|");
        }
    }
}

fn demo_comparable_float_eq_abs_debug(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, y) in float_pair_gen().get(gm, config).take(limit) {
        let cx = ComparableFloat(x.clone());
        let cy = ComparableFloat(y.clone());
        if cx.eq_abs(&cy) {
            println!("|{cx:#x}| = |{cy:#x}|");
        } else {
            println!("|{cx:#x}| ≠ |{cy:#x}|");
        }
    }
}

fn demo_comparable_float_eq_abs_extreme(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, y) in float_pair_gen_var_10().get(gm, config).take(limit) {
        let cx = ComparableFloat(x.clone());
        let cy = ComparableFloat(y.clone());
        if cx.eq_abs(&cy) {
            println!("|{x}| = |{y}|");
        } else {
            println!("|{x}| ≠ |{y}|");
        }
    }
}

fn demo_comparable_float_eq_abs_extreme_debug(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, y) in float_pair_gen_var_10().get(gm, config).take(limit) {
        let cx = ComparableFloat(x.clone());
        let cy = ComparableFloat(y.clone());
        if cx.eq_abs(&cy) {
            println!("|{cx:#x}| = |{cy:#x}|");
        } else {
            println!("|{cx:#x}| ≠ |{cy:#x}|");
        }
    }
}

fn demo_comparable_float_ref_eq_abs(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, y) in float_pair_gen().get(gm, config).take(limit) {
        let cx = ComparableFloatRef(&x);
        let cy = ComparableFloatRef(&y);
        if cx.eq_abs(&cy) {
            println!("|{x}| = |{y}|");
        } else {
            println!("|{x}| ≠ |{y}|");
        }
    }
}

fn demo_comparable_float_ref_eq_abs_debug(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, y) in float_pair_gen().get(gm, config).take(limit) {
        let cx = ComparableFloatRef(&x);
        let cy = ComparableFloatRef(&y);
        if cx.eq_abs(&cy) {
            println!("|{cx:#x}| = |{cy:#x}|");
        } else {
            println!("|{cx:#x}| ≠ |{cy:#x}|");
        }
    }
}

#[allow(clippy::no_effect, unused_must_use)]
fn benchmark_float_eq_abs_algorithms(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Float.eq_abs(&Float)",
        BenchmarkType::Algorithms,
        float_pair_gen().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_float_max_complexity_bucketer("x", "y"),
        &mut [
            ("default", &mut |(x, y)| no_out!(x.eq_abs(&y))),
            ("using abs", &mut |(x, y)| no_out!(x.abs() == y.abs())),
        ],
    );
}

#[allow(clippy::no_effect, unused_must_use)]
fn benchmark_comparable_float_eq_abs_algorithms(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "ComparableFloat.eq_abs(&ComparableFloat)",
        BenchmarkType::Algorithms,
        float_pair_gen().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_float_max_complexity_bucketer("x", "y"),
        &mut [
            ("default", &mut |(x, y)| {
                no_out!(ComparableFloat(x).eq_abs(&ComparableFloat(y)));
            }),
            ("default", &mut |(x, y)| {
                no_out!(ComparableFloat(x.abs()) == ComparableFloat(y.abs()));
            }),
        ],
    );
}

#[allow(clippy::no_effect, unused_must_use)]
fn benchmark_comparable_float_ref_eq_abs_algorithms(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "ComparableFloatRef.eq_abs(&ComparableFloatRef)",
        BenchmarkType::Algorithms,
        float_pair_gen().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_float_max_complexity_bucketer("x", "y"),
        &mut [
            ("default", &mut |(x, y)| {
                no_out!(ComparableFloatRef(&x).eq_abs(&ComparableFloatRef(&y)));
            }),
            ("default", &mut |(x, y)| {
                no_out!(ComparableFloatRef(&x.abs()) == ComparableFloatRef(&y.abs()));
            }),
        ],
    );
}
