// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::basic::signeds::PrimitiveSigned;
use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base::test_util::bench::bucketers::pair_1_bit_bucketer;
use malachite_base::test_util::bench::{BenchmarkType, run_benchmark};
use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::generators::{
    signed_unsigned_pair_gen_var_1, unsigned_pair_gen_var_2,
};
use malachite_base::test_util::runner::Runner;

pub(crate) fn register(runner: &mut Runner) {
    register_unsigned_demos!(runner, demo_balanced_mod_power_of_2_unsigned);
    register_signed_demos!(runner, demo_balanced_mod_power_of_2_signed);
    register_signed_demos!(runner, demo_balanced_mod_power_of_2_assign_signed);

    register_unsigned_benches!(
        runner,
        benchmark_balanced_mod_power_of_2_algorithms_unsigned
    );
    register_signed_benches!(runner, benchmark_balanced_mod_power_of_2_algorithms_signed);
}

// Whether the result fits in the signed type; the generators also produce inputs whose result
// does not.
fn fits_unsigned<T: PrimitiveUnsigned>(x: T, pow: u64) -> bool {
    let top = T::power_of_2(T::WIDTH - 1);
    !(pow == T::WIDTH && x == top || pow > T::WIDTH && x >= top)
}

fn demo_balanced_mod_power_of_2_unsigned<T: PrimitiveUnsigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for (x, pow) in unsigned_pair_gen_var_2::<T, u64>()
        .get(gm, config)
        .filter(|&(x, pow)| fits_unsigned(x, pow))
        .take(limit)
    {
        println!(
            "{}.balanced_mod_power_of_2({}) = {}",
            x,
            pow,
            x.balanced_mod_power_of_2(pow)
        );
    }
}

fn demo_balanced_mod_power_of_2_signed<T: PrimitiveSigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for (x, pow) in signed_unsigned_pair_gen_var_1::<T, u64>()
        .get(gm, config)
        .filter(|&(x, pow)| !(pow == T::WIDTH && x == T::MIN))
        .take(limit)
    {
        println!(
            "({}).balanced_mod_power_of_2({}) = {}",
            x,
            pow,
            x.balanced_mod_power_of_2(pow)
        );
    }
}

fn demo_balanced_mod_power_of_2_assign_signed<T: PrimitiveSigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for (mut x, pow) in signed_unsigned_pair_gen_var_1::<T, u64>()
        .get(gm, config)
        .filter(|&(x, pow)| !(pow == T::WIDTH && x == T::MIN))
        .take(limit)
    {
        let old_x = x;
        x.balanced_mod_power_of_2_assign(pow);
        println!("x := {old_x}; x.balanced_mod_power_of_2_assign({pow}); x = {x}");
    }
}

fn benchmark_balanced_mod_power_of_2_algorithms_unsigned<T: PrimitiveUnsigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        &format!("{}.balanced_mod_power_of_2(u64)", T::NAME),
        BenchmarkType::Algorithms,
        unsigned_pair_gen_var_2::<T, u64>()
            .get(gm, config)
            .filter(|&(_, pow)| pow < T::WIDTH),
        gm.name(),
        limit,
        file_name,
        &pair_1_bit_bucketer("x"),
        &mut [
            ("default", &mut |(x, pow)| {
                no_out!(x.balanced_mod_power_of_2(pow));
            }),
            ("using balanced_mod", &mut |(x, pow)| {
                no_out!(x.balanced_mod(T::power_of_2(pow)));
            }),
        ],
    );
}

fn benchmark_balanced_mod_power_of_2_algorithms_signed<T: PrimitiveSigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        &format!("{}.balanced_mod_power_of_2(u64)", T::NAME),
        BenchmarkType::Algorithms,
        signed_unsigned_pair_gen_var_1::<T, u64>()
            .get(gm, config)
            .filter(|&(_, pow)| pow + 1 < T::WIDTH),
        gm.name(),
        limit,
        file_name,
        &pair_1_bit_bucketer("x"),
        &mut [
            ("default", &mut |(x, pow)| {
                no_out!(x.balanced_mod_power_of_2(pow));
            }),
            ("using balanced_mod", &mut |(x, pow)| {
                no_out!(x.balanced_mod(T::power_of_2(pow)));
            }),
        ],
    );
}
