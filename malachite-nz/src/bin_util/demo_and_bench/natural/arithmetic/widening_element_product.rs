// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::arithmetic::traits::WideningElementProduct;
use malachite_base::test_util::bench::bucketers::unsigned_vector_dimension_bucketer;
use malachite_base::test_util::bench::{BenchmarkType, run_benchmark};
use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::generators::unsigned_vector_gen;
use malachite_base::test_util::runner::Runner;
use malachite_nz::natural::Natural;

pub(crate) fn register(runner: &mut Runner) {
    register_demo!(runner, demo_unsigned_vector_to_widening_element_product);
    register_demo!(runner, demo_unsigned_vector_into_widening_element_product);
    register_bench!(
        runner,
        benchmark_unsigned_vector_widening_element_product_evaluation_strategy
    );
}

fn demo_unsigned_vector_to_widening_element_product(gm: GenMode, config: &GenConfig, limit: usize) {
    for v in unsigned_vector_gen().get(gm, config).take(limit) {
        let product: Natural = v.to_widening_element_product();
        println!("{v}.to_widening_element_product() = {product}");
    }
}

fn demo_unsigned_vector_into_widening_element_product(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for v in unsigned_vector_gen().get(gm, config).take(limit) {
        let v_old = v.clone();
        let product: Natural = v.into_widening_element_product();
        println!("{v_old}.into_widening_element_product() = {product}");
    }
}

fn benchmark_unsigned_vector_widening_element_product_evaluation_strategy(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "UnsignedVector<u64>.to_widening_element_product()",
        BenchmarkType::EvaluationStrategy,
        unsigned_vector_gen().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &unsigned_vector_dimension_bucketer("v"),
        &mut [
            (
                "UnsignedVector<u64>.to_widening_element_product()",
                &mut |v| {
                    let product: Natural = v.to_widening_element_product();
                    no_out!(product);
                },
            ),
            (
                "UnsignedVector<u64>.into_widening_element_product()",
                &mut |v| {
                    let product: Natural = v.into_widening_element_product();
                    no_out!(product);
                },
            ),
        ],
    );
}
