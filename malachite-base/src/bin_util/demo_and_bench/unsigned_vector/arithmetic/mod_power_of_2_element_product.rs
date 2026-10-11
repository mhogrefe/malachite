// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::arithmetic::traits::ModPowerOf2ElementProduct;
use malachite_base::test_util::bench::bucketers::pair_1_unsigned_vector_dimension_bucketer;
use malachite_base::test_util::bench::{BenchmarkType, run_benchmark};
use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::generators::unsigned_vector_unsigned_pair_gen_var_4;
use malachite_base::test_util::runner::Runner;

pub(crate) fn register(runner: &mut Runner) {
    register_demo!(
        runner,
        demo_unsigned_vector_to_mod_power_of_2_element_product
    );
    register_demo!(
        runner,
        demo_unsigned_vector_into_mod_power_of_2_element_product
    );
    register_bench!(
        runner,
        benchmark_unsigned_vector_mod_power_of_2_element_product_evaluation_strategy
    );
}

fn demo_unsigned_vector_to_mod_power_of_2_element_product(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for (v, pow) in unsigned_vector_unsigned_pair_gen_var_4::<u64>()
        .get(gm, config)
        .take(limit)
    {
        println!(
            "{v}.to_mod_power_of_2_element_product({pow}) = {}",
            v.to_mod_power_of_2_element_product(pow)
        );
    }
}

fn demo_unsigned_vector_into_mod_power_of_2_element_product(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for (v, pow) in unsigned_vector_unsigned_pair_gen_var_4::<u64>()
        .get(gm, config)
        .take(limit)
    {
        let v_old = v.clone();
        println!(
            "{v_old}.into_mod_power_of_2_element_product({pow}) = {}",
            v.into_mod_power_of_2_element_product(pow)
        );
    }
}

fn benchmark_unsigned_vector_mod_power_of_2_element_product_evaluation_strategy(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "UnsignedVector<u64>.to_mod_power_of_2_element_product(u64)",
        BenchmarkType::EvaluationStrategy,
        unsigned_vector_unsigned_pair_gen_var_4::<u64>().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_1_unsigned_vector_dimension_bucketer("v"),
        &mut [
            (
                "UnsignedVector<u64>.to_mod_power_of_2_element_product(u64)",
                &mut |(v, pow)| {
                    no_out!(v.to_mod_power_of_2_element_product(pow));
                },
            ),
            (
                "UnsignedVector<u64>.into_mod_power_of_2_element_product(u64)",
                &mut |(v, pow)| {
                    no_out!(v.into_mod_power_of_2_element_product(pow));
                },
            ),
        ],
    );
}
