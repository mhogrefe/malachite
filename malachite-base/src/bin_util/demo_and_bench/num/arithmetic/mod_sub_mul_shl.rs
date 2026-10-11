// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::arithmetic::traits::ModShl;
use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base::test_util::bench::bucketers::quintuple_4_bucketer;
use malachite_base::test_util::bench::{BenchmarkType, run_benchmark};
use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::generators::unsigned_quintuple_gen_var_2;
use malachite_base::test_util::runner::Runner;

pub(crate) fn register(runner: &mut Runner) {
    register_unsigned_demos!(runner, demo_mod_sub_mul_shl);
    register_unsigned_demos!(runner, demo_mod_sub_mul_shl_assign);
    register_unsigned_benches!(runner, benchmark_mod_sub_mul_shl_algorithms);
    register_unsigned_benches!(runner, benchmark_mod_sub_mul_shl_assign);
}

fn demo_mod_sub_mul_shl<T: PrimitiveUnsigned>(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, y, z, bits, m) in unsigned_quintuple_gen_var_2::<T>()
        .get(gm, config)
        .take(limit)
    {
        println!(
            "{} - {} * {} * 2^{} ≡ {} mod {}",
            x,
            y,
            z,
            bits,
            x.mod_sub_mul_shl(y, z, bits, m),
            m
        );
    }
}

fn demo_mod_sub_mul_shl_assign<T: PrimitiveUnsigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for (mut x, y, z, bits, m) in unsigned_quintuple_gen_var_2::<T>()
        .get(gm, config)
        .take(limit)
    {
        let old_x = x;
        x.mod_sub_mul_shl_assign(y, z, bits, m);
        println!("x := {old_x}; x.mod_sub_mul_shl_assign({y}, {z}, {bits}, {m}); x = {x}");
    }
}

fn benchmark_mod_sub_mul_shl_algorithms<T: PrimitiveUnsigned + ModShl<u64, T, Output = T>>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        &format!(
            "{}.mod_sub_mul_shl({}, {}, u64, {})",
            T::NAME,
            T::NAME,
            T::NAME,
            T::NAME
        ),
        BenchmarkType::Algorithms,
        unsigned_quintuple_gen_var_2::<T>().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &quintuple_4_bucketer("bits"),
        &mut [
            ("default", &mut |(x, y, z, bits, m)| {
                no_out!(x.mod_sub_mul_shl(y, z, bits, m));
            }),
            ("unfused", &mut |(x, y, z, bits, m)| {
                no_out!(x.mod_sub(y.mod_mul(z, m).mod_shl(bits, m), m));
            }),
        ],
    );
}

fn benchmark_mod_sub_mul_shl_assign<T: PrimitiveUnsigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        &format!(
            "{}.mod_sub_mul_shl_assign({}, {}, u64, {})",
            T::NAME,
            T::NAME,
            T::NAME,
            T::NAME
        ),
        BenchmarkType::Single,
        unsigned_quintuple_gen_var_2::<T>().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &quintuple_4_bucketer("bits"),
        &mut [("Malachite", &mut |(mut x, y, z, bits, m)| {
            x.mod_sub_mul_shl_assign(y, z, bits, m);
        })],
    );
}
