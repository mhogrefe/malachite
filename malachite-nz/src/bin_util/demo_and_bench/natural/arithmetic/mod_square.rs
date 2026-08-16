// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::arithmetic::traits::{
    ModMul, ModPowPrecomputed, ModSquare, ModSquareAssign, ModSquarePrecomputed,
    ModSquarePrecomputedAssign, Square,
};
use malachite_base::test_util::bench::{BenchmarkType, run_benchmark};
use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::runner::Runner;
use malachite_nz::natural::Natural;
use malachite_nz::test_util::bench::bucketers::pair_2_natural_bit_bucketer;
use malachite_nz::test_util::generators::natural_pair_gen_var_8;

pub(crate) fn register(runner: &mut Runner) {
    register_demo!(runner, demo_natural_mod_square_assign);
    register_demo!(runner, demo_natural_mod_square_assign_ref);
    register_demo!(runner, demo_natural_mod_square);
    register_demo!(runner, demo_natural_mod_square_val_ref);
    register_demo!(runner, demo_natural_mod_square_ref_val);
    register_demo!(runner, demo_natural_mod_square_ref_ref);
    register_demo!(runner, demo_natural_mod_square_precomputed);
    register_demo!(runner, demo_natural_mod_square_precomputed_val_ref);
    register_demo!(runner, demo_natural_mod_square_precomputed_ref_val);
    register_demo!(runner, demo_natural_mod_square_precomputed_ref_ref);
    register_demo!(runner, demo_natural_mod_square_precomputed_assign);
    register_demo!(runner, demo_natural_mod_square_precomputed_assign_ref);

    register_bench!(
        runner,
        benchmark_natural_mod_square_assign_evaluation_strategy
    );
    register_bench!(runner, benchmark_natural_mod_square_evaluation_strategy);
    register_bench!(runner, benchmark_natural_mod_square_algorithms);
    register_bench!(runner, benchmark_natural_mod_square_precomputed_algorithms);
}

fn demo_natural_mod_square_assign(gm: GenMode, config: &GenConfig, limit: usize) {
    for (mut n, m) in natural_pair_gen_var_8().get(gm, config).take(limit) {
        let n_old = n.clone();
        let m_old = m.clone();
        n.mod_square_assign(m);
        println!("x := {n_old}; x.mod_square_assign({m_old}); x = {n}");
    }
}

fn demo_natural_mod_square_assign_ref(gm: GenMode, config: &GenConfig, limit: usize) {
    for (mut n, m) in natural_pair_gen_var_8().get(gm, config).take(limit) {
        let n_old = n.clone();
        n.mod_square_assign(&m);
        println!("x := {n_old}; x.mod_square_assign(&{m}); x = {n}");
    }
}

fn demo_natural_mod_square(gm: GenMode, config: &GenConfig, limit: usize) {
    for (n, m) in natural_pair_gen_var_8().get(gm, config).take(limit) {
        let n_old = n.clone();
        let m_old = m.clone();
        println!("{}.square() ≡ {} mod {}", n_old, n.mod_square(m), m_old);
    }
}

fn demo_natural_mod_square_val_ref(gm: GenMode, config: &GenConfig, limit: usize) {
    for (n, m) in natural_pair_gen_var_8().get(gm, config).take(limit) {
        let n_old = n.clone();
        println!("{}.square() ≡ {} mod &{}", n_old, n.mod_square(&m), m);
    }
}

fn demo_natural_mod_square_ref_val(gm: GenMode, config: &GenConfig, limit: usize) {
    for (n, m) in natural_pair_gen_var_8().get(gm, config).take(limit) {
        let m_old = m.clone();
        println!("(&{}).square() ≡ {} mod {}", n, (&n).mod_square(m), m_old);
    }
}

fn demo_natural_mod_square_ref_ref(gm: GenMode, config: &GenConfig, limit: usize) {
    for (n, m) in natural_pair_gen_var_8().get(gm, config).take(limit) {
        println!("(&{}).square() ≡ {} mod &{}", n, (&n).mod_square(&m), m);
    }
}

fn demo_natural_mod_square_precomputed(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, m) in natural_pair_gen_var_8().get(gm, config).take(limit) {
        let x_old = x.clone();
        let m_old = m.clone();
        let data = ModPowPrecomputed::<Natural>::precompute_mod_pow_data(&m);
        println!(
            "{}.mod_square_precomputed({}, &data) = {}",
            x_old,
            m_old,
            x.mod_square_precomputed(m, &data)
        );
    }
}

fn demo_natural_mod_square_precomputed_val_ref(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, m) in natural_pair_gen_var_8().get(gm, config).take(limit) {
        let x_old = x.clone();
        let data = ModPowPrecomputed::<Natural>::precompute_mod_pow_data(&m);
        println!(
            "{}.mod_square_precomputed(&{}, &data) = {}",
            x_old,
            m,
            x.mod_square_precomputed(&m, &data)
        );
    }
}

fn demo_natural_mod_square_precomputed_ref_val(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, m) in natural_pair_gen_var_8().get(gm, config).take(limit) {
        let m_old = m.clone();
        let data = ModPowPrecomputed::<Natural>::precompute_mod_pow_data(&m);
        println!(
            "(&{}).mod_square_precomputed({}, &data) = {}",
            x,
            m_old,
            (&x).mod_square_precomputed(m, &data)
        );
    }
}

fn demo_natural_mod_square_precomputed_ref_ref(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, m) in natural_pair_gen_var_8().get(gm, config).take(limit) {
        let data = ModPowPrecomputed::<Natural>::precompute_mod_pow_data(&m);
        println!(
            "(&{}).mod_square_precomputed(&{}, &data) = {}",
            x,
            m,
            (&x).mod_square_precomputed(&m, &data)
        );
    }
}

fn demo_natural_mod_square_precomputed_assign(gm: GenMode, config: &GenConfig, limit: usize) {
    for (mut x, m) in natural_pair_gen_var_8().get(gm, config).take(limit) {
        let x_old = x.clone();
        let m_old = m.clone();
        let data = ModPowPrecomputed::<Natural>::precompute_mod_pow_data(&m);
        x.mod_square_precomputed_assign(m, &data);
        println!("x := {x_old}; x.mod_square_precomputed_assign({m_old}, &data); x = {x}");
    }
}

fn demo_natural_mod_square_precomputed_assign_ref(gm: GenMode, config: &GenConfig, limit: usize) {
    for (mut x, m) in natural_pair_gen_var_8().get(gm, config).take(limit) {
        let x_old = x.clone();
        let data = ModPowPrecomputed::<Natural>::precompute_mod_pow_data(&m);
        x.mod_square_precomputed_assign(&m, &data);
        println!("x := {x_old}; x.mod_square_precomputed_assign(&{m}, &data); x = {x}");
    }
}

fn benchmark_natural_mod_square_assign_evaluation_strategy(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Natural.mod_square_assign(Natural)",
        BenchmarkType::EvaluationStrategy,
        natural_pair_gen_var_8().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_2_natural_bit_bucketer("m"),
        &mut [
            ("Natural.mod_square_assign(Natural)", &mut |(mut n, m)| {
                n.mod_square_assign(m);
            }),
            ("Natural.mod_square_assign(&Natural)", &mut |(mut n, m)| {
                n.mod_square_assign(&m);
            }),
        ],
    );
}

fn benchmark_natural_mod_square_evaluation_strategy(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Natural.mod_square(Natural)",
        BenchmarkType::EvaluationStrategy,
        natural_pair_gen_var_8().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_2_natural_bit_bucketer("m"),
        &mut [
            ("Natural.mod_square(Natural)", &mut |(n, m)| {
                no_out!(n.mod_square(m));
            }),
            ("Natural.mod_square(&Natural)", &mut |(n, m)| {
                no_out!(n.mod_square(&m));
            }),
            ("(&Natural).mod_square(Natural)", &mut |(n, m)| {
                no_out!((&n).mod_square(m));
            }),
            ("(&Natural).mod_square(&Natural)", &mut |(n, m)| {
                no_out!((&n).mod_square(&m));
            }),
        ],
    );
}

#[allow(clippy::unnecessary_operation, unused_must_use)]
fn benchmark_natural_mod_square_algorithms(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Natural.mod_square(Natural)",
        BenchmarkType::Algorithms,
        natural_pair_gen_var_8().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_2_natural_bit_bucketer("m"),
        &mut [
            ("Natural.mod_square(Natural)", &mut |(n, m)| {
                no_out!((&n).mod_square(&m));
            }),
            ("Natural.mod_mul(Natural, Natural)", &mut |(n, m)| {
                no_out!((&n).mod_mul(&n, &m));
            }),
            ("Natural.square() % Natural", &mut |(n, m)| {
                no_out!((&n).square() % &m);
            }),
        ],
    );
}

fn benchmark_natural_mod_square_precomputed_algorithms(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Natural.mod_square_precomputed(Natural, &ModMulData)",
        BenchmarkType::Algorithms,
        natural_pair_gen_var_8().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_2_natural_bit_bucketer("m"),
        &mut [
            ("default", &mut |(x, m)| {
                for _ in 0..10 {
                    (&x).mod_square(&m);
                }
            }),
            ("precomputed", &mut |(x, m)| {
                let data = ModPowPrecomputed::<Natural>::precompute_mod_pow_data(&m);
                for _ in 0..10 {
                    (&x).mod_square_precomputed(&m, &data);
                }
            }),
        ],
    );
}
