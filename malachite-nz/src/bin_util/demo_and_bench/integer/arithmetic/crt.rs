// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::arithmetic::traits::BalancedCrt;
use malachite_base::test_util::bench::{BenchmarkType, run_benchmark};
use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::runner::Runner;
use malachite_nz::test_util::bench::bucketers::*;
use malachite_nz::test_util::generators::integer_natural_natural_natural_quadruple_gen_var_1;
use malachite_nz::test_util::integer::arithmetic::crt::balanced_crt_simple;

pub(crate) fn register(runner: &mut Runner) {
    register_demo!(runner, demo_integer_balanced_crt);
    register_demo!(runner, demo_integer_balanced_crt_ref);

    register_bench!(runner, benchmark_integer_balanced_crt_evaluation_strategy);
    register_bench!(runner, benchmark_integer_balanced_crt_algorithms);
}

fn demo_integer_balanced_crt(gm: GenMode, config: &GenConfig, limit: usize) {
    for (r1, m1, r2, m2) in integer_natural_natural_natural_quadruple_gen_var_1()
        .get(gm, config)
        .take(limit)
    {
        let r1_old = r1.clone();
        let m1_old = m1.clone();
        let r2_old = r2.clone();
        let m2_old = m2.clone();
        println!(
            "{r1_old}.balanced_crt({m1_old}, {r2_old}, {m2_old}) = {:?}",
            r1.balanced_crt(m1, r2, m2)
        );
    }
}

fn demo_integer_balanced_crt_ref(gm: GenMode, config: &GenConfig, limit: usize) {
    for (r1, m1, r2, m2) in integer_natural_natural_natural_quadruple_gen_var_1()
        .get(gm, config)
        .take(limit)
    {
        println!(
            "(&{r1}).balanced_crt(&{m1}, &{r2}, &{m2}) = {:?}",
            (&r1).balanced_crt(&m1, &r2, &m2)
        );
    }
}

fn benchmark_integer_balanced_crt_evaluation_strategy(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Integer.balanced_crt(Natural, Natural, Natural)",
        BenchmarkType::EvaluationStrategy,
        integer_natural_natural_natural_quadruple_gen_var_1().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &quadruple_integer_natural_natural_natural_max_bit_bucketer("r1", "m1", "r2", "m2"),
        &mut [
            (
                "Integer.balanced_crt(Natural, Natural, Natural)",
                &mut |(r1, m1, r2, m2)| {
                    no_out!(r1.balanced_crt(m1, r2, m2));
                },
            ),
            (
                "(&Integer).balanced_crt(&Natural, &Natural, &Natural)",
                &mut |(r1, m1, r2, m2)| {
                    no_out!((&r1).balanced_crt(&m1, &r2, &m2));
                },
            ),
        ],
    );
}

fn benchmark_integer_balanced_crt_algorithms(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Integer.balanced_crt(Natural, Natural, Natural)",
        BenchmarkType::Algorithms,
        integer_natural_natural_natural_quadruple_gen_var_1().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &quadruple_integer_natural_natural_natural_max_bit_bucketer("r1", "m1", "r2", "m2"),
        &mut [
            ("default", &mut |(r1, m1, r2, m2)| {
                no_out!(r1.balanced_crt(m1, r2, m2));
            }),
            ("simple", &mut |(r1, m1, r2, m2)| {
                no_out!(balanced_crt_simple(r1, m1, r2, m2));
            }),
        ],
    );
}
