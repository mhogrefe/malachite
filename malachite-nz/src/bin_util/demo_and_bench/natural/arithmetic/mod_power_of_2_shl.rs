// Copyright © 2024 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::arithmetic::traits::{ModPowerOf2, ModPowerOf2Shl, ModPowerOf2ShlAssign};
use malachite_base::num::basic::signeds::PrimitiveSigned;
use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base::test_util::bench::bucketers::triple_3_bucketer;
use malachite_base::test_util::bench::{run_benchmark, BenchmarkType};
use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::runner::Runner;
use malachite_nz::natural::Natural;
use malachite_nz::test_util::generators::{
    natural_signed_unsigned_triple_gen_var_1, natural_unsigned_unsigned_triple_gen_var_6,
};
use std::ops::Shl;

pub(crate) fn register(runner: &mut Runner) {
    register_unsigned_demos!(runner, demo_natural_mod_power_of_2_shl_assign_unsigned);
    register_unsigned_demos!(runner, demo_natural_mod_power_of_2_shl_unsigned);
    register_unsigned_demos!(runner, demo_natural_mod_power_of_2_shl_unsigned_ref);
    register_signed_demos!(runner, demo_natural_mod_power_of_2_shl_assign_signed);
    register_signed_demos!(runner, demo_natural_mod_power_of_2_shl_signed);
    register_signed_demos!(runner, demo_natural_mod_power_of_2_shl_signed_ref);

    register_unsigned_benches!(runner, benchmark_natural_mod_power_of_2_shl_assign_unsigned);
    register_unsigned_benches!(
        runner,
        benchmark_natural_mod_power_of_2_shl_unsigned_evaluation_strategy
    );
    register_unsigned_benches!(
        runner,
        benchmark_natural_mod_power_of_2_shl_unsigned_algorithms
    );
    register_signed_benches!(runner, benchmark_natural_mod_power_of_2_shl_assign_signed);
    register_signed_benches!(
        runner,
        benchmark_natural_mod_power_of_2_shl_signed_evaluation_strategy
    );
    register_signed_benches!(
        runner,
        benchmark_natural_mod_power_of_2_shl_signed_algorithms
    );
}

fn demo_natural_mod_power_of_2_shl_assign_unsigned<T: PrimitiveUnsigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) where
    Natural: ModPowerOf2ShlAssign<T>,
{
    for (mut n, u, pow) in natural_unsigned_unsigned_triple_gen_var_6::<T>()
        .get(gm, config)
        .take(limit)
    {
        let n_old = n.clone();
        n.mod_power_of_2_shl_assign(u, pow);
        println!("x := {n_old}; x.mod_power_of_2_shl_assign({u}, {pow}); x = {n}");
    }
}

fn demo_natural_mod_power_of_2_shl_unsigned<T: PrimitiveUnsigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) where
    Natural: ModPowerOf2Shl<T, Output = Natural>,
{
    for (n, u, pow) in natural_unsigned_unsigned_triple_gen_var_6::<T>()
        .get(gm, config)
        .take(limit)
    {
        let n_old = n.clone();
        println!(
            "{}.mod_power_of_2_shl({}, {}) = {}",
            n_old,
            u,
            pow,
            n.mod_power_of_2_shl(u, pow)
        );
    }
}

fn demo_natural_mod_power_of_2_shl_unsigned_ref<T: PrimitiveUnsigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) where
    for<'a> &'a Natural: ModPowerOf2Shl<T, Output = Natural>,
{
    for (n, u, pow) in natural_unsigned_unsigned_triple_gen_var_6::<T>()
        .get(gm, config)
        .take(limit)
    {
        let n_old = n.clone();
        println!(
            "(&{}).mod_power_of_2_shl({}, {}) = {}",
            n_old,
            u,
            pow,
            (&n).mod_power_of_2_shl(u, pow)
        );
    }
}

fn demo_natural_mod_power_of_2_shl_assign_signed<T: PrimitiveSigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) where
    Natural: ModPowerOf2ShlAssign<T>,
{
    for (mut n, i, pow) in natural_signed_unsigned_triple_gen_var_1::<T>()
        .get(gm, config)
        .take(limit)
    {
        let n_old = n.clone();
        n.mod_power_of_2_shl_assign(i, pow);
        println!("x := {n_old}; x.mod_power_of_2_shl_assign({i}, {pow}); x = {n}");
    }
}

fn demo_natural_mod_power_of_2_shl_signed<T: PrimitiveSigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) where
    Natural: ModPowerOf2Shl<T, Output = Natural>,
{
    for (n, i, pow) in natural_signed_unsigned_triple_gen_var_1::<T>()
        .get(gm, config)
        .take(limit)
    {
        let n_old = n.clone();
        println!(
            "{}.mod_power_of_2_shl({}, {}) = {}",
            n_old,
            i,
            pow,
            n.mod_power_of_2_shl(i, pow)
        );
    }
}

fn demo_natural_mod_power_of_2_shl_signed_ref<T: PrimitiveSigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) where
    for<'a> &'a Natural: ModPowerOf2Shl<T, Output = Natural>,
{
    for (n, i, pow) in natural_signed_unsigned_triple_gen_var_1::<T>()
        .get(gm, config)
        .take(limit)
    {
        let n_old = n.clone();
        println!(
            "(&{}).mod_power_of_2_shl({}, {}) = {}",
            n_old,
            i,
            pow,
            (&n).mod_power_of_2_shl(i, pow)
        );
    }
}

fn benchmark_natural_mod_power_of_2_shl_assign_unsigned<T: PrimitiveUnsigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) where
    Natural: ModPowerOf2ShlAssign<T>,
{
    run_benchmark(
        &format!("Natural.mod_power_of_2_shl_assign({}, u64)", T::NAME),
        BenchmarkType::Single,
        natural_unsigned_unsigned_triple_gen_var_6::<T>().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &triple_3_bucketer("pow"),
        &mut [("Malachite", &mut |(mut x, y, pow)| {
            x.mod_power_of_2_shl_assign(y, pow)
        })],
    );
}

fn benchmark_natural_mod_power_of_2_shl_unsigned_evaluation_strategy<T: PrimitiveUnsigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) where
    Natural: ModPowerOf2Shl<T>,
    for<'a> &'a Natural: ModPowerOf2Shl<T>,
{
    run_benchmark(
        &format!("Natural.mod_power_of_2_shl_assign({}, u64)", T::NAME),
        BenchmarkType::EvaluationStrategy,
        natural_unsigned_unsigned_triple_gen_var_6::<T>().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &triple_3_bucketer("pow"),
        &mut [
            (
                &format!("Natural.mod_power_of_2_shl({}, u64)", T::NAME),
                &mut |(x, y, pow)| no_out!(x.mod_power_of_2_shl(y, pow)),
            ),
            (
                &format!("(&Natural).mod_power_of_2_shl({}, u64)", T::NAME),
                &mut |(x, y, pow)| no_out!((&x).mod_power_of_2_shl(y, pow)),
            ),
        ],
    );
}

fn benchmark_natural_mod_power_of_2_shl_unsigned_algorithms<T: PrimitiveUnsigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) where
    Natural: ModPowerOf2Shl<T> + Shl<T, Output = Natural>,
{
    run_benchmark(
        &format!("Natural.mod_power_of_2_shl_assign({}, u64)", T::NAME),
        BenchmarkType::Algorithms,
        natural_unsigned_unsigned_triple_gen_var_6::<T>().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &triple_3_bucketer("pow"),
        &mut [
            ("default", &mut |(x, y, pow)| {
                no_out!(x.mod_power_of_2_shl(y, pow))
            }),
            (
                &format!("(Natural << {}).mod_power_of_2(u64)", T::NAME),
                &mut |(x, y, pow)| no_out!((x << y).mod_power_of_2(pow)),
            ),
        ],
    );
}

fn benchmark_natural_mod_power_of_2_shl_assign_signed<T: PrimitiveSigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) where
    Natural: ModPowerOf2ShlAssign<T>,
{
    run_benchmark(
        &format!("Natural.mod_power_of_2_shl_assign({}, u64)", T::NAME),
        BenchmarkType::Single,
        natural_signed_unsigned_triple_gen_var_1::<T>().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &triple_3_bucketer("pow"),
        &mut [("Malachite", &mut |(mut x, y, pow)| {
            x.mod_power_of_2_shl_assign(y, pow)
        })],
    );
}

fn benchmark_natural_mod_power_of_2_shl_signed_evaluation_strategy<T: PrimitiveSigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) where
    Natural: ModPowerOf2Shl<T>,
    for<'a> &'a Natural: ModPowerOf2Shl<T>,
{
    run_benchmark(
        &format!("Natural.mod_power_of_2_shl_assign({}, u64)", T::NAME),
        BenchmarkType::EvaluationStrategy,
        natural_signed_unsigned_triple_gen_var_1::<T>().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &triple_3_bucketer("pow"),
        &mut [
            (
                &format!("Natural.mod_power_of_2_shl({}, u64)", T::NAME),
                &mut |(x, y, pow)| no_out!(x.mod_power_of_2_shl(y, pow)),
            ),
            (
                &format!("(&Natural).mod_power_of_2_shl({}, u64)", T::NAME),
                &mut |(x, y, pow)| no_out!((&x).mod_power_of_2_shl(y, pow)),
            ),
        ],
    );
}

fn benchmark_natural_mod_power_of_2_shl_signed_algorithms<T: PrimitiveSigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) where
    Natural: ModPowerOf2Shl<T> + Shl<T, Output = Natural>,
{
    run_benchmark(
        &format!("Natural.mod_power_of_2_shl_assign({}, u64)", T::NAME),
        BenchmarkType::Algorithms,
        natural_signed_unsigned_triple_gen_var_1::<T>().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &triple_3_bucketer("pow"),
        &mut [
            ("default", &mut |(x, y, pow)| {
                no_out!(x.mod_power_of_2_shl(y, pow))
            }),
            (
                &format!("(Natural << {}).mod_power_of_2(u64)", T::NAME),
                &mut |(x, y, pow)| no_out!((x << y).mod_power_of_2(pow)),
            ),
        ],
    );
}
