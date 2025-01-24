// Copyright © 2025 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base::test_util::bench::bucketers::pair_max_bit_bucketer;
use malachite_base::test_util::bench::{run_benchmark, BenchmarkType};
use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::generators::{unsigned_pair_gen_var_33, unsigned_pair_gen_var_34};
use malachite_base::test_util::runner::Runner;

pub(crate) fn register(runner: &mut Runner) {
    register_unsigned_demos!(runner, demo_lcm);
    register_unsigned_demos!(runner, demo_lcm_assign);
    register_unsigned_demos!(runner, demo_checked_lcm);

    register_unsigned_benches!(runner, benchmark_lcm);
    register_unsigned_benches!(runner, benchmark_lcm_assign);
    register_unsigned_benches!(runner, benchmark_checked_lcm);
}

fn demo_lcm<T: PrimitiveUnsigned>(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, y) in unsigned_pair_gen_var_34::<T>().get(gm, config).take(limit) {
        println!("{}.lcm({}) = {}", x, y, x.lcm(y));
    }
}

fn demo_lcm_assign<T: PrimitiveUnsigned>(gm: GenMode, config: &GenConfig, limit: usize) {
    for (mut x, y) in unsigned_pair_gen_var_34::<T>().get(gm, config).take(limit) {
        let old_x = x;
        x.lcm_assign(y);
        println!("x := {old_x}; x.lcm_assign({y}); x = {x}");
    }
}

fn demo_checked_lcm<T: PrimitiveUnsigned>(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, y) in unsigned_pair_gen_var_33::<T>().get(gm, config).take(limit) {
        println!("{}.checked_lcm({}) = {:?}", x, y, x.checked_lcm(y));
    }
}

fn benchmark_lcm<T: PrimitiveUnsigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        &format!("{}.lcm({})", T::NAME, T::NAME),
        BenchmarkType::Single,
        unsigned_pair_gen_var_34::<T>().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_max_bit_bucketer("x", "y"),
        &mut [("Malachite", &mut |(x, y)| no_out!(x.lcm(y)))],
    );
}

fn benchmark_lcm_assign<T: PrimitiveUnsigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        &format!("{}.lcm_assign({})", T::NAME, T::NAME),
        BenchmarkType::Single,
        unsigned_pair_gen_var_34::<T>().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_max_bit_bucketer("x", "y"),
        &mut [("Malachite", &mut |(mut x, y)| x.lcm_assign(y))],
    );
}

fn benchmark_checked_lcm<T: PrimitiveUnsigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        &format!("{}.checked_lcm({})", T::NAME, T::NAME),
        BenchmarkType::Single,
        unsigned_pair_gen_var_33::<T>().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_max_bit_bucketer("x", "y"),
        &mut [("Malachite", &mut |(x, y)| no_out!(x.checked_lcm(y)))],
    );
}
