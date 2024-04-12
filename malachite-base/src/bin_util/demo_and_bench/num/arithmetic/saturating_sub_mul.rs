// Copyright © 2024 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::basic::signeds::PrimitiveSigned;
use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base::test_util::bench::bucketers::triple_max_bit_bucketer;
use malachite_base::test_util::bench::{run_benchmark, BenchmarkType};
use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::generators::{signed_triple_gen, unsigned_triple_gen_var_19};
use malachite_base::test_util::runner::Runner;

pub(crate) fn register(runner: &mut Runner) {
    register_unsigned_demos!(runner, demo_saturating_sub_mul_unsigned);
    register_signed_demos!(runner, demo_saturating_sub_mul_signed);
    register_unsigned_demos!(runner, demo_saturating_sub_mul_assign_unsigned);
    register_signed_demos!(runner, demo_saturating_sub_mul_assign_signed);

    register_unsigned_benches!(runner, benchmark_saturating_sub_mul_unsigned);
    register_signed_benches!(runner, benchmark_saturating_sub_mul_signed);
    register_unsigned_benches!(runner, benchmark_saturating_sub_mul_assign_unsigned);
    register_signed_benches!(runner, benchmark_saturating_sub_mul_assign_signed);
}

fn demo_saturating_sub_mul_unsigned<T: PrimitiveUnsigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for (x, y, z) in unsigned_triple_gen_var_19::<T>()
        .get(gm, config)
        .take(limit)
    {
        println!(
            "{}.saturating_sub_mul({}, {}) = {}",
            x,
            y,
            z,
            x.saturating_sub_mul(y, z)
        );
    }
}

fn demo_saturating_sub_mul_signed<T: PrimitiveSigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for (x, y, z) in signed_triple_gen::<T>().get(gm, config).take(limit) {
        println!(
            "{}.saturating_sub_mul({}, {}) = {}",
            x,
            y,
            z,
            x.saturating_sub_mul(y, z)
        );
    }
}

fn demo_saturating_sub_mul_assign_unsigned<T: PrimitiveUnsigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for (mut x, y, z) in unsigned_triple_gen_var_19::<T>()
        .get(gm, config)
        .take(limit)
    {
        let old_x = x;
        x.saturating_sub_mul_assign(y, z);
        println!("x := {old_x}; x.saturating_sub_mul_assign({y}, {z}); x = {x}");
    }
}

fn demo_saturating_sub_mul_assign_signed<T: PrimitiveSigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for (mut x, y, z) in signed_triple_gen::<T>().get(gm, config).take(limit) {
        let old_x = x;
        x.saturating_sub_mul_assign(y, z);
        println!("x := {old_x}; x.saturating_sub_mul_assign({y}, {z}); x = {x}");
    }
}

fn benchmark_saturating_sub_mul_unsigned<T: PrimitiveUnsigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        &format!("{}.saturating_sub_mul({}, {})", T::NAME, T::NAME, T::NAME),
        BenchmarkType::Single,
        unsigned_triple_gen_var_19::<T>().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &triple_max_bit_bucketer("x", "y", "z"),
        &mut [("Malachite", &mut |(x, y, z)| {
            no_out!(x.saturating_sub_mul(y, z))
        })],
    );
}

fn benchmark_saturating_sub_mul_signed<T: PrimitiveSigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        &format!("{}.saturating_sub_mul({}, {})", T::NAME, T::NAME, T::NAME),
        BenchmarkType::Single,
        signed_triple_gen::<T>().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &triple_max_bit_bucketer("x", "y", "z"),
        &mut [("Malachite", &mut |(x, y, z)| {
            no_out!(x.saturating_sub_mul(y, z))
        })],
    );
}

fn benchmark_saturating_sub_mul_assign_unsigned<T: PrimitiveUnsigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        &format!(
            "{}.saturating_sub_mul_assign({}, {})",
            T::NAME,
            T::NAME,
            T::NAME
        ),
        BenchmarkType::Single,
        unsigned_triple_gen_var_19::<T>().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &triple_max_bit_bucketer("x", "y", "z"),
        &mut [("Malachite", &mut |(mut x, y, z)| {
            x.saturating_sub_mul_assign(y, z)
        })],
    );
}

fn benchmark_saturating_sub_mul_assign_signed<T: PrimitiveSigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        &format!(
            "{}.saturating_sub_mul_assign({}, {})",
            T::NAME,
            T::NAME,
            T::NAME
        ),
        BenchmarkType::Single,
        signed_triple_gen::<T>().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &triple_max_bit_bucketer("x", "y", "z"),
        &mut [("Malachite", &mut |(mut x, y, z)| {
            x.saturating_sub_mul_assign(y, z)
        })],
    );
}
