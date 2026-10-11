// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use core::iter::Product;
use malachite_base::test_util::bench::bucketers::vec_len_bucketer;
use malachite_base::test_util::bench::{BenchmarkType, run_benchmark};
use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::generators::unsigned_vec_gen;
use malachite_base::test_util::runner::Runner;
use malachite_nz::natural::Natural;
use malachite_nz::test_util::natural::arithmetic::mul::natural_product_of_unsigneds_naive;

pub(crate) fn register(runner: &mut Runner) {
    register_demo!(runner, demo_natural_product_of_unsigneds);
    register_demo!(runner, demo_natural_product_of_unsigneds_ref);
    register_bench!(runner, benchmark_natural_product_of_unsigneds_algorithms);
    register_bench!(
        runner,
        benchmark_natural_product_of_unsigneds_evaluation_strategy
    );
}

fn demo_natural_product_of_unsigneds(gm: GenMode, config: &GenConfig, limit: usize) {
    for xs in unsigned_vec_gen::<u64>().get(gm, config).take(limit) {
        println!(
            "product({:?}) = {}",
            xs,
            Natural::product(xs.iter().copied())
        );
    }
}

fn demo_natural_product_of_unsigneds_ref(gm: GenMode, config: &GenConfig, limit: usize) {
    for xs in unsigned_vec_gen::<u64>().get(gm, config).take(limit) {
        println!("product({:?}) = {}", xs, Natural::product(xs.iter()));
    }
}

fn benchmark_natural_product_of_unsigneds_algorithms(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Natural::product(Iterator<Item=u64>)",
        BenchmarkType::Algorithms,
        unsigned_vec_gen::<u64>().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &vec_len_bucketer(),
        &mut [
            ("default", &mut |xs| {
                no_out!(Natural::product(xs.into_iter()));
            }),
            ("Natural::product of Naturals", &mut |xs| {
                no_out!(Natural::product(xs.into_iter().map(Natural::from)));
            }),
            ("naive", &mut |xs| {
                no_out!(natural_product_of_unsigneds_naive(&xs));
            }),
        ],
    );
}

fn benchmark_natural_product_of_unsigneds_evaluation_strategy(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Natural::product(Iterator<Item=u64>)",
        BenchmarkType::EvaluationStrategy,
        unsigned_vec_gen::<u64>().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &vec_len_bucketer(),
        &mut [
            ("Natural::product(Iterator<Item=u64>)", &mut |xs| {
                no_out!(Natural::product(xs.into_iter()));
            }),
            ("Natural::product(Iterator<Item=&u64>)", &mut |xs| {
                no_out!(Natural::product(xs.iter()));
            }),
        ],
    );
}
