// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::arithmetic::traits::{
    ModPowerOf2Dot, ModPowerOf2Sub, ModPowerOf2SubDot, ModPowerOf2SubDotAssign,
};
use malachite_base::test_util::bench::bucketers::quadruple_1_unsigned_vector_dimension_bucketer;
use malachite_base::test_util::bench::{BenchmarkType, run_benchmark};
use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::generators::*;
use malachite_base::test_util::runner::Runner;

pub(crate) fn register(runner: &mut Runner) {
    register_demo!(runner, demo_unsigned_vector_mod_power_of_2_sub_dot);
    register_demo!(runner, demo_unsigned_vector_mod_power_of_2_sub_dot_ref);
    register_demo!(runner, demo_unsigned_vector_mod_power_of_2_sub_dot_assign);
    register_bench!(
        runner,
        benchmark_unsigned_vector_mod_power_of_2_sub_dot_algorithms
    );
    register_bench!(
        runner,
        benchmark_unsigned_vector_mod_power_of_2_sub_dot_evaluation_strategy
    );
}

fn demo_unsigned_vector_mod_power_of_2_sub_dot(gm: GenMode, config: &GenConfig, limit: usize) {
    for (v, w, x, pow) in
        unsigned_vector_unsigned_vector_unsigned_unsigned_quadruple_gen_var_1::<u64>()
            .get(gm, config)
            .take(limit)
    {
        let v_old = v.clone();
        let w_old = w.clone();
        println!(
            "{x}.mod_power_of_2_sub_dot({v_old}, {w_old}, {pow}) = {}",
            x.mod_power_of_2_sub_dot(v, w, pow)
        );
    }
}

fn demo_unsigned_vector_mod_power_of_2_sub_dot_ref(gm: GenMode, config: &GenConfig, limit: usize) {
    for (v, w, x, pow) in
        unsigned_vector_unsigned_vector_unsigned_unsigned_quadruple_gen_var_1::<u64>()
            .get(gm, config)
            .take(limit)
    {
        println!(
            "{x}.mod_power_of_2_sub_dot(&{v}, &{w}, {pow}) = {}",
            x.mod_power_of_2_sub_dot(&v, &w, pow)
        );
    }
}

fn demo_unsigned_vector_mod_power_of_2_sub_dot_assign(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for (v, w, mut x, pow) in
        unsigned_vector_unsigned_vector_unsigned_unsigned_quadruple_gen_var_1::<u64>()
            .get(gm, config)
            .take(limit)
    {
        let x_old = x;
        x.mod_power_of_2_sub_dot_assign(&v, &w, pow);
        println!("x := {x_old}; x.mod_power_of_2_sub_dot_assign(&{v}, &{w}, {pow}); x = {x}");
    }
}

fn benchmark_unsigned_vector_mod_power_of_2_sub_dot_algorithms(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "u64.mod_power_of_2_sub_dot(UnsignedVector<u64>, UnsignedVector<u64>)",
        BenchmarkType::Algorithms,
        unsigned_vector_unsigned_vector_unsigned_unsigned_quadruple_gen_var_1::<u64>()
            .get(gm, config),
        gm.name(),
        limit,
        file_name,
        &quadruple_1_unsigned_vector_dimension_bucketer("v"),
        &mut [
            ("default", &mut |(v, w, x, pow)| {
                no_out!(x.mod_power_of_2_sub_dot(&v, &w, pow));
            }),
            (
                "mod_power_of_2_dot, then mod_power_of_2_sub",
                &mut |(v, w, x, pow)| {
                    no_out!(x.mod_power_of_2_sub((&v).mod_power_of_2_dot(&w, pow), pow));
                },
            ),
        ],
    );
}

fn benchmark_unsigned_vector_mod_power_of_2_sub_dot_evaluation_strategy(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "u64.mod_power_of_2_sub_dot(UnsignedVector<u64>, UnsignedVector<u64>)",
        BenchmarkType::EvaluationStrategy,
        unsigned_vector_unsigned_vector_unsigned_unsigned_quadruple_gen_var_1::<u64>()
            .get(gm, config),
        gm.name(),
        limit,
        file_name,
        &quadruple_1_unsigned_vector_dimension_bucketer("v"),
        &mut [
            (
                "u64.mod_power_of_2_sub_dot(UnsignedVector<u64>, UnsignedVector<u64>)",
                &mut |(v, w, x, pow)| {
                    no_out!(x.mod_power_of_2_sub_dot(v, w, pow));
                },
            ),
            (
                "u64.mod_power_of_2_sub_dot(&UnsignedVector<u64>, &UnsignedVector<u64>)",
                &mut |(v, w, x, pow)| {
                    no_out!(x.mod_power_of_2_sub_dot(&v, &w, pow));
                },
            ),
            (
                "u64.mod_power_of_2_sub_dot_assign(&UnsignedVector<u64>, &UnsignedVector<u64>)",
                &mut |(v, w, mut x, pow)| {
                    no_out!(x.mod_power_of_2_sub_dot_assign(&v, &w, pow));
                },
            ),
        ],
    );
}
