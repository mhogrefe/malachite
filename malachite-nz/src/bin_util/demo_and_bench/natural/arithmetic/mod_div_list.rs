// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::arithmetic::traits::ModDivList;
use malachite_base::test_util::bench::{BenchmarkType, run_benchmark};
use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::runner::Runner;
use malachite_nz::test_util::bench::bucketers::triple_3_natural_bit_bucketer;
use malachite_nz::test_util::generators::natural_triple_gen_var_3;
use malachite_nz::test_util::natural::arithmetic::mod_div_list::mod_div_list_simple;

pub(crate) fn register(runner: &mut Runner) {
    register_demo!(runner, demo_natural_mod_div_list);
    register_demo!(runner, demo_natural_mod_div_list_val_val_ref);
    register_demo!(runner, demo_natural_mod_div_list_val_ref_val);
    register_demo!(runner, demo_natural_mod_div_list_val_ref_ref);
    register_demo!(runner, demo_natural_mod_div_list_ref_val_val);
    register_demo!(runner, demo_natural_mod_div_list_ref_val_ref);
    register_demo!(runner, demo_natural_mod_div_list_ref_ref_val);
    register_demo!(runner, demo_natural_mod_div_list_ref_ref_ref);

    register_bench!(runner, benchmark_natural_mod_div_list_evaluation_strategy);
    register_bench!(runner, benchmark_natural_mod_div_list_algorithms);
}

fn demo_natural_mod_div_list(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, y, m) in natural_triple_gen_var_3().get(gm, config).take(limit) {
        let x_old = x.clone();
        let y_old = y.clone();
        let m_old = m.clone();
        println!(
            "{x_old}.mod_div_list({y_old}, {m_old}) = {:?}",
            x.mod_div_list(y, m)
        );
    }
}

fn demo_natural_mod_div_list_val_val_ref(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, y, m) in natural_triple_gen_var_3().get(gm, config).take(limit) {
        let x_old = x.clone();
        let y_old = y.clone();
        println!(
            "{x_old}.mod_div_list({y_old}, &{m}) = {:?}",
            x.mod_div_list(y, &m)
        );
    }
}

fn demo_natural_mod_div_list_val_ref_val(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, y, m) in natural_triple_gen_var_3().get(gm, config).take(limit) {
        let x_old = x.clone();
        let m_old = m.clone();
        println!(
            "{x_old}.mod_div_list(&{y}, {m_old}) = {:?}",
            x.mod_div_list(&y, m)
        );
    }
}

fn demo_natural_mod_div_list_val_ref_ref(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, y, m) in natural_triple_gen_var_3().get(gm, config).take(limit) {
        let x_old = x.clone();
        println!(
            "{x_old}.mod_div_list(&{y}, &{m}) = {:?}",
            x.mod_div_list(&y, &m)
        );
    }
}

fn demo_natural_mod_div_list_ref_val_val(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, y, m) in natural_triple_gen_var_3().get(gm, config).take(limit) {
        let y_old = y.clone();
        let m_old = m.clone();
        println!(
            "(&{x}).mod_div_list({y_old}, {m_old}) = {:?}",
            (&x).mod_div_list(y, m)
        );
    }
}

fn demo_natural_mod_div_list_ref_val_ref(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, y, m) in natural_triple_gen_var_3().get(gm, config).take(limit) {
        let y_old = y.clone();
        println!(
            "(&{x}).mod_div_list({y_old}, &{m}) = {:?}",
            (&x).mod_div_list(y, &m)
        );
    }
}

fn demo_natural_mod_div_list_ref_ref_val(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, y, m) in natural_triple_gen_var_3().get(gm, config).take(limit) {
        let m_old = m.clone();
        println!(
            "(&{x}).mod_div_list(&{y}, {m_old}) = {:?}",
            (&x).mod_div_list(&y, m)
        );
    }
}

fn demo_natural_mod_div_list_ref_ref_ref(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, y, m) in natural_triple_gen_var_3().get(gm, config).take(limit) {
        println!(
            "(&{x}).mod_div_list(&{y}, &{m}) = {:?}",
            (&x).mod_div_list(&y, &m)
        );
    }
}

fn benchmark_natural_mod_div_list_evaluation_strategy(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Natural.mod_div_list(Natural, Natural)",
        BenchmarkType::EvaluationStrategy,
        natural_triple_gen_var_3().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &triple_3_natural_bit_bucketer("m"),
        &mut [
            (
                "Natural.mod_div_list(Natural, Natural)",
                &mut |(x, y, m)| {
                    no_out!(x.mod_div_list(y, m));
                },
            ),
            (
                "Natural.mod_div_list(Natural, &Natural)",
                &mut |(x, y, m)| {
                    no_out!(x.mod_div_list(y, &m));
                },
            ),
            (
                "Natural.mod_div_list(&Natural, Natural)",
                &mut |(x, y, m)| {
                    no_out!(x.mod_div_list(&y, m));
                },
            ),
            (
                "Natural.mod_div_list(&Natural, &Natural)",
                &mut |(x, y, m)| {
                    no_out!(x.mod_div_list(&y, &m));
                },
            ),
            (
                "(&Natural).mod_div_list(Natural, Natural)",
                &mut |(x, y, m)| {
                    no_out!((&x).mod_div_list(y, m));
                },
            ),
            (
                "(&Natural).mod_div_list(Natural, &Natural)",
                &mut |(x, y, m)| {
                    no_out!((&x).mod_div_list(y, &m));
                },
            ),
            (
                "(&Natural).mod_div_list(&Natural, Natural)",
                &mut |(x, y, m)| {
                    no_out!((&x).mod_div_list(&y, m));
                },
            ),
            (
                "(&Natural).mod_div_list(&Natural, &Natural)",
                &mut |(x, y, m)| {
                    no_out!((&x).mod_div_list(&y, &m));
                },
            ),
        ],
    );
}

fn benchmark_natural_mod_div_list_algorithms(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Natural.mod_div_list(Natural, Natural)",
        BenchmarkType::Algorithms,
        natural_triple_gen_var_3().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &triple_3_natural_bit_bucketer("m"),
        &mut [
            ("default", &mut |(x, y, m)| no_out!(x.mod_div_list(y, m))),
            ("simple", &mut |(x, y, m)| {
                no_out!(mod_div_list_simple(x, y, m));
            }),
        ],
    );
}
