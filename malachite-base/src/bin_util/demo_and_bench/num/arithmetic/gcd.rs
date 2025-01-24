// Copyright © 2025 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::arithmetic::gcd::{gcd_binary, gcd_euclidean, gcd_fast_a, gcd_fast_b};
use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base::test_util::bench::bucketers::pair_max_bit_bucketer;
use malachite_base::test_util::bench::{run_benchmark, BenchmarkType};
use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::generators::unsigned_pair_gen_var_27;
use malachite_base::test_util::runner::Runner;

pub(crate) fn register(runner: &mut Runner) {
    register_unsigned_demos!(runner, demo_gcd);
    register_unsigned_demos!(runner, demo_gcd_assign);

    register_unsigned_benches!(runner, benchmark_gcd_algorithms);
    register_unsigned_benches!(runner, benchmark_gcd_assign);
}

fn demo_gcd<T: PrimitiveUnsigned>(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, y) in unsigned_pair_gen_var_27::<T>().get(gm, config).take(limit) {
        println!("{}.gcd({}) = {}", x, y, x.gcd(y));
    }
}

fn demo_gcd_assign<T: PrimitiveUnsigned>(gm: GenMode, config: &GenConfig, limit: usize) {
    for (mut x, y) in unsigned_pair_gen_var_27::<T>().get(gm, config).take(limit) {
        let old_x = x;
        x.gcd_assign(y);
        println!("x := {old_x}; x.gcd_assign({y}); x = {x}");
    }
}

fn benchmark_gcd_algorithms<T: PrimitiveUnsigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        &format!("{}.gcd({})", T::NAME, T::NAME),
        BenchmarkType::Algorithms,
        unsigned_pair_gen_var_27::<T>().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_max_bit_bucketer("x", "y"),
        &mut [
            ("default", &mut |(x, y)| no_out!(x.gcd(y))),
            ("Euclidean", &mut |(x, y)| no_out!(gcd_euclidean(x, y))),
            ("binary", &mut |(x, y)| no_out!(gcd_binary(x, y))),
            ("fast A", &mut |(x, y)| no_out!(gcd_fast_a(x, y))),
            ("fast B", &mut |(x, y)| no_out!(gcd_fast_b(x, y))),
        ],
    );
}

fn benchmark_gcd_assign<T: PrimitiveUnsigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        &format!("{}.gcd_assign({})", T::NAME, T::NAME),
        BenchmarkType::Single,
        unsigned_pair_gen_var_27::<T>().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_max_bit_bucketer("x", "y"),
        &mut [("Malachite", &mut |(mut x, y)| x.gcd_assign(y))],
    );
}
