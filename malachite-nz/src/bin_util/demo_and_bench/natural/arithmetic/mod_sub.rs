// Copyright © 2025 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::arithmetic::traits::{Mod, ModSub, ModSubAssign};
use malachite_base::test_util::bench::{run_benchmark, BenchmarkType};
use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::runner::Runner;
use malachite_nz::integer::Integer;
use malachite_nz::test_util::bench::bucketers::triple_3_natural_bit_bucketer;
use malachite_nz::test_util::generators::natural_triple_gen_var_3;

pub(crate) fn register(runner: &mut Runner) {
    register_demo!(runner, demo_natural_mod_sub_assign);
    register_demo!(runner, demo_natural_mod_sub_assign_val_ref);
    register_demo!(runner, demo_natural_mod_sub_assign_ref_val);
    register_demo!(runner, demo_natural_mod_sub_assign_ref_ref);
    register_demo!(runner, demo_natural_mod_sub);
    register_demo!(runner, demo_natural_mod_sub_val_val_ref);
    register_demo!(runner, demo_natural_mod_sub_val_ref_val);
    register_demo!(runner, demo_natural_mod_sub_val_ref_ref);
    register_demo!(runner, demo_natural_mod_sub_ref_val_val);
    register_demo!(runner, demo_natural_mod_sub_ref_val_ref);
    register_demo!(runner, demo_natural_mod_sub_ref_ref_val);
    register_demo!(runner, demo_natural_mod_sub_ref_ref_ref);

    register_bench!(runner, benchmark_natural_mod_sub_assign_evaluation_strategy);
    register_bench!(runner, benchmark_natural_mod_sub_algorithms);
    register_bench!(runner, benchmark_natural_mod_sub_evaluation_strategy);
}

fn demo_natural_mod_sub_assign(gm: GenMode, config: &GenConfig, limit: usize) {
    for (mut x, y, m) in natural_triple_gen_var_3().get(gm, config).take(limit) {
        let x_old = x.clone();
        let y_old = y.clone();
        let m_old = m.clone();
        x.mod_sub_assign(y, m);
        println!("x := {x_old}; x.mod_sub_assign({y_old}, {m_old}); x = {x}");
    }
}

fn demo_natural_mod_sub_assign_val_ref(gm: GenMode, config: &GenConfig, limit: usize) {
    for (mut x, y, m) in natural_triple_gen_var_3().get(gm, config).take(limit) {
        let x_old = x.clone();
        let m_old = m.clone();
        let y_old = y.clone();
        x.mod_sub_assign(y, &m);
        println!("x := {x_old}; x.mod_sub_assign({y_old}, &{m_old}); x = {x}");
    }
}

fn demo_natural_mod_sub_assign_ref_val(gm: GenMode, config: &GenConfig, limit: usize) {
    for (mut x, y, m) in natural_triple_gen_var_3().get(gm, config).take(limit) {
        let x_old = x.clone();
        let m_old = m.clone();
        x.mod_sub_assign(&y, m);
        println!("x := {x_old}; x.mod_sub_assign(&{y}, {m_old}); x = {x}");
    }
}

fn demo_natural_mod_sub_assign_ref_ref(gm: GenMode, config: &GenConfig, limit: usize) {
    for (mut x, y, m) in natural_triple_gen_var_3().get(gm, config).take(limit) {
        let x_old = x.clone();
        x.mod_sub_assign(&y, &m);
        println!("x := {x_old}; x.mod_sub_assign(&{y}, &{m}); x = {x}");
    }
}

fn demo_natural_mod_sub(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, y, m) in natural_triple_gen_var_3().get(gm, config).take(limit) {
        let x_old = x.clone();
        let y_old = y.clone();
        let m_old = m.clone();
        println!("{} - {} ≡ {} mod {}", x_old, y_old, x.mod_sub(y, m), m_old);
    }
}

fn demo_natural_mod_sub_val_val_ref(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, y, m) in natural_triple_gen_var_3().get(gm, config).take(limit) {
        let x_old = x.clone();
        let y_old = y.clone();
        println!("{} - {} ≡ {} mod {}", x_old, y_old, x.mod_sub(y, &m), m);
    }
}

fn demo_natural_mod_sub_val_ref_val(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, y, m) in natural_triple_gen_var_3().get(gm, config).take(limit) {
        let x_old = x.clone();
        let m_old = m.clone();
        println!("{} - {} ≡ {} mod {}", x_old, y, x.mod_sub(&y, m), m_old);
    }
}

fn demo_natural_mod_sub_val_ref_ref(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, y, m) in natural_triple_gen_var_3().get(gm, config).take(limit) {
        let x_old = x.clone();
        println!("{} - {} ≡ {} mod {}", x_old, y, x.mod_sub(&y, &m), m);
    }
}

fn demo_natural_mod_sub_ref_val_val(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, y, m) in natural_triple_gen_var_3().get(gm, config).take(limit) {
        let y_old = y.clone();
        let m_old = m.clone();
        println!("{} - {} ≡ {} mod {}", x, y_old, (&x).mod_sub(y, m), m_old);
    }
}

fn demo_natural_mod_sub_ref_val_ref(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, y, m) in natural_triple_gen_var_3().get(gm, config).take(limit) {
        let y_old = y.clone();
        println!("{} - {} ≡ {} mod {}", x, y_old, (&x).mod_sub(y, &m), m);
    }
}

fn demo_natural_mod_sub_ref_ref_val(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, y, m) in natural_triple_gen_var_3().get(gm, config).take(limit) {
        let m_old = m.clone();
        println!("{} - {} ≡ {} mod {}", x, y, (&x).mod_sub(&y, m), m_old);
    }
}

fn demo_natural_mod_sub_ref_ref_ref(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, y, m) in natural_triple_gen_var_3().get(gm, config).take(limit) {
        println!("{} - {} ≡ {} mod {}", x, y, (&x).mod_sub(&y, &m), m);
    }
}

fn benchmark_natural_mod_sub_assign_evaluation_strategy(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Natural.mod_sub_assign(Natural, Natural)",
        BenchmarkType::EvaluationStrategy,
        natural_triple_gen_var_3().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &triple_3_natural_bit_bucketer("m"),
        &mut [
            (
                "Natural.mod_sub_assign(Natural, Natural)",
                &mut |(mut x, y, m)| no_out!(x.mod_sub_assign(y, m)),
            ),
            (
                "Natural.mod_sub_assign(Natural, &Natural)",
                &mut |(mut x, y, m)| no_out!(x.mod_sub_assign(y, &m)),
            ),
            (
                "Natural.mod_sub_assign(&Natural, Natural)",
                &mut |(mut x, y, m)| no_out!(x.mod_sub_assign(&y, m)),
            ),
            (
                "Natural.mod_sub_assign(&Natural, &Natural)",
                &mut |(mut x, y, m)| no_out!(x.mod_sub_assign(&y, &m)),
            ),
        ],
    );
}

#[allow(clippy::no_effect, unused_must_use)]
fn benchmark_natural_mod_sub_algorithms(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Natural.mod_sub(Natural, Natural)",
        BenchmarkType::Algorithms,
        natural_triple_gen_var_3().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &triple_3_natural_bit_bucketer("m"),
        &mut [
            ("default", &mut |(x, y, m)| no_out!(x.mod_sub(y, m))),
            ("naive", &mut |(x, y, m)| {
                no_out!((Integer::from(x) - Integer::from(y)).mod_op(Integer::from(m)))
            }),
        ],
    );
}

fn benchmark_natural_mod_sub_evaluation_strategy(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Natural.mod_sub(Natural, Natural)",
        BenchmarkType::EvaluationStrategy,
        natural_triple_gen_var_3().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &triple_3_natural_bit_bucketer("m"),
        &mut [
            ("Natural.mod_sub(Natural, Natural)", &mut |(x, y, m)| {
                no_out!(x.mod_sub(y, m))
            }),
            ("Natural.mod_sub(Natural, &Natural)", &mut |(x, y, m)| {
                no_out!(x.mod_sub(y, &m))
            }),
            ("Natural.mod_sub(&Natural, Natural)", &mut |(x, y, m)| {
                no_out!(x.mod_sub(&y, m))
            }),
            ("Natural.mod_sub(&Natural, &Natural)", &mut |(x, y, m)| {
                no_out!(x.mod_sub(&y, &m))
            }),
            ("(&Natural).mod_sub(Natural, Natural)", &mut |(x, y, m)| {
                no_out!((&x).mod_sub(y, m))
            }),
            ("(&Natural).mod_sub(Natural, &Natural)", &mut |(x, y, m)| {
                no_out!((&x).mod_sub(y, &m))
            }),
            ("(&Natural).mod_sub(&Natural, Natural)", &mut |(x, y, m)| {
                no_out!((&x).mod_sub(&y, m))
            }),
            (
                "(&Natural).mod_sub(&Natural, &Natural)",
                &mut |(x, y, m)| no_out!((&x).mod_sub(&y, &m)),
            ),
        ],
    );
}
