// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::arithmetic::traits::Crt;
use malachite_base::test_util::bench::{BenchmarkType, run_benchmark};
use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::runner::Runner;
use malachite_nz::test_util::bench::bucketers::quadruple_natural_max_bit_bucketer;
use malachite_nz::test_util::generators::natural_quadruple_gen_var_5;
use malachite_nz::test_util::natural::arithmetic::crt::crt_simple;

pub(crate) fn register(runner: &mut Runner) {
    register_demo!(runner, demo_natural_crt);
    register_demo!(runner, demo_natural_crt_ref);

    register_bench!(runner, benchmark_natural_crt_evaluation_strategy);
    register_bench!(runner, benchmark_natural_crt_algorithms);
}

fn demo_natural_crt(gm: GenMode, config: &GenConfig, limit: usize) {
    for (r1, m1, r2, m2) in natural_quadruple_gen_var_5().get(gm, config).take(limit) {
        let r1_old = r1.clone();
        let m1_old = m1.clone();
        let r2_old = r2.clone();
        let m2_old = m2.clone();
        println!(
            "{r1_old}.crt({m1_old}, {r2_old}, {m2_old}) = {:?}",
            r1.crt(m1, r2, m2)
        );
    }
}

fn demo_natural_crt_ref(gm: GenMode, config: &GenConfig, limit: usize) {
    for (r1, m1, r2, m2) in natural_quadruple_gen_var_5().get(gm, config).take(limit) {
        println!(
            "(&{r1}).crt(&{m1}, &{r2}, &{m2}) = {:?}",
            (&r1).crt(&m1, &r2, &m2)
        );
    }
}

fn benchmark_natural_crt_evaluation_strategy(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Natural.crt(Natural, Natural, Natural)",
        BenchmarkType::EvaluationStrategy,
        natural_quadruple_gen_var_5().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &quadruple_natural_max_bit_bucketer("r1", "m1", "r2", "m2"),
        &mut [
            (
                "Natural.crt(Natural, Natural, Natural)",
                &mut |(r1, m1, r2, m2)| {
                    no_out!(r1.crt(m1, r2, m2));
                },
            ),
            (
                "(&Natural).crt(&Natural, &Natural, &Natural)",
                &mut |(r1, m1, r2, m2)| {
                    no_out!((&r1).crt(&m1, &r2, &m2));
                },
            ),
        ],
    );
}

fn benchmark_natural_crt_algorithms(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Natural.crt(Natural, Natural, Natural)",
        BenchmarkType::Algorithms,
        natural_quadruple_gen_var_5().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &quadruple_natural_max_bit_bucketer("r1", "m1", "r2", "m2"),
        &mut [
            ("default", &mut |(r1, m1, r2, m2)| {
                no_out!(r1.crt(m1, r2, m2));
            }),
            ("simple", &mut |(r1, m1, r2, m2)| {
                no_out!(crt_simple(r1, m1, r2, m2));
            }),
        ],
    );
}
