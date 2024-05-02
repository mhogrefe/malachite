// Copyright © 2024 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::arithmetic::traits::{Mod, ModNeg, ModNegAssign};
use malachite_base::num::conversion::traits::ExactFrom;
use malachite_base::test_util::bench::{run_benchmark, BenchmarkType};
use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::runner::Runner;
use malachite_nz::integer::Integer;
use malachite_nz::natural::Natural;
use malachite_nz::test_util::bench::bucketers::pair_2_natural_bit_bucketer;
use malachite_nz::test_util::generators::natural_pair_gen_var_8;

pub(crate) fn register(runner: &mut Runner) {
    register_demo!(runner, demo_natural_mod_neg_assign);
    register_demo!(runner, demo_natural_mod_neg_assign_ref);
    register_demo!(runner, demo_natural_mod_neg);
    register_demo!(runner, demo_natural_mod_neg_val_ref);
    register_demo!(runner, demo_natural_mod_neg_ref_val);
    register_demo!(runner, demo_natural_mod_neg_ref_ref);

    register_bench!(runner, benchmark_natural_mod_neg_assign_evaluation_strategy);
    register_bench!(runner, benchmark_natural_mod_neg_evaluation_strategy);
    register_bench!(runner, benchmark_natural_mod_neg_algorithms);
}

fn demo_natural_mod_neg_assign(gm: GenMode, config: &GenConfig, limit: usize) {
    for (mut n, m) in natural_pair_gen_var_8().get(gm, config).take(limit) {
        let n_old = n.clone();
        let m_old = m.clone();
        n.mod_neg_assign(m);
        println!("x := {n_old}; x.mod_neg_assign({m_old}); x = {n}");
    }
}

fn demo_natural_mod_neg_assign_ref(gm: GenMode, config: &GenConfig, limit: usize) {
    for (mut n, m) in natural_pair_gen_var_8().get(gm, config).take(limit) {
        let n_old = n.clone();
        n.mod_neg_assign(&m);
        println!("x := {n_old}; x.mod_neg_assign(&{m}); x = {n}");
    }
}

fn demo_natural_mod_neg(gm: GenMode, config: &GenConfig, limit: usize) {
    for (n, m) in natural_pair_gen_var_8().get(gm, config).take(limit) {
        let n_old = n.clone();
        let m_old = m.clone();
        println!("-{} ≡ {} mod {}", n_old, n.mod_neg(m), m_old);
    }
}

fn demo_natural_mod_neg_val_ref(gm: GenMode, config: &GenConfig, limit: usize) {
    for (n, m) in natural_pair_gen_var_8().get(gm, config).take(limit) {
        let n_old = n.clone();
        println!("-{} ≡ {} mod &{}", n_old, n.mod_neg(&m), m);
    }
}

fn demo_natural_mod_neg_ref_val(gm: GenMode, config: &GenConfig, limit: usize) {
    for (n, m) in natural_pair_gen_var_8().get(gm, config).take(limit) {
        let m_old = m.clone();
        println!("&(-{}) ≡ {} mod {}", n, (&n).mod_neg(m), m_old);
    }
}

fn demo_natural_mod_neg_ref_ref(gm: GenMode, config: &GenConfig, limit: usize) {
    for (n, m) in natural_pair_gen_var_8().get(gm, config).take(limit) {
        println!("&(-{}) ≡ {} mod &{}", n, (&n).mod_neg(&m), m);
    }
}

fn benchmark_natural_mod_neg_assign_evaluation_strategy(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Natural.mod_neg_assign(Natural)",
        BenchmarkType::EvaluationStrategy,
        natural_pair_gen_var_8().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_2_natural_bit_bucketer("m"),
        &mut [
            ("Natural.mod_neg_assign(Natural)", &mut |(mut n, m)| {
                n.mod_neg_assign(m)
            }),
            ("Natural.mod_neg_assign(&Natural)", &mut |(mut n, m)| {
                n.mod_neg_assign(&m)
            }),
        ],
    );
}

fn benchmark_natural_mod_neg_evaluation_strategy(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Natural.mod_neg(Natural)",
        BenchmarkType::EvaluationStrategy,
        natural_pair_gen_var_8().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_2_natural_bit_bucketer("m"),
        &mut [
            ("Natural.mod_neg(Natural)", &mut |(n, m)| {
                no_out!(n.mod_neg(m))
            }),
            ("Natural.mod_neg(&Natural)", &mut |(n, m)| {
                no_out!(n.mod_neg(&m))
            }),
            ("(&Natural).mod_neg(Natural)", &mut |(n, m)| {
                no_out!((&n).mod_neg(m))
            }),
            ("(&Natural).mod_neg(&Natural)", &mut |(n, m)| {
                no_out!((&n).mod_neg(&m))
            }),
        ],
    );
}

fn benchmark_natural_mod_neg_algorithms(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Natural.mod_neg(Natural)",
        BenchmarkType::Algorithms,
        natural_pair_gen_var_8().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_2_natural_bit_bucketer("m"),
        &mut [
            ("Natural.mod_neg(Natural)", &mut |(n, m)| {
                no_out!(n.mod_neg(m))
            }),
            ("(-Natural).mod(Natural)", &mut |(n, m)| {
                no_out!(Natural::exact_from((-n).mod_op(Integer::from(m))))
            }),
        ],
    );
}
