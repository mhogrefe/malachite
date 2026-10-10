// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base::test_util::bench::bucketers::quadruple_4_bucketer;
use malachite_base::test_util::bench::{BenchmarkType, run_benchmark};
use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::generators::unsigned_quadruple_gen_var_3;
use malachite_base::test_util::runner::Runner;

pub(crate) fn register(runner: &mut Runner) {
    register_unsigned_demos!(runner, demo_mod_power_of_2_sub_mul);
    register_unsigned_demos!(runner, demo_mod_power_of_2_sub_mul_assign);
    register_unsigned_benches!(runner, benchmark_mod_power_of_2_sub_mul_algorithms);
    register_unsigned_benches!(runner, benchmark_mod_power_of_2_sub_mul_assign);
}

fn demo_mod_power_of_2_sub_mul<T: PrimitiveUnsigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for (x, y, z, pow) in unsigned_quadruple_gen_var_3::<T>()
        .get(gm, config)
        .take(limit)
    {
        println!(
            "{} - {} * {} ≡ {} mod 2^{}",
            x,
            y,
            z,
            x.mod_power_of_2_sub_mul(y, z, pow),
            pow
        );
    }
}

fn demo_mod_power_of_2_sub_mul_assign<T: PrimitiveUnsigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for (mut x, y, z, pow) in unsigned_quadruple_gen_var_3::<T>()
        .get(gm, config)
        .take(limit)
    {
        let old_x = x;
        x.mod_power_of_2_sub_mul_assign(y, z, pow);
        println!("x := {old_x}; x.mod_power_of_2_sub_mul_assign({y}, {z}, {pow}); x = {x}");
    }
}

fn benchmark_mod_power_of_2_sub_mul_algorithms<T: PrimitiveUnsigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        &format!(
            "{}.mod_power_of_2_sub_mul({}, {}, u64)",
            T::NAME,
            T::NAME,
            T::NAME
        ),
        BenchmarkType::Algorithms,
        unsigned_quadruple_gen_var_3::<T>().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &quadruple_4_bucketer("pow"),
        &mut [
            ("default", &mut |(x, y, z, pow)| {
                no_out!(x.mod_power_of_2_sub_mul(y, z, pow));
            }),
            ("unfused", &mut |(x, y, z, pow)| {
                no_out!(x.mod_power_of_2_sub(y.mod_power_of_2_mul(z, pow), pow));
            }),
        ],
    );
}

fn benchmark_mod_power_of_2_sub_mul_assign<T: PrimitiveUnsigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        &format!(
            "{}.mod_power_of_2_sub_mul_assign({}, {}, u64)",
            T::NAME,
            T::NAME,
            T::NAME
        ),
        BenchmarkType::Single,
        unsigned_quadruple_gen_var_3::<T>().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &quadruple_4_bucketer("pow"),
        &mut [("Malachite", &mut |(mut x, y, z, pow)| {
            x.mod_power_of_2_sub_mul_assign(y, z, pow);
        })],
    );
}
