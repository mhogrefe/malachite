// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::arithmetic::traits::{
    BalancedMod, BalancedModPowerOf2, BalancedModPowerOf2Assign, PowerOf2,
};
use malachite_base::test_util::bench::{BenchmarkType, run_benchmark};
use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::runner::Runner;
use malachite_nz::integer::Integer;
use malachite_nz::test_util::bench::bucketers::pair_1_integer_polynomial_bit_bucketer;
use malachite_nz::test_util::generators::integer_polynomial_unsigned_pair_gen_var_1;

pub(crate) fn register(runner: &mut Runner) {
    register_demo!(runner, demo_integer_polynomial_balanced_mod_power_of_2);
    register_demo!(runner, demo_integer_polynomial_balanced_mod_power_of_2_ref);
    register_demo!(
        runner,
        demo_integer_polynomial_balanced_mod_power_of_2_assign
    );

    register_bench!(
        runner,
        benchmark_integer_polynomial_balanced_mod_power_of_2_algorithms
    );
}

fn demo_integer_polynomial_balanced_mod_power_of_2(gm: GenMode, config: &GenConfig, limit: usize) {
    for (p, pow) in integer_polynomial_unsigned_pair_gen_var_1()
        .get(gm, config)
        .take(limit)
    {
        let p_old = p.clone();
        println!(
            "({p_old}).balanced_mod_power_of_2({pow}) = {}",
            p.balanced_mod_power_of_2(pow)
        );
    }
}

fn demo_integer_polynomial_balanced_mod_power_of_2_ref(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for (p, pow) in integer_polynomial_unsigned_pair_gen_var_1()
        .get(gm, config)
        .take(limit)
    {
        println!(
            "(&({p})).balanced_mod_power_of_2({pow}) = {}",
            (&p).balanced_mod_power_of_2(pow)
        );
    }
}

fn demo_integer_polynomial_balanced_mod_power_of_2_assign(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for (mut p, pow) in integer_polynomial_unsigned_pair_gen_var_1()
        .get(gm, config)
        .take(limit)
    {
        let p_old = p.clone();
        p.balanced_mod_power_of_2_assign(pow);
        println!("p := {p_old}; p.balanced_mod_power_of_2_assign({pow}); p = {p}");
    }
}

fn benchmark_integer_polynomial_balanced_mod_power_of_2_algorithms(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "IntegerPolynomial.balanced_mod_power_of_2(u64)",
        BenchmarkType::Algorithms,
        integer_polynomial_unsigned_pair_gen_var_1().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_1_integer_polynomial_bit_bucketer("p"),
        &mut [
            ("default", &mut |(p, pow)| {
                no_out!(p.balanced_mod_power_of_2(pow));
            }),
            ("using balanced_mod", &mut |(p, pow)| {
                no_out!(p.balanced_mod(Integer::power_of_2(pow)));
            }),
        ],
    );
}
