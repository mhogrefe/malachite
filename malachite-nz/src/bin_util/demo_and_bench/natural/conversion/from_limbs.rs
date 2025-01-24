// Copyright © 2025 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::test_util::bench::bucketers::vec_len_bucketer;
use malachite_base::test_util::bench::{run_benchmark, BenchmarkType};
use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::generators::unsigned_vec_gen;
use malachite_base::test_util::runner::Runner;
use malachite_nz::natural::Natural;

pub(crate) fn register(runner: &mut Runner) {
    register_demo!(runner, demo_natural_from_limbs_asc);
    register_demo!(runner, demo_natural_from_limbs_desc);
    register_demo!(runner, demo_natural_from_owned_limbs_asc);
    register_demo!(runner, demo_natural_from_owned_limbs_desc);
    register_bench!(runner, benchmark_natural_from_limbs_asc_evaluation_strategy);
    register_bench!(
        runner,
        benchmark_natural_from_limbs_desc_evaluation_strategy
    );
}

fn demo_natural_from_limbs_asc(gm: GenMode, config: &GenConfig, limit: usize) {
    for xs in unsigned_vec_gen().get(gm, config).take(limit) {
        println!(
            "from_limbs_asc({:?}) = {:?}",
            xs,
            Natural::from_limbs_asc(&xs)
        );
    }
}

fn demo_natural_from_limbs_desc(gm: GenMode, config: &GenConfig, limit: usize) {
    for xs in unsigned_vec_gen().get(gm, config).take(limit) {
        println!(
            "from_limbs_desc({:?}) = {:?}",
            xs,
            Natural::from_limbs_desc(&xs)
        );
    }
}

fn demo_natural_from_owned_limbs_asc(gm: GenMode, config: &GenConfig, limit: usize) {
    for xs in unsigned_vec_gen().get(gm, config).take(limit) {
        println!(
            "from_owned_limbs_asc({:?}) = {:?}",
            xs,
            Natural::from_owned_limbs_asc(xs.clone())
        );
    }
}

fn demo_natural_from_owned_limbs_desc(gm: GenMode, config: &GenConfig, limit: usize) {
    for xs in unsigned_vec_gen().get(gm, config).take(limit) {
        println!(
            "from_owned_limbs_desc({:?}) = {:?}",
            xs,
            Natural::from_owned_limbs_desc(xs.clone())
        );
    }
}

fn benchmark_natural_from_limbs_asc_evaluation_strategy(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Natural.from_limbs_asc(&[Limb])",
        BenchmarkType::EvaluationStrategy,
        unsigned_vec_gen().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &vec_len_bucketer(),
        &mut [
            ("Natural::from_limbs_asc(&[u32])", &mut |ref xs| {
                no_out!(Natural::from_limbs_asc(xs))
            }),
            ("Natural::from_owned_limbs_asc(&[u32])", &mut |xs| {
                no_out!(Natural::from_owned_limbs_asc(xs))
            }),
        ],
    );
}

fn benchmark_natural_from_limbs_desc_evaluation_strategy(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Natural.from_limbs_desc(&[Limb])",
        BenchmarkType::EvaluationStrategy,
        unsigned_vec_gen().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &vec_len_bucketer(),
        &mut [
            ("Natural::from_limbs_desc(&[u32])", &mut |ref xs| {
                no_out!(Natural::from_limbs_desc(xs))
            }),
            ("Natural::from_owned_limbs_desc(&[u32])", &mut |xs| {
                no_out!(Natural::from_owned_limbs_desc(xs))
            }),
        ],
    );
}
