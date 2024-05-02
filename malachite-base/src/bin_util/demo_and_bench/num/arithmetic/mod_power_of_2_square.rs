// Copyright © 2024 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base::test_util::bench::bucketers::pair_2_bucketer;
use malachite_base::test_util::bench::{run_benchmark, BenchmarkType};
use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::generators::unsigned_pair_gen_var_17;
use malachite_base::test_util::runner::Runner;

pub(crate) fn register(runner: &mut Runner) {
    register_unsigned_demos!(runner, demo_mod_power_of_2_square);
    register_unsigned_demos!(runner, demo_mod_power_of_2_square_assign);
    register_unsigned_benches!(runner, benchmark_mod_power_of_2_square);
    register_unsigned_benches!(runner, benchmark_mod_power_of_2_square_assign);
}

fn demo_mod_power_of_2_square<T: PrimitiveUnsigned>(gm: GenMode, config: &GenConfig, limit: usize) {
    for (n, pow) in unsigned_pair_gen_var_17::<T>().get(gm, config).take(limit) {
        println!(
            "{}.square() ≡ {} mod 2^{}",
            n,
            n.mod_power_of_2_square(pow),
            pow
        );
    }
}

fn demo_mod_power_of_2_square_assign<T: PrimitiveUnsigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for (mut n, pow) in unsigned_pair_gen_var_17::<T>().get(gm, config).take(limit) {
        let old_n = n;
        n.mod_power_of_2_square_assign(pow);
        println!("n := {old_n}; n.mod_power_of_2_square_assign({pow}); n = {n}");
    }
}

fn benchmark_mod_power_of_2_square<T: PrimitiveUnsigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        &format!("{}.mod_power_of_2_square(u64)", T::NAME),
        BenchmarkType::Single,
        unsigned_pair_gen_var_17::<T>().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_2_bucketer("pow"),
        &mut [("Malachite", &mut |(n, pow)| {
            no_out!(n.mod_power_of_2_square(pow))
        })],
    );
}

fn benchmark_mod_power_of_2_square_assign<T: PrimitiveUnsigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        &format!("{}.mod_power_of_2_square_assign(u64)", T::NAME),
        BenchmarkType::Single,
        unsigned_pair_gen_var_17::<T>().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_2_bucketer("pow"),
        &mut [("Malachite", &mut |(mut n, pow)| {
            n.mod_power_of_2_square_assign(pow)
        })],
    );
}
