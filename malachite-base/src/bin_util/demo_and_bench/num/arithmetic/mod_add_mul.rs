// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base::test_util::bench::bucketers::quadruple_max_bit_bucketer;
use malachite_base::test_util::bench::{BenchmarkType, run_benchmark};
use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::generators::unsigned_quadruple_gen_var_4;
use malachite_base::test_util::runner::Runner;

pub(crate) fn register(runner: &mut Runner) {
    register_unsigned_demos!(runner, demo_mod_add_mul);
    register_unsigned_demos!(runner, demo_mod_add_mul_assign);
    register_unsigned_benches!(runner, benchmark_mod_add_mul_algorithms);
    register_unsigned_benches!(runner, benchmark_mod_add_mul_assign);
}

fn demo_mod_add_mul<T: PrimitiveUnsigned>(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, y, z, m) in unsigned_quadruple_gen_var_4::<T>()
        .get(gm, config)
        .take(limit)
    {
        println!(
            "{} + {} * {} ≡ {} mod {}",
            x,
            y,
            z,
            x.mod_add_mul(y, z, m),
            m
        );
    }
}

fn demo_mod_add_mul_assign<T: PrimitiveUnsigned>(gm: GenMode, config: &GenConfig, limit: usize) {
    for (mut x, y, z, m) in unsigned_quadruple_gen_var_4::<T>()
        .get(gm, config)
        .take(limit)
    {
        let old_x = x;
        x.mod_add_mul_assign(y, z, m);
        println!("x := {old_x}; x.mod_add_mul_assign({y}, {z}, {m}); x = {x}");
    }
}

fn benchmark_mod_add_mul_algorithms<T: PrimitiveUnsigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        &format!(
            "{}.mod_add_mul({}, {}, {})",
            T::NAME,
            T::NAME,
            T::NAME,
            T::NAME
        ),
        BenchmarkType::Algorithms,
        unsigned_quadruple_gen_var_4::<T>().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &quadruple_max_bit_bucketer("x", "y", "z", "m"),
        &mut [
            ("default", &mut |(x, y, z, m)| {
                no_out!(x.mod_add_mul(y, z, m));
            }),
            ("unfused", &mut |(x, y, z, m)| {
                no_out!(x.mod_add(y.mod_mul(z, m), m));
            }),
        ],
    );
}

fn benchmark_mod_add_mul_assign<T: PrimitiveUnsigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        &format!(
            "{}.mod_add_mul_assign({}, {}, {})",
            T::NAME,
            T::NAME,
            T::NAME,
            T::NAME
        ),
        BenchmarkType::Single,
        unsigned_quadruple_gen_var_4::<T>().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &quadruple_max_bit_bucketer("x", "y", "z", "m"),
        &mut [("Malachite", &mut |(mut x, y, z, m)| {
            x.mod_add_mul_assign(y, z, m);
        })],
    );
}
