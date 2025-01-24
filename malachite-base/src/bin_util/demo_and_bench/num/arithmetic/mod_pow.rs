// Copyright © 2025 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::arithmetic::mod_pow::simple_binary_mod_pow;
use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base::test_util::bench::bucketers::triple_2_3_product_bit_bucketer;
use malachite_base::test_util::bench::{run_benchmark, BenchmarkType};
use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::generators::{
    unsigned_triple_gen_var_14, unsigned_triple_gen_var_15,
};
use malachite_base::test_util::num::arithmetic::mod_pow::naive_mod_pow;
use malachite_base::test_util::runner::Runner;

pub(crate) fn register(runner: &mut Runner) {
    register_unsigned_demos!(runner, demo_mod_pow);
    register_unsigned_demos!(runner, demo_mod_pow_assign);
    register_unsigned_benches!(runner, benchmark_mod_pow_algorithms);
    register_unsigned_benches!(runner, benchmark_mod_pow_naive_algorithms);
    register_unsigned_benches!(runner, benchmark_mod_pow_assign);
    register_unsigned_benches!(runner, benchmark_mod_pow_precomputed_algorithms);
}

fn demo_mod_pow<T: PrimitiveUnsigned>(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, exp, m) in unsigned_triple_gen_var_15::<T, u64>()
        .get(gm, config)
        .take(limit)
    {
        println!("{}.pow({}) ≡ {} mod {}", x, exp, x.mod_pow(exp, m), m);
    }
}

fn demo_mod_pow_assign<T: PrimitiveUnsigned>(gm: GenMode, config: &GenConfig, limit: usize) {
    for (mut x, exp, m) in unsigned_triple_gen_var_15::<T, u64>()
        .get(gm, config)
        .take(limit)
    {
        let old_x = x;
        x.mod_pow_assign(exp, m);
        println!("x := {old_x}; x.mod_pow_assign({exp}, {m}); x = {x}");
    }
}

fn benchmark_mod_pow_algorithms<T: PrimitiveUnsigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        &format!("{}.mod_pow(u64, {})", T::NAME, T::NAME),
        BenchmarkType::Algorithms,
        unsigned_triple_gen_var_15::<T, u64>().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &triple_2_3_product_bit_bucketer("exp", "m"),
        &mut [
            ("default", &mut |(x, exp, m)| no_out!(x.mod_pow(exp, m))),
            ("simple binary", &mut |(x, exp, m)| {
                no_out!(simple_binary_mod_pow(x, exp, m))
            }),
        ],
    );
}

fn benchmark_mod_pow_naive_algorithms<T: PrimitiveUnsigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        &format!("{}.mod_pow(u64, {})", T::NAME, T::NAME),
        BenchmarkType::Algorithms,
        unsigned_triple_gen_var_14::<T, u64>().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &triple_2_3_product_bit_bucketer("exp", "m"),
        &mut [
            ("default", &mut |(x, exp, m)| no_out!(x.mod_pow(exp, m))),
            ("naive", &mut |(x, exp, m)| {
                no_out!(naive_mod_pow(x, exp, m))
            }),
            ("simple binary", &mut |(x, exp, m)| {
                no_out!(simple_binary_mod_pow(x, exp, m))
            }),
        ],
    );
}

fn benchmark_mod_pow_assign<T: PrimitiveUnsigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        &format!("{}.mod_pow_assign(u64, {})", T::NAME, T::NAME),
        BenchmarkType::Single,
        unsigned_triple_gen_var_15::<T, u64>().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &triple_2_3_product_bit_bucketer("exp", "m"),
        &mut [("Malachite", &mut |(mut x, exp, m)| x.mod_pow_assign(exp, m))],
    );
}

fn benchmark_mod_pow_precomputed_algorithms<T: PrimitiveUnsigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        &format!("{}.mod_pow(u64, {})", T::NAME, T::NAME),
        BenchmarkType::Algorithms,
        unsigned_triple_gen_var_15::<T, u64>().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &triple_2_3_product_bit_bucketer("exp", "m"),
        &mut [
            ("default", &mut |(x, exp, m)| {
                for _ in 0..10 {
                    x.mod_pow(exp, m);
                }
            }),
            ("precomputed", &mut |(x, exp, m)| {
                let data = T::precompute_mod_pow_data(&m);
                for _ in 0..10 {
                    x.mod_pow_precomputed(exp, m, &data);
                }
            }),
        ],
    );
}
