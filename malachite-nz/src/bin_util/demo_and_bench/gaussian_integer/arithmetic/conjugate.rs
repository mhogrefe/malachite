// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::arithmetic::traits::{Conjugate, ConjugateAssign};
use malachite_base::test_util::bench::{BenchmarkType, run_benchmark};
use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::runner::Runner;
use malachite_nz::test_util::bench::bucketers::gaussian_integer_bit_bucketer;
use malachite_nz::test_util::generators::gaussian_integer_gen;

pub(crate) fn register(runner: &mut Runner) {
    register_demo!(runner, demo_gaussian_integer_conjugate);
    register_demo!(runner, demo_gaussian_integer_conjugate_ref);
    register_demo!(runner, demo_gaussian_integer_conjugate_assign);

    register_bench!(
        runner,
        benchmark_gaussian_integer_conjugate_evaluation_strategy
    );
}

fn demo_gaussian_integer_conjugate(gm: GenMode, config: &GenConfig, limit: usize) {
    for x in gaussian_integer_gen().get(gm, config).take(limit) {
        println!("conjugate({}) = {}", x.clone(), x.conjugate());
    }
}

fn demo_gaussian_integer_conjugate_ref(gm: GenMode, config: &GenConfig, limit: usize) {
    for x in gaussian_integer_gen().get(gm, config).take(limit) {
        println!("conjugate(&{}) = {}", x, (&x).conjugate());
    }
}

fn demo_gaussian_integer_conjugate_assign(gm: GenMode, config: &GenConfig, limit: usize) {
    for mut x in gaussian_integer_gen().get(gm, config).take(limit) {
        let old_x = x.clone();
        x.conjugate_assign();
        println!("x := {old_x}; x.conjugate_assign(); x = {x}");
    }
}

#[allow(unused_must_use)]
fn benchmark_gaussian_integer_conjugate_evaluation_strategy(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "GaussianInteger.conjugate()",
        BenchmarkType::EvaluationStrategy,
        gaussian_integer_gen().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &gaussian_integer_bit_bucketer("x"),
        &mut [
            ("GaussianInteger.conjugate()", &mut |x| {
                no_out!(x.conjugate());
            }),
            ("(&GaussianInteger).conjugate()", &mut |x| {
                no_out!((&x).conjugate());
            }),
        ],
    );
}
