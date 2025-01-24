// Copyright © 2025 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::arithmetic::traits::{JacobiSymbol, KroneckerSymbol};
use malachite_base::test_util::bench::bucketers::pair_vec_max_len_bucketer;
use malachite_base::test_util::bench::{run_benchmark, BenchmarkType};
use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::generators::unsigned_vec_pair_gen_var_32;
use malachite_base::test_util::runner::Runner;
use malachite_nz::natural::arithmetic::kronecker_symbol::{
    limbs_jacobi_symbol_init, limbs_jacobi_symbol_same_length,
};
use malachite_nz::test_util::bench::bucketers::{
    pair_2_natural_bit_bucketer, pair_2_pair_natural_max_bit_bucketer,
};
use malachite_nz::test_util::generators::{
    natural_pair_gen, natural_pair_gen_rm, natural_pair_gen_var_12, natural_pair_gen_var_12_rm,
};
use malachite_nz::test_util::natural::arithmetic::kronecker_symbol::jacobi_symbol_simple;

pub(crate) fn register(runner: &mut Runner) {
    register_demo!(runner, demo_limbs_jacobi_symbol_same_length);
    register_demo!(runner, demo_natural_jacobi_symbol);
    register_demo!(runner, demo_natural_jacobi_symbol_val_ref);
    register_demo!(runner, demo_natural_jacobi_symbol_ref_val);
    register_demo!(runner, demo_natural_jacobi_symbol_ref_ref);
    register_demo!(runner, demo_natural_kronecker_symbol);
    register_demo!(runner, demo_natural_kronecker_symbol_val_ref);
    register_demo!(runner, demo_natural_kronecker_symbol_ref_val);
    register_demo!(runner, demo_natural_kronecker_symbol_ref_ref);

    register_bench!(runner, benchmark_limbs_jacobi_symbol_same_length);
    register_bench!(runner, benchmark_natural_jacobi_symbol_library_comparison);
    register_bench!(runner, benchmark_natural_jacobi_symbol_evaluation_strategy);
    register_bench!(runner, benchmark_natural_jacobi_symbol_algorithms);
    register_bench!(
        runner,
        benchmark_natural_kronecker_symbol_library_comparison
    );
    register_bench!(
        runner,
        benchmark_natural_kronecker_symbol_evaluation_strategy
    );
}

fn demo_limbs_jacobi_symbol_same_length(gm: GenMode, config: &GenConfig, limit: usize) {
    for (mut xs, mut ys) in unsigned_vec_pair_gen_var_32().get(gm, config).take(limit) {
        let xs_old = xs.clone();
        let ys_old = ys.clone();
        let bits = limbs_jacobi_symbol_init(xs[0], ys[0], 0);
        let s = limbs_jacobi_symbol_same_length(&mut xs, &mut ys, bits);
        println!("limbs_jacobi_symbol_same_length({xs_old:?}, {ys_old:?}) = {s}");
    }
}

fn demo_natural_jacobi_symbol(gm: GenMode, config: &GenConfig, limit: usize) {
    for (n, m) in natural_pair_gen_var_12().get(gm, config).take(limit) {
        let n_old = n.clone();
        let m_old = m.clone();
        println!(
            "{}.jacobi_symbol({}) = {}",
            n_old,
            m_old,
            n.jacobi_symbol(m)
        );
    }
}

fn demo_natural_jacobi_symbol_val_ref(gm: GenMode, config: &GenConfig, limit: usize) {
    for (n, m) in natural_pair_gen_var_12().get(gm, config).take(limit) {
        let n_old = n.clone();
        println!("{}.jacobi_symbol({}) = {}", n_old, m, n.jacobi_symbol(&m));
    }
}

fn demo_natural_jacobi_symbol_ref_val(gm: GenMode, config: &GenConfig, limit: usize) {
    for (n, m) in natural_pair_gen_var_12().get(gm, config).take(limit) {
        let m_old = m.clone();
        println!("{}.jacobi_symbol({}) = {}", n, m_old, (&n).jacobi_symbol(m));
    }
}

fn demo_natural_jacobi_symbol_ref_ref(gm: GenMode, config: &GenConfig, limit: usize) {
    for (n, m) in natural_pair_gen_var_12().get(gm, config).take(limit) {
        println!("{}.jacobi_symbol({}) = {}", n, m, (&n).jacobi_symbol(&m));
    }
}

fn demo_natural_kronecker_symbol(gm: GenMode, config: &GenConfig, limit: usize) {
    for (n, m) in natural_pair_gen().get(gm, config).take(limit) {
        let n_old = n.clone();
        let m_old = m.clone();
        println!(
            "{}.kronecker_symbol({}) = {}",
            n_old,
            m_old,
            n.kronecker_symbol(m)
        );
    }
}

fn demo_natural_kronecker_symbol_val_ref(gm: GenMode, config: &GenConfig, limit: usize) {
    for (n, m) in natural_pair_gen().get(gm, config).take(limit) {
        let n_old = n.clone();
        println!(
            "{}.kronecker_symbol({}) = {}",
            n_old,
            m,
            n.kronecker_symbol(&m)
        );
    }
}

fn demo_natural_kronecker_symbol_ref_val(gm: GenMode, config: &GenConfig, limit: usize) {
    for (n, m) in natural_pair_gen().get(gm, config).take(limit) {
        let m_old = m.clone();
        println!(
            "{}.kronecker_symbol({}) = {}",
            n,
            m_old,
            (&n).kronecker_symbol(m)
        );
    }
}

fn demo_natural_kronecker_symbol_ref_ref(gm: GenMode, config: &GenConfig, limit: usize) {
    for (n, m) in natural_pair_gen().get(gm, config).take(limit) {
        println!(
            "{}.kronecker_symbol({}) = {}",
            n,
            m,
            (&n).kronecker_symbol(&m)
        );
    }
}

fn benchmark_limbs_jacobi_symbol_same_length(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "limbs_jacobi_symbol_same_length(&mut [Limb], &mut [Limb], u8)",
        BenchmarkType::Single,
        unsigned_vec_pair_gen_var_32().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_vec_max_len_bucketer("xs", "ys"),
        &mut [("Malachite", &mut |(mut xs, mut ys)| {
            let bits = limbs_jacobi_symbol_init(xs[0], ys[0], 0);
            limbs_jacobi_symbol_same_length(&mut xs, &mut ys, bits);
        })],
    );
}

fn benchmark_natural_jacobi_symbol_library_comparison(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Natural.jacobi_symbol(Natural)",
        BenchmarkType::LibraryComparison,
        natural_pair_gen_var_12_rm().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_2_pair_natural_max_bit_bucketer("x", "y"),
        &mut [
            ("Malachite", &mut |(_, (x, y))| no_out!(x.jacobi_symbol(y))),
            ("rug", &mut |((x, y), _)| no_out!(x.jacobi(&y))),
        ],
    );
}

fn benchmark_natural_jacobi_symbol_evaluation_strategy(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Natural.jacobi_symbol(Natural)",
        BenchmarkType::EvaluationStrategy,
        natural_pair_gen_var_12().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_2_natural_bit_bucketer("m"),
        &mut [
            ("Natural.jacobi_symbol(Natural)", &mut |(n, m)| {
                no_out!(n.jacobi_symbol(m))
            }),
            ("Natural.jacobi_symbol(&Natural)", &mut |(n, m)| {
                no_out!(n.jacobi_symbol(&m))
            }),
            ("(&Natural).jacobi_symbol(Natural)", &mut |(n, m)| {
                no_out!((&n).jacobi_symbol(m))
            }),
            ("(&Natural).jacobi_symbol(&Natural)", &mut |(n, m)| {
                no_out!((&n).jacobi_symbol(&m))
            }),
        ],
    );
}

fn benchmark_natural_jacobi_symbol_algorithms(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Natural.jacobi_symbol(Natural)",
        BenchmarkType::Algorithms,
        natural_pair_gen_var_12().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_2_natural_bit_bucketer("m"),
        &mut [
            ("default", &mut |(n, m)| no_out!(n.jacobi_symbol(m))),
            ("simple", &mut |(n, m)| no_out!(jacobi_symbol_simple(n, m))),
        ],
    );
}

fn benchmark_natural_kronecker_symbol_library_comparison(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Natural.kronecker_symbol(Natural)",
        BenchmarkType::LibraryComparison,
        natural_pair_gen_rm().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_2_pair_natural_max_bit_bucketer("x", "y"),
        &mut [
            ("Malachite", &mut |(_, (x, y))| {
                no_out!(x.kronecker_symbol(y))
            }),
            ("rug", &mut |((x, y), _)| no_out!(x.kronecker(&y))),
        ],
    );
}

fn benchmark_natural_kronecker_symbol_evaluation_strategy(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Natural.kronecker_symbol(Natural)",
        BenchmarkType::EvaluationStrategy,
        natural_pair_gen().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_2_natural_bit_bucketer("m"),
        &mut [
            ("Natural.kronecker_symbol(Natural)", &mut |(n, m)| {
                no_out!(n.kronecker_symbol(m))
            }),
            ("Natural.kronecker_symbol(&Natural)", &mut |(n, m)| {
                no_out!(n.kronecker_symbol(&m))
            }),
            ("(&Natural).kronecker_symbol(Natural)", &mut |(n, m)| {
                no_out!((&n).kronecker_symbol(m))
            }),
            ("(&Natural).kronecker_symbol(&Natural)", &mut |(n, m)| {
                no_out!((&n).kronecker_symbol(&m))
            }),
        ],
    );
}
