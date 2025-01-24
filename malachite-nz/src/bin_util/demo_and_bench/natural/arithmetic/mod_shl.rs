// Copyright © 2025 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::arithmetic::traits::{ModShl, ModShlAssign};
use malachite_base::num::basic::signeds::PrimitiveSigned;
use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base::test_util::bench::bucketers::triple_3_bit_bucketer;
use malachite_base::test_util::bench::{run_benchmark, BenchmarkType};
use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::runner::Runner;
use malachite_nz::natural::Natural;
use malachite_nz::test_util::generators::{
    natural_natural_signed_triple_gen_var_1, natural_natural_unsigned_triple_gen_var_6,
};
use std::ops::Shl;

pub(crate) fn register(runner: &mut Runner) {
    register_unsigned_demos!(runner, demo_natural_mod_shl_assign_unsigned);
    register_unsigned_demos!(runner, demo_natural_mod_shl_assign_unsigned_ref);
    register_unsigned_demos!(runner, demo_natural_mod_shl_unsigned);
    register_unsigned_demos!(runner, demo_natural_mod_shl_unsigned_val_ref);
    register_unsigned_demos!(runner, demo_natural_mod_shl_unsigned_ref_val);
    register_unsigned_demos!(runner, demo_natural_mod_shl_unsigned_ref_ref);
    register_signed_demos!(runner, demo_natural_mod_shl_assign_signed);
    register_signed_demos!(runner, demo_natural_mod_shl_assign_signed_ref);
    register_signed_demos!(runner, demo_natural_mod_shl_signed);
    register_signed_demos!(runner, demo_natural_mod_shl_signed_val_ref);
    register_signed_demos!(runner, demo_natural_mod_shl_signed_ref_val);
    register_signed_demos!(runner, demo_natural_mod_shl_signed_ref_ref);

    register_unsigned_benches!(
        runner,
        benchmark_natural_mod_shl_assign_unsigned_evaluation_strategy
    );
    register_unsigned_benches!(runner, benchmark_natural_mod_shl_unsigned_algorithms);
    register_unsigned_benches!(
        runner,
        benchmark_natural_mod_shl_unsigned_evaluation_strategy
    );
    register_signed_benches!(
        runner,
        benchmark_natural_mod_shl_assign_signed_evaluation_strategy
    );
    register_signed_benches!(runner, benchmark_natural_mod_shl_signed_algorithms);
    register_signed_benches!(runner, benchmark_natural_mod_shl_signed_evaluation_strategy);
}

fn demo_natural_mod_shl_assign_unsigned<T: PrimitiveUnsigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) where
    Natural: ModShlAssign<T>,
{
    for (mut n, m, u) in natural_natural_unsigned_triple_gen_var_6::<T>()
        .get(gm, config)
        .take(limit)
    {
        let n_old = n.clone();
        n.mod_shl_assign(u, m.clone());
        println!("x := {n_old}; x.mod_shl_assign({u}, {m}); x = {n}");
    }
}

fn demo_natural_mod_shl_assign_unsigned_ref<T: PrimitiveUnsigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) where
    for<'a> Natural: ModShlAssign<T, &'a Natural>,
{
    for (mut n, m, u) in natural_natural_unsigned_triple_gen_var_6::<T>()
        .get(gm, config)
        .take(limit)
    {
        let n_old = n.clone();
        n.mod_shl_assign(u, &m);
        println!("x := {n_old}; x.mod_shl_assign({u}, &{m}); x = {n}");
    }
}

fn demo_natural_mod_shl_unsigned<T: PrimitiveUnsigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) where
    Natural: ModShl<T, Output = Natural>,
{
    for (n, m, u) in natural_natural_unsigned_triple_gen_var_6::<T>()
        .get(gm, config)
        .take(limit)
    {
        let n_old = n.clone();
        println!(
            "{}.mod_shl({}, {}) = {}",
            n_old,
            u,
            m.clone(),
            n.mod_shl(u, m)
        );
    }
}

fn demo_natural_mod_shl_unsigned_val_ref<T: PrimitiveUnsigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) where
    for<'a> Natural: ModShl<T, &'a Natural, Output = Natural>,
{
    for (n, m, u) in natural_natural_unsigned_triple_gen_var_6::<T>()
        .get(gm, config)
        .take(limit)
    {
        let n_old = n.clone();
        println!("{}.mod_shl({}, &{}) = {}", n_old, u, m, n.mod_shl(u, &m));
    }
}

fn demo_natural_mod_shl_unsigned_ref_val<T: PrimitiveUnsigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) where
    for<'a> &'a Natural: ModShl<T, Natural, Output = Natural>,
{
    for (n, m, u) in natural_natural_unsigned_triple_gen_var_6::<T>()
        .get(gm, config)
        .take(limit)
    {
        println!(
            "(&{}).mod_shl({}, {}) = {}",
            n,
            u,
            m.clone(),
            (&n).mod_shl(u, m)
        );
    }
}

fn demo_natural_mod_shl_unsigned_ref_ref<T: PrimitiveUnsigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) where
    for<'a, 'b> &'a Natural: ModShl<T, &'b Natural, Output = Natural>,
{
    for (n, m, u) in natural_natural_unsigned_triple_gen_var_6::<T>()
        .get(gm, config)
        .take(limit)
    {
        println!("(&{}).mod_shl({}, &{}) = {}", n, u, m, (&n).mod_shl(u, &m));
    }
}

fn demo_natural_mod_shl_assign_signed<T: PrimitiveSigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) where
    Natural: ModShlAssign<T>,
{
    for (mut n, m, i) in natural_natural_signed_triple_gen_var_1::<T>()
        .get(gm, config)
        .take(limit)
    {
        let n_old = n.clone();
        n.mod_shl_assign(i, m.clone());
        println!("x := {n_old}; x.mod_shl_assign({i}, {m}); x = {n}");
    }
}

fn demo_natural_mod_shl_assign_signed_ref<T: PrimitiveSigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) where
    for<'a> Natural: ModShlAssign<T, &'a Natural>,
{
    for (mut n, m, i) in natural_natural_signed_triple_gen_var_1::<T>()
        .get(gm, config)
        .take(limit)
    {
        let n_old = n.clone();
        n.mod_shl_assign(i, &m);
        println!("x := {n_old}; x.mod_shl_assign({i}, &{m}); x = {n}");
    }
}

fn demo_natural_mod_shl_signed<T: PrimitiveSigned>(gm: GenMode, config: &GenConfig, limit: usize)
where
    Natural: ModShl<T, Output = Natural>,
{
    for (n, m, i) in natural_natural_signed_triple_gen_var_1::<T>()
        .get(gm, config)
        .take(limit)
    {
        let n_old = n.clone();
        println!(
            "{}.mod_shl({}, {}) = {}",
            n_old,
            i,
            m.clone(),
            n.mod_shl(i, m)
        );
    }
}

fn demo_natural_mod_shl_signed_val_ref<T: PrimitiveSigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) where
    for<'a> Natural: ModShl<T, &'a Natural, Output = Natural>,
{
    for (n, m, i) in natural_natural_signed_triple_gen_var_1::<T>()
        .get(gm, config)
        .take(limit)
    {
        let n_old = n.clone();
        println!("{}.mod_shl({}, &{}) = {}", n_old, i, m, n.mod_shl(i, &m));
    }
}

fn demo_natural_mod_shl_signed_ref_val<T: PrimitiveSigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) where
    for<'a> &'a Natural: ModShl<T, Natural, Output = Natural>,
{
    for (n, m, i) in natural_natural_signed_triple_gen_var_1::<T>()
        .get(gm, config)
        .take(limit)
    {
        println!(
            "(&{}).mod_shl({}, {}) = {}",
            n,
            i,
            m.clone(),
            (&n).mod_shl(i, m)
        );
    }
}

fn demo_natural_mod_shl_signed_ref_ref<T: PrimitiveSigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) where
    for<'a, 'b> &'a Natural: ModShl<T, &'b Natural, Output = Natural>,
{
    for (n, m, i) in natural_natural_signed_triple_gen_var_1::<T>()
        .get(gm, config)
        .take(limit)
    {
        println!("(&{}).mod_shl({}, &{}) = {}", n, i, m, (&n).mod_shl(i, &m));
    }
}

#[allow(clippy::trait_duplication_in_bounds)]
fn benchmark_natural_mod_shl_assign_unsigned_evaluation_strategy<T: PrimitiveUnsigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) where
    for<'a> Natural: ModShlAssign<T> + ModShlAssign<T, &'a Natural>,
{
    run_benchmark(
        &format!("Natural.mod_shl_assign({}, Natural)", T::NAME),
        BenchmarkType::EvaluationStrategy,
        natural_natural_unsigned_triple_gen_var_6::<T>().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &triple_3_bit_bucketer("m"),
        &mut [
            (
                &format!("Natural.mod_shl_assign({}, Natural)", T::NAME),
                &mut |(mut x, m, y)| no_out!(x.mod_shl_assign(y, m)),
            ),
            (
                &format!("Natural.mod_shl_assign({}, &Natural)", T::NAME),
                &mut |(mut x, m, y)| no_out!(x.mod_shl_assign(y, &m)),
            ),
        ],
    );
}

#[allow(clippy::no_effect, unused_must_use)]
fn benchmark_natural_mod_shl_unsigned_algorithms<T: PrimitiveUnsigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) where
    Natural: ModShl<T, Output = Natural> + Shl<T, Output = Natural>,
{
    run_benchmark(
        &format!("Natural.mod_shl({}, Natural)", T::NAME),
        BenchmarkType::Algorithms,
        natural_natural_unsigned_triple_gen_var_6::<T>().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &triple_3_bit_bucketer("m"),
        &mut [
            ("default", &mut |(x, m, y)| no_out!(x.mod_shl(y, m))),
            ("using << and %", &mut |(x, m, y)| no_out!((x << y) % m)),
        ],
    );
}

#[allow(clippy::trait_duplication_in_bounds)]
fn benchmark_natural_mod_shl_unsigned_evaluation_strategy<T: PrimitiveUnsigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) where
    for<'a> Natural: ModShl<T, Output = Natural> + ModShl<T, &'a Natural, Output = Natural>,
    for<'a, 'b> &'a Natural:
        ModShl<T, Natural, Output = Natural> + ModShl<T, &'b Natural, Output = Natural>,
{
    run_benchmark(
        &format!("Natural.mod_shl({}, Natural)", T::NAME),
        BenchmarkType::EvaluationStrategy,
        natural_natural_unsigned_triple_gen_var_6::<T>().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &triple_3_bit_bucketer("m"),
        &mut [
            (
                &format!("Natural.mod_shl({}, Natural)", T::NAME),
                &mut |(x, m, y)| no_out!(x.mod_shl(y, m)),
            ),
            (
                &format!("Natural.mod_shl({}, &Natural)", T::NAME),
                &mut |(x, m, y)| no_out!(x.mod_shl(y, &m)),
            ),
            (
                &format!("(&Natural).mod_shl({}, Natural)", T::NAME),
                &mut |(x, m, y)| no_out!((&x).mod_shl(y, m)),
            ),
            (
                &format!("(&Natural).mod_shl({}, &Natural)", T::NAME),
                &mut |(x, m, y)| no_out!((&x).mod_shl(y, &m)),
            ),
        ],
    );
}

#[allow(clippy::trait_duplication_in_bounds)]
fn benchmark_natural_mod_shl_assign_signed_evaluation_strategy<T: PrimitiveSigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) where
    for<'a> Natural: ModShlAssign<T> + ModShlAssign<T, &'a Natural>,
{
    run_benchmark(
        &format!("Natural.mod_shl_assign({}, Natural)", T::NAME),
        BenchmarkType::EvaluationStrategy,
        natural_natural_signed_triple_gen_var_1::<T>().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &triple_3_bit_bucketer("m"),
        &mut [
            (
                &format!("Natural.mod_shl_assign({}, Natural)", T::NAME),
                &mut |(mut x, m, y)| no_out!(x.mod_shl_assign(y, m)),
            ),
            (
                &format!("Natural.mod_shl_assign({}, &Natural)", T::NAME),
                &mut |(mut x, m, y)| no_out!(x.mod_shl_assign(y, &m)),
            ),
        ],
    );
}

#[allow(clippy::no_effect, unused_must_use)]
fn benchmark_natural_mod_shl_signed_algorithms<T: PrimitiveSigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) where
    Natural: ModShl<T, Output = Natural> + Shl<T, Output = Natural>,
{
    run_benchmark(
        &format!("Natural.mod_shl({}, Natural)", T::NAME),
        BenchmarkType::Algorithms,
        natural_natural_signed_triple_gen_var_1::<T>().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &triple_3_bit_bucketer("m"),
        &mut [
            ("default", &mut |(x, m, y)| no_out!(x.mod_shl(y, m))),
            ("using << and %", &mut |(x, m, y)| no_out!((x << y) % m)),
        ],
    );
}

#[allow(clippy::trait_duplication_in_bounds)]
fn benchmark_natural_mod_shl_signed_evaluation_strategy<T: PrimitiveSigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) where
    for<'a> Natural: ModShl<T, Output = Natural> + ModShl<T, &'a Natural, Output = Natural>,
    for<'a, 'b> &'a Natural:
        ModShl<T, Natural, Output = Natural> + ModShl<T, &'b Natural, Output = Natural>,
{
    run_benchmark(
        &format!("Natural.mod_shl({}, Natural)", T::NAME),
        BenchmarkType::EvaluationStrategy,
        natural_natural_signed_triple_gen_var_1::<T>().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &triple_3_bit_bucketer("m"),
        &mut [
            (
                &format!("Natural.mod_shl({}, Natural)", T::NAME),
                &mut |(x, m, y)| no_out!(x.mod_shl(y, m)),
            ),
            (
                &format!("Natural.mod_shl({}, &Natural)", T::NAME),
                &mut |(x, m, y)| no_out!(x.mod_shl(y, &m)),
            ),
            (
                &format!("(&Natural).mod_shl({}, Natural)", T::NAME),
                &mut |(x, m, y)| no_out!((&x).mod_shl(y, m)),
            ),
            (
                &format!("(&Natural).mod_shl({}, &Natural)", T::NAME),
                &mut |(x, m, y)| no_out!((&x).mod_shl(y, &m)),
            ),
        ],
    );
}
