// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::arithmetic::traits::{CanonicalizeUnit, CanonicalizeUnitAssign};
use malachite_base::test_util::bench::{BenchmarkType, run_benchmark};
use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::runner::Runner;
use malachite_nz::test_util::bench::bucketers::natural_bit_bucketer;
use malachite_nz::test_util::generators::natural_gen;

pub(crate) fn register(runner: &mut Runner) {
    register_demo!(runner, demo_natural_canonicalize_unit);
    register_demo!(runner, demo_natural_canonicalize_unit_ref);
    register_demo!(runner, demo_natural_canonicalize_unit_assign);

    register_bench!(
        runner,
        benchmark_natural_canonicalize_unit_evaluation_strategy
    );
}

fn demo_natural_canonicalize_unit(gm: GenMode, config: &GenConfig, limit: usize) {
    for x in natural_gen().get(gm, config).take(limit) {
        println!(
            "({}).canonicalize_unit() = {}",
            x.clone(),
            x.canonicalize_unit()
        );
    }
}

fn demo_natural_canonicalize_unit_ref(gm: GenMode, config: &GenConfig, limit: usize) {
    for x in natural_gen().get(gm, config).take(limit) {
        println!(
            "(&{}).canonicalize_unit() = {}",
            x,
            (&x).canonicalize_unit()
        );
    }
}

fn demo_natural_canonicalize_unit_assign(gm: GenMode, config: &GenConfig, limit: usize) {
    for mut x in natural_gen().get(gm, config).take(limit) {
        let old_x = x.clone();
        x.canonicalize_unit_assign();
        println!("x := {old_x}; x.canonicalize_unit_assign(); x = {x}");
    }
}

#[allow(unused_must_use)]
fn benchmark_natural_canonicalize_unit_evaluation_strategy(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Natural.canonicalize_unit()",
        BenchmarkType::EvaluationStrategy,
        natural_gen().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &natural_bit_bucketer("x"),
        &mut [
            ("Natural.canonicalize_unit()", &mut |x| {
                no_out!(x.canonicalize_unit());
            }),
            ("(&Natural).canonicalize_unit()", &mut |x| {
                no_out!((&x).canonicalize_unit());
            }),
        ],
    );
}
