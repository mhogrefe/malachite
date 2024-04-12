// Copyright © 2024 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::arithmetic::traits::{ModShr, ModShrAssign};
use malachite_base::num::basic::signeds::PrimitiveSigned;
use malachite_base::test_util::bench::bucketers::triple_3_bit_bucketer;
use malachite_base::test_util::bench::{run_benchmark, BenchmarkType};
use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::runner::Runner;
use malachite_nz::natural::Natural;
use malachite_nz::test_util::generators::natural_natural_signed_triple_gen_var_1;
use std::ops::Shr;

pub(crate) fn register(runner: &mut Runner) {
    register_signed_demos!(runner, demo_natural_mod_shr_assign);
    register_signed_demos!(runner, demo_natural_mod_shr_assign_ref);
    register_signed_demos!(runner, demo_natural_mod_shr);
    register_signed_demos!(runner, demo_natural_mod_shr_val_ref);
    register_signed_demos!(runner, demo_natural_mod_shr_ref_val);
    register_signed_demos!(runner, demo_natural_mod_shr_ref_ref);

    register_signed_benches!(runner, benchmark_natural_mod_shr_assign_evaluation_strategy);
    register_signed_benches!(runner, benchmark_natural_mod_shr_algorithms);
    register_signed_benches!(runner, benchmark_natural_mod_shr_evaluation_strategy);
}

fn demo_natural_mod_shr_assign<T: PrimitiveSigned>(gm: GenMode, config: &GenConfig, limit: usize)
where
    Natural: ModShrAssign<T>,
{
    for (mut n, m, i) in natural_natural_signed_triple_gen_var_1::<T>()
        .get(gm, config)
        .take(limit)
    {
        let n_old = n.clone();
        n.mod_shr_assign(i, m.clone());
        println!("x := {n_old}; x.mod_shr_assign({i}, {m}); x = {n}");
    }
}

fn demo_natural_mod_shr_assign_ref<T: PrimitiveSigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) where
    for<'a> Natural: ModShrAssign<T, &'a Natural>,
{
    for (mut n, m, i) in natural_natural_signed_triple_gen_var_1::<T>()
        .get(gm, config)
        .take(limit)
    {
        let n_old = n.clone();
        n.mod_shr_assign(i, &m);
        println!("x := {n_old}; x.mod_shr_assign({i}, &{m}); x = {n}");
    }
}

fn demo_natural_mod_shr<T: PrimitiveSigned>(gm: GenMode, config: &GenConfig, limit: usize)
where
    Natural: ModShr<T, Output = Natural>,
{
    for (n, m, i) in natural_natural_signed_triple_gen_var_1::<T>()
        .get(gm, config)
        .take(limit)
    {
        let n_old = n.clone();
        println!(
            "{}.mod_shr({}, {}) = {}",
            n_old,
            i,
            m.clone(),
            n.mod_shr(i, m)
        );
    }
}

fn demo_natural_mod_shr_val_ref<T: PrimitiveSigned>(gm: GenMode, config: &GenConfig, limit: usize)
where
    for<'a> Natural: ModShr<T, &'a Natural, Output = Natural>,
{
    for (n, m, i) in natural_natural_signed_triple_gen_var_1::<T>()
        .get(gm, config)
        .take(limit)
    {
        let n_old = n.clone();
        println!("{}.mod_shr({}, &{}) = {}", n_old, i, m, n.mod_shr(i, &m));
    }
}

fn demo_natural_mod_shr_ref_val<T: PrimitiveSigned>(gm: GenMode, config: &GenConfig, limit: usize)
where
    for<'a> &'a Natural: ModShr<T, Natural, Output = Natural>,
{
    for (n, m, i) in natural_natural_signed_triple_gen_var_1::<T>()
        .get(gm, config)
        .take(limit)
    {
        println!(
            "(&{}).mod_shr({}, {}) = {}",
            n,
            i,
            m.clone(),
            (&n).mod_shr(i, m)
        );
    }
}

fn demo_natural_mod_shr_ref_ref<T: PrimitiveSigned>(gm: GenMode, config: &GenConfig, limit: usize)
where
    for<'a, 'b> &'a Natural: ModShr<T, &'b Natural, Output = Natural>,
{
    for (n, m, i) in natural_natural_signed_triple_gen_var_1::<T>()
        .get(gm, config)
        .take(limit)
    {
        println!("(&{}).mod_shr({}, &{}) = {}", n, i, m, (&n).mod_shr(i, &m));
    }
}

#[allow(clippy::trait_duplication_in_bounds)]
fn benchmark_natural_mod_shr_assign_evaluation_strategy<T: PrimitiveSigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) where
    for<'a> Natural: ModShrAssign<T> + ModShrAssign<T, &'a Natural>,
{
    run_benchmark(
        &format!("Natural.mod_shr_assign({}, Natural)", T::NAME),
        BenchmarkType::EvaluationStrategy,
        natural_natural_signed_triple_gen_var_1::<T>().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &triple_3_bit_bucketer("m"),
        &mut [
            (
                &format!("Natural.mod_shr_assign({}, Natural)", T::NAME),
                &mut |(mut x, m, y)| no_out!(x.mod_shr_assign(y, m)),
            ),
            (
                &format!("Natural.mod_shr_assign({}, &Natural)", T::NAME),
                &mut |(mut x, m, y)| no_out!(x.mod_shr_assign(y, &m)),
            ),
        ],
    );
}

#[allow(clippy::no_effect, unused_must_use)]
fn benchmark_natural_mod_shr_algorithms<T: PrimitiveSigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) where
    Natural: ModShr<T, Output = Natural> + Shr<T, Output = Natural>,
{
    run_benchmark(
        &format!("Natural.mod_shr({}, Natural)", T::NAME),
        BenchmarkType::Algorithms,
        natural_natural_signed_triple_gen_var_1::<T>().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &triple_3_bit_bucketer("m"),
        &mut [
            ("default", &mut |(x, m, y)| no_out!(x.mod_shr(y, m))),
            ("using >> and %", &mut |(x, m, y)| no_out!((x >> y) % m)),
        ],
    );
}

#[allow(clippy::trait_duplication_in_bounds)]
fn benchmark_natural_mod_shr_evaluation_strategy<T: PrimitiveSigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) where
    for<'a> Natural: ModShr<T, Output = Natural> + ModShr<T, &'a Natural, Output = Natural>,
    for<'a, 'b> &'a Natural:
        ModShr<T, Natural, Output = Natural> + ModShr<T, &'b Natural, Output = Natural>,
{
    run_benchmark(
        &format!("Natural.mod_shr({}, Natural)", T::NAME),
        BenchmarkType::EvaluationStrategy,
        natural_natural_signed_triple_gen_var_1::<T>().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &triple_3_bit_bucketer("m"),
        &mut [
            (
                &format!("Natural.mod_shr({}, Natural)", T::NAME),
                &mut |(x, m, y)| no_out!(x.mod_shr(y, m)),
            ),
            (
                &format!("Natural.mod_shr({}, &Natural)", T::NAME),
                &mut |(x, m, y)| no_out!(x.mod_shr(y, &m)),
            ),
            (
                &format!("(&Natural).mod_shr({}, Natural)", T::NAME),
                &mut |(x, m, y)| no_out!((&x).mod_shr(y, m)),
            ),
            (
                &format!("(&Natural).mod_shr({}, &Natural)", T::NAME),
                &mut |(x, m, y)| no_out!((&x).mod_shr(y, &m)),
            ),
        ],
    );
}
