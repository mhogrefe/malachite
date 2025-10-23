// Copyright © 2025 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::arithmetic::traits::{ModPowerOf2, ModPowerOf2Shr, ModPowerOf2ShrAssign};
use malachite_base::num::basic::signeds::PrimitiveSigned;
use malachite_base::test_util::bench::bucketers::triple_3_bucketer;
use malachite_base::test_util::bench::{BenchmarkType, run_benchmark};
use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::runner::Runner;
use malachite_nz::natural::Natural;
use malachite_nz::test_util::generators::natural_signed_unsigned_triple_gen_var_1;
use std::ops::Shr;

pub(crate) fn register(runner: &mut Runner) {
    register_signed_demos!(runner, demo_natural_mod_power_of_2_shr_assign);
    register_signed_demos!(runner, demo_natural_mod_power_of_2_shr);
    register_signed_demos!(runner, demo_natural_mod_power_of_2_shr_ref);

    register_signed_benches!(runner, benchmark_natural_mod_power_of_2_shr_assign);
    register_signed_benches!(
        runner,
        benchmark_natural_mod_power_of_2_shr_evaluation_strategy
    );
    register_signed_benches!(runner, benchmark_natural_mod_power_of_2_shr_algorithms);
}

fn demo_natural_mod_power_of_2_shr_assign<T: PrimitiveSigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) where
    Natural: ModPowerOf2ShrAssign<T>,
{
    for (mut n, i, pow) in natural_signed_unsigned_triple_gen_var_1::<T>()
        .get(gm, config)
        .take(limit)
    {
        let n_old = n.clone();
        n.mod_power_of_2_shr_assign(i, pow);
        println!("x := {n_old}; x.mod_power_of_2_shr_assign({i}, {pow}); x = {n}");
    }
}

fn demo_natural_mod_power_of_2_shr<T: PrimitiveSigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) where
    Natural: ModPowerOf2Shr<T, Output = Natural>,
{
    for (n, i, pow) in natural_signed_unsigned_triple_gen_var_1::<T>()
        .get(gm, config)
        .take(limit)
    {
        let n_old = n.clone();
        println!(
            "{}.mod_power_of_2_shr({}, {}) = {}",
            n_old,
            i,
            pow,
            n.mod_power_of_2_shr(i, pow)
        );
    }
}

fn demo_natural_mod_power_of_2_shr_ref<T: PrimitiveSigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) where
    for<'a> &'a Natural: ModPowerOf2Shr<T, Output = Natural>,
{
    for (n, i, pow) in natural_signed_unsigned_triple_gen_var_1::<T>()
        .get(gm, config)
        .take(limit)
    {
        let n_old = n.clone();
        println!(
            "(&{}).mod_power_of_2_shr({}, {}) = {}",
            n_old,
            i,
            pow,
            (&n).mod_power_of_2_shr(i, pow)
        );
    }
}

fn benchmark_natural_mod_power_of_2_shr_assign<T: PrimitiveSigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) where
    Natural: ModPowerOf2ShrAssign<T>,
{
    run_benchmark(
        &format!("Natural.mod_power_of_2_shr_assign({}, u64)", T::NAME),
        BenchmarkType::Single,
        natural_signed_unsigned_triple_gen_var_1::<T>().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &triple_3_bucketer("pow"),
        &mut [("Malachite", &mut |(mut x, y, pow)| {
            x.mod_power_of_2_shr_assign(y, pow);
        })],
    );
}

fn benchmark_natural_mod_power_of_2_shr_evaluation_strategy<T: PrimitiveSigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) where
    Natural: ModPowerOf2Shr<T>,
    for<'a> &'a Natural: ModPowerOf2Shr<T>,
{
    run_benchmark(
        &format!("Natural.mod_power_of_2_shr_assign({}, u64)", T::NAME),
        BenchmarkType::EvaluationStrategy,
        natural_signed_unsigned_triple_gen_var_1::<T>().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &triple_3_bucketer("pow"),
        &mut [
            (
                &format!("Natural.mod_power_of_2_shr({}, u64)", T::NAME),
                &mut |(x, y, pow)| no_out!(x.mod_power_of_2_shr(y, pow)),
            ),
            (
                &format!("(&Natural).mod_power_of_2_shr({}, u64)", T::NAME),
                &mut |(x, y, pow)| no_out!((&x).mod_power_of_2_shr(y, pow)),
            ),
        ],
    );
}

fn benchmark_natural_mod_power_of_2_shr_algorithms<T: PrimitiveSigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) where
    Natural: ModPowerOf2Shr<T> + Shr<T, Output = Natural>,
{
    run_benchmark(
        &format!("Natural.mod_power_of_2_shr_assign({}, u64)", T::NAME),
        BenchmarkType::Algorithms,
        natural_signed_unsigned_triple_gen_var_1::<T>().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &triple_3_bucketer("pow"),
        &mut [
            ("default", &mut |(x, y, pow)| {
                no_out!(x.mod_power_of_2_shr(y, pow));
            }),
            (
                &format!("(Natural >> {}).mod_power_of_2(u64)", T::NAME),
                &mut |(x, y, pow)| no_out!((x >> y).mod_power_of_2(pow)),
            ),
        ],
    );
}
