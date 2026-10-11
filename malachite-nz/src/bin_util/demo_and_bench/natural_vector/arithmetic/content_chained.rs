// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::arithmetic::traits::ContentChained;
use malachite_base::test_util::bench::{BenchmarkType, run_benchmark};
use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::runner::Runner;
use malachite_nz::test_util::bench::bucketers::pair_1_natural_vector_bit_bucketer;
use malachite_nz::test_util::generators::natural_vector_natural_pair_gen;

pub(crate) fn register(runner: &mut Runner) {
    register_demo!(runner, demo_natural_vector_content_chained);
    register_demo!(runner, demo_natural_vector_content_chained_val_ref);
    register_demo!(runner, demo_natural_vector_content_chained_ref_val);
    register_demo!(runner, demo_natural_vector_content_chained_ref_ref);
    register_bench!(
        runner,
        benchmark_natural_vector_content_chained_evaluation_strategy
    );
}

fn demo_natural_vector_content_chained(gm: GenMode, config: &GenConfig, limit: usize) {
    for (v, x) in natural_vector_natural_pair_gen()
        .get(gm, config)
        .take(limit)
    {
        let v_old = v.clone();
        let x_old = x.clone();
        println!(
            "{v_old}.content_chained({x_old}) = {}",
            v.content_chained(x)
        );
    }
}

fn demo_natural_vector_content_chained_val_ref(gm: GenMode, config: &GenConfig, limit: usize) {
    for (v, x) in natural_vector_natural_pair_gen()
        .get(gm, config)
        .take(limit)
    {
        let v_old = v.clone();
        println!("{v_old}.content_chained(&{x}) = {}", v.content_chained(&x));
    }
}

fn demo_natural_vector_content_chained_ref_val(gm: GenMode, config: &GenConfig, limit: usize) {
    for (v, x) in natural_vector_natural_pair_gen()
        .get(gm, config)
        .take(limit)
    {
        let x_old = x.clone();
        println!(
            "(&{v}).content_chained({x_old}) = {}",
            (&v).content_chained(x)
        );
    }
}

fn demo_natural_vector_content_chained_ref_ref(gm: GenMode, config: &GenConfig, limit: usize) {
    for (v, x) in natural_vector_natural_pair_gen()
        .get(gm, config)
        .take(limit)
    {
        println!(
            "(&{v}).content_chained(&{x}) = {}",
            (&v).content_chained(&x)
        );
    }
}

fn benchmark_natural_vector_content_chained_evaluation_strategy(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "NaturalVector.content_chained(Natural)",
        BenchmarkType::EvaluationStrategy,
        natural_vector_natural_pair_gen().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_1_natural_vector_bit_bucketer("v"),
        &mut [
            ("NaturalVector.content_chained(Natural)", &mut |(v, x)| {
                no_out!(v.content_chained(x));
            }),
            ("NaturalVector.content_chained(&Natural)", &mut |(v, x)| {
                no_out!(v.content_chained(&x));
            }),
            (
                "(&NaturalVector).content_chained(Natural)",
                &mut |(v, x)| {
                    no_out!((&v).content_chained(x));
                },
            ),
            (
                "(&NaturalVector).content_chained(&Natural)",
                &mut |(v, x)| {
                    no_out!((&v).content_chained(&x));
                },
            ),
        ],
    );
}
