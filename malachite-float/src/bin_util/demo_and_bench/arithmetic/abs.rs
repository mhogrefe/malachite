// Copyright © 2025 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::arithmetic::traits::{Abs, AbsAssign};
use malachite_base::test_util::bench::{run_benchmark, BenchmarkType};
use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::runner::Runner;
use malachite_float::test_util::bench::bucketers::{
    float_complexity_bucketer, pair_2_float_complexity_bucketer,
};
use malachite_float::test_util::generators::{float_gen, float_gen_rm, float_gen_var_12};
use malachite_float::{ComparableFloat, ComparableFloatRef};

pub(crate) fn register(runner: &mut Runner) {
    register_demo!(runner, demo_float_abs_negative_zero);
    register_demo!(runner, demo_float_abs_negative_zero_debug);
    register_demo!(runner, demo_float_abs_negative_zero_extreme);
    register_demo!(runner, demo_float_abs_negative_zero_extreme_debug);
    register_demo!(runner, demo_float_abs_negative_zero_ref);
    register_demo!(runner, demo_float_abs_negative_zero_ref_debug);
    register_demo!(runner, demo_float_abs_negative_zero_assign);
    register_demo!(runner, demo_float_abs_negative_zero_assign_debug);
    register_demo!(runner, demo_float_abs);
    register_demo!(runner, demo_float_abs_debug);
    register_demo!(runner, demo_float_abs_extreme);
    register_demo!(runner, demo_float_abs_extreme_debug);
    register_demo!(runner, demo_float_abs_ref);
    register_demo!(runner, demo_float_abs_ref_debug);
    register_demo!(runner, demo_float_abs_assign);
    register_demo!(runner, demo_float_abs_assign_debug);

    register_bench!(
        runner,
        benchmark_float_abs_negative_zero_evaluation_strategy
    );
    register_bench!(runner, benchmark_float_abs_negative_zero_assign);
    register_bench!(runner, benchmark_float_abs_library_comparison);
    register_bench!(runner, benchmark_float_abs_evaluation_strategy);
    register_bench!(runner, benchmark_float_abs_assign);
}

fn demo_float_abs_negative_zero(gm: GenMode, config: &GenConfig, limit: usize) {
    for n in float_gen().get(gm, config).take(limit) {
        println!(
            "abs_negative_zero({}) = {}",
            n.clone(),
            n.abs_negative_zero()
        );
    }
}

fn demo_float_abs_negative_zero_debug(gm: GenMode, config: &GenConfig, limit: usize) {
    for n in float_gen().get(gm, config).take(limit) {
        println!(
            "abs_negative_zero({:#x}) = {:#x}",
            ComparableFloat(n.clone()),
            ComparableFloat(n.abs_negative_zero())
        );
    }
}

fn demo_float_abs_negative_zero_extreme(gm: GenMode, config: &GenConfig, limit: usize) {
    for n in float_gen_var_12().get(gm, config).take(limit) {
        println!(
            "abs_negative_zero({}) = {}",
            n.clone(),
            n.abs_negative_zero()
        );
    }
}

fn demo_float_abs_negative_zero_extreme_debug(gm: GenMode, config: &GenConfig, limit: usize) {
    for n in float_gen_var_12().get(gm, config).take(limit) {
        println!(
            "abs_negative_zero({:#x}) = {:#x}",
            ComparableFloat(n.clone()),
            ComparableFloat(n.abs_negative_zero())
        );
    }
}

fn demo_float_abs_negative_zero_ref(gm: GenMode, config: &GenConfig, limit: usize) {
    for n in float_gen().get(gm, config).take(limit) {
        println!(
            "abs_negative_zero_ref(&{}) = {}",
            n,
            n.abs_negative_zero_ref()
        );
    }
}

fn demo_float_abs_negative_zero_ref_debug(gm: GenMode, config: &GenConfig, limit: usize) {
    for n in float_gen().get(gm, config).take(limit) {
        println!(
            "abs_negative_zero(&{:#x}) = {:#x}",
            ComparableFloatRef(&n),
            ComparableFloat(n.abs_negative_zero_ref())
        );
    }
}

fn demo_float_abs_negative_zero_assign(gm: GenMode, config: &GenConfig, limit: usize) {
    for mut n in float_gen().get(gm, config).take(limit) {
        let n_old = n.clone();
        n.abs_negative_zero_assign();
        println!("n := {n_old}; n.abs_negative_zero_assign(); n = {n}");
    }
}

fn demo_float_abs_negative_zero_assign_debug(gm: GenMode, config: &GenConfig, limit: usize) {
    for mut n in float_gen().get(gm, config).take(limit) {
        let n_old = n.clone();
        n.abs_negative_zero_assign();
        println!(
            "n := {:#x}; n.abs_negative_zero_assign(); n = {:#x}",
            ComparableFloat(n_old),
            ComparableFloat(n)
        );
    }
}

fn demo_float_abs(gm: GenMode, config: &GenConfig, limit: usize) {
    for n in float_gen().get(gm, config).take(limit) {
        println!("|{}| = {}", n.clone(), n.abs());
    }
}

fn demo_float_abs_debug(gm: GenMode, config: &GenConfig, limit: usize) {
    for n in float_gen().get(gm, config).take(limit) {
        println!(
            "|{:#x}| = {:#x}",
            ComparableFloat(n.clone()),
            ComparableFloat(n.abs())
        );
    }
}

fn demo_float_abs_extreme(gm: GenMode, config: &GenConfig, limit: usize) {
    for n in float_gen_var_12().get(gm, config).take(limit) {
        println!("|{}| = {}", n.clone(), n.abs());
    }
}

fn demo_float_abs_extreme_debug(gm: GenMode, config: &GenConfig, limit: usize) {
    for n in float_gen_var_12().get(gm, config).take(limit) {
        println!(
            "|{:#x}| = {:#x}",
            ComparableFloat(n.clone()),
            ComparableFloat(n.abs())
        );
    }
}

fn demo_float_abs_ref(gm: GenMode, config: &GenConfig, limit: usize) {
    for n in float_gen().get(gm, config).take(limit) {
        println!("|&{}| = {}", n, (&n).abs());
    }
}

fn demo_float_abs_ref_debug(gm: GenMode, config: &GenConfig, limit: usize) {
    for n in float_gen().get(gm, config).take(limit) {
        println!(
            "|&{:#x}| = {:#x}",
            ComparableFloatRef(&n),
            ComparableFloat((&n).abs())
        );
    }
}

fn demo_float_abs_assign(gm: GenMode, config: &GenConfig, limit: usize) {
    for mut n in float_gen().get(gm, config).take(limit) {
        let n_old = n.clone();
        n.abs_assign();
        println!("n := {n_old}; n.abs_assign(); n = {n}");
    }
}

fn demo_float_abs_assign_debug(gm: GenMode, config: &GenConfig, limit: usize) {
    for mut n in float_gen().get(gm, config).take(limit) {
        let n_old = n.clone();
        n.abs_assign();
        println!(
            "n := {:#x}; n.abs_assign(); n = {:#x}",
            ComparableFloat(n_old),
            ComparableFloat(n)
        );
    }
}

fn benchmark_float_abs_negative_zero_evaluation_strategy(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Rational.abs_negative_zero()",
        BenchmarkType::EvaluationStrategy,
        float_gen().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &float_complexity_bucketer("x"),
        &mut [
            ("Rational.abs_negative_zero()", &mut |n| {
                no_out!(n.abs_negative_zero())
            }),
            ("(&Rational).abs_negative_zero_ref()", &mut |n| {
                no_out!(n.abs_negative_zero_ref())
            }),
        ],
    );
}

fn benchmark_float_abs_negative_zero_assign(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Rational.abs_negative_zero_assign()",
        BenchmarkType::Single,
        float_gen().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &float_complexity_bucketer("x"),
        &mut [("Malachite", &mut |mut n| n.abs_negative_zero_assign())],
    );
}

#[allow(unused_must_use)]
fn benchmark_float_abs_library_comparison(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Rational.abs()",
        BenchmarkType::LibraryComparison,
        float_gen_rm().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_2_float_complexity_bucketer("x"),
        &mut [
            ("Malachite", &mut |(_, n)| no_out!(n.abs())),
            ("rug", &mut |(n, _)| no_out!(n.abs())),
        ],
    );
}

fn benchmark_float_abs_evaluation_strategy(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Rational.abs()",
        BenchmarkType::EvaluationStrategy,
        float_gen().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &float_complexity_bucketer("x"),
        &mut [
            ("Rational.abs()", &mut |n| no_out!(n.abs())),
            ("(&Rational).abs()", &mut |n| no_out!((&n).abs())),
        ],
    );
}

fn benchmark_float_abs_assign(gm: GenMode, config: &GenConfig, limit: usize, file_name: &str) {
    run_benchmark(
        "Rational.abs_assign()",
        BenchmarkType::Single,
        float_gen().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &float_complexity_bucketer("x"),
        &mut [("Malachite", &mut |mut n| n.abs_assign())],
    );
}
