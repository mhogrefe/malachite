// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base::test_util::bench::bucketers::quintuple_5_bucketer;
use malachite_base::test_util::bench::{BenchmarkType, run_benchmark};
use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::generators::unsigned_quintuple_gen_var_1;
use malachite_base::test_util::runner::Runner;

pub(crate) fn register(runner: &mut Runner) {
    register_unsigned_demos!(runner, demo_mod_power_of_2_add_mul_shl);
    register_unsigned_demos!(runner, demo_mod_power_of_2_add_mul_shl_assign);
    register_unsigned_benches!(runner, benchmark_mod_power_of_2_add_mul_shl_algorithms);
    register_unsigned_benches!(runner, benchmark_mod_power_of_2_add_mul_shl_assign);
}

fn demo_mod_power_of_2_add_mul_shl<T: PrimitiveUnsigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for (x, y, z, bits, pow) in unsigned_quintuple_gen_var_1::<T>()
        .get(gm, config)
        .take(limit)
    {
        println!(
            "{} + {} * {} * 2^{} ≡ {} mod 2^{}",
            x,
            y,
            z,
            bits,
            x.mod_power_of_2_add_mul_shl(y, z, bits, pow),
            pow
        );
    }
}

fn demo_mod_power_of_2_add_mul_shl_assign<T: PrimitiveUnsigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for (mut x, y, z, bits, pow) in unsigned_quintuple_gen_var_1::<T>()
        .get(gm, config)
        .take(limit)
    {
        let old_x = x;
        x.mod_power_of_2_add_mul_shl_assign(y, z, bits, pow);
        println!(
            "x := {old_x}; x.mod_power_of_2_add_mul_shl_assign({y}, {z}, {bits}, {pow}); x = {x}"
        );
    }
}

fn benchmark_mod_power_of_2_add_mul_shl_algorithms<T: PrimitiveUnsigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        &format!(
            "{}.mod_power_of_2_add_mul_shl({}, {}, u64, u64)",
            T::NAME,
            T::NAME,
            T::NAME
        ),
        BenchmarkType::Algorithms,
        unsigned_quintuple_gen_var_1::<T>().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &quintuple_5_bucketer("pow"),
        &mut [
            ("default", &mut |(x, y, z, bits, pow)| {
                no_out!(x.mod_power_of_2_add_mul_shl(y, z, bits, pow));
            }),
            ("unfused", &mut |(x, y, z, bits, pow)| {
                no_out!(x.mod_power_of_2_add(
                    y.mod_power_of_2_mul(z, pow).mod_power_of_2_shl(bits, pow),
                    pow
                ));
            }),
        ],
    );
}

fn benchmark_mod_power_of_2_add_mul_shl_assign<T: PrimitiveUnsigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        &format!(
            "{}.mod_power_of_2_add_mul_shl_assign({}, {}, u64, u64)",
            T::NAME,
            T::NAME,
            T::NAME
        ),
        BenchmarkType::Single,
        unsigned_quintuple_gen_var_1::<T>().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &quintuple_5_bucketer("pow"),
        &mut [("Malachite", &mut |(mut x, y, z, bits, pow)| {
            x.mod_power_of_2_add_mul_shl_assign(y, z, bits, pow);
        })],
    );
}
