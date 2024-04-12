// Copyright © 2024 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::arithmetic::coprime_with::{
    coprime_with_check_2, coprime_with_check_2_3, coprime_with_check_2_3_5,
};
use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base::test_util::bench::bucketers::pair_max_bit_bucketer;
use malachite_base::test_util::bench::{run_benchmark, BenchmarkType};
use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::generators::unsigned_pair_gen_var_27;
use malachite_base::test_util::runner::Runner;

pub(crate) fn register(runner: &mut Runner) {
    register_unsigned_demos!(runner, demo_coprime_with);
    register_unsigned_benches!(runner, benchmark_coprime_with_algorithms);
}

fn demo_coprime_with<T: PrimitiveUnsigned>(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, y) in unsigned_pair_gen_var_27::<T>().get(gm, config).take(limit) {
        if x.coprime_with(y) {
            println!("{x} is coprime with {y}");
        } else {
            println!("{x} is not coprime with {y}");
        }
    }
}

#[allow(clippy::unnecessary_operation, unused_must_use)]
fn benchmark_coprime_with_algorithms<T: PrimitiveUnsigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        &format!("{}.coprime_with({})", T::NAME, T::NAME),
        BenchmarkType::Algorithms,
        unsigned_pair_gen_var_27::<T>().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_max_bit_bucketer("x", "y"),
        &mut [
            ("default", &mut |(x, y)| no_out!(x.coprime_with(y))),
            ("no divisibility check", &mut |(x, y)| {
                no_out!(x.gcd(y) == T::ONE)
            }),
            ("check divisibility by 2", &mut |(x, y)| {
                no_out!(coprime_with_check_2(x, y))
            }),
            ("check divisibility by 2 and 3", &mut |(x, y)| {
                no_out!(coprime_with_check_2_3(x, y))
            }),
            ("check divisibility by 2, 3, and 5", &mut |(x, y)| {
                no_out!(coprime_with_check_2_3_5(x, y))
            }),
        ],
    );
}
